# Glossary

Shared vocabulary for FHIR and this project, so the specs, the code, and agents
all use terms the same way.

## FHIR concepts

- **FHIR** — Fast Healthcare Interoperability Resources; the HL7 standard for
  exchanging electronic health records.
- **Release** — a published version of FHIR. This crate models three, as
  separate module trees and separate cargo features:
  - **R5** — FHIR Release 5 (5.0.0), under `fhir::r5`; the default.
  - **R4** — FHIR Release 4 (4.0.1), under `fhir::r4`; opt-in.
  - **R3** — FHIR Release 3 (3.0.2), also published as **STU3**, under
    `fhir::r3`; opt-in.
  Releases are not compatible: an R3, R4 and R5 `Patient` are three different
  Rust types. See [`../spec/12-fhir-releases.md`](../spec/12-fhir-releases.md).
- **Resource** — a top-level FHIR entity that can stand alone and be exchanged
  (e.g. `Patient`, `Observation`, `Encounter`). Carries a `resourceType`.
- **DomainResource** — the base for most resources; adds `text` (narrative),
  `contained`, `extension`, `modifierExtension`.
- **Datatype** — a reusable value structure used inside resources. Split into:
  - **Primitive type** — a single scalar value (`string`, `boolean`, `code`,
    `dateTime`, `decimal`, …). Lowercase initial in FHIR.
  - **Complex type** — a structured value (`CodeableConcept`, `Reference`,
    `Period`, `Quantity`, …). Uppercase initial in FHIR.
- **Element** — the unit an `ElementDefinition` describes; every field of every
  resource/datatype is an element with a path, cardinality, and type(s).
- **Backbone element** — an anonymous nested structure inside a resource (e.g.
  `Patient.contact`). Modeled here as a named nested struct.
- **Cardinality** — `min..max` occurrence count, e.g. `0..1`, `1..1`, `0..*`.
- **Choice element `[x]`** — an element that may be one of several types, e.g.
  `Observation.value[x]`. Modeled as a single generated `FhirChoice` enum (one
  variant per type), serializing to `valueQuantity` | `valueString` | ….
- **CodeSystem** — a set of coded concepts (codes) with meanings.
- **ValueSet** — a selected set of codes drawn from one or more CodeSystems.
- **ConceptMap** — a mapping between codes in different systems.
- **Canonical URL** — the stable, version-independent identifier of a
  conformance resource (StructureDefinition, ValueSet, …).
- **StructureDefinition** — the FHIR metadata resource that defines a
  resource/datatype/profile; the generator reads these.
- **Snapshot / Differential** — the fully-resolved vs. delta list of elements
  in a StructureDefinition. The generator uses the snapshot.
- **Narrative** — human-readable XHTML summary of a resource (`text`).
- **Extension** — the FHIR mechanism for adding data not in the base
  definition; every element may carry extensions.
- **Profile** — a StructureDefinition that constrains an existing type while
  keeping that type's element paths. `MoneyQuantity` and `Age` are profiles on
  `Quantity`, and become their own Rust structs.
- **`contentReference`** — an element that reuses another backbone's children
  rather than declaring its own; typed as that backbone's struct.
- **`targetProfile`** — the resource types a `Reference` element may point at.

## Project terms

- **The model** — a release's Rust types under `<release>::{types, resources,
  codes}`.
- **The generator** — `crate::codegen`, which turns a release's spec JSON into
  its finished `src/<release>` tree (`cargo run -- r3`, `cargo run -- r4`).
- **The legacy parse layer** — `r5::parse`, the original R5-only generator,
  which emits a rough starting point to `tmp/out/*.rs`, plus the splicing
  generators that edit `fhir-release-5/src` in place.
- **Release-independent core** — the `fhir-core` crate (`validate`, `coded`,
  `builder`, `meta`, `temporal`, `summary`, `xml`, `client`, `release`) that
  every release re-exports rather than duplicating.
- **`Release`** — the trait naming a FHIR release as a type (`r3::R3`,
  `r4::R4`, `r5::R5`), so code can be generic over a release.
- **`#[fhir_version("r4")]`** — the derive-macro attribute naming the release
  for the few generated paths that are release-specific. Defaults to `r5`.
- **System element** — FHIR infrastructure that is a bare JSON attribute with no
  `_field` sibling: every `<Type>.id`, and `Extension.url`. R4/R5 mark these
  with a FHIRPath system type; R3 does not, so the generator decides
  structurally.
- **The green gate** — build + tests + doctests + `clippy --all-targets` all
  clean; the release bar for every change.
- **Newtype primitive** — a primitive represented as `struct X(pub Inner)` that
  serializes transparently as its inner scalar.
- **`Coded<E>`** — the wrapper for a `required`-binding coded field: a
  `Known(E)` code-enum value or an `Unknown(String)` wire-compatible fallback.
- **Non-empty vector (`Vec1`)** — [`vec1::Vec1<T>`](https://docs.rs/vec1), the
  Rust type for a `1..*` element; guarantees ≥1 element at compile time. (A
  `0..*` element is a bare `Vec<T>`.)
- **Primitive extension (`_field`)** — the `<field>_ext` sibling of type
  `Element` that carries `id`/`extension` on a primitive value, serialized to
  the FHIR `_field` key.
- **Builder** — the `#[derive(Builder)]`-generated `Type::builder()…build()`,
  which enforces required (`1..1`) fields at `build()`.
- **Prelude** — `fhir::r5::prelude` / `fhir::r4::prelude`, a re-export of the
  most-used items. `fhir::prelude` is the R5 one, kept at the root because R5
  is the default release.
- **Typed reference** — `Reference<T = Any>`, a phantom-typed newtype over the
  FHIR reference wire form; `Reference<Any>` is the untyped default. R5 only,
  and not yet used by any generated field.
- **Meta table** — `<release>::meta`, the compile-time path-keyed table of
  per-element cardinality, bindings, choice types, reference targets, and
  summary membership. Its types are shared (`crate::meta`); the data is
  generated per release.
- **Round-trip-of-default** — the preferred test/doctest pattern: serialize a
  `Default` value and deserialize it back to an equal value.
- **Nested backbone struct** — a Rust struct named `<Parent><Field>` (e.g.
  `TimingRepeat`) representing a FHIR backbone element.
- **Spec-driven development** — behaviour is written in `spec/*` first, then
  implemented; the specs are the source of truth.

## Crate identity

- Package/crate name: **`fhir`** (import as `use fhir::…`).
- Workspace member: **`fhir-derive-macros`** (proc-macro crate).
