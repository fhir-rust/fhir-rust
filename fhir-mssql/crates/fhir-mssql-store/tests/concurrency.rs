//! Adversarial concurrency (spec T11.6), ported from the MySQL suite.
//!
//! This port had no concurrency suite at all. Writing one for `fhir-mysql`
//! found real defects, so the expectation here was not "confirm it works" but
//! "find out".
//!
//! Two properties are in scope, and two are not:
//!
//! - **Torn reads (R4.5).** A read spans a base table and many child tables.
//!   `get` wraps the read in `BEGIN TRANSACTION … ROLLBACK TRANSACTION` for
//!   exactly this reason — statements issued outside a transaction each land
//!   on whatever the table looks like at that instant, and a concurrent write
//!   between them would reconstruct a resource that never existed.
//! - **Version assignment (H5.4).** N racing writers must produce N distinct
//!   consecutive versions and a chain that still verifies. `put` takes
//!   `WITH (UPDLOCK, ROWLOCK)` on the base row before reading the chain tip,
//!   which is the serialisation this test is checking for.
//!
//! Not tested here, because this port does not implement them: conditional
//! create/delete, and version-guarded (`If-Match`) updates. There is no
//! `put_audited` and no `expected_version` anywhere in the crate. Their
//! absence belongs in the conformance matrix, not in a test asserting a
//! missing feature.
//!
//! Needs `FHIR_MSSQL_TEST_DSN`; `scripts/db.sh up` prints it. Run with
//! `--test-threads=1` alongside the rest of this port's live suite (see
//! `mssql_store.rs`) — each test here is itself internally concurrent, and
//! running several of *those* concurrently against one small container is
//! its own way to deadlock.

use std::sync::Arc;

use fhir_mssql_map::model::RelMap;
use fhir_mssql_store::mssql::MsSqlStore;
use serde_json::{Value, json};

mod common;

fn dsn() -> Option<String> {
    common::dsn().map(str::to_string)
}

fn relmap() -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-mssql-map/assets/fhir-mssql-relmap-r5.json.gz");
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok().map(Arc::new)
}

/// Only `Patient`: creating every table takes a long time.
fn sampled(schema: &str) -> Option<Arc<RelMap>> {
    let mut m = (*relmap()?).clone();
    m.resources.retain(|k, _| k == "Patient");
    assert!(!m.resources.is_empty(), "Patient missing from the r5 map");
    m.schema = schema.to_string();
    Some(Arc::new(m))
}

async fn fresh(schema: &str) -> Option<Arc<MsSqlStore>> {
    let store = MsSqlStore::connect(&dsn()?, sampled(schema)?)
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("concurrency-checksum").await.expect("init");
    Some(Arc::new(store))
}

/// Every field encodes the same generation, so an incoherent mix is
/// detectable without knowing which generation a reader should have seen.
fn patient(id: &str, n: usize) -> Value {
    json!({
        "resourceType": "Patient",
        "id": id,
        "active": n.is_multiple_of(2),
        "name": [{
            "family": format!("Family{n}"),
            "given": [format!("Given{n}"), format!("Middle{n}")]
        }],
        "telecom": [{"system": "phone", "value": format!("{n:06}")}]
    })
}

fn generation(p: &Value) -> Option<usize> {
    p.pointer("/name/0/family")?
        .as_str()?
        .trim_start_matches("Family")
        .parse()
        .ok()
}

/// `family` is on a child table, `given` on a grandchild, `telecom` on
/// another child, `active` on the base table — so a torn read shows up as a
/// disagreement between them.
fn assert_coherent(p: &Value) {
    let n = generation(p).expect("family present and well-formed");
    assert_eq!(
        p.pointer("/name/0/given/0").and_then(Value::as_str),
        Some(format!("Given{n}").as_str()),
        "torn read: {p}"
    );
    assert_eq!(
        p.pointer("/name/0/given/1").and_then(Value::as_str),
        Some(format!("Middle{n}").as_str()),
        "torn read: {p}"
    );
    assert_eq!(
        p.pointer("/telecom/0/value").and_then(Value::as_str),
        Some(format!("{n:06}").as_str()),
        "torn read: {p}"
    );
    assert_eq!(
        p.get("active").and_then(Value::as_bool),
        Some(n.is_multiple_of(2)),
        "torn read: {p}"
    );
}

/// R4.5 — a reader must never observe a half-applied write.
///
/// Unlike SQLite, this store is pool-backed, so a reader and a writer land on
/// different connections even through one handle. No second handle is needed
/// to reach the race.
///
/// **Fixed, in two tries.** SQL Server's default `READ COMMITTED` isolation
/// does not give a multi-statement transaction one snapshot the way
/// PostgreSQL's/MySQL's default does. The first fix attempted —
/// `READ_COMMITTED_SNAPSHOT` alone — was tried live and **did not** stop the
/// torn read: it only gives each individual statement its own snapshot, not
/// the whole transaction one. What actually works is `get` issuing
/// `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` before its `BEGIN TRANSACTION`,
/// which needs `ALLOW_SNAPSHOT_ISOLATION` at the database level
/// (`scripts/db.sh`'s `post_ready`) — and needs a database this port can run
/// `ALTER DATABASE` against at all, which is why the DSN now names one
/// (`database=fhir_mssql`) rather than landing in `master`, where the option
/// cannot be set. See the long comment on `get` in `mssql.rs` and `M14.25` in
/// `spec/14-mssql-dialect.md`.
#[tokio::test]
async fn reads_never_tear_under_concurrent_writes() {
    let Some(store) = fresh("fhir_mssql_conc_torn").await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
        return;
    };
    store
        .put(&patient("torn", 0), &fhir_mssql_store::Audit::default())
        .await
        .expect("seed");

    let writer = tokio::spawn({
        let store = store.clone();
        async move {
            for n in 1..=120 {
                store
                    .put(&patient("torn", n), &fhir_mssql_store::Audit::default())
                    .await
                    .expect("write");
            }
        }
    });

    let reader = tokio::spawn({
        let store = store.clone();
        async move {
            let mut seen = 0usize;
            let mut generations = std::collections::HashSet::new();
            for _ in 0..400 {
                if let Some(got) = store.get("Patient", "torn").await.expect("read") {
                    assert_coherent(&got);
                    generations.insert(generation(&got).expect("generation"));
                    seen += 1;
                }
                tokio::task::yield_now().await;
            }
            (seen, generations.len())
        }
    });

    writer.await.expect("writer");
    let (seen, distinct) = reader.await.expect("reader");

    // Without these the test passes trivially on a reader that saw nothing,
    // or one that only ever saw the seed (T11.12).
    assert!(seen > 0, "the reader never saw the resource at all");
    assert!(
        distinct > 1,
        "the reader only ever saw one generation, so it never raced the writer \
         and this test proved nothing"
    );

    store.drop_schema().await.expect("drop");
}

/// H5.4 — racing writers produce distinct consecutive versions, and the
/// chain still verifies.
///
/// The chain digest of version *n* commits to the digest of *n−1*. Two
/// writers that read the same tip append two rows claiming the same
/// predecessor, and the chain then verifies for neither. Asserting versions
/// alone would miss that; asserting the chain alone would not localise it.
#[tokio::test]
async fn racing_writers_get_distinct_versions_and_a_verifiable_chain() {
    let Some(store) = fresh("fhir_mssql_conc_ver").await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
        return;
    };
    store
        .put(&patient("ver", 0), &fhir_mssql_store::Audit::default())
        .await
        .expect("seed");

    const WRITERS: usize = 8;
    let racers: Vec<_> = (1..=WRITERS)
        .map(|n| {
            let store = store.clone();
            tokio::spawn(async move {
                store
                    .put(&patient("ver", n), &fhir_mssql_store::Audit::default())
                    .await
            })
        })
        .collect();

    let mut versions = Vec::new();
    let mut refused = 0usize;
    for r in racers {
        match r.await.expect("join") {
            Ok(p) => versions.push(p.version_id),
            // A writer refused under contention is acceptable — a lost
            // update silently accepted is not. Count them so the assertion
            // below can tell the two apart.
            Err(_) => refused += 1,
        }
    }
    versions.sort_unstable();
    eprintln!(
        "{} succeeded, {refused} refused: {versions:?}",
        versions.len()
    );

    // Every one of these matters, and the first alone would pass trivially
    // if seven of eight writers had been refused — the "floor of at least 20
    // tolerates losing four of twenty-four" failure T11.11 warns about.
    let distinct: std::collections::HashSet<_> = versions.iter().copied().collect();
    assert_eq!(
        distinct.len(),
        versions.len(),
        "two writers were handed the same version_id: {versions:?}"
    );
    assert_eq!(
        versions.len() + refused,
        WRITERS,
        "a write neither succeeded nor was refused, so one vanished"
    );
    assert!(
        versions.len() > WRITERS / 2,
        "only {} of {WRITERS} writers made progress ({refused} refused); this \
         port serialises so hard the test above proves little",
        versions.len()
    );
    assert_eq!(
        versions,
        (2..=(versions.len() as i64 + 1)).collect::<Vec<_>>(),
        "successful versions must be consecutive from the seed: {versions:?}"
    );

    let history = store.history("Patient", "ver").await.expect("history");
    assert_eq!(
        history.len(),
        versions.len() + 1,
        "history rows must match the writes that succeeded"
    );

    let breaks = store.verify_audit().await.expect("verify");
    assert!(
        breaks.is_empty(),
        "the chain does not verify after concurrent appends: {breaks:?}"
    );

    store.drop_schema().await.expect("drop");
}
