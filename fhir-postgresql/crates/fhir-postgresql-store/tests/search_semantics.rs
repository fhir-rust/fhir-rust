//! Search semantics against live PostgreSQL: token, string, date (incl.
//! precision edges and Periods), reference, number, quantity, _id, and
//! unsupported-parameter behavior. Gated on FHIR_POSTGRESQL_TEST_DB.

use std::path::PathBuf;
use std::sync::Arc;

use fhir_postgresql_store::Store;
use serde_json::json;

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

async fn test_store() -> Option<Store> {
    test_store_in("searchtest").await
}

/// Each test installs its own schema: two tests dropping and creating the
/// same one concurrently deadlock on the catalog locks.
async fn test_store_in(schema: &str) -> Option<Store> {
    let db = std::env::var("FHIR_POSTGRESQL_TEST_DB").ok()?;
    let defs = spec_defs()?;
    // SAFETY: set before concurrent access matters in this test binary.
    unsafe { std::env::set_var("PGDATABASE", &db) };
    let map = Arc::new(fhir_postgresql_gen::generate(&defs, schema).expect("generate"));
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, map).await.expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("search-checksum").await.expect("init");
    Some(store)
}

async fn ids(store: &Store, rtype: &str, params: &[(&str, &str)]) -> Vec<String> {
    let p: Vec<(String, String)> = params
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    store.search(rtype, &p, 100, 0).await.expect("search")
}

#[tokio::test]
async fn search_semantics() {
    let Some(store) = test_store().await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };

    // Fixtures.
    store
        .put(&json!({
            "resourceType": "Patient", "id": "alice",
            "active": true, "gender": "female", "birthDate": "1980-03-15",
            "name": [{"family": "Smith", "given": ["Alice", "Beth"]}],
            "identifier": [{"system": "http://acme.org/mrn", "value": "MRN-1"}],
            "managingOrganization": {"reference": "Organization/acme"}
        }))
        .await
        .expect("alice");
    store
        .put(&json!({
            "resourceType": "Patient", "id": "bob",
            "active": false, "gender": "male", "birthDate": "1980-11",
            "name": [{"family": "Smithson"}],
            "identifier": [{"system": "http://other.org", "value": "MRN-1"}]
        }))
        .await
        .expect("bob");
    store
        .put(&json!({
            "resourceType": "Patient", "id": "carol",
            "gender": "female", "birthDate": "1994-06-01",
            "name": [{"family": "Jones", "given": ["Carol"]}]
        }))
        .await
        .expect("carol");
    store
        .put(&json!({
            "resourceType": "Observation", "id": "obs1", "status": "final",
            "code": {"coding": [{"system": "http://loinc.org", "code": "8480-6"}]},
            "subject": {"reference": "Patient/alice"},
            "effectivePeriod": {"start": "2026-01-01", "end": "2026-03-01"},
            "valueQuantity": {"value": 120.5, "system": "http://unitsofmeasure.org", "code": "mm[Hg]"}
        }))
        .await
        .expect("obs1");
    store
        .put(&json!({
            "resourceType": "Observation", "id": "obs2", "status": "preliminary",
            "code": {"coding": [{"system": "http://loinc.org", "code": "9279-1"}]},
            "subject": {"reference": "Patient/bob"},
            "effectiveDateTime": "2026-06-15T10:30:00Z",
            "valueQuantity": {"value": 16, "system": "http://unitsofmeasure.org", "code": "/min"}
        }))
        .await
        .expect("obs2");

    // Token on primitive (code + boolean).
    assert_eq!(
        ids(&store, "Patient", &[("gender", "female")]).await,
        ["alice", "carol"]
    );
    assert_eq!(
        ids(&store, "Patient", &[("active", "true")]).await,
        ["alice"]
    );
    // Token system|value on Identifier.
    assert_eq!(
        ids(
            &store,
            "Patient",
            &[("identifier", "http://acme.org/mrn|MRN-1")]
        )
        .await,
        ["alice"]
    );
    assert_eq!(
        ids(&store, "Patient", &[("identifier", "MRN-1")]).await,
        ["alice", "bob"]
    );
    // Token on CodeableConcept coding.
    assert_eq!(
        ids(
            &store,
            "Observation",
            &[("code", "http://loinc.org|8480-6")]
        )
        .await,
        ["obs1"]
    );
    // OR values.
    assert_eq!(
        ids(&store, "Observation", &[("code", "8480-6,9279-1")]).await,
        ["obs1", "obs2"]
    );

    // String: prefix (default), exact, contains; multi-part name.
    assert_eq!(
        ids(&store, "Patient", &[("family", "Smith")]).await,
        ["alice", "bob"]
    );
    assert_eq!(
        ids(&store, "Patient", &[("family:exact", "Smith")]).await,
        ["alice"]
    );
    assert_eq!(ids(&store, "Patient", &[("name", "beth")]).await, ["alice"]);
    assert_eq!(
        ids(&store, "Patient", &[("name:contains", "ones")]).await,
        ["carol"]
    );

    // String search is accent-insensitive as well as case-insensitive
    // (P6.6). Every spelling below must find the same patient, including the
    // decomposed form, which an ILIKE comparison misses.
    store
        .put(&json!({
            "resourceType": "Patient", "id": "dora",
            "name": [{"family": "Müller", "given": ["Zoë"]}]
        }))
        .await
        .expect("put dora");
    for term in ["Müller", "muller", "MULLER", "Mu\u{308}ller", "mül"] {
        assert_eq!(
            ids(&store, "Patient", &[("family", term)]).await,
            ["dora"],
            "family={term:?} should match the folded column"
        );
    }
    assert_eq!(
        ids(&store, "Patient", &[("name:contains", "oe")]).await,
        ["dora"],
        ":contains folds the term and the stored value alike"
    );
    // `:exact` is defined as the literal string, so folding must not leak
    // into it: an unaccented spelling is not an exact match.
    assert_eq!(
        ids(&store, "Patient", &[("family:exact", "Müller")]).await,
        ["dora"]
    );
    assert!(
        ids(&store, "Patient", &[("family:exact", "muller")])
            .await
            .is_empty(),
        ":exact must stay literal"
    );
    // Non-Latin scripts fold too: Greek final sigma and case.
    store
        .put(&json!({
            "resourceType": "Patient", "id": "elena",
            "name": [{"family": "ΑΘΉΝΑ"}]
        }))
        .await
        .expect("put elena");
    assert_eq!(
        ids(&store, "Patient", &[("family", "αθην")]).await,
        ["elena"]
    );

    // Date precision: partial birthDate "1980-11" is inside 1980.
    assert_eq!(
        ids(&store, "Patient", &[("birthdate", "1980")]).await,
        ["alice", "bob"]
    );
    assert_eq!(
        ids(&store, "Patient", &[("birthdate", "1980-03")]).await,
        ["alice"]
    );
    assert_eq!(
        ids(&store, "Patient", &[("birthdate", "ge1980-06")]).await,
        ["bob", "carol"]
    );
    assert_eq!(
        ids(&store, "Patient", &[("birthdate", "lt1980-06")]).await,
        ["alice"]
    );
    // AND across parameters.
    assert_eq!(
        ids(
            &store,
            "Patient",
            &[("gender", "female"), ("birthdate", "ge1990")]
        )
        .await,
        ["carol"]
    );

    // Date on choice effective[x]: Period overlap and dateTime point.
    assert_eq!(
        ids(&store, "Observation", &[("date", "2026-02")]).await,
        ["obs1"]
    );
    assert_eq!(
        ids(&store, "Observation", &[("date", "2026-06-15")]).await,
        ["obs2"]
    );
    assert_eq!(
        ids(&store, "Observation", &[("date", "ge2026-01")]).await,
        ["obs1", "obs2"]
    );

    // Reference.
    assert_eq!(
        ids(&store, "Observation", &[("subject", "Patient/alice")]).await,
        ["obs1"]
    );
    assert_eq!(
        ids(&store, "Patient", &[("organization", "Organization/acme")]).await,
        ["alice"]
    );

    // Quantity.
    assert_eq!(
        ids(
            &store,
            "Observation",
            &[("value-quantity", "120.5|http://unitsofmeasure.org|mm[Hg]")]
        )
        .await,
        ["obs1"]
    );
    assert_eq!(
        ids(&store, "Observation", &[("value-quantity", "gt100")]).await,
        ["obs1"]
    );

    // _id and paging.
    assert_eq!(
        ids(&store, "Patient", &[("_id", "bob,carol")]).await,
        ["bob", "carol"]
    );
    let page: Vec<String> = store.search("Patient", &[], 2, 1).await.expect("page");
    assert_eq!(page, ["bob", "carol"]);

    // Unsupported parameter errors (strict semantics at the store level).
    let err = store
        .search(
            "Patient",
            &[("deceased".to_string(), "true".to_string())],
            10,
            0,
        )
        .await
        .expect_err("deceased is uncompiled (exists() expression)");
    assert!(err.to_string().contains("not supported"), "{err}");
    let err = store
        .search("Patient", &[("nope".to_string(), "x".to_string())], 10, 0)
        .await
        .expect_err("unknown param");
    assert!(err.to_string().contains("unsupported"), "{err}");
}

/// A string prefix search must use its index in the *generic* plan (P6.6).
///
/// This is the regression that motivated the folded column. Written the
/// obvious way — `col ILIKE $1 || '%'`, or even a plain `LIKE $1` against a
/// `text_pattern_ops` index — PostgreSQL can only extract a prefix from a
/// *constant* pattern, so the generic plan scans the whole table. The fix is
/// to compute the range bounds in Rust and emit `>= AND <`, which is an
/// ordinary index condition the planner needs no pattern analysis to use.
#[tokio::test]
async fn string_prefix_search_uses_its_index() {
    let Some(store) = test_store_in("searchplan").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };
    let params = vec![("family".to_string(), "müll".to_string())];
    let plan = store
        .explain_generic_for_test("Patient", &params)
        .await
        .expect("explain")
        .join("\n");
    assert!(
        plan.contains("Index Scan") || plan.contains("Index Only Scan"),
        "prefix search fell back to a sequential scan under a generic plan:\n{plan}"
    );
    assert!(
        plan.contains(">=") && plan.contains('<'),
        "expected a range index condition, got:\n{plan}"
    );
}
