# fhir-mysql tasks

> **Parts of this file were untrue of this port (audit [F-27](../spec/databases/audit.md#f-27)).**
> The REST-server and CLI entries were **misattributed ancestor work**, not
> unfinished plans: the server is [`fhir-loco`](../fhir-loco/) (owner decision
> 2026-08-03). Class 1 was resolved 2026-08-06 by **deletion, not unticking** —
> each such entry below is now a one-line tombstone. There is no
> `fhir-*-server` crate, no `serve` binary, and no CLI in this workspace.
>
> Do not read a `[x]` here as evidence. The
> [conformance matrix](../spec/databases/conformance-matrix.md) is the status document to
> trust.
>
> This port's store tasks also described **PostgreSQL** mechanisms it does not
> use — `tokio-postgres`, `pg_advisory_xact_lock`, staged-schema install,
> `ILIKE` — because the file was copied per port and never re-read. Those found
> by the audit were corrected in place 2026-08-06. (`FOR UPDATE` was never
> contamination here — this store really uses it, H5.4.)

Work breakdown for the plan's milestones. Each task lists its acceptance
criterion. Order within a milestone is roughly dependency order.

## M1 — Engine proven (R5 vertical slice)

- [x] **T1 Workspace scaffold.** Cargo workspace per plan D14
  (`fhir-mysql-map`, `fhir-mysql-gen`, `fhir-mysql-store`, `fhir-mysql` — the server crate
  arrives with M4), CI (fmt, clippy, test, live-database job — the port's
  workflow file provisions `mysql:8.4`, though per-port workflows are inert in
  the monorepo: GitHub reads only the root `.github/workflows/` (**F-49**)).
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
### T8. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)
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
- [x] **T12 fhir_mysql_meta + idempotent init.** Checksum recorded; re-init
  no-ops on matching checksum and refuses a mismatch; chunked `drop_schema`.
  **Not** staged-schema + atomic rename: MySQL DDL implicitly commits, so the
  install applies statement-by-statement and is not atomic — see T4 and T68
  (**F-27** class 3). There is no `fhir-mysql drop` CLI (`C0.17`).

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
  `tests/mysql_store.rs` are green against live MySQL 8.4 (there is no
  `fhir-mysql search` CLI — `C0.17`). Single-hop `_include` (via compiled reference
  targets) and `_revinclude` (via the search machinery) with
  search.mode=include entries and dangling-reference tolerance.
  *Remaining:* chained `reference.`, cursor paging, lenient handling.
- [x] **T15 Index emission + explain audit.** One index per distinct
  search-target column set emitted with the DDL. The EXPLAIN audit "in
  tests/bench.rs" and its figures (R5: 1,813 indexes; 5.8 s init; index scans
  at 100k resources) were the reference port's — there is no `tests/bench.rs`
  in this port, and no EXPLAIN audit has been run here (**F-27** class 3).
  *Note:* prefix string search is a range predicate against the folded
  `_norm` column under a NO PAD binary collation (`:contains` falls back to
  `LIKE`); MySQL has no `ILIKE` — the `ILIKE`/`text_pattern_ops` note here
  was PostgreSQL's.

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
> records a full-corpus live round trip of 7,396/7,396 across r3/r4/r5 and the
> README claims all three corpora round-trip losslessly. (This note used to
> add that "`fhir-mysql serve` mounts all three" — no such binary ever existed
> here; that was the T23 misattribution, F-27.) Either the boxes are stale or
> the README overstates. Since the spec is meant to be the source of truth,
> reconcile before release rather than after.

- [ ] **T21 R4 artifacts.** Run generator on 4.0.1; fix spec-parsing deltas.
  *Accept:* full R4 examples corpus round-trips live; REST suite green on
  `/r4`.
- [ ] **T22 R3 artifacts.** Same for 3.0.2.
  *Accept:* full R3 examples corpus round-trips live; REST suite green on
  `/r3`.
### T23. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

## M6 — Production hardening

### T-validate. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27) — no `fhir` dependency, `validate` feature, or `validate_tests` exists in this workspace

### T-graceful. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

### T24. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

### T25. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)
- [x] **T26 Migrations + upgrade.** `init` stores the map asset in
  fhir_mysql_meta; `init --upgrade` diffs installed vs current maps and
  applies additive DDL (new tables/columns/indexes) in lock-safe chunks;
  destructive steps refuse without --allow-destructive; column type
  changes always demand manual migration. *Accept met:* upgrade test —
  reduced install upgrades to full, data survives, re-upgrade no-ops,
  downgrade guarded then forced.
- [x] **T27 Encrypted connections.** The `serve --tls-cert/--tls-key` HTTPS
  listener, axum-server, and the loopback bind guard this entry used to
  describe are [`fhir-loco`](../fhir-loco/)'s concern — nothing in this
  library binds a socket (F-27). What is real here is store-level:
  `ssl::SslMode` in MySQL's own `--ssl-mode` vocabulary, read from
  `FHIR_MYSQL_SSL_MODE`, defaulting to `VERIFY_IDENTITY`, live-verified by
  `tests/ssl_live.rs` (**F-54**; details under T32).
- [~] **T28 Benchmarks + regression gate.** Done: gated bench harness
  (load throughput, read latency, EXPLAIN audit) + doc/benchmarks.md with
  measured numbers (6,146 res/s; 1.18 ms reads at 100k).
  *Remaining:* CI regression gate against a recorded baseline; comparison
  against the historical jsonb implementation.
- [x] **T29 Book + generated schema docs.** mdBook (9 chapters:
  introduction, getting started, storage model, SQL querying, search,
  versions, operations, architecture, trust boundary — the REST-API chapter
  went with T73); builds locally. Column/table→FHIR-path mapping ships
  inside the map assets themselves. *Remaining nicety:* a rendered
  path→table index page.
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

- [x] **T31 Snapshot reads (R4.5).** The defect was real and this port had it
  live: `get` read the base row and every child table with no enclosing
  transaction, so a reader observed `name` from one version beside `telecom`
  from the next (audit **F-21**). Fixed in the native store: every
  multi-statement read runs in one transaction — `start_transaction`, i.e.
  plain `START TRANSACTION` under InnoDB's default REPEATABLE READ — rolled
  back rather than committed at each exit (`mysql.rs`). The
  `REPEATABLE READ READ ONLY` spelling and the `lib.rs:554`/`:512` citations
  were the PostgreSQL store's; `lib.rs` here is 51 lines of shared vocabulary
  (**F-27** class 3).
  *Accept:* `tests/concurrency.rs::reads_never_tear_under_concurrent_writes`
  (T11.6).
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
  which are libpq names this port does not read. This entry once claimed
  `SslPolicy`, a rustls connector, a `serve` startup guard and a TLS-only CI
  job (**F-27** class 3); the mechanism above replaced them, verified in both
  directions by `tests/ssl_live.rs`, and the conformance matrix records
  `O10.7` as met (`•`).

- [ ] **T33 Atomic conditional interactions (A7.10).** Not started in this
  port: no `conditional_create` or `conditional_delete` exists here at all
  (the conformance matrix says `—`), and the earlier text claiming them
  *done* under `pg_advisory_xact_lock` described the PostgreSQL store
  (**F-27** class 3). When built, match and write must share one transaction
  serialized by this engine's own locking — `GET_LOCK` on a hash of the
  criteria (session-scoped, so it must be released on every path including
  errors) or `FOR UPDATE` — not pg advisory locks.
- [ ] **T34 Audit envelope on history (M3.15, PR12.1–PR12.4).** Half real,
  and the earlier *Done* overstated the other half. Real: `put` and `delete`
  take a caller-supplied `Audit` and record actor, actor source, client,
  request id and reason on every history row, bound into the chain preimage
  (`mysql.rs`; `put(&self, resource, audit)`), and `upgrade` reconciles the
  audit columns against `information_schema.columns` (M14.36). A plain write
  is attributed, not anonymous. Missing, which is why the box is open: the
  `put_audited`/`delete_audited`/`transact_audited`/`conditional_*_audited`
  variants and optimistic concurrency (`expected_version`) do not exist in
  this port. `PrincipalPolicy`, `--trust-proxy` and `--require-principal`
  were server machinery and belong to [`fhir-loco`](../fhir-loco/) (F-27).
### T35. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

### T36. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

### T37. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

### T38. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

### T39. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

- [x] **T40 Diagnostics hygiene (A7.11).** The store half is real:
  `StoreError` distinguishes `Unsupported` (client-safe — it names the
  caller's own parameter, e.g. an unknown resource type or search parameter;
  `lib.rs:35`) from `Other` (internal detail). The OperationOutcome mapping,
  incident ids and response bodies this entry used to describe are the
  server's surface, which is [`fhir-loco`](../fhir-loco/)'s (F-27).

### P1 — missing guarantees

### T41. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27). The store-level disclosure log itself is real — `log_access`/`log_access_batch` and the access-log table, tested in `tests/mysql_store.rs` and `tests/redaction.rs` (T74) — but the `--audit-mode` flags, async queue and `tests/audit_async.rs` described here never existed in this port

- [x] **T42 Tamper-evident history (M3.16, M3.17).** `prev_hash`/`row_hash`
  chain per resource id; per-table `SIGNAL` append-only triggers, proven to
  refuse an UPDATE and a DELETE and to permit a flagged erasure (T63); and
  `verify_audit` walking every chain — a store API, not a CLI. The chain is
  computed **in Rust** by the shared `fhir-store::chain` (**F-07**) over
  `canon::canonicalize` output, with the timestamp rendered in UTC in Rust
  and stored verbatim — see T74. The "computed in SQL over
  `resource::jsonb::text`" description that stood here was the PostgreSQL
  store's (**F-27** class 3). *Accept met:*
  `tests/mysql_store.rs::verify_audit_accepts_a_clean_chain_and_catches_tampering`
  tampers with a history row behind the application's back and verification
  names it.
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
  refers to what was removed — which is what a `chain_witness`/checkpoint
  layer exists to notice. **Neither exists in this port** (`chain_witness`
  and `emit_checkpoint`: zero hits in `mysql.rs`; the matrix rows are `—`) —
  the sentence this one replaces described the ancestor's witness digests
  and `audit_checkpoint` log target as this crate's (corrected 2026-08-06,
  the same class as **F-78**).

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
  environment; a deployment wanting a secrets manager or an HSM wires
  `Store::with_chain_keys` (`mysql.rs:179`) to it from the embedding
  application — there is no CLI in this workspace to surface it (`C0.17`).
- [x] **T43 Worldwide string search (P6.6).** Folding is Rust-side
  (`fhir-mysql-map::fold`) into materialized `_norm` companion columns at
  write time; prefix search is a range predicate against them, and there is
  no SQL fold function — the emitted one was dropped, since it never had a
  caller (T63, M14.5). The `_norm` and exact-comparison columns bind
  `utf8mb4_0900_bin`, MySQL 8's NO PAD binary collation (T70), because the
  range bound is only sound under codepoint order and `:exact` must not pad.
  The ~40 lines of PostgreSQL planner analysis that stood here —
  `fhir_mysql_norm(col)` expression indexes, `text_pattern_ops`,
  `plan_cache_mode`, `COLLATE "C"`, `tests/search_semantics.rs` — were the
  reference port's (**F-27** class 3). `:exact` still deliberately compares
  the *stored* column. Migration: `upgrade` adds the `_norm` columns and
  `backfill_norm` fills them — distinct values, batched, resumable — before
  returning (`tests/upgrade.rs`; **F-15**).

### T44. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)

### T45. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27)
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
  The adversarial tests that keep T31–T42 honest. *Done here:*
  `tests/concurrency.rs` covers torn reads and racing writers getting
  distinct versions with a verifiable chain (T11.6 — not racing conditional
  creates or `If-Match`, since neither operation exists in this port);
  `tests/redaction.rs` covers T11.7. The audit assertions — the envelope, the
  disclosure record, chain verification, tamper detection — live in
  `tests/mysql_store.rs` (T11.8). There is no `audit.rs` or `audit_async.rs`
  in this port — those names, and the async path they tested, were the
  reference port's (**F-27** class 3).
- [x] **T49 Erasure (M3.18).** `fhir-mysql purge` with tombstone rows,
  `--allow-erasure`, and a test that `verify-audit` reports a purge as a
  recorded hole rather than a chain break. *Done:* `Store::purge` and
  `fhir-mysql purge`, the tombstone carrying who/when/why plus the terminated
  chain hash, and the append-only trigger relaxed to permit `DELETE` only
  inside a transaction that sets `fhir_mysql.erasure`. The book states the two
  limits plainly: backups and replicas are outside this, and the guard stops
  accidents rather than the application itself.
### T50. — removed 2026-08-06: misattributed ancestor (REST/CLI) work; the server is [fhir-loco](../fhir-loco/) (F-27). `book/src/trust-boundary.md` does exist, but the entry's substance — a worked `serve` invocation with its flags — was the ancestor server's

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
- [x] **T58 CI/CD on GitHub and Codeberg.** The pipeline files exist in this
  port (`.github/workflows/`, `.woodpecker/`) and provision `mysql:8.4` — not
  the "live-PostgreSQL suite" this entry used to claim (**F-27** class 3).
  Two caveats the tick must not hide: in the monorepo the per-port
  `.github/workflows/` files are **inert** — GitHub reads only the root
  `.github/workflows/`, whose `gates.yml` runs the shared-core and
  doc-example gates (**F-49**) — and the tag/release/SBOM machinery described
  here has not been exercised from this repository. See `doc/ci.md`.

## M14 — MySQL port

Tracks `spec/14-mysql-dialect.md`. The repo began as a rename of the PostgreSQL
original, so every task here is a *departure* from an inherited PostgreSQL
implementation, not new ground.

- [x] **T60 Rename repair.** The initial `fhirpg` → `fhir-mysql` substitution
  rewrote Rust paths, SQL identifiers, the GUC prefix, and env vars into forms
  that are not legal in their respective languages, and the workspace did not
  resolve. Redone with the correct spelling per context.
- [x] **T61 Dialect annex.** `spec/14-mysql-dialect.md`. Records what changes,
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

- [x] **T64 Store layer.** Done — see T74/T75. The native store is
  `store/src/mysql.rs` (2,077 lines) plus `mysql_search.rs` (618) on
  `mysql_async`; the inherited `tokio-postgres` store this entry described
  ("still speak `$n` placeholders, `::jsonb` casts, advisory locks") is
  deleted, and `lib.rs` is 51 lines of shared vocabulary.
- [x] **T65 Surrogate key computation.** Done: `surrogate_key`
  (`mysql.rs:683`) hashes the delimiter-joined natural key in Rust —
  delimited, not concatenated, so `("ab","c")` and `("a","bc")` cannot
  collide (unit test at `mysql.rs:754`) — and the `Ext`/`Deep` inserts fill
  `key_hash` (T74, M14.12).
- [ ] **T66 Decimal sort columns.** `ColTy::Numeric` needs a derived
  `<name>_sort` companion in the generated map, following the pattern `date`
  and `dateTime` already use, so range search has something numeric to index.
  Touches the gen crate, which T63 did not. Until it lands, numeric and
  quantity range search is a known regression from PostgreSQL.
- [x] **T67 Wire the canonicalizer into the chain.** Done: the write path
  computes the chain preimage from `canon::canonicalize` and stores the
  canonical bytes, so what is read back is what was signed (`mysql.rs:897`;
  see T74).
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
  The "store not yet ported (T64), suites self-skip" sentence this entry
  ended on is stale: the native store has since landed (T64/T74/T75) and the
  store suites run for real against `mysql:8.4`. Note also that in the
  monorepo the per-port workflow files are inert — GitHub reads only the root
  `.github/workflows/` (**F-49**).

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
- [ ] **`transact_audited`, `resign_history`.** (`upgrade` is no longer on
  this list — it landed, with `backfill_norm`, at `mysql.rs:298`/`:565`,
  live-verified by `tests/upgrade.rs`; **F-15** closed here. See T90a.)
- [ ] **Decimal sort columns**, as for SQLite; `CAST(… AS DECIMAL(65,30))` is
  correct but unindexed.

### Cross-cutting, all repos

- [x] **Git remotes — resolved (F-11).** The ports were merged into one
  monorepo with a single remote; the ancestor-`origin` warning and the
  `688641a` shared-history question that stood here are moot.
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
- [x] **T90a Backfill the `_norm` columns — done.** The premise ("the SQLite,
  MySQL and MariaDB stores have no `upgrade` yet") is stale: all three now
  have `upgrade` + `backfill_norm`. Here they are `mysql.rs:298`/`:565`,
  live-verified by `tests/upgrade.rs` (8 tests — including that rows written
  before the folded column existed are backfilled, that the backfill is
  resumable, and that a second upgrade is a no-op). **F-15** is closed for
  this port: a database written before the fold fix migrates rather than
  reloads. The principle stands — deploying a new fold without backfilling is
  worse than not fixing it, which is why the backfill runs *inside* `upgrade`
  (M14.37).
