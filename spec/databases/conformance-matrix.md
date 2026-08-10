# Conformance matrix

Non-normative. This records **what is true today**, per port, against the
[normative core](index.md). It is the document `C0.9` requires a conformance
claim to be justified against, and step 4 of §13's audit procedure.

Measured 2026-07-31 by reading each port's `store` crate surface, `ddl.rs`
binding, test directory, and CI configuration; refreshed after the same day's
audit-remediation pass, and again 2026-08-06 (**F-74** — nine cells had gone
stale against the code, most in the "the former scaffolds have no store"
direction).

Test counts below name their unit explicitly: a bare number is the port's
**store crate** unless it says whole-port. (An earlier revision mixed the two
without saying so: mysql/mariadb "102" is whole-port — gen 13 + map 49 +
store 40 — while mssql "33" and oracle "7" are store-crate counts.)

Where this file and a README disagreed, this file was right — that was
[`audit.md`](audit.md) **F-01**, now fixed, so the six READMEs should agree with
this table. If one does not, the README is the defect.

## Conformance level

Per `C0.8`. The level is a claim about what has been **verified for that port**,
not about what its code contains.

| Port | Level | Basis |
| --- | --- | --- |
| `fhir-postgresql` | **Reference** | full store, 11 test files (`audit`, `bench`, `chain_portability`, `concurrency`, `history_page`, `live`, `m2_semantics`, `redaction`, `search_semantics`, `ssl_default`, `upgrade`), 24 test functions. Live PostgreSQL 18 gate **re-run 2026-08-03**: 1,200 live corpus round-trips (400 per release), 0 failures. Until **F-55** that gate could not resolve its inputs at all — `db.sh` pointed at the ancestor project's path — so the live half of this level had no evidence in this repository |
| `fhir-sqlite` | **Store**, nearing Reference | native store; tests incl. concurrency, redaction, round-trip-by-type, and upgrade+backfill, none needing a server; some operations return `Unsupported`. A boolean-token search defect (`active=true` silently matching nothing — SQLite's TEXT/INTEGER affinity rule never converts the word `"true"`) was found and fixed 2026-08-04, **F-71**, adding one test |
| `fhir-mysql` | **Store** | native store + search; **102** tests incl. round-trip-by-type, concurrency, redaction, upgrade+backfill and the new live TLS suite, green against live MySQL 8.4 (measured 2026-08-03). The corpus links this rests on could not resolve until **F-55**. **F-90** (2026-08-10): the *full* schema does not install on stock InnoDB — the widest `value[x]` tables exceed the row-size limit; found by the first full-schema CI install, open, fix scheduled at the generator |
| `fhir-mariadb` | **Store** | native store + search; same suites, **102** tests, green against live MariaDB 11.4 (measured 2026-08-03). The corpus links this rests on could not resolve until **F-55**. **F-90** (2026-08-10): the *full* schema does not install on stock InnoDB — the widest `value[x]` tables exceed the row-size limit; found by the first full-schema CI install, open, fix scheduled at the generator |
| `fhir-mssql` | **Store** | native store + search; **36** tests (13 `mssql_store.rs`, 2 `concurrency.rs`, 2 `redaction.rs`, 6 `roundtrip_types.rs`, 1 `ssl_live.rs`, 12 `upgrade.rs`) green against live `azure-sql-edge`, **0 ignored** (measured 2026-08-10; store **F-65**, `upgrade.rs` closing this port's share of **F-15**, its last three tests F-47 step 4's `path` conversion). `M3.15`/`M3.16`/`M3.17`/`M3.18`/`H5.4`/`R4.5` all now live-verified where they were previously untested or, for `R4.5`, briefly confirmed violated before being fixed in a same-day follow-up pass. `upgrade` is one transaction (`M14.35`) — T-SQL DDL is transactional, unlike MySQL/MariaDB, so a failed upgrade rolls back rather than half-applying. No `conditional_create_audited`, `put_audited`, or `transact_audited` |
| `fhir-oracle` | **Store** | native store + search; **20** tests (**7** `oracle_store.rs`, **12** `upgrade.rs`, **1** `root_extension.rs`) green against live `gvenzl/oracle-free:23-slim-faststart`, **0 ignored**, run `--test-threads=1` (store **F-68**; `upgrade`+`backfill_norm` 2026-08-09 closing **F-15**'s last port; the `path` conversion and the **F-85** root-level-extension fix 2026-08-10, F-47 step 5). The DDL emitter was already Oracle and executed (**F-08**); connecting a store found four real defects (paragraph below), and the upgrade pass added three normative rules of its own (`M14.35`–`M14.37`: resumable non-transactional DDL, chunked meta past `ORA-01461`, ROWID-keyset backfill). No `search_page` concurrency coverage, no `conditional_create_audited`/`put_audited`/`transact_audited`, and `R4.5` is a confirmed, not merely unverified, gap |

`fhir-mssql` moved from Scaffold to Store in the same pass that gave it a
store at all: `search`/`search_full`/`search_page` were built and live-tested
alongside the existing `put`/`get`/`delete`/`history`/`vread`/`verify_audit`/
`purge`/`log_access`, and five real defects surfaced by running that work live
were fixed (`F-65`) — a cross-column collation conflict that broke every
chained reference search, `verify_audit` never checking the keyed tag it
wrote, `connect` returning `Ok` for an unreachable server, `purge`
double-counting erased versions, and a torn read under concurrent writers
(`R4.5`). The last of those needed two tries: `READ_COMMITTED_SNAPSHOT` alone
was tried live first and did not stop the torn read, and `SET TRANSACTION
ISOLATION LEVEL SNAPSHOT` (backed by a dedicated database, since `master`
refuses the option) is what actually did. `C0.9` requires the level be
justified by tests that *ran*: all of the above did, and the live suite is
green with nothing `#[ignore]`d.

`fhir-oracle` moved from Scaffold to Store the same way: **F-66** had recorded
a store that compiled but had never connected to a database, on the honest
grounds that no Instant Client existed on this host. That blocker is gone —
Oracle Instant Client for macOS arm64 downloads directly, no login required —
and running the store live against `gvenzl/oracle-free:23-slim-faststart`
found four real defects no amount of reading would have surfaced (**F-68**):
Oracle folds an *unquoted* username to uppercase for session identity
regardless of how `CREATE USER` quoted it, so a lowercase schema
(`M14.5`'s "three users" decision, as written) made every DDL statement
`ORA-01031` against a session that was really `"R5"` — fixed by creating
users unquoted and setting the map's schema to match, uppercase; a
speculative `SET TRANSACTION READ ONLY` for `R4.5` (`M14.19`) turned out to
fail with `ORA-01466` on any session that had ever run DDL, reproduced with a
minimal 3-statement probe, and was removed rather than shipped broken —
`R4.5` regresses from "believed addressable" to a confirmed, open gap; two
`insert_row` call sites double-qualified the schema (`ORA-00926`, from
passing an already-qualified table string alongside a separate schema
argument); and timestamp/date columns bound as plain strings relied on
Oracle's session `NLS_TIMESTAMP_FORMAT` rather than ISO 8601
(`ORA-01843`), fixed by binding real `chrono` types instead. A fifth defect —
token search binding the string `"true"` against a `NUMBER(1)` boolean
column (`ORA-01722`, Oracle does not coerce `'true'`/`'false'` to a number
the way SQL Server/MySQL do) — was found by the same live test suite and
fixed by adding an `i64` bind path (`Bind::I64`) used whenever the target
column is `ColTy::Bool`.

## Store operations

`•` implemented · `~` implemented with a stated limitation · `—` absent

| Operation | pg | sqlite | mysql | mariadb | mssql | oracle |
| --- | :-: | :-: | :-: | :-: | :-: | :-: |
| `init` | • | • | • | • | • | • |
| `init --upgrade` | • | • | • | • | • | • |
| `put` | • | • | • | • | • | • |
| `put_audited` | • | • | — | — | — | — |
| `get` | • | • | • | • | • | • |
| `delete` | • | • | • | • | • | • |
| `delete_audited` | • | • | — | — | — | — |
| `history` | • | • | • | • | • | • |
| `history_page` (type/system) | • | • | — | — | — | — | added 2026-08-10 for `fhir-loco`'s `SV2.17`; merged per-type history, newest first, `_since` at-or-after. pg's landed with the multi-port wiring (`SV1.10`), live-tested |
| `vread` | • | • | • | • | • | • |
| `search` / `search_full` | • | • | • | • | • | • |
| `search_page` (cursor) | • | • | • | • | • | ~ |
| `transact_audited` | • | ~ | — | — | — | — |
| `conditional_create` | • | • | — | — | — | — |
| `conditional_delete` | • | • | — | — | — | — |
| `log_access` | • | • | • | • | • | • |
| `verify_audit` | • | • | • | • | • | • |
| `purge` (erasure) | • | • | • | • | • | • |
| `emit_checkpoint` | • | • | — | — | — | — |
| `chain_witness` | • | — | — | — | — | — |
| `resign_history` | • | — | — | — | — | — |
| `backfill_norm` | • | • | • | • | • | • |
| `export` | — | — | — | — | — | — |

The `init --upgrade`/`backfill_norm` rows are complete as of 2026-08-09:
`fhir-oracle`, the last `—`, now has both — `tests/upgrade.rs`, 9 live
tests; see this port's paragraph below and **F-15**, now closed on all six.

`put_audited` and `delete_audited` have their own rows as of 2026-08-06: an
earlier revision folded each into one cell with its plain sibling and showed
the union as `•` for all six, while the `_audited` variants exist only in
`fhir-postgresql` and `fhir-sqlite` — as this file's own port paragraphs
already said (**F-74**). (`search_page`'s pg cell was briefly challenged
during the same audit on the theory that pg pages by offset only — checked
against the code, `search_page` takes `after_id: Option<&str>`, a keyset
cursor for the default id ordering, so `•` stands.)

`fhir-mssql` is `•` in both rows as of this revision, closing this port's share
of **F-15**: `MsSqlStore::upgrade`/`backfill_norm` diff the installed map asset
against the current one, apply the DDL, and backfill folded search columns,
live-verified against `azure-sql-edge` by `tests/upgrade.rs` (9 tests). Until
this revision every statement of the upgrade DDL itself was rejected by SQL
Server — MySQL's `ADD COLUMN` spelling (**F-25**), and a `NOT NULL` column with
no default added to tables that by definition have rows (**F-26**). Both were
fixed and unit-tested at the map layer but had never been executed against a
server; this revision is what finally executed them, live, for the first time.
Two more defects surfaced doing it, neither anticipated by the DDL emitter's
own unit tests: `DROP TABLE` on a table a live `FOREIGN KEY` still references
fails with error 3726 regardless of destructive-table drop order (fixed by
ordering children before their base table, `M14.36`), and this port's `init`
previously stored only a bare `checksum`, not the map asset an upgrade needs to
diff against — now fixed to store `map_asset`/`fhir_version` alongside it.
Unlike `fhir-mysql`/`fhir-mariadb`, the DDL apply is one transaction: SQL
Server's DDL is transactional, so a failed upgrade rolls back rather than
half-applying (`M14.35`).

`fhir-oracle` is `•` in both rows as of 2026-08-09 — the last port, fully
closing **F-15** (and **F-47** step 1): `OracleStore::upgrade`/
`backfill_norm`, live-verified by `tests/upgrade.rs` (9 tests,
mutation-checked — skipping the backfill makes the seeded patient unfindable
by their own name). Three things are this engine's own, now normative in its
annex: the upgrade is **resumable rather than transactional** — Oracle has no
transactional DDL, so every statement is wrapped to tolerate having already
run and the recovery for a partial upgrade is rerunning `upgrade` (`M14.35`,
the third answer after mssql's one-transaction and mysql's reported-partial);
the ~1 MB map asset cannot bind as one string (`ORA-01461`) and is stored as
chunked meta rows (`M14.36`); and the backfill pages by ROWID keyset because
a `CLOB` source column can be neither `DISTINCT`ed nor `=`-compared
(`ORA-00932`/`ORA-22848`, `M14.37`) — the values-based loop the other five
ports share is illegal here.

`fhir-sqlite`'s `transact_audited` returns `Unsupported` rather than emulating
atomicity by compensation, which is the right answer: a FHIR transaction Bundle
is atomic by definition, and a compensating unwind is not — readers between ops
observe a half-applied bundle, and a process dying mid-unwind leaves partial
state permanently.

`fhir-mssql`'s `get` is `•`: `R4.5` was confirmed violated under concurrent
writers by a live torn read, then fixed in a same-day follow-up pass — see
`M14.25` and **F-65**. `put`/`delete` were always `•`: the requirement they
carry, `H5.4` write serialization, was live-verified from the start (8 of 8
racing writers got distinct consecutive versions and a chain that still
verifies); it was specifically the multi-statement *read* that tore.

`fhir-oracle`'s `get` is `•` despite `R4.5` being an open gap there (see
Core requirements below): the operation itself works and is exercised live
by every test in the suite, it simply carries no snapshot-isolation
protection under concurrent writers — the same distinction the table draws
for `P6.4a`, a `•`/`~` operation cell is not a claim about every requirement
that touches it. `search_page` is `~`: the function exists and is exercised
transitively (`search` calls into the same query builder), but no test calls
it directly with a cursor, so its paging-specific behaviour is unverified.
`put`/`delete` use `SELECT … FOR UPDATE` for `H5.4` (`M14.20` discharged in
code), but — unlike mssql — no concurrent-writer test exists for this port,
so the mechanism is present and unverified under contention rather than
live-confirmed.

## Core requirements

`•` satisfied · `~` partially · `!` violated · `?` unverified · `—` not applicable

| Requirement | pg | sqlite | mysql | mariadb | mssql | oracle | Note |
| --- | :-: | :-: | :-: | :-: | :-: | :-: | --- |
| `S1.1` three FHIR versions | • | • | • | • | • | • | assets committed for r3/r4/r5 in all six |
| `S1.4` engine floor declared | • | • | • | • | • | • | mssql 2016 (`M14.3`), oracle 12.2 (`M14.2`) — F-09 fixed |
| `G2.1`–`G2.3` generation | • | • | • | • | • | • | `gen/src` identical across ports |
| `G2.4` identifier budget | • | • | • | • | • | • | 63 everywhere; now justified on Oracle by the 12.2 floor |
| `G2.5` idempotent install | • | • | • | • | ? | ? | staged-schema on pg |
| `M3.4a` `ords` domain | • | • | • | • | ? | ? | text image on the non-array engines |
| `M3.6a` `Numeric` not fixed-scale | • | • | • | • | • | • | Oracle binds `VARCHAR2(64 CHAR)`, not `NUMBER` (`M14.7`) — `NUMBER` would normalize `1.50` to `1.5` and break `M3.6` |
| `M3.6b` `TextC` binary NO PAD | • | • | • | • | • | • | Oracle binds `VARCHAR2`, not `CHAR` — `CHAR` is blank-padded (`M14.10`); the AL32UTF8 default collation is already byte-exact |
| `M3.6c` `Jsonb` not re-normalizing | ~ | • | • | • | • | • | Oracle binds `CLOB`, never its JSON type (`M14.12`): JSON re-normalizes, so the bytes read back would not be the bytes the chain signed. pg still binds `jsonb` (`M14.13`), but since **F-07** no digest depends on how it renders; it can still alter a value on the way *in* |
| `M3.15` audit envelope | • | • | • | • | • | • | mssql and oracle now both write and read it live (`disclosures_are_recorded`) |
| `M3.16` chain, two families | • | • | • | • | • | • | mssql and oracle: `verify_audit` recomputes both SHA-256 and SHA-3-256 live, 0 breaks on a clean chain. mssql's suite also flips a byte and confirms both families break; oracle's does not yet (same `T11.8` gap as sqlite/mysql/mariadb) |
| `M3.16b` app-computed digest | • | • | • | • | • | • | **F-07** fixed — pg's pre-image is now Rust `canon.rs`. mssql and oracle both use the same shared `chain::preimage`/`link` |
| `M3.16c` checkpoint | • | ~ | — | — | — | — | sqlite emits but has no `chain_witness` |
| `M3.16d` re-sign | • | — | — | — | — | — | mssql's `verify_audit` now *reads* the countersign table (**F-65**) but nothing writes to it — `resign_history` does not exist |
| `M3.17` append-only trigger | • | • | • | • | • | ~ | oracle: a forbidden `UPDATE` and `DELETE` were each refused on a live 26ai history table and a declared erasure then succeeded (`M14.29`) — verified by hand (**F-51**), and now also exercised live through the store's own erasure-escape path (`purge_erases_history_and_leaves_a_verifiable_hole`); still `~` not `•` because no test drives a store-mediated *forbidden* delete and checks it is refused. mssql: live-verified — a raw `DELETE` on history is refused without the `SESSION_CONTEXT` erasure flag, `purge` succeeds with it |
| `M3.18` erasure + tombstone | • | • | • | • | • | • | mssql and oracle: both have `purge_erases_history_and_leaves_a_verifiable_hole`, live |
| `R4.2` lossless round-trip | • | • | • | • | • | • | **F-42**: 7,399 spec examples round-trip in **every** port (r3 1664 / r4 2911 / r5 2824, 0 failures) — map layer, no store needed. **F-20** — three ports broke on bool/int (mysql/mariadb also on dates, by panic); fixed and verified live on each. mssql had the same defect class (`cell_text`, `hist_entry`, `[ords]`), found and fixed before any store code shipped, and `roundtrip_types.rs` now guards it. The **live** round-trip through PostgreSQL is separate evidence and was unrunnable until **F-55**; it now passes 1,200 examples, 400 per release |
| `R4.5` snapshot reads | • | • | • | • | • | ! | **F-21** — all three non-pg ports tore until fixed; each now reads in a transaction, tested. mssql: a bare transaction wrap was tried and, live, still tore under this engine's default `READ COMMITTED` — fixed with `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` backed by `ALLOW_SNAPSHOT_ISOLATION` on a dedicated database (**F-65**); `READ_COMMITTED_SNAPSHOT` alone was tried first and live-confirmed insufficient. oracle: `M14.19`'s presumed answer, `SET TRANSACTION READ ONLY`, was tried and found to fail live with `ORA-01466` on any session that had ever executed DDL — reproduced with a minimal 3-statement probe with no application logic involved — so the call was removed rather than shipped broken; `get` currently reads with no snapshot-isolation protection at all (**F-68**) |
| `R4.7` consumption audit | • | • | • | • | ~ | ~ | in shared `reconstruct.rs`; its error was flattened to a string on all three non-pg ports until **F-23**. mssql not separately re-examined this pass |
| `H5.4` serialized `version_id` | • | • | • | • | • | ? | **F-24** — mysql/mariadb had no row lock (1 of 8 writers succeeded); now `FOR UPDATE`, 8 of 8. mssql: `WITH (UPDLOCK, ROWLOCK)`, live-verified 8 of 8. oracle: `SELECT … FOR UPDATE` (`M14.20` discharged in code, native Oracle locking syntax), but no concurrent-writer test exists for this port — mechanism present, unverified under contention |
| `P6.1` params compile | ~ | ~ | ~ | ~ | ? | ? | **92.4%** of R5 after **F-38** (was 94.8%, but 51 of those silently dropped a `where()` value restriction). Shipped assets still carry the old compilation until regenerated |
| `P6.4a` indexable as bound | • | • | • | • | ! | ~ | mssql drops token index on `NVARCHAR(MAX)`. oracle: every one of R5's 1,947 search targets is now indexed via its adjunct — `search_index_gaps` returns **0** — and the indexes were created on a live 26ai (9,479 of them). `~` not `•` because the **query** half (`U6`/`U7`) is unexercised live — the store exists now (**F-68**), the adjunct query path is not yet live-tested — and because no test re-runs the full install (**F-51**, narrowed by F-68) |
| `U1`–`U5`, `U9` adjunct channel | n/a | n/a | n/a | n/a | • | • | map, generation, shredding; `TEXT_ADJUNCTS` false on the four (`U9` forbids them there). Confirmed installed on SQL Server: `binary(32)` digests, `nvarchar(450)` bounded |
| `U11` every search-reachable column | n/a | n/a | n/a | n/a | • | ? | the generator walks `string`, `token`, `uri` and `reference` targets **and** the extension and deep tables (`url`, `v_text` both; `leaf` digest-only) — **F-46**. `tests/adjuncts_in_ddl.rs` asserts map and DDL agree over every table: 3,713 columns on mssql, mutation-verified. mssql `•` because the schema installed on a live engine; oracle `?` — same generated code, no engine to try it on |
| `U11a` map and DDL agree | • | • | • | • | • | • | `tests/adjuncts_in_ddl.rs` in all six — **not** `n/a` on the four with no adjuncts: there the test asserts the count is *zero*, which is the claim that needs checking. Mutation-verified both ways — deleting the emission fails it, forcing `needs_adjunct` false trips the vacuity guard |
| `U1a` dialect half of the trigger | n/a | n/a | n/a | n/a | • | • | `ddl::needs_adjunct` gates on the column's type; without it a token search over a `Bool` grew two columns nothing could read |
| `U2a`/`U2b` adjuncts match the operation | n/a | n/a | n/a | n/a | • | • | measured on R5: **2900** adjuncts / 3713 columns — 813 both (string/uri, and `url`/`v_text`), 2087 digest-only (token/reference, and `leaf`), 0 bounded-only. 2110 come from the `ColTy` path, 790 from the extension and deep tables (**F-46**). `Adjuncts` records which exist |
| `U4a` SHA-256, 32 bytes binary | ? | ? | ? | ? | ? | ? | `digest()` returns `[u8; 32]`, mutation-verified; the **binding** is unproven everywhere — the four ports never materialize the column, and of the two that do (both Store level now), mssql's builder does not query adjuncts and oracle's `RAW(32)` bind is unexercised live |
| `U6`, `U7` confirm-after-match | n/a | n/a | n/a | n/a | — | ? | both ports have stores and search builders now (**F-65**, **F-68**). mssql `—`: its builder states plainly that adjuncts "are not wired into `TargetKind` at all — no port queries them yet". oracle `?`: `oracle_search.rs` compares a client-computed SHA-256 against the digest adjunct as `RAW(32)`, but its own module doc records that path as unexercised by the live suite |
| `U8` mutation-verified | — | — | — | — | ~ | ~ | fold-level invariants verified by mutation; the search-result assertion `U8` asks for needs `U6`/`U7` |
| `U10` annex record | n/a | n/a | n/a | n/a | • | • | `M14.32`/`M14.33`; `M14.26`/`M14.27` |
| `U12`/`U12a` bounded beats adjuncts | • | • | • | • | • | • | closed by **F-47** (six steps, 2026-08-09/10). `path` was the one offender: the four `TEXT` ports satisfy `U12` natively (`TEXT` indexes there, and `path` never had adjuncts); mssql and oracle now bind `NVARCHAR(path_bound)`/`VARCHAR2(path_bound CHAR)` — fresh installs via `create_table`, existing ones via live-verified upgrade conversions (`M14.37`; `M14.38`, add-copy-drop-rename, resumable). `path_bound` is `U12a`'s declared capacity limit, recorded per release in the asset (192/192/384 for R3/R4/R5), enforced loudly at shred time, tested in all six (`gen/tests/path_bound.rs`). `v_kind` proved bounded everywhere already (the finding's table was stale). oracle's `"path"` is nullable by design — NULL is the empty attach path (`M14.39`, **F-85**) |
| `P6.6` fold in Rust | • | • | • | • | • | • | `fold.rs` identical across ports |
| `P6.8` parameter binding | • | • | • | • | — | — | fuzz seed corpus committed in all six |
| `O10.4a` backfill on fold change | • | • | • | • | • | • | **F-15** fixed on all six ports, live-verified by a `tests/upgrade.rs` on each; oracle last, 2026-08-09 (**F-47** step 1) |
| `O10.7` encrypted transport | • | — | • | • | ! | ? | all three networked ports **default to verifying**. pg since **F-17** (`tests/ssl_default.rs`); mysql/mariadb since **F-54**, which also had to enable the `rustls-tls` Cargo feature — `minimal` excluded TLS entirely. Live-verified on MySQL 8.4 and MariaDB 11.4 by asserting `VERIFY_IDENTITY` **rejects** a self-signed certificate; mutation-verified both ways. `—` for SQLite (embedded file, no connection); oracle `?` — the store exists now (**F-68**) but transport security is undecided (`M14.22`). mssql `!`, not unverified: `tests/ssl_live.rs` now proves the trust/no-trust *mechanism* works (`TrustServerCertificate=false` reproducibly rejects `azure-sql-edge`'s self-signed certificate; `=true` accepts it) — but the certificate-parsing code in that same dependency chain (`rustls-webpki 0.101.7`) carries three unpatched CVEs, now confirmed reaching the shipping `fhir-mssql-store` crate rather than only a dev-dependency as `deny.toml` used to (wrongly) claim. `native-tls` was tried as an escape and fails the handshake outright on this host. See **F-67** |
| `O10.10` supply-chain evidence | • | • | • | • | • | • | `deny.toml` + CI in all six |
| `O10.12` CI runs target engine | ~ | ~ | ~ | ~ | ~ | — | each port's CI now lives at the repository root (`<port>-ci.yml`, F-49 closed 2026-08-06) and provisions the right engine (mssql SQL Server 2022; oracle's gate stays removed rather than faked, F-06). Still `~`, not `•`: no hosted run has executed yet — the cells turn `•` when the first push runs them green |
| `T11.1` corpus round-trip | • | • | • | • | ? | ? | |
| `T11.2` live gate | • | • | • | • | • | • | mssql: 36 of 36 store-crate tests green against live `azure-sql-edge`, 0 `#[ignore]`d, plus the DDL test (earlier revisions said 23, then 33 — counts from before `upgrade.rs` grew, F-74; the last three are F-47 step 4's conversion tests). oracle: 20 of 20 store-crate tests (7 `oracle_store.rs`: `init`/`put`-`get`/rewrite/`history`-`vread`-`delete`-`verify_audit`/`purge`/`search`/`log_access`; 12 `upgrade.rs`; 1 `root_extension.rs`) green against live `gvenzl/oracle-free:23-slim-faststart`, 0 ignored, run `--test-threads=1` (**F-68**, **F-15**, **F-85**) |
| `T11.6` concurrency adversarial | • | • | ~ | ~ | • | — | suites now on all five; mysql/mariadb cover torn reads and version assignment but cannot cover conditional ops or `If-Match` — neither is implemented there. mssql: both torn-read (`R4.5`) and version-assignment (`H5.4`) cases pass live, 0 `#[ignore]`d — the torn-read case failed on its first live run and is now fixed (**F-65**). oracle has no `concurrency.rs` yet — `R4.5` is a known-open gap there and `H5.4` is unverified under contention (see above) |
| `T11.7` redaction | • | • | • | • | • | — | `redaction.rs` on all five; found **F-23** on three of them. mssql: run at `DEBUG`, not `TRACE` — `tiberius`'s own packet tracing logs raw row content at `TRACE`, outside this store's control, so that ceiling cannot be promised (see `redaction.rs`'s module doc). oracle has no `redaction.rs` yet |
| `T11.8` audit assertions | • | ~ | ~ | ~ | • | ~ | corrected: all assert chain verification, disclosure logging, append-only, and erasure. Missing on sqlite/mysql/mariadb/oracle: per-algorithm independent tamper detection, and truncated-chain-vs-checkpoint. mssql now asserts per-algorithm detection across all **three** signals (sha256, sha3-256, hmac-sha256) — the hmac check did not exist before this pass (**F-65**) — but still has no checkpoint |
| `T11.9` fuzzing run | ? | ? | ? | ? | ? | ? | targets committed; not shown to run in CI |
| `T11.13` skips fail where promised | • | • | ? | ? | • | • | sqlite needs no server, so nothing skips; mysql/mariadb suites still self-skip without a DSN. mssql and oracle: `eprintln!` + return only when the three env vars are absent; once set, any real failure panics via `expect`/`assert!` rather than being swallowed |
| `T11.14` ignored tests tracked | • | • | • | • | • | • | Oracle's eleven ignored tests have now been **replaced** with Oracle-asserting ones (**F-08**); the crate has 48 tests, 0 ignored |
| `T11.15` tests are deterministic | ? | ? | ? | ? | ~ | ~ | new requirement (**F-52**). mssql's `mssql_ddl.rs` specifically was flaky two runs in three, fixed, and now passes 5/5 in isolation — but running the *full* live suite (`scripts/db.sh test`, no `--test-threads=1`) against `azure-sql-edge` reproduced live-server contention twice while verifying the `upgrade`/`backfill_norm` work (**F-15**): a different unrelated test deadlocked (SQL Server error 1205) on one run, four `upgrade.rs` tests failed on another — both times every failing test passed cleanly rerun alone with `--test-threads=1`. So `~`, not `•`: individual tests are deterministic, the suite as a whole is not safe to run at full parallelism against one shared server instance. oracle is `~` for the same reason: 20/20 green serially, but every test shares the one uppercase `R5` schema (`M14.5`), and a full-parallelism run against a fresh container failed 6 of 7 — run `--test-threads=1`. The others are `?` — not suspected, but no port has been run repeatedly enough to say |
| `X15.1` shared core identical | • | • | • | • | • | • | `scripts/check-shared-core.sh`, 100 files, token-based (`X15.1a`, F-48). Gated in CI since `gates.yml` and its inputs were committed (`60bfcbe`); the per-family CI consolidation followed on 2026-08-06, closing **F-49** |
| `X15.2` canonical form in Rust | • | • | • | • | • | • | **F-07** fixed — `canon.rs` ported into pg; gate now has 0 exemptions |
| `X15.6` annex covers checklist | • | ~ | ~ | ~ | • | • | pg written (F-14), mssql/oracle rewritten (F-16); sqlite/mysql/mariadb annexes predate the checklist |
| `X15.9` annex ratified | ! | ! | ! | ! | ! | ! | all six are marked *proposed* |
| `X15.11` cross-port chain verify | • | • | • | • | — | — | **F-07** fixed; pg proven by `chain_portability.rs`, which recomputes a chain from the exported rows alone |
| `W16.3` crate description | • | • | • | • | • | • | F-02 fixed |
| `W16.6` CI checks shared core | • | • | • | • | • | • | `scripts/check-shared-core.sh` — F-10 fixed |
| `W16.8` docs not substituted | • | • | • | • | • | • | READMEs rewritten (F-01); books' engine substitution corrected (F-56; oracle's missing banner added under F-83) |
| `W16.9` examples runnable | • | • | • | • | • | • | READMEs now show library usage; signatures checked against source |
| `W16.15` git remote correct | • | • | • | • | • | • | **F-11** resolved by the monorepo merge — one remote, `fhir-rust/fhir-rust` |

## A correction

An earlier revision of this table marked `T11.8` as `?` for sqlite, mysql, and
mariadb on the reasoning "`audit.rs` on pg only". That was wrong: all three
assert chain verification, disclosure logging, append-only enforcement, and
erasure — inside their `*_store.rs` files rather than in a file named
`audit.rs`. Judging coverage by filename over-reported the gap.

The row now reads `~` with the specific remaining gaps named. Over-reporting a
gap is a smaller error than under-reporting one, but it is the same kind of
error, and this table is the document that is supposed to be checkable.

## How to read the `?` column

`?` is not a soft `•`. It means the requirement is plausibly satisfied by shared
code and **nothing in that port's test suite demonstrates it**. `R4.5`, `H5.4`,
`T11.6`, `T11.7`, and `T11.8` are the ones that matter most: they are the
concurrency and audit guarantees, they are the requirements §13 maps to HIPAA
§164.312(b) and (c), and outside `fhir-postgresql` there is no test file that
exercises any of them.

A port at Store level with `?` in those rows can hold data. Whether it can hold
*patient* data is the question those tests exist to answer, and for four of the
six ports it is currently unanswered rather than answered yes.

The documentation pass did not change a single `?` in those rows — it fixed
documentation, metadata, annexes, and pipelines, none of which is evidence about
concurrency or audit behaviour. **Writing the tests did.** Porting
`roundtrip_types.rs`, `concurrency.rs`, and `redaction.rs` to sqlite, mysql, and
mariadb turned eleven `?` cells into `•` and found five defects doing it
(**F-20** to **F-24**), four of them High.

The `?` cells that remain outside `fhir-postgresql` are now concentrated in
`T11.8` and `T11.9`, and in `fhir-oracle`'s concurrency rows (`H5.4`,
`T11.6`) — no port remains at Scaffold; `fhir-oracle` reached Store the same
way mssql did, by connecting a store and running it (**F-68**).

## What would move each port up a level

- **`fhir-sqlite` → Reference.** `concurrency.rs` and `redaction.rs` are done
  (and found four defects doing it), and `upgrade` + `backfill_norm` now close
  this port's quarter of **F-15**. Remaining: the two `T11.8` gaps above,
  `chain_witness`, and resolving `transact_audited` by splitting `put`/`delete`
  so their bodies run inside a caller-supplied `BEGIN IMMEDIATE`.
- **`fhir-mysql`, `fhir-mariadb` → Reference.** All four suites are ported and
  green; doing so found **F-20**, **F-21**, **F-23**, **F-24**, and — writing
  the upgrade path — **F-28**. `upgrade` and `backfill_norm` now exist and are
  verified live, closing **F-15** here. What remains is missing *features*
  rather than unverified ones: no `put_audited` and no `expected_version`
  anywhere in either crate, so there is no optimistic concurrency to test;
  likewise no `transact_audited`, no conditional create/delete, no
  `emit_checkpoint`. Also the `T11.8` gaps shared with sqlite.
- **`fhir-mssql` → Reference.** Reached Store this pass (**F-65**): a real
  `tiberius` store with search, live-tested, including `R4.5` — fixed in a
  same-day follow-up once the first attempt (`READ_COMMITTED_SNAPSHOT` alone)
  was tried live and found insufficient; `SET TRANSACTION ISOLATION LEVEL
  SNAPSHOT` backed by a dedicated database is what actually works. `O10.7`'s
  verification mechanism is now confirmed live (**F-67**), but the same pass
  found the four TLS advisories `deny.toml` had been ignoring now reach the
  shipping store crate, not just a dev-dependency as previously believed —
  `native-tls` was tried as a fix and fails the handshake on this host, so
  this is a standing risk needing an owner decision, not the last item on a
  checklist. `upgrade`/`backfill_norm` are also now done, closing this port's
  share of **F-15** — live-verified by `tests/upgrade.rs` (9 tests), and
  genuinely atomic (`M14.35`) unlike `fhir-mysql`/`fhir-mariadb`'s equivalent,
  since T-SQL DDL is transactional. What remains, in rough order: the `M14.34`
  decision above; verification against full SQL Server rather than
  `azure-sql-edge` (`M14.31`); and the unindexable-column decision (`M14.16`).
- **`fhir-oracle` → Reference.** Reached Store this pass (**F-68**): Oracle
  Instant Client installed on the host, a real `scripts/db.sh` and
  `tests/oracle_store.rs` built, and four real defects found and fixed by
  running the store live for the first time (uppercase-schema case-folding,
  `R4.5`'s `SET TRANSACTION READ ONLY` failing with `ORA-01466`, a
  double-schema-qualification bug, and a timestamp-binding bug). What
  remains, in rough order: an actual `R4.5` fix (the annex now needs a new
  answer, not just a name — `M14.19`); a `concurrency.rs` to verify `H5.4`
  under contention, which the code already attempts via `FOR UPDATE`; a
  `redaction.rs`; and `O10.7` transport security, currently undecided
  (`M14.22`). (`upgrade`/`backfill_norm` were on this list until
  2026-08-09 — done and live-verified, closing **F-15**'s last port.)
- **`fhir-postgresql` → clean Reference.** The chain pre-image now comes from
  `canon.rs` (**F-07** fixed, `M14.12`), the dead `_norm` SQL function is gone
  (**F-18** fixed), and the TLS default verifies (**F-17** fixed 2026-08-03,
  `tests/ssl_default.rs` — an earlier revision of this bullet still called it
  an open owner decision while the `O10.7` row above said it was fixed,
  **F-74**). What is left: the shared `T11.8` gaps (truncated-chain-vs-
  checkpoint is the one pg itself lacks a direct test for) and deeper chained
  reference search.

---

Part of the [fhir-databases specification](index.md).
