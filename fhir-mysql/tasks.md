# fhir-mysql tasks

> **Parts of this file are untrue of this port (audit [F-27](../spec/databases/audit.md#f-27)).**
> The `M4 — REST server` milestone, `T8 CLI v1`, and `T23 Multi-version serve`
> are checked off, and none of that code exists in any port: there is no
> `fhir-*-server` crate, no `serve` binary, and no REST test suite anywhere in
> this repository.
>
> Do not read a `[x]` here as evidence. The
> [conformance matrix](../spec/databases/conformance-matrix.md) is the status document to
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
  (`fhir-mysql-map`, `fhir-mysql-gen`, `fhir-mysql-store`, `fhir-mysql` — the server crate
  arrives with M4), CI (fmt, clippy, test, live-PG job).
  *Done:* `.github/workflows/ci.yml`; tests self-skip without inputs.
- [x] **T2 Spec-package ingestion.** profiles-resources.json +
  profiles-types.json parsed directly (simpler than reusing the fhir
  crate's parser; that crate still backs `--validate` later) into element
  trees with cardinality, types, choice and contentReference info — for all
  three versions, not just R5. SearchParameters ingestion moves to M3.
- [x] **T3 Relational map format.** `fhir-mysql-map::model`: node arena (cycles
  via indexes), tables, typed columns, choice variants, reference splits,
  extension/spill channels, 63-byte registry with deterministic
  abbreviation + hash fallback. Assets: `assets/fhir-mysql-relmap-{r3,r4,r5}
  .json.gz` + CHECKSUMS.txt.
- [x] **T4 DDL generator + scale spike.** Full R5 = 7,355 tables installs
  via a direct statement-by-statement install. **Not** staged-schema +
  rename: MySQL has no transactional DDL, so the install applies statements directly and is not atomic; the staged-schema-then-rename dance PostgreSQL uses has no equivalent. The 9.5 s figure below was PostgreSQL's.
  (Corrected, **F-27** class 3.) Numbers in doc/benchmarks.md;
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
- [x] **T7 Store layer: init/load/read.** `mysql_async` with a connection pool;
  transactional put with history append; multi-table reads; chunked
  multi-row inserts; values bound as text so decimal scale and partial dates
  survive (`M3.6`). **Not** tokio-postgres, and there are no `($n::text)`
  casts — that was PostgreSQL's wire protocol (**F-27** class 3).
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
  (`FHIR_MYSQL_PROPTEST_CASES`; default 500 locally).
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
- [x] **T12 fhir_mysql_meta + idempotent init.** Staged-schema install +
  atomic rename, checksum recorded; re-init no-ops on matching checksum
  and refuses a mismatch. Chunked `drop_schema` + `fhir-mysql drop --yes`.
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
  unsupported-parameter errors; `fhir-mysql search` CLI; `_sort` (base-table
  params + _id/_lastUpdated, honest errors otherwise) and
  `_total=accurate`. *Accept mostly met:* search_semantics + rest suites
  green against live PG. Single-hop `_include` (via compiled reference
  targets) and `_revinclude` (via the search machinery) with
  search.mode=include entries and dangling-reference tolerance.
  *Remaining:* chained `reference.`, cursor paging, lenient handling.
- [x] **T15 Index emission + explain audit.** One index per distinct
  search-target column set emitted with the DDL (R5: 1,813 indexes; full
  init 5.8 s). EXPLAIN audit in tests/bench.rs: token/reference/date
  searches all plan index scans at 100k resources; the test fails when a
  full scan appears. *Note:* prefix string search uses `LIKE` against the folded column with a binary collation; MySQL has no `ILIKE` —
  the `ILIKE`/`text_pattern_ops` note here was PostgreSQL's (**F-27**
  class 3).

## M4 — REST server: **moved to `fhir-loco`**

The REST server exists. It is **not** in this port and never will be: it is a
separate crate, [`fhir-loco`](../fhir-loco/), built on Loco.rs, Axum, Tokio and
Hyper, and it currently mounts over `fhir-sqlite`.

That resolves what **F-27** class 1 recorded as undecided. These milestones were
not "planned and unfinished" — they were **misattributed**, inherited from the
ancestor project where the server lived inside the port. Deleting them is
therefore correct, and unticking them would have been wrong: it would have
asserted that this port is going to grow a server, which it is not
(`C0.17`, `C0.18`).

What `fhir-loco` serves today:

| Route | Methods |
| --- | --- |
| `/{version}/metadata` | `GET` |
| `/{version}/{rtype}` | `GET` (search), `POST` (create) |
| `/{version}/{rtype}/{id}` | `GET`, `PUT`, `DELETE` |
| `/{version}/{rtype}/{id}/_history` | `GET` |
| `/{version}/{rtype}/{id}/_history/{vid}` | `GET` |

Authentication is PASETO v4.public, and there is no unauthenticated mode.

**What this port owes the server** is the store API it calls, which is
everything in M1–M3 and M6–M7 below. If you came here looking for endpoint
work, it is in `fhir-loco`.

## M5 — R4 and R3

> **Ledger drift, needs a human call.** T21 and T22 are unchecked, but T7
> records a full-corpus live round trip of 7,396/7,396 across r3/r4/r5, the
> README claims all three corpora round-trip losslessly, and `fhir-mysql serve`
> mounts all three (T23, checked). Either the boxes are stale or the README
> overstates. Since the spec is meant to be the source of truth, reconcile
> before release rather than after.

- [ ] **T21 R4 artifacts.** Run generator on 4.0.1; fix spec-parsing deltas.
  *Accept:* full R4 examples corpus round-trips live; REST suite green on
  `/r4`.
- [ ] **T22 R3 artifacts.** Same for 3.0.2.
  *Accept:* full R3 examples corpus round-trips live; REST suite green on
  `/r3`.
- [x] **T23 Multi-version serve.** *Done, in [`fhir-loco`](../fhir-loco/),
  not here.* It loads `r3`, `r4` and `r5` maps at startup and routes on
  `/{version}/…`; this port supplies the store it reads. Not `fhir-mysql
  serve`, which does not exist (`C0.18`).

## M6 — Production hardening

- [x] **T-validate (V9.2).** `fhir-mysql load --validate` deserializes each
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
- [x] **T-graceful.** *Belongs to [`fhir-loco`](../fhir-loco/).* Shutdown
  is the server's concern, and Loco owns the signal handling; a library
  has no process to shut down.

- [~] **T24 Observability.** Done: /health, /ready, /metrics (Prometheus
  text: request/response-class/latency counters), X-Request-Id
  (propagated or generated) with per-request tracing that logs
  method/path/status only — never resource content. *Remaining:* JSON log
  format wiring in the CLI, an automated redaction test, latency
  histogram buckets.
- [~] **T25 Pool + timeout hardening.** Done: server-side
  statement_timeout (FHIR_MYSQL_STATEMENT_TIMEOUT_MS, default 30 s), pool
  wait timeout 2 s, exhaustion → 503 + Retry-After.
  *Remaining:* an automated saturation test.
- [x] **T26 Migrations + upgrade.** `init` stores the map asset in
  fhir_mysql_meta; `init --upgrade` diffs installed vs current maps and
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
  assets embedded in the binary so `cargo install fhir-mysql` is
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
  (`fhir-mysql-store/src/lib.rs:554`), while `put_in` deletes and re-inserts
  (`:512`). A write landing mid-read reconstructs a resource that never
  existed. Wrap every multi-statement read in one
  `REPEATABLE READ READ ONLY` transaction; same for `export` and search
  materialization.
  *Accept:* a reader loop against a writer loop over 10k iterations never
  observes a torn resource and never errors (T11.6).
- [x] **T32 Encrypted database transport (O10.7).** Done 2026-08-03
  (**F-54**). `ssl::SslMode` in MySQL's own `--ssl-mode` vocabulary, read from
  `FHIR_MYSQL_SSL_MODE`, defaulting to `VERIFY_IDENTITY`; `connect_with`
  applies it. The `rustls-tls` Cargo feature had to be enabled too — the port
  was built with `minimal`, which excludes TLS entirely, so no amount of code
  could have encrypted anything.

  *Accept:* `tests/ssl_live.rs` against live MySQL asserts that
  `VERIFY_IDENTITY` **rejects** the container's self-signed certificate —
  proving verification is not a no-op, which a succeeding connection would not.
  Mutation-verified two ways.

  *Remaining:* a live test against a correctly-certificated server, which needs
  a CA fixture rather than a stock container. `scripts/db.sh` prints
  `FHIR_MYSQL_SSL_MODE=DISABLED` for the loopback dev container, so a
  green local suite is **not** evidence of a verified link.

  The fix is `mysql_async`'s own TLS options plus a verified certificate by
  default — **not** `tokio-postgres-rustls`, and not `sslmode`/`PGSSLROOTCERT`,
  which are libpq names this port does not read. This entry previously claimed
  `SslPolicy`, a rustls connector, `Store::connect_with`, a `serve` startup
  guard and a TLS-only CI job as *done*; none of them exists here (**F-27**
  class 3). It is now unticked, because unlike the REST milestones this one is
  unambiguously planned: `O10.7` requires it. Tracked as **F-54**.

- [x] **T33 Atomic conditional interactions (A7.10).** `If-None-Exist`
  searches then writes (`fhir-mysql-server/src/lib.rs:444`); two concurrent
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
  (`fhir-mysql-server/src/lib.rs:739`). Add `--base-url`, honor forwarded
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
  Bundle entry urls. *Remaining:* the same check inside `fhir-mysql load`.
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

- [x] **T41 Access log (PR12.5, PR12.6).** `fhir_mysql_access_log` per schema;
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
  `fhir-mysql verify-audit` walking every chain, and the documented `REVOKE`
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

  *A defect this work introduced.* The rotation test set `FHIR_MYSQL_CHAIN_KEY`
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

  First: the cheap-looking version — an expression index on `fhir_mysql_norm(col)`
  plus a predicate `fhir_mysql_norm(col) LIKE fhir_mysql_norm($1) || '%'` — **does not
  use the index**. PostgreSQL's btree prefix optimization for `LIKE` needs a
  *constant* pattern, and `fhir_mysql_norm($1)` is only stable, not constant.
  Folding the term in Rust instead keeps the pattern constant, but then the
  fold exists twice — once in Rust, once in SQL — and the two must agree for
  every codepoint, forever, or a patient is simply not found. So the fold
  lives in Rust only (`fhir-mysql-map::fold`), and the database stores its output.

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

  `--pool-size` takes precedence over `FHIR_MYSQL_POOL_SIZE`: a flag the operator
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
  histogram (`fhir_mysql_request_latency_seconds`, default 1ms–10s buckets) so
  `histogram_quantile` answers p99. A running total plus a count gives only
  the mean, and the mean cannot tell "every request took 40ms" from "99%
  took 5ms and 1% took 4 seconds" — which is the case anyone is paged for.
- [x] **T46 Honest CapabilityStatement (A7.12).** *In
  [`fhir-loco`](../fhir-loco/)* — `GET /{version}/metadata`, generated from
  the map this port produces.

  Corrected 2026-08-03: it declared `read`, `vread` and `search-type` while
  the router also served `POST`, `PUT` and `DELETE`, so a
  conformance-driven client would have concluded the server was read-only.
  `A7.12` in the other direction. Now asserted by
  `metadata_declares_every_interaction_the_router_serves`.

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
- [x] **T49 Erasure (M3.18).** `fhir-mysql purge` with tombstone rows,
  `--allow-erasure`, and a test that `verify-audit` reports a purge as a
  recorded hole rather than a chain break. *Done:* `Store::purge` and
  `fhir-mysql purge`, the tombstone carrying who/when/why plus the terminated
  chain hash, and the append-only trigger relaxed to permit `DELETE` only
  inside a transaction that sets `fhir_mysql.erasure`. The book states the two
  limits plainly: backups and replicas are outside this, and the guard stops
  accidents rather than the application itself.
- [x] **T50 Trust-boundary chapter (PR12.8).** One table in the book: what
  fhir-mysql guarantees, what the perimeter must provide, what neither does yet.
  *Done:* `book/src/trust-boundary.md`, including a worked `serve`
  invocation where every flag is explained, the `REVOKE` grants, and an
  honest statement of what the hash chain does *not* prove (an attacker who
  can recompute it — hence: ship `row_hash` off-box).

### P2 — reach

- [ ] **T51 Type- and system-level `_history`.** Required for CDC,
  replication, and incremental export; currently instance-level only.
- [ ] **T52 Bulk Data `$export`.** Async kickoff, NDJSON output, status
  polling — ONC/HTI expects it, and the earlier fhir-mysql had the client half.
- [ ] **T53 `X-Provenance` and `AuditEvent` projection (PR12.7).** Store
  submitted Provenance; expose the access log as queryable `AuditEvent`.
- [ ] **T54 Inferno / Touchstone conformance run.** External validation of
  §7 and A7.12 against the published test kits.
- [ ] **T55 `_summary` and `_elements`.** Common in production clients;
  currently 501.
- [ ] **T56 PATCH.** JSON Patch and FHIRPath Patch, declared in the
  CapabilityStatement.
- [ ] **T57 Restore and failover drills.** A documented, tested PITR
  restore and a `fhir-mysql fsck` that checks orphan rows, ordinal gaps, and
  history/current agreement.
- [x] **T58 CI/CD on GitHub and Codeberg.** Parallel pipelines on both
  forges (`.github/workflows/`, `.woodpecker/`): fmt, clippy, unit tests,
  book, live-PostgreSQL suite, advisories/licenses/SBOM. Tag builds
  artifacts with checksums and a CycloneDX SBOM attached to the release;
  crates.io publishing is manual and confirmation-gated, since a published
  version can be yanked but never replaced. Also verifies the declared MSRV,
  which until now was a claim nothing checked. See `doc/ci.md`.

## M14 — MySQL port

Tracks `spec/14-14-mysql-dialect.md`. The repo began as a rename of the PostgreSQL
original, so every task here is a *departure* from an inherited PostgreSQL
implementation, not new ground.

- [x] **T60 Rename repair.** The initial `fhirpg` → `fhir-mysql` substitution
  rewrote Rust paths, SQL identifiers, the GUC prefix, and env vars into forms
  that are not legal in their respective languages, and the workspace did not
  resolve. Redone with the correct spelling per context.
- [x] **T61 Dialect annex.** `spec/14-14-mysql-dialect.md`. Records what changes,
  what does not, and — where a PostgreSQL guarantee cannot be reproduced —
  says so rather than quietly dropping it. Status: proposed, not ratified.
- [x] **T62 Canonical JSON.** The hash chain committed to PostgreSQL's `jsonb`
  rendering, which no other engine reproduces. Canonicalization moved into Rust
  as `canon::canonicalize` in the map crate: keys sorted by UTF-8 bytes, number
  lexemes verbatim, minimal escaping, infallible. **RFC 8785 was evaluated and
  rejected** — it serializes numbers as IEEE-754 doubles, which would destroy
  the decimal precision M3.6 requires. *Accept:* 13 unit tests, including that
  `1.50` and `1.5` do not collide.
- [x] **T63 DDL for MySQL.** `ddl.rs` re-emitted: backquoted identifiers;
  `ords smallint[]` → `VARBINARY(255)` holding the same text image (the
  database only ever stores it and enforces PK uniqueness — nothing orders,
  subscripts, or unnests it); `DATETIME(6)` rather than `TIMESTAMP`, whose range
  ends in 2038; `LONGTEXT` rather than native `JSON`, so the bytes read back are
  the bytes the chain signed; index prefix lengths computed against InnoDB's
  3072-byte key limit; **hash-surrogate primary keys on `Ext`/`Deep`**, because
  their natural keys hold unbounded text and a prefix index cannot enforce
  uniqueness over the full value; the shared plpgsql guard replaced by
  per-table `SIGNAL` triggers gated on `@fhir_mysql_erasure`; the `_norm`
  function dropped entirely, since it never had a caller. *Accept:* schema
  installs on a live server and the append-only triggers are proven to refuse
  an UPDATE and a DELETE, and to permit a flagged erasure.

- [x] **T70 Divergence policy (M14.0a–M14.0g).** fhir-mysql and fhir-mariadb are
  now explicitly independent: each uses whatever its engine does best, a schema
  installed by one need not be readable by the other, and the emitted SQL will
  differ. What stays shared is *behaviour* — round-trip fidelity, search
  semantics, and the canonical form the chain signs — so the conformance suite,
  not the schema text, is the contract.
  *Found while applying this:* the exact-comparison collation was
  `utf8mb4_bin`, which is **PAD SPACE** — `'Smith' = 'Smith '` evaluates true,
  silently widening `:exact` matching and weakening key identity. PostgreSQL's
  `COLLATE "C"` never pads. Now `utf8mb4_0900_bin`, MySQL 8's NO PAD binary
  collation, with a live test asserting the property rather than the name.
  *Still MySQL's limitations, documented rather than worked around:* triggers
  need drop-then-create, and the audit-envelope `ADD COLUMN` statements are not
  idempotent.

- [ ] **T64 Store layer.** The large one, and the reason this is not yet a
  working MySQL server: `store/src/lib.rs` and `search.rs` still speak
  `tokio-postgres` with `$n` placeholders, `::jsonb`/`::timestamptz` casts,
  advisory locks, `SET LOCAL` GUCs, and a staging-schema install. Swap to
  `mysql_async`, `?` placeholders, `GET_LOCK`, `information_schema`.
- [ ] **T65 Surrogate key computation.** T63 emits `key_hash BINARY(32)` on
  `Ext`/`Deep` but nothing fills it; the store must compute it in Rust over the
  canonically joined natural key.
- [ ] **T66 Decimal sort columns.** `ColTy::Numeric` needs a derived
  `<name>_sort` companion in the generated map, following the pattern `date`
  and `dateTime` already use, so range search has something numeric to index.
  Touches the gen crate, which T63 did not. Until it lands, numeric and
  quantity range search is a known regression from PostgreSQL.
- [ ] **T67 Wire the canonicalizer into the chain.** T62 built and tested it
  but nothing calls it yet; `chain.rs`'s preimage still expects the database's
  rendering.
- [ ] **T68 Non-atomic install.** MySQL DDL implicitly commits, so a staged
  install cannot be atomic. Needs the readiness-marker scheme and the operator
  documentation for an interrupted install.
- [ ] **T69 Amend sections 1–13.** They still describe PostgreSQL throughout,
  including the `ords[1]` query idiom the book teaches.

- [x] **T71 Local container testing (`scripts/db.sh`, `doc/containers.md`).**
  The live suite is where most of this project's guarantees are actually
  checked, and it previously needed a hand-rolled server. `scripts/db.sh` now
  starts the same pinned image CI uses (`docker.io/library/mysql:8.4`), waits until it genuinely
  answers, lays out the FHIR definitions and example corpus, and runs the suite.
  Podman by default, Docker if that is what is installed.
  *Two defects found and fixed while building it:* the readiness probe accepted
  a Unix socket, and both the PostgreSQL and MySQL official images run a
  temporary socket-only server while initializing — so it reported ready before
  the mapped port was open, and the first tests to connect failed for no visible
  reason. Probes are now TCP-only. And every live test carried machine-local
  absolute fallback paths, one of them a stale scratchpad directory from an
  earlier session, which is why the corpus tests had been skipping silently
  rather than failing; 52 test files now derive those paths from
  `CARGO_MANIFEST_DIR`.

- [x] **T72 CI retargeted to the real engine.** The inherited pipelines still
  provisioned PostgreSQL 18 and set `PG*` variables, so the ports' "live
  database" jobs were green while testing an engine none of them ships against.
  Both forges now run `mysql:8.4`: `.github/workflows/ci.yml` (`database`) and
  `.woodpecker/database.yaml`, using the same pinned image as
  `scripts/db.sh`, so local and CI runs are the same claim.
  The `mysql_ddl` suite runs with `FHIR_MYSQL_DDL_FULL=1`, installing every
  resource type rather than a sample. The TLS-only job is removed until the
  store speaks MySQL, since the plaintext-refusal guard is store-layer
  behaviour; tracked rather than faked.
  Where the store is not yet ported (T64), the store and server suites are still
  invoked and log an explicit notice that they self-skip — a visible gap beats a
  gap inferred from a job's absence.

- [x] **T73 Reduced to an embeddable library.** Scope correction: this project
  is a library to embed, not an HTTP server and not a command-line tool.
  Removed the server crate (and Axum, tower, axum-server with it) and the CLI
  binary crate; the workspace is now `-map`, `-gen`, `-store`.
  The generated relmaps lived inside the CLI crate and had to outlive it, so
  `crates/fhir-mysql/assets/` moved to a top-level `assets/`, with test paths
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

- [~] **T74 Native MySQL store — started.** Same parallel-module approach the
  SQLite port used: `store/src/mysql.rs` is built alongside the inherited
  PostgreSQL store, which is deleted at parity. `mysql_async` chosen —
  async and pure Rust, so it needs no client library on the host and no
  `spawn_blocking`; decision D5's reason for rejecting `sqlx` holds here too.
  *Landed:* `connect` (which probes the server rather than failing at first
  use), `init`, `installed_checksum`, `table_count`, `trigger_count`,
  `drop_schema`, `ping`, and `surrogate_key` for M14.12's hash primary key.
  *Two details worth keeping:* the surrogate hash delimits its components rather
  than concatenating them, because `("ab","c")` and `("a","bc")` would otherwise
  collide — presenting as silent data loss and being very hard to trace back.
  And `Debug` on the store is shallow by design: the pool's own `Debug` prints
  connection options, and a DSN can carry a password.
  *Accept:* 3 tests against real MySQL 8.4 in a container. `init` installs
  tables *and* triggers, counted separately — a schema with tables but no
  triggers would look healthy while guaranteeing nothing (M3.17). A failed
  install asserts the error says the schema is left partial and cites M14.22,
  then verifies the tables really do remain, so the message is true rather than
  merely reassuring.
  *Also landed:* `put` and `get`, so MySQL round-trips real resources. Writes go
  through one transaction; a rewrite deletes the base row and lets
  `ON DELETE CASCADE` clear the children. The chain preimage is computed from
  `canon::canonicalize` and the canonical bytes are what get stored, so what is
  read back is exactly what was signed. Timestamps are rendered in UTC in Rust
  rather than by `NOW()`, which follows the session time zone — a verifier
  elsewhere would otherwise recompute different bytes and call every row broken.
  `Ext` and `Deep` inserts fill the `key_hash` surrogate (M14.12).
  *Accept:* 6 tests against real MySQL 8.4, including that `9.60` survives the
  round trip, that a rewrite removes stale child rows rather than shadowing them,
  and that a resource with three extensions comes back whole — which is the
  surrogate-key collision check, since a collision would present as a lost
  extension rather than as a key error.
  *Fixed while testing:* the test sampler trimmed the map to "the first six"
  resource types, which is alphabetical and therefore excludes `Observation`;
  every round-trip test failed with `unknown resource type`, which looks like a
  store bug and was not. It now selects by name and asserts the selection is
  non-empty.
  *Also landed:* `history`, `vread`, `delete` (tombstone with `op='D'`, base row
  removed, history retained), and `verify_audit` checking both hash chains and
  the keyed tag. The erasure-tombstone and stored-`prev_hash` rules from the
  SQLite port carry over: an `X` row is excluded from link checking, because its
  predecessors were deleted on purpose and checking the link would report every
  lawful erasure as tampering; and the MAC is verified against the row's stored
  `prev_hash` rather than the walk's tip, which is what lets a tombstone keep a
  meaningful tag.
  *Accept:* 8 tests against real MySQL 8.4. The tamper test drops the append-only
  UPDATE trigger — the deliberate act M3.17 exists to make visible — rewrites an
  actor, and asserts all three layers object. Mutation-checked: with the hash
  comparison disabled only `hmac-sha256` objects and the test fails, so it is
  load-bearing.
  *Fixed while testing:* `last_updated` is `DATETIME(6)` and the driver returns
  it as a date value, not text — but the hash chain commits to the *text* form.
  All three read sites now format it in SQL with `DATE_FORMAT(…, '%f')` to
  exactly the shape `utc_micros` wrote, so the bytes cannot drift with a driver
  or server change in rounding.
  *Also landed:* search. `store/src/mysql_search.rs` is forked from the inherited
  PostgreSQL builder, like the SQLite port's. Backquoted identifiers, plain `?`
  placeholders (MySQL binds strictly in order, so numbered `?n` would be a lie),
  `ISNULL(col)` to emulate the `NULLS LAST` MySQL lacks, `DATE_ADD(…, INTERVAL 1
  SECOND)` for interval arithmetic, and an ISO sentinel for `infinity`.
  Numbers compare through `CAST(… AS DECIMAL(65,30))`: the column holds the exact
  lexical form because M3.6 demands it, and text compares lexicographically —
  under which `"9" > "10"`. `DECIMAL` rather than the SQLite port's `REAL`
  because MySQL has it and it is exact within range; the ports need not agree,
  and here the better tool exists. Verified by mutation: without the cast `gt9`
  returns nothing at all.
  *Caught by a test rather than by review:* three identifiers survived the
  double-quote-to-backquote conversion (`c0x."rid"` and two others), which MySQL
  rejects as a syntax error. A search test exercising a qualified token found
  them; the `search_values_are_bound_never_interpolated` test now also asserts no
  double-quoted identifier remains.
  *Also landed:* the access log and `purge` (GDPR Art. 17, M3.18). The
  append-only trigger permits the delete only while `@fhir_mysql_erasure` is
  set, and that variable is **per-connection** — so `purge` holds one connection
  for the whole operation and clears the flag on every exit path. Setting it on a
  pooled connection and deleting on another would leave the trigger firing and
  look like the trigger was broken; leaving it set would hand an unrelated later
  request permission to delete history.

- [x] **T75 PostgreSQL store removed from fhir-mysql.** The inherited `Store` is
  gone, and with it `tokio-postgres`, `deadpool-postgres`,
  `tokio-postgres-rustls`, `rustls`, `rustls-native-certs`, and `futures-util`.
  `fhir-mysql-store` now depends on `mysql_async`, `serde_json`, `tokio`,
  `thiserror`, `tracing`, and the crypto crates the chain needs. `lib.rs` went
  from 2,720 lines to the shared vocabulary and nothing else. Eight inherited
  test files that exercised the deleted store went too; they self-skipped without
  a PostgreSQL database, so the drop in the count is stubs that never ran.
  *Accept:* 13 store tests against real MySQL 8.4, covering schema install,
  CRUD, history, vread, delete, search, chain verification, tamper detection,
  purge, the erasure flag, and the disclosure log.

## Remaining work (as of the port commit)

Native MySQL store, no PostgreSQL dependencies. Behind fhir-sqlite by the
following surface, all of which fhir-sqlite already has:

- [ ] **Conditional create/delete** (`If-None-Exist`). Note the race: two callers
  submitting the same criteria at once both search, both find nothing, both
  create. SQLite solves it with a write gate taken before the search; MySQL is
  genuinely multi-writer, so this needs `GET_LOCK` on the criteria — and MySQL's
  named locks are *session*-scoped, not transaction-scoped, so they must be
  released on every path including errors.
- [ ] **`status`, `get_versioned`, `get_all`, `put_audited` (If-Match),
  `delete_audited`, `access_log_for`** — the surface an HTTP layer needs.
- [ ] **`transact_audited`, `resign_history`, `upgrade`.**
- [ ] **Decimal sort columns**, as for SQLite; `CAST(… AS DECIMAL(65,30))` is
  correct but unindexed.

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
- [ ] **T90a Backfill the `_norm` columns.** The fix changes stored folded
  values, so any database written before it holds stale ones and will miss the
  searches this repaired. fhir-postgresql has `backfill_norm` on its upgrade
  path; the SQLite, MySQL and MariaDB stores have no `upgrade` yet, so for them
  this is currently a re-load rather than a migration. **Deploying the new fold
  against an existing database without backfilling is worse than not fixing it**,
  because searches would then match neither spelling.
