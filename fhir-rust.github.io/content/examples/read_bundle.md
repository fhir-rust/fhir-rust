# `read_bundle` — Read a FHIR® R5 `Bundle` and dispatch on each entry's resource type

Run with:

```sh
cargo run --example read_bundle
```

A search or transaction `Bundle` carries a heterogeneous list of resources.
Each `Bundle.entry.resource` is held as raw `serde_json::Value` (its type is
only known at runtime), so we deserialize each one into the polymorphic
`Resource` enum, which serde selects by the `resourceType` tag.

## The program

```rust
use fhir::r5::resources::{Bundle, Resource};

fn main() {
    // A minimal searchset bundle containing two different resource types.
    let bundle_json = serde_json::json!({
        "resourceType": "Bundle",
        "type": "searchset",
        "total": 2,
        "entry": [
            { "resource": { "resourceType": "Patient", "id": "pat-1", "active": true } },
            { "resource": { "resourceType": "Observation", "id": "obs-1",
                            "status": "final",
                            "code": { "text": "Body temperature" } } }
        ]
    });

    // Parse the envelope. `Bundle.type` is required (`1..1`).
    let bundle: Bundle = serde_json::from_value(bundle_json).expect("parse Bundle");
    println!("Bundle type: {}", bundle.r#type.code());

    // Walk the entries, deserializing each contained resource by its type tag.
    for entry in bundle.entry.into_iter() {
        let Some(value) = entry.resource else {
            continue;
        };
        match serde_json::from_value(value).expect("parse entry resource") {
            Resource::Patient(patient) => {
                println!(
                    "- Patient {} (active: {:?})",
                    patient.id.as_ref().map_or("?", |id| &id.0),
                    patient.active.map(|b| b.0),
                );
            }
            Resource::Observation(observation) => {
                println!(
                    "- Observation {} (status: {})",
                    observation.id.as_ref().map_or("?", |id| &id.0),
                    observation.status.code(),
                );
            }
            other => println!("- some other resource: {other:?}"),
        }
    }
}
```

*Source: [`fhir/examples/read_bundle.rs`](../fhir/examples/read_bundle.rs) in the repository.*

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven International and their use of these trademarks does not constitute an endorsement by HL7.
