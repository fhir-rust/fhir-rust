//! Adversarial concurrency (spec T11.6). Gated on FHIR_POSTGRESQL_TEST_DB.
//!
//! Each test here corresponds to a defect that a single-threaded test suite
//! cannot see:
//!
//! - **Torn reads (R4.5).** A read spans a base table and many child tables.
//!   Issued as separate statements, a concurrent write between them
//!   reconstructs a resource that never existed. The reader loop below fails
//!   if it ever observes one.
//! - **Conditional-create races (A7.10).** Search-then-write lets two
//!   identical conditional creates both find nothing and both create — a
//!   patient entered twice.
//! - **Optimistic concurrency (D11).** N writers presenting the same
//!   `If-Match` must produce exactly one winner.

use std::path::PathBuf;
use std::sync::Arc;

use fhir_postgresql_store::{CondCreate, Store, StoreError};
use serde_json::{Value, json};

fn spec_defs() -> Option<PathBuf> {
    let root = std::env::var("FHIR_POSTGRESQL_SPEC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../fhir/doc/fhir-specifications"
            ))
        });
    let defs = root.join("r5").join("fhir-definitions-json");
    defs.exists().then_some(defs)
}

/// A store on its own schema. The tests in this binary run concurrently by
/// design, so they must not share one — installing and dropping the same
/// schema from three threads collides in `pg_namespace`, which says nothing
/// about the code under test.
async fn test_store(schema: &str) -> Option<Arc<Store>> {
    let db = std::env::var("FHIR_POSTGRESQL_TEST_DB").ok()?;
    let defs = spec_defs()?;
    // SAFETY: single test binary, set before any threads matter.
    unsafe { std::env::set_var("PGDATABASE", &db) };
    let map = Arc::new(fhir_postgresql_gen::generate(&defs, schema).expect("generate"));
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, map).await.expect("connect");
    store.drop_schema().await.expect("drop");
    assert!(store.init("conc-checksum").await.expect("init"));
    Some(Arc::new(store))
}

/// A patient whose data spans the base table and several child tables, so a
/// torn read has somewhere to show up: `name` (child), `name.given`
/// (grandchild), `telecom` (child), and `active` (base column).
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

/// Every field of a resource must come from the same generation.
fn generation(p: &Value) -> Option<String> {
    let family = p.pointer("/name/0/family")?.as_str()?;
    Some(family.trim_start_matches("Family").to_string())
}

fn assert_coherent(p: &Value) {
    let n: usize = generation(p)
        .expect("family present")
        .parse()
        .expect("generation");
    let given = p.pointer("/name/0/given/0").and_then(Value::as_str);
    let middle = p.pointer("/name/0/given/1").and_then(Value::as_str);
    let phone = p.pointer("/telecom/0/value").and_then(Value::as_str);
    let active = p.get("active").and_then(Value::as_bool);
    assert_eq!(given, Some(format!("Given{n}").as_str()), "torn: {p}");
    assert_eq!(middle, Some(format!("Middle{n}").as_str()), "torn: {p}");
    assert_eq!(phone, Some(format!("{n:06}").as_str()), "torn: {p}");
    assert_eq!(active, Some(n.is_multiple_of(2)), "torn: {p}");
}

#[tokio::test]
async fn reads_never_tear_under_concurrent_writes() {
    let Some(store) = test_store("conc_torn").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };
    store.put(&patient("torn", 0)).await.expect("seed");

    let writer_store = store.clone();
    let writer = tokio::spawn(async move {
        for n in 1..=200 {
            writer_store.put(&patient("torn", n)).await.expect("write");
        }
    });

    let reader_store = store.clone();
    let reader = tokio::spawn(async move {
        let mut seen = 0usize;
        for _ in 0..600 {
            if let Some(got) = reader_store.get("Patient", "torn").await.expect("read") {
                assert_coherent(&got.resource);
                seen += 1;
            }
        }
        seen
    });

    writer.await.expect("writer");
    let seen = reader.await.expect("reader");
    assert!(seen > 0, "the reader never saw the resource at all");
}

#[tokio::test]
async fn racing_conditional_creates_produce_one_resource() {
    let Some(store) = test_store("conc_cond").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
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
            // Each racer proposes its own id, exactly as the server does.
            resource["id"] = json!(format!("cond-{i}"));
            tokio::spawn(async move {
                store
                    .conditional_create("Patient", &criteria, &resource)
                    .await
            })
        })
        .collect();

    let mut created = 0;
    let mut existing = 0;
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

#[tokio::test]
async fn racing_if_match_updates_have_one_winner() {
    let Some(store) = test_store("conc_lock").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };
    let seed = store.put(&patient("lock", 1)).await.expect("seed");

    let racers: Vec<_> = (0..8)
        .map(|n| {
            let store = store.clone();
            let body = patient("lock", 10 + n);
            let expected = seed.version_id;
            tokio::spawn(async move { store.put_if(&body, Some(expected)).await })
        })
        .collect();

    let mut winners = 0;
    let mut conflicts = 0;
    for r in racers {
        match r.await.expect("join") {
            Ok(_) => winners += 1,
            Err(StoreError::Conflict { .. }) => conflicts += 1,
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
    assert_eq!(winners, 1, "exactly one If-Match writer may win");
    assert_eq!(conflicts, 7);
}
