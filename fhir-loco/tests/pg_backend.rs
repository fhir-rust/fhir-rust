//! The postgres backend, end to end: boot with `FHIR_LOCO_BACKEND=postgresql`
//! and run a CRUD round-trip plus search through HTTP against a live
//! PostgreSQL — the multi-port wiring's proof that `AnyStore` serves the same
//! surface over a second engine.
//!
//! A separate test **binary**, deliberately: the store is a process-global
//! `OnceLock` and the backend is chosen at boot, so a postgres-backed process
//! cannot share a process with the sqlite-backed suite.
//!
//! Needs a live server (`fhir-postgresql/scripts/db.sh up`) and
//! `FHIR_LOCO_TEST_PG=1`; skipped otherwise. Connection details follow the
//! `PG*` environment (`PGHOST`/`PGPORT`/`PGUSER`/`PGPASSWORD`/`PGSSLMODE`),
//! same as the store's own live suite.

use std::sync::OnceLock;

use fhir_loco::app::App;
use loco_rs::testing::prelude::*;

const FHIR_JSON: &str = "application/fhir+json";

fn issuer() -> &'static pasetors::keys::AsymmetricKeyPair<pasetors::version4::V4> {
    use pasetors::keys::Generate;
    static KP: OnceLock<pasetors::keys::AsymmetricKeyPair<pasetors::version4::V4>> =
        OnceLock::new();
    KP.get_or_init(|| {
        pasetors::keys::AsymmetricKeyPair::<pasetors::version4::V4>::generate()
            .expect("generate test keypair")
    })
}

fn bearer(subject: &str) -> String {
    use pasetors::claims::Claims;
    let mut claims = Claims::new().expect("claims");
    claims.subject(subject).expect("subject");
    let token =
        pasetors::public::sign(&issuer().secret, &claims, None, None).expect("sign test token");
    format!("Bearer {token}")
}

#[tokio::test]
async fn crud_and_search_round_trip_on_the_postgres_backend() {
    if std::env::var("FHIR_LOCO_TEST_PG").is_err() {
        eprintln!("skipping: set FHIR_LOCO_TEST_PG=1 (and PG* env) to run");
        return;
    }

    // Install a small R5 schema through the library, the way an operator
    // would — this service deliberately does not own installation.
    let assets = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-postgresql/crates/fhir-postgresql-map/assets");
    let bytes = std::fs::read(assets.join("fhir-postgresql-relmap-r5.json.gz"))
        .expect("relmap asset; run from a checkout with fhir-postgresql beside it");
    let mut map =
        fhir_postgresql_map::model::RelMap::from_gz_bytes(&bytes).expect("relmap decodes");
    map.resources
        .retain(|k, _| k == "Patient" || k == "Practitioner");
    let cfg = fhir_postgresql_store::pg_config(None).expect("pg config from PG* env");
    let store = fhir_postgresql_store::Store::connect(cfg.clone(), std::sync::Arc::new(map))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("loco-pg-test").await.expect("install schema");
    drop(store);

    // Boot the app on the postgres backend. The mount reads the full asset
    // map; only Patient/Practitioner tables exist, which is fine — the
    // sampled install carries the checksum the probe looks for, and the
    // tests below touch only what is installed.
    let dsn = format!(
        "host={} port={} user={} password={} dbname={}",
        std::env::var("PGHOST").unwrap_or_else(|_| "127.0.0.1".into()),
        std::env::var("PGPORT").unwrap_or_else(|_| "5432".into()),
        std::env::var("PGUSER").unwrap_or_else(|_| "fhir".into()),
        std::env::var("PGPASSWORD").unwrap_or_else(|_| "fhir".into()),
        std::env::var("PGDATABASE").unwrap_or_else(|_| "fhir".into()),
    );
    unsafe {
        std::env::set_var(
            fhir_loco::auth::ENV_PUBLIC_KEY,
            hex::encode(issuer().public.as_bytes()),
        );
        std::env::set_var("FHIR_LOCO_BACKEND", "postgresql");
        std::env::set_var("FHIR_LOCO_PG_DSN", &dsn);
        std::env::set_var("FHIR_LOCO_ASSETS", assets.to_str().expect("utf-8"));
    }

    request::<App, _, _>(|request, _ctx| async move {
        // Create, read, update, history, delete — the store's distinctions
        // preserved over HTTP, exactly as on sqlite.
        let res = request
            .put("/r5/Patient/pg-1")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-pg"))
            .text(r#"{"resourceType":"Patient","id":"pg-1","name":[{"family":"Postgres"}]}"#)
            .await;
        assert!(res.status_code().is_success(), "create: {}", res.text());

        let res = request.get("/r5/Patient/pg-1").await;
        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).expect("json");
        assert_eq!(body["name"][0]["family"], "Postgres");

        let res = request
            .get("/r5/Patient?family=Postgres&_total=accurate")
            .await;
        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).expect("json");
        assert_eq!(body["total"], 1, "{body}");
        assert_eq!(body["entry"][0]["search"]["mode"], "match");

        // Type-level history on the second backend (SV2.17 over pg's
        // history_page).
        let res = request.get("/r5/Patient/_history").await;
        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).expect("json");
        assert_eq!(body["type"], "history");
        assert!(body["entry"].as_array().is_some_and(|e| !e.is_empty()));

        // Conditional delete on the second backend (SV2.19 over pg's
        // conditional_delete_audited) — a separate resource from pg-1, so
        // the instance-level delete below is unaffected.
        let res = request
            .put("/r5/Patient/pg-2")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-pg"))
            .text(r#"{"resourceType":"Patient","id":"pg-2","name":[{"family":"ToDelete"}]}"#)
            .await;
        assert!(res.status_code().is_success(), "create: {}", res.text());
        let res = request
            .delete("/r5/Patient?family=ToDelete")
            .add_header("authorization", &bearer("dr-pg"))
            .await;
        assert_eq!(
            res.status_code(),
            204,
            "conditional delete on the second backend: {}",
            res.text()
        );
        let res = request.get("/r5/Patient/pg-2").await;
        assert_eq!(res.status_code(), 410, "the match must actually be gone");
        // No match is idempotent here too, not an error.
        let res = request
            .delete("/r5/Patient?family=ToDelete")
            .add_header("authorization", &bearer("dr-pg"))
            .await;
        assert_eq!(res.status_code(), 204);

        let res = request
            .delete("/r5/Patient/pg-1")
            .add_header("authorization", &bearer("dr-pg"))
            .await;
        assert!(res.status_code().is_success());
        // Deleted answers 410, never-existed 404 — the distinction is the
        // store's; the second backend must preserve it too.
        let res = request.get("/r5/Patient/pg-1").await;
        assert_eq!(res.status_code(), 410);
        let res = request.get("/r5/Patient/never-was").await;
        assert_eq!(res.status_code(), 404);
    })
    .await;
}
