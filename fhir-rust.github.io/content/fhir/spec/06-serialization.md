# 06 — Serialization

Defines the JSON mapping between the Rust model and canonical FHIR® JSON. This is
the normative version of the conventions in
[`../agents/conventions.md`](../agents/conventions.md).

Applies to every modelled release, identically: the releases differ in *which*
elements exist, never in how an element is mapped.

## Requirements

### Struct shape

- **R6.1** Every datatype/resource struct MUST derive, in order:
  `Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate`. A
  struct with a `1..*` (`Vec1`) field is the sole exception: it omits `Default`
  (a non-empty vector has no empty value). Types that expose a builder also
  derive `Builder`.
- **R6.2** Structs MUST carry `#[serde_with::skip_serializing_none]` so `None`
  optional fields are omitted from output.
- **R6.3** Structs MUST carry `#[serde(rename_all = "camelCase")]`. Per-field
  `#[serde(rename)]` is only allowed when camelCase cannot produce the correct
  FHIR key, and MUST be present when it cannot. serde only uppercases the letter
  following an underscore, so it can never reproduce a FHIR name with
  consecutive capitals: `truth_tp` becomes `truthTp`, never `truthTP`. Fields
  such as `truthTP`, `queryFP`, `carrierAIDC` and `requestURL` therefore spell
  their key out. Omitting the rename silently drops the element on both read and
  write, which no test of default values would catch.
- **R6.4** Structs MUST NOT use `#[serde(deny_unknown_fields)]` (forward
  compatibility with unknown/extension fields).
- **R6.5** No field may be a bare `f64`/`f32` (breaks `Eq`); use
  `types::Decimal`.

### Cardinality mapping

- **R6.6** FHIR `(min, max)` maps to Rust as:

  | Cardinality | Rust |
  | --- | --- |
  | `0..1` | `Option<T>` |
  | `1..1` | `T` |
  | `0..*` | `Vec<T>` — but `::fhir_core::PrimVec<T>` when `T` is a primitive (R6.7a) |
  | `1..*` | `vec1::Vec1<T>` |

- **R6.7** A `Vec` field using `skip_serializing_if = "Vec::is_empty"` MUST also
  declare `#[serde(default)]`, so an omitted array deserializes to an empty
  `Vec` instead of erroring. (Serializing to `{}`/omitted but failing to
  deserialize it is a defect.)

- **R6.7a** *(added 2026-08-10, audit **F-86**)* A repeating **primitive**
  (`0..*`) MUST use `::fhir_core::PrimVec<T>`, not `Vec<T>`. FHIR JSON
  represents such an element as parallel arrays, and a position carrying
  only an extension is a **null** in the value array (`"event": [null]`
  beside `"_event": [{…}]`) — a position `Vec<T>` cannot hold; the model
  rejected it, and before F-87's fix then silently dropped the element.
  `PrimVec` is transparently `Vec<Option<T>>`: `None` is the extension-only
  placeholder, serialized back as the same `null`, so the wire form
  round-trips exactly. The same `#[serde(default)]` rule as R6.7 applies,
  with `PrimVec::is_empty` as the skip predicate.

  Stated residual: a `1..*` repeating primitive keeps `vec1::Vec1<T>` and
  its type-level non-emptiness, so an extension-only position in a
  *required* repeating primitive remains unrepresentable. No example in
  any official corpus (R2 spec suite, R3/R4/R4B/R5 full sets) uses that
  shape; if one ever does, the refusal is loud (F-87), not silent.

### Field naming

- **R6.8** Rust field names are snake_case of the FHIR element name; camelCase
  serde renaming reproduces the FHIR JSON key.
- **R6.9** Rust keywords MUST be raw identifiers (`r#type`, `r#use`, `r#ref`,
  `r#abstract`); serde still sees the unescaped name.

### Choice elements `[x]`

- **R6.10** A choice element MUST be a single generated Rust `enum`, one variant
  per allowed type, derived with `#[derive(FhirChoice)]` and held on the parent
  via `#[serde(flatten)]` (as `Option<<Struct><Base>>`). Exactly one variant can
  be set, so "at most one" is a compile-time property. Serialization MUST still
  produce the FHIR keys `value<Type>` (`valueQuantity`, `valueString`, …).
  Primitive variants carry the paired `_value<Type>` extension via that
  release's `choice::Primitive<T>`; complex variants hold `Box<T>`. A choice
  field is always `Option`, even where the specification makes the element
  mandatory: a choice enum has no default, and R6.1 requires one. The full
  contract is [spec 11](11-choice-types.md).

### Primitive extensions

- **R6.11** A primitive-typed element MUST carry a `_field` sibling for its `id`
  and `extension`, per [spec 09](09-primitive-extensions.md). Elements typed as
  a FHIRPath system type (`http://hl7.org/fhirpath/System.String`, used for
  `Element.id` and `Extension.url`) are not primitive *datatypes* and MUST NOT
  get one.

### Polymorphic slots

- **R6.12** `Resource`/`DomainResource`-typed elements are
  `::serde_json::Value`, except `contained`, which is the release's typed
  `Resource` enum (spec 04, R4.5 as amended 2026-08-06, T47). A top-level
  polymorphic resource uses the `Resource` enum tagged by `resourceType`
  (spec 04, R4.7); serde reads and writes the discriminator, so the wire
  form of a typed `contained` is byte-for-byte what the raw form was.

### Round-trip guarantee

- **R6.13** For any value `v`, `from_value(to_value(v)) == v` MUST hold for the
  supported representations. Tests and doctests assert this via the
  round-trip-of-default pattern (spec 07, and
  [`../agents/testing.md`](../agents/testing.md)).

## Acceptance criteria

1. A representative resource with nested backbones and choice fields round-trips
   through `serde_json`, in every release.
2. Optional `None` fields and empty `Vec`s are omitted from output and re-read
   without error.
3. FHIR keys in output are camelCase; keyword fields serialize to their FHIR
   names (`type`, `use`, …); names with consecutive capitals survive unchanged.
4. The official example resources round-trip exactly, except where the example
   itself violates the specification (spec 12, acceptance 7).
5. `cargo clippy --all-targets` reports zero warnings for every release.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
