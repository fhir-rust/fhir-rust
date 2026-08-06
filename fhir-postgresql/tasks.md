# fhir-postgresql tasks

> **Parts of this file were untrue of this port (audit [F-27](../spec/databases/audit.md#f-27)).**
> The REST-server and CLI entries were misattributed ancestor-project work:
> the server is [`fhir-loco`](../fhir-loco/), a separate crate, and no port
> has a CLI (`C0.17`, `C0.18`). Class 1 was resolved 2026-08-06 by
> **deleting** those entries — each is now a one-line tombstone keeping its
> task id, because unticking would have asserted that this port is going to
> grow a server, and it is not.
>
> Do not read a `[x]` here as evidence. The
> [conformance matrix](../spec/databases/conformance-matrix.md) is the status document to
> trust.

Work breakdown for the plan's milestones. Each task lists its acceptance
criterion. Order within a milestone is roughly dependency order.

## M1 — Engine proven (R5 vertical slice)

- [x] **T1 Workspace scaffold.** Cargo workspace per plan D14
  (`fhir-postgresql-map`, `fhir-postgresql-gen`, `fhir-postgresql-store`; the
  ancestor plan's server crate became [`fhir-loco`](../fhir-loco/)),
  CI (fmt, clippy, test, live-PG job).
  *Done:* `.github/workflows/ci.yml`; tests self-skip without inputs.
- [x] **T2 Spec-package ingestion.** profiles-resources.json +
  profiles-types.json parsed directly (simpler than reusing the fhir
  crate's parser; that crate still backs `--validate` later) into element
  trees with cardinality, types, choice and contentReference info — for all
  three versions, not just R5. SearchParameters ingestion moves to M3.
- [x] **T3 Relational map format.** `fhir-postgresql-map::model`: node arena (cycles
  via indexes), tables, typed columns, choice variants, reference splits,
  extension/spill channels, 63-byte registry with deterministic
  abbreviation + hash fallback. Assets: `assets/fhir-postgresql-relmap-{r3,r4,r5}
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
- **T8.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the
  server is [fhir-loco](../fhir-loco/), and no port has a CLI (**F-27**).
- [x] **T9 Round-trip property tests.** Map-driven random-resource
  generator (deterministic SplitMix64 seeds — no proptest dependency):
  deep recursion, sparse primitive arrays with extensions, nested
  extensions, choice variants, decimals, partial dates. 10k cases pass
  (`FHIR_POSTGRESQL_PROPTEST_CASES`; default 500 locally).
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
- [x] **T12 fhir_postgresql_meta + idempotent init.** Staged-schema install +
  atomic rename, checksum recorded; re-init no-ops on matching checksum
  and refuses a mismatch. Chunked `drop_schema` + `fhir-postgresql drop --yes`.
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
  unsupported-parameter errors; `_sort` (base-table
  params + _id/_lastUpdated, honest errors otherwise) and
  `_total=accurate`. *Accept mostly met:* the `search_semantics` suite is
  green against live PG (the `fhir-postgresql search` CLI and REST suites
  this entry once cited were the ancestor's, **F-27**). Single-hop
  `_include` (via compiled reference targets) and `_revinclude` (via the
  search machinery) with search.mode=include entries and
  dangling-reference tolerance. Single-hop chained reference search
  (`subject:Patient.name=x`) is implemented (`src/search.rs:148`), and
  `search_page` takes a keyset cursor (`after_id`, `lib.rs:1487`).
  *Remaining:* deeper-than-one-hop chains (refused honestly), lenient
  handling.
- [x] **T15 Index emission + explain audit.** One index per distinct
  search-target column set emitted with the DDL (R5: 1,813 indexes; full
  init 5.8 s). EXPLAIN audit in tests/bench.rs: token/reference/date
  searches all plan index scans at 100k resources; the test fails on seq
  scans. *Note:* ILIKE-prefix string search bypasses btree — revisit with
  text_pattern_ops if profiles demand.

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
> redirected — not `fhir-postgresql serve`, which does not exist). Either
> the boxes are stale or the README overstates. Since the spec is meant to
> be the source of truth, reconcile before release rather than after.

- [ ] **T21 R4 artifacts.** Run generator on 4.0.1; fix spec-parsing deltas.
  *Accept:* full R4 examples corpus round-trips live; REST suite green on
  `/r4`.
- [ ] **T22 R3 artifacts.** Same for 3.0.2.
  *Accept:* full R3 examples corpus round-trips live; REST suite green on
  `/r3`.
- [x] **T23 Multi-version serve.** *Done, in [`fhir-loco`](../fhir-loco/),
  not here.* It loads `r3`, `r4` and `r5` maps at startup and routes on
  `/{version}/…`; this port supplies the store it reads. Not `fhir-postgresql
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
- [x] **T25 Statement-timeout hardening.** Runaway statements die
  server-side: `statement_timeout` is set on every connection
  (`FHIR_POSTGRESQL_STATEMENT_TIMEOUT_MS`, default 30 s; overridable, never
  unset — `crates/fhir-postgresql-store/src/lib.rs:468`). The pool-wait
  timeout and the 503 + Retry-After on exhaustion this entry used to claim
  were the ancestor server's, not this library's (**F-27**).
- [x] **T26 Migrations + upgrade.** `init` stores the map asset in
  fhir_postgresql_meta; `init --upgrade` diffs installed vs current maps and
  applies additive DDL (new tables/columns/indexes) in lock-safe chunks;
  destructive steps refuse without --allow-destructive; column type
  changes always demand manual migration. *Accept met:* upgrade test —
  reduced install upgrades to full, data survives, re-upgrade no-ops,
  downgrade guarded then forced.
- [x] **T27 TLS to the database.** The `serve --tls-cert/--tls-key`,
  axum-server and SIGTERM claims this entry used to carry were the ancestor
  server's — there is no `serve` (**F-27**); a bind guard is
  [`fhir-loco`](../fhir-loco/)'s concern. What is real here is store-level
  TLS: `SslPolicy` with a **verifying default** (`Require`, **F-17** fixed),
  pinned by `crates/fhir-postgresql-store/tests/ssl_default.rs`.
- [~] **T28 Benchmarks + regression gate.** Done: gated bench harness
  (load throughput, read latency, EXPLAIN audit) + doc/benchmarks.md with
  measured numbers (6,146 res/s; 1.18 ms reads at 100k).
  *Remaining:* CI regression gate against a recorded baseline; comparison
  against the historical jsonb implementation.
- [x] **T29 Book + generated schema docs.** mdBook (9 chapters:
  introduction, getting started, storage model, querying, search,
  FHIR versions, operations, architecture, trust boundary — the REST
  chapter went with the server, T61); builds locally and in
  CI. Column/table→FHIR-path mapping ships inside the map assets
  themselves. *Remaining nicety:* a rendered path→table index page.
- [~] **T30 Security review + release.** Done: LICENSE-MIT/APACHE,
  CHANGELOG, publish metadata (versioned internal deps, keywords), map
  assets embedded in the published crates (`RelMap::bundled()`, **F-33** —
  an earlier revision said "embedded in the binary so `cargo install
  fhir-postgresql` is self-contained"; there is no binary and no such
  crate, `C0.18`), `cargo publish --dry-run` clean for the leaf crate;
  cargo-deny + SBOM run in the port workflow (`O10.10`).
  *Remaining (human decisions):* pick the release version, publish the
  three crates in dependency order, tag.

## M7 — Trustworthy under load and under audit

The gap between "works end to end" and "may hold patient data". Ordered by
severity: P0 items are defects in what already ships, P1 items are missing
guarantees, P2 items are reach.

### P0 — defects in shipped behaviour

- [x] **T31 Snapshot reads (R4.5).** `Store::get` reads the base row and
  every child table as separate implicit transactions
  (`fhir-postgresql-store/src/lib.rs:554`), while `put_in` deletes and re-inserts
  (`:512`). A write landing mid-read reconstructs a resource that never
  existed. Wrap every multi-statement read in one
  `REPEATABLE READ READ ONLY` transaction; same for `export` and search
  materialization.
  *Accept:* a reader loop against a writer loop over 10k iterations never
  observes a torn resource and never errors (T11.6).
- [x] **T32 Encrypted database transport (O10.7).** `NoTls` is hard-coded
  (`fhir-postgresql-store/src/lib.rs:186`), so PHI crosses to PostgreSQL in clear and
  `sslmode=require` cannot be honored. Add `tokio-postgres-rustls`, honor
  `sslmode`/`PGSSLROOTCERT`, default `prefer`, and refuse a non-loopback
  `--bind` over an unencrypted connection without `--allow-insecure-db`.
  *Done:* `SslPolicy`, rustls connector, `PGSSLROOTCERT` trust anchors,
  `Store::connect_with`, and a startup warning whenever the link is
  unencrypted. fhir-postgresql's `require` validates the certificate where
  libpq's does not — a documented deviation, in the safe direction.

  **Not done, and previously claimed as done:** the startup refusal.
  `refuse_insecure_db`, the `startup_guard_tests` policy table, and the
  `--bind`/`--allow-insecure-db` interaction do not exist in any port, and
  there is no `serve` binary for them to guard (`C0.17`, `C0.18`). The
  reasoning recorded there — that a bind which will not resolve must count as
  not-loopback, or the check silently skips itself — is a sound argument about
  code nobody has written. Corrected under **F-27**; a bind guard belongs to
  the server, which is [`fhir-loco`](../fhir-loco/), not this library.

  **The default now verifies** — `SslPolicy::Require` is the default,
  closing **F-17**: `O10.7` asks for a verifying default and this port has
  one, pinned by `crates/fhir-postgresql-store/tests/ssl_default.rs`. It was
  a breaking change, recorded in the CHANGELOG under Unreleased; departure
  `M14.27` records the history.

  A live test against a TLS-only PostgreSQL is **written but does not run**.
  The job exists — `.github/workflows/ci.yml`, `tls-database` — but this
  repository's workflows all sit under `<family>/.github/workflows/`, which
  GitHub does not read, so none of them execute (**F-49**).

  What it *would* do, and what someone running it by hand gets: a
  `hostssl`-only server, a step that first proves plaintext really is refused —
  a gate that silently permits downgrade tests nothing — then the live suite
  with `PGSSLMODE=require` and the self-signed certificate as its own trust
  anchor. The design is right; nothing has executed it.

  *Remaining:* nothing on GitHub. There is no Woodpecker counterpart, because
  Woodpecker starts services before workspace steps run, so a certificate
  generated in a step does not exist when the database container boots; the
  workarounds (a committed test key, or docker-in-docker) are each worse than
  the gap. Recorded in `doc/ci.md` rather than left to be discovered.
- [x] **T33 Atomic conditional interactions (A7.10).** Two concurrent
  identical conditional creates must not both create: match and write move
  into one transaction guarded by `pg_advisory_xact_lock` on the criteria
  hash; same for conditional delete.
  *Done:* `Store::conditional_create`/`conditional_delete` take
  `pg_advisory_xact_lock` on a sorted hash of the criteria, then match and
  write in one transaction. (The `fhir-postgresql-server/src/lib.rs:444`
  citation this entry used to carry named a crate that never existed here;
  the HTTP half is `fhir-loco`'s — **F-27**.) *Accept met:* 8 racing
  conditional creates yield exactly one resource and seven `Existing`.
  *Remaining:* conditional update.
- [x] **T34 Audit envelope on history (M3.15, PR12.1–PR12.4).** History
  records no actor at all. Add the audit columns to the generated history
  DDL and thread an audit principal through the store write path. Additive,
  so `upgrade` migrates existing installs.
  *Done (store half):* `Audit` envelope threaded through `put_audited`/
  `delete_audited`/`transact_audited`/`conditional_*_audited`; upgrade
  reconciles the new columns idempotently. The `PrincipalPolicy`,
  `--trust-proxy`, `--require-principal` and `Audit::cli()` claims were the
  ancestor server/CLI's (**F-27**): the principal is a caller-supplied
  value type (`fhir-store::Audit`), and verifying it is `fhir-loco`'s job.
  *Accept met:* the `audit` suite asserts the envelope, and that a plain
  `put` records `unauthenticated` rather than nothing.
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
  parameter) from `Other` (internal detail, never client-facing).
  Search-compilation errors are `Unsupported`, so the honest "this
  parameter is not supported" messages survive. The OperationOutcome and
  incident-id HTTP mapping this entry used to describe is the server's,
  i.e. [`fhir-loco`](../fhir-loco/)'s (**F-27**).

### P1 — missing guarantees

- [x] **T41 Access log (PR12.5).** *Done (store half):*
  `fhir_postgresql_access_log` per schema, written by `Store::log_access`/
  `log_access_batch` and read back by `access_log_for` (`lib.rs:1526`), so
  every disclosure a caller reports is recorded naming the actor and the
  subject. The `--audit-mode sync|async|off` flags, bounded queue,
  `audit_read`, the fail-closed 503 read paths and `tests/audit_async.rs`
  never existed in this port — they were the ancestor server's (**F-27**,
  `PR12.6`); whether a disclosure is recorded before the response is
  released is the caller's ([`fhir-loco`](../fhir-loco/)'s) decision.

- [x] **T42 Tamper-evident history (M3.16, M3.17).** `prev_hash`/`row_hash`
  chain per resource id, `BEFORE UPDATE OR DELETE` reject triggers,
  `Store::verify_audit` walking every chain, and the documented `REVOKE`
  grants. *Done.* The chain pre-image is computed **in Rust** — the stored
  normalized form is canonicalized by `fhir_postgresql_map::canon` and
  chained by the shared `fhir-store::chain` (**F-07** fixed) — not "in SQL"
  over `resource::jsonb::text`, as this entry said before F-07;
  `tests/chain_portability.rs` proves an outside verifier can recompute a
  chain from the stored bytes alone. *Accept met:* the `audit` suite
  tampers with a history row behind the application's back and the chain
  names exactly that version.
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

  *A defect this work introduced.* The rotation test set `FHIR_POSTGRESQL_CHAIN_KEY`
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

  First: the cheap-looking version — an expression index on `fhir_postgresql_norm(col)`
  plus a predicate `fhir_postgresql_norm(col) LIKE fhir_postgresql_norm($1) || '%'` — **does not
  use the index**. PostgreSQL's btree prefix optimization for `LIKE` needs a
  *constant* pattern, and `fhir_postgresql_norm($1)` is only stable, not constant.
  Folding the term in Rust instead keeps the pattern constant, but then the
  fold exists twice — once in Rust, once in SQL — and the two must agree for
  every codepoint, forever, or a patient is simply not found. So the fold
  lives in Rust only (`fhir-postgresql-map::fold`), and the database stores its output.

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

- **T44.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (edge limit flags; `tests/edge_limits.rs` does not exist); the server is
  [fhir-loco](../fhir-loco/) (**F-27**).
- **T45.** — removed 2026-08-06: misattributed ancestor (REST/CLI) work
  (`--admin-bind`, request-latency histogram); the server is
  [fhir-loco](../fhir-loco/) (**F-27**).
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
  database refusing to let history be rewritten. (There is no
  `audit_async.rs` and never was in this port: the async audit queue it
  claimed to pin was the ancestor server's — **F-27**.)
- [x] **T49 Erasure (M3.18).** *Done (store half):* `Store::purge`, the
  tombstone carrying who/when/why plus the terminated chain hash, the
  append-only trigger relaxed to permit `DELETE` only inside a transaction
  that sets `fhir_postgresql.erasure`, and a test that `verify_audit`
  reports a purge as a recorded hole rather than a chain break. The
  `fhir-postgresql purge` CLI and its `--allow-erasure` flag were the
  ancestor's — there is no CLI (**F-27**, `C0.17`). The book states the two
  limits plainly: backups and replicas are outside this, and the guard stops
  accidents rather than the application itself.
- [x] **T50 Trust-boundary chapter (PR12.8).** One table in the book: what
  fhir-postgresql guarantees, what the perimeter must provide, what neither does yet.
  *Done:* `book/src/trust-boundary.md`, the `REVOKE` grants, and an
  honest statement of what the hash chain does *not* prove (an attacker who
  can recompute it — hence: ship `row_hash` off-box). The "worked `serve`
  invocation" this entry used to cite was the ancestor server's (**F-27**).

### P2 — reach

- [ ] **T51 Type- and system-level `_history`.** Required for CDC,
  replication, and incremental export; currently instance-level only.
- [ ] **T52 Bulk Data `$export`.** Async kickoff, NDJSON output, status
  polling — ONC/HTI expects it, and the earlier fhir-postgresql had the client half.
- [ ] **T53 `X-Provenance` and `AuditEvent` projection (PR12.7).** Store
  submitted Provenance; expose the access log as queryable `AuditEvent`.
- [ ] **T54 Inferno / Touchstone conformance run.** External validation of
  §7 and A7.12 against the published test kits.
- [ ] **T55 `_summary` and `_elements`.** Common in production clients;
  currently 501.
- [ ] **T56 PATCH.** JSON Patch and FHIRPath Patch, declared in the
  CapabilityStatement.
- [ ] **T57 Restore and failover drills.** A documented, tested PITR
  restore and a `fhir-postgresql fsck` that checks orphan rows, ordinal gaps, and
  history/current agreement.
- [~] **T58 CI/CD.** Corrected: the per-port pipelines under
  `fhir-postgresql/.github/workflows/` and `.woodpecker/` are **inert** —
  GitHub reads only the repository root's `.github/workflows/` (**F-49**),
  so "parallel pipelines on both forges" never ran. What actually gates
  commits is the root `gates.yml`, which runs the shared-core and
  doc-example gates only. The tag/SBOM/MSRV machinery described in
  `doc/ci.md` is unexecuted workflow text, not CI.

- [x] **T60 Local container testing (`scripts/db.sh`, `doc/containers.md`).**
  The live suite is where most of this project's guarantees are actually
  checked, and it previously needed a hand-rolled server. `scripts/db.sh` now
  starts the same pinned image CI uses (`docker.io/library/postgres:18`), waits until it genuinely
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

- [x] **T61 Reduced to an embeddable library.** Scope correction: this project
  is a library to embed, not an HTTP server and not a command-line tool.
  Removed the server crate (and Axum, tower, axum-server with it) and the CLI
  binary crate; the workspace is now `-map`, `-gen`, `-store`.
  The generated relmaps lived inside the CLI crate and had to outlive it, so
  `crates/fhir-postgresql/assets/` moved to `crates/fhir-postgresql-map/assets/`,
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

## Remaining work (as of the port commit)

This project keeps the PostgreSQL store: that *is* its product. It is now
library-only — no HTTP server, no CLI.

- [~] **T21/T22 R4 and R3 artifacts.** The r3/r4/r5 relmap assets are
  committed (`crates/fhir-postgresql-map/assets/fhir-postgresql-relmap-{r3,r4,r5}.json.gz`)
  and the matrix's `S1.1` row is satisfied; the M5 boxes stay unchecked only
  for the live full-corpus acceptance runs.
- [ ] **P2 reach items** (T51–T57): type/system-level `_history`, Bulk
  `$export`, `X-Provenance`/`AuditEvent`, Inferno/Touchstone conformance,
  `_summary`/`_elements`, PATCH, restore and failover drills.
- [ ] **Sections 7 and 8 of the spec were removed** with the REST API and CLI.
  Sections 1–6 and 9–13 still assume a server in places and want a read-through.

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
- [x] **T90a Backfill the `_norm` columns — done.** The fix changes stored
  folded values, so any database written before it holds stale ones and will
  miss the searches this repaired. fhir-postgresql has `backfill_norm` on its
  upgrade path, and the SQLite, MySQL and MariaDB stores now have `upgrade` +
  `backfill_norm` too, live-verified (**F-15** closed; e.g.
  `fhir-sqlite/crates/fhir-sqlite-store/src/sqlite.rs:390`/`:632`, with 8
  tests in that port's `tests/upgrade.rs`). **Deploying the new fold against
  an existing database without backfilling is worse than not fixing it**,
  because searches would then match neither spelling.
