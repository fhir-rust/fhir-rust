//! Every column type survives a database round trip (spec R4.2).
//!
//! This suite exists because of audit finding **F-20**: `Patient.active` did
//! not come back. SQLite binds `Bool`, `Int`, and `BigInt` all to `INTEGER`
//! (M14.10), the read path asked rusqlite for a `String`, the conversion
//! failed, and the failure landed in an `if let Ok(Some(v))` that discarded it.
//! Every boolean and every integer element vanished from every reconstructed
//! resource — silently, which is the part that matters.
//!
//! The existing `sqlite_store.rs` suite missed it for one reason: every
//! resource it round-trips is built from strings. That is the argument for
//! testing *types* rather than resources, which is what this file does.
//!
//! R4.2 is the invariant the whole project exists to protect, and `C0.13` says
//! a port may never trade it away. So these assert the value came back, not
//! merely that the read succeeded.

use std::sync::Arc;

use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::{Audit, sqlite::SqliteStore};
use serde_json::{Value, json};

fn relmap(version: &str) -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(format!("fhir-sqlite-relmap-{version}.json.gz"));
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok().map(Arc::new)
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/test-scratch")
        .join(format!("rt-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

async fn fresh(name: &str) -> Option<SqliteStore> {
    let map = relmap("r5")?;
    let store = SqliteStore::open(scratch(name).join("fhir.sqlite"), map)
        .await
        .expect("open");
    store.init("roundtrip-types").await.expect("init");
    Some(store)
}

/// Write, read, and require the resource back **exactly**.
async fn assert_round_trips(store: &SqliteStore, resource: &Value) {
    let rtype = resource["resourceType"].as_str().expect("resourceType");
    let id = resource["id"].as_str().expect("id");
    store.put(resource, &Audit::cli()).await.expect("put");
    let back = store
        .get(rtype, id)
        .await
        .expect("get")
        .expect("resource is present");
    assert_eq!(&back, resource, "round trip lost or altered data");
}

/// The regression itself. `active` is a `boolean`, which SQLite stores as
/// `INTEGER` — the case that was silently dropped.
#[tokio::test]
async fn booleans_survive_a_round_trip() {
    let Some(store) = fresh("bool").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    for (id, active) in [("bool-t", true), ("bool-f", false)] {
        assert_round_trips(
            &store,
            &json!({
                "resourceType": "Patient",
                "id": id,
                "active": active,
                "name": [{"family": "Bool"}]
            }),
        )
        .await;
    }
}

/// `false` deserves its own assertion. A reader that rendered every non-null
/// integer as `true` would pass the `true` case and lose every `false`, which
/// clinically is the more dangerous direction: "not active" becoming "active".
#[tokio::test]
async fn a_false_boolean_is_not_confused_with_an_absent_one() {
    let Some(store) = fresh("boolfalse").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let present = json!({
        "resourceType": "Patient", "id": "p", "active": false,
        "name": [{"family": "Present"}]
    });
    let absent = json!({
        "resourceType": "Patient", "id": "a",
        "name": [{"family": "Absent"}]
    });
    assert_round_trips(&store, &present).await;
    assert_round_trips(&store, &absent).await;

    let got_absent = store
        .get("Patient", "a")
        .await
        .expect("get")
        .expect("present");
    assert!(
        got_absent.get("active").is_none(),
        "an element that was never written must stay absent, not become false"
    );
}

/// Integers bind to `INTEGER` too, so they were dropped by the same code path.
#[tokio::test]
async fn integers_survive_a_round_trip() {
    let Some(store) = fresh("int").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    assert_round_trips(
        &store,
        &json!({
            "resourceType": "Patient",
            "id": "int-1",
            "multipleBirthInteger": 3,
            "name": [{"family": "Twin"}]
        }),
    )
    .await;
}

/// Decimals keep their lexical form (M3.6a): `1.50` is not `1.5`.
#[tokio::test]
async fn decimal_precision_survives_a_round_trip() {
    let Some(store) = fresh("dec").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    assert_round_trips(
        &store,
        &json!({
            "resourceType": "Observation",
            "id": "dec-1",
            "status": "final",
            "code": {"text": "weight"},
            "valueQuantity": {"value": 1.50, "unit": "kg"}
        }),
    )
    .await;
}

/// Partial dates keep their precision (M3.6): `1974-12` must not become
/// `1974-12-01`.
#[tokio::test]
async fn partial_dates_survive_a_round_trip() {
    let Some(store) = fresh("date").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    for (id, birth) in [
        ("d-y", "1974"),
        ("d-ym", "1974-12"),
        ("d-full", "1974-12-25"),
    ] {
        assert_round_trips(
            &store,
            &json!({
                "resourceType": "Patient",
                "id": id,
                "birthDate": birth,
                "name": [{"family": "Dated"}]
            }),
        )
        .await;
    }
}

/// One resource carrying every affected type at once, because a per-type test
/// can pass while their interaction does not.
#[tokio::test]
async fn a_resource_mixing_every_column_type_survives() {
    let Some(store) = fresh("mixed").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    assert_round_trips(
        &store,
        &json!({
            "resourceType": "Patient",
            "id": "mixed-1",
            "active": false,
            "birthDate": "1974-12",
            "multipleBirthInteger": 2,
            "name": [{"family": "Ærø", "given": ["Anna", "Marie"]}],
            "telecom": [{"system": "phone", "value": "555-0100"}],
            "managingOrganization": {"reference": "Organization/hl7"}
        }),
    )
    .await;
}
