//! Build a FHIR R3 `Patient`, serialize it, validate it, and read it back.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example r3_patient --features r3
//! ```
//!
//! The R3 model is laid out exactly like the R5 one, so everything here is the
//! R5 example with `r5` changed to `r3`. What differs is the *content*. Two
//! differences show up in this one short program:
//!
//! - A resource's `id` is a `types::Id` in R3, where R4 and R5 type it as a
//!   `types::String`.
//! - R3 has no `canonical` or `url` primitive at all; both arrived in R4.
//!
//! The models keep such differences visible instead of papering over them.

use fhir::r3::coded::Coded;
use fhir::r3::codes::{AdministrativeGender, ContactPointSystem};
use fhir::r3::resources::Patient;
use fhir::r3::types::{Boolean, ContactPoint, HumanName, Id, String as FhirString};
use fhir::r3::validate::Validate;

fn main() {
    let patient = Patient::builder()
        .id(Id("pat-1".to_string())) // R3 types a resource id as `id`, not `string`
        .active(Boolean(true))
        .gender(Coded::Known(AdministrativeGender::Female))
        .name(vec![HumanName {
            family: Some(FhirString("Chalmers".to_string())),
            given: vec![FhirString("Jane".to_string())],
            ..Default::default()
        }])
        .telecom(vec![ContactPoint {
            system: Some(Coded::Known(ContactPointSystem::Phone)),
            value: Some(FhirString("+1 555 0100".to_string())),
            ..Default::default()
        }])
        .build()
        .expect("Patient has no required fields, so this cannot fail");

    let json = serde_json::to_string_pretty(&patient).expect("serialize");
    println!("{json}\n");

    // The model validates against the R3 constraints, not R5's.
    let issues = patient.validate();
    println!("validation issues: {}", issues.len());
    assert!(issues.is_empty());

    // Round-trip: canonical FHIR JSON parses back to an equal value.
    let parsed: Patient = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed, patient);
    println!("round-trip: ok");
}
