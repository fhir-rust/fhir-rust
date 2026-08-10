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
        let acknowledged = std::env::var("FHIR_LOCO_TLS_TERMINATED_UPSTREAM")
            .is_ok_and(|v| v == "true" || v == "1");
        if let Err(e) = crate::auth::listener_posture(&ctx.config.server.binding, acknowledged) {
            return Err(loco_rs::Error::Message(e));
        }
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

        let db = std::env::var("FHIR_LOCO_DB").unwrap_or_else(|_| "fhir.sqlite".to_string());
        let assets = std::env::var("FHIR_LOCO_ASSETS").unwrap_or_else(|_| "assets".to_string());
        let backend = std::env::var("FHIR_LOCO_BACKEND").unwrap_or_else(|_| "sqlite".to_string());
        let pg_dsn = std::env::var("FHIR_LOCO_PG_DSN").ok();
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

        // The admin plane — /health, /ready, /metrics — on its own listener
        // (SV4.3): operational endpoints must be exposable to an operations
        // network without exposing the FHIR API's PHI to it. Off unless
        // FHIR_LOCO_ADMIN_BIND is set.
        crate::admin::spawn_if_configured();
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
