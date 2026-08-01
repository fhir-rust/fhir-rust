# JSON serialization

The model round-trips to and from canonical FHIR JSON. This crate is checked
against the **official FHIR R5 example set** (~2800 files); 99.9% round-trip
exactly, and the remainder are documented in
[`tasks-roundtrip-failures.md`](https://github.com/joelparkerhenderson/fhir-rust-crate/blob/main/tasks-roundtrip-failures.md).

## The basics

- Unset optional fields are **omitted** (`skip_serializing_none`).
- Field names are `camelCase`; Rust fields are `snake_case`.
- A **bare resource struct does not emit `resourceType`** — that tag is added by
  the `Resource` enum. Round-trip through `Resource`, or expect no `resourceType`
  on a bare struct.

## Choice elements on the wire

A `value[x]` choice serializes to exactly one `value<Type>` key on the parent
object (`valueQuantity`, `valueString`, …), thanks to `#[serde(flatten)]` on the
enum:

```rust
use fhir::r5::resources::Observation;
use fhir::r5::resources::observation::ObservationValue;
use fhir::r5::choice::Primitive;
use fhir::r5::types::String as FhirString;

let mut obs = Observation::default();
obs.value = Some(ObservationValue::String(Primitive::new(FhirString("positive".into()))));
let v = serde_json::to_value(&obs).unwrap();
assert_eq!(v["valueString"], "positive");
```

## Primitive extensions (`_field`)

In FHIR, even a primitive value may carry an `id` and `extension`s. JSON puts
those in a *sibling* property with a leading underscore: `birthDate` holds the
value, `_birthDate` its extensions. The model represents the sibling as a
`<field>_ext` field of type `Element`:

```rust
use fhir::r5::resources::Patient;
use fhir::r5::types::{Date, Element, Extension, String as FhirString};

let patient = Patient {
    birth_date: Some(Date("1970-03-25".into())),
    birth_date_ext: Some(Element {
        extension: vec![Extension {
            url: FhirString("http://hl7.org/fhir/StructureDefinition/patient-birthTime".into()),
            ..Default::default()
        }],
        ..Default::default()
    }),
    ..Default::default()
};
let v = serde_json::to_value(&patient).unwrap();
assert!(v.get("_birthDate").is_some());
```

Repeating primitives use `Vec<Option<Element>>`, aligned position-by-
position with the value array (JSON `null` where an entry has no extension). See
[`spec/09-primitive-extensions.md`](https://github.com/joelparkerhenderson/fhir-rust-crate/blob/main/spec/09-primitive-extensions.md).

## Summary serialization

`fhir::r5::summary::to_summary_value` prunes a resource to its `_summary=true`
view (summary + mandatory elements):

```rust
use fhir::r5::resources::Patient;
use fhir::r5::summary::to_summary_value;

let p = Patient::default();
let summary = to_summary_value(&p, "Patient");
assert_eq!(summary["resourceType"], "Patient");
```
