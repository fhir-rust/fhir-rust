# fhir-sqlite tasks

> **Parts of this file were untrue of this port (audit [F-27](../spec/databases/audit.md#f-27)).**
> The REST-server and CLI entries were misattributed ancestor-project work:
> the server is [`fhir-loco`](../fhir-loco/), a separate crate that mounts
> this port, and there is no CLI (`C0.17`, `C0.18`). Class 1 was resolved
> 2026-08-06 by **deleting** those entries — each is now a one-line tombstone
> keeping its task id, because unticking would have asserted that this port
> is going to grow a server, and it is not.
>
> Do not read a `[x]` here as evidence. The
> [conformance matrix](../spec/databases/conformance-matrix.md) is the status document to
> trust.
>
> This port's store tasks also described **PostgreSQL** mechanisms it does not
> use — `tokio-postgres`, `FOR UPDATE` row locks, staged-schema install,
> `ILIKE` — because the file was copied per port and never re-read (class 3,
> now corrected in place). The store is real; where an entry names a
> mechanism, it now names SQLite's.

Work breakdown for the plan's milestones. Each task lists its acceptance
criterion. Order within a milestone is roughly dependency order.

## M1 — Engine proven (R5 vertical slice)

- [x] **T1 Workspace scaffold.** Cargo workspace per plan D14
  (`fhir-sqlite-map`, `fhir-sqlite-gen`, `fhir-sqlite-store`; the ancestor
  plan's server crate became [`fhir-loco`](../fhir-loco/)),
  CI (fmt, clippy, test — SQLite-only, no service container).
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
  via a direct statement-by-statement install. **Not** staged-schema +
  rename: SQLite has no transactional DDL and no schema-rename, so the install applies statements directly; a failed install is cleaned up by unlinking the file, because here the schema **is** the file. The 9.5 s figure below was PostgreSQL's.
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
- [x] **T7 Store layer: init/load/read.** `rusqlite`, one connection per store;
  transactional put with history append; multi-table reads; chunked
  multi-row inserts; values bound as text so decimal scale and partial dates
  survive (`M3.6`). **Not** tokio-postgres, and there are no `($n::text)`
  casts — that was PostgreSQL's wire protocol (**F-27** class 3).
  *Accept:* full-corpus round trip green in this port (map layer, **F-42**);
  the bulk-benchmark figures an earlier revision cited here (6,146 res/s,
  1.18 ms reads) were `fhir-postgresql`'s — this port has no bench harness,
  a recorded gap (**F-64**).
- **T8.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the
  server is [fhir-loco](../fhir-loco/), and no port has a CLI (**F-27**).
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
  *Accept met:* asserted in `tests/sqlite_store.rs` (an earlier revision
  named an `m2_semantics` test this port does not have — that file is
  `fhir-postgresql`'s) — create→update→delete shows D/U/C history, vread of
  each version matches, deleted reads as Deleted.
- [x] **T11 Optimistic concurrency.** `put_if(resource, expected_version)`
  with the version check inside the same `BEGIN IMMEDIATE` transaction as
  the write, serialized by the process-level `write_gate`
  (`sqlite.rs:127`) — SQLite has no `FOR UPDATE` row locks; that wording
  was PostgreSQL's (**F-27** class 3). `StoreError::Conflict` for the
  caller's 412; expected 0 = create-only (If-None-Exist shape).
  *Accept met:* two racing conditional writers — exactly one wins.
- [x] **T12 fhir_sqlite_meta + idempotent init.** Statement-by-statement
  install — **not** staged-schema + atomic rename, which SQLite cannot do
  (see T4); a failed install is cleaned up by unlinking the file, because
  the schema *is* the file. Checksum recorded; re-init no-ops on matching
  checksum and refuses a mismatch. `drop_schema` exists; the
  `fhir-sqlite drop --yes` CLI never did (**F-27**).
  *Accept met* in `tests/sqlite_store.rs`.

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
  unsupported-parameter errors; `_sort` (base-table
  params + _id/_lastUpdated, honest errors otherwise) and
  `_total=accurate`. *Accept mostly met:* the search assertions in
  `tests/sqlite_store.rs` run against a local file — the "search_semantics +
  rest suites green against live PG" and the `fhir-sqlite search` CLI this
  entry once cited were the reference port's (**F-27**). Single-hop
  `_include` (via compiled reference
  targets) and `_revinclude` (via the search machinery) with
  search.mode=include entries and dangling-reference tolerance.
  *Remaining:* chained `reference.`, cursor paging, lenient handling.
- [x] **T15 Index emission.** One index per distinct search-target column
  set emitted with the DDL (`CREATE INDEX "s"."ix" ON "t"` per T63). There
  is no EXPLAIN audit and no `tests/bench.rs` in this port — the
  1,813-index count, the 5.8 s init and the plan assertions were the
  PostgreSQL reference's (**F-27** class 3). Prefix string search uses
  `LIKE` over the Rust-folded `_norm` column (T43); the
  `ILIKE`/`text_pattern_ops` note here was PostgreSQL's too.

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
> README claims all three corpora round-trip losslessly, and
> [`fhir-loco`](../fhir-loco/) loads all three maps at startup (T23,
> redirected — not `fhir-sqlite serve`, which does not exist). Either the
> boxes are stale or the README overstates. Since the spec is meant to be
> the source of truth, reconcile before release rather than after.

- [ ] **T21 R4 artifacts.** Run generator on 4.0.1; fix spec-parsing deltas.
  *Accept:* full R4 examples corpus round-trips live; REST suite green on
  `/r4`.
- [ ] **T22 R3 artifacts.** Same for 3.0.2.
  *Accept:* full R3 examples corpus round-trips live; REST suite green on
  `/r3`.
- [x] **T23 Multi-version serve.** *Done, in [`fhir-loco`](../fhir-loco/),
  not here.* It loads `r3`, `r4` and `r5` maps at startup and routes on
  `/{version}/…`; this port supplies the store it reads. Not `fhir-sqlite
  serve`, which does not exist (`C0.18`).

## M6 — Production hardening

- **T-validate.** — removed 2026-08-06: misattributed ancestor (REST/CLI)
  work; there is no `validate` build feature, no `load --validate`, and no
  `validate_tests` here — the `fhir` crate is not a dependency of any port
  crate (**F-27**).
- [x] **T-graceful.** *Belongs to [`fhir-loco`](../fhir-loco/).* Shutdown
  is the server's concern, and Loco owns the signal handling; a library
  has no process to shut down.

- **T24.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (/health, /ready, /metrics, X-Request-Id); the server is
  [fhir-loco](../fhir-loco/) (**F-27**).
- **T25.** — removed 2026-08-06: misattributed ancestor (server
  pool/timeout) work; the server is [fhir-loco](../fhir-loco/) (**F-27**).
  SQLite has no `statement_timeout` and no pool; the store's real timeout
  knob is `busy_timeout` = 30 s (`sqlite.rs:170`).
- [x] **T26 Migrations + upgrade.** `init` stores the map asset in
  fhir_sqlite_meta; `init --upgrade` diffs installed vs current maps and
  applies additive DDL (new tables/columns/indexes) in one transaction
  (`M14.31` — an earlier revision said "lock-safe chunks", the ancestor's
  approach);
  destructive steps refuse without --allow-destructive; column type
  changes always demand manual migration. *Accept met:* upgrade test —
  reduced install upgrades to full, data survives, re-upgrade no-ops,
  downgrade guarded then forced.
- **T27.** — removed 2026-08-06: misattributed ancestor (TLS serve/bind
  guard) work; the server is [fhir-loco](../fhir-loco/) (**F-27**). A SQLite
  file has no connection to encrypt, so `O10.7` is vacuous here — see T32
  for the honest statement of what displaces it.
- [ ] **T28 Benchmarks + regression gate.** Unticked 2026-08-06: the "gated
  bench harness" and the measured numbers (6,146 res/s; 1.18 ms reads) this
  entry carried were `fhir-postgresql`'s (**F-64**) — there is no `bench.rs`
  in this workspace. A real SQLite harness is the recorded gap; a CI
  regression gate would follow it.
- [x] **T29 Book + generated schema docs.** mdBook (9 chapters:
  introduction, getting started, storage model, querying, search,
  FHIR versions, operations, architecture, trust boundary — the REST
  chapter went with the server, T71); builds locally and in
  CI. Column/table→FHIR-path mapping ships inside the map assets
  themselves. *Remaining nicety:* a rendered path→table index page.
- [~] **T30 Security review + release.** Done: LICENSE-MIT/APACHE,
  CHANGELOG, publish metadata (versioned internal deps, keywords), map
  assets embedded in the published crates (`RelMap::bundled()`, **F-33** —
  an earlier revision said "embedded in the binary so `cargo install
  fhir-sqlite` is self-contained"; there is no binary and no such crate,
  `C0.18`), `cargo publish --dry-run` clean for the leaf crate; cargo-deny
  + SBOM run in the port workflow (`O10.10`).
  *Remaining (human decisions):* pick the release version, publish the
  three crates in dependency order, tag.

## M7 — Trustworthy under load and under audit

The gap between "works end to end" and "may hold patient data". Ordered by
severity: P0 items are defects in what already ships, P1 items are missing
guarantees, P2 items are reach.

### P0 — defects in shipped behaviour

- [x] **T31 Snapshot reads (R4.5).** A write landing mid-read must not let
  a reader reconstruct a resource that never existed: every multi-statement
  read runs inside one **deferred read transaction**, which under WAL
  observes a stable snapshot for its duration (annex `M14.20`,
  `spec/14-sqlite-dialect.md:232`). SQLite has no `REPEATABLE READ READ
  ONLY`, and the `lib.rs:554`/`:512` citations this entry used to carry
  were the PostgreSQL reference's (**F-27** class 3).
  *Accept:* a reader loop against a writer loop never observes a torn
  resource and never errors (T11.6, `tests/concurrency.rs`).
- [x] **T32 Encrypted database transport (O10.7).** **Vacuous here, and that is
  the honest answer.** SQLite is a local file: there is no connection, so there
  is nothing to encrypt and `O10.7` has no work to do. What it displaces is not
  nothing — the PHI is at rest in a file whose protection is filesystem
  permissions and disk encryption, both the deployment's responsibility, and the
  port README says so.

  This entry previously described `SslPolicy`, a rustls connector,
  `PGSSLROOTCERT` trust anchors, a `serve` startup guard and a TLS-only
  PostgreSQL CI job. None of that exists in this port — `SslPolicy` and
  `connect_with` are `fhir-postgresql`'s, and no port has `serve` or
  `refuse_insecure_db` at all. Corrected under **F-27** class 3; the security
  claim was the most misleading of the set, because it asserted PHI-in-transit
  protection for a link that does not exist.

- [x] **T33 Atomic conditional interactions (A7.10).** Two concurrent
  identical conditional creates must not both create. *Done:*
  `conditional_create_audited`/`conditional_delete_audited`
  (`sqlite.rs:2827`/`:2857`) take the process-level `write_gate` **before**
  the criteria search, so search-then-write is indivisible — SQLite has no
  `pg_advisory_xact_lock`; one writer at a time is structural here (annex
  `M14.18`/`M14.19`). The `fhir-sqlite-server/src/lib.rs:444` citation this
  entry used to carry named a crate that never existed (**F-27**).
  *Accept met:* racing conditional creates yield exactly one resource.
  *Remaining:* conditional update.
- [x] **T34 Audit envelope on history (M3.15, PR12.1–PR12.4).** History
  records no actor at all. Add the audit columns to the generated history
  DDL and thread an audit principal through the store write path.
  *Done (store half):* `Audit` threaded through `put_audited`/
  `delete_audited`/`conditional_*_audited` (`transact_audited` stays
  `Unsupported` — T64c); `upgrade` *diffs* the audit envelope rather than
  reconciling it in place, because SQLite has no `ADD COLUMN IF NOT EXISTS`
  (`M14.32`). The `PrincipalPolicy`, `--trust-proxy`, `--require-principal`
  and `Audit::cli()` claims were the ancestor server/CLI's (**F-27**): the
  principal is a caller-supplied value type (`fhir-store::Audit`), and
  verifying it is `fhir-loco`'s job.
- **T35.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (`--base-url`, forwarded headers); the server is
  [fhir-loco](../fhir-loco/) (**F-27**).
- **T36.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (Bundle precondition handling); the server is
  [fhir-loco](../fhir-loco/) (**F-27**).
- **T37.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (`rewrite_refs`); the server is [fhir-loco](../fhir-loco/) (**F-27**).
- **T38.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work;
  `valid_fhir_id` exists nowhere in this port; the server is
  [fhir-loco](../fhir-loco/) (**F-27**).
- **T39.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (PHI response headers, CORS); the server is
  [fhir-loco](../fhir-loco/) (**F-27**).
- [x] **T40 Diagnostics hygiene (A7.11).** *Done (store half):* `StoreError`
  distinguishes `Unsupported` (client-safe: names the caller's own
  parameter, or the operation the port honestly declines — e.g.
  `transact_audited`) from `Other` (internal detail, never client-facing).
  Search-compilation errors are `Unsupported`, so the honest "this
  parameter is not supported" messages survive. The OperationOutcome and
  incident-id HTTP mapping this entry used to describe is the server's,
  i.e. [`fhir-loco`](../fhir-loco/)'s (**F-27**).

### P1 — missing guarantees

- [x] **T41 Access log (PR12.5).** *Done (store half):*
  `fhir_sqlite_access_log`, written by `log_access`/`log_access_batch` and
  read back by `access_log_for` (T64/T64b), recording actor and subject for
  each disclosure a caller reports. The `--audit-mode sync|async|off`
  flags, bounded queue, `audit_read`, the fail-closed 503 read paths and
  `tests/audit_async.rs` never existed in this port — they were the
  ancestor server's (**F-27**, `PR12.6`); whether a disclosure is recorded
  before the response is released is the caller's
  ([`fhir-loco`](../fhir-loco/)'s) decision.

- [x] **T42 Tamper-evident history (M3.16, M3.17).** `prev_hash`/`row_hash`
  chain per resource id, per-table `BEFORE UPDATE`/`BEFORE DELETE` triggers
  with `RAISE(ABORT)` (T63), and `verify_audit` walking every chain.
  *Done.* The chain pre-image is computed **in Rust** — canonical bytes
  from `fhir_sqlite_map::canon`, chained by the shared `fhir-store::chain`
  (**F-07**) — never "in SQL" over `resource::jsonb::text`, which was the
  PostgreSQL reference's pre-F-07 design and was never true here. The
  `fhir-sqlite verify-audit` CLI and the `REVOKE` grants were the
  ancestor's (**F-27**); SQLite has no `GRANT`/`REVOKE`. *Accept met:*
  `verify_audit_detects_a_tampered_history_row` (T64) edits a stored
  resource behind the store's back and both chains flag it.
- [x] **T59 Tamper evidence that survives the database (M3.16a-c).** Two
  chains in two design families, a keyed tag, and an external witness.

  *The correction that shaped it.* T42 computed the chain in SQL, for two
  real reasons — it covered the database's own `now()`, and it could not race
  the read of the previous digest. Both survive without it: the timestamp is
  read in the same transaction and written back verbatim, and the write path
  already holds the `write_gate` and its `BEGIN IMMEDIATE` transaction
  before appending history (SQLite has no `SELECT … FOR UPDATE`).
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
  refers to what was removed — so a witness layer must digest every head.
  Here that layer is incomplete: `emit_checkpoint` writes to an
  `audit_checkpoint` log target (T64b), but `chain_witness` itself is still
  unimplemented in this port (T64c).

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
  column gets a `_norm` companion holding the folded value, computed in
  Rust at write time (`fhir-sqlite-map::fold`); search runs `LIKE` over the
  folded column (`sqlite_search.rs`). No SQL fold function exists — T63
  records that the inherited `fhir_sqlite_norm` SQL function was dropped,
  having never had a caller. The ~40 lines of PostgreSQL planner analysis
  this entry used to carry — `fhir_sqlite_norm(col)` expression indexes,
  `text_pattern_ops`, `plan_cache_mode`, `COLLATE "C"`, and the
  `search_semantics.rs` test — were the reference port's (**F-27** class 3).
  `:exact` deliberately compares the *stored* column: it is defined as the
  literal string, so folding must not leak into it.
  Migration: `upgrade` adds the columns and `backfill_norm` fills them,
  resumably and in batches (T90a).

- **T44.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (edge limit flags; `tests/edge_limits.rs` does not exist); the server is
  [fhir-loco](../fhir-loco/) (**F-27**).
- **T45.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (`--admin-bind`, the `fhir_sqlite_request_latency_seconds` histogram);
  the server is [fhir-loco](../fhir-loco/) (**F-27**).
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
  covers torn reads and racing writers (T11.6); `redaction.rs` covers
  T11.7; the audit assertions — the envelope, chain verification,
  tampered-row detection, erasure tombstones — live in
  `tests/sqlite_store.rs` (T11.8), exactly as the conformance matrix's
  "A correction" section records. There is no `audit.rs` and no
  `audit_async.rs` in this port; the async audit queue was the ancestor
  server's (**F-27**).
- [x] **T49 Erasure (M3.18).** *Done (store half):* `SqliteStore::purge`
  leaves a tombstone recording who/when/why plus the hash the chain ended
  on, and the append-only trigger permits the `DELETE` only while the
  in-schema erasure flag row exists — inserted and removed inside the same
  transaction, so an aborted erasure cannot leave the escape hatch open the
  way PostgreSQL's session GUC could (T63/T64; the `fhir_sqlite.erasure`
  GUC wording here was the reference port's). `verify_audit` reports a
  purge as a recorded hole rather than a chain break. The
  `fhir-sqlite purge` CLI and its `--allow-erasure` flag went with the CLI
  crate (T71, **F-27**).
- [x] **T50 Trust-boundary chapter (PR12.8).** One table in the book: what
  fhir-sqlite guarantees, what the deployment must provide, what neither
  does yet. *Done:* `book/src/trust-boundary.md`, and an honest statement
  of what the hash chain does *not* prove (an attacker who can recompute
  it — hence: ship `row_hash` off-box). The "worked `serve` invocation" and
  `REVOKE` grants this entry used to cite were the ancestor server's —
  there is no `serve`, and SQLite has no `GRANT`/`REVOKE` (**F-27**).

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
- [~] **T58 CI/CD.** Corrected: the per-port pipelines under
  `fhir-sqlite/.github/workflows/` and `.woodpecker/` are **inert** —
  GitHub reads only the repository root's `.github/workflows/` (**F-49**) —
  and the "live-PostgreSQL suite" they describe is the wrong engine
  entirely: this port's tests run against a local file and need no
  service at all. What actually gates commits is the root `gates.yml`,
  which runs the shared-core and doc-example gates only.

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
  *Also landed, since removed:* the CLI was wired to `SqliteStore` at the
  time and verified by driving the built binary — `init` installing 9,490
  statements, `load`, `get` returning `9.60` intact, `search status=final`,
  `verify-audit` naming its layers, `purge` refusing without
  `--allow-erasure` — but the CLI crate was later removed by the
  library-scope correction (T71), so none of those commands exist today;
  what survives is the store surface they exercised. (The "commands with no
  SQLite store behind them" list this entry kept — `export`,
  `chain-resign`, `chain-witness`, `init --upgrade` — went with the CLI
  too; `upgrade` has since been implemented on the store itself, T90a.)
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
  cleared, and **[`fhir-loco`](../fhir-loco/)** now answers requests from a
  SQLite file. Not `fhir-sqlite serve` — no such binary exists (`C0.18`); the
  work is real, the name in this entry was the ancestor project's.
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
  `chain_witness`, and `export`. Each fails saying so rather than
  pretending. `init --upgrade` is no longer on this list:
  `SqliteStore::upgrade` (`sqlite.rs:390`) and `backfill_norm` (`:632`)
  exist, with 8 tests in `tests/upgrade.rs` — see T90a.

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
  `crates/fhir-sqlite/assets/` moved to `crates/fhir-sqlite-map/assets/`,
  with test paths repointed and the published checksums re-verified against
  the moved files.
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
- [ ] **`resign_history`, `chain_witness`, `export`** are unimplemented;
  each fails saying so rather than pretending. (`init --upgrade` used to be
  on this list; it is done — `SqliteStore::upgrade` + `backfill_norm`,
  T90a.)
- [ ] **T65 Decimal sort columns (M14.11).** `ColTy::Numeric` needs a derived
  `<name>_sort` companion in the generated map, following the pattern `date` and
  `dateTime` already use. Until then numeric range search works via
  `CAST(… AS REAL)`, which is correct but gives up the index.
- [ ] **T67 Amend spec sections 1–13.** They still describe PostgreSQL
  throughout, including the `ords[1]` query idiom the book teaches, which a TEXT
  column cannot support.

### Cross-cutting, all repos

- [x] **Git remotes and shared history** — resolved by the monorepo merge:
  the ports are directories in one repository with one remote,
  `git@github.com:fhir-rust/fhir-rust.git`, and no per-port `.git`
  (**F-11**).
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
  *Since closed elsewhere too:* every other port now carries
  `upgrade`/`backfill_norm` as well (audit **F-15**, closed on all six —
  Oracle, the last, on 2026-08-09). Still true
  here: a SQLite database installed before `init` recorded the asset has
  nothing to diff and must be reloaded.
- [x] **T91 Boolean token search matched nothing (audit F-71) — fixed
  2026-08-04.** `active=true` silently returned zero rows: SQLite's affinity
  rule only promotes a bound TEXT value to NUMERIC when it *looks like* a
  number, so `active = ?` bound to `"true"` compared the TEXT value
  `'true'` against the column's INTEGER storage class and never matched —
  FHIR's own boolean-token spelling, found live. Fixed with `col_is_bool` +
  `bool_token_as_bind` (`sqlite_search.rs:281-293`, `:372-375`), which bind
  `"1"`/`"0"` for `ColTy::Bool` columns. *Accept:*
  `boolean_token_search_finds_a_true_value` (`tests/sqlite_store.rs:836`).
