# Tasks — `fhir` improvement plan

Executable task list for `plan.md`. Ordered; respect the `Depends` column.
Each task is sized for one focused Claude (Opus) session unless marked EPIC
(multi-session; has sub-tasks).

Conventions for the executing session:

- **Verify** (every task): `cargo build && cargo test && cargo test --doc &&
  cargo clippy --all-targets` clean; `cargo doc --no-deps` zero warnings.
  The round-trip gates are `tests/roundtrip_r{3,4,5}_examples.rs` and
  `tests/roundtrip_r2_spec.rs`, run in CI's `corpus` and `all-releases`
  jobs (the `roundtrip_official_examples` name this line used to cite never
  shipped; see T2/T28).
- **Branch + commit** a baseline before any mass edit; commit again after
  verification. Never leave large uncommitted work while agents run.
- **Mass edits across `fhir-release-N/src/{types,resources}`** must use
  Read+Edit-only agents (no Bash) or generator output — see memory note
  `parallel-file-edit-agents-no-bash`.
- Breaking changes land only in the phase's designated version bump.

---

## Phase 0 — Infrastructure & trust

### T1. GitHub Actions CI — *done*
- *Status:* `ci.yml` has grown to 16 jobs; the README badge is present.
- **Do:** `.github/workflows/ci.yml`: jobs for (a) `cargo build --all-targets`,
  (b) `cargo test` + `cargo test --doc`, (c) `cargo clippy --all-targets -- -D warnings`,
  (d) `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`,
  (e) `cargo publish --dry-run -p fhir-derive-macros` and `--dry-run` for `fhir`
  (allow the parent dry-run to be non-blocking until fhir-derive-macros is on
  crates.io — it fails registry resolution until then). Cache cargo. Add badge
  to README.
- **Accept:** workflow file lints (`act` optional); README badge present.
- **Depends:** —

### T2. Official-examples round-trip test suite  ⭐ highest value — *done, superseded by T28*
- *Status:* superseded by T28's full-corpus gate. A file named
  `tests/roundtrip_official_examples.rs` never existed; the real suites are
  `tests/roundtrip_r{3,4,5}_examples.rs` (plus `roundtrip_r2_spec.rs`).
- **Do:** Fetch FHIR R5 `examples-json.zip`
  (https://hl7.org/fhir/R5/examples-json.zip) into
  `doc/fhir-specifications/r5/fhir-examples-json/` (git-LFS or a
  `cargo xtask fetch-examples` script + .gitignore, decide by repo-size
  budget; ~100MB unzipped → prefer fetch-script + CI download).
  Add `tests/roundtrip_official_examples.rs`: for each `*.json`, parse to
  `fhir::r5::resources::Resource`, re-serialize, assert
  `serde_json::Value` equality. Provide an `#[ignore]`-by-default full run +
  a committed curated subset (~50 diverse files) always-on.
- **Accept:** curated subset passes; full-run failures are enumerated in
  `tasks-roundtrip-failures.md` (this becomes the Phase 1/2 burn-down list).
- **Depends:** —

### T3. Publish hygiene — *done*
- **Do:** Fix root `Cargo.toml` `include` (remove or create `llms.txt`,
  `llms.json`; keep `LICENSE.md`, README). Add `[package.metadata.docs.rs]`
  (all-features). Create `CHANGELOG.md` (0.1.0 entry summarizing current
  state) and `CONTRIBUTING.md` (build/test commands, generator workflow,
  doc conventions from AGENTS.md). Confirm `cargo publish --dry-run
  -p fhir-derive-macros` green.
- **Accept:** dry-run green; no missing-file include warnings.
- **Depends:** —

### T4. Property-based round-trip tests — *open*
- *Status:* no `proptest` anywhere in the tree — the one genuinely un-started
  Phase 0 task. Fuzzing (T31) covers adversarial *input*, not the
  populated-value round-trip property this asks for. See T48.
- **Do:** dev-dep `proptest`. Arbitrary-ish generators for 5 representative
  types (Patient, Observation, Bundle, CodeableConcept, Timing) producing
  populated values; assert serde round-trip equality.
- **Accept:** `cargo test proptest_` passes, ≥5 types covered.
- **Depends:** —

### T5. MSRV + toolchain policy — *done*
- *Status:* the CI `msrv` job pins 1.88.0 and all 13 crates declare
  `rust-version = "1.88"`. (The facade briefly said `1.97`, contradicting
  everything else — found by the 2026-08-06 audit, fixed under T37.)
- **Do:** Determine minimum supported Rust (edition 2024 ⇒ likely 1.85+);
  set `rust-version` in both Cargo.tomls; add CI job on that toolchain.
- **Accept:** CI green on MSRV.
- **Depends:** T1

---

## Phase 1 — Primitive extensions (`_field`)  → version 0.2

### T6. Prototype `_field` siblings on 3 resources  (decision task) — *done*
- *Status:* `tests/primitive_extensions_prototype.rs`; the decision is recorded
  in `spec/09-primitive-extensions.md`.
- **Do:** On Patient, Observation, Questionnaire: add
  `pub _birth_date: Option<types::Element>`-style siblings (serde rename
  `_birthDate`) for scalar primitives, and `Option<Vec<Option<Element>>>` for
  repeating primitives (FHIR aligns arrays with nulls). Also prototype the
  wrapper-type alternative (`FhirField<T>`) on a branch for comparison.
  Run curated round-trip subset incl. examples with `_field` data.
- **Accept:** written comparison in `spec/09-primitive-extensions.md` with a
  decision; chosen prototype passes round-trip on `_field` examples.
- **Depends:** T2

### T7. EPIC: roll `_field` across the model — *done*
- *Status:* shipped as generated `<name>_ext` fields serde-renamed to
  `_<name>`, emitted by `src/codegen/extension_ext_gen.rs`.
- **Do:** Apply the T6 decision to all ~208 structs. Preferred route:
  extend the generator to emit the siblings and use it to produce a
  mechanical edit list; else Read+Edit-only agent fan-out with an exact
  per-file spec. Update `#[derive(Validate)]` to recurse `_field` elements.
- **Accept:** full official-examples run: zero failures attributable to
  dropped `_field`; CHANGELOG 0.2 entry; version bump.
- **Depends:** T6
- **Sub-tasks:** T7a types (~50), T7b resources A–M, T7c resources N–Z,
  T7d validate-derive update, T7e docs+example `examples/primitive_extensions.rs`.

---

## Phase 2 — Type safety  → version 0.3

### T8. Generator: extract element metadata (bindings, choice lists, targetProfiles) — *done*
- *Status:* lives as `ElementMeta`/`BindingMeta` in `fhir-core/src/meta.rs`,
  with a generated table per release, rather than the `src/r5/meta.rs` named
  below.
- **Do:** In `src/r5/parse/`, surface per-element: binding strength +
  valueSet URL, `value[x]` type lists, reference targetProfiles, isSummary,
  cardinality. Emit as a queryable table (JSON in `tmp/out/` + a
  `src/r5/meta.rs` lookup) — foundation for T9–T12 and Phase 3.
- **Accept:** unit tests assert known facts (Patient.gender binding=required
  → administrative-gender; Observation.value[x] has 11 types;
  Observation.subject targets Patient|Group|Device|Location…).
- **Depends:** —

### T9. EPIC: choice types as enums — *done*
- *Status:* shipped via `#[derive(FhirChoice)]` (e.g. `ObservationValue`);
  spec 11.
- **Do:** Generate `ObservationValue { Quantity(..), String(..), … }`-style
  enums (custom Serialize/Deserialize keyed by `value<Type>` field names, incl.
  paired `_value<Type>`), replace flattened fields. Keep
  `#[deprecated]` accessor methods mirroring old field names where cheap.
- **Accept:** official-examples full run green; compile-time "exactly one"
  enforced; CHANGELOG migration notes; version bump 0.3.
- **Depends:** T7, T8
- **Sub-tasks:** T9a enum codegen + serde impls with unit tests;
  T9b apply to datatypes; T9c resources A–M; T9d resources N–Z;
  T9e docs/examples update (`examples/build_patient.rs` etc. compile).

### T10. Coded fields use `codes::` enums (required bindings only) — *done*
- *Status:* shipped as `Coded<E>` (spec 05), guarded by
  `tests/coded_bindings.rs`.
- **Do:** For elements with binding strength `required` whose CodeSystem enum
  exists in `codes.rs`: switch field type from `types::Code` to the enum.
  Add a fallback variant policy first (decision: add
  `#[serde(untagged)] Other(String)`-style variant vs closed enum → document
  in `spec/05-code-systems.md`; recommend fallback variant for wire
  compatibility).
- **Accept:** examples suite green; ≥100 fields migrated; docs updated.
- **Depends:** T8, T9 (avoid double-churn on the same structs)

### T11. Typed references — *partial*
- *Status:* the machinery shipped
  (`fhir-release-5/src/types/reference.rs`: `cast`/`into_any`/`resolve`) but
  zero generated fields use it — `types::Reference<` does not appear in any
  resource module. Generator emission of typed fields is recorded as Future
  work in specs 04, 08 and 13.
- **Do:** `Reference<T: ResourceType = Any>` newtype-with-phantom over the
  existing struct; generator picks `Reference<Patient>` where targetProfile
  is a single type, `Reference<Any>` otherwise. `Deref` to untyped;
  `.resolve(&bundle)` helper.
- **Accept:** examples suite green; doctest showing typed resolve.
- **Depends:** T8

### T12. Primitive value APIs — *partial*
- *Status:* precision parsing lives in `fhir-core::temporal`
  (`DateParts::parse` and kin); the newtypes themselves gained no inherent
  `parse_parts()` accessors. The `Decimal` half was superseded by T26.
- **Do:** Keep `String` storage; add `Date::parse_parts() -> (year, Option<month>, Option<day>)`,
  ordering per FHIR precision rules, `DateTime`/`Instant`/`Time` equivalents;
  `Decimal` behind `serde_json/arbitrary_precision` audit (feature `precise-decimal`).
- **Accept:** unit tests for precision edge cases (e.g. `"2024"`, `"2024-03"`);
  no representation change (round-trip untouched).
- **Depends:** —

---

## Phase 3 — Validation depth  → version 0.4

### T13. Cardinality + required-binding validation — *done*
- *Status:* `Coded<E>` covers required bindings; `1..*` is `vec1::Vec1`, so an
  empty list is unrepresentable rather than merely reported.
- **Do:** Using T8 metadata: `Validate` reports empty 1..* Vecs, and
  code-not-in-valueset for required bindings (via `codes.rs` enums / a
  generated membership set).
- **Accept:** unit tests: invalid Patient.gender code and empty
  `CodeSystem.concept` (1..*) each yield a pathed issue.
- **Depends:** T8, T10

### T14. Common-invariant subset — *done, exceeded*
- *Status:* 8 invariant classes enforced (the ask was ≥3); the committed
  coverage report is `spec/10-invariants-coverage.md`.
- **Do:** Generator recognizes ~5 recurring constraint shapes from
  ElementDefinition.constraint (e.g. "shall have value or children",
  xor-pairs) and emits checks; unrecognized constraints are listed (not
  silently dropped) in a generated report.
- **Accept:** ≥3 invariant classes enforced with tests; coverage report
  committed at `spec/10-invariants-coverage.md`.
- **Depends:** T13

### T15. `OperationOutcome` bridge — *done*
- *Status:* `impl From<Vec<ValidationIssue>> for OperationOutcome` per release
  (e.g. `fhir-release-5/src/validate.rs`).
- **Do:** `impl From<Vec<ValidationIssue>> for OperationOutcome` + severity
  mapping; example `examples/operation_outcome.rs`.
- **Accept:** doctest + example run.
- **Depends:** T13

---

## Phase 4 — Ergonomics  → version 0.5

### T16. Builders (generated) — *done*
- **Do:** `#[derive(Builder)]`-style generated builders (own proc-macro or
  generator-emitted `impl`), required 1..1 fields enforced at `build()`.
  Start: 10 most-used resources (Patient, Observation, Encounter, Condition,
  MedicationRequest, Practitioner, Organization, Bundle, DiagnosticReport,
  AllergyIntolerance) + all general-purpose datatypes.
- **Accept:** `examples/build_patient.rs` rewritten with builder; doctests.
- **Depends:** T9 (builder API shaped by choice enums)

### T17. Prelude + extension helpers — *done*
- **Do:** `fhir::prelude` (Resource, top resources, common types, Validate,
  codes commonly used); `ExtensionExt` trait: `extension(url)`,
  `extensions(url)`, `set_extension`, modifier-extension accessor.
- **Accept:** doctests; `examples/extensions.rs`.
- **Depends:** —

### T18. Bundle utilities + typed `contained` — *partial*
- *Status:* the `bundle_util` half is done (`iter_resources`,
  `resources::<T>()`, `next_link`, transaction/batch builders). Typed
  `contained` is **not**: it is still `Vec<serde_json::Value>` everywhere
  (e.g. `fhir-release-5/src/resources/patient.rs`), and no recorded decision
  reversed it — spec 04 R4.5 now *requires* the raw representation, which
  contradicts this task rather than closing it. See T47.
- **Do:** `Bundle::resources::<T>() -> impl Iterator<&T>`, transaction/batch
  builder, `next`-link paging helper; change `contained` fields to
  `Option<Vec<Resource>>` across resources (generator/agent fan-out) with
  local-reference resolution helper.
- **Accept:** examples suite green (contained is exercised heavily there);
  `examples/transaction_bundle.rs`.
- **Depends:** T2 (oracle), T9 landed (avoid churn)

---

## Phase 5 — Interop  → version 0.6

### T19. REST client (feature `client`) — *done*
- *Status:* shipped, then hardened under T29.
- **Do:** `fhir::client::Client` (reqwest, tokio): read/vread/create/update/
  delete/search + capability fetch; error → `OperationOutcome`; generated
  search-parameter builder from bundled `search-parameters.json` (typed
  params per resource). Integration tests against the public HAPI test
  server behind `#[ignore]`.
- **Accept:** `examples/client_crud.rs` (feature-gated) runs against HAPI;
  unit tests with a mock server (wiremock).
- **Depends:** T8 (search params), T15

### T20. Summary serialization — *done*
- **Do:** isSummary metadata (T8) → `to_summary_value(&self)` or a serializer
  wrapper emitting only summary elements + mandatory ones.
- **Accept:** Patient summary matches spec's `_summary` semantics on
  examples; doctest.
- **Depends:** T8

### T21. EPIC: XML support (feature `xml`) — *done*
- *Status:* the `xml` feature shipped (with the `xml_depth` test and fuzz
  targets from T31); the "gate behind milestone review" caveat is spent.
- **Do:** quick-xml-based `to_xml`/`from_xml` driven by generator metadata;
  validate against official XML examples (`examples.zip` XML variant).
- **Accept:** curated XML round-trip subset green.
- **Depends:** T8; review after Phase 4 whether demand justifies it

---

## Phase 6 — Multi-version (0.7+)

### T22. EPIC: R4B model — *obsolete, superseded*
- *Status:* multi-release support shipped as R2–R6 in separate
  `fhir-release-N` crates instead of `src/r4b/`. R4B itself remains future
  work (spec 12).
- **Do:** Point generator at R4B definitions → `src/r4b/`; feature flags
  `r5` (default) / `r4b`; shared primitives where identical.
- **Accept:** R4B examples round-trip subset green; compile-time measured
  and documented.
- **Depends:** generator hardening from T7–T9

---

## Documentation / tutorials / examples track (interleave; one per phase)

### T23. mdBook guide — *done, one half open*
- *Status:* `book/` is written and built in CI's `book` job; the GitHub Pages
  deploy half never happened. See T46.
- **Do:** `book/` with chapters: Getting started; Model mapping; JSON
  serialization deep-dive (incl. `_field` once T7 lands); Validation;
  Terminology & codes; Extensions; Bundles; Code generator internals. CI job
  builds it (`mdbook build`), deploy to GitHub Pages.
- **Accept:** `mdbook build` green in CI; linked from README.
- **Depends:** T1 (CI); content updated at the end of each phase

### T24. Example set expansion — *mostly done*
- *Status:* 13 examples are wired; `search_response.rs` and
  `typed_references.rs` (blocked on T11) were never written. See T43.
- **Do (rolling):** `extensions.rs` (T17), `primitive_extensions.rs` (T7),
  `transaction_bundle.rs` + `search_response.rs` (T18),
  `operation_outcome.rs` (T15), `typed_references.rs` (T11),
  `client_crud.rs` (T19). Every example: header comment tutorial style
  (match the existing four), runs cleanly, and is listed in README +
  `lib.rs` "More examples".
- **Accept:** `cargo build --examples` green; each prints sensible output.

### T25. `llms.txt` / `llms.json` — *done; rot repaired under T38*
- *Status:* `bin/check-llms` checks the real module roots and passes (27
  modules); `llms.json` says 3.0.0, five releases, 442 R5 enums, and
  mentions `r2`/`r6`/`convert`. Still open: `llms.txt` is byte-identical to
  `fhir.md` (22 MB duplicated in git) — see T38's remaining decision.
- **Do:** Author AI-readable crate summaries (crate purpose, module map, key
  types, examples index); restore them to `Cargo.toml` `include`; add a CI
  check that they mention every top-level module.
- **Accept:** files exist, included in package, dry-run green.
- **Depends:** T3

---

## Phase A — Assurance (spec 13)

What has to be true before the crate is depended on for clinical work. P0
items are defects in shipped behaviour; P1 items are missing guarantees.

### T26. Lexical `Decimal` (R2.2, R2.2a) — **P0** — *done*
- **Why:** in a default build `0.50` serializes as `0.5`, `1.000` as `1.0`,
  and `12345678901234567890.5` as `1.2345678901234567e+19`. FHIR treats
  decimal precision as clinically significant, so this silently changes what
  a lab result or a dose says. The `precise-decimal` escape hatch is a global
  `serde_json` feature, so whether a build is correct depends on unrelated
  crates in the graph.
- **Done:** `serde_json/arbitrary_precision` is now a non-optional dependency
  feature, so precision is a guarantee a dependent cannot switch off, and
  `Decimal` is a hand-written type at the crate root (`fhir::decimal`) shared
  by all three releases: lexical `Eq`, numeric `PartialOrd`, `new`/`as_str`/
  `as_f64`/`as_number`, FHIR-production validation. `precise-decimal` is a
  deprecated no-op. *Breaking* (private field) → **fhir 2.0**.
- **The `RawValue` approach was tried and rejected**, which is the finding
  worth keeping: it preserves precision on `from_str` but fails through
  `#[serde(flatten)]` — serde's `Content` buffering cannot hold a raw token,
  so the whole `value[x]` choice variant is *silently dropped*. The choice
  unit tests caught it; a corpus test would have too, but only because the
  loss was total. Recorded in spec 02 R2.2.
- **Accept met:** spec 02 acceptance 2a and 2b (`decimal::tests`), plus the
  full lib suite (717 tests) and the R3/R4 models.

### T27. Round-trip oracle can see precision (R13.3) — **P0** — *done*
- **Why:** the round-trip suite compares `serde_json::Value`s, where
  `0.50 == 0.5`, so the test that should have caught T26 is blind to it.
- **Done:** T26 repaired this as a side effect — with `arbitrary_precision`
  guaranteed, `serde_json::Value` numbers compare by lexeme, so the existing
  `Value`-based oracle can see a decimal regression. `src/decimal.rs`
  `oracle_tests` pins that property directly, so if the representation ever
  changes again the failure lands there instead of the corpus suite quietly
  going blind.
- **Note:** rewriting the oracle to compare canonicalized text is therefore
  no longer needed for decimals. It would still be needed if the crate ever
  had to guarantee byte-identical key ordering, which it does not.

### T28. Full-corpus gate in CI (R13.1, R13.2) — **P0** — *done*
- **Why:** `roundtrip_full_official_examples` is `#[ignore]` and its data is
  uncommitted, so CI only runs a curated subset chosen to pass. fhirpg's
  corpus gate is currently stricter than this crate's own.
- **Do:** fetch (or vendor) the official examples for R3/R4/R5 in CI via
  `bin/fetch-examples`; run all three suites un-ignored; convert
  `tasks-roundtrip-failures.md`'s remaining entries into a named allowlist
  with a reason each and an asserted length.
- **Done, and it found things immediately.** All three corpora now run as a
  gate (7,400 examples): R5 2823/2824, R4 2713/2912, R3 1490/1664.
  - **A live data-loss bug:** R5 dropped
    `ConceptMap...target.product.attribute`/`value[x]` — see T35.
  - **A stale exemption:** the gate refused an allowlist entry that had
    started passing, which is the property that keeps the list shrinking.
  - **372 R4/R3 failures reduced to three causes**, all of them HL7 examples
    that violate their own specification (omitted `linkId`, `base`, `code`,
    each `min=1`). Recorded as counted `KnownFailureClass` entries rather
    than 372 lines — and explicitly *not* fixed by relaxing the model, which
    would have turned a correct model into a green test.
- **Accept met:** CI fails on any new mismatch, on a stale allowlist entry,
  and on any change to a class count in either direction.

### T29. Client hardening (R13.5–R13.10) — **P0** — *done*
- **Do:** default connect/request timeouts and a response size cap;
  percent-encode path and query components (today `id` is interpolated raw,
  so `../Patient/other` retargets the request); `If-Match` on update/delete
  and `ETag` on reads; conditional create; bounded retry with backoff for
  idempotent methods only; `next`-link paging iterator; a bearer-token hook;
  truncate and mark the raw body in `ClientError::Status`.
- **Accept met:** `a_stalled_server_times_out`,
  `a_hostile_id_cannot_retarget_the_request`, `an_oversized_body_is_refused`,
  and `debug_output_does_not_leak_the_body`.
- **Note:** `Release` gained `next_link` so paging is written once rather
  than per release. R5 types `Bundle.link.relation` as a bound code where
  R3/R4 use a string, so the three impls genuinely differ.

### T30. Publish 1.2.1 (R13.16) — **P0, small** — *done*
- **Why:** published 1.2.0 fails to compile with `r3`/`r4` for downstream
  users, which is why fhirpg's `--validate` is R5-only. The tree is fine
  (`cargo check --no-default-features --features r4` is clean), so this is a
  release, not a fix.
- **Do:** cut 1.2.1 from the current tree; add a CI job that builds every
  documented feature combination from a packaged `.crate`, not the worktree.
- **Accept:** fhirpg can enable `--validate` for R3 and R4.
- **Done:** the workspace is published, and CI's `features` and
  `publish-dry-run` jobs build the documented feature combinations.

### T31. Fuzzing and depth bounds (R13.4) — **P1** — *done*
- **Do:** `cargo-fuzz` targets for `Resource` deserialization per release;
  assert bounded recursion for `Questionnaire.item.item` and nested
  primitive extensions; run a short fuzz budget in CI.
- **Done:** ten targets (`parse_xml_*`, `roundtrip_json_*`, one pair per
  release). The `fuzz` job runs the R5 pair on every push with a 120-second
  budget; `fuzz-releases` covers all five nightly, because each target needs
  its own build — compiling the models together is what forced the crate
  split. It found a remote DoS in the XML reader, fixed in fhir-core 2.2.0
  with `xml::MAX_DEPTH` matching the bound `serde_json` already applied.

### T32. Supply-chain evidence (R13.13–R13.15) — **P1** — *done*
- **Done:** a `supply-chain` CI job (cargo-deny + CycloneDX SBOM uploaded as
  an artifact) with a `deny.toml` policy — permissive licences only,
  wildcards and unknown registries denied; `#![forbid(unsafe_code)]`;
  `llms.txt` dropped from `include`; a `package-size` CI job asserting the
  published `.crate` stays under 10 MB.
- **Also fixed, found while doing it:** the generator emitted code `cargo
  fmt` disagreed with — a trailing space after `Version:` when the spec
  states none, and over-width lines rustfmt rewrapped and the next
  generation unwrapped. `cargo fmt --check` was therefore a gate that could
  only stay green until someone regenerated. The generator now runs its
  output through `rustfmt`; regenerating R3 and R4 produces zero
  disagreements.
- **Remaining:** the committed `fhir-specifications-parser.profdata` at the
  repository root is still there — it is not in `include`, so it does not
  ship, but it does not belong in git either. Tracked as T42.

### T33. Say what validation is (R13.11) — **P1, small** — *done*
- **Done:** README's Validation section now states what is not checked —
  FHIRPath invariants (311 of 314 keys), profiles, terminology, reference
  resolution — and that a resource this crate calls valid may still be
  rejected by a conformant server. The remaining half landed too:
  `src/lib.rs`'s crate guide now carries the same caveat.

### T35. Audit `src/r5` for drift from the generator — **P0** — *done*
- **Why, concretely:** `ConceptMap.group.element.target.product` was typed
  `Vec<types::Element>` in R5 and silently dropped `attribute` and
  `value[x]` on every deserialize. R3 and R4 — which are *generated* — had it
  right: `Vec<...DependsOn>`, per the element's `contentReference`. R5 is
  hand-documented and never regenerated (`cargo run -- r5` refuses without
  `--out`), so it silently predates a generator fix. The default-enabled,
  flagship release was the stale one, and only the full corpus caught it.
- **The concern is the class, not the case.** Comparing generated R5 against
  committed R5 (normalizing the `crate::r5::coded` vs `crate::coded` path
  spellings) leaves ~77 files differing. Most are benign — hand-added prose
  comments the generator does not emit, and deliberate hand-written
  improvements it cannot produce, notably the typed `Reference<T>` and its
  `PhantomData`. But some look semantic and need a verdict each:
  - `Ratio.denominator`: `Option<Quantity>` generated vs `Quantity`
    committed — a cardinality disagreement, and only one can be right.
  - `SimpleQuantity`: the generator emits `code_ext`/`system_ext`/`unit_ext`
    primitive-extension siblings that the committed tree lacks.
  - `ProductShelfLife`: the generator emits `modifier_extension`; committed
    does not.
  - The generator reports 442 R5 code enums where the README says 419,
    suggesting the committed tree came from a different spec or generator
    vintage entirely.
- **Do:** classify every one of the ~77 as (a) generator is right → fix R5,
  (b) hand-written on purpose → teach the generator, or (c) cosmetic → ignore.
  Then add a CI check that compares `cargo run -- r5 --out` against `src/r5`
  modulo the sanctioned hand-written differences, so this cannot silently
  reopen.
- **Do not** simply regenerate over `src/r5`: it carries hand-written prose
  and the typed-`Reference` work, which is exactly why it drifted and exactly
  why a blind regeneration would be a regression.
- **Measuring this took three attempts, which is worth recording so the next
  person does not repeat it.** Comparing field declarations line-by-line gave
  77 "differences"; keying fields by name per *file* gave 99. Both were
  mostly artifacts — equivalent module paths (`crate::r5::coded` vs
  `crate::coded`, `::vec1::` vs `vec1::`), multi-line declarations split by a
  line-based grep, and — the real trap — files containing several structs
  that share a field name, so `ClaimResponseItemDetail.detail` was being
  compared against `ClaimResponseAddItemDetail.detail`. Only a struct-aware
  comparison gives usable numbers: **18 type disagreements, 27 fields present
  only in the generator, 5 only in the committed tree.**

- **Fixed so far, each spec-verified and pinned by a regression test in the
  relevant module's `drift_tests`:**
  - `ConceptMap...target.product` — was `Vec<types::Element>`, dropping
    `attribute` and `value[x]`. (Found by the corpus gate.)
  - `Ratio.denominator` — was required; the spec says `0..1`, so a valid
    Ratio carrying only a numerator was *rejected outright*. Three existing
    tests had to change: they asserted the defaulted denominator, which is to
    say they encoded the bug.
  - `Address.use`, `HumanName.use`, `ContactPoint.use` — no `_use` sibling,
    so primitive extensions on them were dropped (spec 09).
  - `ProductShelfLife.modifierExtension` — absent entirely. The worst of the
    set: a modifier extension changes what the element *means*, so dropping
    one yields a resource that reads as understood and is not.

- **Why the corpus could not find these.** No published example puts a
  primitive extension on `Address.use`, or a modifier extension on
  `ProductShelfLife`. The corpus proves the model handles the data that
  exists; it says nothing about data the specification permits. Hence the
  targeted tests.

- **Closed by enumeration, not by discovery.** Two defects came from one
  construct — an unresolved `contentReference` (`product`,
  `characteristic`). Rather than wait for a third to surface, all **78** R5
  `contentReference` elements were checked against the committed model: every
  one now resolves to its backbone struct. `tests/content_reference.rs`
  keeps it that way, and it is the right shape of test for this defect
  class — the corpus found `product` and could *never* have found
  `characteristic`, since raw JSON round-trips perfectly.

- **Cleared, not a defect:** `ImagingSelectionInstance.image_region_2_d/3_d`
  appeared missing from the committed tree. They are present under a
  different Rust identifier (`image_region2_d`) that derives the same
  `imageRegion2D` wire name; `imagingselection-example-2d-image-region-
  selection.json` exercises it and the gate passes. The third false positive
  this audit produced from identifier-shaped noise.

- **Still to triage:**
  - **The `Coded<E>` regressions have one root cause, and it is not
    carelessness.** 13 fields across 11 files use a plain `types::Code` where
    the generator emits the bound enum. All are `required` bindings, which
    spec 05 says must be `Coded<E>`. Attempting the conversion fails to
    compile: `FhirVersion`, `FhirTypes`, `DetectedissueStatus`,
    `Imagingselection2Dgraphictype` and others **do not exist in the
    committed `codes.rs`**, which has 419 enums where the generator now
    produces 442. Whoever wrote those fields could not have used `Coded<E>`;
    the enum was not there.

    **Done:** `src/r5/codes.rs` regenerated (419 -> 442 enums) and **10 of
    the 13** fields converted. Three were rejected — see below — and the
    regeneration carried two breaking changes beyond the additions:
    - **12 enums renamed**, not added: `FHIRVersion` -> `FhirVersion`,
      `Imagingselection2dgraphictype` -> `Imagingselection2Dgraphictype`,
      `Icd10Procedures` -> `ExIcd10Procedures`, `CdshooksIndicator` ->
      `Indicator`, `NhinPurposeofuse` -> `Purposeofuse`, and others. None is
      referenced inside the model — checked before overwriting — but all are
      public exports, so downstream code naming them breaks.
    - **`HttpVerb` variants renamed** (`POST` -> `Post`, …), which broke
      `bundle_util.rs` and is how it was noticed. Enums *not* referenced
      internally can change shape with nothing to catch it, so the rename
      list above is the changelog's job, not the compiler's.

  - **Three conversions were rejected, and this is the important part.**
    `TaskIntent`, `TransportIntent` and `DetectedissueStatus` are generated
    with a **single `Unknown` variant**: the generator cannot resolve those
    value sets from the bundled packages. `Coded<E>` over such an enum is
    *worse* than a plain `Code` — every real value (`order`, `plan`,
    `proposal`) becomes `Unknown("order")` while the signature advertises
    type safety. They stay `types::Code`.

    This inverts the original reading. These three were filed as "the
    hand-maintained tree fell behind"; in fact `types::Code` was almost
    certainly a deliberate, correct call by whoever met the same empty enum.
    The audit was one step from turning a considered decision into a defect.

    Nothing in the corpus could have caught it: `Coded::Unknown` preserves the
    string, so a wrongly-bound field round-trips perfectly.
    `tests/coded_bindings.rs` asserts a real code from each bound value set
    deserializes to `Known(_)`, and it rejected the three on first run.

### T36. The generator emits enums it cannot populate — **P1** — *done*
- **Do:** `codes_gen` produces an enum for every `required` binding even when
  the value set cannot be expanded from the bundled packages, yielding a type
  with one `Unknown` variant (`TaskIntent`, `TransportIntent`,
  `DetectedissueStatus`, and possibly more). Such a type promises type safety
  and delivers none, and any field bound to it is better left `types::Code`.
- Either resolve those value sets (they compose codes from another
  CodeSystem, which the expander does not follow), or emit no enum at all and
  let the field stay a `Code` — but not a type that lies.
- **Root cause, and it is narrower than "the expander is broken".** Enums are
  built from **CodeSystem** resources, and a binding is resolved by taking the
  last segment of its ValueSet URL and looking for an enum of that name. That
  is correct whenever a ValueSet and its like-named CodeSystem hold the same
  codes. It fails when a ValueSet *composes* several systems, because the
  composition is never read:
  - `task-intent` = `task-intent` CodeSystem (1 concept: `unknown`) **+**
    `request-intent` (8 concepts)
  - `transport-intent` = same shape
  - `detectedissue-status` = `observation-status` (3) **+**
    `detectedissue-status` (1)

- **Done (the safe half):** `codegen.rs` no longer offers a one-variant enum
  for binding, so such a field stays `types::Code`. Honest, rather than a
  `Coded<E>` that turns every real value into `Unknown` while the signature
  claims a checked value set.

- **This was already shipping.** R4's `Task.intent` — `1..1`, required-bound,
  the field that distinguishes a proposal from an order — was
  `Coded<TaskIntent>` at HEAD, so every real intent deserialized to
  `Coded::Unknown("order")`. Regenerating R4 now yields `types::Code`: one
  line changed, a live defect removed. Nothing could have caught it from
  data, because `Coded::Unknown` round-trips perfectly; it took
  `tests/coded_bindings.rs`, which asserts a genuine code resolves to
  `Known(_)`.

- **Done (the real half):** `spec::read_value_sets` plus `compose_codes`
  resolve `ValueSet.compose` — explicit concept lists taken as given, bare
  `system` includes expanded from that CodeSystem, `exclude` subtracted — and
  the union supersedes the like-named system wherever it covers more.

  | enum | before | after | bound to |
  | --- | ---: | ---: | --- |
  | `TaskIntent` | 1 | 9 | `Task.intent` (required) |
  | `TransportIntent` | 1 | 9 | `Transport.intent` (required) |
  | `DetectedissueStatus` | 1 | 4 | `DetectedIssue.status` (required) |
  | `EventTiming` | 13 | **27** | `Timing.repeat.when` (required) |
  | `TimingAbbreviation` | 1 | 17 | — (`Timing.code` is a *preferred* binding, so no field changed) |

  **`EventTiming` is the one that mattered and nobody was looking at it.** It
  was *partially* populated, which is worse than empty: 14 of 27 codes
  resolved to `Unknown` while their siblings resolved to `Known`, on a
  required binding. The 14 are `AC` (before meals), `PC` (after meals), `HS`
  (at bedtime), `WAKE`, and the meal-specific variants — the vocabulary for
  *when a dose is taken*. A prescription saying "before meals" kept that fact
  on the wire and lost it from the type.

  Found only because the fix addressed the cause. Three degenerate enums were
  what a test happened to catch; the resolver runs over all 442 and found
  three more.

- **The 18 remaining one-variant enums are correct, not a gap.** This was
  initially recorded as unfinished work; checking it showed otherwise. Each
  of `AssetAvailability`, `ConceptmapProperties`, the fourteen `Contract*`
  enums, `MedicinalProductAdditionalMonitoring` and
  `MedicinalProductSpecialMeasures` is generated from a CodeSystem that
  **the specification defines with exactly one concept** — FHIR ships them as
  placeholder terminologies for extension. A one-variant enum is the faithful
  model.

  Nor does the guard cost anything there: all **13** bindings to those value
  sets are `example` strength, and an example binding never becomes
  `Coded<E>` regardless. Zero required bindings are affected.

  So the guard is now a net with nothing in it — which is the right state for
  it. It exists to stop a *future* resolution gap from silently producing a
  lying type, not to paper over a present one.
  `tests/coded_bindings.rs::no_required_binding_resolves_to_a_degenerate_enum`
  turns that from a claim into a check.

- **Still not resolved:** an `include` naming another `valueSet` returns
  `None` and falls back rather than answering partially. No enum in R5
  currently needs that path — it is a robustness measure for future spec
  revisions, not an outstanding defect.

- **Accept met:** no field is bound to an enum with fewer than two variants;
  every required binding whose value set the resolver can reach now resolves
  to an enum that can represent it. `tests/coded_bindings.rs` proves a real
  code from each lands in `Known(_)`, including the recovered
  meal-relative `EventTiming` codes.
  - **Done.** Two `Option<Vec<T>>` fields
    (`MedicationKnowledge...patientCharacteristic`,
    `AdministrableProductDefinition...withdrawalPeriod`) now follow the
    crate's own `0..* -> Vec<T>` convention, which had reached R3/R4 by
    regeneration and only partly R5.
  - **Done.** 13 missing `_ext` primitive-extension siblings added
    (`MoneyQuantity` x5, `SimpleQuantity` x4, `Device.udiCarrier` x2,
    `DeviceDefinition`, `TestScript...assert`). Each silently dropped a
    `_field` object — the same class as the `_use` bug.

- **Accept: met.** `tests/r5_drift.rs` generates R5 to a temporary directory
  and compares field types per struct; every remaining difference is listed
  in a `SANCTIONED` table with a reason. The count went 18 -> 5 -> 0.
  From here a generator fix that R5 does not receive fails a test, rather
  than waiting for a corpus run or a bug report.

  The seven sanctioned differences: the typed `Reference<T>` (T11), three
  fields whose bound enum is degenerate (T36), the `ImagingSelection`
  identifier spelling (same wire name, corpus-verified), and
  `Identifier::assigner`'s box, which the generator needs to break the
  `Reference`/`Identifier` cycle and the typed `Reference<T>` does not.

  *Note on method, since it cost four attempts:* comparing field declarations
  is harder than it looks. Line-based diffing, per-file field keying, and a
  naive "split the type at `,\n`" all produced long lists of phantom
  differences — the last because stripping a trailing comment leaves a space
  before the newline, so the split silently swallowed the *next* field's
  attribute into the type string. The working version keys by
  `(struct, field)` and scans to the terminating comma at generic depth zero.

### T34. Cross-release conversion (spec 13 Future work) — **P2** — *done*
- **Why the old advice was not enough:** "round-trip through JSON and see what
  serde refuses" reports the *first* error and stops, and the commonest
  difference between releases — an element the target does not have — is not an
  error at all. Serde ignores unknown keys, so the field vanishes unreported.
  The mechanism was quietest exactly where the data loss was.
- **Done:** `fhir::convert::between::<S, T>(&json)` (engine in
  `fhir-core::convert`, spec 14). Driven by both releases' `ElementMeta`
  tables, so every pair converts both ways and a new release is convertible by
  existing. Returns a `LossReport` of elements removed, choice variants the
  target does not admit, cardinality narrowed, JSON kind changed, required
  elements absent, and bindings moved — separating losses that dropped data
  from warnings about data kept. No `From`/`Into`: R12.4 is intact.
- **It is structural, deliberately.** A *renamed* element is reported as
  removed rather than guessed at; a wrong mapping is worse than an absent one
  because it is invisible in the result. HL7's cross-version extension maps
  would supply the real remappings — recorded as spec 14 Future work.
- **Three bugs the tests found, in order of nastiness:**
  1. A document with no `resourceType` returned null with an *empty* report —
     a silent failure in the module whose whole purpose is to have none. Now
     `LossKind::NotAResource`. Found by the example, not by a unit test.
  2. Recursive backbones were unresolvable: `ElementMeta` had no
     `contentReference`, and it cannot be inferred from the path (see the
     CHANGELOG for the three ways guessing fails). Added to the table and the
     generator; all five releases regenerated.
  3. Cardinality was narrowed but never widened, so an element singular in the
     source and repeating in the target produced "invalid type: map, expected
     a sequence" from the target model.
- **Accept met:** spec 14's seven criteria (strict mode was added after this
  entry was first written). The load-bearing ones are that a
  release converted to *itself* returns an equal document and an empty report
  (the walk's own oracle), and that every corpus document the target rejects
  was predicted by the report — with a guard asserting that check is not
  vacuous.
- **Also fixed on the way:** `fhir-release-5`'s generated element table had
  drifted from the generator in 63 `target_profiles` entries.

---

## Phase B — Audit 2026-08-06 (drift between docs and tree)

The tree moved faster than its documents: the workspace is thirteen crates at
3.0.0 modelling five releases, and text all over the repository still
describes three releases in one crate at 1.x. P0 items break a CI gate or
contradict the build; P1 items are missing guarantees; P2 items are stale
text and leftovers. Each fact below was verified against the tree on the date
above.

### T37. Reconcile the MSRV declaration — **P0** — *done*
- **Why:** the facade's `Cargo.toml` said `rust-version = "1.97"` while the
  12 member crates say `1.88`, the CI `msrv` job pins 1.88.0, and AGENTS.md
  and CONTRIBUTING.md both document 1.88. The msrv job builds on 1.88 and
  would refuse the facade.
- **Done (2026-08-06):** facade reverted to `1.88` — nothing anywhere
  motivated 1.97; every other statement of the MSRV already agreed.
  *Remaining nicety:* a check that every workspace crate declares the same
  `rust-version` (one grep in CI), so a stray edit like this one cannot
  land silently again.

### T38. Repair `bin/check-llms` and regenerate the llms artifacts — **P0** — *done except the llms.txt decision*
- **Why:** `bin/check-llms` collected module names from `src/r3.rs`,
  `src/r4.rs` and `src/r5.rs`, which no longer exist, and exited 1 — so the
  `llms` CI job could not pass. `llms.json` was stale: version 1.1.0, "419"
  code enums, no mention of `r2`, `r6` or `convert`.
- **Done (2026-08-06):** the check now collects from the real roots —
  `src/lib.rs` (`pub mod` + the `pub use ::fhir_release_N as rN` aliases)
  and `fhir-release-5/src/lib.rs` (every release exposes the same module
  shape, R12.2) — and refuses to pass vacuously if extraction finds fewer
  than 15 modules (R13.20). `llms.json` updated: version 3.0.0, five
  releases with per-release counts (incl. r2 94/265 and r6 161/459), R5
  enums corrected to 442, `r2`/`r6`/`convert` module entries added.
  `./bin/check-llms` → `OK: llms.txt and llms.json mention all 27 modules`,
  exit 0.
- **Remaining (owner decision):** whether `llms.txt` — byte-identical to
  `fhir.md`, 22 MB duplicated in git — should exist as a separate file at
  all.

### T39. `#![forbid(unsafe_code)]` in all 13 crates — **P1** — *done*
- *Status (2026-08-06):* the attribute is in all 13 crate roots; a
  full-feature `cargo check --workspace --features "r2 r3 r4 r6 xml client"`
  compiles clean (the tree has no `unsafe` outside string literals). Spec
  13's R13.14 status updated to match.
- **Why:** R13.14 was met by 1 of 13 crates — only the facade
  (`src/lib.rs`) declares it. `fhir-core`, which carries the REST client and
  the XML reader — the network and parsing surface — does not; neither do
  `fhir-derive-macros` or any release crate.
- **Do:** add the attribute to every crate root (each release crate's
  `lib.rs` is hand-maintained, so this is twelve one-line edits), and note it
  in spec 13's R13.14 status when done.

### T40. Sweep AGENTS.md + AGENTS/ for release-count and path drift — **P1** — *done*
- *Status (2026-08-06):* AGENTS.md now says five releases (R2 row added),
  R6 published-by-necessity (R12.14a), reservations -1/-7..-10 at 0.0.1,
  tmp/out/ untracked, and "the other four are generated"; architecture,
  conventions, glossary and code-generation updated to `fhir-release-N/src`.
- **Why (all verified):** AGENTS.md says "Four releases are modelled" and its
  table omits R2; it calls R6 "unpublished" and annotates it `publish =
  false` (no crate sets `publish`, and R6 *must* be published — R12.14a); its
  reservation list omits `fhir-release-1` and `fhir-release-10`; it claims
  `tmp/out/` is tracked (it is not). `AGENTS/code-generation.md`,
  `conventions.md` and `glossary.md` still use `src/<release>/` output paths.
- **Do:** one consistency pass over AGENTS.md and AGENTS/, against the same
  ground truth as this audit (five releases, `fhir-release-N/src` paths, all
  five release crates published).

### T41. CHANGELOG and identity-string drift — **P2** — *done*
- *Status (2026-08-06):* retroactive `fhir-derive-macros 1.2.0` entry added;
  the 3.0.0 entry's "untouched at 1.1.0"/"0.0.0" corrected (CLAUDE.md too);
  README says stable (3.0), five models; lib.rs says five releases and
  `fhir = "3"`; the package description now names R2-R6 and `convert`.
- **Why (all verified):** CHANGELOG has no `fhir-derive-macros` 1.2.0 entry,
  and its 3.0.0 entry says derive-macros is "untouched at 1.1.0" (it is
  1.2.0) and "the reservation crates stay at 0.0.0" (they are 0.0.1).
  README says "stable (1.0). All three data models"; `src/lib.rs` opens with
  "Three releases are modelled" and its install snippets say `fhir = "1"`;
  the facade's `Cargo.toml` description names only R5, R4 and R3.
- **Do:** add the missing CHANGELOG entry, correct the 3.0.0 entry, and bring
  README, `src/lib.rs` and the package description up to five releases at
  3.0.0.

### T42. Remove the committed profdata — **P2** — *done*
- *Status (2026-08-06):* untracked (`git rm --cached`), deleted from the
  working tree, and added to `.gitignore`.
- **Why:** T32's remaining item: `fhir-specifications-parser.profdata` is
  tracked at the repository root. It does not ship, but it does not belong in
  git.
- **Do:** `git rm --cached fhir-specifications-parser.profdata` and add it
  (or `*.profdata`) to `.gitignore`.

### T43. The never-written examples, plus one for `convert` — **P2**
- **Why:** T24 left `search_response.rs` unwritten, and `typed_references.rs`
  is blocked on T11. Meanwhile 3.0.0's headline feature — cross-release
  conversion (spec 14) — has no standalone example at all.
- **Do:** write `search_response.rs` and a dedicated `convert` example now;
  `typed_references.rs` when T11 completes.

### T44. `main.rs` usage text is stale — **P2** — *done*
- *Status (2026-08-06):* USAGE now lists r2-r6 (and the alias/version
  spellings) and the real `fhir-release-N/src` default; `cargo check` clean.
- **Why:** the USAGE string says the release argument is "r3, r4, or r5" and
  that `--out` defaults to `src/<release>`. The parser accepts `r2` through
  `r6`, and the default output is `fhir-release-N/src`.
- **Do:** make the USAGE string match `codegen::Version::parse` and
  `source_dir()`.

### T45. Relabel `tasks-roundtrip-failures.md` as historical — **P2** — *done*
- *Status (2026-08-06):* header rewritten — historical burn-down record,
  pointing at the in-test allowlists and the real test targets.
- **Why:** its regenerate command names the dead
  `roundtrip_official_examples` test target, and the operative allowlists now
  live in the roundtrip test files themselves (T28). The document is the
  burn-down record, not the current state.
- **Do:** add a header saying so, pointing at the allowlists in
  `tests/roundtrip_r{3,4,5}_examples.rs`.

### T46. mdBook GitHub Pages deploy (T23's unfinished half)
- **Do:** either deploy `book/` to GitHub Pages from the existing CI build,
  or record the decision not to and close T23.

### T47. Typed `contained` (T18's unfinished half)
- **Why:** T18 asked for `Option<Vec<Resource>>`; the tree kept
  `Vec<serde_json::Value>`, and spec 04 R4.5 now mandates the raw
  representation. The task and the spec contradict each other and no decision
  is recorded.
- **Do:** decide — most likely record R4.5 as the deliberate reversal and
  close T18's second half — and make tasks.md and spec 04 agree.

### T48. Property-based tests (T4)
- **Do:** either add the proptest round-trip suite T4 describes, or record
  the decision that the corpus gate (T28) plus fuzzing (T31) cover the
  property and close T4.

---

## Suggested execution order (first five sessions) — *historical*

This block described the plan before any of it ran. It was executed; the
workspace is now published at 3.0.0. Kept for the record:

1. **T2** (examples oracle) + **T1** (CI) — everything else gets safer.
2. **T3 + T5 + T25** (publish hygiene bundle) — then actually publish 0.1:
   `cargo publish -p fhir-derive-macros && cargo publish`.
3. **T8** (generator metadata) — unlocks most of Phases 2–5.
4. **T6** (prototype `_field`, make the representation decision).
5. **T7** (EPIC `_field` rollout) → ship 0.2.
