# Terminology and codes

FHIR `CodeSystem`s are generated as type-safe Rust enums under `fhir::r5::codes`.
Each enum serializes to its canonical code string:

```rust
use fhir::r5::codes::AdministrativeGender;

let gender = AdministrativeGender::Female;
assert_eq!(serde_json::to_value(&gender).unwrap(), "female");

let parsed: AdministrativeGender = serde_json::from_value("male".into()).unwrap();
assert_eq!(parsed, AdministrativeGender::Male);
```

## `Coded<E>`: required bindings with a fallback

A field with a `required` binding is typed as its code enum, but wrapped in
`Coded<E>` so that a code outside the value set — a newer code, a local
extension, or simply invalid data — still round-trips instead of failing to
parse:

```rust
use fhir::r5::coded::Coded;
use fhir::r5::codes::AdministrativeGender;

// A known code parses into the enum variant.
let known: Coded<AdministrativeGender> =
    serde_json::from_value(serde_json::json!("female")).unwrap();
assert_eq!(known, Coded::Known(AdministrativeGender::Female));

// An unknown code is preserved verbatim.
let unknown: Coded<AdministrativeGender> =
    serde_json::from_value(serde_json::json!("robot")).unwrap();
assert!(unknown.is_unknown());

// Both serialize back to their code string.
assert_eq!(known.code(), "female");
assert_eq!(unknown.code(), "robot");
```

A `Coded::Unknown` value is, by definition, outside its required value set, so
[validation](validation.md) reports it. See
[`spec/05-code-systems.md`](https://github.com/joelparkerhenderson/fhir-rust-crate/blob/main/spec/05-code-systems.md)
for the design rationale (why a wrapper rather than an `Other` variant on every
enum).

## The metadata table

The binding of each element — its strength and value set — is available from the
`fhir::r5::meta` table, keyed by FHIR path:

```rust
use fhir::r5::meta;

let gender = meta::element("Patient.gender").unwrap();
let binding = gender.binding.unwrap();
assert_eq!(binding.strength, meta::BindingStrength::Required);
assert!(binding.value_set.unwrap().contains("administrative-gender"));
```
