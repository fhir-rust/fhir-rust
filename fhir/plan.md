# Improvement Plan — `fhir` (FHIR R5 for Rust)

Status: proposed. Companion file: `tasks.md` (discrete, executable tasks with
acceptance criteria). Intended to be executed incrementally by Claude
(Opus) sessions; each phase is independently shippable.

## Vision

Make this crate the best-in-class Rust implementation of HL7 FHIR R5:
faithful to the specification's JSON representation, type-safe where the spec
allows it, pleasant to use, deeply documented, and verifiable against the
official specification artifacts — then extend outward (client, XML,
multi-version).

## Current state (baseline as of 2026-07-11, commit `c83f96c`)

Done and green (build, 646 unit tests, 241 doctests, clippy pedantic, rustdoc
zero warnings):

- 158 R5 resources (`src/r5/resources/`), 50 complex datatypes + 21 primitive
  newtypes (`src/r5/types/`), 419 code-system enums (`src/r5/codes.rs`).
- Polymorphic `Resource` enum tagged by `resourceType`.
- Recursive `#[derive(Validate)]` (proc-macro crate `fhir-derive-macros/`) +
  primitive format constraints (`src/r5/validate.rs`).
- Comprehensive rustdoc: crate guide with tutorials in `lib.rs`, per-field
  docs, `# Examples` doctests on every datatype/resource; README front page;
  4 runnable `examples/`.
- Code generator (`src/r5/parse/`) over the bundled spec JSON
  (`doc/fhir-specifications/r5/fhir-definitions-json/`).

Known gaps (from `AGENTS.md` idea list + this plan's analysis): primitive
extensions, choice-type enums, coded-enum fields, typed references, real
date/time value types, deeper validation, no integration tests, no CI, no
XML, single version (R5 only), no client.

## Guiding constraints

1. **Round-trip fidelity is sacred.** Any representation change must preserve
   byte-level JSON round-trip for canonical FHIR JSON. The official spec
   examples are the oracle (Phase 1 makes them a test suite).
2. **Breaking changes are batched per minor version** (0.2, 0.3, …) since the
   crate is pre-1.0. Each phase below names its target version.
3. **Generator-first where possible.** Mass model changes (208+ structs)
   should be produced by the code generator or by scripted/agent fan-out —
   never by hand-editing 200 files ad hoc.
4. **Operational lesson (hard-won):** when fanning out parallel agents that
   edit files in the shared working tree, give them Read+Edit ONLY (no Bash);
   commit a protected baseline first. See
   `~/.claude/.../memory/parallel-file-edit-agents-no-bash.md`.

## Phases

### Phase 0 — Infrastructure & trust (target: 0.1.x patch releases)

Goal: make correctness observable and regressions impossible to miss.
No API changes.

- **CI (GitHub Actions):** build, test (unit + doc), clippy pedantic
  `--all-targets`, `cargo doc` warnings-as-errors, `cargo publish --dry-run`
  for both crates, MSRV check. Badge in README.
- **Official-examples round-trip suite:** download the FHIR R5
  `examples-json.zip` (once, as a dev asset or fetched in CI), and add
  `tests/roundtrip_official_examples.rs`: for every example JSON, deserialize
  into `Resource`, re-serialize, and compare as `serde_json::Value` equality.
  This is the single highest-value correctness investment available: it will
  surface every field-name, cardinality, and choice-type mismatch at once.
  Expect an initial failure list → burn it down file by file.
- **Publish readiness:** fix `Cargo.toml` `include` (currently lists
  `llms.txt`/`llms.json` which don't exist — either create them or remove);
  add `CHANGELOG.md`, `CONTRIBUTING.md`; docs.rs metadata
  (`[package.metadata.docs.rs]`); publish `fhir-derive-macros` first, then
  `fhir` (registry resolves the versioned dep).
- **Property tests:** proptest/arbitrary round-trip for a sample of types.

### Phase 1 — JSON fidelity: primitive extensions (target: 0.2)

FHIR JSON represents extensions on primitives as sibling `_field` objects
(e.g. `"birthDate": "1970-01-01", "_birthDate": {"extension": [...]}`,
including array alignment with `null`s). Today these are silently dropped.

- Introduce a generic carrier, e.g. `Element<T>`/`FhirField<T>` holding
  `value: Option<T>` + `id`/`extension`, OR add explicit
  `pub _field: Option<types::Element>` siblings. **Decision required** —
  prototype both on Patient; pick by ergonomics + round-trip cleanliness
  (serde for split-field pairing likely needs a custom
  `Deserialize`/`Serialize` on a wrapper, or generator-emitted `_field`
  siblings with `skip_serializing_none`). The `_field` sibling approach is
  mechanical, obvious in JSON terms, and derive-macro friendly; the wrapper
  is more ergonomic but a much deeper serde surgery. Recommendation: start
  with `_field` siblings (generator/agent fan-out, ~208 structs), reassess
  wrapper for 0.5+.
- Round-trip suite from Phase 0 gates the change (spec examples use `_field`
  heavily).

### Phase 2 — Type safety (target: 0.3)

- **Choice types as enums.** Replace flattened `value_string`/`value_quantity`
  `Option` fields with a generated
  `#[serde(untagged)]`-style enum or (better) generator-emitted
  `#[serde(flatten)]` + custom impls keyed by suffix. Guarantees "exactly one
  of" at the type level. This is the largest breaking change; do it via the
  generator + spec's `value[x]` element data, resource by resource, with the
  round-trip suite green at every step.
- **Coded fields use the code enums.** Where an element has a *required*
  binding to a complete CodeSystem already generated in `codes.rs` (e.g.
  `Patient.gender` → `AdministrativeGender`), change the field type from
  `types::Code` to the enum. Needs a binding-strength extraction pass in the
  generator, plus an `#[serde(other)]`-style escape variant for forward
  compatibility (decide: closed enum vs `Code`-fallback variant).
- **Typed references.** `Reference<T = AnyResource>` phantom-typed by
  targetProfile (e.g. `subject: Option<Reference<Patient>>`), with untyped
  fallback where targetProfile is multiple/any. Purely additive if the typed
  form derefs to the untyped struct.
- **Real value primitives.** `Date`, `DateTime`, `Instant`, `Time` currently
  wrap `String`. Add parsed accessors (year/month/day precision model per
  FHIR) without changing the stored representation (FHIR date precision does
  not map cleanly onto `chrono` types; keep the string as source of truth,
  offer `.parse_parts()`); `Decimal` precision audit (serde_json
  `arbitrary_precision` feature flag).

### Phase 3 — Validation depth (target: 0.4)

- **Cardinality + required bindings:** generator emits per-struct metadata so
  `Validate` can report missing 1..1 elements (currently unrepresentable —
  they're plain `T`, so instead validate required *bindings* and non-empty
  1..* Vecs) and value-set membership for required bindings.
- **Invariant subset:** implement the most common FHIRPath constraint
  patterns (exists/implies/xor on sibling fields) as generated checks; a full
  FHIRPath engine is out of scope until Phase 6+ (track as stretch).
- **`OperationOutcome` bridge:** `Vec<ValidationIssue>` →
  `OperationOutcome` conversion for server-ish use.

### Phase 4 — Ergonomics (target: 0.5)

- **Builders:** `Patient::builder().family("Chalmers").given("Peter")…` —
  generated, typed, with required-field enforcement where 1..1.
- **Prelude:** `fhir::prelude::*` exporting the ~30 most-used items.
- **Extension helpers:** `res.extension_by_url(url)`, `set_extension`,
  modifier-extension guards.
- **Bundle utilities:** typed entry iteration
  (`bundle.resources::<Observation>()`), transaction/batch builder,
  searchset paging (`next` link walk), `contained` resolution by local ref.
- **Contained resources typed:** change `contained: Option<Vec<serde_json::Value>>`
  to `Option<Vec<Resource>>` (depends on Resource enum having complete
  coverage — it does; verify against examples suite).

### Phase 5 — Interop (target: 0.6)

- **REST client** (feature `client`, reqwest): typed CRUD
  (`read/create/update/delete/search`), search-parameter builder generated
  from `search-parameters.json` (already bundled), auth hooks, paging.
- **XML** (feature `xml`): FHIR XML representation; likely a distinct
  serializer (quick-xml) driven by the same generator metadata. Large; gate
  behind milestone review.
- **Summary serialization** (`_summary=true`): per-field `isSummary` metadata
  from the spec → a `to_summary_value()` or serializer mode.

### Phase 6 — Multi-version & advanced (target: 0.7+)

- R4B under `src/r4b/` via the same generator pointed at R4B definitions;
  shared primitive layer; version feature flags to control compile time.
- Per-resource-domain feature flags if compile times demand it.
- FHIRPath engine (stretch), $validate against StructureDefinition
  snapshots (stretch), terminology service client (stretch).

### Documentation / tutorials / examples track (continuous, every phase)

- **mdBook guide** in `book/` (or extend `spec/`): chapters = Getting
  started; Data model mapping; Serialization deep-dive (incl. `_field`);
  Validation; Terminology & codes; Extensions; Bundles & servers; Using the
  generator. CI-built.
- **New examples** (one per phase feature as it lands):
  `extensions.rs`, `transaction_bundle.rs`, `search_response.rs`,
  `operation_outcome.rs`, `primitive_extensions.rs`, `typed_references.rs`,
  `client_crud.rs` (feature-gated).
- **Doctest per code enum family** and per builder.
- Keep `llms.txt`/`llms.json` (AI-readable crate summary) in sync — generate
  from rustdoc JSON if practical.

## Sequencing & dependencies

```
Phase 0 (CI + examples oracle)  ──► gates everything after it
Phase 1 (_field)                ──► needs Phase 0 oracle
Phase 2 (choice enums, coded enums, refs) ──► needs Phase 1 landed (both touch every struct; avoid double churn — do Phase 1 & 2 generator work in one modeling epic if capacity allows)
Phase 3 validation              ──► needs Phase 2 metadata plumbing in generator
Phase 4 ergonomics              ──► mostly independent; contained-typing needs oracle
Phase 5 client/XML              ──► independent of 2–4 except search builder
Phase 6 multi-version           ──► after generator hardening in 1–3
```

## Risks

| Risk | Mitigation |
|---|---|
| Choice-enum change breaks every downstream user | Batch in 0.3, migration notes in CHANGELOG, keep old field names as `#[deprecated]` accessors where feasible |
| `_field` serde pairing is fiddly | Prototype on 3 resources first; official-examples oracle decides |
| Generator emits subtly wrong output at scale | Never regenerate + overwrite blindly; diff-review per bundle; oracle suite |
| Mass agent edits clobber the tree | Read+Edit-only agents; commit baseline first (see memory note) |
| Compile time balloons (choice enums, builders) | Measure per phase; feature-flag domains if >2× baseline |
| crates.io name churn | Names settled: `fhir` (owned) + `fhir-derive-macros`; publish derive crate first |

## Verification standard (every task)

`cargo build && cargo test && cargo test --doc && cargo clippy --all-targets`
clean, `cargo doc --no-deps` zero warnings, and — once Phase 0 lands — the
official-examples round-trip suite green. Nontrivial features also get a
runnable example exercised end-to-end.
