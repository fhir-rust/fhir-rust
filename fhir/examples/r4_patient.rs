//! Build a FHIR R4 `Patient`, serialize it, validate it, and read it back.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example r4_patient --features r4
//! ```
//!
//! The R4 model is laid out exactly like the R5 one, so everything here is the
//! R5 example with `r5` changed to `r4`. What differs is the *content*: R4 and
//! R5 disagree about which elements exist and what they are called, and the two
//! models keep that difference visible instead of papering over it.

use fhir::r4::coded::Coded;
use fhir::r4::codes::{AdministrativeGender, ContactPointSystem};
use fhir::r4::resources::Patient;
use fhir::r4::types::{Boolean, ContactPoint, HumanName, String as FhirString};
use fhir::r4::validate::Validate;

fn main() {
    let patient = Patient::builder()
        .id(FhirString("pat-1".to_string()))
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

    // The model validates against the R4 constraints, not R5's.
    let issues = patient.validate();
    println!("validation issues: {}", issues.len());
    assert!(issues.is_empty());

    // Round-trip: canonical FHIR JSON parses back to an equal value.
    let parsed: Patient = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed, patient);
    println!("round-trip: ok");
}
