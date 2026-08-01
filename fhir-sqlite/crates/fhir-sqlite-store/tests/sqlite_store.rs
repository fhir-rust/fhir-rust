//! The SQLite store, exercised against real databases (T64).
//!
//! Unlike the inherited PostgreSQL suites, these need no server and no
//! environment variables, so they always run. That is the point: the
//! PostgreSQL tests self-skip when a database is absent, which meant a green
//! run proved very little; here a green run proves the schema installed.

use std::sync::Arc;

use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::sqlite::SqliteStore;

fn relmap(version: &str) -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(format!("fhir-sqlite-relmap-{version}.json.gz"));
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok().map(Arc::new)
}

/// Scratch path under the workspace `target/`, not `TMPDIR`, matching the
/// convention the map crate's DDL tests use.
///
/// Panics if two tests ask for the same name. They would otherwise share a
/// directory — the path keys on the process id, which every test in a binary
/// shares — and silently delete each other's database as cargo runs them in
/// parallel. That failed as three unrelated-looking assertion errors before this
/// guard existed, so the collision is worth catching by name rather than by
/// symptom.
fn scratch(name: &str) -> std::path::PathBuf {
    use std::sync::{Mutex, OnceLock};
    static TAKEN: OnceLock<Mutex<std::collections::HashSet<String>>> = OnceLock::new();
    let taken = TAKEN.get_or_init(|| Mutex::new(std::collections::HashSet::new()));
    assert!(
        taken
            .lock()
            .expect("scratch registry")
            .insert(name.to_string()),
        "two tests both asked for the scratch name {name:?}; give each its own"
    );

    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/test-scratch")
        .join(format!("store-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

#[tokio::test]
async fn init_installs_the_full_r5_schema() {
    let Some(map) = relmap("r5") else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let dir = scratch("init-r5");
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("open");

    assert_eq!(
        store.installed_checksum().await.expect("checksum probe"),
        None,
        "a fresh database must not claim to have a schema"
    );

    let n = store.init("test-checksum").await.expect("init");
    assert!(n > 1000, "only {n} statements applied");

    let tables = store.table_count().await.expect("table count");
    assert!(
        tables > 7000,
        "expected the full R5 schema, got {tables} tables"
    );
    assert_eq!(
        store.installed_checksum().await.expect("checksum"),
        Some("test-checksum".to_string()),
        "init must record the map checksum it installed from"
    );

    // The schema lives in its own file, named for the version, so several FHIR
    // versions can be attached to one process (M14.15).
    assert!(
        dir.join("fhir-r5.sqlite").exists(),
        "expected a per-version database file"
    );

    store.drop_schema().await.expect("drop");
    assert_eq!(
        store
            .installed_checksum()
            .await
            .expect("checksum after drop"),
        None
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn init_is_all_or_nothing() {
    // SQLite's DDL is transactional, which is why the staged-schema install
    // PostgreSQL needs is not carried over (M14.16). A second `init` collides on
    // an existing table, and the failure must leave the schema as it was rather
    // than half-modified.
    let Some(map) = relmap("r3") else {
        eprintln!("skipping: no r3 relmap asset");
        return;
    };
    let dir = scratch("atomic-r3");
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("open");

    store.init("first").await.expect("first init");
    let before = store.table_count().await.expect("count");

    let err = store
        .init("second")
        .await
        .expect_err("installing over an existing schema must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("already exists") || msg.contains("installing schema"),
        "unhelpful error: {msg}"
    );

    let after = store.table_count().await.expect("count");
    assert_eq!(
        before, after,
        "a failed init changed the schema — the transaction did not roll back"
    );
    // And the original checksum is intact, not the one that failed.
    assert_eq!(
        store.installed_checksum().await.expect("checksum"),
        Some("first".to_string())
    );

    store.drop_schema().await.expect("drop");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn foreign_keys_are_enforced() {
    // SQLite ignores foreign keys unless asked. The child tables rely on
    // ON DELETE CASCADE to clear a resource's rows when it is rewritten, so a
    // store that forgot the pragma would silently orphan rows and only fail much
    // later, as an ordinal gap during reconstruction.
    let Some(map) = relmap("r3") else {
        eprintln!("skipping: no r3 relmap asset");
        return;
    };
    let dir = scratch("fk-r3");
    let db = dir.join("fhir.sqlite");
    let store = SqliteStore::open(&db, map).await.expect("open");
    store.init("fk").await.expect("init");
    let schema = store.schema().to_string();
    drop(store);

    let conn = rusqlite::Connection::open(&db).expect("reopen");
    conn.pragma_update(None, "foreign_keys", "ON")
        .expect("pragma");
    conn.execute_batch(&format!(
        "ATTACH DATABASE '{}' AS \"{schema}\"",
        dir.join(format!("fhir-{schema}.sqlite")).display()
    ))
    .expect("attach");

    let on: i64 = conn
        .query_row("PRAGMA foreign_keys", [], |r| r.get(0))
        .expect("read pragma");
    assert_eq!(on, 1, "foreign keys must be enabled");

    // A child row for a resource that does not exist must be refused.
    let err = conn.execute(
        &format!(
            "INSERT INTO \"{schema}\".\"patient_name\" (\"rid\", \"ords\") VALUES ('nope', '{{1}}')"
        ),
        [],
    );
    match err {
        Err(e) => {
            let m = e.to_string().to_lowercase();
            assert!(
                m.contains("foreign key"),
                "expected a foreign-key violation, got: {e}"
            );
        }
        Ok(_) => panic!("orphan child row was accepted; ON DELETE CASCADE cannot be relied on"),
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn wal_mode_and_durability_pragmas_are_set() {
    // WAL is what lets readers proceed during a write, which is the basis for
    // snapshot reads (M14.19); FULL synchronous is a deliberate choice for
    // health records carrying a hash chain.
    let Some(map) = relmap("r3") else {
        eprintln!("skipping: no r3 relmap asset");
        return;
    };
    let dir = scratch("pragmas");
    let db = dir.join("fhir.sqlite");
    let store = SqliteStore::open(&db, map).await.expect("open");
    store.init("pragmas").await.expect("init");
    drop(store);

    let conn = rusqlite::Connection::open(&db).expect("reopen");
    let mode: String = conn
        .query_row("PRAGMA journal_mode", [], |r| r.get(0))
        .expect("journal_mode");
    assert_eq!(mode.to_lowercase(), "wal", "WAL must persist in the file");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A resource exercising the shapes that actually break: repeating elements,
/// nested repeats, a decimal whose trailing zero must survive, and a choice type.
///
/// Parsed from text rather than built with `serde_json::json!`, and that is not
/// incidental. The `json!` macro turns `9.60` into an `f64` *literal* at compile
/// time, so the trailing zero is gone before any store sees it — M3.6's
/// guarantee is about preserving what was *parsed*, and it cannot be tested with
/// a value that was never parsed. Anything constructing resources in Rust from
/// float literals has already lost the precision.
fn observation() -> serde_json::Value {
    serde_json::from_str(
        r#"{
            "resourceType": "Observation",
            "id": "obs-1",
            "status": "final",
            "code": { "coding": [
                { "system": "http://loinc.org", "code": "2339-0", "display": "Glucose" },
                { "system": "http://example.org", "code": "gluc" }
            ] },
            "valueQuantity": { "value": 9.60, "unit": "mg/dL", "system": "http://unitsofmeasure.org" },
            "note": [ { "text": "first" }, { "text": "second" } ]
        }"#,
    )
    .expect("fixture parses")
}

#[tokio::test]
async fn put_then_get_round_trips_a_resource() {
    let Some(map) = relmap("r5") else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let dir = scratch("roundtrip");
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("open");
    store.init("rt").await.expect("init");

    let audit = fhir_sqlite_store::Audit::default();
    let src = observation();

    let put = store.put(&src, &audit).await.expect("put");
    assert_eq!(put.id, "obs-1");
    assert_eq!(put.version_id, 1);
    assert_eq!(put.kind, fhir_sqlite_store::sqlite::PutKind::Created);

    let got = store
        .get("Observation", "obs-1")
        .await
        .expect("get")
        .expect("resource should exist");

    // Semantic equality, field by field, so a failure names what drifted.
    for key in ["resourceType", "id", "status"] {
        assert_eq!(got.get(key), src.get(key), "{key} differs");
    }
    assert_eq!(
        got["code"]["coding"].as_array().map(Vec::len),
        Some(2),
        "repeating coding lost entries"
    );
    assert_eq!(got["code"]["coding"][0]["code"], "2339-0");
    assert_eq!(got["code"]["coding"][1]["code"], "gluc");
    assert_eq!(
        got["note"].as_array().map(Vec::len),
        Some(2),
        "repeating note lost entries"
    );
    assert_eq!(got["note"][0]["text"], "first");
    assert_eq!(got["note"][1]["text"], "second");

    // M3.6: a decimal's original textual precision must survive round-trip.
    // 9.60 must not come back as 9.6, which is precisely what REAL storage or
    // a float-based canonical form would have done.
    let v = got["valueQuantity"]["value"].to_string();
    assert_eq!(v, "9.60", "decimal precision lost: {v}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn rewrite_bumps_the_version_and_replaces_children() {
    let Some(map) = relmap("r5") else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let dir = scratch("rewrite");
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("open");
    store.init("rw").await.expect("init");
    let audit = fhir_sqlite_store::Audit::default();

    store.put(&observation(), &audit).await.expect("v1");

    // Second version has one fewer note; the removed child row must be gone,
    // not merely shadowed. This is what ON DELETE CASCADE buys.
    let mut v2 = observation();
    v2["note"] = serde_json::json!([{ "text": "only" }]);
    let put = store.put(&v2, &audit).await.expect("v2");
    assert_eq!(put.version_id, 2);
    assert_eq!(put.kind, fhir_sqlite_store::sqlite::PutKind::Updated);

    let got = store
        .get("Observation", "obs-1")
        .await
        .expect("get")
        .expect("exists");
    assert_eq!(
        got["note"].as_array().map(Vec::len),
        Some(1),
        "stale child rows survived the rewrite"
    );
    assert_eq!(got["note"][0]["text"], "only");

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn history_stores_the_canonical_bytes_the_chain_signed() {
    // The point of moving canonicalization into Rust (M14.15): what is stored is
    // exactly what was hashed, so a verifier does not depend on the database's
    // JSON rendering — and the same resource yields the same preimage on every
    // engine.
    let Some(map) = relmap("r5") else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let dir = scratch("canon");
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("open");
    store.init("canon").await.expect("init");
    let audit = fhir_sqlite_store::Audit::default();

    let src = observation();
    store.put(&src, &audit).await.expect("put");

    let stored = store
        .history_canon("Observation", "obs-1", 1)
        .await
        .expect("read history")
        .expect("history row exists");
    let expected = fhir_sqlite_map::canon::canonicalize(&src);
    assert_eq!(stored, expected, "stored bytes are not the canonical bytes");

    // Key order in the submission must not change what gets signed.
    let mut reordered = serde_json::Map::new();
    for k in [
        "note",
        "valueQuantity",
        "code",
        "status",
        "id",
        "resourceType",
    ] {
        reordered.insert(k.to_string(), src[k].clone());
    }
    assert_eq!(
        fhir_sqlite_map::canon::canonicalize(&serde_json::Value::Object(reordered)),
        expected,
        "canonical form depends on submitted key order"
    );
    // And the decimal survives into the signed bytes.
    assert!(
        stored.contains("9.60"),
        "signed bytes lost decimal precision"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Open a store on a fresh scratch database with the schema installed.
async fn fresh(name: &str, version: &str) -> Option<(std::path::PathBuf, SqliteStore)> {
    fresh_keyed(name, version, fhir_sqlite_store::chain::KeyRing::default()).await
}

/// Same, but with a key ring attached — keys live on the store now, so a keyed
/// test has to say so when it opens one rather than at each call.
async fn fresh_keyed(
    name: &str,
    version: &str,
    keys: fhir_sqlite_store::chain::KeyRing,
) -> Option<(std::path::PathBuf, SqliteStore)> {
    let map = relmap(version)?;
    let dir = scratch(name);
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("open")
        .with_chain_keys(keys);
    store.init(name).await.expect("init");
    Some((dir, store))
}

#[tokio::test]
async fn history_records_every_version_and_distinguishes_create_from_update() {
    let Some((dir, store)) = fresh("history", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();

    store.put(&observation(), &audit).await.expect("v1");
    let mut v2 = observation();
    v2["status"] = serde_json::json!("amended");
    store.put(&v2, &audit).await.expect("v2");

    let h = store
        .history("Observation", "obs-1")
        .await
        .expect("history");
    assert_eq!(h.len(), 2, "expected two versions");
    // Newest first.
    assert_eq!(h[0].version_id, 2);
    assert_eq!(h[1].version_id, 1);
    // The op column must tell a create from an update; inferring it from the
    // version number would be wrong for a resource recreated after deletion.
    assert_eq!(h[1].op, 'C', "first version should be a create");
    assert_eq!(h[0].op, 'U', "second version should be an update");

    // Each version's stored resource is the one submitted at the time.
    assert_eq!(h[1].resource.as_ref().unwrap()["status"], "final");
    assert_eq!(h[0].resource.as_ref().unwrap()["status"], "amended");

    // vread reaches an old version that the live tables no longer hold.
    let old = store
        .vread("Observation", "obs-1", 1)
        .await
        .expect("vread")
        .expect("version 1 exists");
    assert_eq!(old.version_id, 1);
    assert_eq!(old.op, 'C');
    assert_eq!(
        old.resource.as_ref().expect("v1 has content")["status"],
        "final"
    );
    assert!(
        store
            .vread("Observation", "obs-1", 99)
            .await
            .expect("vread")
            .is_none(),
        "a version that was never written must not be invented"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn delete_tombstones_and_keeps_history() {
    let Some((dir, store)) = fresh("delete", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();

    store.put(&observation(), &audit).await.expect("put");
    let v = store
        .delete("Observation", "obs-1", &audit)
        .await
        .expect("delete")
        .expect("something was deleted");
    assert_eq!(v, 2, "the tombstone is the next version");

    // Gone from the live view...
    assert!(
        store
            .get("Observation", "obs-1")
            .await
            .expect("get")
            .is_none(),
        "deleted resource still readable"
    );
    // ...but history survives, which is what makes the deletion auditable.
    let h = store
        .history("Observation", "obs-1")
        .await
        .expect("history");
    assert_eq!(h.len(), 2);
    assert_eq!(h[0].op, 'D');
    assert!(
        h[0].resource.is_none(),
        "a tombstone must not carry resource content"
    );
    // The pre-deletion version is still retrievable.
    assert!(
        store
            .vread("Observation", "obs-1", 1)
            .await
            .expect("vread")
            .is_some()
    );

    // Deleting again is not an error, it is a no-op.
    assert!(
        store
            .delete("Observation", "obs-1", &audit)
            .await
            .expect("second delete")
            .is_none()
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn verify_audit_accepts_an_untouched_chain() {
    let Some((dir, store)) = fresh("verify-ok", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();

    store.put(&observation(), &audit).await.expect("v1");
    let mut v2 = observation();
    v2["status"] = serde_json::json!("amended");
    store.put(&v2, &audit).await.expect("v2");
    store
        .delete("Observation", "obs-1", &audit)
        .await
        .expect("delete");

    let breaks = store.verify_audit().await.expect("verify");
    assert!(
        breaks.is_empty(),
        "a chain nobody touched reported breaks: {breaks:?}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn verify_audit_detects_a_tampered_history_row() {
    // The whole point of the chain. Edit a stored resource behind the store's
    // back and both chains must notice — this is also what proves the canonical
    // bytes are really what the hash covers.
    let Some((dir, store)) = fresh("verify-tamper", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();
    store.put(&observation(), &audit).await.expect("v1");
    let schema = store.schema().to_string();
    assert!(store.verify_audit().await.expect("verify").is_empty());
    drop(store);

    // The append-only trigger forbids UPDATE on history, so a tamperer has to
    // disable it — which is exactly the deliberate act M3.17 is designed to make
    // visible. Simulate that here.
    let att = dir.join(format!("fhir-{schema}.sqlite"));
    let conn = rusqlite::Connection::open(&att).expect("open attached db");
    let trg: String = conn
        .query_row(
            "SELECT name FROM sqlite_master WHERE type='trigger' AND tbl_name='observation_history' AND name LIKE '%upd%'",
            [],
            |r| r.get(0),
        )
        .expect("find the append-only update trigger");
    conn.execute_batch(&format!("DROP TRIGGER \"{trg}\""))
        .expect("drop trigger");
    let n = conn
        .execute(
            "UPDATE \"observation_history\" SET \"resource\" = replace(\"resource\", '\"final\"', '\"entered-in-error\"') WHERE \"version_id\" = 1",
            [],
        )
        .expect("tamper");
    assert_eq!(n, 1, "tamper did not modify a row");
    drop(conn);

    let map = relmap("r5").expect("relmap");
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("reopen");
    let breaks = store.verify_audit().await.expect("verify");

    assert!(!breaks.is_empty(), "tampering went undetected");
    // Both chains must flag it, not just one.
    let algs: std::collections::BTreeSet<&str> = breaks.iter().map(|b| b.algorithm).collect();
    assert!(
        algs.contains("sha256") && algs.contains("sha3-256"),
        "only {algs:?} noticed the change; both chains should"
    );
    for b in &breaks {
        assert_eq!(b.id, "obs-1");
        assert_eq!(b.version_id, 1);
        assert!(
            b.detail.contains("differ"),
            "unhelpful detail: {}",
            b.detail
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

/// A throwaway key, deliberately constant: it protects nothing in a test, and a
/// generated one would make failures unreproducible.
fn test_keys(id: &str) -> fhir_sqlite_store::chain::KeyRing {
    let k = fhir_sqlite_store::chain::ChainKey::from_hex(
        id,
        "4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f",
    )
    .expect("key parses");
    fhir_sqlite_store::chain::KeyRing::new(vec![k])
}

#[tokio::test]
async fn keyed_tags_are_written_and_verified() {
    let Some((dir, store)) = fresh_keyed("mac-ok", "r5", test_keys("ci")).await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();

    store.put(&observation(), &audit).await.expect("put");
    store
        .delete("Observation", "obs-1", &audit)
        .await
        .expect("delete");

    let breaks = store.verify_audit().await.expect("verify");
    assert!(breaks.is_empty(), "keyed chain reported breaks: {breaks:?}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn a_tampered_keyed_row_fails_its_tag_too() {
    // Before this, the MAC was written but never re-verified, so a keyed
    // deployment got strictly less checking than an unkeyed one — the tag was
    // decorative. This asserts it now carries weight.
    let Some((dir, store)) = fresh_keyed("mac-tamper", "r5", test_keys("ci")).await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();
    store.put(&observation(), &audit).await.expect("put");
    let schema = store.schema().to_string();
    drop(store);

    let att = dir.join(format!("fhir-{schema}.sqlite"));
    let conn = rusqlite::Connection::open(&att).expect("open");
    let trg: String = conn
        .query_row(
            "SELECT name FROM sqlite_master WHERE type='trigger' AND tbl_name='observation_history' AND name LIKE '%upd%'",
            [], |r| r.get(0),
        )
        .expect("find trigger");
    conn.execute_batch(&format!("DROP TRIGGER \"{trg}\""))
        .expect("drop trigger");
    conn.execute(
        "UPDATE \"observation_history\" SET \"actor\" = 'someone-else' WHERE \"version_id\" = 1",
        [],
    )
    .expect("tamper");
    drop(conn);

    let map = relmap("r5").expect("relmap");
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("reopen")
        .with_chain_keys(test_keys("ci"));
    let breaks = store.verify_audit().await.expect("verify");

    let algs: std::collections::BTreeSet<&str> = breaks.iter().map(|b| b.algorithm).collect();
    assert!(
        algs.contains("hmac-sha256"),
        "the keyed tag did not notice the change; saw {algs:?}"
    );
    // The hash chains cover the actor too, so they should also object — the tag
    // is defence in depth, not the only check.
    assert!(
        algs.contains("sha256"),
        "hash chain missed it; saw {algs:?}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn rows_signed_with_an_unheld_key_are_not_called_tampering() {
    // "I cannot check this" and "this was altered" are different claims. A store
    // verifying with the wrong key must say nothing rather than cry wolf, or
    // operators learn to ignore the report.
    //
    // Keys live on the store now, so testing a *different* verifier means
    // reopening with a different ring — verifying through the same handle would
    // only ever use the key that signed, which proves nothing.
    let Some((dir, store)) = fresh_keyed("mac-unheld", "r5", test_keys("original")).await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();
    store.put(&observation(), &audit).await.expect("put");
    drop(store);

    let db = dir.join("fhir.sqlite");

    // A ring holding a different key id cannot verify those rows, and must not
    // pretend the rows are broken.
    let map = relmap("r5").expect("relmap");
    let other = SqliteStore::open(&db, map)
        .await
        .expect("reopen")
        .with_chain_keys(test_keys("someone-elses"));
    let breaks = other.verify_audit().await.expect("verify");
    assert!(
        !breaks.iter().any(|b| b.algorithm == "hmac-sha256"),
        "rows signed by a key we do not hold were reported as tampering: {breaks:?}"
    );
    drop(other);

    // An empty ring behaves the same way: silence, not accusation.
    let map = relmap("r5").expect("relmap");
    let unkeyed = SqliteStore::open(&db, map).await.expect("reopen");
    let breaks = unkeyed.verify_audit().await.expect("verify");
    assert!(
        breaks.is_empty(),
        "verifying without keys reported breaks: {breaks:?}"
    );
    drop(unkeyed);

    // And the key that did sign them still verifies, so the rows really are
    // sound — without this the test would pass just as well on garbage.
    let map = relmap("r5").expect("relmap");
    let right = SqliteStore::open(&db, map)
        .await
        .expect("reopen")
        .with_chain_keys(test_keys("original"));
    assert!(
        right.verify_audit().await.expect("verify").is_empty(),
        "the signing key could not verify its own rows"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// An Observation with a given id, status, LOINC code and numeric value.
fn obs(id: &str, status: &str, code: &str, value: &str) -> serde_json::Value {
    serde_json::from_str(&format!(
        r#"{{
            "resourceType": "Observation",
            "id": "{id}",
            "status": "{status}",
            "code": {{ "coding": [ {{ "system": "http://loinc.org", "code": "{code}" }} ] }},
            "valueQuantity": {{ "value": {value}, "unit": "mg/dL" }}
        }}"#
    ))
    .expect("fixture parses")
}

/// Seed three Observations into a database of this test's own.
///
/// The name is a parameter, not a constant: `scratch` keys on the process id,
/// which every test in a binary shares, so two tests asking for the same scratch
/// name delete each other's database the moment cargo runs them in parallel.
async fn seeded(name: &str) -> Option<(std::path::PathBuf, SqliteStore)> {
    let (dir, store) = fresh(name, "r5").await?;
    let audit = fhir_sqlite_store::Audit::default();
    for (id, status, code, value) in [
        ("a", "final", "2339-0", "9"),
        ("b", "final", "2339-0", "10"),
        ("c", "amended", "1234-5", "100"),
    ] {
        store
            .put(&obs(id, status, code, value), &audit)
            .await
            .expect("seed");
    }
    Some((dir, store))
}

#[tokio::test]
async fn search_by_token_and_by_id() {
    let Some((dir, store)) = seeded("search-token").await else {
        return;
    };
    let p = |k: &str, v: &str| vec![(k.to_string(), v.to_string())];

    let r = store
        .search_full("Observation", &p("status", "final"), 10, 0, &[], true)
        .await
        .expect("search");
    assert_eq!(r.ids, vec!["a", "b"], "token search returned {:?}", r.ids);
    assert_eq!(r.total, Some(2), "_total should count matches");

    // A token with a system qualifier.
    let r = store
        .search_full(
            "Observation",
            &p("code", "http://loinc.org|1234-5"),
            10,
            0,
            &[],
            false,
        )
        .await
        .expect("search");
    assert_eq!(r.ids, vec!["c"]);

    // _id, and a comma list meaning OR.
    let r = store
        .search_full("Observation", &p("_id", "a,c"), 10, 0, &[], false)
        .await
        .expect("search");
    assert_eq!(r.ids, vec!["a", "c"]);

    // No match is an empty page, not an error.
    let r = store
        .search_full("Observation", &p("status", "cancelled"), 10, 0, &[], true)
        .await
        .expect("search");
    assert!(r.ids.is_empty());
    assert_eq!(r.total, Some(0));

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn numeric_search_compares_arithmetically_not_lexicographically() {
    // The case that makes this worth a test: values are stored as their exact
    // lexical form, because M3.6 requires that to survive round-trip. Compared
    // as text, "9" > "10" and "100" < "9" — so `gt9` would miss 10 and 100.
    // Only an explicit numeric comparison gets this right.
    let Some((dir, store)) = seeded("search-numeric").await else {
        return;
    };
    let p = |v: &str| vec![("value-quantity".to_string(), v.to_string())];

    let r = store
        .search_full("Observation", &p("gt9"), 10, 0, &[], false)
        .await
        .expect("search");
    let mut got = r.ids.clone();
    got.sort();
    assert_eq!(
        got,
        vec!["b", "c"],
        "gt9 should match 10 and 100, got {:?} — this is the lexicographic bug",
        r.ids
    );

    let r = store
        .search_full("Observation", &p("lt50"), 10, 0, &[], false)
        .await
        .expect("search");
    let mut got = r.ids.clone();
    got.sort();
    assert_eq!(got, vec!["a", "b"], "lt50 should match 9 and 10");

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn paging_and_total_are_independent() {
    let Some((dir, store)) = seeded("search-paging").await else {
        return;
    };
    // _total counts every match; the page is bounded by _count/_offset. A count
    // that shrank with the page would make paging impossible to drive.
    let first = store
        .search_full("Observation", &[], 2, 0, &[], true)
        .await
        .expect("page 1");
    assert_eq!(first.ids.len(), 2);
    assert_eq!(first.total, Some(3), "total must ignore paging");

    let second = store
        .search_full("Observation", &[], 2, 2, &[], true)
        .await
        .expect("page 2");
    assert_eq!(second.ids.len(), 1);
    assert_eq!(second.total, Some(3));

    // The two pages together cover the set exactly once.
    let mut all: Vec<String> = first.ids.into_iter().chain(second.ids).collect();
    all.sort();
    assert_eq!(all, vec!["a", "b", "c"]);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn search_values_are_bound_never_interpolated() {
    // The fuzz invariant the PostgreSQL original protects: an attacker-supplied
    // value must reach the database as a parameter, never as SQL text.
    let Some(map) = relmap("r5") else {
        return;
    };
    let rm = map.resources.get("Observation").expect("Observation");
    let nasty = "'; DROP TABLE patient; --";
    let q = fhir_sqlite_store::sqlite_search::build_search_sql(
        &map,
        rm,
        &[("status".to_string(), nasty.to_string())],
        10,
        0,
        &[],
        None,
    )
    .expect("build");

    assert!(
        !q.sql.contains("DROP TABLE"),
        "attacker value was interpolated into SQL:\n{}",
        q.sql
    );
    assert!(
        q.binds.iter().any(|b| b.contains("DROP TABLE")),
        "attacker value did not reach the binds"
    );
    // And the emitted SQL is SQLite's dialect, not the one it was forked from.
    assert!(
        !q.sql.contains("::text"),
        "PostgreSQL cast survived: {}",
        q.sql
    );
    assert!(!q.sql.contains("ILIKE"), "ILIKE survived: {}", q.sql);
    assert!(q.sql.contains('?'), "no bound placeholders: {}", q.sql);
}

#[tokio::test]
async fn disclosures_are_logged() {
    let Some((dir, store)) = fresh("access-log", "r5").await else {
        return;
    };
    assert_eq!(store.access_log_len().await.expect("len"), 0);

    let rec = fhir_sqlite_store::AccessRecord {
        audit: fhir_sqlite_store::Audit {
            actor: "dr-who".into(),
            actor_source: Some("header:X-Fhir-Sqlite-Principal".into()),
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
    // A read that found nothing is still a disclosure attempt worth recording.
    let mut miss = rec.clone();
    miss.outcome = "not-found".into();
    miss.result_count = Some(0);
    store.log_access(&miss).await.expect("log miss");

    assert_eq!(store.access_log_len().await.expect("len"), 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn purge_erases_history_and_leaves_a_verifiable_hole() {
    let Some((dir, store)) = fresh_keyed("purge", "r5", test_keys("ci")).await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();

    store.put(&observation(), &audit).await.expect("v1");
    let mut v2 = observation();
    v2["status"] = serde_json::json!("amended");
    store.put(&v2, &audit).await.expect("v2");
    assert_eq!(
        store
            .history("Observation", "obs-1")
            .await
            .expect("h")
            .len(),
        2
    );

    let report = store
        .purge("Observation", "obs-1", &audit)
        .await
        .expect("purge");
    assert!(report.existed);
    assert_eq!(report.versions_erased, 2, "both versions should be gone");

    // The resource is unreadable and its old versions are unrecoverable...
    assert!(
        store
            .get("Observation", "obs-1")
            .await
            .expect("get")
            .is_none()
    );
    assert!(
        store
            .vread("Observation", "obs-1", 1)
            .await
            .expect("vread")
            .is_none(),
        "an erased version must not still be readable"
    );

    // ...but the erasure left a record that it happened. A hole you can see is
    // the whole point: silence would be indistinguishable from never existing.
    let h = store
        .history("Observation", "obs-1")
        .await
        .expect("history");
    assert_eq!(h.len(), 1, "expected exactly the tombstone");
    assert_eq!(h[0].op, 'X');
    assert!(h[0].resource.is_none(), "a tombstone must carry no content");
    assert_eq!(h[0].version_id, 3, "the tombstone continues the numbering");

    // And the chain still verifies: a lawful erasure is not tampering.
    let breaks = store.verify_audit().await.expect("verify");
    assert!(
        breaks.is_empty(),
        "a lawful erasure was reported as tampering: {breaks:?}"
    );

    // Purging something unknown is a no-op, not an error.
    let again = store
        .purge("Observation", "nobody", &audit)
        .await
        .expect("purge missing");
    assert!(!again.existed);
    assert_eq!(again.versions_erased, 0);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn history_cannot_be_deleted_without_the_erasure_flag() {
    // The flag is what separates a sanctioned erasure from a stray DELETE. This
    // asserts the trigger is doing that work at the store's own schema, not just
    // in the DDL unit tests.
    let Some((dir, store)) = fresh("erasure-flag", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();
    store.put(&observation(), &audit).await.expect("put");
    let schema = store.schema().to_string();
    drop(store);

    let att = dir.join(format!("fhir-{schema}.sqlite"));
    let conn = rusqlite::Connection::open(&att).expect("open");

    let err = conn
        .execute(
            "DELETE FROM \"observation_history\" WHERE \"id\" = 'obs-1'",
            [],
        )
        .expect_err("history delete should be refused without the flag");
    assert!(
        err.to_string().contains("append-only"),
        "unexpected error: {err}"
    );

    // With the flag row present the same delete is permitted — and the flag is
    // an ordinary row, so it rolls back with a failed transaction rather than
    // leaking permission the way a session variable would.
    conn.execute(
        "INSERT INTO \"fhir_sqlite_erasure\" (\"token\") VALUES ('t')",
        [],
    )
    .expect("set flag");
    let n = conn
        .execute(
            "DELETE FROM \"observation_history\" WHERE \"id\" = 'obs-1'",
            [],
        )
        .expect("flagged delete should be permitted");
    assert_eq!(n, 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn a_tampered_erasure_tombstone_is_caught() {
    // Backs the claim that verifying the MAC against the row's stored prev_hash
    // buys real coverage: the tombstone is the only surviving evidence that an
    // erasure happened, so forging one must not be free.
    let Some((dir, store)) = fresh_keyed("tomb-tamper", "r5", test_keys("ci")).await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();
    store.put(&observation(), &audit).await.expect("put");
    store
        .purge("Observation", "obs-1", &audit)
        .await
        .expect("purge");
    assert!(store.verify_audit().await.expect("verify").is_empty());
    let schema = store.schema().to_string();
    drop(store);

    // Rewrite who did the erasing — the single most attractive edit on a
    // tombstone, since it is the only remaining record of responsibility.
    let att = dir.join(format!("fhir-{schema}.sqlite"));
    let conn = rusqlite::Connection::open(&att).expect("open");
    let trg: String = conn
        .query_row(
            "SELECT name FROM sqlite_master WHERE type='trigger' AND tbl_name='observation_history' AND name LIKE '%upd%'",
            [], |r| r.get(0),
        )
        .expect("find trigger");
    conn.execute_batch(&format!("DROP TRIGGER \"{trg}\""))
        .expect("drop trigger");
    conn.execute(
        "UPDATE \"observation_history\" SET \"actor\" = 'not-me' WHERE \"op\" = 'X'",
        [],
    )
    .expect("tamper");
    drop(conn);

    let map = relmap("r5").expect("relmap");
    let store = SqliteStore::open(dir.join("fhir.sqlite"), map)
        .await
        .expect("reopen")
        .with_chain_keys(test_keys("ci"));
    let breaks = store.verify_audit().await.expect("verify");
    assert!(
        breaks.iter().any(|b| b.algorithm == "hmac-sha256"),
        "a forged tombstone went unnoticed: {breaks:?}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn status_distinguishes_live_deleted_and_unknown() {
    // These are three different HTTP answers — 200, 410 Gone, 404 — and
    // collapsing the last two would tell a caller that a record it once held
    // never existed.
    let Some((dir, store)) = fresh("status", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();
    use fhir_sqlite_store::ResourceStatus as S;

    assert!(matches!(
        store.status("Observation", "obs-1").await.expect("status"),
        S::Unknown
    ));

    store.put(&observation(), &audit).await.expect("put");
    assert!(matches!(
        store.status("Observation", "obs-1").await.expect("status"),
        S::Active(1)
    ));

    store
        .delete("Observation", "obs-1", &audit)
        .await
        .expect("delete");
    match store.status("Observation", "obs-1").await.expect("status") {
        S::Deleted(v) => assert_eq!(v, 2, "deleted status should name the tombstone version"),
        other => panic!("expected Deleted, got {other:?}"),
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn if_match_refuses_a_stale_write() {
    // Optimistic concurrency. Without this a client that read v1, thought about
    // it, and wrote back would silently discard whatever landed in between.
    let Some((dir, store)) = fresh("if-match", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();

    let first = store
        .put_audited(&observation(), None, &audit)
        .await
        .expect("create");
    assert!(first.created);
    assert_eq!(first.version_id, 1);

    // Someone else writes v2.
    let mut v2 = observation();
    v2["status"] = serde_json::json!("amended");
    store.put(&v2, &audit).await.expect("v2");

    // Our write, still believing v1, must be refused rather than clobbering it.
    let err = store
        .put_audited(&observation(), Some(1), &audit)
        .await
        .expect_err("stale write should be refused");
    assert!(
        matches!(
            err,
            fhir_sqlite_store::StoreError::Conflict {
                expected: 1,
                found: 2
            }
        ),
        "unexpected error: {err:?}"
    );

    // Naming the current version lets it through.
    let ok = store
        .put_audited(&observation(), Some(2), &audit)
        .await
        .expect("current-version write");
    assert!(!ok.created);
    assert_eq!(ok.version_id, 3);

    // And the resource still exists exactly once at the new version.
    assert!(matches!(
        store.status("Observation", "obs-1").await.expect("status"),
        fhir_sqlite_store::ResourceStatus::Active(3)
    ));

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn get_all_preserves_order_and_absence() {
    // The caller is resolving _include targets and needs to know *which* ones
    // were missing, so a shorter list would lose the association.
    let Some((dir, store)) = seeded("get-all").await else {
        return;
    };
    let want = vec![
        ("Observation".to_string(), "a".to_string()),
        ("Observation".to_string(), "nope".to_string()),
        ("Observation".to_string(), "c".to_string()),
    ];
    let got = store.get_all(&want).await.expect("get_all");
    assert_eq!(got.len(), 3, "absence must hold its place");
    assert!(got[0].is_some());
    assert!(got[1].is_none(), "a missing id must be None in place");
    assert!(got[2].is_some());
    assert_eq!(got[0].as_ref().unwrap().version_id, 1);
    assert_eq!(got[2].as_ref().unwrap().resource["id"], "c");

    store.ping().await.expect("ping");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn vread_distinguishes_a_deleted_version_from_a_missing_one() {
    // 410 Gone and 404 are different answers, and the caller can only tell them
    // apart if vread reports the deletion rather than simply finding nothing.
    let Some((dir, store)) = fresh("vread-shapes", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();
    store.put(&observation(), &audit).await.expect("put");
    store
        .delete("Observation", "obs-1", &audit)
        .await
        .expect("delete");

    let v1 = store
        .vread("Observation", "obs-1", 1)
        .await
        .expect("vread")
        .expect("v1");
    assert_eq!(v1.op, 'C');
    assert!(v1.resource.is_some());

    let v2 = store
        .vread("Observation", "obs-1", 2)
        .await
        .expect("vread")
        .expect("v2 is the deletion");
    assert_eq!(v2.op, 'D');
    assert!(
        v2.resource.is_none(),
        "a deletion carries no content, and that is how it is recognised"
    );

    assert!(
        store
            .vread("Observation", "obs-1", 9)
            .await
            .expect("vread")
            .is_none(),
        "a version that was never written must not be invented"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn conditional_create_is_idempotent_and_refuses_ambiguity() {
    use fhir_sqlite_store::{CondCreate, CondDelete};
    let Some((dir, store)) = fresh("conditional", "r5").await else {
        return;
    };
    let audit = fhir_sqlite_store::Audit::default();
    let crit = |v: &str| vec![("status".to_string(), v.to_string())];

    // Nothing matches: it creates.
    match store
        .conditional_create_audited("Observation", &crit("final"), &observation(), &audit)
        .await
        .expect("first")
    {
        CondCreate::Created(p) => assert_eq!(p.version_id, 1),
        other => panic!("expected Created, got {other:?}"),
    }

    // Now one matches: the same request must return it rather than make a
    // second copy. That idempotence is the whole point of If-None-Exist.
    let mut second = observation();
    second["id"] = serde_json::json!("obs-2");
    match store
        .conditional_create_audited("Observation", &crit("final"), &second, &audit)
        .await
        .expect("second")
    {
        CondCreate::Existing(id) => assert_eq!(id, "obs-1"),
        other => panic!("expected Existing, got {other:?}"),
    }
    assert!(
        store
            .get("Observation", "obs-2")
            .await
            .expect("get")
            .is_none(),
        "a duplicate was created despite a match"
    );

    // Two match: refuse rather than guess.
    store.put(&second, &audit).await.expect("put obs-2");
    assert!(
        matches!(
            store
                .conditional_create_audited("Observation", &crit("final"), &observation(), &audit)
                .await
                .expect("third"),
            CondCreate::Multiple
        ),
        "two matches must be refused, not resolved by guessing"
    );

    // Conditional delete follows the same three-way shape.
    assert_eq!(
        store
            .conditional_delete_audited("Observation", &crit("final"), &audit)
            .await
            .expect("ambiguous delete"),
        CondDelete::Multiple,
        "deleting several records because a query was vague is not a guess to make"
    );
    assert_eq!(
        store
            .conditional_delete_audited("Observation", &crit("cancelled"), &audit)
            .await
            .expect("no match"),
        CondDelete::NoMatch
    );
    store
        .delete("Observation", "obs-2", &audit)
        .await
        .expect("narrow it");
    assert_eq!(
        store
            .conditional_delete_audited("Observation", &crit("final"), &audit)
            .await
            .expect("single delete"),
        CondDelete::Deleted
    );
    assert!(
        store
            .get("Observation", "obs-1")
            .await
            .expect("get")
            .is_none()
    );

    let _ = std::fs::remove_dir_all(&dir);
}
