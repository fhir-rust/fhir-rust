# 13 — Assurance for mission-critical use

Specs 02–12 define *what the model is*. This one defines what has to be true
before a hospital, a national exchange, or a regulated device can depend on it:
how fidelity is proven rather than asserted, how the REST client behaves when
the network misbehaves, and what evidence ships with a release.

The crate's claim is narrow and should stay narrow — it is a data model and a
client, not a server and not a validator of clinical safety. The point of this
spec is that the narrow claim be *airtight*, and that everything outside it be
stated rather than implied.

## Requirements

### Fidelity is proven, not sampled

- **R13.1** The full official example corpus for every modelled release MUST
  round-trip in CI, not only a curated subset. Today
  `roundtrip_full_official_examples` is `#[ignore]` and its data is not
  committed (`tests/roundtrip_r5_examples.rs`), so CI exercises only examples
  "chosen to pass today" — which cannot detect a regression in the examples it
  does not run. The corpus MUST be fetched or vendored in CI and the gate MUST
  fail on any new mismatch.
- **R13.2** Known non-round-tripping examples MUST be an explicit, named
  allowlist with a reason per entry, so the count can only go down by
  intention. `tasks-roundtrip-failures.md` records the burn-down; the allowlist
  is the machine-readable half of it.
- **R13.2a** A release whose corpus is not separately published MUST still be
  round-tripped against real data. DSTU2 has no downloadable example set, but
  its definition bundles embed 1572 real resources and are committed, so
  `roundtrip_r2_spec` runs unconditionally with no fetch step. This is not a
  formality: it is the check that caught the generator dropping every
  `nameReference` element in DSTU2 — 92 of them, including
  `Bundle.entry.link` and `ValueSet.codeSystem.concept.concept`. Nothing
  errored; the model silently discarded nested concepts and entry links.
  Tests that build their values in Rust cannot find a field the generator
  never emitted, because they can only name fields that exist.
- **R13.3** The round-trip oracle MUST be able to observe a change in a
  decimal's lexical form. Before R2.2, it could not: `serde_json::Value`
  comparison reported `0.50 == 0.5` as equal, so the test that should have
  caught the crate's most serious fidelity defect was blind to it by
  construction. With `arbitrary_precision` guaranteed (R2.2), `Value` numbers
  compare by lexeme and the existing oracle satisfies this requirement — the
  fix to the representation repaired the test as a side effect. A regression
  test MUST pin that property directly, so it cannot silently lapse if the
  representation changes again.
- **R13.4** Deep or adversarial input MUST NOT crash a process embedding this
  crate. Parsing depth, recursive backbone elements (`Questionnaire.item.item`),
  and pathological primitive extensions MUST be covered by fuzz targets, and
  any recursion the model performs on untrusted input MUST be bounded.

### The client behaves under adversity

- **R13.5** `ReleaseClient` MUST apply a default request timeout, a default
  connect timeout, and a response body size cap. A FHIR client with no timeout
  (`reqwest::Client::new()`) hangs forever against a stalled server, and an
  unbounded body lets a hostile or broken peer exhaust memory.
- **R13.6** Path and query components MUST be percent-encoded. `resource_type`
  and `id` are interpolated into the URL directly today
  (`src/client.rs`), so an id containing `/`, `?`, or `..` changes which
  interaction is performed.
- **R13.7** The client MUST support the interactions that make writes safe:
  `If-Match` on update and delete (optimistic concurrency), conditional create
  via `If-None-Exist`, and `ETag` exposure on reads. A client that cannot send
  `If-Match` cannot participate in the concurrency control every FHIR server
  implements.
- **R13.8** The client MUST offer bounded, opt-in retry with exponential
  backoff and jitter for idempotent interactions only (`GET`, `PUT`, `DELETE`),
  and MUST NOT retry `POST` without an explicit idempotency strategy.
- **R13.9** The client MUST provide paging (`Bundle.link[relation=next]`
  traversal) as a bounded iterator, and MUST provide an authorization hook
  (bearer token supplier) so credentials are not smuggled through a
  hand-built `reqwest::Client`.
- **R13.10** The client MUST NOT log or `Debug`-print resource content or
  credentials. `ReleaseClientError::Status` carries a raw body today; it MUST
  be truncated and marked as potentially containing PHI.

### Validation says what it does

- **R13.11** The crate MUST NOT be described as providing FHIR *validation*
  without qualification. It provides structural validation plus three
  invariants of 314 (spec 10). README and crate docs MUST state, in the same
  breath as "validation", what is not checked: FHIRPath invariants, profile
  conformance, terminology membership beyond required-binding enums, and
  reference resolution.
- **R13.12** A FHIRPath evaluator, if added, MUST be introduced as a separate
  capability with its own spec and conformance suite against the published
  FHIRPath test cases — not folded silently into `Validate`.

### Tests must be able to fail

A test that cannot fail is worse than no test: it occupies the space where
coverage would go and reports success while doing it. Each requirement below
exists because a test in this repository was found in exactly that state.

- **R13.17** Adversarial input MUST be covered by fuzz targets that are run,
  not merely committed. `fuzz/` holds targets for the JSON reader, the XML
  reader, and the parse/serialize round trip, and CI MUST run each on every
  change with a bounded time budget and a committed seed corpus. A crash, a
  panic, an abort, or a stack overflow is a failure — a caller can handle an
  `Err`, but nothing can be done about a process that has already died. This
  is the enforcement half of R13.4, which mandated fuzz targets that did not
  exist until the XML reader was found to abort the process on about 160 KB
  of nested input.
- **R13.17a** A generated example MUST exercise something. Every generated
  type carried the same doctest: round-trip `Type::default()` and assert the
  result equals the input. For most types `default()` serializes to `{}`, so
  roughly two thousand doctests across the generated releases were asserting
  that an empty object survives a round trip — no field name, no serde
  rename, no type, and no cardinality was covered by any of them. A generated
  example MUST populate at least one field and assert the **wire name** it
  serializes to, which is the part a server sees and the part a wrong
  `rename_all` silently breaks. Where a type has no field that an example can
  construct, the fallback MUST be the bare round trip, and the reason MUST be
  recorded in the generator rather than left to look deliberate.
- **R13.18** A test asserting a defect is fixed MUST be shown to fail without
  the fix. Reverting the fix, or otherwise mutating the code it guards, MUST
  make the test fail; a test not verified this way is presumed decorative
  until it is. This is not a formality. The recursion test read the wrong
  struct's field for 196 of its 204 cases and passed while checking
  declarations the elements do not have, and the DSTU2 defect it now catches
  had shipped past a full green suite.
- **R13.19** A regression MUST be pinned by the narrowest assertion that
  catches it. Prefer an exact value or a named set over a threshold: a floor
  of "at least 20" tolerates losing four of twenty-four elements, and "more
  than zero" tolerates losing all but one. Where the expected set is large,
  commit it as a snapshot so a regression names what changed rather than
  reporting a smaller number, and require the regeneration step to be
  explicit so a shrinking baseline cannot be adopted by accident.
- **R13.20** Coverage MUST NOT degrade silently. A check that skips — because
  a corpus is absent, a dependency is unpublished, or a file could not be
  located — MUST say so and MUST fail if it ends up checking nothing. A skip
  is indistinguishable from a pass in a CI summary, which is how a corpus
  test addressed to one machine's temporary directory went unnoticed while
  running zero examples.

### Release evidence

- **R13.13** CI MUST run `cargo deny` (advisories, licenses, sources, bans) as
  the RUSTSEC advisory gate, and a release MUST publish a CycloneDX SBOM. A
  crate that handles patient data is a component in someone's IEC 62304 file;
  the evidence is cheap to produce continuously and expensive to reconstruct
  later. The `cargo deny` run MUST use `--all-features`: the default feature
  set omits `client`, `xml`, and the non-default releases, so a
  default-features scan is silent about the network and parsing dependencies a
  deployment is most exposed through.

  An earlier revision of this requirement also demanded `cargo audit`. That was
  deliberately dropped as redundant: cargo-deny's advisory check draws on the
  same RUSTSEC database and additionally honours the `[advisories] ignore`
  policy in `deny.toml`, so it is the single gate — the rationale is recorded
  in `security.yml` itself.
- **R13.14** The crate MUST declare `#![forbid(unsafe_code)]`. *Status:* met by
  1 of the 13 workspace crates. Only the `fhir` facade declares it
  (`src/lib.rs`); `fhir-core` — which carries the REST client and the XML
  reader — `fhir-derive-macros`, and the release crates do not. This is an open
  gap, tracked as T39 in `tasks.md`.
- **R13.15** The published package MUST contain only what a consumer compiles
  against. `llms.txt` is 22 MB and was in the `include` list when this
  requirement was written; it was removed under T32, so the requirement is now
  met. Large generated documentation artifacts belong in the repository or a
  release asset, not in every downstream `cargo vendor` and CI cache.
- **R13.16** A published release MUST be installable in every documented
  feature combination. The motivating defect: `fhir` 1.2.0's `r3`/`r4`
  features failed to compile for downstream users (they built from this
  repository), which blocked fhirpg's `--validate` on those releases. Fixed by
  T30 (published as 1.2.1); the workspace is now at 3.0.0 and CI's `features`
  and `publish-dry-run` jobs build the documented feature combinations as a
  release gate.

## Rationale

Three of these requirements exist because a test could not have caught the
defect it was written to catch. R13.3 is the clearest case: the round-trip
suite is genuinely good, and it still cannot see decimal precision loss,
because the comparison happens after the information is already gone. Where a
test's oracle is weaker than the property, the property is unverified no matter
how many cases run.

R13.5–R13.10 are all the same observation applied to the client: the happy path
was implemented, and the adversarial path — a server that stalls, a peer that
floods, an id that is not an id, two writers racing — was not. That is normal
for a v1 client and disqualifying for a mission-critical one.

## Future work

- R4B and R6 models (spec 12).
- Typed `Reference<T>` (spec 04).

## Acceptance criteria

1. CI round-trips the complete official corpus for R3, R4 and R5, with a
   named allowlist whose length is asserted.
2. The round-trip oracle fails when a decimal's lexical form changes.
3. A fuzz target for `Resource` deserialization runs in CI and has no
   outstanding crashes.
4. A client test asserts that a server which never responds produces a timeout
   error within the configured budget, not a hang.
5. A client test asserts that `id = "../Patient/other"` does not produce a
   request to a different resource.
6. `cargo deny --all-features check` passes in CI as the single advisory gate
   (R13.13); a release publishes an SBOM.
7. The published `.crate` for the current version is under 10 MB and contains
   no `llms.txt`.
