//! The Oracle store, exercised against a real server (`T11.2`).
//!
//! Needs `FHIR_ORACLE_TEST_USER`/`_PASSWORD`/`_CONNECT`; `scripts/db.sh up`
//! prints them, alongside `FHIR_ORACLE_TEST_DSN` (kept as the "is live
//! testing configured" signal every port's tests check the same way).
//! Skips silently without them.
//!
//! Also needs Oracle Instant Client on the host — see `scripts/db.sh`'s
//! header comment. Without it every test here fails with `DPI-1047`, not a
//! skip, the same way a missing `FHIR_ORACLE_TEST_DSN` would not be conflated
//! with "the server rejected the connection".
//!
//! This is the file that closes **F-68**: `fhir-oracle-store` was written
//! entirely without a database to run against (F-66) and is exercised here
//! for the first time. Every operation below was hand-verified once against
//! a live `gvenzl/oracle-free:23-slim-faststart` before being written down
//! as a test, and five real defects were found and fixed doing it — four
//! before this file existed (uppercase schema case-folding, `R4.5`'s `SET
//! TRANSACTION READ ONLY` failing with `ORA-01466`, a double
//! schema-qualification bug, and a timestamp-binding bug) and a fifth
//! (`ORA-01722` binding a boolean token as text) found by this suite itself
//! on its first run — see `oracle.rs`'s module doc and `audit.md` **F-68**
//! for the account.

use std::sync::Arc;

use fhir_oracle_map::model::RelMap;
use fhir_oracle_store::oracle::OracleStore;

fn creds() -> Option<(String, String, String)> {
    let user = std::env::var("FHIR_ORACLE_TEST_USER").ok()?;
    let password = std::env::var("FHIR_ORACLE_TEST_PASSWORD").ok()?;
    let connect = std::env::var("FHIR_ORACLE_TEST_CONNECT").ok()?;
    Some((user, password, connect))
}

/// A map trimmed to the resource types a test actually uses, and to the
/// uppercase schema this engine requires (`M14.5`; found live — Oracle folds
/// unquoted usernames to uppercase for authentication regardless of how
/// `CREATE USER` quoted them, so the schema every generated identifier
/// qualifies with must be uppercase too, or every table lookup fails
/// `ORA-01031`).
fn sampled(want: &[&str]) -> Option<Arc<RelMap>> {
    let mut m = RelMap::bundled("r5").ok()?;
    m.resources.retain(|k, _| want.contains(&k.as_str()));
    assert!(!m.resources.is_empty(), "none of {want:?} are in the r5 map");
    m.schema = "R5".to_string();
    Some(Arc::new(m))
}

async fn fresh(want: &[&str]) -> Option<OracleStore> {
    let (user, password, connect) = creds()?;
    let map = sampled(want)?;
    let store = OracleStore::connect(&user, &password, &connect, map)
        .await
        .expect("connect");
    store.drop_schema().await.ok();
    Some(store)
}

/// Parsed from text, not built with `serde_json::json!`: the macro turns
/// `9.60` into an `f64` literal at compile time, so the trailing zero would
/// be gone before any store saw it — found live, the hard way, the first
/// time this test ran and blamed the store for what was actually its own
/// fixture (`M3.6` is about preserving what was *parsed*).
fn observation(id: &str, status: &str) -> serde_json::Value {
    serde_json::from_str(&format!(
        r#"{{
            "resourceType": "Observation",
            "id": "{id}",
            "status": "{status}",
            "code": {{ "coding": [
                {{ "system": "http://loinc.org", "code": "2339-0" }},
                {{ "system": "http://example.org", "code": "gluc" }}
            ] }},
            "valueQuantity": {{ "value": 9.60, "unit": "mg/dL" }},
            "note": [ {{ "text": "first" }}, {{ "text": "second" }} ]
        }}"#
    ))
    .expect("fixture parses")
}

#[tokio::test]
async fn init_installs_tables_and_triggers() {
    let Some(store) = fresh(&["Observation", "Patient"]).await else {
        eprintln!("skipping: set FHIR_ORACLE_TEST_USER/_PASSWORD/_CONNECT to run");
        return;
    };
    let n = store.init("test-checksum").await.expect("init");
    assert!(n > 50, "only {n} statements applied");
    assert_eq!(
        store.installed_checksum().await.expect("checksum"),
        Some("test-checksum".to_string()),
        "init must record the map checksum it installed from"
    );
    store.ping().await.expect("ping");
    store.drop_schema().await.expect("drop");
}

#[tokio::test]
async fn put_then_get_round_trips_a_resource() {
    let Some(store) = fresh(&["Observation", "Patient"]).await else {
        eprintln!("skipping: set FHIR_ORACLE_TEST_USER/_PASSWORD/_CONNECT to run");
        return;
    };
    store.init("rt").await.expect("init");
    let audit = fhir_oracle_store::Audit::default();

    let src = observation("obs-1", "final");
    let put = store.put(&src, &audit).await.expect("put");
    assert_eq!(put.id, "obs-1");
    assert_eq!(put.version_id, 1);
    assert!(put.created);

    let got = store
        .get("Observation", "obs-1")
        .await
        .expect("get")
        .expect("present");
    assert_eq!(got, src, "round trip lost or altered data");
    assert_eq!(
        got["valueQuantity"]["value"].to_string(),
        "9.60",
        "decimal precision lost (M3.6)"
    );

    store.drop_schema().await.expect("drop");
}

#[tokio::test]
async fn rewrite_replaces_children_and_bumps_the_version() {
    let Some(store) = fresh(&["Observation", "Patient"]).await else {
        eprintln!("skipping: set FHIR_ORACLE_TEST_USER/_PASSWORD/_CONNECT to run");
        return;
    };
    store.init("rw").await.expect("init");
    let audit = fhir_oracle_store::Audit::default();

    store.put(&observation("obs-1", "final"), &audit).await.expect("v1");

    let mut v2 = observation("obs-1", "amended");
    v2["note"] = serde_json::from_str(r#"[{"text":"only"}]"#).expect("json");
    let put = store.put(&v2, &audit).await.expect("v2");
    assert_eq!(put.version_id, 2);
    assert!(!put.created);

    let got = store
        .get("Observation", "obs-1")
        .await
        .expect("get")
        .expect("present");
    assert_eq!(
        got["note"].as_array().map(Vec::len),
        Some(1),
        "stale child rows survived the rewrite"
    );
    assert_eq!(got["status"], "amended");

    store.drop_schema().await.expect("drop");
}

#[tokio::test]
async fn history_vread_delete_and_verify_audit() {
    let Some(store) = fresh(&["Observation", "Patient"]).await else {
        eprintln!("skipping: set FHIR_ORACLE_TEST_USER/_PASSWORD/_CONNECT to run");
        return;
    };
    store.init("hist").await.expect("init");
    let audit = fhir_oracle_store::Audit::default();

    store.put(&observation("obs-1", "final"), &audit).await.expect("v1");
    store.put(&observation("obs-1", "amended"), &audit).await.expect("v2");
    let tomb = store
        .delete("Observation", "obs-1", &audit)
        .await
        .expect("delete")
        .expect("something was deleted");
    assert_eq!(tomb, 3);

    let h = store.history("Observation", "obs-1").await.expect("history");
    assert_eq!(h.len(), 3, "expected create, update, delete");
    assert_eq!(h[0].version_id, 3);
    assert_eq!(h[0].op, 'D');
    assert!(h[0].resource.is_none(), "a tombstone carries no content");
    assert_eq!(h[2].op, 'C');
    assert_eq!(h[1].op, 'U');

    assert!(store.get("Observation", "obs-1").await.expect("get").is_none());
    let v1 = store
        .vread("Observation", "obs-1", 1)
        .await
        .expect("vread")
        .expect("v1");
    assert_eq!(v1.resource.as_ref().unwrap()["status"], "final");
    let v3 = store
        .vread("Observation", "obs-1", 3)
        .await
        .expect("vread")
        .expect("v3 is the deletion");
    assert_eq!(v3.op, 'D');
    assert!(v3.resource.is_none());

    let breaks = store.verify_audit().await.expect("verify");
    assert!(breaks.is_empty(), "a chain nobody touched reported breaks: {breaks:?}");

    store.drop_schema().await.expect("drop");
}

#[tokio::test]
async fn purge_erases_history_and_leaves_a_verifiable_hole() {
    let Some(store) = fresh(&["Observation", "Patient"]).await else {
        eprintln!("skipping: set FHIR_ORACLE_TEST_USER/_PASSWORD/_CONNECT to run");
        return;
    };
    store.init("purge").await.expect("init");
    let audit = fhir_oracle_store::Audit::default();

    store.put(&observation("obs-1", "final"), &audit).await.expect("v1");
    store.put(&observation("obs-1", "amended"), &audit).await.expect("v2");
    assert_eq!(store.history("Observation", "obs-1").await.expect("h").len(), 2);

    let report = store.purge("Observation", "obs-1", &audit).await.expect("purge");
    assert!(report.existed);
    assert_eq!(report.versions_erased, 2);

    assert!(store.get("Observation", "obs-1").await.expect("get").is_none());
    assert!(
        store
            .vread("Observation", "obs-1", 1)
            .await
            .expect("vread")
            .is_none(),
        "an erased version must not still be readable"
    );

    let h = store.history("Observation", "obs-1").await.expect("history");
    assert_eq!(h.len(), 1, "expected exactly the tombstone");
    assert_eq!(h[0].op, 'P');
    assert!(h[0].resource.is_none());

    let breaks = store.verify_audit().await.expect("verify");
    assert!(breaks.is_empty(), "a lawful erasure was reported as tampering: {breaks:?}");

    let again = store.purge("Observation", "nobody", &audit).await.expect("purge missing");
    assert!(!again.existed);

    store.drop_schema().await.expect("drop");
}

#[tokio::test]
async fn search_by_token_and_family_name() {
    let Some(store) = fresh(&["Observation", "Patient"]).await else {
        eprintln!("skipping: set FHIR_ORACLE_TEST_USER/_PASSWORD/_CONNECT to run");
        return;
    };
    store.init("search").await.expect("init");
    let audit = fhir_oracle_store::Audit::default();

    let patient = serde_json::json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{ "family": "Ærø", "given": ["Anna", "Marie"] }],
        "birthDate": "1974-12",
        "active": true
    });
    store.put(&patient, &audit).await.expect("put patient");

    let by_family = store
        .search("Patient", &[("family".to_string(), "Aero".to_string())], 10, 0)
        .await
        .expect("family search");
    assert_eq!(by_family, vec!["example".to_string()], "fold-insensitive family search");

    let by_active = store
        .search("Patient", &[("active".to_string(), "true".to_string())], 10, 0)
        .await
        .expect("token search");
    assert_eq!(by_active, vec!["example".to_string()]);

    store.drop_schema().await.expect("drop");
}

#[tokio::test]
async fn disclosures_are_recorded() {
    let Some(store) = fresh(&["Observation", "Patient"]).await else {
        eprintln!("skipping: set FHIR_ORACLE_TEST_USER/_PASSWORD/_CONNECT to run");
        return;
    };
    store.init("log").await.expect("init");
    assert_eq!(store.access_log_len().await.expect("len"), 0);

    let rec = fhir_oracle_store::AccessRecord {
        audit: fhir_oracle_store::Audit {
            actor: "dr-who".into(),
            actor_source: Some("header:X-Fhir-Oracle-Principal".into()),
            client: Some("10.0.0.1".into()),
            request_id: Some("req-1".into()),
            reason: Some("treatment".into()),
        },
        interaction: "read".into(),
        rtype: Some("Observation".into()),
        id: Some("obs-1".into()),
        version_id: Some(1),
        outcome: "ok".into(),
        result_count: Some(1),
    };
    store.log_access(&rec).await.expect("log");
    assert_eq!(store.access_log_len().await.expect("len"), 1);

    store.drop_schema().await.expect("drop");
}
