//! Type- and system-level history (`history_page`) and the mount probe
//! (`installed_checksum`) — the store half of `fhir-loco`'s `SV2.17` and
//! its multi-port wiring. Needs `FHIR_POSTGRESQL_TEST_DB` (live server);
//! `scripts/db.sh up` prints it.

use std::sync::Arc;

use fhir_postgresql_map::model::RelMap;
use fhir_postgresql_store::Store;
use serde_json::json;

mod common;

fn relmap() -> Option<RelMap> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-postgresql-map/assets/fhir-postgresql-relmap-r5.json.gz");
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok()
}

async fn seeded(schema: &str) -> Option<Store> {
    let _db = common::test_db()?;
    let mut m = relmap()?;
    m.resources
        .retain(|k, _| k == "Patient" || k == "Observation");
    m.schema = schema.to_string();
    let cfg = fhir_postgresql_store::pg_config(None).expect("pg config");
    let store = Store::connect(cfg, Arc::new(m)).await.expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("hp-sum").await.expect("init");
    store
        .put(&json!({"resourceType": "Patient", "id": "p1"}))
        .await
        .expect("p1 v1");
    store
        .put(&json!({"resourceType": "Patient", "id": "p1", "active": true}))
        .await
        .expect("p1 v2");
    store
        .put(&json!({"resourceType": "Observation", "id": "o1", "status": "final"}))
        .await
        .expect("o1 v1");
    store.delete("Observation", "o1").await.expect("o1 del");
    Some(store)
}

/// The full slice: type scope, system scope, count, since, deletions —
/// the same assertions `fhir-sqlite`'s suite makes, against live Postgres.
#[tokio::test]
async fn history_page_serves_both_scopes_newest_first() {
    let Some(store) = seeded("loco_hp_a").await else {
        eprintln!("skipping: set FHIR_POSTGRESQL_TEST_DB to run");
        return;
    };
    let obs = store
        .history_page(Some("Observation"), 100, None)
        .await
        .expect("type page");
    assert_eq!(obs.len(), 2);
    assert_eq!(obs[0].2.op, 'D', "newest first: the deletion leads");
    assert!(obs[0].2.resource.is_none());
    assert!(
        store.history_page(Some("Nope"), 10, None).await.is_err(),
        "an unknown type refuses rather than serving []"
    );

    let all = store.history_page(None, 100, None).await.expect("system");
    assert_eq!(all.len(), 4, "two patient versions, two observation");
    let stamps: Vec<&str> = all.iter().map(|r| r.2.last_updated.as_str()).collect();
    let mut sorted = stamps.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(stamps, sorted, "merged order must stay newest-first");
    assert_eq!(
        store.history_page(None, 2, None).await.expect("page").len(),
        2
    );

    let newest = all[0].2.last_updated.clone();
    let since = store
        .history_page(None, 100, Some(&newest))
        .await
        .expect("since");
    assert!(!since.is_empty(), "at-or-after includes the boundary");
    assert!(
        store
            .history_page(None, 100, Some("9999-01-01T00:00:00Z"))
            .await
            .expect("since")
            .is_empty()
    );
}

/// The mount probe: `Some(checksum)` on an installed schema, `None` on an
/// uninstalled one — never an error for mere absence.
#[tokio::test]
async fn installed_checksum_distinguishes_installed_from_absent() {
    let Some(store) = seeded("loco_hp_b").await else {
        eprintln!("skipping: set FHIR_POSTGRESQL_TEST_DB to run");
        return;
    };
    assert_eq!(
        store.installed_checksum().await.expect("probe").as_deref(),
        Some("hp-sum")
    );
    store.drop_schema().await.expect("drop");
    assert_eq!(
        store.installed_checksum().await.expect("probe"),
        None,
        "absence is None, not an error"
    );
}
