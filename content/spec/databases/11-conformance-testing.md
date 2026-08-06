# 11. Conformance testing

- **T11.1** Round-trip property tests (`R4.2`) over every example resource
  shipped with each FHIR specification, plus generated resources.
- **T11.2** Live-database integration tests exercise every store operation
  against the port's target engine in CI, at the version its annex declares
  (`O10.12`). *This requirement historically read "every REST interaction in
  §7"; §7 is retired (`C0.15`) and the citation is unresolved — see `C0.16`.*
- **T11.3** Search semantics tests derive cases from the FHIR search
  specification per parameter type, including precision-edge dates and token
  system matching.
- **T11.4** Where a port publishes a conformance description, it MUST be
  generated from what is actually implemented (the relational map + supported
  params), never hand-edited.
- **T11.5** Load benchmarks are tracked in `doc/benchmarks.md`; a regression
  gate compares against the recorded baseline.
- **T11.6** Concurrency is tested adversarially, not assumed: a reader looping
  against a writer MUST never observe a torn resource (`R4.5`); N racing
  conditional creates with identical criteria MUST produce exactly one resource;
  N racing precondition-guarded updates MUST produce exactly one success and
  N−1 failures; N racing writers to one resource id MUST produce N distinct
  consecutive `version_id`s and a chain that verifies (`H5.4`).
- **T11.7** A redaction test asserts that no log line emitted during a full
  CRUD + search cycle over a resource containing a distinctive marker value ever
  contains that marker (`O10.2`), and that no error surfaced to a caller echoes
  a submitted value.
- **T11.8** An audit test asserts that every write records its principal
  (`M3.15`), that every read appends an access record (`PR12.5`), that the hash
  chain verifies in every configured algorithm (`M3.16`, `M3.16a`), and that a
  direct `UPDATE`/`DELETE` on a history table is rejected by the database
  (`M3.17`).

  A test MUST also assert that tampering is caught **independently by each
  algorithm**, since a chain that only ever fails in one of them proves nothing
  about the others. A test MUST assert that a **truncated** chain still verifies
  clean while the checkpoint changes (`M3.16c`) — that gap is the checkpoint's
  whole reason for existing, and a test that only checked the checkpoint moved
  would not show it. A test MUST assert that rotating a key leaves history
  signed under the retired key verifiable, and that dropping that key yields
  *unverifiable*, never a break (`M3.16b`).

- **T11.9** Adversarial input MUST be covered by fuzz targets that are **run,
  not merely committed**. Parsers that accept documents from outside the process
  MUST be fuzzed on every change with a bounded time budget and a committed seed
  corpus, and a crash, panic, abort, or stack overflow MUST fail the build.

  A stack overflow is not unwindable: it is not caught by `catch_unwind`, a
  worker thread cannot contain it, and the process ends. For a component holding
  clinical data, one document ending the process is a denial of service that
  requires no cleverness. The sibling `fhir` crate's XML reader aborted on
  roughly 160 KB of nested input, well under any sane size limit, and nothing
  detected it for the life of the module.

- **T11.10** A test asserting a defect is fixed MUST be shown to fail without
  the fix. Reverting the fix, or mutating the code it guards, MUST make the test
  fail; a test not verified this way is presumed decorative until it is. This
  matters most for the tamper-evidence tests in `T11.8`, where a test that
  cannot fail is indistinguishable from a control that works — and the
  distinction is the entire value of the control.
- **T11.11** A regression MUST be pinned by the narrowest assertion that catches
  it. Prefer an exact value or a named set over a threshold: a floor of "at
  least 20" tolerates losing four of twenty-four, and "more than zero" tolerates
  losing all but one. Where the expected set is large, commit it as a snapshot
  so a regression names what changed, and keep regeneration an explicit,
  reviewed step so a shrinking baseline cannot be adopted by accident.
- **T11.12** Coverage MUST NOT degrade silently. A check that skips — because a
  corpus is absent, a database is unreachable, or a path could not be resolved —
  MUST say so, and MUST fail if it ends up checking nothing. A skip is
  indistinguishable from a pass in a CI summary.

  The corpus test in the original located its inputs through an absolute path
  into one machine's temporary directory: it skipped silently in CI for its
  whole life, and on the machine where that directory survived it reported a
  data-fidelity failure that was really a missing fetch. Inputs MUST be resolved
  relative to the repository or an environment variable, never an absolute path
  outside it.

- **T11.13** A test that self-skips without its database MUST NOT be the only
  evidence for a conformance level (`C0.9`). Where a port's live suite requires
  a DSN that its own pipeline never sets, that suite is not a gate, and the
  [conformance matrix](conformance-matrix.md) MUST record the requirement as
  unverified rather than as passing. Originating defect **F-06**, fixed.
- **T11.14** A test disabled with `#[ignore]` because it asserts another
  engine's behaviour MUST be accompanied by an entry in the port's `tasks.md`
  and in the [conformance matrix](conformance-matrix.md). An ignored test is a
  known gap; an ignored test nobody tracks is a forgotten one.
- **T11.15** A test MUST be **deterministic**: the same tree against the same
  engine MUST produce the same verdict. A test that passes intermittently is not
  weaker evidence than a failing one — it is worse, because the habit it teaches
  is to run it again, and a re-run is indistinguishable from a fix.

  This applies with most force to a test that sets up shared state. Where a live
  test clears a database before installing, the teardown MUST fail loudly on
  error and the test MUST assert the state is actually clean before proceeding.
  A discarded teardown error does not vanish; it reappears later as a failure
  attributed to correct code. That is exactly what **F-52** did — a broken
  cleanup surfaced as a rejected `CREATE TABLE` eight statements downstream, and
  the statement it blamed was fine.

  A test whose flakiness is suspected but not yet understood MUST be recorded in
  [`audit.md`](audit.md) rather than retried until green.

---

Part of the [fhir-databases specification](index.md).
