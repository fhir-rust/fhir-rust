//! The MySQL store, exercised against a real server (T64).
//!
//! Needs `FHIR_MYSQL_TEST_DSN`; `scripts/db.sh up` prints it. Skips silently
//! without one, which is the convention the rest of the suite uses — and the
//! reason `doc/containers.md` warns that a live test finishing in 0.00s did not
//! run.

use std::sync::Arc;

use fhir_mariadb_map::model::RelMap;
use fhir_mariadb_store::mariadb::MariaDbStore;

fn dsn() -> Option<String> {
    std::env::var("FHIR_MYSQL_TEST_DSN").ok()
}

fn relmap(version: &str) -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(format!("fhir-mariadb-relmap-{version}.json.gz"));
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok().map(Arc::new)
}

/// A map trimmed to the resource types a test actually uses.
///
/// Creating every InnoDB table takes tens of minutes — slow enough that the DDL
/// suite samples too. Named rather than counted: taking "the first six" is
/// alphabetical, so it silently excludes `Observation` and every test then fails
/// with `unknown resource type`, which looks like a store bug and is not.
fn sampled(version: &str, schema: &str, want: &[&str]) -> Option<Arc<RelMap>> {
    let mut m = (*relmap(version)?).clone();
    m.resources.retain(|k, _| want.contains(&k.as_str()));
    assert!(
        !m.resources.is_empty(),
        "none of {want:?} are in the {version} map"
    );
    m.schema = schema.to_string();
    Some(Arc::new(m))
}

async fn fresh(schema: &str) -> Option<MariaDbStore> {
    let map = sampled("r5", schema, &["Observation", "Patient"])?;
    let store = MariaDbStore::connect(&dsn()?, map).await.expect("connect");
    store.drop_schema().await.expect("drop");
    Some(store)
}

#[tokio::test]
async fn connect_fails_loudly_on_a_bad_dsn() {
    if dsn().is_none() {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    }
    let map = relmap("r5").expect("relmap");
    // A store that constructs successfully and only fails at first use is a
    // worse diagnostic than a connection error at startup.
    let err = MariaDbStore::connect("mysql://root@127.0.0.1:1/", map)
        .await
        .expect_err("connecting to a dead port should fail");
    assert!(
        matches!(err, fhir_mariadb_store::StoreError::Db(_)),
        "unexpected error: {err:?}"
    );
}

#[tokio::test]
async fn init_installs_tables_and_triggers() {
    let Some(store) = fresh("fhir_mariadb_store_init").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };

    assert_eq!(
        store.installed_checksum().await.expect("probe"),
        None,
        "a database with no schema must not claim to have one"
    );

    let n = store.init("test-checksum").await.expect("init");
    assert!(n > 50, "only {n} statements applied");

    let tables = store.table_count().await.expect("count");
    assert!(tables > 20, "expected a schema, got {tables} tables");

    // Triggers are the enforcement behind M3.17. A schema with its tables but
    // not its triggers would look healthy while guaranteeing nothing, so they
    // are counted separately rather than assumed to follow.
    let triggers = store.trigger_count().await.expect("triggers");
    assert!(triggers > 0, "no append-only triggers were installed");
    assert_eq!(
        triggers % 2,
        0,
        "triggers come in pairs (UPDATE and DELETE per history table), got {triggers}"
    );

    assert_eq!(
        store.installed_checksum().await.expect("checksum"),
        Some("test-checksum".to_string()),
        "init must record the map checksum it installed from"
    );

    store.ping().await.expect("ping");
    store.drop_schema().await.expect("drop");
    assert_eq!(
        store.installed_checksum().await.expect("after drop"),
        None,
        "dropping the schema must clear the installed marker"
    );
    store.close().await.expect("close");
}

#[tokio::test]
async fn a_failed_install_reports_how_far_it_got() {
    // MySQL commits DDL implicitly, so a failed install leaves a partial schema
    // (M14.22). That is a real regression from PostgreSQL, and the error has to
    // say so — an operator cleaning up needs to know whether the database is
    // empty or half-built.
    let Some(store) = fresh("fhir_mariadb_store_partial").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("first").await.expect("first install");
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
    assert!(
        msg.contains("partial") && msg.contains("M14.22"),
        "the error must say the schema is left partial and cite why:\n{msg}"
    );

    // And the tables really are still there — the claim in the message is true.
    assert_eq!(
        store.table_count().await.expect("count"),
        before,
        "table count changed after a failed install"
    );

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}

/// Parsed from text, not built with `serde_json::json!`: the macro turns `9.60`
/// into an `f64` literal at compile time, so the trailing zero would be gone
/// before any store saw it. M3.6 is about preserving what was *parsed*.
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
async fn put_then_get_round_trips_a_resource() {
    let Some(store) = fresh("fhir_mariadb_store_rt").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("rt").await.expect("init");
    let audit = fhir_mariadb_store::Audit::default();

    let src = observation("obs-1", "final");
    let put = store.put(&src, &audit).await.expect("put");
    assert_eq!(put.id, "obs-1");
    assert_eq!(put.version_id, 1);
    assert_eq!(put.kind, fhir_mariadb_store::mariadb::PutKind::Created);

    let got = store
        .get("Observation", "obs-1")
        .await
        .expect("get")
        .expect("present");

    for key in ["resourceType", "id", "status"] {
        assert_eq!(got.get(key), src.get(key), "{key} differs");
    }
    assert_eq!(
        got["code"]["coding"].as_array().map(Vec::len),
        Some(2),
        "repeating coding lost entries"
    );
    assert_eq!(got["code"]["coding"][1]["code"], "gluc");
    assert_eq!(got["note"].as_array().map(Vec::len), Some(2));
    assert_eq!(got["note"][1]["text"], "second");

    // M3.6: 9.60 must not come back as 9.6. TEXT storage is what makes this
    // possible; DECIMAL would have padded it and DOUBLE would have rounded it.
    assert_eq!(
        got["valueQuantity"]["value"].to_string(),
        "9.60",
        "decimal precision lost"
    );

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}

#[tokio::test]
async fn rewrite_replaces_children_and_bumps_the_version() {
    let Some(store) = fresh("fhir_mariadb_store_rw").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("rw").await.expect("init");
    let audit = fhir_mariadb_store::Audit::default();

    store
        .put(&observation("obs-1", "final"), &audit)
        .await
        .expect("v1");

    // One fewer note: the removed child row must be gone, not shadowed. That is
    // what ON DELETE CASCADE buys, and it only works because the foreign keys
    // are real.
    let mut v2 = observation("obs-1", "amended");
    v2["note"] = serde_json::from_str(r#"[{"text":"only"}]"#).expect("json");
    let put = store.put(&v2, &audit).await.expect("v2");
    assert_eq!(put.version_id, 2);
    assert_eq!(put.kind, fhir_mariadb_store::mariadb::PutKind::Updated);

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
    assert_eq!(got["note"][0]["text"], "only");
    assert_eq!(got["status"], "amended");

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}

#[tokio::test]
async fn extensions_get_distinct_surrogate_keys() {
    // The surrogate is the primary key on Ext/Deep (M14.12). If two rows hashed
    // alike the second would be rejected as a duplicate, and the loss would look
    // like a shredding bug rather than a key collision — so this checks that a
    // resource with several extensions round-trips whole.
    let Some(store) = fresh("fhir_mariadb_store_ext").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("ext").await.expect("init");
    let audit = fhir_mariadb_store::Audit::default();

    let src: serde_json::Value = serde_json::from_str(
        r#"{
            "resourceType": "Observation",
            "id": "ext-1",
            "status": "final",
            "extension": [
                { "url": "http://example.org/a", "valueString": "one" },
                { "url": "http://example.org/b", "valueString": "two" },
                { "url": "http://example.org/c", "valueInteger": 3 }
            ]
        }"#,
    )
    .expect("fixture parses");

    store.put(&src, &audit).await.expect("put");
    let got = store
        .get("Observation", "ext-1")
        .await
        .expect("get")
        .expect("present");
    assert_eq!(
        got["extension"].as_array().map(Vec::len),
        Some(3),
        "an extension was lost — likely a surrogate-key collision"
    );
    assert_eq!(got["extension"][0]["valueString"], "one");
    assert_eq!(got["extension"][2]["valueInteger"], 3);

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}

/// A throwaway key, deliberately constant: it protects nothing in a test, and a
/// generated one would make failures unreproducible.
fn test_keys(id: &str) -> fhir_mariadb_store::chain::KeyRing {
    let k = fhir_mariadb_store::chain::ChainKey::from_hex(
        id,
        "4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f4f",
    )
    .expect("key parses");
    fhir_mariadb_store::chain::KeyRing::new(vec![k])
}

async fn fresh_keyed(
    schema: &str,
    keys: fhir_mariadb_store::chain::KeyRing,
) -> Option<MariaDbStore> {
    let map = sampled("r5", schema, &["Observation", "Patient"])?;
    let store = MariaDbStore::connect(&dsn()?, map)
        .await
        .expect("connect")
        .with_chain_keys(keys);
    store.drop_schema().await.expect("drop");
    Some(store)
}

#[tokio::test]
async fn history_and_vread_distinguish_a_deletion() {
    let Some(store) = fresh("fhir_mariadb_store_hist").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("hist").await.expect("init");
    let audit = fhir_mariadb_store::Audit::default();

    store
        .put(&observation("obs-1", "final"), &audit)
        .await
        .expect("v1");
    store
        .put(&observation("obs-1", "amended"), &audit)
        .await
        .expect("v2");
    let tomb = store
        .delete("Observation", "obs-1", &audit)
        .await
        .expect("delete")
        .expect("something was deleted");
    assert_eq!(tomb, 3);

    let h = store
        .history("Observation", "obs-1")
        .await
        .expect("history");
    assert_eq!(h.len(), 3, "expected create, update, delete");
    assert_eq!(h[0].version_id, 3);
    assert_eq!(h[0].op, 'D');
    assert!(h[0].resource.is_none(), "a tombstone carries no content");
    // 'C' and 'U' are distinct, and the op is part of the hashed preimage, so it
    // cannot be corrected after the fact.
    assert_eq!(h[2].op, 'C');
    assert_eq!(h[1].op, 'U');
    assert_eq!(h[2].resource.as_ref().unwrap()["status"], "final");

    // Gone from the live view, still reachable by version.
    assert!(
        store
            .get("Observation", "obs-1")
            .await
            .expect("get")
            .is_none()
    );
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
    assert!(
        store
            .vread("Observation", "obs-1", 9)
            .await
            .expect("vread")
            .is_none(),
        "a version never written must not be invented"
    );

    // Deleting again is a no-op, not an error.
    assert!(
        store
            .delete("Observation", "obs-1", &audit)
            .await
            .expect("again")
            .is_none()
    );

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}

#[tokio::test]
async fn verify_audit_accepts_a_clean_chain_and_catches_tampering() {
    let Some(store) = fresh_keyed("fhir_mariadb_store_verify", test_keys("ci")).await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("verify").await.expect("init");
    let audit = fhir_mariadb_store::Audit::default();

    store
        .put(&observation("obs-1", "final"), &audit)
        .await
        .expect("v1");
    store
        .put(&observation("obs-1", "amended"), &audit)
        .await
        .expect("v2");
    store
        .delete("Observation", "obs-1", &audit)
        .await
        .expect("delete");

    let breaks = store.verify_audit().await.expect("verify");
    assert!(
        breaks.is_empty(),
        "a chain nobody touched reported breaks: {breaks:?}"
    );

    // Now tamper. The append-only trigger forbids UPDATE on history, so a
    // tamperer has to drop it first — which is exactly the deliberate act M3.17
    // is designed to make visible.
    let dsn = dsn().expect("dsn");
    let raw = MariaDbStore::connect(
        &dsn,
        sampled("r5", "fhir_mariadb_store_verify", &["Observation"]).expect("map"),
    )
    .await
    .expect("connect");
    raw.exec_raw(
        "DROP TRIGGER `fhir_mariadb_store_verify`.`observation_history_append_only_upd_trg_ix`",
    )
    .await
    .expect("drop trigger");
    raw.exec_raw(
        "UPDATE `fhir_mariadb_store_verify`.`observation_history` \
         SET `actor` = 'not-me' WHERE `version_id` = 1",
    )
    .await
    .expect("tamper");

    let breaks = store.verify_audit().await.expect("verify");
    assert!(!breaks.is_empty(), "tampering went undetected");
    let algs: std::collections::BTreeSet<&str> = breaks.iter().map(|b| b.algorithm).collect();
    assert!(
        algs.contains("sha256") && algs.contains("sha3-256"),
        "both hash chains should object; saw {algs:?}"
    );
    assert!(
        algs.contains("hmac-sha256"),
        "the keyed tag should object too; saw {algs:?}"
    );

    store.drop_schema().await.expect("drop");
    raw.close().await.expect("close raw");
    store.close().await.expect("close");
}

/// An Observation with a given status, LOINC code, and numeric value.
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

#[tokio::test]
async fn search_by_token_number_and_paging() {
    let Some(store) = fresh("fhir_mariadb_store_search").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("search").await.expect("init");
    let audit = fhir_mariadb_store::Audit::default();
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

    let p = |k: &str, v: &str| vec![(k.to_string(), v.to_string())];

    let r = store
        .search_full("Observation", &p("status", "final"), 10, 0, &[], true)
        .await
        .expect("token search");
    assert_eq!(r.ids, vec!["a", "b"]);
    assert_eq!(r.total, Some(2));

    // A token with a system qualifier.
    let r = store
        .search("Observation", &p("code", "http://loinc.org|1234-5"), 10, 0)
        .await
        .expect("qualified token");
    assert_eq!(r, vec!["c"]);

    // The case that makes numeric search worth testing: values are stored in
    // their exact lexical form because M3.6 demands it, and compared as text
    // "9" > "10", so gt9 would miss both larger values.
    let mut got = store
        .search("Observation", &p("value-quantity", "gt9"), 10, 0)
        .await
        .expect("numeric search");
    got.sort();
    assert_eq!(
        got,
        vec!["b", "c"],
        "gt9 should match 10 and 100 — this is the lexicographic trap"
    );

    let mut got = store
        .search("Observation", &p("value-quantity", "lt50"), 10, 0)
        .await
        .expect("numeric search");
    got.sort();
    assert_eq!(got, vec!["a", "b"]);

    // _total counts every match; the page is bounded separately, or a client
    // cannot tell how far it has to page.
    let r = store
        .search_full("Observation", &[], 2, 0, &[], true)
        .await
        .expect("page 1");
    assert_eq!(r.ids.len(), 2);
    assert_eq!(r.total, Some(3), "_total must ignore paging");
    let r2 = store
        .search_full("Observation", &[], 2, 2, &[], true)
        .await
        .expect("page 2");
    assert_eq!(r2.ids.len(), 1);
    assert_eq!(r2.total, Some(3));
    let mut all: Vec<String> = r.ids.into_iter().chain(r2.ids).collect();
    all.sort();
    assert_eq!(all, vec!["a", "b", "c"], "pages must cover the set once");

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}

#[tokio::test]
async fn search_values_are_bound_never_interpolated() {
    // The invariant the PostgreSQL fuzz target protects: an attacker-supplied
    // value must reach the database as a parameter, never as SQL text.
    let Some(map) = sampled("r5", "unused", &["Observation"]) else {
        return;
    };
    let rm = map.resources.get("Observation").expect("Observation");
    let nasty = "'; DROP TABLE patient; --";
    let q = fhir_mariadb_store::mariadb_search::build_search_sql(
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
    // And the emitted SQL is MySQL's dialect, not the one it was forked from.
    assert!(
        !q.sql.contains("::text"),
        "PostgreSQL cast survived: {}",
        q.sql
    );
    assert!(!q.sql.contains("ILIKE"), "ILIKE survived: {}", q.sql);
    assert!(
        !q.sql.contains('"'),
        "double-quoted identifier survived: {}",
        q.sql
    );
    assert!(
        q.sql.contains('`'),
        "identifiers are not backquoted: {}",
        q.sql
    );
    assert!(q.sql.contains('?'), "no bound placeholders: {}", q.sql);
}

#[tokio::test]
async fn purge_erases_history_and_leaves_a_verifiable_hole() {
    let Some(store) = fresh_keyed("fhir_mariadb_store_purge", test_keys("ci")).await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("purge").await.expect("init");
    let audit = fhir_mariadb_store::Audit::default();

    store
        .put(&observation("obs-1", "final"), &audit)
        .await
        .expect("v1");
    store
        .put(&observation("obs-1", "amended"), &audit)
        .await
        .expect("v2");
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

    // Unreadable and unrecoverable...
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

    // ...but the erasure left evidence that it happened. A hole you can see is
    // the point: silence would be indistinguishable from never existing.
    let h = store
        .history("Observation", "obs-1")
        .await
        .expect("history");
    assert_eq!(h.len(), 1, "expected exactly the tombstone");
    assert_eq!(h[0].op, 'X');
    assert!(h[0].resource.is_none());
    assert_eq!(h[0].version_id, 3, "the tombstone continues the numbering");

    // A lawful erasure is not tampering.
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

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}

#[tokio::test]
async fn history_cannot_be_deleted_without_the_erasure_flag() {
    // The session variable is what separates a sanctioned erasure from a stray
    // DELETE. It is per-connection, which is exactly why `purge` holds one
    // connection for the whole operation — a flag set on a pooled connection and
    // a delete issued on another would look like a broken trigger.
    let Some(store) = fresh("fhir_mariadb_store_flag").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("flag").await.expect("init");
    let audit = fhir_mariadb_store::Audit::default();
    store
        .put(&observation("obs-1", "final"), &audit)
        .await
        .expect("put");

    let err = store
        .exec_raw(
            "DELETE FROM `fhir_mariadb_store_flag`.`observation_history` WHERE `id` = 'obs-1'",
        )
        .await
        .expect_err("history delete should be refused without the flag");
    assert!(
        err.to_string().contains("append-only"),
        "unexpected error: {err}"
    );

    // The resource and its history are untouched.
    assert_eq!(
        store
            .history("Observation", "obs-1")
            .await
            .expect("h")
            .len(),
        1
    );

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}

#[tokio::test]
async fn disclosures_are_recorded() {
    let Some(store) = fresh("fhir_mariadb_store_log").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.init("log").await.expect("init");
    assert_eq!(store.access_log_len().await.expect("len"), 0);

    let rec = fhir_mariadb_store::AccessRecord {
        audit: fhir_mariadb_store::Audit {
            actor: "dr-who".into(),
            actor_source: Some("header:X-Fhir-Mysql-Principal".into()),
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
    store.log_access_batch(&[miss]).await.expect("batch");

    assert_eq!(store.access_log_len().await.expect("len"), 2);

    store.drop_schema().await.expect("drop");
    store.close().await.expect("close");
}
