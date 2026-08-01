# fhir-mssql plan

Ground-up rewrite of fhir-mssql: fully normalized relational storage of FHIR
R3/R4/R5 in PostgreSQL 18, with a FHIR REST server and CLI. The prior
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
- **D6 — axum for HTTP.** Boring, maintained, tower middleware for
  timeouts/limits/tracing.
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
- **D13 — Auth is perimeter, not core.** The server implements no
  authentication; deployments front it with their identity layer. This keeps
  the trust boundary explicit and auditable. Documented prominently (O10.5).
  **Amended by D15:** the perimeter authenticates, but fhir-mssql must still
  *record* who acted.
- **D14 — Workspace layout.** One cargo workspace:
  `fhir-mssql-map` (relational map types + generic shred/reconstruct engine),
  `fhir-mssql-gen` (spec → DDL + map), `fhir-mssql-store` (PostgreSQL layer:
  init/load/search/history), `fhir-mssql-server` (axum), `fhir-mssql` (CLI binary
  tying it together). Generated artifacts live in `assets/` and are embedded
  in the binary.
- **D15 — Attribution is core, even though authentication is not.** D13
  keeps identity *verification* outside; it does not excuse anonymous
  history. fhir-mssql accepts a principal from a trusted proxy (PR12.1–PR12.3)
  and records it on every write and every read. Rationale: HIPAA
  §164.312(b) asks who accessed a record, and no perimeter can answer that
  for us — the perimeter knows the identity, only the store knows which rows
  were touched. Consequence: a schema change (M3.15) and an access log
  (PR12.5), both additive.
- **D16 — Audit before latency.** Access logging defaults to `async` with a
  bounded queue that **fails closed** (PR12.6). Dropping a disclosure record
  to keep latency down is the wrong trade for this system; a deployment that
  disagrees says so explicitly with `--allow-unaudited`.
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
- **D20 — Encrypt the database link by default.** rustls, `sslmode`
  honored, and a startup refusal when a non-loopback bind meets an
  unencrypted database connection (O10.7). PHI in flight between the server
  and PostgreSQL is exactly as sensitive as PHI in flight to the client.

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
- **R7 — `unaccent` is an extension, not core PostgreSQL.** P6.6 depends on
  it, and managed providers vary in whether an unprivileged role may create
  it. Mitigation: `init` probes for it and fails with a clear instruction
  rather than degrading silently; a fallback pure-SQL folding function
  covers Latin-1/Latin Extended-A when the extension is unavailable.
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
  example; `init`/`load`/`transform`/`export` work; live-PG round-trip tests
  green. Exit criterion: R4.2 holds for the entire R5 examples corpus.
- **M2 — History + CRUD semantics.** version_id/history/soft delete;
  transactional writes; ETag concurrency; `fhir_mssql_meta` and idempotent init.
- **M3 — Search.** Search-parameter compiler, indexes, result parameters,
  paging; generated support matrix; search test suite.
- **M4 — REST server.** axum server: CRUD, vread/history, search,
  capability statement, batch/transaction, OperationOutcome errors, limits;
  integration suite for §7.
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
