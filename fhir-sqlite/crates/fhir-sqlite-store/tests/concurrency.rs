//! Adversarial concurrency (spec T11.6), ported from the PostgreSQL suite.
//!
//! Needs no server and no environment variables: SQLite is bundled, so these
//! always run. That is the point — the inherited PostgreSQL suites self-skip
//! without a database, and a suite that skips is indistinguishable from one
//! that passes (T11.12).
//!
//! Each test corresponds to a defect a single-threaded suite cannot see:
//!
//! - **Torn reads (R4.5).** A read spans a base table and many child tables.
//!   Issued as separate statements, a concurrent write between them
//!   reconstructs a resource that never existed.
//! - **Conditional-create races.** Search-then-write lets two identical
//!   conditional creates both find nothing and both create — a patient entered
//!   twice.
//! - **Optimistic concurrency.** N writers presenting the same expected version
//!   must produce exactly one winner.
//! - **Version assignment (H5.4).** N racing writers must produce N distinct
//!   consecutive versions and a chain that still verifies.
//!
//! # One handle or two
//!
//! `SqliteStore` is one connection behind a mutex, so two tasks sharing one
//! handle are serialised by that mutex and can never interleave mid-read.
//! Testing torn reads through a single handle would therefore assert nothing —
//! it could not fail however broken the code was (T11.10).
//!
//! The tests that care about isolation open a **second handle on the same
//! file**, which is a second SQLite connection and an ordinary deployment
//! shape. There, WAL and whatever transaction the reader opens are the only
//! things standing between a reader and a half-applied write.

use std::sync::Arc;

use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::sqlite::SqliteStore;
use fhir_sqlite_store::{Audit, CondCreate};
use serde_json::{Value, json};

fn relmap(version: &str) -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(format!("fhir-sqlite-relmap-{version}.json.gz"));
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok().map(Arc::new)
}

/// A scratch directory under the workspace `target/`, not `TMPDIR`.
///
/// Matches the convention `sqlite_store.rs` established, and panics on a
/// duplicate name for the same reason: two tests sharing a directory silently
/// delete each other's database, and the collision surfaces as three unrelated
/// assertion failures rather than as itself.
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
        .join(format!("conc-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// An installed store, plus the path so a second handle can be opened on it.
async fn fresh(name: &str) -> Option<(Arc<SqliteStore>, std::path::PathBuf)> {
    let map = relmap("r5")?;
    let dir = scratch(name);
    let db = dir.join("fhir.sqlite");
    let store = SqliteStore::open(&db, map).await.expect("open");
    store.init("concurrency-checksum").await.expect("init");
    Some((Arc::new(store), db))
}

/// A second connection to the same database, as a separate process would have.
async fn second_handle(db: &std::path::Path) -> Arc<SqliteStore> {
    let map = relmap("r5").expect("relmap");
    Arc::new(
        SqliteStore::open(db, map)
            .await
            .expect("open second handle"),
    )
}

/// Every field encodes the same generation number, so an incoherent mix is
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
    let family = p.pointer("/name/0/family")?.as_str()?;
    family.trim_start_matches("Family").parse().ok()
}

/// Assert every field of the resource came from one generation.
///
/// `family` lives on a child table (`patient_name`), `given` on a grandchild
/// (`patient_name_given`), `telecom` on another child, and `active` on the base
/// table — so a torn read shows up as a disagreement between them.
fn assert_coherent(p: &Value) {
    let n = generation(p).expect("family present and well-formed");
    let given = p.pointer("/name/0/given/0").and_then(Value::as_str);
    let middle = p.pointer("/name/0/given/1").and_then(Value::as_str);
    let phone = p.pointer("/telecom/0/value").and_then(Value::as_str);
    let active = p.get("active").and_then(Value::as_bool);
    assert_eq!(given, Some(format!("Given{n}").as_str()), "torn read: {p}");
    assert_eq!(
        middle,
        Some(format!("Middle{n}").as_str()),
        "torn read: {p}"
    );
    assert_eq!(phone, Some(format!("{n:06}").as_str()), "torn read: {p}");
    assert_eq!(active, Some(n.is_multiple_of(2)), "torn read: {p}");
}

/// R4.5 — a reader on its own connection must never observe a half-applied
/// write.
///
/// The reader uses a **second handle**, so the store's connection mutex is not
/// what makes this pass. If it passes, SQLite's isolation is doing the work; if
/// it fails, the read path is issuing independent statements outside any
/// transaction and R4.5 is unmet.
#[tokio::test]
async fn reads_never_tear_under_concurrent_writes() {
    let Some((store, db)) = fresh("torn").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    store
        .put(&patient("torn", 0), &Audit::cli())
        .await
        .expect("seed");

    let reader_store = second_handle(&db).await;

    let writer = tokio::spawn({
        let store = store.clone();
        async move {
            for n in 1..=150 {
                store
                    .put(&patient("torn", n), &Audit::cli())
                    .await
                    .expect("write");
            }
        }
    });

    let reader = tokio::spawn(async move {
        let mut seen = 0usize;
        let mut generations = std::collections::HashSet::new();
        for _ in 0..600 {
            if let Some(got) = reader_store.get("Patient", "torn").await.expect("read") {
                assert_coherent(&got);
                generations.insert(generation(&got).expect("generation"));
                seen += 1;
            }
            tokio::task::yield_now().await;
        }
        (seen, generations.len())
    });

    writer.await.expect("writer");
    let (seen, distinct) = reader.await.expect("reader");

    // Without these the test passes trivially on a reader that never observed
    // anything, or one that only ever saw the seed row (T11.12).
    assert!(seen > 0, "the reader never saw the resource at all");
    assert!(
        distinct > 1,
        "the reader only ever saw generation {distinct}, so it never raced the \
         writer and this test proved nothing"
    );
}

/// Racing conditional creates must yield exactly one resource.
///
/// This is the race the `write_gate` exists for: a conditional create is a
/// search followed by a write, and SQLite's own single-writer lock does not
/// span the gap between them.
#[tokio::test]
async fn racing_conditional_creates_produce_one_resource() {
    let Some((store, _db)) = fresh("cond").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let criteria = vec![("identifier".to_string(), "urn:mrn|12345".to_string())];
    let resource = json!({
        "resourceType": "Patient",
        "identifier": [{"system": "urn:mrn", "value": "12345"}],
        "name": [{"family": "Race"}]
    });

    let racers: Vec<_> = (0..8)
        .map(|i| {
            let store = store.clone();
            let criteria = criteria.clone();
            let mut resource = resource.clone();
            resource["id"] = json!(format!("cond-{i}"));
            tokio::spawn(async move {
                store
                    .conditional_create_audited("Patient", &criteria, &resource, &Audit::cli())
                    .await
            })
        })
        .collect();

    let (mut created, mut existing) = (0, 0);
    for r in racers {
        match r.await.expect("join").expect("conditional create") {
            CondCreate::Created(_) => created += 1,
            CondCreate::Existing(_) => existing += 1,
            CondCreate::Multiple => panic!("criteria matched several resources"),
        }
    }
    assert_eq!(created, 1, "exactly one racer may create");
    assert_eq!(existing, 7, "the rest must find the winner");

    let all = store
        .search("Patient", &criteria, 100, 0)
        .await
        .expect("search");
    assert_eq!(
        all.len(),
        1,
        "the chart must hold one patient, not {}",
        all.len()
    );
}

/// N writers presenting the same expected version: exactly one wins.
#[tokio::test]
async fn racing_version_guarded_updates_have_one_winner() {
    let Some((store, _db)) = fresh("ifmatch").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let put = store
        .put(&patient("lock", 0), &Audit::cli())
        .await
        .expect("seed");
    let base_version = put.version_id;

    let racers: Vec<_> = (1..=8)
        .map(|n| {
            let store = store.clone();
            tokio::spawn(async move {
                store
                    .put_audited(&patient("lock", n), Some(base_version), &Audit::cli())
                    .await
            })
        })
        .collect();

    let (mut won, mut lost) = (0, 0);
    for r in racers {
        match r.await.expect("join") {
            Ok(_) => won += 1,
            Err(_) => lost += 1,
        }
    }
    assert_eq!(
        won, 1,
        "exactly one writer may win a version-guarded update"
    );
    assert_eq!(lost, 7, "the losers must be refused, not silently applied");

    let got = store
        .get("Patient", "lock")
        .await
        .expect("read")
        .expect("present");
    assert_coherent(&got);
}

/// H5.4 — racing unguarded writers produce distinct consecutive versions, and
/// the chain still verifies.
///
/// The chain digest of version *n* commits to the digest of version *n−1*, so a
/// race that interleaved two appends would leave two rows claiming the same
/// predecessor and a chain that verifies for neither. Asserting the versions
/// alone would miss that; asserting the chain alone would not localise it.
#[tokio::test]
async fn racing_writers_get_distinct_versions_and_a_verifiable_chain() {
    let Some((store, _db)) = fresh("versions").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    store
        .put(&patient("ver", 0), &Audit::cli())
        .await
        .expect("seed");

    const WRITERS: usize = 12;
    let racers: Vec<_> = (1..=WRITERS)
        .map(|n| {
            let store = store.clone();
            tokio::spawn(async move { store.put(&patient("ver", n), &Audit::cli()).await })
        })
        .collect();

    let mut versions = Vec::new();
    for r in racers {
        versions.push(r.await.expect("join").expect("write").version_id);
    }
    versions.sort_unstable();

    let distinct: std::collections::HashSet<_> = versions.iter().copied().collect();
    assert_eq!(
        distinct.len(),
        WRITERS,
        "two writers were handed the same version_id: {versions:?}"
    );
    assert_eq!(
        versions,
        (2..=(WRITERS as i64 + 1)).collect::<Vec<_>>(),
        "versions must be consecutive from the seed, with no gaps: {versions:?}"
    );

    let history = store.history("Patient", "ver").await.expect("history");
    assert_eq!(history.len(), WRITERS + 1, "one history row per write");

    let breaks = store.verify_audit().await.expect("verify");
    assert!(
        breaks.is_empty(),
        "the chain does not verify after concurrent appends: {breaks:?}"
    );
}
