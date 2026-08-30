# Where FHIR® concepts show up in this repo

Pointers, not copies. Code moves; these paths were checked against the repo
at the time of writing, but read the linked file for the current, exact
version rather than trusting a remembered snippet — especially for anything
version- or engine-specific.

## The Rust data model (a resource, a datatype, an extension)

- **A resource as a Rust type**: `fhir/fhir-r4/src/resources/` (and the
  equivalent under `fhir-r2` .. `fhir-r6`) — one file per resource type,
  e.g. `patient.rs`, `observation.rs`. Each is a struct with `serde`
  derives that round-trips to canonical FHIR JSON.
- **A datatype**: `fhir/fhir-r4/src/types/` — `human_name.rs`,
  `codeable_concept.rs`, `period.rs`, and the rest.
- **A generated, searchable index of every resource and field**:
  [`fhir/fhir.md`](../../fhir/fhir.md) — the fastest way to answer "does
  resource X have field Y" without opening source.
- **Runnable examples**, closer to a tutorial than a reference:
  `fhir/examples/` — `build_patient.rs` (constructing a resource in code),
  `extensions.rs` (reading and writing extensions), `code_systems.rs` (the
  type-safe code-system enums), `client_crud.rs` (create/read/update/delete
  patterns), `convert_release.rs` (moving a resource between FHIR versions).
  Run any of them with `cargo run --example <name>` from `fhir/`.
- **What "R2 through R6" means in code**: the feature-gated modules
  `fhir::r2` .. `fhir::r6` in the `fhir` crate (default feature is R5) — see
  `fhir/README.md` for which releases are on by default.

## Storing a resource as relational tables

- **The storage tutorials**, written to be read in order with no server
  required for the first one: `doc/tutorial-01-getting-started.md` through
  `doc/tutorial-06-porting.md`.
- **What shredding a resource into tables looks like conceptually**:
  `doc/storage-model.md` and `doc/tutorial-02-storage-model.md` — base
  tables, child tables for repeating elements, how a `Reference` becomes a
  foreign key.
- **The store API a program actually calls** (`put`, `get`, `search`,
  `history`, `vread`, `delete`, `verify_audit`, …): e.g.
  `fhir-postgresql/crates/fhir-postgresql-store/src/lib.rs` — every port has
  the same shape at `fhir-<engine>/crates/fhir-<engine>-store/src/lib.rs`,
  because that core is shared across all six (see the maintainer skill for
  why).
- **Comparing what the six engines actually support today**:
  [`doc/choosing-an-engine.md`](../../doc/choosing-an-engine.md) and the
  [conformance matrix](../../spec/databases/conformance-matrix.md) — trust
  these over a README's prose if they ever disagree.

## Audit and PHI

- **The tamper-evident audit chain**: `fhir-store/src/chain.rs` — the
  engine-agnostic implementation every port depends on and re-exports.
- **What is guaranteed about PHI, in plain language**:
  [`doc/trust-boundary.md`](../../doc/trust-boundary.md) — written for
  someone evaluating the library, not just for implementers.
- **Search and history in practice**:
  `doc/tutorial-04-search.md` (search parameters, modifiers, paging) and
  `doc/tutorial-05-history-and-audit.md` (versions, attribution, erasure).

## The REST API

- **What HTTP interactions are actually served, and their status codes**:
  [`fhir-loco/README.md`](../../fhir-loco/README.md) and
  [`fhir-loco/spec/index.md`](../../fhir-loco/spec/index.md) — this is the
  only crate in the repository that speaks HTTP; none of the six database
  ports do (that split is deliberate — see the maintainer skill).

## If an example seems to contradict this file

Prefer the source file every time — these pointers describe *where to look*,
not a frozen snapshot of what's there.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
