# `typed_references` — Typed references: what the compiler knows about where a reference points

Run with:

```sh
cargo run --example typed_references
```

Where the FHIR® specification says a reference targets exactly one resource
type, the generated field says so too: `AllergyIntolerance.patient` is
`Reference<Patient>`, not a stringly-typed link. Where the specification
allows several targets — `Observation.subject` may be a Patient, Group,
Device, or Location — the field stays `Reference<Any>`, because a type
claiming one target would be wrong four ways. The wire form is identical
either way: the target is a zero-sized compile-time marker.

`resolve` uses the marker: resolving a `Reference<Patient>` inside a
`Bundle` refuses a matching entry whose `resourceType` is not `Patient`.

## The program

```rust
use fhir::r5::resources::{AllergyIntolerance, Bundle, Patient};
use fhir::r5::types;
use fhir::r5::types::reference::Reference;

fn main() {
    // An AllergyIntolerance whose `patient` field is typed: the specification
    // gives that element exactly one targetProfile, so the generator did too.
    let allergy: AllergyIntolerance = serde_json::from_value(serde_json::json!({
        "resourceType": "AllergyIntolerance",
        "id": "al-1",
        "patient": { "reference": "Patient/pat-1" }
    }))
    .expect("valid AllergyIntolerance");

    // The field's type carries the target. This line only compiles because
    // `patient` is `Reference<Patient>`:
    let patient_ref: &types::Reference<Patient> = &allergy.patient;
    println!(
        "AllergyIntolerance.patient -> {}",
        patient_ref.reference.as_ref().map_or("?", |r| &r.0)
    );

    // Resolving inside a Bundle checks the marker against the entry.
    let bundle: Bundle = serde_json::from_value(serde_json::json!({
        "resourceType": "Bundle",
        "type": "collection",
        "entry": [
            { "resource": { "resourceType": "Patient", "id": "pat-1",
                            "name": [{ "family": "Chalmers" }] } }
        ]
    }))
    .expect("valid Bundle");

    let resolved = patient_ref.resolve(&bundle).expect("resolves");
    println!(
        "resolved to a {} named {}",
        resolved["resourceType"].as_str().unwrap_or("?"),
        resolved["name"][0]["family"].as_str().unwrap_or("?")
    );

    // A wrong-type entry is refused even when the reference string matches:
    // the same id under resourceType Observation resolves to nothing.
    let wrong: Bundle = serde_json::from_value(serde_json::json!({
        "resourceType": "Bundle",
        "type": "collection",
        "entry": [
            { "resource": { "resourceType": "Observation", "id": "pat-1", "status": "final",
                            "code": { "text": "not a patient" } } }
        ]
    }))
    .expect("valid Bundle");
    assert!(patient_ref.resolve(&wrong).is_none());
    println!("a matching id under the wrong resourceType is refused");

    // The untyped form still exists where the spec allows many targets, and
    // `cast` moves between the two when the caller knows better than the type.
    let any: Reference = serde_json::from_value(serde_json::json!({
        "reference": "Patient/pat-1"
    }))
    .expect("valid Reference");
    let typed: Reference<Patient> = any.cast();
    assert!(typed.resolve(&bundle).is_some());
    println!("cast: Reference<Any> -> Reference<Patient> (wire form unchanged)");
}
```

*Source: [`fhir/examples/typed_references.rs`](../fhir/examples/typed_references.rs) in the repository.*

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven International and their use of these trademarks does not constitute an endorsement by HL7.
