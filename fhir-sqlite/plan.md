# fhir-sqlite plan

> **This plan is the ancestor project's, adapted.** It was written for
> `fhir-postgresql` and copied to every port; the engine-specific text below has
> been corrected, but read it as a record of *why* decisions were made, not as a
> statement of what this port has done. The
> [conformance matrix](../spec/databases/conformance-matrix.md) is the status
> document, and `tasks.md` is the work breakdown (audit **F-61**).
>
> Two things it says that are true of no port: there is no CLI, and the REST
> server is a separate crate, [`fhir-loco`](../fhir-loco/) (`C0.17`, `C0.18`).

Ground-up rewrite of fhir-sqlite: fully normalized relational storage of FHIR®
R3/R4/R5 in SQLite 3. The REST server is a separate crate — `fhir-loco` — and
there is no CLI (`C0.17`, `C0.18`). The prior
fhirbase-style implementation (jsonb bodies) remains in git history and is a
reference, not a base. Normative behaviour: [`spec/index.md`](spec/index.md).
Work breakdown: [`tasks.md`](tasks.md).

## Decisions

- **D1 — Fully normalized schema.** One base table per resource type; child
  tables for every repeating/nested element; no JSONB for live data.
  Chosen by the owner over hybrid-JSONB and views. Consequence: thousands of
  generated tables per version; a database handles this, humans don't — so
  everything is generated (D3) and documented by generated indexes.
- **D2 — All resource types, three versions.** R5 5.0.0 (default), R4 4.0.1,
  R3 3.0.2, each complete, each in its own attached database file (`r5`/`r4`/`r3`).
  Chosen by the owner. Consequence: the generator and generic engine must be
  version-agnostic; only the spec packages differ.
- **D3 — Metadata-driven engine, not mass codegen.** The generator emits DDL
  plus a compact relational map; one generic runtime walks the map to shred
  and reconstruct. Rationale: generating Rust for ~3 versions × ~150
  resources × deep nesting would explode compile times and binary size for
  zero runtime benefit; the map-walking engine is a few thousand lines,
  testable once, correct everywhere. The typed `fhir` crate is used for
  optional strict validation, not as the storage path (its structures don't
  know table names, and double-deserializing every write would be waste).
- **D4 — The `fhir` crate (ours) supplies the typed model.** v1.2.0,
  R3/R4/R5 serde types + spec parser. The generator reuses its
  spec-package parsing where practical rather than re-implementing
  StructureDefinition traversal.
- **D5 — rusqlite, not sqlx.** SQL here is generated and
  dynamic; sqlx's compile-time checking can't see it, so its cost buys
  nothing. (The original reasoning was tokio-postgres's pipelining and
  binary-format parameters; this port's driver is rusqlite.)
- **D6 — axum for HTTP.** Inherited decision, now owned by
  [`fhir-loco`](../fhir-loco/): the HTTP layer left this port and the axum
  choice went with it. Retained because this is where it was recorded.
- **D7 — History is JSONB.** `<resource>_history` stores full-resource
  snapshots as jsonb. This is the sanctioned exception to D1 (with contained
  resources, M3.13): history is write-once audit data read only by
  vread/history; normalizing every historical version would multiply the
  hardest part of the system for no query benefit. The owner's "not merely
  JSON/JSONB" constraint governs live queryable data, which stays fully
  relational.
- **D8 — Verbatim-text temporals with derived sort columns.** FHIR partial
  dates ("2026-07") cannot live losslessly in native date types. Store the
  lexical form; generate a typed `_sort` column for indexing and search
  ranges. Same pattern as decimal: `numeric` preserves precision; where it
  can't (trailing zeros beyond scale), the shredder records the lexical form
  in the primitive-extension channel — round-trip fidelity is the invariant
  (R4.2) and property tests are the enforcement.
- **D9 — Identifier length is a generator problem.** A 63-byte
  limit vs paths like `MedicinalProductDefinition.name.usage`; deterministic
  abbreviation + hash-suffix on collision, with a generated path→name index
  (G2.4). No hand-maintained rename table.
- **D10 — No cross-resource foreign keys.** FHIR allows dangling references;
  enforcement would make load order matter and break real-world data.
  References are parsed into (type, id) columns for joins; an advisory
  integrity report replaces constraints (M3.10).
- **D11 — ETag optimistic concurrency.** `W/"{version_id}"`, If-Match on
  PUT/DELETE, 412 on mismatch; transactions serialize per-resource writes.
- **D12 — Reject unknown elements.** Silent data loss is disqualifying in a
  clinical system; anything the map doesn't know is a 422/load error naming
  the path (R4.3).
- **D13 — Auth is perimeter, not core.** This library implements no
  authentication; that is [`fhir-loco`](../fhir-loco/)'s job (`SV3.x`,
  PASETO v4.public, no unauthenticated mode). This keeps the trust boundary
  explicit and auditable (O10.5). **Amended by D15:** the server
  authenticates, but fhir-sqlite must still *record* who acted — the store
  takes a caller-supplied `Audit` principal on every audited write.
- **D14 — Workspace layout.** One cargo workspace:
  `fhir-sqlite-map` (relational map types + generic shred/reconstruct engine),
  `fhir-sqlite-gen` (spec → DDL + map), `fhir-sqlite-store` (SQLite 3 layer:
  init/load/search/history). **There is no `fhir-sqlite-server` and no
  `fhir-sqlite` CLI binary** — this decision was the ancestor project's, and the
  REST surface became a separate crate, `fhir-loco` (`C0.17`, `C0.18`).
  Generated artifacts live in `assets/` and are embedded in the crate.
- **D15 — Attribution is core, even though authentication is not.** D13
  keeps identity *verification* outside; it does not excuse anonymous
  history. The trusted-proxy header extraction the original PR12.1–PR12.3
  text described was never built here — there is no proxy or header code in
  this library. What exists: the store accepts an `Audit` principal as a
  caller-supplied value type (from the shared `fhir-store` crate) and
  records it on every audited write and every reported disclosure.
  Rationale: HIPAA §164.312(b) asks who accessed a record, and no perimeter
  can answer that for us — the caller knows the identity, only the store
  knows which rows were touched. Consequence: a schema change (M3.15) and
  an access log (PR12.5), both additive.
- **D16 — Audit before latency.** The `--audit-mode` machinery and
  `--allow-unaudited` opt-out the original text described do not exist — no
  mode flags, no bounded queue; `tasks.md` (T41) says the same. What the
  library does is simpler and stricter: audited writes record their audit
  row synchronously, in the same `BEGIN IMMEDIATE` transaction as the
  write, so a write without its record cannot commit. Whether a
  *disclosure* is recorded before a response is released is the caller's
  (`fhir-loco`'s) decision via `log_access`.
- **D17 — Tamper-evidence by hash chain, per resource id.** A global chain
  would serialize every write; per-id chains keep concurrency and still make
  a silent edit or deletion detectable (M3.16). Chosen over write-once
  storage or an external ledger, both of which push the problem into the
  deployment.
- **D18 — Snapshot reads.** Multi-table reads run in one **deferred read
  transaction**, which under WAL observes a stable snapshot for its
  duration (R4.5; annex `M14.20`,
  [`spec/14-sqlite-dialect.md`](spec/14-sqlite-dialect.md)). SQLite has no
  `REPEATABLE READ READ ONLY` syntax — that wording was PostgreSQL's. The
  alternative is reconstructing resources that never existed, which is not
  a trade a clinical store gets to make.
- **D19 — Normalize for search, keep the original for truth.** Accent- and
  case-insensitive matching (P6.6) uses generated normalized columns, not
  mangled stored values. The stored column stays lexically exact for
  round-trip (R4.2); the normalized column exists purely to be indexed and
  matched against.
- **D20 — Protect the data at rest.** The original decision — rustls,
  `sslmode`, a bind guard (O10.7) — is vacuous for an embedded file: there
  is no connection to encrypt. What displaces it is the at-rest obligation:
  the PHI sits in a file whose protection is filesystem permissions and
  disk encryption, both the deployment's responsibility (see `tasks.md`
  T32, which says the same).

## Risks

- **R1 — Schema scale.** ~3,000+ tables per version; `init` time, catalog
  bloat, and dump/restore ergonomics need measurement early (task T4 spike).
  Mitigation: per-version database file; `init` applies the DDL
  statement-by-statement and a failed install is cleaned up by unlinking
  the file (T4/T12 — the "one transaction" wording was the ancestor's;
  `upgrade`, by contrast, genuinely is one transaction, `M14.31`);
  benchmarks from milestone 1.
- **R2 — Reconstruction performance.** Reading one resource touches many
  tables. Mitigation: single round-trip per read using a generated
  multi-table query (one query with UNION/ordering or per-table queries
  pipelined); measure against the old jsonb design in `doc/benchmarks.md`;
  history jsonb (D7) gives vread a fast path.
- **R3 — Search-parameter breadth.** Hundreds of parameters per version;
  FHIRPath expressions in SearchParameter definitions vary in complexity.
  Mitigation: compile the tractable 95% mechanically; emit a generated
  support matrix; lenient-handling for the rest (P6.5) so nothing lies.
- **R4 — Spec-package parsing across three versions.** R3's
  StructureDefinitions differ in detail from R5's. Mitigation: reuse the
  fhir crate's parser; golden-file tests per version.
- **R5 — Extension fidelity.** The relational extension encoding (M3.11) is
  the most intricate part of round-trip. Mitigation: it is exercised by
  every spec example containing extensions plus targeted proptests; built in
  milestone 1, not bolted on.
- **R6 — Audit write amplification.** Every read gains an insert (PR12.5)
  and every write gains a hash computation and wider history row (M3.15,
  M3.16). Mitigation: batched async inserts on a dedicated connection, a
  measured before/after in `doc/benchmarks.md`, and `sync` mode reserved for
  deployments that ask for it. Accept a real cost here; the alternative is
  not shipping into a hospital.
- **R7 — the fold is pure Rust, not a database extension.** This risk was
  PostgreSQL's `unaccent`; it does not apply here, and is retained because the
  decision it drove — fold in Rust — is why P6.6 works identically on all six
  engines.
- **R8 — Schema migration for the audit columns.** M3.15/M3.16 change every
  history table across three versions. Mitigation: the changes are purely
  additive, so `init --upgrade` (T26) already covers them; existing rows get
  `actor = 'unknown (pre-audit)'` and a null hash chain, and `verify-audit`
  reports chains as starting at the first hashed version rather than
  claiming a break.
- **R9 — Snapshot reads under long transactions.** A deferred read
  transaction under WAL holds its snapshot open, and readers pin WAL frames
  until they finish. There is no `statement_timeout` in SQLite — that
  mitigation was PostgreSQL's; what exists is `busy_timeout` (30 s, set at
  open — `sqlite.rs:170`), which bounds how long a writer waits, and the
  single-writer `write_gate`, which keeps write transactions short.

## Milestones

- **M1 — Engine proven (R5, vertical slice).** Generator produces DDL + map
  for all R5 resource types; shred/reconstruct round-trips every R5 spec
  example; the store's `init`/`put`/`get` work; round-trip tests green
  against a local file. (The original listed the
  `init`/`load`/`transform`/`export` CLI verbs and "live-PG" tests — there
  is no CLI, `export` exists in no port, and this port needs no server.)
  Exit criterion: R4.2 holds for the entire R5 examples corpus.
- **M2 — History + CRUD semantics.** version_id/history/soft delete;
  transactional writes; ETag concurrency; `fhir_sqlite_meta` and idempotent init.
- **M3 — Search.** Search-parameter compiler, indexes, result parameters,
  paging; generated support matrix; search test suite.
- **M4 — REST server.** *Not this port's milestone.* It became
  [`fhir-loco`](../fhir-loco/) — Loco.rs, Axum — serving CRUD, vread/history,
  search and a CapabilityStatement over a store. Retained here because the
  decision to keep HTTP out of the port is a real design decision and this is
  where it is recorded.
- **M5 — R4 and R3.** Run the same generator + engine over 4.0.1 and 3.0.2
  spec packages; version-specific quirks fixed; full example-corpus
  round-trip per version.
- **M6 — Production hardening.** Metrics, health, logging redaction,
  migrations/upgrade path, TLS feature, benchmarks + regression gate, book,
  security review, crates.io release.
- **M7 — Trustworthy under load and under audit.** The gap between "works"
  and "may hold patient data". Correctness under concurrency (snapshot
  reads, atomic conditionals, honored preconditions), the audit envelope and
  access log, tamper-evident history, encrypted database transport,
  configured service base URL, PHI response headers, edge resource limits,
  worldwide string search, and supply-chain evidence. Exit criterion: the
  §13 compliance table has a passing test in every Evidence cell.

## Non-goals (this rewrite)

- SMART-on-FHIR / OAuth in-core (D13), terminology services ($expand,
  $validate-code), FHIRPath query engine, subscriptions, GraphQL, Bulk Data
  *export* serving (import via `load` is in; `bulkget` client can return
  later), profile/IG validation beyond base spec (the ePL IG informed R5
  requirements but IG-specific profile enforcement is future work).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
