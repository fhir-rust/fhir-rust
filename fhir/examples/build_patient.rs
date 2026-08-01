//! Build a FHIR R5 `Patient` with the generated builder and serialize it.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example build_patient
//! ```
//!
//! Every resource and general-purpose datatype derives a builder:
//! `Type::builder()` returns a `TypeBuilder` with a chainable setter per field
//! and a `build()` that fails if a required (`1..1`) field was not set. Optional
//! and repeating fields default to absent/empty.

use fhir::r5::coded::Coded;
use fhir::r5::codes::{AdministrativeGender, ContactPointSystem};
use fhir::r5::resources::Patient;
use fhir::r5::types::{Boolean, ContactPoint, HumanName, String as FhirString};

fn main() {
    let patient = Patient::builder()
        .id(FhirString("pat-1".to_string()))
        .active(Boolean(true))
        // `gender` has a required binding, so it is a code enum wrapped in `Coded`.
        .gender(Coded::Known(AdministrativeGender::Male))
        .name(vec![HumanName {
            family: Some(FhirString("Chalmers".to_string())),
            given: vec![
                FhirString("Peter".to_string()),
                FhirString("James".to_string()),
            ],
            ..Default::default()
        }])
        .telecom(vec![
            ContactPoint::builder()
                .system(Coded::Known(ContactPointSystem::Phone))
                .value(FhirString("(03) 5555 6473".to_string()))
                .build()
                .expect("build ContactPoint"),
        ])
        // Patient has no required 1..1 fields, so build() cannot fail here.
        .build()
        .expect("build Patient");

    let json = serde_json::to_string_pretty(&patient).expect("serialize Patient");
    println!("{json}");
}
