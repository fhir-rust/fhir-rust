//! Type- and system-level history (`history_page`) — the store half of
//! `fhir-loco`'s `SV2.17`.

use std::sync::Arc;

use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::{Audit, sqlite::SqliteStore};
use serde_json::json;

fn relmap() -> Option<RelMap> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-sqlite-map/assets/fhir-sqlite-relmap-r5.json.gz");
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok()
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/test-scratch")
        .join(format!("hp-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// A small installed store with history across two types, including a
/// deletion.
async fn seeded(name: &str) -> Option<SqliteStore> {
    let mut m = relmap()?;
    m.resources
        .retain(|k, _| k == "Patient" || k == "Observation");
    let db = scratch(name).join("fhir.sqlite");
    let store = SqliteStore::open(&db, Arc::new(m)).await.expect("open");
    store.init("sum").await.expect("init");
    let a = Audit::cli();
    store
        .put(&json!({"resourceType": "Patient", "id": "p1"}), &a)
        .await
        .expect("p1 v1");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "p1", "active": true}),
            &a,
        )
        .await
        .expect("p1 v2");
    store
        .put(
            &json!({"resourceType": "Observation", "id": "o1", "status": "final"}),
            &a,
        )
        .await
        .expect("o1 v1");
    store.delete("Observation", "o1", &a).await.expect("o1 del");
    Some(store)
}

/// Type-level: only that type, newest first, deletions included.
#[tokio::test]
async fn type_level_history_is_newest_first_and_keeps_deletions() {
    let Some(store) = seeded("type").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let rows = store
        .history_page(Some("Observation"), 100, None)
        .await
        .expect("page");
    assert_eq!(rows.len(), 2, "one create + one deletion");
    assert!(rows.iter().all(|(t, _, _)| t == "Observation"));
    // Newest first: the deletion precedes the create it deleted.
    assert_eq!(rows[0].2.op, 'D');
    assert!(rows[0].2.resource.is_none(), "a deletion has no content");
    assert_eq!(rows[1].2.op, 'C');

    let err = store.history_page(Some("Nope"), 10, None).await;
    assert!(
        err.is_err(),
        "an unknown type refuses rather than serving []"
    );
}

/// System-level spans types; `count` bounds it; `since` filters it.
#[tokio::test]
async fn system_level_history_spans_types_and_respects_count_and_since() {
    let Some(store) = seeded("system").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let all = store.history_page(None, 100, None).await.expect("page");
    assert_eq!(all.len(), 4, "two patient versions, two observation");
    let types: std::collections::BTreeSet<&str> = all.iter().map(|(t, _, _)| t.as_str()).collect();
    assert_eq!(types.len(), 2, "both types present: {types:?}");
    // Globally newest-first even across per-type queries.
    let stamps: Vec<&str> = all.iter().map(|r| r.2.last_updated.as_str()).collect();
    let mut sorted = stamps.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(stamps, sorted, "merged order must stay newest-first");

    assert_eq!(
        store.history_page(None, 2, None).await.expect("page").len(),
        2,
        "count bounds the slice"
    );

    // `_since` is at-or-after: everything since the newest stamp is that
    // stamp's rows; since just past it is nothing.
    let newest = all[0].2.last_updated.clone();
    let since = store
        .history_page(None, 100, Some(&newest))
        .await
        .expect("page");
    assert!(!since.is_empty(), "at-or-after includes the boundary");
    assert!(since.iter().all(|r| r.2.last_updated >= newest));
    let none = store
        .history_page(None, 100, Some("9999-01-01T00:00:00Z"))
        .await
        .expect("page");
    assert!(none.is_empty());
}
