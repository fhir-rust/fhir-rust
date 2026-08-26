# 04 — Resources

Defines how the FHIR® resources are represented, and the polymorphic `Resource`
enum.

Applies to every modelled release.

| Release | Resources |
| --- | --- |
| R6 | 161 |
| R5 | 158 |
| R4 | 146 |
| R3 | 117 |
| R2 | 94 |

## Background

A resource is a top-level FHIR entity that can be exchanged on its own
(`Patient`, `Observation`, `Encounter`, `Bundle`, …). Most inherit from
`DomainResource`, which adds `text`, `contained`, `extension`, and
`modifierExtension`; all inherit `id`, `meta`, `implicitRules`, and `language`.

## Requirements

- **R4.1** Each resource MUST be a Rust struct following the canonical
  conventions (spec 06), including `#[derive(…, Validate)]` and
  `#[derive(Builder)]`.
- **R4.2** Fields MUST use the cardinality mapping of spec 06 and reference
  `types::X`, a nested backbone struct, or `::serde_json::Value`.
- **R4.3** **Nested backbone elements** MUST be modelled as named nested structs
  named by concatenating PascalCase path segments (e.g. `Patient.contact` →
  `PatientContact`, `Claim.item.detail.subDetail` → `ClaimItemDetailSubDetail`),
  recursing to any depth. This is the same rule as datatypes (spec 03, R3.4).
- **R4.4** An element that reuses another backbone's children via
  `contentReference` MUST be typed as that backbone's struct rather than
  duplicating it. Releases spell the reference differently — R4 writes a bare
  fragment (`#Observation.referenceRange`), R5 a full canonical URL with the
  same fragment — and both MUST resolve identically.
- **R4.5** An element whose FHIR type is `Resource`/`DomainResource` MUST be
  represented as `::serde_json::Value` — such a slot may hold any resource at
  all, and its consumers (`bundle_util`, `Parameters`) dispatch on the JSON —
  **except `contained`**, which MUST be the release's own
  `resources::Resource` enum: it holds this release's resources and nothing
  else, and typing it makes contained resources validate with their
  container. *(Amended 2026-08-06, T47; before the amendment `contained` was
  also required to stay raw.)*
- **R4.6** Each resource lives in `fhir-rN/src/resources/<snake>.rs`,
  declared in `fhir-rN/src/resources.rs` (`pub mod` + `pub use <Pascal>`).
- **R4.7** `fhir-rN/src/resources.rs` MUST define a **polymorphic
  `Resource` enum**:

  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
  #[serde(tag = "resourceType")]
  pub enum Resource {
      Patient(Box<Patient>),
      Observation(Box<Observation>),
      // … one Box<T> variant per resource …
  }
  ```

  Variants MUST be `Box`ed (to keep the enum small) and the serde tag MUST be
  `resourceType`, so a JSON object `{"resourceType":"Patient", …}` deserializes
  to `Resource::Patient(..)`.
- **R4.8** Only concrete resources are modelled. The abstract bases (`Resource`,
  `DomainResource`) contribute their elements to every resource's snapshot and
  MUST NOT become structs of their own.

## Release differences

The releases do not agree on the resource list, and it grew with each one: 94
in R2, 117 in R3, 146 in R4, 158 in R5, 161 in R6. R4 has 20 resources R5 does
not (`CatalogEntry`,
`DeviceUseStatement`, `DocumentManifest`, `Media`, `RequestGroup`, the
`MedicinalProduct*` family, …); R5 has 32 that R4 does not (`ActorDefinition`,
`Citation`, `Ingredient`, `Permission`, `RequestOrchestration`,
`SubscriptionTopic`, `Transport`, …).

Resources present in more than one release often differ in their elements, and
sometimes in the type of an element they share. A resource's own `id` is typed
`id` in R3 and `string` in R4/R5, which the models reflect faithfully rather
than normalizing. See spec 12.

## Documentation

- Module header with FHIR name, canonical URL, version, one-line description,
  and a link to the published specification.
- A struct-level `# Examples` doctest that round-trips the default value.
- One-line `///` per public field, from the FHIR `short` text.

## Notes and future work

- Individual resource structs do **not** carry a `resourceType` field; the
  discriminator is handled by the `Resource` enum. Adding a per-struct
  `resourceType` for standalone serialization is future work.
- Every release provides a phantom-typed `Reference<T>` with a `ResourceType`
  marker trait, so a reference can name its target at compile time; the
  generator emits the machinery, one marker impl per resource, and — where an
  element's `targetProfile` names exactly one modelled resource — the typed
  field itself (`AllergyIntolerance.patient: Reference<Patient>`). Multiple
  or abstract targets stay `Reference<Any>`, the default parameter, and the
  wire form is identical either way. *(Rolled out 2026-08-09, T11; until
  then the machinery was an R5-only hand-written prototype no field used.)*

## Acceptance criteria

1. Every resource module its release defines exists and re-exports from
   `resources.rs`.
2. The `Resource` enum has one `Box<T>` variant per resource and deserializes by
   `resourceType`.
3. Backbone-heavy resources (ExplanationOfBenefit, Claim, CapabilityStatement,
   …) define all required nested structs with no duplicate fields.
4. Every resource round-trips its default value through JSON and derives
   `Validate`.
5. Build, tests, doctests, and pedantic clippy are clean for every release.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
