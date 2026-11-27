use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
    Result,
};

#[allow(unused_imports)]
use crate::{controllers, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self>(mode, environment, config).await
    }

    /// Open the FHIR stores before the server accepts traffic.
    ///
    /// `before_run`, not `boot`: `boot` is not on the path the `start` command
    /// takes, so initialising there left every request answering 503 while the
    /// health check stayed green — the worst combination, since a load balancer
    /// sees a healthy instance serving nothing.
    ///
    /// Failing here is deliberate for the same reason. A store that cannot be
    /// opened should stop the process, not become a per-request error.
    ///
    /// Configured by environment rather than by feature flag, so a deployment
    /// can point at a different database without a rebuild.
    async fn before_run(ctx: &AppContext) -> Result<()> {
        // The listener's transport posture first (SV3.11): a non-loopback
        // bind of this plain-HTTP listener needs the deployment's explicit
        // acknowledgement that TLS terminates upstream. Refusing here, before
        // anything is served, is the point — the same
        // decide-deliberately-or-not-at-all shape as O10.7's database rule.
        let acknowledged = crate::config::env_flag("FHIR_LOCO_TLS_TERMINATED_UPSTREAM")
            .map_err(loco_rs::Error::Message)?;
        if let Err(e) = crate::auth::listener_posture(&ctx.config.server.binding, acknowledged) {
            return Err(loco_rs::Error::Message(e));
        }
        // PR12.5: a read whose disclosure cannot be recorded is refused,
        // unless the deployment has chosen otherwise, out loud.
        let fail_open = crate::config::disclosure_fail_open().map_err(loco_rs::Error::Message)?;
        if fail_open {
            tracing::warn!(
                "FHIR_LOCO_DISCLOSURE_FAIL_OPEN=true: content will be served even when its \
                 disclosure cannot be recorded (PR12.5)"
            );
        }
        crate::controllers::fhir::set_disclosure_fail_open(fail_open);

        // Establish how principals are proven, before anything can be written.
        //
        // This fails the boot when the key is missing or unusable, and that is
        // the whole design: there is no unauthenticated mode to fall back to,
        // so a misconfigured deployment stops here rather than starting and
        // trusting whoever calls it.
        match crate::auth::init() {
            Ok(_) => tracing::info!("authentication: PASETO v4.public required on every request"),
            Err(e) => return Err(loco_rs::Error::Message(format!("authentication: {e}"))),
        }

        let db = crate::config::env_or("FHIR_LOCO_DB", "fhir.sqlite")
            .map_err(loco_rs::Error::Message)?;
        let assets =
            crate::config::env_or("FHIR_LOCO_ASSETS", "assets").map_err(loco_rs::Error::Message)?;
        let backend = crate::config::env_or("FHIR_LOCO_BACKEND", "sqlite")
            .map_err(loco_rs::Error::Message)?;
        let pg_dsn = crate::config::env_opt("FHIR_LOCO_PG_DSN").map_err(loco_rs::Error::Message)?;
        let cfg = match backend.as_str() {
            "sqlite" => crate::store::BackendConfig::Sqlite {
                db_path: &db,
                assets_dir: &assets,
            },
            "postgresql" => crate::store::BackendConfig::Postgres {
                dsn: pg_dsn.as_deref().ok_or_else(|| {
                    loco_rs::Error::Message(
                        "FHIR_LOCO_BACKEND=postgresql needs FHIR_LOCO_PG_DSN".to_string(),
                    )
                })?,
                assets_dir: &assets,
            },
            other => {
                return Err(loco_rs::Error::Message(format!(
                    "unknown FHIR_LOCO_BACKEND {other:?} (sqlite | postgresql)"
                )));
            }
        };
        match crate::store::init(cfg).await {
            Ok(v) if v.mounted().is_empty() => tracing::warn!(
                db = %db, assets = %assets,
                "no installed FHIR schemas found; serving metadata only"
            ),
            Ok(v) => tracing::info!(db = %db, versions = ?v.mounted(), "FHIR stores mounted"),
            Err(e) => return Err(loco_rs::Error::Message(format!("opening FHIR store: {e}"))),
        }

        // The SNOMED CT release behind the terminology operations (SV2.20):
        // loaded before traffic, and a configured release that will not load
        // stops the boot, for the store's reason above. Off the async
        // workers: a full edition takes a while to read.
        match tokio::task::spawn_blocking(crate::terminology::init_from_env)
            .await
            .map_err(|e| loco_rs::Error::Message(format!("loading SNOMED CT: {e}")))?
        {
            Ok(Some(concepts)) => tracing::info!(concepts, "SNOMED CT release loaded"),
            Ok(None) => {
                tracing::info!("no SNOMED CT release configured; terminology operations off")
            }
            Err(e) => return Err(loco_rs::Error::Message(e)),
        }

        // The admin plane — /health, /ready, /metrics — on its own listener
        // (SV4.3): operational endpoints must be exposable to an operations
        // network without exposing the FHIR API's PHI to it. Off unless
        // FHIR_LOCO_ADMIN_BIND is set.
        // Validate the export lifetime now, so a bad setting stops startup.
        crate::config::export_ttl_secs().map_err(loco_rs::Error::Message)?;
        // Exported PHI has a lifetime across restarts too: remove what a
        // previous process orphaned, then expire on a timer.
        crate::controllers::export::start_retention().map_err(loco_rs::Error::Message)?;
        crate::admin::spawn_if_configured().map_err(loco_rs::Error::Message)?;
        Ok(())
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::home::routes())
            .add_route(controllers::fhir::routes())
            .add_route(controllers::export::routes())
            .add_route(controllers::terminology::routes())
    }

    /// Time every request into the admin plane's latency histogram (SV4.3).
    async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
        Ok(router.layer(axum::middleware::from_fn(crate::admin::record)))
    }
    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
}
