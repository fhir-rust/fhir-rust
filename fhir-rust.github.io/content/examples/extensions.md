# `extensions` — Read and write FHIR® extensions ergonomically with `ExtensionExt`

Run with:

```sh
cargo run --example extensions
```

Extensions are a `Vec<Extension>` keyed by `url`. The `ExtensionExt` trait
(in the prelude) adds `extension(url)`, `extensions(url)`, `set_extension`,
and `add_extension` to every resource and datatype that carries them.

## The program

```rust
use fhir::prelude::*;
use fhir::r5::choice::Primitive;
use fhir::r5::types::extension::ExtensionValue;

const EYE_COLOR: &str = "http://example.org/fhir/StructureDefinition/eye-color";

fn main() {
    let mut patient = Patient::default();

    // Set an extension (replaces any existing one with the same url).
    patient.set_extension(Extension {
        url: FhirString(EYE_COLOR.to_string()),
        value: Some(ExtensionValue::String(Primitive::new(FhirString(
            "blue".to_string(),
        )))),
        ..Default::default()
    });

    // Read it back by url.
    if let Some(ext) = patient.extension(EYE_COLOR)
        && let Some(ExtensionValue::String(p)) = &ext.value
    {
        println!("eye color: {}", p.value.0);
    }

    // Setting again replaces rather than duplicates.
    patient.set_extension(Extension {
        url: FhirString(EYE_COLOR.to_string()),
        value: Some(ExtensionValue::String(Primitive::new(FhirString(
            "green".to_string(),
        )))),
        ..Default::default()
    });
    println!(
        "extensions with that url: {}",
        patient.extensions(EYE_COLOR).len()
    );

    let json = serde_json::to_string_pretty(&patient).expect("serialize");
    println!("{json}");
}
```

*Source: [`fhir/examples/extensions.rs`](../fhir/examples/extensions.rs) in the repository.*

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven International and their use of these trademarks does not constitute an endorsement by HL7.
