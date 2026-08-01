# Model mapping

The model is generated from the FHIR specification, and the mapping from FHIR to
Rust is uniform and predictable.

## Cardinality

| FHIR cardinality | Rust type        |
|------------------|------------------|
| `0..1`           | `Option<T>`      |
| `1..1`           | `T`              |
| `0..*`           | `Vec<T>`         |
| `1..*`           | `Vec1<T>`        |

A `1..*` element uses [`vec1::Vec1<T>`](https://docs.rs/vec1) (a non-empty vector),
so the "at least one" constraint holds at compile time; a `0..*` element is a bare
`Vec<T>` that is empty when absent (`#[serde(default, skip_serializing_if =
"Vec::is_empty")]`).

## Primitives are transparent newtypes

A FHIR primitive is a thin newtype whose JSON form is just the underlying value:

```rust
use fhir::r5::types::{Boolean, Code, Integer64};

assert_eq!(serde_json::to_value(Code("final".to_string())).unwrap(), "final");
assert_eq!(serde_json::to_value(Boolean(true)).unwrap(), true);
// `integer64` serializes as a JSON *string*, per the FHIR spec:
assert_eq!(serde_json::to_value(Integer64(9_000_000_000)).unwrap(), "9000000000");
```

## Complex datatypes and backbone elements

Complex datatypes (`HumanName`, `CodeableConcept`, …) are plain structs. Nested
*backbone* elements become nested structs named `<Parent><Field>` — e.g.
`Patient.contact` is `PatientContact`, `Bundle.entry` is `BundleEntry`.

## Choice elements (`value[x]`)

A `value[x]` choice is one generated enum per element, held via
`#[serde(flatten)]`, so exactly one type is set at compile time:

```rust
use fhir::r5::resources::Observation;
use fhir::r5::resources::observation::ObservationValue;
use fhir::r5::types::Quantity;

let mut obs = Observation::default();
obs.value = Some(ObservationValue::Quantity(Box::new(Quantity {
    value: Some(fhir::r5::types::Decimal(serde_json::Number::from(98))),
    ..Default::default()
})));
```

See [JSON serialization](json-serialization.md) for the wire format.

## Coded fields

A `required`-binding coded field is typed as its code enum, wrapped in `Coded<E>`
so out-of-value-set codes still round-trip. See
[Terminology and codes](terminology-and-codes.md).

## References

`Reference` is `Reference<T = Any>`, a phantom-typed newtype over the same wire
form. `Reference<Any>` (the default) is the untyped reference; `.resolve(&bundle)`
looks a reference up within a `Bundle`.
