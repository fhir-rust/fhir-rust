//! `Coded<E>` against a real generated R5 enum.
//!
//! Lives here rather than in fhir-core because it needs a release model:
//! the core crate defines `Coded<E>` but has no codes to instantiate it
//! with.

#![cfg(feature = "r5")]

use fhir::coded::Coded;
use fhir::r5::codes::AdministrativeGender;

#[test]
fn known_and_unknown_roundtrip() {
    for (json, expect) in [
        (
            serde_json::json!("male"),
            Coded::Known(AdministrativeGender::Male),
        ),
        (serde_json::json!("xyz"), Coded::Unknown("xyz".to_string())),
    ] {
        let parsed: Coded<AdministrativeGender> = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(parsed, expect);
        assert_eq!(serde_json::to_value(&parsed).unwrap(), json);
    }
}

#[test]
fn default_is_known() {
    assert_eq!(
        Coded::<AdministrativeGender>::default(),
        Coded::Known(AdministrativeGender::default())
    );
}
