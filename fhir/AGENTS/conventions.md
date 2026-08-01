# Conventions

These conventions are **mandatory** and uniform across the whole model, and
across every FHIR release. The generator emits them and hand-authored code must
match them exactly, so that every datatype and resource looks and behaves the
same. The normative details live in
[`../spec/06-serialization.md`](../spec/06-serialization.md).

Examples below use R5 paths. Everything applies verbatim to R4 and R3 with `r5`
replaced — with one addition: a type outside the default release carries
`#[fhir_version("r4")]` (or `"r3"`) so the derive macros know which `meta`
table, `types::Element`, and `choice::Primitive` to name.

## The canonical struct

Every complex datatype and resource struct looks like this:

```rust
use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    /// Short description from the FHIR ElementDefinition.
    pub field: Option<types::String>,
}
```

The R4 equivalent is the same with `crate::r4::types` and one extra attribute
(R3 likewise, with `"r3"`):

```rust
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Example { /* … */ }
```

Rules embodied above:

- **Derives, in this order:** `Debug, Default, Clone, Serialize, Deserialize,
  PartialEq, Eq, Validate`. Never use bare `f64`/`f32` — they break `Eq`.
  A struct with a `1..*` (`Vec1`) field **cannot** derive `Default` (there is no
  empty non-empty vector); those structs drop `Default` and their `# Examples`
  doctest is marked `ignore`. Add `#[derive(Builder)]` on types that should get
  a `Type::builder()`.
- `#[serde_with::skip_serializing_none]` so `None` fields are omitted.
- `#[serde(rename_all = "camelCase")]` maps snake_case Rust fields to camelCase
  FHIR JSON keys. Do **not** add per-field `#[serde(rename)]` unless the JSON
  key cannot be produced by camelCase. It genuinely cannot when the FHIR name
  has consecutive capitals — serde only uppercases the letter after an
  underscore, so `truth_tp` becomes `truthTp`, never `truthTP`. Those fields
  (`truthTP`, `queryFP`, `carrierAIDC`, `requestURL`, …) spell their key out.
- Do **not** add `#[serde(deny_unknown_fields)]` on model structs.

## Field types and cardinality

Map the FHIR `ElementDefinition` `(min, max)` to Rust wrappers:

| FHIR cardinality | Rust type |
| --- | --- |
| `0..1` | `Option<T>` |
| `1..1` | `T` |
| `0..*` | `Vec<T>` |
| `1..*` | `vec1::Vec1<T>` |

`T` is a `types::Pascal(code)` (e.g. `types::CodeableConcept`), a nested
backbone struct, or `::serde_json::Value` for polymorphic `Resource` slots.

- **`0..*` → bare `Vec<T>`** (empty when absent), carrying
  `#[serde(default, skip_serializing_if = "Vec::is_empty")]`. The `default` is
  mandatory: without it an omitted array serializes to nothing but fails to
  deserialize (a real round-trip bug we have already hit). Construct with
  `vec![…]`, not `Some(vec![…])`; read as a slice, no `Option` unwrap.
- **`1..*` → [`vec1::Vec1<T>`](https://docs.rs/vec1)** (non-empty), so "at least
  one" is a compile-time property. These structs lose `Default` (see above).

## Coded values (required bindings)

A coded field whose binding strength is `required` is typed as its release's
`codes` enum wrapped in `fhir::coded::Coded<E>` — a
`Known(E) | Unknown(String)` untagged fallback that keeps wire compatibility
with codes outside the value set. Use `Coded<E>`, not the opaque `types::Code`,
for required bindings. `Coded<E>` is one type shared by every release; the enum
inside it is release-specific. See
[`../spec/05-code-systems.md`](../spec/05-code-systems.md).

## Choice elements `[x]`

A FHIR choice element such as `value[x]` is a single generated **enum**, one
variant per allowed type, held via `#[serde(flatten)]` with
`#[derive(FhirChoice)]`:

```rust
pub value: Option<ObservationValue>, // enum: Quantity(..) | String(..) | Boolean(..) | …
```

The enum is named `<Struct><Base>` (e.g. `ObservationValue`) and lives in the
type's module. Primitive variants use `fhir::r5::choice::Primitive<T>` (to carry
the paired `_value<Type>` extension); complex variants hold `Box<T>`. Serde
still emits the FHIR keys `valueQuantity`, `valueString`, … See
[`../spec/11-choice-types.md`](../spec/11-choice-types.md).

## Field names

- snake_case of the last path segment.
- Rust keywords become raw identifiers: `type` → `r#type`, `use` → `r#use`,
  `ref` → `r#ref`, `abstract` → `r#abstract`. `rename_all` still sees the
  unescaped name, so no explicit rename is needed.

## Primitives

Primitives are newtypes, not empty structs:

- string-like (`string`, `code`, `id`, `uri`, dates, …) → `struct X(pub String)`
- `boolean` → `struct Boolean(pub bool)`, `integer` → `i32`,
  `positiveInt`/`unsignedInt` → `u32`
- `decimal` → `struct Decimal(pub serde_json::Number)` (precision + `Eq`)
- `integer64` → `struct Integer64(pub i64)` serialized **as a JSON string**

A tuple newtype serializes transparently as its inner value. Details in
[`../spec/02-primitive-types.md`](../spec/02-primitive-types.md).

## Module wiring

- Each datatype is `src/<release>/types/<snake>.rs`, declared in
  `src/<release>/types.rs` with both `pub mod <snake>;` and
  `pub use <snake>::<Pascal>;`.
- Each resource is `src/<release>/resources/<snake>.rs`, declared the same way
  in `src/<release>/resources.rs`, which also defines the `Resource` enum.
- The module is named after the definition's `name`, not its `type`. A profile
  such as `MoneyQuantity` has `type: "Quantity"` but lives in
  `money_quantity.rs` as `struct MoneyQuantity`.

## Documentation

- Module header: `//!` block with the FHIR name, URL, Version, a one-line
  description, and the FHIR/UML links.
- Struct: a `///` doc with a `# Examples` doctest that round-trips the default
  value through `serde_json` (see [`testing.md`](testing.md)). Structs without
  `Default` (those with a `1..*`/`Vec1` field) mark that doctest `ignore`.
- Every public field: a one-line `///` from its FHIR `short` text.
