# Getting started

## Install

```toml
[dependencies]
fhir = "1"
serde_json = "1" # or any other serde data format
```

That gives you FHIR R5. For R4 or R3, or several releases at once, see
[FHIR releases](fhir-releases.md).

The crate is imported as `fhir`. A `fhir::prelude` re-exports the most-used
items (`fhir::r4::prelude` and `fhir::r3::prelude` are its counterparts):

```rust
use fhir::prelude::*;
```

## Build a resource

Every resource and general-purpose datatype has a builder. Required (`1..1`)
fields are enforced at `build()`:

```rust
use fhir::r5::resources::Patient;
use fhir::r5::coded::Coded;
use fhir::r5::codes::AdministrativeGender;
use fhir::r5::types::{Boolean, String as FhirString};

let patient = Patient::builder()
    .id(FhirString("pat-1".to_string()))
    .active(Boolean(true))
    .gender(Coded::Known(AdministrativeGender::Male))
    .build()
    .expect("Patient has no required fields");
```

Or construct the struct directly and spread `..Default::default()`:

```rust
# use fhir::r5::resources::Patient;
# use fhir::r5::types::{Boolean, String as FhirString};
let patient = Patient {
    id: Some(FhirString("pat-1".to_string())),
    active: Some(Boolean(true)),
    ..Default::default()
};
```

## Serialize and parse

Everything derives `serde::Serialize`/`Deserialize`, so you work through
`serde_json`:

```rust
# use fhir::r5::resources::Patient;
# let patient = Patient::default();
let json = serde_json::to_string_pretty(&patient).unwrap();
let parsed: Patient = serde_json::from_str(&json).unwrap();
assert_eq!(parsed, patient);
```

## Read a resource of unknown type

When you receive JSON but do not know its `resourceType`, deserialize into the
polymorphic `Resource` enum:

```rust
use fhir::r5::resources::Resource;

let json = serde_json::json!({ "resourceType": "Patient", "id": "pat-1" });
match serde_json::from_value(json).unwrap() {
    Resource::Patient(patient) => assert_eq!(patient.id.unwrap().0, "pat-1"),
    _ => unreachable!(),
}
```

## Where to go next

- [Model mapping](model-mapping.md) — how FHIR maps to Rust types.
- [Validation](validation.md) — check resources against the spec.
- The runnable [`examples/`](https://github.com/joelparkerhenderson/fhir-rust-crate/tree/main/examples)
  directory.
