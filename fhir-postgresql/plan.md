# fhir-postgresql plan

> **A record of decisions, not of status.** This plan is the ancestor
> project's, adapted: it was written when the server and CLI lived inside the
> port, and it survives as the record of *why* decisions were made. Two things
> it assumes are true of no port: there is no CLI, and the REST server is a
> separate crate, [`fhir-loco`](../fhir-loco/) (`C0.17`, `C0.18`). The
> [conformance matrix](../spec/databases/conformance-matrix.md) is the status
> document (audit **F-61**).

Ground-up rewrite of fhir-postgresql: fully normalized relational storage of FHIR
R3/R4/R5 in PostgreSQL 18, as an embeddable library — the REST server is a
separate crate, `fhir-loco`, and there is no CLI. The prior
fhirbase-style implementation (jsonb bodies) remains in git history and is a
reference, not a base. Normative behaviour: [`spec/index.md`](spec/index.md).
Work breakdown: [`tasks.md`](tasks.md).

## Decisions

- **D1 — Fully normalized schema.** One base table per resource type; child
  tables for every repeating/nested element; no JSONB for live data.
  Chosen by the owner over hybrid-JSONB and views. Consequence: thousands of
  generated tables per version; PostgreSQL handles this, humans don't — so
  everything is generated (D3) and documented by generated indexes.
- **D2 — All resource types, three versions.** R5 5.0.0 (default), R4 4.0.1,
  R3 3.0.2, each complete, each in its own PostgreSQL schema (`r5`/`r4`/`r3`).
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
- **D5 — tokio-postgres + deadpool, not sqlx.** SQL here is generated and
  dynamic; sqlx's compile-time checking can't see it, so its cost buys
  nothing. tokio-postgres gives pipelining and binary-format parameters.
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
- **D9 — Identifier length is a generator problem.** 63-byte PostgreSQL
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
  authenticates, but fhir-postgresql must still *record* who acted — the
  store takes a caller-supplied `Audit` principal on every audited write.
- **D14 — Workspace layout.** One cargo workspace:
  `fhir-postgresql-map` (relational map types + generic shred/reconstruct engine),
  `fhir-postgresql-gen` (spec → DDL + map), `fhir-postgresql-store` (PostgreSQL layer:
  init/load/search/history). **There is no `fhir-postgresql-server` and no
  `fhir-postgresql` CLI binary** — this decision was the ancestor project's, and the
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
  knows which rows were touched. Consequence: a schema change (M3.15) and an
  access log (PR12.5), both additive.
- **D16 — Audit before latency.** The `--audit-mode` machinery and
  `--allow-unaudited` opt-out the original text described do not exist — no
  mode flags, no bounded queue. What the library does is simpler and
  stricter: audited writes record their audit row synchronously, in the
  same transaction as the write, so a write without its record cannot
  commit. Whether a *disclosure* is recorded before a response is released
  is the caller's (`fhir-loco`'s) decision via `log_access`.
- **D17 — Tamper-evidence by hash chain, per resource id.** A global chain
  would serialize every write; per-id chains keep concurrency and still make
  a silent edit or deletion detectable (M3.16). Chosen over write-once
  storage or an external ledger, both of which push the problem into the
  deployment.
- **D18 — Snapshot reads.** Multi-table reads run in one
  `REPEATABLE READ READ ONLY` transaction (R4.5). The cost is one extra
  round trip per read; the alternative is reconstructing resources that
  never existed, which is not a trade a clinical store gets to make.
- **D19 — Normalize for search, keep the original for truth.** Accent- and
  case-insensitive matching (P6.6) uses generated normalized columns, not
  mangled stored values. The stored column stays lexically exact for
  round-trip (R4.2); the normalized column exists purely to be indexed and
  matched against.
- **D20 — Encrypt the database link by default.** rustls, `sslmode` honored,
  and — since **F-17** was fixed — a default that *verifies* the server
  certificate (`SslPolicy::Require`, pinned by `tests/ssl_default.rs`),
  which is stronger than the original text asked for (O10.7). The "startup
  refusal when a non-loopback bind meets an unencrypted connection" half was
  server fiction: a bind guard is [`fhir-loco`](../fhir-loco/)'s concern,
  not a library's. PHI in flight to PostgreSQL is exactly as sensitive as
  PHI in flight to the client.

## Risks

- **R1 — Schema scale.** ~3,000+ tables per version; `init` time, catalog
  bloat, and dump/restore ergonomics need measurement early (task T4 spike).
  Mitigation: per-version PostgreSQL schemas, generated DDL applied in one
  transaction, benchmarks from milestone 1.
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
- **R7 — `unaccent` is an extension, not core PostgreSQL.** Retired: the
  SQL `unaccent` dependency was deliberately removed. The fold is pure Rust
  (`fhir_postgresql_map::fold`) and the database stores its output, so P6.6
  needs no extension at all — `ddl.rs:444` asserts the emitted DDL contains
  no `unaccent`. Kept because the decision it drove — fold in Rust — is why
  P6.6 works identically on all six engines.
- **R8 — Schema migration for the audit columns.** M3.15/M3.16 change every
  history table across three versions. Mitigation: the changes are purely
  additive, so `init --upgrade` (T26) already covers them; existing rows get
  `actor = 'unknown (pre-audit)'` and a null hash chain, and `verify-audit`
  reports chains as starting at the first hashed version rather than
  claiming a break.
- **R9 — Snapshot reads under long transactions.** REPEATABLE READ readers
  hold a snapshot; a slow reconstruction of a very large resource delays
  vacuum. Mitigation: reads are already bounded by `statement_timeout`, the
  transaction is READ ONLY, and bloat is watched by the existing metrics.

## Milestones

- **M1 — Engine proven (R5, vertical slice).** Generator produces DDL + map
  for all R5 resource types; shred/reconstruct round-trips every R5 spec
  example; the store's `init`/`put`/`get` work; live-PG round-trip tests
  green. (The original listed the `init`/`load`/`transform`/`export` CLI
  verbs — there is no CLI, and `export` exists in no port at all.) Exit
  criterion: R4.2 holds for the entire R5 examples corpus.
- **M2 — History + CRUD semantics.** version_id/history/soft delete;
  transactional writes; ETag concurrency; `fhir_postgresql_meta` and idempotent init.
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
- **M6 — Production hardening.** Migrations/upgrade path, database TLS
  (there is no `tls` cargo feature in this workspace — `SslPolicy` is
  unconditional), benchmarks + regression gate, book, security review.
  Nothing has been published to crates.io; the metrics/health/redaction
  items were the server's and left with it.
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
