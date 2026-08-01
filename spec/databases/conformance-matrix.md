# Conformance matrix

Non-normative. This records **what is true today**, per port, against the
[normative core](index.md). It is the document `C0.9` requires a conformance
claim to be justified against, and step 4 of §13's audit procedure.

Measured 2026-07-31 by reading each port's `store` crate surface, `ddl.rs`
binding, test directory, and CI configuration; refreshed after the same day's
audit-remediation pass.

Where this file and a README disagreed, this file was right — that was
[`audit.md`](audit.md) **F-01**, now fixed, so the six READMEs should agree with
this table. If one does not, the README is the defect.

## Conformance level

Per `C0.8`. The level is a claim about what has been **verified for that port**,
not about what its code contains.

| Port | Level | Basis |
| --- | --- | --- |
| `fhir-postgresql` | **Reference** | full store, 8 test files incl. concurrency, audit, redaction, upgrade, bench; live PostgreSQL 18 gate |
| `fhir-sqlite` | **Store**, nearing Reference | native store; 61 tests incl. concurrency, redaction, round-trip-by-type, and upgrade+backfill, none needing a server; some operations return `Unsupported` |
| `fhir-mysql` | **Store** | native store + search; 97 tests incl. round-trip-by-type, concurrency, redaction and upgrade+backfill, green against live MySQL 8.4 |
| `fhir-mariadb` | **Store** | native store + search; same four suites, 97 tests, green against live MariaDB 11.4 |
| `fhir-mssql` | **Scaffold** | T-SQL DDL emitter only; no store. Its live test now runs against SQL Server 2022 in CI (F-06 fixed), so **Schema** level is reachable as soon as one green run exists |
| `fhir-oracle` | **Scaffold** | DDL emitter is verbatim MySQL (F-08); no store; no map tests |

`fhir-mssql` is held at Scaffold until its repointed pipeline has actually run.
The T-SQL DDL test was verified by hand against `azure-sql-edge`, and CI now
provisions SQL Server 2022, invokes the test target that exists (`mssql_ddl`,
not `mysql_ddl`), and sets `FHIR_MSSQL_REQUIRE_DB=1` so an absent database fails
rather than skips. What is missing is a green run to point at — and `C0.9`
requires the level be justified by tests that *ran*, not by a pipeline that
should now work.

## Store operations

`•` implemented · `~` implemented with a stated limitation · `—` absent

| Operation | pg | sqlite | mysql | mariadb | mssql | oracle |
| --- | :-: | :-: | :-: | :-: | :-: | :-: |
| `init` | • | • | • | • | — | — |
| `init --upgrade` | • | • | • | • | — | — |
| `put` / `put_audited` | • | • | • | • | — | — |
| `get` | • | • | • | • | — | — |
| `delete` / `delete_audited` | • | • | • | • | — | — |
| `history` | • | • | • | • | — | — |
| `vread` | • | • | • | • | — | — |
| `search` / `search_full` | • | • | • | • | — | — |
| `search_page` (cursor) | • | • | • | • | — | — |
| `transact_audited` | • | ~ | — | — | — | — |
| `conditional_create` | • | • | — | — | — | — |
| `conditional_delete` | • | • | — | — | — | — |
| `log_access` | • | • | • | • | — | — |
| `verify_audit` | • | • | • | • | — | — |
| `purge` (erasure) | • | • | • | • | — | — |
| `emit_checkpoint` | • | • | — | — | — | — |
| `chain_witness` | • | — | — | — | — | — |
| `resign_history` | • | — | — | — | — | — |
| `backfill_norm` | • | • | • | • | — | — |
| `export` | — | — | — | — | — | — |

A `—` in the `init --upgrade` row means no store calls it, not that the DDL
behind it is sound. `fhir-mssql` emits upgrade DDL from its map crate, and until
this revision every statement of it was rejected by SQL Server — MySQL's
`ADD COLUMN` spelling (**F-25**), and a `NOT NULL` column with no default added
to tables that by definition have rows (**F-26**). Both are fixed and
unit-tested; neither has been executed against a server, because no store exists
to execute it.

`fhir-sqlite`'s `transact_audited` returns `Unsupported` rather than emulating
atomicity by compensation, which is the right answer: a FHIR transaction Bundle
is atomic by definition, and a compensating unwind is not — readers between ops
observe a half-applied bundle, and a process dying mid-unwind leaves partial
state permanently.

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
| `M3.6a` `Numeric` not fixed-scale | • | • | • | • | • | ! | Oracle emits MySQL `TEXT` |
| `M3.6b` `TextC` binary NO PAD | • | • | • | • | • | ! | Oracle emits `utf8mb4_0900_bin` |
| `M3.6c` `Jsonb` not re-normalizing | ~ | • | • | • | • | ! | pg still binds `jsonb` (`M14.13`), but since **F-07** no digest depends on how it renders; it can still alter a value on the way *in* |
| `M3.15` audit envelope | • | • | • | • | — | — | |
| `M3.16` chain, two families | • | • | • | • | — | — | |
| `M3.16b` app-computed digest | • | • | • | • | — | — | **F-07** fixed — pg's pre-image is now Rust `canon.rs` |
| `M3.16c` checkpoint | • | ~ | — | — | — | — | sqlite emits but has no `chain_witness` |
| `M3.16d` re-sign | • | — | — | — | — | — | |
| `M3.17` append-only trigger | • | • | • | • | • | ? | emitted in DDL; unverified on mssql/oracle |
| `M3.18` erasure + tombstone | • | • | • | • | — | — | |
| `R4.2` lossless round-trip | • | • | • | • | ? | ? | **F-20** — all three broke on bool/int (mysql/mariadb also on dates, by panic); fixed and verified live on each |
| `R4.5` snapshot reads | • | • | • | • | — | — | **F-21** — all three non-pg ports tore until fixed; each now reads in a transaction, tested |
| `R4.7` consumption audit | • | • | • | • | ~ | ~ | in shared `reconstruct.rs`; its error was flattened to a string on all three non-pg ports until **F-23** |
| `H5.4` serialized `version_id` | • | • | • | • | — | — | **F-24** — mysql/mariadb had no row lock (1 of 8 writers succeeded); now `FOR UPDATE`, 8 of 8 |
| `P6.1` params compile | • | • | • | • | ? | ? | 94.8% of R5 measured on pg only |
| `P6.4a` indexable as bound | • | • | • | • | ! | ? | mssql drops token index on `NVARCHAR(MAX)` |
| `P6.6` fold in Rust | • | • | • | • | • | • | `fold.rs` identical across ports |
| `P6.8` parameter binding | • | • | • | • | — | — | fuzz seed corpus committed in all six |
| `O10.4a` backfill on fold change | • | • | • | • | — | — | **F-15** fixed on sqlite, mysql, mariadb; mssql/oracle have no store |
| `O10.7` encrypted transport | ~ | — | ? | ? | ? | ? | not applicable to embedded SQLite; pg defaults to unverified (**F-17**) |
| `O10.10` supply-chain evidence | • | • | • | • | • | • | `deny.toml` + CI in all six |
| `O10.12` CI runs target engine | • | • | • | • | • | — | mssql now SQL Server 2022; oracle's gate removed rather than faked (F-06 fixed) |
| `T11.1` corpus round-trip | • | • | • | • | ? | ? | |
| `T11.2` live gate | • | • | • | • | ~ | — | mssql: repointed, not yet run. oracle: nothing to gate |
| `T11.6` concurrency adversarial | • | • | ~ | ~ | — | — | suites now on all four; mysql/mariadb cover torn reads and version assignment but cannot cover conditional ops or `If-Match` — neither is implemented there |
| `T11.7` redaction | • | • | • | • | — | — | `redaction.rs` on all four; found **F-23** on three of them |
| `T11.8` audit assertions | • | ~ | ~ | ~ | — | — | corrected: all three assert chain verification, disclosure logging, append-only, and erasure. Missing everywhere but pg: per-algorithm independent tamper detection, and truncated-chain-vs-checkpoint |
| `T11.9` fuzzing run | ? | ? | ? | ? | ? | ? | targets committed; not shown to run in CI |
| `T11.13` skips fail where promised | • | • | ? | ? | • | — | sqlite needs no server, so nothing skips; mysql/mariadb suites still self-skip without a DSN |
| `T11.14` ignored tests tracked | • | • | • | • | • | • | Oracle's 11 ignored tests are in `tasks.md` and its annex |
| `X15.1` shared core identical | • | • | • | • | • | • | now **gated** in CI, not merely observed (F-10) |
| `X15.2` canonical form in Rust | • | • | • | • | • | • | **F-07** fixed — `canon.rs` ported into pg; gate now has 0 exemptions |
| `X15.6` annex covers checklist | • | ~ | ~ | ~ | • | • | pg written (F-14), mssql/oracle rewritten (F-16); sqlite/mysql/mariadb annexes predate the checklist |
| `X15.9` annex ratified | ! | ! | ! | ! | ! | ! | all six are marked *proposed* |
| `X15.11` cross-port chain verify | • | • | • | • | — | — | **F-07** fixed; pg proven by `chain_portability.rs`, which recomputes a chain from the exported rows alone |
| `W16.3` crate description | • | • | • | • | • | • | F-02 fixed |
| `W16.6` CI checks shared core | • | • | • | • | • | • | `scripts/check-shared-core.sh` — F-10 fixed |
| `W16.8` docs not substituted | • | • | • | • | • | • | READMEs rewritten (F-01); **books are still substituted** |
| `W16.9` examples runnable | • | • | • | • | • | • | READMEs now show library usage; signatures checked against source |
| `W16.15` git remote correct | ! | ! | ! | ! | ! | ! | **F-11** |

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
`T11.8` and `T11.9`, and in the two Scaffold ports.

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
- **`fhir-mssql` → Schema.** Done in principle — CI provisions SQL Server 2022,
  runs the target that exists, and fails rather than skips without a database
  (**F-06** fixed). What is left is a green run to cite (`C0.9`), then
  verification against full SQL Server rather than `azure-sql-edge` (`M14.31`).
  Then a `tiberius` store, and the unindexable-column decision (`M14.16`).
- **`fhir-oracle` → Schema.** Write an actual Oracle `ddl.rs` (**F-08**) — the
  [annex](../fhir-oracle/spec/14-oracle-dialect.md) now lists every decision it
  requires, starting with the `VARCHAR2`/`CLOB` boundary. The version floor is
  settled (**F-09** fixed). Then a driver, which is blocked on whether an Oracle
  Free image runs on arm64.
- **`fhir-postgresql` → clean Reference.** The chain pre-image now comes from
  `canon.rs` (**F-07** fixed, `M14.12`) and the dead `_norm` SQL function is gone
  (**F-18** fixed). What is left is one owner decision: the TLS default
  (**F-17**, `M14.27`).

---

Part of the [fhir-databases specification](index.md).
