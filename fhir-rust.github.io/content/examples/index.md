# Examples

Runnable programs from the model crate’s `examples/` directory. Each is
a tutorial in its header comment and a complete program below it; run
one from a checkout with `cargo run --example <name>` (some need extra
cargo features — the page says which).

- [`build_patient`](build_patient.md) — Build a FHIR® R5 `Patient` with the generated builder and serialize it
- [`client_crud`](client_crud.md) — FHIR REST client CRUD against the public HAPI test server (feature `client`)
- [`code_systems`](code_systems.md) — Use FHIR R5 code systems as type-safe Rust enums
- [`convert_release`](convert_release.md) — Convert a resource between FHIR releases, and read the loss report
- [`extensions`](extensions.md) — Read and write FHIR extensions ergonomically with `ExtensionExt`
- [`operation_outcome`](operation_outcome.md) — Turn validation results into a FHIR `OperationOutcome`
- [`primitive_extensions`](primitive_extensions.md) — Read and write FHIR *primitive extensions* — the `_field` siblings
- [`r3_patient`](r3_patient.md) — Build a FHIR R3 `Patient`, serialize it, validate it, and read it back
- [`r4_and_r5_side_by_side`](r4_and_r5_side_by_side.md) — Use the R4 and R5 models in one program, and convert between them
- [`r4_patient`](r4_patient.md) — Build a FHIR R4 `Patient`, serialize it, validate it, and read it back
- [`read_bundle`](read_bundle.md) — Read a FHIR R5 `Bundle` and dispatch on each entry's resource type
- [`search_response`](search_response.md) — Consume a FHIR search response: typed entries, the total, and paging
- [`transaction_bundle`](transaction_bundle.md) — Build a FHIR transaction `Bundle` and read resources back out of one
- [`tutorial`](tutorial.md) — The guide's end-to-end tutorial, as a runnable program
- [`typed_references`](typed_references.md) — Typed references: what the compiler knows about where a reference points
- [`validate_resource`](validate_resource.md) — Validate FHIR R5 values with the [`Validate`] trait

The database family’s worked examples are a guide of their own:
[Examples](../doc/examples.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven International and their use of these trademarks does not constitute an endorsement by HL7.
