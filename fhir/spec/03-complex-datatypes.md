# 03 — Complex datatypes

Defines how the FHIR complex datatypes are represented in Rust.

Applies to every modelled release.

| Release | Complex datatypes |
| --- | --- |
| R5 | 50 |
| R4 | 43 |
| R3 | 36 |

## Background

Complex datatypes are structured values reused inside resources
(`CodeableConcept`, `Reference`, `Period`, `Quantity`, `Identifier`, `Timing`,
`Dosage`, `ElementDefinition`, …). They have an uppercase initial letter in
FHIR.

## Requirements

- **R3.1** Each complex datatype MUST be a Rust struct following the canonical
  conventions of spec 06: the standard derive set (including `Validate`),
  `#[serde_with::skip_serializing_none]`, and
  `#[serde(rename_all = "camelCase")]`.
- **R3.2** Fields MUST use the cardinality mapping of spec 06
  (`0..1`→`Option<T>`, `1..1`→`T`, `0..*`→`Vec<T>`, `1..*`→`vec1::Vec1<T>`).
- **R3.3** Field types MUST reference `types::X` (another datatype or a
  primitive), a nested backbone struct in the same file, or
  `::serde_json::Value` where the FHIR type is `Resource`/`DomainResource`.
- **R3.4** **Nested backbone elements** MUST be modelled as named nested structs
  in the same module, named by concatenating the PascalCase path segments (e.g.
  `Timing.repeat` → `TimingRepeat`, `Dosage.doseAndRate` →
  `DosageDoseAndRate`). The parent field references the nested struct directly.
  Nesting recurses to any depth. Each nested struct carries the full derive set
  and serde attributes.
- **R3.5** Choice elements `[x]` MUST expand per spec 06 and spec 11.
- **R3.6** Each datatype lives in `src/<release>/types/<snake>.rs`, declared in
  `src/<release>/types.rs` (`pub mod` + `pub use <Pascal>`).
- **R3.7** A datatype that constrains another (a *profile*) MUST be named for
  itself, not for the type it constrains. `MoneyQuantity`, `SimpleQuantity`,
  `Age`, `Distance`, `Count` and `Duration` are all `type: "Quantity"` in the
  specification, and MUST be six distinct modules and structs.
- **R3.8** A type cycle MUST be broken with `Box`. `Reference` holds an
  `Identifier` which holds a `Reference`; without indirection neither type has a
  size. Only fields that store their type inline (`Option<T>`, `T`) can close a
  cycle — a `Vec<T>` already indirects, so `Extension.extension:
  Vec<types::Extension>` needs no `Box`. Which edge is boxed MUST be
  deterministic.
- **R3.9** Bare `Vec` fields using `skip_serializing_if = "Vec::is_empty"`
  MUST also declare `#[serde(default)]` (round-trip requirement, spec 06).

## Coverage and release differences

The general-purpose types (Address, Annotation, Attachment, CodeableConcept,
Coding, ContactPoint, HumanName, Identifier, Period, Quantity, Range, Ratio,
Reference, SampledData, Signature, Timing, …), the metadata types
(ContactDetail, DataRequirement, Expression, ParameterDefinition,
RelatedArtifact, TriggerDefinition, UsageContext, …) and the special-purpose
types (Dosage, ElementDefinition, Extension, Meta, Narrative, …) exist in both
releases.

R3's 36 datatypes are a strict subset of R4's 43, which is *not* a subset of
R5's 50:

| | Datatypes it adds over the previous release | Datatypes it drops |
| --- | --- | --- |
| R4 over R3 | `Expression`, `MarketingStatus`, `MoneyQuantity`, `Population`, `ProdCharacteristic`, `ProductShelfLife`, `SubstanceAmount` (and the `canonical`/`url` primitives) | — |
| R5 over R4 | `Availability`, `CodeableReference`, `ExtendedContactDetail`, `MonetaryComponent`, `RatioRange`, `VirtualServiceDetail`, and the abstract bases `Base`, `DataType`, `PrimitiveType`, `BackboneType` (and the `integer64` primitive) | `Population`, `ProdCharacteristic`, `SubstanceAmount` |

These differences are why a datatype from one release MUST NOT be used to model
another (spec 12). Even where a datatype exists in every release its elements
may differ: `Extension.url` is a `uri` in R3 and a `string` afterwards.

## Documentation

- Module header with FHIR name, canonical URL, version, one-line description,
  and a link to the published specification.
- A struct-level `# Examples` doctest that round-trips the default value
  (marked `ignore` for the few datatypes with a `1..*`/`Vec1` field, which have
  no `Default`).
- One-line `///` per public field, from the FHIR `short` text.

## Acceptance criteria

1. Every complex datatype module its release defines exists and re-exports from
   `types.rs`.
2. Backbone-bearing types (Timing, Dosage, ElementDefinition, DataRequirement,
   …) define the required nested structs with no duplicate fields.
3. Every datatype round-trips its default value through JSON.
4. Every datatype derives `Validate` and participates in recursive validation
   (spec 07).
5. Build, tests, doctests, and pedantic clippy are clean for every release.
