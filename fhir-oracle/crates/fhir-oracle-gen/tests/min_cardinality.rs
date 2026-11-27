//! Minimum cardinality at write (`V9.1`, audit **F-111**): an element whose
//! `ElementDefinition.min` is 1 or more must be present, or the shredder
//! refuses the resource and names the path (`V9.3`). Nothing is written on a
//! refusal because shredding happens before any statement runs.
//!
//! Each case below was shown to fail with the check it exercises removed
//! from `shred::handle_elem` (`T11.10`).

use fhir_oracle_map::error::ShredError;
use fhir_oracle_map::model::RelMap;
use fhir_oracle_map::shred::{MISSING_REQUIRED, shred};
use serde_json::{Value, json};

fn r5() -> Option<RelMap> {
    let spec = std::env::var("FHIR_ORACLE_SPEC_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::path::PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../fhir/doc/fhir-specifications"
            ))
        });
    let defs = spec.join("r5").join("fhir-definitions-json");
    if !defs.exists() {
        return None;
    }
    Some(fhir_oracle_gen::generate(&defs, "r5").expect("generate"))
}

/// The missing path, if `v` was refused for cardinality; panics on any other
/// error so a malformed fixture cannot pass as a cardinality refusal.
fn missing_path(map: &RelMap, v: &Value) -> Option<String> {
    let rt = v["resourceType"].as_str().expect("resourceType");
    match shred(&map.resources[rt], v) {
        Ok(_) => None,
        Err(ShredError::At { path, msg }) if msg == MISSING_REQUIRED => Some(path),
        Err(e) => panic!("unexpected error for {v}: {e}"),
    }
}

#[test]
fn required_elements_are_enforced_where_their_parent_exists() {
    let Some(map) = r5() else {
        // T11.12: a skip says so. The model family ships these definitions in
        // this repository, so this only happens on a partial checkout.
        eprintln!("skipping: no R5 definitions");
        return;
    };
    let code = json!({"text": "Synthetic assay"});

    // The map records it at all: Observation.status and .code are 1..1.
    let ok = json!({"resourceType": "Observation", "id": "o", "status": "final", "code": code});
    assert_eq!(
        missing_path(&map, &ok),
        None,
        "a minimal valid Observation is accepted"
    );

    // The audit's case: no code.
    let no_code = json!({"resourceType": "Observation", "id": "o", "status": "final"});
    assert_eq!(missing_path(&map, &no_code).as_deref(), Some("code"));
    let no_status = json!({"resourceType": "Observation", "id": "o", "code": code});
    assert_eq!(missing_path(&map, &no_status).as_deref(), Some("status"));

    // A primitive present only as its extension sibling is present (FHIR
    // allows a required primitive to carry a data-absent-reason instead).
    let ext_only = json!({"resourceType": "Observation", "id": "o", "code": code,
        "_status": {"extension": [{"url": "http://hl7.org/fhir/StructureDefinition/data-absent-reason",
                                   "valueCode": "unknown"}]}});
    assert_eq!(missing_path(&map, &ext_only), None);

    // A required child of an optional backbone binds only when the backbone
    // is there: component is 0..*, component.code is 1..1.
    let comp_ok = json!({"resourceType": "Observation", "id": "o", "status": "final", "code": code,
        "component": [{"code": code}]});
    assert_eq!(missing_path(&map, &comp_ok), None);
    let comp_bad = json!({"resourceType": "Observation", "id": "o", "status": "final", "code": code,
        "component": [{"code": code}, {"valueString": "no code"}]});
    assert_eq!(
        missing_path(&map, &comp_bad).as_deref(),
        Some("component.code")
    );

    // A required choice: any variant satisfies it, none does not.
    // UsageContext.code and .value[x] are both 1..1.
    let ctx = |uc: Value| {
        json!({"resourceType": "CodeSystem", "id": "c", "status": "draft",
        "content": "not-present", "useContext": [uc]})
    };
    let usage = json!({"system": "http://terminology.hl7.org/CodeSystem/usage-context-type", "code": "focus"});
    assert_eq!(
        missing_path(
            &map,
            &ctx(json!({"code": usage, "valueCodeableConcept": {"text": "x"}}))
        ),
        None
    );
    assert_eq!(
        missing_path(&map, &ctx(json!({"code": usage}))).as_deref(),
        Some("useContext.value[x]")
    );
}
