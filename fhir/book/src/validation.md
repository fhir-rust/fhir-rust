# Validation

The `Validate` trait reports every problem it finds as a `ValidationIssue`
`{ path, message }`. Validation is **recursive**: calling `.validate()` on a
resource walks every field, prefixing each nested issue's `path` with the field
name.

```rust
use fhir::r5::resources::Patient;
use fhir::r5::types::Uri;
use fhir::r5::validate::Validate;

let mut patient = Patient::default();
assert!(patient.validate().is_empty());

patient.implicit_rules = Some(Uri(" http://bad ".to_string())); // whitespace-wrapped
let issues = patient.validate();
assert_eq!(issues.len(), 1);
assert_eq!(issues[0].path, "implicit_rules.uri");
```

## What is checked

- **Primitive formats.** `Code`, `Id`, `Uri`, `Oid`, `Uuid`, `Canonical`, and
  `Url` check their FHIR constraints (e.g. an `Id` matches `[A-Za-z0-9-.]{1,64}`).
- **Cardinality.** An empty `1..*` element is reported. Because bare `Vec` is
  also used for some `0..*` fields, the real cardinality is read from the
  metadata table at validation time.
- **Required-binding codes.** A coded field is typed `Coded<Enum>`; a value that
  fell back to `Coded::Unknown` is outside the value set and is reported.
- **Invariants.** A structurally-checkable subset: `ext-1` (an extension has a
  value XOR nested extensions), and `dom-2`/`dom-4` (rules on contained
  resources). Coverage of all 314 constraint keys is tracked in
  [`spec/10-invariants-coverage.md`](https://github.com/joelparkerhenderson/fhir-rust-crate/blob/main/spec/10-invariants-coverage.md).

## Bridging to `OperationOutcome`

A FHIR server reports problems as an `OperationOutcome`. Validation output drops
straight into one:

```rust
use fhir::r5::resources::Patient;
use fhir::r5::resources::operation_outcome::OperationOutcome;
use fhir::r5::types::Uri;
use fhir::r5::validate::Validate;

let mut patient = Patient::default();
patient.implicit_rules = Some(Uri(" bad ".to_string()));
let outcome: OperationOutcome = patient.validate().into();
assert_eq!(outcome.issue.len(), 1);
```
