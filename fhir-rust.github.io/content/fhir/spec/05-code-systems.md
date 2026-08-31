# 05 — Code systems

Defines the type-safe code-system enums in each release's `codes` module, and
how a coded field is typed.

Applies to every modelled release.

| Release | Code-system enums |
| --- | --- |
| R6 | 459 |
| R5 | 442 |
| R4 | 486 |
| R3 | 386 |
| R2 | 265 |

The counts do not track release age: they follow how many `complete` code
systems each release's `valuesets.json` happens to publish. R5 moved several to
external terminologies that no FHIR® element binds to with `required` strength.

## Background

FHIR `code` values are drawn from **CodeSystems**. Many are small, closed
enumerations (`administrative-gender` → male | female | other | unknown).
Representing those as Rust enums gives compile-time safety and exhaustive
matching.

## Generating the enums

- **R5.1** For every `CodeSystem` in the release's `valuesets.json` whose
  `content` is `complete` and which has at least one concept, `codes` MUST
  provide a Rust `enum` whose variants are its codes. A system that is not
  `complete` does not list all its codes, so a closed Rust enum would reject
  valid data; those stay `types::Code`.
- **R5.2** Each enum MUST derive `Debug, Clone, Serialize, Deserialize,
  PartialEq, Eq, Default`; the first concept is the `#[default]` variant, so
  that `Coded<E>` and every struct holding one can derive `Default`.
- **R5.3** Each variant MUST serialize to its exact FHIR code string via
  `#[serde(rename = "<code>")]`, so `to_value`/`from_value` use the canonical
  codes, not the Rust identifiers.
- **R5.4** Variant identifiers MUST be sanitized deterministically:
  - PascalCase of the code, with non-alphanumeric characters treated as word
    separators.
  - A leading digit is prefixed with `N`.
  - `Self`, which is reserved even in type position, becomes `SelfCode`.
  - Collisions within an enum are de-duplicated with a numeric suffix, in
    specification order.
- **R5.5** An enum's name MUST be derived from the **last segment of the code
  system's URL**, sanitized as in R5.4 — not from its `name` — because that is
  what a value-set binding refers to. A URL need not be tidy
  (`urn:iso-astm:E1762-95:2013` is real), so the same sanitizing applies. The
  first system to claim a name keeps it.
- **R5.6** `codes` MUST be generated deterministically (same input → identical
  file) and MUST carry doc comments: the system's description, its URL wrapped
  in `<…>` so rustdoc treats it as a link, and each concept's display as the
  variant's doc.

## Typing coded fields

- **R5.7** An element with a `required` binding whose value set maps to a
  generated enum MUST be typed as that enum rather than the opaque
  `types::Code`, so the compiler enforces the value set. The enum is resolved
  from the value set URL's last segment (ignoring any `|version` suffix) by the
  same rule as R5.5, so binding and enum always agree. The releases spell the
  bound value set differently — `valueSet` in R4/R5, `valueSetReference` or
  `valueSetUri` in R3 — and all three MUST resolve identically (spec 12,
  R12.17).
- **R5.8** A coded field MUST NOT be a bare enum. A closed enum would reject any
  code outside the value set, which is unacceptable for a data-exchange library:
  real data carries newer codes, local extensions, and simply invalid values,
  and reading MUST NOT fail. The field is wrapped instead:

  ```rust
  pub enum Coded<E> {           // fhir::coded
      Known(E),                 // a recognized code from the value set
      Unknown(String),          // any other code, preserved verbatim
  }
  ```

  `Coded<E>` is `#[serde(untagged)]`: deserialization tries `E` first and falls
  back to `Unknown`; serialization emits the code string either way. So a
  required-binding field is `Option<Coded<AdministrativeGender>>`, or
  `Coded<…>` for `1..1`.

- **R5.9** `Coded<E>` is generic over the enum and therefore release-independent.
  It MUST be defined once (`fhir::coded`) and re-exported by each release, not
  duplicated.
- **R5.10** A `Coded::Unknown` value is by definition outside its required value
  set, so `Validate` MUST report it (spec 07).

### Why the wrapper rather than an `Other(String)` variant

Adding `Other(String)` to all 900+ generated enums would put the fallback in
every enum's match arms, defeat exhaustive matching, and have to be re-derived
on every regeneration. One wrapper localizes the policy, leaves `codes` a pure
translation of the specification, and makes the retype round-trip-safe even
where an enum mapping turns out to be wrong: the value simply lands in
`Unknown`.

## Future work

- Expose value-set membership so `Validate` can check `extensible` and
  `preferred` bindings, not just `required` ones.
- Provide `FromStr`/`Display` conveniences alongside serde.

## Acceptance criteria

1. `codes` contains an enum for every complete `CodeSystem` its release
   publishes with at least one concept.
2. `AdministrativeGender::Female` serializes to `"female"`; `"male"`
   deserializes to `AdministrativeGender::Male`.
3. A code outside a required value set round-trips unchanged through
   `Coded::Unknown` and is reported by `Validate`.
4. Generation is deterministic and the module compiles with zero clippy
   warnings.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
