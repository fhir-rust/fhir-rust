//! Read and write FHIR *primitive extensions* — the `_field` siblings.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example primitive_extensions
//! ```
//!
//! In FHIR, even a primitive value such as `birthDate` may carry an `id` and
//! `extension`s. JSON can't hang those on a bare scalar, so FHIR puts them in a
//! *sibling* property whose name is the field prefixed with an underscore:
//! `birthDate` holds the value, `_birthDate` holds its extensions. This crate
//! models that sibling as a `<field>_ext` field of type [`Element`], serialized
//! back to the `_field` key. See `spec/09-primitive-extensions.md`.
//!
//! [`Element`]: fhir::r5::types::Element

use fhir::r5::choice::Primitive;
use fhir::r5::resources::Patient;
use fhir::r5::types::extension::ExtensionValue;
use fhir::r5::types::{Date, DateTime, Element, Extension, String as FhirString};

fn main() {
    // A Patient whose birth date is known precisely down to the time of day.
    // The exact time is carried as the standard `patient-birthTime` extension
    // on the `_birthDate` sibling, not on `birthDate` itself.
    let json = serde_json::json!({
        "resourceType": "Patient",
        "id": "pat-1",
        "birthDate": "1970-03-25",
        "_birthDate": {
            "extension": [{
                "url": "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
                "valueDateTime": "1970-03-25T14:35:00-05:00"
            }]
        }
    });

    // Parse it. Both the value and its extension sibling deserialize.
    let patient: Patient = serde_json::from_value(json.clone()).expect("parse Patient");

    println!(
        "birthDate value: {}",
        patient.birth_date.as_ref().unwrap().0
    );

    // Reach into the `_birthDate` extension to recover the precise birth time.
    // `Extension.value[x]` is the `ExtensionValue` choice enum.
    if let Some(birth_time) = patient
        .birth_date_ext
        .as_ref()
        .iter()
        .flat_map(|e| &e.extension)
        .filter(|ext| ext.url.0.ends_with("patient-birthTime"))
        .find_map(|ext| match &ext.value {
            Some(ExtensionValue::DateTime(p)) => Some(&p.value),
            _ => None,
        })
    {
        println!("precise birthTime: {}", birth_time.0);
    }

    // Round-trip: the `_birthDate` sibling is preserved exactly. (A bare resource
    // struct does not emit the `resourceType` tag — that is added by the
    // polymorphic `Resource` enum — so we compare the `_birthDate` sibling.)
    let reserialized = serde_json::to_value(&patient).expect("serialize");
    assert_eq!(
        reserialized.get("_birthDate"),
        json.get("_birthDate"),
        "primitive extension must round-trip"
    );
    println!("round-trip: _birthDate preserved ✓");

    // Building one from scratch: set the value and the sibling side by side.
    let built = Patient {
        birth_date: Some(Date("2000-01-01".to_string())),
        birth_date_ext: Some(Element {
            extension: vec![Extension {
                url: FhirString(
                    "http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string(),
                ),
                value: Some(ExtensionValue::DateTime(Primitive::new(DateTime(
                    "2000-01-01T06:00:00Z".to_string(),
                )))),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let built_json = serde_json::to_string_pretty(&built).expect("serialize built");
    println!("\nbuilt Patient:\n{built_json}");
}
