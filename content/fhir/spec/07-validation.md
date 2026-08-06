# 07 — Validation

Defines the `Validate` trait, its primitive checks, and the
`#[derive(Validate)]` procedural macro.

Validation is release-independent in shape and release-specific only in its
primitive format rules and its element metadata, so the trait is shared and each
release supplies the rest.

## Background

Beyond Rust's type system, FHIR imposes constraints (primitive regexes,
cardinality, invariants). This crate provides a lightweight, dependency-free
validation layer that walks a value and reports issues.

## Requirements

### The trait

- **R7.1** `fhir::validate` MUST define, once for every release:

  ```rust
  pub struct ValidationIssue { pub path: String, pub message: String }
  pub trait Validate {
      fn validate(&self) -> Vec<ValidationIssue>;
      fn is_valid(&self) -> bool { self.validate().is_empty() }
  }
  ```

  An empty result means valid.

- **R7.2** Blanket impls MUST cover `Option<T>`, `Vec<T>`, `vec1::Vec1<T>`,
  `Box<T>` and `PhantomData<T>` for `T: Validate`, and `::serde_json::Value` and
  `String` (both structurally valid).
- **R7.2a** `fhir::r4::validate::Validate` and `fhir::r5::validate::Validate`
  MUST be re-exports of that one trait, not two traits. A generic function
  bounded on it therefore accepts values of any release, and one
  `#[derive(Validate)]` implementation serves them all.

### Primitive checks

- **R7.3** Each release MUST implement `Validate` for **its own** primitives,
  with the FHIR format constraint where one exists. The rules are shared
  helpers, so releases cannot drift apart on what a valid `code` is:
  - `code`: non-empty, trimmed, single internal spaces
    (`[^\s]+(\s[^\s]+)*`).
  - `id`: 1..=64 chars from `[A-Za-z0-9-.]`.
  - `oid`: begins `urn:oid:`; `uuid`: begins `urn:uuid:`.
  - `uri`, `canonical`, `url`: non-empty and not surrounded by whitespace.
  - Other primitives are valid by construction (no extra check yet).

  A release implements only the primitives it defines: R3 has no `canonical` or
  `url`, so it checks two fewer types than R4 and R5 (spec 02).

### The derive macro

- **R7.4** The `fhir-derive-macros` crate MUST provide `#[derive(Validate)]` that
  generates a recursive `crate::validate::Validate` implementation:
  - For a **struct**: validate every field; prefix each returned issue's `path`
    with the field name, dot-separated (e.g. a bad `Coding.code` yields path
    `code.code`).
  - For an **enum**: match the active variant and validate its data; unit
    variants produce no issues.
- **R7.5** Every complex datatype, resource, nested backbone struct, and the
  `Resource` enum MUST `#[derive(Validate)]`. Primitives implement it by hand
  (R7.3), so they MUST NOT derive it.
- **R7.6** Validation MUST NOT allocate or fail for valid values beyond the
  returned `Vec` (no panics, no I/O).
- **R7.7** Where the macro must name a release — the `meta` table it consults
  for cardinality — it MUST resolve it from the type's `#[fhir_version("…")]`
  attribute (spec 12, R12.11), defaulting to R5.

### Bridging to `OperationOutcome`

- **R7.8** Each release MUST provide `From<Vec<ValidationIssue>>` for its own
  `OperationOutcome`, mapping each issue to an `error`/`invalid` entry with the
  message in `diagnostics` and the path in `expression`. An empty issue list
  MUST produce one `information`/`informational` entry, because
  `OperationOutcome.issue` is `1..*`.

### Metadata-driven checks

Beyond primitive formats, `Validate` reports two problems that the Rust types
cannot express on their own, using the release's `meta` table (spec 08):

- **R7.9 Empty `1..*` elements.** A mandatory repeating element must have at
  least one entry. Because a bare `Vec<T>` is also used for `0..*`, cardinality
  MUST be read from `meta` at validation time (via `meta::struct_prefix` plus
  the field's FHIR name), not inferred from the Rust type. Reported at the
  field's path.
- **R7.10 Required-binding codes.** A `required`-binding field is typed
  `Coded<Enum>` (spec 05); a value that fell back to `Coded::Unknown` is by
  definition outside the value set, and MUST be reported at `<field>.code`.

### Invariants

- **R7.11** The structurally checkable FHIR invariants MUST be enforced by the
  derive macro, and every invariant that is not enforced MUST be enumerated
  rather than silently ignored. See [spec 10](10-invariants-coverage.md).

## Future work

- A `#[derive(Validate)]` field attribute for cardinality/invariant metadata,
  removing the run-time `meta` lookup.
- FHIRPath-based invariant (`constraint`) evaluation.
- Date/time/base64 format checks for the remaining primitives.
- Checking `extensible` and `preferred` bindings, which needs value-set
  membership (spec 05, Future work).

## Acceptance criteria

1. `Id("patient-1").is_valid()` is true; `Id("bad id!").is_valid()` is false.
2. A `Coding` with `code = "bad  code"` yields exactly one issue with path
   `code.code`.
3. A default-constructed resource validates with no panics.
4. An empty `1..*` element yields a pathed issue; an empty `0..*` `Vec` does not.
5. A `Coded::Unknown` value yields a `<field>.code` issue.
6. A function bounded on `fhir::validate::Validate` accepts values from every
   release.
7. `fhir-derive-macros` builds and the whole model derives `Validate` without
   clippy warnings.
