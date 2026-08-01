# fhir-sqlite tasks

> **Parts of this file are untrue of this port (audit [F-27](../spec/audit.md#f-27)).**
> The `M4 — REST server` milestone, `T8 CLI v1`, and `T23 Multi-version serve`
> are checked off, and none of that code exists in any port: there is no
> `fhir-*-server` crate, no `serve` binary, and no REST test suite anywhere in
> this repository.
>
> Do not read a `[x]` here as evidence. The
> [conformance matrix](../spec/conformance-matrix.md) is the status document to
> trust.
>
> This port's store tasks also describe **PostgreSQL** mechanisms it does not use
> — `tokio-postgres`, `FOR UPDATE` row locks, staged-schema install, `ILIKE` —
> because the file was copied per port and never re-read. The store is real; the
> description of how it works is the reference port's.

Work breakdown for the plan's milestones. Each task lists its acceptance
criterion. Order within a milestone is roughly dependency order.

## M1 — Engine proven (R5 vertical slice)

- [x] **T1 Workspace scaffold.** Cargo workspace per plan D14
  (`fhir-sqlite-map`, `fhir-sqlite-gen`, `fhir-sqlite-store`, `fhir-sqlite` — the server crate
  arrives with M4), CI (fmt, clippy, test, live-PG job).
  *Done:* `.github/workflows/ci.yml`; tests self-skip without inputs.
- [x] **T2 Spec-package ingestion.** profiles-resources.json +
  profiles-types.json parsed directly (simpler than reusing the fhir
  crate's parser; that crate still backs `--validate` later) into element
  trees with cardinality, types, choice and contentReference info — for all
  three versions, not just R5. SearchParameters ingestion moves to M3.
- [x] **T3 Relational map format.** `fhir-sqlite-map::model`: node arena (cycles
  via indexes), tables, typed columns, choice variants, reference splits,
  extension/spill channels, 63-byte registry with deterministic
  abbreviation + hash fallback. Assets: `assets/fhir-sqlite-relmap-{r3,r4,r5}
  .json.gz` + CHECKSUMS.txt.
- [x] **T4 DDL generator + scale spike.** Full R5 = 7,355 tables installs
  in 9.5 s via staged-schema + rename (single-transaction DDL exhausts
  PostgreSQL's lock budget — G2.5 amended). Numbers in doc/benchmarks.md;
  risk R1 retired. *Remaining for M3:* search indexes; value-set CHECKs
  (M3.7) not yet emitted.
- [x] **T5 Shredder.** Generic walker: scalars, ords-array child tables,
  choices (incl. force-split wide choices), reference parsing, extension
  leaf rows, primitive extensions with null-padded arrays, element ids,
  contained, type-cycle spill, unknown-element rejection.
- [x] **T6 Reconstructor.** Inverse walker with consumption auditing (every
  row must be used exactly once — gaps surface as integrity errors, never
  silent loss). *Accept exceeded:* the entire three-version corpus
  (7,399 examples) round-trips in memory.
- [x] **T7 Store layer: init/load/read.** tokio-postgres + deadpool;
  transactional put with history append; pipelined multi-table reads;
  chunked multi-row inserts; text-image wire protocol with explicit casts.
  *Accept:* full-corpus live round trip 7,396/7,396 across r3/r4/r5;
  bulk benchmark: 6,146 res/s load, 1.18 ms reads (doc/benchmarks.md).
- [x] **T8 CLI v1.** `gen`, `init`, `load` (NDJSON/Bundle/single, gzip,
  content-detection, per-resource error reporting, nonzero exit), `get`,
  `delete`, `export`, `transform`. *Remaining:* streaming (bounded-memory)
  reads for multi-GB NDJSON — currently whole-file.
- [x] **T9 Round-trip property tests.** Map-driven random-resource
  generator (deterministic SplitMix64 seeds — no proptest dependency):
  deep recursion, sparse primitive arrays with extensions, nested
  extensions, choice variants, decimals, partial dates. 10k cases pass
  (`FHIR_SQLITE_PROPTEST_CASES`; default 500 locally).
  *Found a real bug the 7,399-example corpus missed:* two cyclic
  contentReferences into one table (QuestionnaireResponse `item.item` +
  `item.answer.item`) made ordinal paths ambiguous — fixed with ordinal
  sign lanes (`Elem::neg_lane`); the reconstructor's consumption audit is
  what caught it.

## M2 — History + CRUD semantics

- [x] **T10 version_id + history tables.** H5.1–H5.3 in the store:
  history append on C/U/D, soft delete, `vread`, `history`, and `status`
  (the 404-vs-410 distinction); version numbering continues past deletes
  (derived from history max, not the base row).
  *Accept met:* m2_semantics integration test — create→update→delete shows
  D/U/C history, vread of each version matches, deleted reads as Deleted.
- [x] **T11 Optimistic concurrency.** `put_if(resource, expected_version)`
  under FOR UPDATE row locks; `StoreError::Conflict` for the API's 412;
  expected 0 = create-only (If-None-Exist shape). *Accept met:* two racing
  conditional writers — exactly one wins.
- [x] **T12 fhir_sqlite_meta + idempotent init.** Staged-schema install +
  atomic rename, checksum recorded; re-init no-ops on matching checksum
  and refuses a mismatch. Chunked `drop_schema` + `fhir-sqlite drop --yes`.
  *Accept met* in m2_semantics.

## M3 — Search

- [x] **T13 Search-parameter compiler.** FHIRPath subset (unions, casts
  `ofType`/`as`, lenient `where(resolve())`) resolved by walking the map
  tree; targets embedded in the map assets per resource; every uncompiled
  parameter carries its reason (`SearchDef::note`).
  *Accept met:* 94.8% of R5's 1,972 parameters compiled (1,870); the
  remainder are composite/special and exists()-style expressions.
- [~] **T14 Query builder + result params.** Done: `Store::search` — AND
  across params, OR across values and targets, all user input bound (no SQL
  interpolation), modifiers :exact/:contains, token system|code, date
  prefixes with precision ranges + Period overlap, quantity value|system|
  code, reference forms, `_id`, `_lastUpdated`, `_count`/offset; strict
  unsupported-parameter errors; `fhir-sqlite search` CLI; `_sort` (base-table
  params + _id/_lastUpdated, honest errors otherwise) and
  `_total=accurate`. *Accept mostly met:* search_semantics + rest suites
  green against live PG. Single-hop `_include` (via compiled reference
  targets) and `_revinclude` (via the search machinery) with
  search.mode=include entries and dangling-reference tolerance.
  *Remaining:* chained `reference.`, cursor paging, lenient handling.
- [x] **T15 Index emission + explain audit.** One index per distinct
  search-target column set emitted with the DDL (R5: 1,813 indexes; full
  init 5.8 s). EXPLAIN audit in tests/bench.rs: token/reference/date
  searches all plan index scans at 100k resources; the test fails on seq
  scans. *Note:* ILIKE-prefix string search bypasses btree — revisit with
  text_pattern_ops if profiles demand.

## M4 — REST server

> **The whole of M4 is untrue of this port (audit [F-27](../spec/audit.md#f-27)).**
> No port in this repository has a `fhir-*-server` crate, a `serve` binary, or a
> REST integration suite; every workspace is exactly `-map`, `-gen`, and
> `-store`. The `[x]` marks below record the ancestor project's history, not
> this port's. Whether a server is planned here at all is undecided — see
> **F-05**.

- [x] **T16 axum skeleton.** `fhir-sqlite-server` crate + `fhir-sqlite serve`:
  versioned base paths, application/fhir+json, 32 MiB body limit,
  OperationOutcome error mapping (400/404/410/412/501/500 with opaque
  internals), /health + /ready. *Accept met* in the rest integration
  suite. *Remaining:* request ids, graceful-shutdown wiring (M6).
- [x] **T17 Full CRUD + history endpoints.** create (server-assigned ids,
  Location + ETag), read (404 vs 410), update with If-Match → 412, delete
  (idempotent 204), instance history bundle, vread.
  *Accept met:* §7 rest suite green, including If-None-Exist conditional
  create (0 → create, 1 → 200 with the match, many → 412).
  *Remaining:* conditional delete-by-search.
- [x] **T18 Search over HTTP.** GET + POST `_search` (query + form
  merged), searchset bundles with fullUrl, self/next links; next-link
  paging verified by walking it in the test. `_count` capped at 1000;
  unimplemented result params answer 501 rather than lying.
- [x] **T19 Batch/transaction.** Batch: independent entries (GET read,
  POST, PUT, DELETE) with per-entry statuses. Transaction: DELETE→POST→PUT
  ordering, urn:uuid reference rewriting (JSON-walk, whole-string match),
  single database transaction via `Store::transact`.
  *Accept met:* urn resolution verified end-to-end; poison-entry
  transaction provably rolls back. *Remaining:* GET entries inside
  transactions, conditional references.
- [x] **T20 CapabilityStatement generation.** Generated per version from
  the map + compiled search params (only supported params listed) — never
  hand-edited. *Remaining:* touchstone-style external validation (M6).

## M5 — R4 and R3

> **Ledger drift, needs a human call.** T21 and T22 are unchecked, but T7
> records a full-corpus live round trip of 7,396/7,396 across r3/r4/r5, the
> README claims all three corpora round-trip losslessly, and `fhir-sqlite serve`
> mounts all three (T23, checked). Either the boxes are stale or the README
> overstates. Since the spec is meant to be the source of truth, reconcile
> before release rather than after.

- [ ] **T21 R4 artifacts.** Run generator on 4.0.1; fix spec-parsing deltas.
  *Accept:* full R4 examples corpus round-trips live; REST suite green on
  `/r4`.
- [ ] **T22 R3 artifacts.** Same for 3.0.2.
  *Accept:* full R3 examples corpus round-trips live; REST suite green on
  `/r3`.
- [x] **T23 Multi-version serve.** `fhir-sqlite serve` mounts every version
  whose assets exist and whose schema is installed; per-version capability
  statements. *Verified:* one process serving r3 + r4 + r5, curl-checked.

## M6 — Production hardening

- [x] **T-validate (V9.2).** `fhir-sqlite load --validate` deserializes each
  resource through the typed `fhir` crate model behind the `validate`
  build feature. **All three versions**, since `fhir` 1.2.1.
  *The upstream bug is fixed and released:* published 1.2.0 could not compile
  its own r3/r4 features — the `Validate` derive expanded to `crate::r5::`
  paths — but only for crates.io consumers, since the repository resolves the
  derive macro through its `path` dependency where the fix already lived. It
  needed `fhir-derive-macros` 1.0.1 published alongside `fhir` 1.2.1; a
  `fhir` release on its own would have pulled the same broken macro.
  *Accept met:* `validate_tests` covers R3/R4/R5 and records the one thing
  `--validate` does not catch (unknown elements — serde ignores them; the
  shredder rejects them per D12).
- [x] **T-graceful.** `fhir-sqlite serve` shuts down cleanly on SIGINT/SIGTERM.
- [~] **T24 Observability.** Done: /health, /ready, /metrics (Prometheus
  text: request/response-class/latency counters), X-Request-Id
  (propagated or generated) with per-request tracing that logs
  method/path/status only — never resource content. *Remaining:* JSON log
  format wiring in the CLI, an automated redaction test, latency
  histogram buckets.
- [~] **T25 Pool + timeout hardening.** Done: server-side
  statement_timeout (FHIR_SQLITE_STATEMENT_TIMEOUT_MS, default 30 s), pool
  wait timeout 2 s, exhaustion → 503 + Retry-After.
  *Remaining:* an automated saturation test.
- [x] **T26 Migrations + upgrade.** `init` stores the map asset in
  fhir_sqlite_meta; `init --upgrade` diffs installed vs current maps and
  applies additive DDL (new tables/columns/indexes) in lock-safe chunks;
  destructive steps refuse without --allow-destructive; column type
  changes always demand manual migration. *Accept met:* upgrade test —
  reduced install upgrades to full, data survives, re-upgrade no-ops,
  downgrade guarded then forced.
- [x] **T27 TLS feature + bind guard.** rustls in-process behind the
  `tls` feature (`serve --tls-cert/--tls-key`, axum-server) with graceful
  shutdown; loopback-default binding (an explicit --bind is the
  non-loopback acknowledgement). *Verified:* live HTTPS smoke test
  (HTTP/2, CapabilityStatement served, clean SIGTERM shutdown).
- [~] **T28 Benchmarks + regression gate.** Done: gated bench harness
  (load throughput, read latency, EXPLAIN audit) + doc/benchmarks.md with
  measured numbers (6,146 res/s; 1.18 ms reads at 100k).
  *Remaining:* CI regression gate against a recorded baseline; comparison
  against the historical jsonb implementation.
- [x] **T29 Book + generated schema docs.** mdBook (9 chapters:
  introduction, getting started, storage model, SQL querying, search,
  REST API, versions, operations, architecture); builds locally and in
  CI. Column/table→FHIR-path mapping ships inside the map assets
  themselves. *Remaining nicety:* a rendered path→table index page.
- [~] **T30 Security review + release.** Done: LICENSE-MIT/APACHE,
  CHANGELOG, publish metadata (versioned internal deps, keywords), map
  assets embedded in the binary so `cargo install fhir-sqlite` is
  self-contained, `cargo publish --dry-run` clean for the leaf crate.
  *Remaining (human decisions):* pick the release version, publish the
  five crates in dependency order, tag; optionally add cargo-audit/deny
  to CI.

## M7 — Trustworthy under load and under audit

The gap between "works end to end" and "may hold patient data". Ordered by
severity: P0 items are defects in what already ships, P1 items are missing
guarantees, P2 items are reach.

### P0 — defects in shipped behaviour

- [x] **T31 Snapshot reads (R4.5).** `Store::get` reads the base row and
  every child table as separate implicit transactions
  (`fhir-sqlite-store/src/lib.rs:554`), while `put_in` deletes and re-inserts
  (`:512`). A write landing mid-read reconstructs a resource that never
  existed. Wrap every multi-statement read in one
  `REPEATABLE READ READ ONLY` transaction; same for `export` and search
  materialization.
  *Accept:* a reader loop against a writer loop over 10k iterations never
  observes a torn resource and never errors (T11.6).
- [x] **T32 Encrypted database transport (O10.7).** `NoTls` is hard-coded
  (`fhir-sqlite-store/src/lib.rs:186`), so PHI crosses to PostgreSQL in clear and
  `sslmode=require` cannot be honored. Add `tokio-postgres-rustls`, honor
  `sslmode`/`PGSSLROOTCERT`, default `prefer`, and refuse a non-loopback
  `--bind` over an unencrypted connection without `--allow-insecure-db`.
  *Done:* `SslPolicy`, rustls connector, `PGSSLROOTCERT` trust anchors,
  `Store::connect_with`, the `serve` startup guard, and a startup warning
  whenever the link is unencrypted. fhir-sqlite's `require` validates the
  certificate where libpq's does not — a documented deviation, in the safe
  direction. The startup refusal is now split out as `refuse_insecure_db`
  and tested as a policy table (`startup_guard_tests`), including the case
  where the bind will not resolve — "I could not tell" must count as
  not-loopback, or the check silently skips itself.
  The live test against a TLS-only PostgreSQL now runs in CI
  (`ci.yml`, `tls-database`): a `hostssl`-only server, with a step that first
  proves plaintext really is refused — a gate that silently permits downgrade
  tests nothing — then runs the live suite with `PGSSLMODE=require` and the
  self-signed certificate as its own trust anchor.

  *Remaining:* nothing on GitHub. There is no Woodpecker counterpart, because
  Woodpecker starts services before workspace steps run, so a certificate
  generated in a step does not exist when the database container boots; the
  workarounds (a committed test key, or docker-in-docker) are each worse than
  the gap. Recorded in `doc/ci.md` rather than left to be discovered.
- [x] **T33 Atomic conditional interactions (A7.10).** `If-None-Exist`
  searches then writes (`fhir-sqlite-server/src/lib.rs:444`); two concurrent
  identical conditional creates both create. Move match and write into one
  transaction guarded by `pg_advisory_xact_lock` on the criteria hash; same
  for conditional delete and conditional update.
  *Done:* `Store::conditional_create`/`conditional_delete` take
  `pg_advisory_xact_lock` on a sorted hash of the criteria, then match and
  write in one transaction. *Accept met:* 8 racing conditional creates yield
  exactly one resource and seven `Existing`. *Remaining:* conditional update
  (the server does not implement it yet).
- [x] **T34 Audit envelope on history (M3.15, PR12.1–PR12.4).** History
  records no actor at all. Add the audit columns to the generated history
  DDL, thread a `Principal` through the store write path, and extract it
  from the configured trusted header behind `--trust-proxy`. Additive, so
  `init --upgrade` migrates existing installs.
  *Done:* `Audit` envelope threaded through `put_audited`/`delete_audited`/
  `transact_audited`/`conditional_*_audited`; `PrincipalPolicy` on the
  server honoring a configured header **only** behind `--trust-proxy`;
  `--require-principal` → 401; CLI writes attributed to the operator
  (`Audit::cli()`). Upgrade reconciles the new columns idempotently.
  *Accept met:* the `audit` suite asserts all three, and that a plain `put`
  records `unauthenticated` rather than nothing.
- [x] **T35 Configured service base URL (A7.7).** Bundle `fullUrl` and
  `link` are built from the `Host` header and hard-coded `http://`
  (`fhir-sqlite-server/src/lib.rs:739`). Add `--base-url`, honor forwarded
  headers only under `--trust-proxy` with a host allowlist.
  *Done:* `BaseUrl` with `--base-url`, `--trust-proxy`, `--allowed-host`;
  the default emits URLs for the address actually bound and reads no request
  header at all. *Remaining:* a test asserting a poisoned `Host` changes
  nothing.
- [x] **T36 Preconditions inside bundles (A7.9).** `parse_entries`
  (`:888`) ignores `ifMatch`/`ifNoneExist`/`ifModifiedSince`. Honor them, or
  fail the entry 501 — never accept-and-ignore. *Done:* `ifMatch` is honored
  in batch and transaction entries (412 on mismatch, mapped as 412 rather
  than a generic 400); `ifNoneExist`/`ifModifiedSince`/`ifNoneMatch` fail the
  entry 501, and fail the whole bundle for a transaction.
- [x] **T37 Reference rewriting precision.** `rewrite_refs` (`:1130`)
  replaces any string equal to a `urn:uuid`, including narrative and
  `valueString`. Restrict to `Reference.reference` values and
  `Bundle.entry.fullUrl`. *Done:* only `reference` keys are rewritten.
- [x] **T38 Resource id validation (R4.6).** No `[A-Za-z0-9\-\.]{1,64}`
  check anywhere; ids from URL or body land in unbounded `text`.
  *Done:* `valid_fhir_id` guards read, vread, history, update, delete, and
  Bundle entry urls. *Remaining:* the same check inside `fhir-sqlite load`.
- [x] **T39 PHI response headers (A7.8).** `Cache-Control: no-store`,
  `Pragma: no-cache`, `X-Content-Type-Options`, `Referrer-Policy`; CORS
  denied unless `--cors-origin` names an origin. *Done:* the four headers are
  set on every response. *Remaining:* `--cors-origin` (there is still no CORS
  layer at all, which is closed rather than open).
- [x] **T40 Diagnostics hygiene (A7.11).** `StoreError::Other` text is
  reflected verbatim into OperationOutcomes (`:174`). Client-visible
  diagnostics become path + rule id; detail goes to the log with the
  incident id. *Done:* `StoreError` now distinguishes `Unsupported`
  (client-safe: names the caller's own parameter) from `Other` (internal:
  logged behind an incident id, never returned). Search-compilation errors
  are `Unsupported`, so the honest "this parameter is not supported"
  messages survive.

### P1 — missing guarantees

- [x] **T41 Access log (PR12.5, PR12.6).** `fhir_sqlite_access_log` per schema;
  every read, vread, history, and search appends a record naming the actor,
  the subject, and (for search) how many resources were disclosed.
  `--audit-mode sync|async|off`, bounded queue, fail closed on saturation,
  per-version counters, `--allow-unaudited` to opt out loudly.

  *On the default.* The spec draft had `async` as the default for
  throughput; it ships as `sync`. The failure `sync` prevents is the one
  that cannot be repaired afterwards — a disclosure with no record is
  indistinguishable, later, from a disclosure that never happened — and a
  fast default would make every deployment silently accept a loss window it
  never chose. `async` announces that window at startup.

  *Fail closed became real, not aspirational.* Recording used to be
  best-effort: a failed insert was logged and the read served anyway. Now
  `audit_read` returns a refusal that the four read paths propagate as 503,
  so in every mode a disclosure that cannot be recorded is not made.

  *Two bugs worth remembering, both in code that compiled and looked right.*
  `Sender::closed()` waits for the *receiver* to drop, so using it to
  shut the writer down deadlocks against the task it is waiting for; the
  sender has to actually be dropped, which means holding it somewhere
  takeable. And a queue-depth gauge maintained by read-then-subtract can
  underflow under concurrency, so depth is derived from the monotonic
  counters instead — an audit queue reporting a nonsense depth is worse than
  one reporting none. `tests/audit_async.rs` pins both, the drain test under
  a timeout so a deadlock fails instead of hanging the suite.

- [x] **T42 Tamper-evident history (M3.16, M3.17).** `prev_hash`/`row_hash`
  chain per resource id, `BEFORE UPDATE OR DELETE` reject triggers,
  `fhir-sqlite verify-audit` walking every chain, and the documented `REVOKE`
  grants. *Done:* all of it. The chain is computed **in SQL** so it covers
  the database's own `now()` and cannot race the read of the previous hash,
  and it hashes `resource::jsonb::text` — the stored normalized form — since
  hashing the submitted text would fail verification against what was
  actually saved. *Accept met:* the `audit` suite tampers with a history row
  behind the application's back and the chain names exactly that version.
- [x] **T59 Tamper evidence that survives the database (M3.16a-c).** Two
  chains in two design families, a keyed tag, and an external witness.

  *The correction that shaped it.* T42 computed the chain in SQL, for two
  real reasons — it covered the database's own `now()`, and it could not race
  the read of the previous digest. Both survive without it: the timestamp is
  read in the same transaction and written back verbatim, and the write path
  already holds a `SELECT … FOR UPDATE` row lock before appending history.
  What SQL-side computation cost was the only fix that matters. The digests
  are unkeyed over a published pre-image, so anyone who can write the rows
  can write matching digests; the answer is a MAC, and a MAC can only be
  introduced where the database is not. A key stored where the attacker
  already has write access protects nothing.

  *Two families, not two digests.* SHA-256 is Merkle–Damgård, SHA3-256 is a
  sponge. MD5 and SHA-1 both fell to one line of cryptanalysis and both were
  Merkle–Damgård; two digests from one family would have bought far less than
  their bit counts suggest. BLAKE3 would add a third (ARX tree) and is
  deliberately absent: it is in neither pgcrypto nor OpenSSL, and it is not
  FIPS-approved, so it could not be the control of record where that matters.

  *Three layers, three jobs.* The digests detect careless modification and
  let an outside auditor check the chain unaided. The `HMAC-SHA-256` tag
  resists forgery. Neither notices a row that is simply **gone** — a chain
  missing its last version verifies perfectly, because nothing left behind
  refers to what was removed — so `chain-witness` digests every head, and
  checkpoints go out on an `audit_checkpoint` log target at startup, after
  erasure, and on an interval.

  *Found by looking, not by testing.* The erasure tombstone terminated only
  the SHA-256 chain: 11 rows, 11 SHA-256 digests, 10 SHA-3. The suite was
  green. And the pre-image hashed a timestamp rendered in the session's
  TimeZone, so a verifier in another zone would have reported every row
  broken — both sides now render UTC explicitly.

  *A defect this work introduced.* The rotation test set `FHIR_SQLITE_CHAIN_KEY`
  with `std::env::set_var`, which is process-global and races concurrent
  readers — that is why it is unsafe — and cargo runs a binary's tests in
  parallel. The symptom appeared in an unrelated test binary.
  `Store::with_chain_keys` replaces it, which is better design anyway.

  *Remaining:* nothing for the control itself. Keys are read from the
  environment; a deployment wanting a secrets manager or an HSM needs
  `with_chain_keys` wired to it, which is API that exists but has no CLI
  surface.
- [x] **T43 Worldwide string search (P6.6).** Each string search target
  column gets a `_norm` companion holding the folded value, computed in Rust
  at write time; prefix search is a range predicate against it. Closes T15's
  unindexed-prefix note. No PostgreSQL extension required.

  *Two traps, both found by measurement rather than reasoning.*

  First: the cheap-looking version — an expression index on `fhir_sqlite_norm(col)`
  plus a predicate `fhir_sqlite_norm(col) LIKE fhir_sqlite_norm($1) || '%'` — **does not
  use the index**. PostgreSQL's btree prefix optimization for `LIKE` needs a
  *constant* pattern, and `fhir_sqlite_norm($1)` is only stable, not constant.
  Folding the term in Rust instead keeps the pattern constant, but then the
  fold exists twice — once in Rust, once in SQL — and the two must agree for
  every codepoint, forever, or a patient is simply not found. So the fold
  lives in Rust only (`fhir-sqlite-map::fold`), and the database stores its output.

  Second, and the reason the first fix is not enough: even against a plain
  column with a `text_pattern_ops` index, `col LIKE $1` **still** scans. The
  prefix is extracted at plan time, so a custom plan (which substitutes
  parameter values) uses the index and a generic plan does not — meaning the
  query is fast in every hand-run `EXPLAIN` with a literal, fast for its first
  few executions, and then quietly degrades once PostgreSQL switches to the
  generic plan. The fix is to stop asking the planner to analyze a pattern at
  all: compute the upper bound in Rust and emit `col >= $1 AND col < $2`, an
  ordinary index condition. `tests/search_semantics.rs::string_prefix_search_uses_its_index`
  pins this with `plan_cache_mode = force_generic_plan`, which is the only
  setting under which the old form visibly fails.

  Ordering is by `COLLATE "C"` — set on the column itself — because the range
  bound is only sound under codepoint order. Under a linguistic collation a
  string beginning with the prefix can sort past the computed upper bound.

  `:exact` deliberately compares the *stored* column: it is defined as the
  literal string, so folding must not leak into it.

  Migration: `upgrade` adds the columns, then backfills them
  (`Store::backfill_norm`) before returning. Without the backfill an upgraded
  install would answer string searches from NULL columns and silently return
  fewer results — the one failure mode a clinical search must not have. The
  backfill folds distinct *values* rather than rows, in batches, and is
  resumable, since each pass only looks at rows still NULL.

- [x] **T44 Edge resource limits (O10.8, P6.7).** Per-request timeout,
  concurrency limit, in-flight cap, configurable pool size; batched result
  materialization instead of one `get` per id; `_include`/`_revinclude` caps
  that warn in the bundle when they truncate. Every ceiling is now a flag —
  `--request-timeout`, `--max-concurrent`, `--max-body-mb`, `--max-count`,
  `--max-included`, `--pool-size` — rather than a constant compiled into the
  binary, because the right value depends on the deployment and the previous
  answer was "rebuild it".

  `--pool-size` takes precedence over `FHIR_SQLITE_POOL_SIZE`: a flag the operator
  typed should not be silently overridden by an environment variable they
  inherited.

  `tests/edge_limits.rs` asserts a configured ceiling is *enforced*, not just
  parsed. A limit that reaches the config struct but never reaches the code
  that should apply it is worse than no limit, because the operator believes
  a ceiling exists.
- [x] **T45 Admin plane separation (O10.9).** `--admin-bind` for
  `/metrics`, `/health`, `/ready`; latency as a histogram so p99 is
  answerable. *Done:* `--admin-bind` serves /health, /ready and /metrics on
  their own address against the same counters, on their own task so they
  answer while the API is shedding load, and latency is a Prometheus
  histogram (`fhir_sqlite_request_latency_seconds`, default 1ms–10s buckets) so
  `histogram_quantile` answers p99. A running total plus a count gives only
  the mean, and the mean cannot tell "every request took 40ms" from "99%
  took 5ms and 1% took 4 seconds" — which is the case anyone is paged for.
- [x] **T46 Honest CapabilityStatement (A7.12).** Declare
  `conditionalCreate`/`Update`/`Delete`, `searchInclude`, `searchRevInclude`,
  `readHistory`, `versioning`, and the `security` block; drop interactions
  that are not implemented. *Done:* per-resource `versioning`, `readHistory`,
  `updateCreate`, `conditionalCreate`/`Update`/`Delete`, `referencePolicy`,
  `searchInclude`/`RevInclude`; system-level `transaction` and `batch`; and a
  `security.description` stating plainly that fhir-sqlite verifies no identity.
- [x] **T47 Supply-chain evidence (O10.10).** `cargo deny` + `cargo audit`
  in CI, CycloneDX SBOM per release, checksums for published artifacts.
  *Done:* a `supply-chain` CI job (cargo-deny + CycloneDX, SBOM uploaded as
  an artifact) and a `deny.toml` policy — permissive licences only, wildcards
  denied, unknown registries denied. *Remaining:* per-binary checksums in the
  release workflow.
- [x] **T48 Concurrency, redaction, and audit test suites (T11.6–T11.8).**
  The adversarial tests that keep T31–T42 honest. *Done:* `concurrency.rs`
  covers torn reads, racing conditional creates, and racing `If-Match`
  updates (T11.6); `redaction.rs` covers T11.7; `audit.rs` covers T11.8 —
  the audit envelope, the disclosure record, chain verification, and the
  database refusing to let history be rewritten. `audit_async.rs` adds the
  async path: batching, the shutdown drain, and saturation refusing rather
  than dropping.
- [x] **T49 Erasure (M3.18).** `fhir-sqlite purge` with tombstone rows,
  `--allow-erasure`, and a test that `verify-audit` reports a purge as a
  recorded hole rather than a chain break. *Done:* `Store::purge` and
  `fhir-sqlite purge`, the tombstone carrying who/when/why plus the terminated
  chain hash, and the append-only trigger relaxed to permit `DELETE` only
  inside a transaction that sets `fhir_sqlite.erasure`. The book states the two
  limits plainly: backups and replicas are outside this, and the guard stops
  accidents rather than the application itself.
- [x] **T50 Trust-boundary chapter (PR12.8).** One table in the book: what
  fhir-sqlite guarantees, what the perimeter must provide, what neither does yet.
  *Done:* `book/src/trust-boundary.md`, including a worked `serve`
  invocation where every flag is explained, the `REVOKE` grants, and an
  honest statement of what the hash chain does *not* prove (an attacker who
  can recompute it — hence: ship `row_hash` off-box).

### P2 — reach

- [ ] **T51 Type- and system-level `_history`.** Required for CDC,
  replication, and incremental export; currently instance-level only.
- [ ] **T52 Bulk Data `$export`.** Async kickoff, NDJSON output, status
  polling — ONC/HTI expects it, and the earlier fhir-sqlite had the client half.
- [ ] **T53 `X-Provenance` and `AuditEvent` projection (PR12.7).** Store
  submitted Provenance; expose the access log as queryable `AuditEvent`.
- [ ] **T54 Inferno / Touchstone conformance run.** External validation of
  §7 and A7.12 against the published test kits.
- [ ] **T55 `_summary` and `_elements`.** Common in production clients;
  currently 501.
- [ ] **T56 PATCH.** JSON Patch and FHIRPath Patch, declared in the
  CapabilityStatement.
- [ ] **T57 Restore and failover drills.** A documented, tested PITR
  restore and a `fhir-sqlite fsck` that checks orphan rows, ordinal gaps, and
  history/current agreement.
- [x] **T58 CI/CD on GitHub and Codeberg.** Parallel pipelines on both
  forges (`.github/workflows/`, `.woodpecker/`): fmt, clippy, unit tests,
  book, live-PostgreSQL suite, advisories/licenses/SBOM. Tag builds
  artifacts with checksums and a CycloneDX SBOM attached to the release;
  crates.io publishing is manual and confirmation-gated, since a published
  version can be yanked but never replaced. Also verifies the declared MSRV,
  which until now was a claim nothing checked. See `doc/ci.md`.

## M14 — SQLite port

Tracks `spec/14-sqlite-dialect.md`. The repo began as a rename of the
PostgreSQL original, so every task here is a *departure* from an inherited
PostgreSQL implementation, not new ground.

- [x] **T60 Rename repair.** The initial `fhirpg` → `fhir-sqlite`
  substitution rewrote Rust paths, SQL identifiers, the GUC prefix, and env
  vars into forms that are not legal in their respective languages, and the
  workspace did not resolve. Redone with the correct spelling per context
  (crate paths, SQL identifiers, `FHIR_SQLITE_*` env vars,
  `X-Fhir-Sqlite-*` headers, asset filenames). *Accept:* `cargo build`,
  `clippy`, `fmt`, and the DB-free suite all pass.
- [x] **T61 Dialect annex.** `spec/14-sqlite-dialect.md`, M14.1–M14.29.
  Records what changes, what does not, and — where a PostgreSQL guarantee
  cannot be reproduced — says so rather than quietly dropping it. Status:
  proposed, not ratified.
- [x] **T62 Canonical JSON (M14.15).** The hash chain committed to
  PostgreSQL's `jsonb` rendering, which no other engine reproduces.
  Canonicalization moved into Rust as `canon::canonicalize` in the map crate:
  keys sorted by UTF-8 bytes, number lexemes verbatim, minimal escaping,
  infallible. **RFC 8785 was evaluated and rejected** — it serializes numbers
  as IEEE-754 doubles, which would destroy the decimal precision M3.6
  requires. *Accept:* 13 unit tests, including that `1.50` and `1.5` do not
  collide.
- [x] **T63 DDL for SQLite (M14.8–M14.13).** `ddl.rs` re-emitted:
  `ords smallint[]` → `TEXT` (the database only ever stores it and enforces
  PK uniqueness — nothing orders, subscripts, or unnests it); the type map
  per M14.10; unqualified `REFERENCES` because SQLite foreign keys cannot
  cross databases; `CREATE INDEX "s"."ix" ON "t"` rather than PostgreSQL's
  shape; the shared plpgsql append-only guard replaced by per-table
  `BEFORE UPDATE`/`BEFORE DELETE` triggers with `RAISE(ABORT)`; the erasure
  GUC replaced by an in-schema flag table (a trigger body may not reference
  another database, so `temp.` cannot serve); the `_norm` function dropped
  entirely, since it never had a caller. *Accept:* all three real schemas
  install in SQLite 3.51 — r5 7,360 tables from 9,490 statements, r4 5,677,
  r3 3,832 — and the append-only triggers are proven to refuse an UPDATE and
  a DELETE, and to permit a flagged erasure.

- [~] **T64 Store layer — in progress.** Being ported as a *parallel* module
  (`store/src/sqlite.rs`) rather than a driver swap: converting all 57 call sites
  at once means nothing compiles, and so nothing is testable, until the whole
  2,717-line file is done. The inherited PostgreSQL module stays until this one
  reaches parity, then is deleted.
  *Landed:* `rusqlite` with `bundled` (a pinned engine, for the same reason
  doc/containers.md pins the CLI); `SqliteStore::open` with the four pragmas that
  are load-bearing rather than tuning — `foreign_keys=ON` above all, since the
  child tables' `ON DELETE CASCADE` is how a rewrite clears its old rows;
  `init` installing the full R5 schema (7,000+ tables) in one transaction;
  `drop_schema`; `put` and `get` round-tripping a resource through shred and
  reconstruct; `history_canon`.
  *Accept:* 7 tests, needing no server and no environment variables, so unlike
  the inherited suites they always run. They prove the schema installs, that a
  failed `init` rolls back completely (M14.16), that an orphan child row is
  refused, that WAL persists, that a rewrite removes stale child rows rather than
  shadowing them, and that decimal precision survives.
  *Also landed:* `history`, `vread`, `delete` (tombstone with `op='D'`, base row
  removed, history retained — the history table deliberately has no foreign key
  to the base table, so a deletion cannot erase its own evidence), and
  `verify_audit` recomputing both hash chains.
  *Accept:* 11 tests. The strongest is `verify_audit_detects_a_tampered_history_row`,
  which drops the append-only UPDATE trigger — the deliberate act M3.17 exists to
  make visible — edits a stored resource behind the store's back, and asserts
  **both** chains flag it. That is also what proves the canonical bytes are what
  the hash actually covers, rather than merely being stored next to it.
  *Fixed while writing it:* `put` recorded every version with `op='U'`. 'C' and
  'U' are distinct in the history column and the op is part of the hashed
  preimage, so this could not have been corrected after the fact.
  *Also landed:* keyed-MAC verification (M3.16b/M3.16d). `verify_audit` now takes
  a `KeyRing` and checks the `row_mac`, which until then was written but never
  re-verified — a keyed deployment was getting strictly *less* checking than an
  unkeyed one, because the tag was decorative. Counter-signatures are honoured
  where the original tag cannot be checked, but never where it disagrees:
  otherwise re-signing would be a way to bless forged history. Rows signed under
  a key this process does not hold warn rather than break, because "I cannot
  check this" and "this was altered" are different claims and conflating them
  teaches operators to ignore the report.
  *Accept:* 14 tests. The keyed-tamper test was mutation-checked — disabling the
  mismatch report makes it fail, so it is load-bearing rather than incidentally
  green.
  *Also landed:* search. `store/src/sqlite_search.rs` is forked from the
  inherited PostgreSQL builder rather than shared with it — the two ports are
  independent by design, and parameterising one builder at all ~18 points where
  the dialects differ would be more coupling than either engine gains. `?n`
  placeholders, no casts, `LIKE` against the Rust-folded column, an ISO sentinel
  for `infinity`, and `datetime(x,'+1 second')` for interval arithmetic. Dates
  need no cast at all: fixed-width UTC text already sorts chronologically, which
  is the whole reason for fixing the width (M14.12).
  *The one real semantic hazard:* numeric columns hold their exact lexical form
  because M3.6 demands it, and compared as text `"9" > "10"`. Numbers and
  quantities therefore compare via `CAST(… AS REAL)`. That gives up the index —
  a range scan becomes a table scan — and it is the right trade until the derived
  sort columns of M14.11 exist. Verified by mutation: without the cast, `gt9`
  returns nothing at all.
  *Accept:* 18 tests, including that attacker-supplied values reach the database
  as parameters and never as SQL text (the invariant the PostgreSQL fuzz target
  protects).
  *Fixed while testing:* three search tests shared one scratch directory and
  deleted each other's database under cargo's parallel runner, surfacing as three
  unrelated-looking assertion failures. `scratch()` now panics on a duplicate
  name so the collision is caught by name rather than by symptom.
  *Also landed:* the access log (PR12.5) and `purge` (GDPR Art. 17, M3.18).
  Erasure removes the history rows and leaves a tombstone recording who, when,
  why, and the hash the chain ended on — a hole you can see, rather than
  something indistinguishable from a chain that never happened. The append-only
  trigger permits the delete only while the erasure flag row exists, and because
  that flag is an ordinary row inserted and removed inside the transaction, an
  aborted erasure cannot leave the escape hatch open the way PostgreSQL's session
  GUC could.
  *Two bugs found by writing the tests, both in the verifier:*
  (1) `verify_audit` checked the tombstone's chain link, which by construction
  points at rows that were deliberately deleted — every lawful erasure would have
  been reported as tampering, the loudest possible false positive on the one
  operation an operator most needs to trust. Tombstones are now excluded from
  link checking.
  (2) The tombstone's keyed MAC is computed over the erased chain's tip, but was
  being verified against the walk's prior, which is empty once those rows are
  gone. The MAC is now verified against the row's *stored* `prev_hash`; the link
  check independently catches a `prev_hash` that disagrees with the walk. This is
  a deliberate divergence — the PostgreSQL original skips tombstones before the
  MAC check and so cannot verify them at all — and it is backed by a test that
  forges a tombstone's actor and asserts it is caught.
  *Also landed:* the CLI is wired to `SqliteStore`, so `fhir-sqlite` is a
  working command-line FHIR store on a file. `--dsn` is now a database path
  (default `fhir.sqlite`), with each FHIR version in its own file beside it.
  Verified by driving the built binary, not by unit tests: `init` installs 9,490
  statements, `load` ingests, `get` returns the resource with `9.60` intact
  through the whole file → SQLite → JSON path, `search status=final` finds it,
  `verify-audit` names the layers that actually ran and warns when unkeyed,
  `purge` refuses without `--allow-erasure` and leaves a tombstone with it, and
  the chain still verifies afterwards.
  *Commands with no SQLite store behind them yet — `export`, `chain-resign`,
  `chain-witness`, `init --upgrade` — now fail saying so.* They previously
  compiled against the PostgreSQL store and would have tried to reach a server
  that is not there; an honest refusal naming the task beats a confusing
  connection error.
  *Also landed:* most of the surface the HTTP layer needs. Measured rather than
  assumed — the server calls fourteen distinct `Store` methods, not the ~53 the
  type exposes — so `SqliteStore` gained `map`, `ping`, `get_versioned`,
  `get_all`, `status`, `put_audited`, `delete_audited`, and `log_access_batch`
  with matching names and shapes, so the server can be pointed at it by changing
  a type rather than being rewritten.
  Notable among these: `status` separates live / deleted / unknown, which are
  200, 410 Gone, and 404 on the wire — collapsing the last two would tell a
  caller that a record it once held never existed. And `put_audited` implements
  `If-Match` optimistic concurrency inside the same `BEGIN IMMEDIATE` as the
  write, because a version check in a separate transaction is a race dressed up
  as a guarantee.
  *Accept:* 25 tests, including that a stale write is refused with the expected
  and found versions, that naming the current version lets it through, and that
  `get_all` keeps absence in place rather than returning a shorter list.
  *Also landed:* `refs_of` for `_include` resolution (`= ANY($1)` has no SQLite
  equivalent, so the ids become a generated `IN (?,?,…)` list, chunked under the
  bound-parameter limit).
  *Deliberately NOT landed: `transact_audited`.* It returns an explicit
  `Unsupported` error. A working compensating version was written and then
  removed: applying each op through the ordinary write path and undoing earlier
  ones on failure is not atomicity, and a FHIR `transaction` Bundle is atomic by
  definition. Readers between ops observe a half-applied bundle, and a process
  that dies mid-unwind leaves the partial state permanently. Shipping that under
  the name `transact` would claim a guarantee the code does not provide, in the
  subsystem whose entire purpose is being trustworthy. Doing it properly needs
  `put` and `delete` split so their bodies can run inside a caller-supplied
  `BEGIN IMMEDIATE`.

- [x] **T64b HTTP server wired to the SQLite store.** All three blockers
  cleared, and `fhir-sqlite serve` now answers requests from a SQLite file.
  Verified by starting the binary and curling it, not by unit tests:
  `GET /r5/Patient/p1` returns the resource with `meta.versionId`, and a
  `family=` search returns a `searchset` Bundle.
  1. **Key ring moved onto the store** (see above).
  2. **`vread` returns a `HistEntry`**, not a bare resource: a deleted version
     has no content, and a caller must tell "version 3 was a deletion" (410)
     from "version 3 does not exist" (404).
  3. **Conditional create/delete implemented.** The race they must survive is
     two callers submitting the same `If-None-Exist` at once — both search, both
     find nothing, both create. PostgreSQL needs an advisory lock keyed on the
     criteria; here a process-level `write_gate` is taken *before* the search, so
     the search-then-create sequence is indivisible. Searching outside the lock
     would be the same race with extra steps.
  Also added along the way: `search`/`search_full`/`search_page` in the shapes
  the HTTP layer expects, `access_log_for`, and `emit_checkpoint` — the last
  deliberately a log line rather than a table row, since a checkpoint's value
  comes from living somewhere the database cannot rewrite.

- [ ] **T64c Remaining store gaps.** `transact_audited` (still an explicit
  `Unsupported`; needs one transaction across all ops), `resign_history`,
  `chain_witness`, `export`, and `init --upgrade`. Each fails saying so rather
  than pretending.

- [x] **Accent folding does not cover Nordic letters — fixed; see T90 under
  Remaining work.** Original finding, kept for the reasoning: Found by curling the
  running server: `fold("Ærø")` is `"ærø"`, so a search for `aero` misses it,
  while `Muñoz` → `munoz` and `Müller` → `muller` work. The cause is that `ñ`
  and `ü` decompose into a base letter plus a combining mark under NFD, which
  the fold strips, whereas `æ`, `ø`, and `å` are distinct letters with nothing
  to strip. PostgreSQL's `unaccent` would map them; `fold::fold` does not.
  **This is inherited, not introduced** — `fold.rs` is byte-identical across all
  four repos (`9f69812b4ede`), so fhir-postgresql has it too. It matters because
  the function's own doc comment cites Ærø as the motivating example: "a patient
  not found rather than a cosmetic difference". Needs a decision on scope
  (Nordic only, or full `unaccent`-equivalent) and it changes stored `_norm`
  values, so it needs the backfill path.

- [x] **T71 Reduced to an embeddable library.** Scope correction: this project
  is a library to embed, not an HTTP server and not a command-line tool.
  Removed the server crate (and Axum, tower, axum-server with it) and the CLI
  binary crate; the workspace is now `-map`, `-gen`, `-store`.
  The generated relmaps lived inside the CLI crate and had to outlive it, so
  `crates/fhir-sqlite/assets/` moved to a top-level `assets/`, with test paths
  repointed and the published checksums re-verified against the moved files.
  Spec sections 7 (REST API) and 8 (CLI) are deleted rather than left standing:
  the spec's own index says code and spec must be reconciled rather than allowed
  to drift, and normative text for features that no longer exist is exactly that
  drift. The numbering keeps its gap rather than being renumbered, so requirement
  ids like `M9.2` still mean what they meant. The book's REST chapter and the CI
  steps naming the removed crates are gone too.
  *Consequence worth stating:* the HTTP-facing work from earlier in this
  milestone — wiring `serve`, `AuditSink`, the REST handlers — is gone with the
  crate. What survives is the part that was always library work: the store, its
  audit chain, search, and the conditional operations.

- [x] **T72 PostgreSQL store removed from the SQLite library.** With the CLI and
  HTTP server gone, the inherited `Store` was dead weight that still dragged
  `tokio-postgres`, `deadpool-postgres`, `tokio-postgres-rustls`, `rustls`,
  `rustls-native-certs`, and `futures-util` into a library whose job is to open
  a local file. All six are gone; `fhir-sqlite-store` now depends on `rusqlite`,
  `serde_json`, `tokio`, `thiserror`, `tracing`, and the crypto crates the hash
  chain needs. `lib.rs` went from 2,719 lines to ~250: it is now the shared
  vocabulary (`StoreError`, `Audit`, `Got`, `HistEntry`, `ResourceStatus`,
  `CondCreate`/`CondDelete`, `PurgeReport`, `ChainBreak`, …) and nothing else.
  The PostgreSQL search builder went with it, and the fuzz target — which
  asserts no attacker-supplied value reaches the SQL text — now runs against
  `sqlite_search`.
  Also removed eight inherited test files that exercised the deleted store. They
  self-skipped without a PostgreSQL database, so the drop from 99 to 83 passing
  tests is 16 stubs that never ran, not lost coverage.
  *Accept:* a scratch crate outside the workspace, depending only on the two
  public crates by path, installs the R5 schema (9,490 statements), writes a
  resource, reads it back with `2.50` intact, searches by token, and verifies
  the audit chain. That is the embeddability claim, tested rather than asserted.

## Remaining work (as of the port commit)

Native SQLite store, complete enough to embed. No PostgreSQL dependencies.

- [ ] **`transact_audited` (T64c)** returns an explicit `Unsupported`. A FHIR
  transaction Bundle is atomic by definition, and the compensating version that
  was written and deleted is not: readers between ops observe a half-applied
  bundle, and a process dying mid-unwind leaves partial state permanently. Doing
  it properly needs `put` and `delete` split so their bodies run inside a
  caller-supplied `BEGIN IMMEDIATE`.
- [ ] **`resign_history`, `chain_witness`, `export`, `init --upgrade`** are
  unimplemented; each fails saying so rather than pretending.
- [ ] **T65 Decimal sort columns (M14.11).** `ColTy::Numeric` needs a derived
  `<name>_sort` companion in the generated map, following the pattern `date` and
  `dateTime` already use. Until then numeric range search works via
  `CAST(… AS REAL)`, which is correct but gives up the index.
- [ ] **T67 Amend spec sections 1–13.** They still describe PostgreSQL
  throughout, including the `ords[1]` query idiom the book teaches, which a TEXT
  column cannot support.

### Cross-cutting, all repos

- [ ] **Git remotes are wrong.** Every database repo still has `origin` =
  `git@github.com:fhirpg/fhirpg.git`, correct for at most one of them. Pushing
  any `port/*` branch as-is would send that port to the upstream project. Set
  each remote before pushing. Nothing has been pushed.
- [ ] **Shared history.** All six database repos descend from `688641a` of the
  original `fhirpg` project. Whether five separate products should keep that
  history, be squashed, or be re-rooted is a decision to make deliberately
  rather than discover after a push.
- [x] **T90 Accent folding misses Nordic letters — fixed.** `fold` now expands
  the letters NFD cannot reach, following PostgreSQL's `unaccent` rules so a
  folded value means the same thing whichever engine stores it: `æ`→`ae`,
  `œ`→`oe`, `ø`→`o`, `đ`/`ð`→`d`, `ł`→`l`, `ß`→`ss`, `þ`→`th`, and others.
  `fold("Ærø")` is now `"aero"`. Verified by mutation: with the expansion
  disabled the test fails with `left: "ærø", right: "aero"`.
  Two of my own test expectations were wrong before the code was: Greek `ό` and
  Cyrillic `й` both carry combining marks and therefore *do* fold, to `ο` and
  `и`. That is accent-insensitive search working across scripts, and the tests
  now assert it. What must never happen is transliteration — romanising
  Cyrillic would make "the same string" a policy rather than a property of the
  text — and there is a test for that too.
- [x] **T90a Backfill the `_norm` columns.** The fix changes stored folded
  values, so any database written before it holds stale ones and will miss the
  searches this repaired. **Deploying the new fold against an existing database
  without backfilling is worse than not fixing it**, because searches would then
  match neither spelling.
  *Done here:* `SqliteStore::upgrade` and `SqliteStore::backfill_norm`, with
  `init` now recording the map asset an upgrade diffs against (`M14.30`). The
  backfill folds distinct values in batches of 1,000, one transaction per batch,
  selecting only rows still NULL — so it is resumable, which matters because a
  SQLite writer holds the single write lock for its transaction's length
  (`M14.34`). *Accept:* 8 tests in `tests/upgrade.rs`, none needing a server;
  mutation-verified — skipping the backfill makes the seeded patient unfindable
  by their own name.
  *Still open on MySQL, MariaDB, MSSQL and Oracle* (audit **F-15**), and on a
  SQLite database installed before `init` recorded the asset, which has nothing
  to diff and must still be reloaded.
