# `r4_and_r5_side_by_side` — Use the R4 and R5 models in one program, and convert between them

Run with:

```sh
cargo run --example r4_and_r5_side_by_side --features "r4 r5"
```

`fhir::r4::resources::Patient` and `fhir::r5::resources::Patient` are
different Rust types. That is deliberate: the releases genuinely disagree,
and a type that quietly stood for both would let an R4 value be written to
an R5 endpoint with elements that mean something else there.

To move data between them, call `fhir::convert`, which converts the JSON
wire form and reports everything it changed or discarded. This example shows
a Patient whose shape is stable across the two releases, then one that is
not, so the report has something to say.

## The program

```rust
use fhir::convert;
use fhir::r4::types::String as R4String;
use fhir::r4::R4;
use fhir::r5::types::String as R5String;
use fhir::r5::R5;

fn main() {
    // An R4 Patient, as it might arrive from an R4 server.
    let r4_patient: fhir::r4::resources::Patient = serde_json::from_value(serde_json::json!({
        "resourceType": "Patient",
        "id": "pat-1",
        "active": true,
        "gender": "female",
        "name": [{ "family": "Chalmers", "given": ["Jane"] }]
    }))
    .expect("valid R4 Patient");
    println!("R4 id: {}", r4_patient.id.as_ref().map_or("", |s| &s.0));

    // Convert to R5. This Patient uses only elements the two releases agree on,
    // so the report comes back empty — and that emptiness is an assertion, not
    // an assumption.
    //
    // Note the `Resource` wrapper: `resourceType` comes from that enum's serde
    // tag, so a bare `Patient` does not say it is a Patient and there would be
    // nothing for the converter to key on. It reports that rather than guessing.
    let as_resource = fhir::r4::resources::Resource::Patient(Box::new(r4_patient.clone()));
    let converted = convert::from_typed::<R4, R5>(&as_resource);
    assert!(
        converted.report.is_lossless(),
        "expected a clean conversion, got:\n{}",
        converted.report
    );
    let r5_patient: fhir::r5::resources::Patient =
        serde_json::from_value(converted.value).expect("the converted document is valid R5");
    println!("R5 id: {}", r5_patient.id.as_ref().map_or("", |s| &s.0));

    assert_eq!(
        r4_patient.name[0].family,
        Some(R4String("Chalmers".to_string()))
    );
    assert_eq!(
        r5_patient.name[0].family,
        Some(R5String("Chalmers".to_string()))
    );

    // Not every resource carries over so cleanly. `Observation.value[x]` allows
    // 11 types in R4 and 13 in R5, and the metadata tables say so:
    let r4_value = fhir::r4::meta::element("Observation.value[x]").expect("R4 element");
    let r5_value = fhir::r5::meta::element("Observation.value[x]").expect("R5 element");
    println!(
        "Observation.value[x] allows {} types in R4 and {} in R5",
        r4_value.types.len(),
        r5_value.types.len()
    );

    let r5_only: Vec<&str> = r5_value
        .type_codes()
        .filter(|code| !r4_value.type_codes().any(|c| c == *code))
        .collect();
    println!("types R5 added: {r5_only:?}");
    assert!(r5_only.contains(&"Attachment"));

    // Going the other way, that difference costs something. An R5 Observation
    // carrying an Attachment has nowhere to put it in R4, and the report says
    // so by name instead of the value quietly evaporating.
    let r5_obs = serde_json::json!({
        "resourceType": "Observation",
        "status": "final",
        "code": { "text": "ultrasound" },
        "valueAttachment": { "title": "scan.png" }
    });
    let back = convert::between::<R5, R4>(&r5_obs);
    println!("\nR5 -> R4 report:\n{}", back.report);
    assert!(back.value.get("valueAttachment").is_none());
    assert!(back.report.discarded_data());
}
```

*Source: [`fhir/examples/r4_and_r5_side_by_side.rs`](../fhir/examples/r4_and_r5_side_by_side.rs) in the repository.*
