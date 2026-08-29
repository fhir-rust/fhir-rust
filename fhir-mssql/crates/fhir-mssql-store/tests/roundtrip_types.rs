//! Every column type survives a database round trip (spec R4.2).
//!
//! Ported from the MySQL suite, written there to settle audit finding
//! **F-20**: `Row::get` panics rather than returning an error on a type
//! mismatch, and the same defect class was found live in this port too
//! (`cell_text`, `hist_entry`, the `[ords]` reads — see `mssql.rs`'s own
//! commentary) before this suite existed to catch it going forward.
//!
//! `Bool` binds to `BIT`, `Int` to `INT`, and the derived sort columns to
//! `DATE`/`DATETIME2(6)`. `mssql`'s `Row::get::<T,_>` is `try_get(idx).
//! unwrap()` internally, so reading any resource carrying a boolean, an
//! integer, or a date without matching the raw `ColumnData` variant panics —
//! and a panic in a library is a denial of service for whatever hosts it
//! (T11.9).
//!
//! These assert the **whole resource** comes back, not that the read
//! succeeded. R4.2 is the invariant the project exists to protect, and
//! `C0.13` says a port may never trade it away.
//!
//! Needs `FHIR_MSSQL_TEST_DSN`; `scripts/db.sh up` prints it.

use std::sync::Arc;

use fhir_mssql_map::model::RelMap;
use fhir_mssql_store::mssql::MsSqlStore;
use serde_json::{Value, json};

mod common;

fn dsn() -> Option<String> {
    common::dsn().map(str::to_string)
}

fn relmap(version: &str) -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-mssql-map/assets")
        .join(format!("fhir-mssql-relmap-{version}.json.gz"));
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok().map(Arc::new)
}

/// Only the resource types a test uses: creating every table takes a long
/// time, which is why the rest of this suite samples too.
fn sampled(schema: &str, want: &[&str]) -> Option<Arc<RelMap>> {
    let mut m = (*relmap("r5")?).clone();
    m.resources.retain(|k, _| want.contains(&k.as_str()));
    assert!(
        !m.resources.is_empty(),
        "none of {want:?} are in the r5 map"
    );
    m.schema = schema.to_string();
    Some(Arc::new(m))
}

async fn fresh(schema: &str, want: &[&str]) -> Option<MsSqlStore> {
    let store = MsSqlStore::connect(&dsn()?, sampled(schema, want)?)
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("roundtrip-types").await.expect("init");
    Some(store)
}

/// Write, read, and require the resource back **exactly**.
async fn assert_round_trips(store: &MsSqlStore, resource: &Value) {
    let rtype = resource["resourceType"].as_str().expect("resourceType");
    let id = resource["id"].as_str().expect("id");
    let audit = fhir_mssql_store::Audit::default();
    store.put(resource, &audit).await.expect("put");
    let back = store
        .get(rtype, id)
        .await
        .expect("get")
        .expect("resource is present");
    assert_eq!(&back, resource, "round trip lost or altered data");
}

/// The `cell_text` check. `active` is a `boolean`, which this port stores as
/// `BIT` and mssql reads back as `ColumnData::Bit`, not `String`.
#[tokio::test]
async fn booleans_survive_a_round_trip() {
    let Some(store) = fresh("fhir_mssql_rt_bool", &["Patient"]).await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
    store.drop_schema().await.expect("drop");
}

/// `false` deserves its own assertion. A reader that rendered every non-null
/// bit as `true` would pass the `true` case and lose every `false` —
/// clinically the more dangerous direction, since "not active" becomes
/// "active".
#[tokio::test]
async fn a_false_boolean_is_not_confused_with_an_absent_one() {
    let Some(store) = fresh("fhir_mssql_rt_boolf", &["Patient"]).await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
        return;
    };
    assert_round_trips(
        &store,
        &json!({
            "resourceType": "Patient", "id": "p", "active": false,
            "name": [{"family": "Present"}]
        }),
    )
    .await;
    assert_round_trips(
        &store,
        &json!({
            "resourceType": "Patient", "id": "a",
            "name": [{"family": "Absent"}]
        }),
    )
    .await;

    let got = store
        .get("Patient", "a")
        .await
        .expect("get")
        .expect("present");
    assert!(
        got.get("active").is_none(),
        "an element that was never written must stay absent, not become false"
    );

    store.drop_schema().await.expect("drop");
}

/// Integers bind to `INT`, so the same code path carries them.
#[tokio::test]
async fn integers_survive_a_round_trip() {
    let Some(store) = fresh("fhir_mssql_rt_int", &["Patient"]).await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
    store.drop_schema().await.expect("drop");
}

/// Decimals keep their lexical form (M3.6a): `1.50` is not `1.5`.
#[tokio::test]
async fn decimal_precision_survives_a_round_trip() {
    let Some(store) = fresh("fhir_mssql_rt_dec", &["Observation"]).await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
    store.drop_schema().await.expect("drop");
}

/// Partial dates keep their precision (M3.6): `1974-12` must not become
/// `1974-12-01`.
#[tokio::test]
async fn partial_dates_survive_a_round_trip() {
    let Some(store) = fresh("fhir_mssql_rt_date", &["Patient"]).await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
    store.drop_schema().await.expect("drop");
}

/// One resource carrying every affected type at once, because a per-type
/// test can pass while their interaction does not.
#[tokio::test]
async fn a_resource_mixing_every_column_type_survives() {
    let Some(store) = fresh("fhir_mssql_rt_mixed", &["Patient"]).await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
    store.drop_schema().await.expect("drop");
}
