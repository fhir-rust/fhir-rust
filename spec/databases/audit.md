# Audit findings

Non-normative. This is the register of known divergences between what the
specification requires, what the documentation claims, and what the code does.
Every finding carries evidence a reader can check.

A finding stays here until it is fixed or the spec is amended to match reality.
Deleting a finding because it is inconvenient, or because the text that stated
it was rewritten, is the failure mode this file exists to prevent.

**Audit date:** 2026-07-31. **Remediation pass:** 2026-07-31. **Scope:** all six
ports at the tree's current state. **Method:** cross-port diff of every shared
file under a name-substituting normalization; read of every spec, README,
`Cargo.toml`, CI config, and `col_sql` binding.

Of twenty-nine findings, **twenty-three are fixed and five remain open**; F-05 is
recorded rather than fixable, because the requirements it concerns are retained
deliberately. Eleven were found during the remediation pass rather than the
original audit (F-17 to F-29), which is the more useful number: fixing the
findings is what surfaced most of the rest.

F-20 to F-23 were found by **writing the missing concurrency and redaction
tests for `fhir-sqlite`** — the work the previous pass named as highest-value
and did not do. F-20 then turned out to affect `fhir-mysql` and `fhir-mariadb`
too, and worse; that was settled by starting the actual engines rather than by
reading more code.

Porting those two suites onward to MySQL and MariaDB found that **F-21 and F-23
were present there as well**, plus a new **F-24** unique to them. Four ports, one
pattern: the defects live in the per-port store layer, which is where nothing
was being tested outside `fhir-postgresql`. All four are real defects in a port shipping at Store level,
three of them High, and every one was invisible to the existing suite. That is
the strongest available argument for the rest of that work: the tests did not
document known gaps, they found unknown ones.

The same held for `fhir-mssql`, from the other direction. Correcting the stale
doc comments in its T-SQL emitter (**F-19**, Low) meant reading eight lines
against the engine they target, which found **F-25** and **F-26** — two High
defects that between them mean the port's upgrade path could never have executed
one statement. A port with no store had a broken migration nobody could have
noticed, because its one live test installs a fresh schema and never takes that
path. Checking whether those two were tracked as tasks then found **F-27**: the
task lists for both scaffold ports tick off a store neither has, citing MySQL
test runs as the evidence.

F-27 grew on the next pass, which is the pattern worth naming. Going back to
write those two files honestly meant reading all six, and the same `[x]` marks
turned out to assert a **REST server and CLI in every port** — including the
reference one — that this repository has never contained: no `fhir-*-server`
crate, no `serve` binary, no REST suite. The first look found the MySQL
contamination because that is what `fhir-mssql` was contaminated *with*; it took
reading the clean port to notice that the fiction was older and shared.

Of the six that remain, one is documentation — F-27, six files of it — and the
rest are not: an Oracle DDL emitter and four owner decisions.

**F-15 is closed everywhere it can be**, and so is **F-07**. SQLite, MySQL and
MariaDB all have `upgrade` and `backfill_norm`, the last two verified against
live engines; `fhir-mssql` and `fhir-oracle` have no store to hang one on, so
theirs arrive with the store work. Closing F-07 also emptied the shared-core
gate's exemption list — **65 files identical across all six ports, nothing
excused**.

What is left is `fhir-oracle`'s DDL emitter, which is that port's whole
remaining job and needs an Oracle to verify against (**F-08**), and four
decisions that are not the implementer's to make (**F-04**, **F-11**, **F-17**,
and **F-27**'s central question).

## Severity

| | Meaning |
| --- | --- |
| **High** | Makes a false claim about clinical software, or defeats a control. |
| **Medium** | A real defect with a bounded blast radius, or a gap that will grow. |
| **Low** | Cosmetic, stale, or already tracked in a port's `tasks.md`. |

## Summary

| Id | Severity | Finding | Status |
| --- | --- | --- | --- |
| [F-01](#f-01) | **High** | READMEs assert the reference port's measured results in five ports that never obtained them | **fixed** |
| [F-02](#f-02) | Medium | All six store crates describe themselves as "PostgreSQL storage layer" | **fixed** |
| [F-03](#f-03) | Low | Stale driver comments in `fhir-oracle` and `fhir-mssql` workspaces | **fixed** |
| [F-04](#f-04) | Medium | §11 and §13 cite retired §7/§8 requirements; three compliance rows rest on them | open — **owner decision** |
| [F-05](#f-05) | Medium | §10 and §12 specify a service that no crate implements | recorded, closed as far as it can be |
| [F-06](#f-06) | **High** | `fhir-mssql` and `fhir-oracle` live gates never ran their own engine | **fixed** |
| [F-07](#f-07) | **High** | `fhir-postgresql` derives its chain pre-image from `jsonb`; chains are not portable | **fixed**, verified live |
| [F-08](#f-08) | **High** | `fhir-oracle`'s DDL emitter is verbatim MySQL and cannot produce an Oracle schema | open — unblocked; the map change (`U1`–`U10`) comes first |
| [F-09](#f-09) | Medium | `fhir-oracle` had no declared engine floor under a 63-byte identifier budget | **fixed** |
| [F-10](#f-10) | Medium | Nothing verifies that the shared Rust core has not diverged across ports | **fixed** |
| [F-11](#f-11) | Medium | All six ports share the ancestor project's git remote | open — **owner decision** |
| [F-12](#f-12) | Medium | `spec/index.md` referenced `AGENTS.md` in every port; no such file existed | **fixed** |
| [F-13](#f-13) | Medium | Sections 1–13 were duplicated across six ports | **fixed** |
| [F-14](#f-14) | Low | `fhir-postgresql` had no dialect annex while the other five did | **fixed** |
| [F-15](#f-15) | Low | `_norm` backfill is unavailable on four ports after a fold change | **fixed** in sqlite, mysql, mariadb (live); mssql/oracle await a store |
| [F-16](#f-16) | **High** | The MSSQL and Oracle dialect annexes were the MySQL annex, unedited | **fixed** |
| [F-17](#f-17) | Medium | `fhir-postgresql` TLS defaults to unverified | open — one-line default change, breaking |
| [F-18](#f-18) | Low | `fhir-postgresql` carried a SQL folding function nothing calls (never emitted — wording corrected) | **fixed** |
| [F-19](#f-19) | Low | `fhir-mssql`'s T-SQL emitter carries MySQL doc comments, one contradicting its code | **fixed** |
| [F-20](#f-20) | **High** | Booleans, integers and dates unreadable: dropped on sqlite, **panic** on mysql/mariadb | **fixed** in all three, verified live |
| [F-21](#f-21) | **High** | Reads tore under concurrent writes — no read transaction | **fixed** in sqlite, mysql, mariadb |
| [F-22](#f-22) | **High** | `fhir-sqlite` version-guarded writes enforced nothing; all racers won | **fixed** |
| [F-23](#f-23) | Medium | Shred/reconstruct errors flattened, erasing the integrity signal | **fixed** in sqlite, mysql, mariadb |
| [F-24](#f-24) | **High** | `fhir-mysql`/`fhir-mariadb` assigned `version_id` with no row lock: 1 of 8 concurrent writes succeeded | **fixed**, verified live |
| [F-25](#f-25) | **High** | `fhir-mssql`'s upgrade DDL emits `ADD COLUMN`, which T-SQL rejects: the upgrade path cannot run at all | **fixed** |
| [F-26](#f-26) | **High** | `fhir-mssql` adds `actor NOT NULL` with no default; SQL Server refuses that on any populated table | **fixed** |
| [F-27](#f-27) | **High** | All six `tasks.md` tick off a REST server and CLI no port has; the two scaffolds also tick off a store | open — headers added, six rewrites remain |
| [F-28](#f-28) | Medium | `schema_wide_objects` documented itself as idempotent on MySQL/MariaDB; its three indexes are not | **fixed** |
| [F-29](#f-29) | Low | `scripts/db.sh` did not give PostgreSQL the `--shm-size` CI gives it, so a local full-corpus run failed where CI passed | **fixed** |

## What remains, and why

| Finding | Why it is not fixed here |
| --- | --- |
| **F-08** | Writing an Oracle DDL emitter. Two of its three blockers cleared this pass. The `VARCHAR2`/`CLOB` boundary — which `M14.9` said MUST be settled *before* `ddl.rs` is written — is now settled by [unbounded string search](unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) (`U1`–`U10`, `P6.9`), and an arm64 Oracle image is confirmed to exist and pull (`M14.23`). What remains is the order of work: the adjunct columns are a **map** change in `model.rs` and `gen/src`, which are shared verbatim across all six ports (`X15.1`), so that lands before any Oracle DDL. And an Oracle has still never been *started* here — it needs more memory than a default Podman machine has. |
| **F-15** | Done for `fhir-sqlite`. MySQL and MariaDB each need an `upgrade` plus a resumable `_norm` backfill (`L14`), and both first need `init` to start recording the map asset — like SQLite, they store only `map_checksum`. Each needs that engine live. `fhir-mssql` and `fhir-oracle` have no store to hang an upgrade on, so theirs arrive with **F-08** and the store work. |
| **F-17** | Changing the default from `Prefer` to `Require` is one line and is **breaking** for any deployment relying on the libpq-compatible default. That is the owner's call; the departure is recorded (`M14.27`) and the README now says to set `verify-full`. |
| **F-04** | Whether to restore §7 or amend the citing requirements is an owner decision. Reconstructing retired requirement text from its citations was considered and rejected: text nobody wrote would carry ratified authority in the section that maps to regulation. |
| **F-11** | Setting six git remotes needs the six target URLs, and the related question — keep the shared ancestor history, squash, or re-root — is the owner's. |
| **F-27** | Six `tasks.md` rewrites of ~800 lines each. Unlike the annexes (F-16) there is no correct sibling to adapt — the REST milestone is fiction in the reference port too — and the central question is the owner's: are these libraries that will grow a server, or libraries that will not? That decides whether the REST tasks are unticked, deleted, or moved out. A planning exercise, not an editing one. Each file now carries a header naming what in it is untrue, so the misleading is stopped while the rewrite waits. |

---

## F-01

**READMEs assert the reference port's measured results in ports that never
obtained them.** Severity: **High**. Violates `C0.11`, `W16.8`, `W16.10`.

Every port's `README.md` carries the PostgreSQL reference's status paragraph
with the engine name substituted:

> all **7,399 official FHIR example resources** (R3 + R4 + R5) round-trip
> **losslessly** through the fully normalized schema — in memory, through live
> \<engine\>, and 10,000 generated property-test cases besides. 94.8% of R5
> search parameters compile to indexed SQL, and `fhir-<engine> serve` mounts
> every installed version with CRUD, history, ETag concurrency, search, and
> all-or-nothing transaction Bundles.

In `fhir-mssql` and `fhir-oracle` there is **no store crate implementation at
all** — `crates/*-store/src/` contains `lib.rs` and `chain.rs` and nothing else,
and neither has a store test file. Nothing round-tripped through either engine,
live or otherwise. `fhir-oracle` additionally has no map test directory.

This is the most serious finding in the register, and the reason is not
pedantry: these are documents about software that would hold patient records,
and the specific claims substituted are the ones a reader would use to decide
whether the thing is safe to deploy.

**Disposition: FIXED.** All six READMEs rewritten to describe their own port at
its actual conformance level (`W16.8`, `C0.11`):

- `fhir-sqlite`, `fhir-mysql`, `fhir-mariadb` — real API, real limitations, and
  the dialect decisions that are actually theirs. Each states that its
  concurrency/redaction/audit guarantees are `?` in the matrix.
- `fhir-mssql`, `fhir-oracle` — lead with a Scaffold warning saying there is no
  store, and say plainly what the previous text claimed and why it was false.
- `fhir-postgresql` — its measurements are its own and stay; its CLI and REST
  server did not exist and are replaced with the library API.

Three of the six were titled "**FHIR in PostgreSQL, relationally**" while
targeting SQLite, MySQL, and MariaDB respectively.

Every README also documented `cargo install --path crates/fhir-<engine>` — a
directory in no workspace (`W16.9`). All six now show library usage that matches
the crates, with constructor names and signatures checked against the source.

## F-02

**All six store crates describe themselves as "PostgreSQL storage layer".**
Severity: Medium. Violates `W16.3`.

```
fhir-sqlite/crates/fhir-sqlite-store/Cargo.toml:
    description = "PostgreSQL storage layer for fhir-sqlite"
```
— and identically in `fhir-mysql`, `fhir-mariadb`, `fhir-mssql`, `fhir-oracle`.

`description` is published to crates.io and rendered on docs.rs. It is read by
exactly the person who has not yet looked at the code.

**Disposition: FIXED.** Each store crate now names its own engine. The two
Scaffold ports say so in the description itself — "SQL Server storage layer for
fhir-mssql (scaffold: no store yet)" — because `W16.14` forbids publishing above
a port's conformance level, and a description that hides the level would be the
same failure one layer down.

## F-03

**Stale driver comments.** Severity: Low. Violates `W16.4`.

`fhir-oracle/Cargo.toml` carried a comment describing "The MySQL driver: async
and pure Rust…" above a dependency list containing **no database driver at
all**. `fhir-mssql/Cargo.toml` carried the same MySQL comment immediately above
its `tiberius` comment, so the file documented two drivers and depended on one.

**Disposition: FIXED.** The stale comment is removed from `fhir-mssql`. In
`fhir-oracle` it is replaced by one explaining why the workspace carries **no**
driver — the port is Scaffold level, and choosing a driver is blocked on two
open questions in its annex, including whether an Oracle Free image runs on
arm64 at all.

## F-04

**§11 and §13 cite requirements from retired sections.** Severity: Medium.

Sections 7 (REST API) and 8 (CLI) were removed as out of scope, but `A7.8`,
`A7.10`, `A7.11`, `A7.12`, `M8`, and "§7" itself are still cited by `T11.2`,
`T11.6`, `T11.7`, `T11.11`, and three rows of the §13 compliance table.

The compliance rows are the sharp end: HIPAA §164.312(e), ONC/HTI FHIR
conformance, and ONC/HTI Bulk Data each map to a struck identifier, and their
evidence cells name runs ("Inferno run", "live TLS smoke test") that cannot be
performed against a library.

**Disposition.** Recorded, not reconstructed. `C0.16` registers each dangling id
with what its citation reveals, and §13 strikes the identifiers so a reviewer
cannot mistake a retired citation for a met obligation. Reconstructing the
requirement text from its citations was considered and rejected: text nobody
wrote would carry ratified authority in the one section that maps to
regulation. Resolving this means the owner either restores §7 or amends the
citing requirements — an owner decision, not an editorial one.

## F-05

**§10 and §12 specify a service that no crate implements.** Severity: Medium.

`O10.1`, `O10.3`, `O10.5`, `O10.7`–`O10.9`, `V9.2`, `V9.3`, and most of
`PR12.1`–`PR12.8` describe `serve`, `--bind`, `--admin-bind`, trusted proxy
CIDRs, request headers, HTTP status codes, and `/metrics`. Every workspace
contains exactly three members — `-map`, `-gen`, `-store` — and no crate depends
on `axum` or `clap`.

**Disposition.** Recorded as `C0.17`, and each affected requirement is marked
**[service]** at its point of use. The requirements are retained rather than
deleted because the obligations are real and will bind whatever service is
built; what changes is that a port is no longer non-conformant for lacking them,
and a reader is no longer misled into thinking they describe shipped behaviour.

`PR12.4`, `PR12.5`, `PR12.6`, and the new `PR12.3a` are explicitly **not**
marked service: the store implements attribution and access logging today, and
those are the requirements §12 exists for.

## F-06

**`fhir-mssql` and `fhir-oracle` live gates never ran their own engine — or any
engine.** Severity: **High**. Violates `C0.10`, `O10.12`, `T11.13`.

Both ports provisioned MySQL in all three places:

```
fhir-mssql/.github/workflows/ci.yml:       image: mysql:8.4
fhir-mssql/.woodpecker/database.yaml:      image: docker.io/library/mysql:8.4
fhir-mssql/scripts/db.sh:                  ENGINE="mysql"  IMAGE=…/mysql:8.4
fhir-oracle/.github/workflows/ci.yml:      image: mysql:8.4
fhir-oracle/.woodpecker/database.yaml:     image: docker.io/library/mysql:8.4
fhir-oracle/scripts/db.sh:                 ENGINE="mysql"  IMAGE=…/mysql:8.4
```

**Correction to the original audit.** This was first recorded as a *silent
skip* — a live test self-skipping for want of a DSN the pipeline never set. That
was wrong, and the truth is worse. Both pipelines invoked
`cargo test -p fhir-<port>-map --test mysql_ddl`, and neither package has a test
target by that name (`fhir-mssql`'s is `mssql_ddl`; `fhir-oracle` has no tests
directory at all). Verified:

```
$ cargo test -p fhir-mssql-map --test mysql_ddl --no-run
error: no test target named `mysql_ddl` in `fhir-mssql-map` package
help: a target with a similar name exists: `mssql_ddl`
```

So the database job did not silently pass — it **could not pass at all**, and
had never executed a single assertion of the T-SQL it existed to verify.
`fhir-mssql` additionally set `FHIR_MSSQL_TEST_DSN` to a *MySQL* DSN, which
`tiberius`'s `Config::from_ado_string` would not have parsed had execution ever
reached it.

The silent-skip risk was real too, just one step further along: the test does
self-skip without a DSN, so a fixed target name alone would have converted a
hard failure into a quiet pass.

**Disposition: FIXED**, differently for each port, because their situations are
not the same.

*`fhir-mssql` — repointed.* There is real T-SQL to verify, so CI now provisions
**SQL Server 2022** (`mcr.microsoft.com/mssql/server:2022-latest`), sets a
proper ADO connection string, and runs `--test mssql_ddl`. `scripts/db.sh` grew
an `mssql` branch that starts the same image, probes readiness with a real
`sqlcmd` query rather than a port check — SQL Server accepts TCP well before it
has recovered the system databases — and honours `FHIR_MSSQL_IMAGE` so Apple
silicon can substitute `azure-sql-edge` knowingly (`M14.31`).

Both pipelines and `scripts/db.sh test` set **`FHIR_MSSQL_REQUIRE_DB=1`**, and
the test now panics instead of skipping when it is set (`M14.30`, `T11.13`).
Verified in both directions: without it the test skips and passes; with it and
no DSN it fails with
`FHIR_MSSQL_REQUIRE_DB is set, so this test must run: set FHIR_MSSQL_TEST_DSN to run`.

*`fhir-oracle` — removed.* There is nothing to point a gate at: no Oracle DDL,
no driver, no store, no map tests. The GitHub `database` job and the Woodpecker
pipeline are deleted and replaced with a comment explaining why, and
`scripts/db.sh` now refuses with an explanation and exit code 1 rather than
starting MySQL. A removed gate is honestly absent; a green gate against the
wrong engine is a false claim about clinical software.

## F-07

**`fhir-postgresql` derives its chain pre-image from `jsonb`.** Severity:
**High**. Violates `X15.2`, `X15.11`, `M3.6c`.

```
fhir-postgresql/crates/fhir-postgresql-store/src/lib.rs:291
    (($1::text)::jsonb)::text AS canon, \
```
— the result of which is passed to `chain::preimage` at `lib.rs:311`.

`fhir-postgresql-map` is the **only** map crate without `canon.rs`; the other
five implement canonical JSON in Rust, and their implementations are identical.
The 0.4.0 changelog moved the *digest* into the application; the *canonical
bytes* stayed in the database.

Consequences, in order of seriousness:

1. A PostgreSQL chain cannot be verified by any other port, so `X15.11` fails
   and the chain format does not survive a port.
2. The bytes signed are whatever `jsonb` produced when it reordered keys and
   rewrote number spellings — a form defined by a PostgreSQL version, not by
   this specification.
3. `M3.16b`'s argument that computation must not live where the data lives is
   only half-satisfied: the digest moved out, the pre-image did not.

**Disposition: FIXED**, verified against live PostgreSQL 18.

`canon.rs` is ported into `fhir-postgresql-map` — byte-identical to the other
five — and both the writer and `verify_audit` now derive the pre-image through
one `canon_of` helper. **The shared-core gate went from two exemptions to zero**:
this finding was the only thing it excused, and `EXEMPT` is now empty.

Writer and verifier canonicalize the **same stored bytes**, so they agree by
construction rather than by both happening to call the same SQL cast. That is
what `M14.13` means when it says fixing the pre-image makes the column type stop
mattering: `jsonb` still normalizes on the way in, but nothing downstream
depends on how it renders on the way out.

**The transition was an owner decision, and the answer was "no transition".**
This is a chain-format change, so rows written under the old form no longer
verify, and the pre-image carries no version marker for a verifier to dispatch
on. Three options were put up: mark the boundary with a per-row `chain_format`
column (`M3.16e`'s own precedent, at the cost of a storage-model change in all
six ports), accept either pre-image during a transition (at the cost of two
definitions of what was signed — the thing `L1`/`X15.2` exist to prevent), or
treat the project as having no installed base and change the format outright.
The last was chosen: the repository is 0.4.0 and has never been pushed
(**F-11**). **The migration for any pre-existing PostgreSQL database is a
reload**, and old rows will report as breaks until then — recorded here rather
than left to be discovered.

**Tested from the outside, which is the point.** `audit.rs` calls
`verify_audit`, so writer and verifier are the same code and would agree even if
both were wrong. The new `chain_portability.rs` recomputes the chain using only
what another port would have — the exported row columns, the shared
`canon::canonicalize`, and `chain::preimage`/`link` — over a plain connection
that never goes through the `Store` that wrote it. A second test asserts the
canonical form is **not** byte-identical to what `jsonb::text` renders, so the
suite cannot pass by the two formats coinciding.

**Mutation-verified** (`T11.10`): restoring the `jsonb` pre-image fails with
`SHA-256 for p1#1 does not match a chain recomputed from canon.rs — the
pre-image is still engine-defined (F-07)`.

## F-08

**`fhir-oracle`'s DDL emitter is verbatim MySQL.** Severity: **High**. Already
tracked in `fhir-oracle/tasks.md`; recorded here because the conformance matrix
depends on it.

`fhir-oracle/crates/fhir-oracle-map/src/ddl.rs::col_sql` emits `TEXT`,
`TINYINT(1)`, `DATETIME(6)`, `LONGTEXT`, and `COLLATE utf8mb4_0900_bin` — none
of which exist in Oracle — and its comments still discuss MySQL's 2038 `TIMESTAMP`
range. The port's own `tasks.md` states this plainly ("Scaffold only… Nothing
here is an Oracle schema") and `#[ignore]`s the eleven MySQL-asserting tests, so
the code is honest; the README (F-01) is not.

## F-09

**`fhir-oracle` has not declared an engine floor.** Severity: Medium. Violates
`S1.4`.

Oracle identifiers were 30 bytes before 12.2 and 128 after. The generator's
budget is 63 (`G2.4`), which is safe on 12.2+ and silently wrong below it — two
distinct table names would collapse into one, which is precisely the collision
`G2.4` exists to make impossible. The port inherited the constant without
inheriting a reason.

**Disposition: FIXED.** `M14.2` in the rewritten annex declares **Oracle 12.2**
as the floor and states the identifier fact that sets it; `M14.3` requires
`init` to verify the server version and refuse below it. Requiring 23ai instead
— which would buy a native `BOOLEAN` — was considered and rejected in `M14.4`,
with `NUMBER(1)` + `CHECK` as the substitute.

## F-10

**Nothing verifies that the shared Rust core has not diverged.** Severity:
Medium. Violates `W16.6`; `X15.12` recommends the cross-engine test that would
also catch it.

Measured at audit time, `model.rs`, `shred.rs`, `reconstruct.rs`, `value.rs`,
`fold.rs`, `error.rs`, and all five `gen/src` modules are **identical across all
six ports** under crate-name normalization, and `canon.rs` is identical across
the five that have it. That is the good news and also the finding: they are
identical because the ports were forked recently, not because anything checks.

Sections 1–13 of the spec were identical for the same reason, and had already
begun to drift — each port's annex opens by admitting that its inherited
sections "still describe PostgreSQL". Shared-by-convention degrades on a
schedule set by how often someone edits one copy.

**Disposition: FIXED.** `scripts/check-shared-core.sh` normalizes the crate-name
substitution and annex cross-references, then diffs all 13 shared modules across
all six ports against a baseline. A new root workflow
(`.github/workflows/monorepo.yml`) runs it, alongside a relative-link checker —
three port spec indexes had carried a broken link to `14-mysql-dialect.md`, a
file existing only in `fhir-mysql`, for as long as the forks had.

It reports **65 files identical, with nothing exempted**. It briefly carried two
entries — `fhir-postgresql` lacked `canon.rs` and the corresponding
`pub mod canon;` line (**F-07**) — and closing that finding deleted both, which
is what an exemption is for: a divergence with an expiry, not a permanent
excuse. The list is printed on every run even when empty, because a gate that
silently tolerates exceptions is the same failure as a test that silently skips
(`T11.12`).

Mutation-verified per `T11.10`: appending one comment line to
`fhir-mysql/.../fold.rs` makes it fail with
`fhir-mysql map/src/fold.rs 2 differing lines`; restoring the file makes it
pass. The link checker was verified the same way, and its first version was
itself wrong — it flagged a shell glob and a T-SQL identifier inside code spans,
so it now strips inline code as well as fenced blocks. A gate that cries wolf
trains a reader to ignore it.

## F-11

**All six ports share the ancestor project's git remote.** Severity: Medium.
Violates `W16.15`. Already tracked identically in all six `tasks.md` files.

Every port's `origin` is the upstream `fhirpg` repository, correct for at most
one of them. Nothing has been pushed. The related open question — whether six
products should keep a shared ancestor history, be squashed, or be re-rooted —
is recorded in the same place and is an owner decision.

## F-12

**`spec/index.md` referenced a file that did not exist.** Severity: Medium.
**Fixed.**

Every port's `spec/index.md` contained "Operational guidance for contributors
lives in `AGENTS.md`". No `AGENTS.md` existed anywhere in the tree. The
monorepo now has [`AGENTS.md`](../AGENTS.md) with [`AGENTS/`](../AGENTS/) topic
files, and each port has one pointing at it.

## F-13

**Sections 1–13 were duplicated across six ports.** Severity: Medium. **Fixed.**

78 files, ~290 KB, byte-identical apart from the product name — 0 to 2 differing
lines per file, measured by normalized diff. Consolidated into `/spec` as the
single source of truth (`W16.5`); each port keeps its `index.md` and its dialect
annex. The old copies remain in git history.

## F-14

**`fhir-postgresql` had no dialect annex.** Severity: Low. Violates `X15.6`.

The other five ports each have `spec/14-<engine>-dialect.md`; PostgreSQL had
none, because it was the original and its dialect *was* the spec. Once the core
became engine-neutral, PostgreSQL's bindings — `jsonb`, `smallint[]` for `ords`,
`COLLATE "C"`, `REPEATABLE READ READ ONLY`, the staged-schema install,
`sslmode` — became departures like any other and needed writing down.

**Disposition: FIXED.** `fhir-postgresql/spec/14-postgresql-dialect.md` now
covers the `X15.6` checklist in 28 requirements.

Writing it was not bookkeeping: it **surfaced two departures that had been
invisible** while this port defined the specification, and both are now tracked
as findings of their own.

- The `jsonb` binding violates `M3.6c` (`M14.13`). It was also the mechanism
  behind **F-07**, which is now fixed — the pre-image no longer depends on how
  the column renders, though `jsonb` can still alter a value on the way in.
- `SslPolicy` defaults to `Prefer`, which does not verify the server
  certificate, against `O10.7` — recorded as `M14.27` and **F-17**.

That is the argument for the annex requirement in general: a binding nobody has
had to justify in writing is a binding nobody has checked.

## F-15

**`_norm` backfill is unavailable on four ports.** Severity: Medium. Violates
`O10.4a`, `L13`. Already tracked in all six `tasks.md` as T90a.

The accent-folding fix (T90) changed stored `_norm` values. `fhir-postgresql`
has `backfill_norm` on its upgrade path; SQLite, MySQL, MariaDB, MSSQL, and
Oracle have no `upgrade` at all, so for them the migration is a full reload.
Deploying the corrected fold against an existing database without backfilling
leaves searches matching neither the old spelling nor the new — silently.

**Disposition: fixed in `fhir-sqlite`; open for the other four.**

`fhir-sqlite` now has `upgrade` and `backfill_norm`, and `init` records the map
asset that makes a diff possible at all (`M14.30`). Three things are SQLite's
doing rather than choices, and each is now a requirement:

| | |
|---|---|
| `M14.31` | The upgrade is **one transaction**. SQLite's DDL is transactional and its write lock is single-holder, so PostgreSQL's chunking has no purpose here and would leave a half-upgraded schema on failure. |
| `M14.32` | The audit envelope is **diffed against `pragma_table_info`**, not reconciled — there is no `ADD COLUMN IF NOT EXISTS`. |
| `M14.33` | A refused `DROP COLUMN` says *why*; SQLite reports every precondition as a bare `SQLITE_ERROR`. |

Eight tests, in `crates/fhir-sqlite-store/tests/upgrade.rs`. They need **no
server and no spec directory**: the "old deployment" is the shipped relmap asset
reduced in memory to the shape the model documents as pre-folding
(`TargetKind::Str::norm` is `None` "only for maps generated before folding
existed"), so the reduction is exact rather than an approximation.

**Mutation-verified** (`T11.10`), and the three mutations are the finding:

- Skipping the backfill — *literally F-15* — makes the seeded patient
  unfindable by their own name: `searching "amelie" did not find the patient
  seeded before the folded column existed`. The behavioural assertion catches
  this on its own, with the `folded > 0` count assertion disabled.
- Reconciling the envelope instead of diffing it fails with
  `duplicate column name: actor` on the **first** upgrade, not the second,
  because a fresh install already has the envelope from `create_table`.
  PostgreSQL's approach ported verbatim would never have worked here.
- Dropping the `IS NULL` guard from the backfill's select makes it re-fold what
  it has already written, which is the resumability property.

**MySQL and MariaDB now have it too**, verified against live MySQL 8.4 and
MariaDB 11.4 under Podman. Eight tests each, the same suite as SQLite's, and
implementing it turned up three things worth recording:

| | |
|---|---|
| `M14.34` | **The meta column could not hold the asset.** `value` was `TEXT`, capped at 65,535 bytes; the hex-coded R5 map asset is ~2.4 MB, 37x over. In strict mode the insert fails; non-strict it **truncates**, and a truncated asset still decodes far enough to look like a map and would produce a *wrong diff*. Now `LONGTEXT`. |
| `M14.35` | **No transactional DDL.** A failed upgrade leaves a partial schema and that cannot be prevented, so it is reported: how many statements applied, that they remain, and which one failed. |
| `M14.36` | **Ordering.** Both reconcile filters must be computed *after* the additive statements run. A history table `create_table` made moments earlier already carries the audit envelope, so a filter built beforehand emits `ADD COLUMN` for columns the table is about to gain. |

The last two were found by running it, not by reading it. The ordering bug
produced `Duplicate column name 'actor'` on a live server; written the other way
round it would have passed any review.

**Mutation-verified** (`T11.10`) on both ports: skipping the backfill makes the
seeded patient unfindable by their own name; dropping the index filter fails
with `Duplicate key name 'fhir_*_access_log_subject_ix'`; dropping the column
filter fails with `Duplicate column name 'actor'`.

**What this does not fix, on SQLite itself.** A database installed before this
revision has no `map_asset`, so it cannot be diffed and `upgrade` refuses it by
name. For those the migration is still a reload — the finding is closed going
forward, not retroactively. That is inherent: the information needed to diff was
never written down, and inferring the old map from the installed schema would be
guessing at exactly the point where guessing wrong corrupts data.

What remains is `fhir-mssql` and `fhir-oracle`, and neither has a store to hang
an upgrade on. Theirs arrive with the store work — and for Oracle, with **F-08**
first. There is no separate upgrade task to schedule for them.

## F-16

**The MSSQL and Oracle dialect annexes are the MySQL annex, unedited.**
Severity: **High**. Violates `X15.6`, `X15.7`, `C0.12`.

`fhir-mssql/spec/14-mssql-dialect.md` and `fhir-oracle/spec/14-oracle-dialect.md`
differ from `fhir-mysql/spec/14-mysql-dialect.md` by **three lines**, all of them
a crate name:

```
$ diff fhir-mysql/spec/14-mysql-dialect.md fhir-mssql/spec/14-mssql-dialect.md
60c60   <  `fhir-mysql-gen` MUST NOT change…      >  `fhir-mssql-gen` MUST NOT change…
296c296 <  crates/fhir-mysql-map/src/canon.rs     >  crates/fhir-mssql-map/src/canon.rs
339c339 <  SET LOCAL fhir_mysql.erasure           >  SET LOCAL fhir_mssql.erasure
```

Both files are titled "**14. MySQL dialect**". Both open "This annex records
where the **MySQL** port departs…". Both declare the target as "**MySQL 8.0 or
later, InnoDB, `utf8mb4`**". Both carry a section headed "Relationship to
fhir-mariadb". The MSSQL annex mentions SQL Server, T-SQL, `NVARCHAR`, and
`tiberius` **zero times**; the three occurrences of "Oracle" in the Oracle annex
are the substituted crate names.

Why this is worse than F-08 rather than the same finding. F-08 is code that was
honestly labelled — `fhir-oracle/tasks.md` says "Scaffold only… Nothing here is
an Oracle schema", and the misleading tests are `#[ignore]`d. F-16 is the
*specification*, the document that decides what the code must do, and it decides
wrongly and confidently. Two consequences follow:

1. **The MSSQL annex contradicts working MSSQL code.** `fhir-mssql`'s `ddl.rs`
   is genuine T-SQL — `BIT`, `NVARCHAR(450) COLLATE Latin1_General_100_BIN2`,
   `DATETIME2(6)`, `OFFSET … FETCH` — written deliberately and verified by hand
   against `azure-sql-edge`. Its annex requires `utf8mb4`, InnoDB, and MySQL 8.0
   features. A reader following the spec would reject the correct code.
2. **Every `M14.x` id in those two files is wrong.** `M14.6` in the MSSQL annex
   is a MySQL requirement about `ords`. A test, a commit, or a review citing it
   cites something that was never decided for this engine — and `C0.5` makes
   those ids permanent.

A corroborating detail: three of the six port `spec/index.md` files linked their
annex as `14-mysql-dialect.md`, a filename that exists only in `fhir-mysql`. The
MariaDB, MSSQL, and Oracle indexes each carried a broken link to the file they
had been copied from. Fixed in this revision.

**Disposition: FIXED.** Both annexes rewritten from the `X15.6` checklist
against their actual engines, and they came out very differently — which is
itself the point.

*`fhir-mssql`* had a great deal already decided, embedded as comments in a
`ddl.rs` that had been written carefully and verified by hand against a server.
The annex now states 31 requirements recovered from it: bracketed identifiers
(double quotes depend on session state), `NVARCHAR` never `VARCHAR` (a code page
would eat a patient name), `NVARCHAR(450)` because 450 × 2 = 900 = the index key
limit, `DATETIME2(6)` because `DATETIME` rounds to 1/300 s and would alter a
value the chain signs, `ords` as `VARBINARY(255)`, and `CREATE OR ALTER TRIGGER`
so there is no `DROP`-then-`CREATE` window in which history is unguarded. It
also records the unindexable-`NVARCHAR(MAX)` gap as an explicit departure
(`M14.16`) rather than a footnote, and names three things that are simply
undecided (`M14.25`–`M14.27`).

*`fhir-oracle`* had nothing decided, so its annex is a **decision list**: what
must be settled and why the obvious answer is wrong. That is what `X15.6` asks
for when nothing has been decided, and it is far more useful than another
engine's confident answers. It settles the engine floor (**F-09**) and leaves
the rest open — with the `VARCHAR2`/`CLOB` boundary called out as the hardest,
because a `CLOB` cannot be indexed or compared with `=` at all, making it a
sharper version of the SQL Server problem in the direction that matters.

**Requirement numbering restarts in both files.** The old `M14.x` ids were
MySQL's requirements wearing another port's name, and `C0.5` makes ids
permanent, so they are withdrawn rather than reused. Both annexes say so at the
top, and both note that any pre-rewrite citation is void and should be traced to
the MySQL annex it actually came from.

## F-17

**`fhir-postgresql` TLS defaults to unverified.** Severity: Medium. Violates
`O10.7`. *Found during the remediation pass, while writing the PostgreSQL
dialect annex (**F-14**).*

`O10.7` requires a port default to verifying the server certificate, because the
connection carries PHI. `SslPolicy` has three effective modes and defaults to
`Prefer`:

| `sslmode` input | Effective mode | Verifies certificate? |
|---|---|---|
| `disable` | `Disable` | — |
| `prefer`, `allow` | `Prefer` **(default)** | no |
| `require`, `verify-ca`, `verify-full` | `Require` | yes, certificate **and** hostname |

Two things in that table are deliberate and good, which is why this is Medium
rather than High. `require` here is **stricter than libpq's**, which encrypts
without validating anything and so does not survive an active attacker; and
collapsing `verify-ca` into full verification errs in the safe direction.

The **default** is the problem. `Prefer` matches libpq, which is why it was
chosen — but libpq's default is a compatibility decision for a general-purpose
client, and this is a component that carries PHI on every connection. A
deployment that sets nothing gets an unverified link and no warning.

**Disposition: open.** Changing the default to `Require` is one line and is
**breaking** for any deployment relying on the libpq-compatible behaviour, so it
is the owner's call. Recorded as a departure in `M14.27`, and the port's README
and annex now both say to set `PGSSLMODE=verify-full` explicitly rather than
describing `verify-full` as "the production setting" as though it were in force.

## F-18

**`fhir-postgresql` carries a SQL folding function that nothing calls.**
Severity: Low. Violates `L3`. *Found during the remediation pass; its
description corrected when fixed.*

`ddl.rs` defined `NORM_FN` and `norm_function(schema, have_unaccent)` —
`fhir_postgresql_norm(text)`, the pre-`P6.6` design. Folding has since moved
into Rust (`fold.rs`, `P6.6`, `L1`), the shredder fills a materialized `_norm`
column, and every index is on that plain column rather than a function
expression, so nothing called it: not the store, not an index, not a generated
predicate.

**Correction to the original wording.** This finding said the function was
emitted "into every schema". It was not. `schema_wide_objects` never included
it and `ddl`/`ddl_in` never called it, so no database ever had one — it was
dead `pub` API in the map crate, reachable only by a caller who went looking.
That makes the finding **narrower** than recorded: the risk was that someone
*could* emit it, not that every deployment had one. The register is worth
correcting rather than quietly closing, because "we said it was worse than it
was" is the same class of error as the reverse.

The risk `L3` exists to prevent is still the right one: a folding function
reachable from the schema invites a query written against it, which would create
the second definition of string equality that `L1` exists to prevent — one in
SQL and one in Rust, which must then agree for every codepoint in Unicode.

**Disposition: FIXED.** Both items deleted, 37 lines. The other five ports had
already dropped theirs and grown a guard test; this one had not, which is the
whole of the finding. `fhir-postgresql` now has the equivalent guard —
necessarily tailored, because its append-only guard *is* a `plpgsql` function,
so the siblings' "emits no function at all" assertion would be wrong here. `L3`
prohibits a *folding* function specifically, and the test asserts that plus the
continued presence of the legitimate one, so it cannot pass by accident if
`schema_wide_objects` ever returns nothing.

This also removes the reason F-18 was batched with **F-07**: it turned out not
to touch the chain pre-image or the store at all, only dead code in the map
crate. It needed no live PostgreSQL, and should not have waited for one.

**Mutation-verified** (`T11.10`): re-adding the emission to
`schema_wide_objects` fails the guard.

## F-19

**`fhir-mssql`'s T-SQL emitter carries MySQL doc comments, one of which
contradicts the code it documents.** Severity: Low. Violates `W16.4` in spirit.
*Found during the remediation pass, while writing the SQL Server annex.*

`fhir-mssql/crates/fhir-mssql-map/src/ddl.rs` is genuine, careful T-SQL, but
several of its doc comments were carried over unedited. Most are legitimate
cross-port comparisons ("tighter than MySQL's 3072", "the hash surrogate key the
MySQL port introduced"). Four are simply wrong about this code:

| Line | Says | Truth |
|---|---|---|
| 31 | "MySQL type mapping for the map's column types" | it is the SQL Server mapping |
| 113 | "MySQL has no `ADD COLUMN IF NOT EXISTS`" | SQL Server is the one that has none here |
| 127 | "because MySQL forbids defaults on `NVARCHAR(MAX)`" | `NVARCHAR(MAX)` is not a MySQL type at all |
| 228–235 | "Two triggers rather than PostgreSQL's one, because MySQL allows a trigger to name only a single event… Each is preceded by a `DROP … IF EXISTS`" | **the code emits neither**: it uses `CREATE OR ALTER`, and a unit test asserts no `DROP TRIGGER` appears |

The last is the one that matters. A doc comment describing a `DROP`-then-
`CREATE` pattern sits directly above code that deliberately avoids it — and
avoiding it is a *correctness* decision, because a `DROP`/`CREATE` pair leaves a
window in which history is unguarded. A reader trusting the comment would
"restore" the drop and quietly reopen that window.

**Disposition: FIXED.** All four are corrected, along with five more found in
the same pass: two `3072`-byte index budgets that are `900` here, a `SIGNAL`/
`MESSAGE_TEXT` note describing a MySQL construct where the code uses `THROW`, a
duplicated doc line on `index_columns` left by an incomplete edit, a foreign-key
comment attributing a schema-wide namespace to MySQL, and two citations of
requirement ids that do not say what they were cited for (`M14.12` and `M14.x`,
now `M14.15` and `M14.16`).

The trigger comment — the one that mattered — now states the `DROP`-then-
`CREATE` prohibition as the correctness decision it is and points at the test
that enforces it (`M14.19`).

**Reading this file closely enough to correct its comments is what found F-25
and F-26**, both High, both in the eight lines the comments were describing.
That is the argument for treating a Low documentation finding as worth the
pass: nobody had read the emitter against the engine it targets.

## F-20

**Every boolean and integer element was silently dropped on read.** Severity:
**High**. Violates `R4.2`, `R4.3`. **Fixed in `fhir-sqlite`; strongly suspected
in `fhir-mysql` and `fhir-mariadb`.**

`Patient.active` did not survive a round trip. Neither `true` nor `false` — the
element simply was not in the resource that came back:

```
active=true -> {"resourceType":"Patient","id":"t","name":[{"family":"Famt"}]}
```

**Mechanism.** SQLite binds `Bool`, `Int`, and `BigInt` all to `INTEGER`
(`M14.10`). The read path asked rusqlite for a `String`, which refuses to coerce
`INTEGER`, and the resulting error landed here:

```rust
if let Ok(Some(v)) = r.get::<_, Option<String>>(i + off) {
    map.insert(name.clone(), v);
}
```

`if let Ok(...)` discards the error, so the column was omitted from the row and
the element vanished from the resource. Every boolean and every integer, in
every resource, on every `get` — and `get` is what `get_versioned`, `get_all`,
and search materialization all funnel through.

This is the invariant the entire project exists to protect (`R4.2`), and the one
`C0.13` says a port may never trade away. The silence is a second violation:
`R4.3` requires data the engine cannot handle be an error naming the path, never
a quiet omission.

**Why the existing suite missed it.** Every resource `sqlite_store.rs`
round-trips is built from strings. There was no boolean and no integer in any
fixture, so a defect that dropped exactly those was invisible to 27 passing
tests.

**Disposition: FIXED** in `fhir-sqlite`. A `cell_text` helper renders each cell
to the text image the shared engine expects, switching on the column's declared
`ColTy` — booleans become `"true"`/`"false"` rather than `"1"`/`"0"`, since that
is what `reconstruct::prim_json` parses. Unrenderable cells are now an error
naming the column, not a silent omission. NULL remains a legitimately absent
element.

New regression suite `roundtrip_types.rs` (6 tests) pins booleans, `false`
specifically, integers, decimal precision, partial dates, and a resource mixing
all of them. It tests **types rather than resources**, which is the lesson: a
fixture-driven suite only covers the types its fixtures happen to contain.

### Confirmed in `fhir-mysql` and `fhir-mariadb` — and worse there

Both carried the identical shape:

```rust
if let Some(Some(v)) = row.get::<Option<String>, _>(i + off) {
```

This was first recorded here as *suspected*, on the reasoning that
`mysql_common`'s `FromValue for String` accepts `Bytes` only, so a `TINYINT(1)`
arriving over the binary protocol as `Value::Int` would fail to convert and be
dropped. **The cause was right and the effect was wrong**, and only running the
engines showed it:

```text
Could not retrieve `Option<String>`: Couldn't convert the value `Int(1)`
Could not retrieve `Option<String>`: Couldn't convert the value `Date("'1974-01-01'")`
```

`Row::get` **panics** on a failed conversion rather than returning `None`. So
these two ports did not lose a field quietly — **they could not read the
resource at all**, and the failure was a panic inside the store.

Three consequences that make this the most serious finding in the register:

1. **The scope is wider than SQLite's.** Dates fail too, because the derived
   `_sort` columns come back as `Value::Date`. Almost every real `Patient`
   carries `active` or `birthDate`, so in practice these ports could not read
   real FHIR data.
2. **A panic in a library is a denial of service** for whatever hosts it
   (`T11.9`). It is not catchable by a worker thread and not a clean error.
3. **Both were documented as Store level and shipped that way.**

Verified against `mysql:8.4` and `mariadb:11.4` in containers: six failures
each before the fix, six passes each after, with the full suites (35 tests per
port) green.

**Disposition: FIXED** in both, by the same `cell_text` treatment as SQLite —
switch on the declared `ColTy`, read the raw `Value` via `Row::as_ref` rather
than forcing a `String`, render booleans as `"true"`/`"false"`, and render the
temporal variants to ISO-8601 so the derived sort columns no longer blow up on
the way past. Anything unrenderable is an error naming the column.

### Why three ports had the same defect

Worth stating, because it is a structural lesson rather than three coincidences.

The shared engine reconstructs from a **text image** — every column arrives as a
`String` and `prim_json` parses it. That contract is implicit: nothing declares
it, and each store re-implements the "get me this cell as a String" step itself.
PostgreSQL's driver happens to render everything as text, so the reference port
never exercised the seam. Every port that did not use PostgreSQL got it wrong,
in the same place, in the way its own driver happened to fail.

`X15.1` keeps the *engine* identical across ports and is doing its job. The
cell-to-text-image conversion sits just below it, in the two-file dialect
surface, where nothing was checking. A shared, typed helper — or a `ColTy`-aware
trait the stores implement against — would have made this one bug instead of
three, and is the obvious follow-up.

## F-21

**Reads tore under concurrent writes.** Severity: **High**. Violates `R4.5`.
**Fixed in `fhir-sqlite`, `fhir-mysql`, and `fhir-mariadb`** — every port that
is not PostgreSQL.

`get()` issued its base-table query and every child-table query as independent
statements with no enclosing transaction. The only transactions in the file were
write-side `Immediate` ones.

A reader on a second connection observed:

```
{"id":"torn","active":true,
 "name":[{"family":"Family8","given":["Given8","Middle8"]}],
 "telecom":[{"system":"phone","value":"000012"}]}
```

— `patient_name` from version 8 beside `patient_telecom` from version 12. A
resource that never existed, which is `R4.5`'s stated failure verbatim.

**Why it hid.** `SqliteStore` is one connection behind a mutex, so two tasks
sharing one handle are serialised and cannot interleave. The defect is only
reachable from a **second handle** — a second connection on the same file, which
is an ordinary deployment shape and the natural one for a multi-process embedded
application. A test written against a single handle would have passed no matter
how broken the code was, which is why the new suite opens two.

The annex's `M14.19` says WAL is "what makes snapshot reads possible at all",
and that was true and insufficient: WAL makes a consistent read *possible*, it
does not take one for you.

**The same defect was then found in MySQL and MariaDB.** Both read with
`pool.get_conn()` and no transaction. `REPEATABLE READ` is the server default
there, which sounds like protection and is not: it is a property of a
*transaction*, and these reads were not in one. Because those stores are
pool-backed, the race is reachable through a single handle — reader and writer
simply land on different connections — so it needs no second handle to provoke.

Observed on MariaDB with the fix reverted:

```
{"id":"torn","name":[{"family":"Family0","given":["Given1","Middle1"]}], …}
```

— `family` from one version and `given` from the next, inside the same name.

**Disposition: FIXED** in all three. SQLite reconstructs inside a deferred read
transaction (`unchecked_transaction`); MySQL and MariaDB inside
`start_transaction`, rolled back at each exit since the read takes no write
lock. Mutation-verified on MariaDB: removing the transaction reproduces the torn
read.

## F-22

**`fhir-sqlite` version-guarded writes enforced nothing under contention.**
Severity: **High**. Violates the optimistic-concurrency requirement `T11.6`
tests. **Fixed.**

Eight writers presenting the same expected version produced **eight successes**.
Every one was told it won; seven updates were silently lost.

**Mechanism.** `put_audited` was a check-then-act across two transactions:
`self.status(…)` in one, `self.put(…)` in another, with nothing spanning the
gap. All eight racers read version 1, all found it matched, all wrote.

The sharpest detail is that the fix already existed and was not used. The store
carries a `write_gate` whose own doc comment explains precisely this hazard —
"a conditional create is a *search* followed by a write, and the engine's lock
does not span the gap between them" — and `conditional_create_audited` takes it.
`put_audited` did not.

**Disposition: FIXED.** `put_audited` now takes `write_gate` for the guarded
case and delegates to a `put_audited_locked` inner function, which
`conditional_create_audited` calls directly since `tokio::sync::Mutex` is not
reentrant. Unguarded writes skip the gate: `put` is a single `BEGIN IMMEDIATE`
that assigns its own version and has nothing to race, so locking them would
serialise every write in the process for no benefit.

## F-23

**Shred and reconstruct errors were flattened into `Other`.** Severity: Medium.
Weakens `R4.7`, `R4.3`. **Fixed in `fhir-sqlite`, `fhir-mysql`, and
`fhir-mariadb`.**

Both call sites did `.map_err(|e| StoreError::Other(e.to_string()))`, discarding
the typed `StoreError::Shred` variant that the enum already declares via
`#[from] ShredError`.

Two consequences, the second worse than the first:

1. A caller cannot distinguish a rejected *resource* from a failed *store*, so
   every bad submission looks like an internal fault — a 500 where the truth is
   a 400.
2. **Reconstruction integrity errors were flattened too.** `R4.7` requires that
   a row-consumption residue — "N of M stored rows unconsumed" — be reported as
   an integrity error. That is the signal that says stored data went unread,
   which is exactly what **F-20** was doing, and rendering it as an untyped
   string makes it indistinguishable from an I/O hiccup.

MySQL and MariaDB carried it identically, and it was found the same way — by
porting `redaction.rs`, whose last assertion is on the error *variant* rather
than only its text.

**Disposition: FIXED** in all three. Both sites use `?` and the existing
`#[from]` conversion. Asserted by each port's `redaction.rs`, which requires a
rejected write to be `StoreError::Shred` and to name the offending path without
echoing the value.

## F-24

**`fhir-mysql` and `fhir-mariadb` assigned `version_id` with no row lock.**
Severity: **High**. Violates `H5.4`. **Fixed, verified live.**

`H5.4` requires `version_id` be assigned "under a lock that serializes writers
for a given resource id". Neither port took one. Measured, eight concurrent
writers to a single resource:

```
1 succeeded, 7 refused: [2]
refused: ERROR 23000 (1062): Duplicate entry 'ver-2' for key 'patient_history.PRIMARY'
```

**Mechanism.** `put` opens a transaction and reads the chain tip with a plain
`SELECT … ORDER BY version_id DESC LIMIT 1`. Under `REPEATABLE READ` that is a
*consistent, non-locking* read, so every writer sees the same tip, computes the
same next version, and races to insert it.

**What was and was not at risk.** No data was corrupted and no update was
silently lost: the history table's primary key rejected the duplicates. That is
the whole reason this is `High` rather than `Critical` — correctness survived on
a backstop rather than on the intended control.

What did fail is availability and diagnosis. Seven of eight writes to a
contended resource failed, and they failed as a **raw duplicate-key database
error**, which a caller cannot distinguish from a genuine version conflict or
from a bug. For comparison, `fhir-postgresql` takes `SELECT … FOR UPDATE` and
`fhir-sqlite` serialises on `BEGIN IMMEDIATE`; both let all writers through.

**Disposition: FIXED.** Both ports now take
`SELECT version_id FROM <base> WHERE id = ? FOR UPDATE` before reading the tip,
mirroring PostgreSQL. Writers queue instead of colliding: **8 of 8 succeed**,
with consecutive versions `[2,3,4,5,6,7,8,9]` and a chain that verifies.

A create has no base row to lock, so racing creates of the same id still resolve
on the primary key — the same backstop, and the same behaviour as PostgreSQL.

**Mutation-verified** (`T11.10`): removing the lock returns the result to
`1 succeeded, 7 refused`.

### A note on the test that nearly missed it

The first version of this test asserted only that no two writers received the
same `version_id`. It **passed** — because seven writers had been refused and
one version is trivially distinct.

That is exactly the failure `T11.11` describes: an assertion looser than the
property it guards. The test now also requires that successes plus refusals
account for every writer, that a majority make progress, and that the versions
are consecutive. Without the second of those, a port that refused *every*
concurrent write would still have looked correct.

## F-25

**`fhir-mssql`'s upgrade DDL is MySQL syntax that SQL Server rejects.**
Severity: **High**. Violates `M14.18`, `O10.4a`. *Found while correcting F-19's
doc comments.*

`history_audit_columns` emitted

```sql
ALTER TABLE [r5].[patient_history] ADD COLUMN [actor] NVARCHAR(MAX) NOT NULL
```

T-SQL has no `COLUMN` keyword in `ALTER TABLE … ADD`. `ADD COLUMN` is MySQL and
PostgreSQL spelling; SQL Server's parser rejects it outright, so **every**
statement the upgrade path emits is a syntax error. The function is the port's
entire mechanism for bringing an installed schema up to `M3.15`/`M3.16`, and it
could never have executed a single statement.

Why the live test did not catch it. `crates/fhir-mssql-map/tests/mssql_ddl.rs`
installs a **fresh** schema, and a fresh history table gets the audit envelope
through `CREATE TABLE`. Nothing in the suite calls `history_audit_columns`, so
the port's one green run against `azure-sql-edge` never touched this path — a
reminder that "verified against a real engine" is scoped to the statements the
test actually runs.

**Disposition: FIXED.** The emitter now produces `ALTER TABLE … ADD [col] type`,
and `M14.32` records the rule. A unit test asserts no `ADD COLUMN`, no
backquote, and the `] ADD [` shape.

**Mutation-verified** (`T11.10`): restoring `ADD COLUMN` fails the test with the
offending statement in the message.

## F-26

**`fhir-mssql` adds a `NOT NULL` column with no default to tables that have
rows.** Severity: **High**. Violates `O10.4a`, `M3.15`. *Found in the same
eight lines as F-25.*

`audit_envelope_columns` bound `actor` to `NVARCHAR(MAX) NOT NULL` with no
`DEFAULT`. SQL Server refuses that on a populated table:

> ALTER TABLE only allows columns to be added that can contain nulls, or have a
> DEFAULT definition specified.

Every history table an upgrade touches has rows — that is what makes it an
upgrade — so even with F-25's syntax corrected, the first statement still fails.

The provenance is the same as F-25's. `fhir-mysql` and `fhir-mariadb` omit the
default because **their** engines forbid one on `TEXT`; the omission was copied
here along with the comment explaining MySQL's constraint. This port has no such
limit. `fhir-postgresql` and `fhir-sqlite` both carry
`NOT NULL DEFAULT 'unauthenticated'`, and this port had no reason not to.

Worth separating from F-25 rather than folding into it: they fail at different
layers — one at the parser, one at the engine's ALTER semantics — and fixing
either alone leaves the upgrade path broken. A reviewer who saw only the syntax
fix would reasonably believe the path now works.

**Disposition: FIXED.** `actor` is now
`NVARCHAR(MAX) NOT NULL DEFAULT 'unauthenticated'`, matching PostgreSQL and
SQLite; `M14.33` records the rule for every column the upgrade path adds. Rows
predating the envelope read as `unauthenticated`, which is the honest answer for
a change recorded before there was anywhere to record a principal.

**Mutation-verified** (`T11.10`): removing the default fails the test.

**Still unverified.** Both fixes are unit-tested, not engine-tested. Closing
them properly needs `history_audit_columns` executed against a populated history
table on a real SQL Server — which is `M14.29`'s live test growing an upgrade
case, and is the next thing this port needs.

## F-27

**Every port's `tasks.md` ticks off work no port has done.** Severity: **High**.
Violates `C0.9`, `W16.4`. *Found while checking whether F-19's fixes were
tracked as tasks; widened on the next pass, which is when the REST claims
surfaced.*

This is **F-01 in the file F-01 did not cover**. The READMEs were rewritten; the
task lists were not, and they make the stronger claim, because a `[x]` asserts
the work is finished. There are two distinct classes of false claim, and the
second is the one the first pass missed.

### Class 1 — a REST server and CLI, checked off in all six

`T16 axum skeleton` reads, in `fhir-postgresql/tasks.md`:

> `fhir-postgresql-server` crate + `fhir-postgresql serve`: versioned base
> paths, application/fhir+json, 32 MiB body limit, OperationOutcome error
> mapping … *Accept met* in the rest integration suite.

Against the tree:

| Claim | Actual |
|---|---|
| `fhir-*-server` crate | no port has one. Every workspace is exactly `-map`, `-gen`, `-store` |
| `fhir-* serve` | no binary target in any port except the two fuzz harnesses |
| "the rest integration suite" | no test file anywhere matches `rest`, `server`, or `http` |

`T8 CLI v1`, `T16`–`T20` (CRUD endpoints, search over HTTP, batch/transaction,
CapabilityStatement generation) and `T23 Multi-version serve` are `[x]` in **all
six** ports. None of that code exists in this repository. This is the same
fiction as `C0.17`'s book chapters and the same service `F-05` records the spec
as specifying — but stated as *completed and tested*, which neither of those
does.

### Class 2 — a store, checked off in the two scaffolds

`fhir-mssql/tasks.md` T75 is checked off and reads:

> `fhir-mssql-store` now depends on `mysql_async`, … *Accept:* 13 store tests
> against real MySQL 8.4, covering schema install, CRUD, history, vread, delete,
> search, chain verification, tamper detection, purge, the erasure flag, and the
> disclosure log.

`fhir-oracle/tasks.md` T75 is the same text with the crate name substituted.
Against the tree:

| Claim | Actual |
|---|---|
| depends on `mysql_async` | neither `Cargo.toml` lists a database driver at all — no `mysql_async`, no `tiberius`, no Oracle client |
| 13 store tests, real MySQL 8.4 | `crates/*-store/` has **no `tests/` directory**; `src/` is `lib.rs` and `chain.rs` |
| CRUD, history, search, purge … | none of those operations exist in either port |

The contamination is broad, not a stray line: `fhir-mssql/tasks.md` names MySQL
38 times against SQL Server 15, and `fhir-oracle/tasks.md` names MySQL 35 times.
Both carry `utf8mb4_bin` collation findings, `InnoDB` index-prefix arithmetic,
`GET_LOCK`, and "the reason this is not yet a working MySQL server" — MySQL
port history, verbatim, in ports that are **Scaffold** (`M14.28`).

Why High rather than Low. These are not stale comments beside correct code, as
F-19 was. They are *acceptance criteria recording tests that were never run on
an engine that was never provisioned*, in the file a contributor reads to learn
what is left to do. The natural reading of `fhir-oracle/tasks.md` is that its
store is finished and verified. Nothing in it is.

### Class 3 — PostgreSQL mechanisms, asserted in the ports that do not use them

Milder, because here the *work* exists — `fhir-sqlite`, `fhir-mysql` and
`fhir-mariadb` do have stores — but the description of it is the reference
port's. `fhir-sqlite`'s `T7` is checked off as "tokio-postgres + deadpool …
text-image wire protocol with explicit casts", `T11` as "under `FOR UPDATE` row
locks", `T12` as "staged-schema install + atomic rename", `T15` as an EXPLAIN
audit failing on "seq scans" with a note about `ILIKE`.

None of that is SQLite. It uses `rusqlite`, serializes on `BEGIN IMMEDIATE`
because it has no `FOR UPDATE`, and drops a schema by unlinking a file because
the schema *is* a file. Counting the three non-PostgreSQL store ports:

| | `tokio-postgres` | `FOR UPDATE` | staged-schema | `ILIKE`/seq-scan |
|---|:-:|:-:|:-:|:-:|
| `fhir-sqlite` | 3 | 2 | 2 | 1 |
| `fhir-mysql` | 5 | 2 | 2 | 1 |
| `fhir-mariadb` | 5 | 2 | 2 | 1 |

This class is Medium on its own. It is recorded here rather than separately
because it has one cause with the other two — a file copied per port and never
re-read — and one fix.

### How the three compare

Class 1 is worse in one respect and better in another. Worse: it is in **all
six**, including the reference port, so there is no clean sibling anywhere in the
repository to adapt. Better: nothing downstream depends on it — no conformance
claim rests on the REST layer, because [the matrix](conformance-matrix.md) has
never listed one.

Class 3 is the one most likely to mislead an *implementer* rather than a reader:
someone extending `fhir-sqlite`'s store who trusts `T11` would go looking for the
row lock that the port deliberately does not have, which is exactly the
reasoning **F-22** turned out to need.

### Why the checkboxes were not simply unticked

Because that would substitute one false statement for another. `[ ]` in these
files means "planned for this port", and it is not established that a REST
server *is* planned here — `F-05` records the opposite tension, that §10 and §12
specify a service no crate implements and the requirements were retained
deliberately. Deciding whether these ports are libraries that will grow a server
or libraries that will not is the owner's call, and it determines whether the
REST tasks should be unticked, deleted, or moved to a separate document.

**Disposition: open, with the misleading stopped.** Each of the six files now
carries a header naming what in it is untrue and pointing at the conformance
matrix, and the `M4 — REST server` section carries the same warning at the point
of use. That is a stopgap, not the fix.

The fix is six rewrites, and unlike the annexes (**F-16**) there is no correct
sibling to adapt: for the two scaffolds the work these files describe has not
been done, and for all six the REST milestone describes a decision nobody has
recorded making. Until then, `tasks.md` MUST NOT be cited as evidence of
anything in any port.

## F-28

**`schema_wide_objects` documented itself as idempotent on MySQL and MariaDB.
Three of its five statements are not.** Severity: Medium. Violates `G2.5`,
`W16.4`. *Found by writing the upgrade path F-15 needed.*

The doc comment read "written idempotently so that `init` and `init --upgrade`
can both apply them". The two tables do carry `IF NOT EXISTS`. The three
access-log indexes are bare `CREATE INDEX`, and MySQL has no
`CREATE INDEX IF NOT EXISTS` at all. Re-applying the list therefore fails:

```text
ERROR 42000 (1061): Duplicate key name 'fhir_mysql_access_log_subject_ix'
```

MariaDB is the same in effect for a different reason: it *does* support
`CREATE INDEX IF NOT EXISTS`, and the emitter simply does not use it.

**Latent, not live.** Nothing had ever re-applied the list — `init` runs once,
and until now there was no upgrade path. So no deployment was harmed. What makes
it worth recording rather than silently correcting is the shape: a comment
asserting a property, no test for that property, and the only caller structured
so the property never got exercised. The claim survived because nothing could
contradict it.

**Disposition: FIXED.** The comment now states exactly which statements are
idempotent and which are not, and points at the requirement (`M14.36`). The
upgrade path filters the index statements against
`information_schema.statistics`, and `a_second_upgrade_is_a_no_op` is the test
that would have caught it.

**Mutation-verified** (`T11.10`) on both ports: removing the filter reproduces
the `Duplicate key name` above.

## F-29

**`scripts/db.sh` did not give PostgreSQL the shared memory CI gives it.**
Severity: Low. Violates `O10.12` in spirit. *Found running the full corpus
locally while fixing F-07.*

The CI workflow starts its TLS PostgreSQL with `--shm-size=1g`. `db.sh` — whose
whole stated purpose is that "a green local run and a green CI run mean the same
thing" — started it with the container default of 64 MB. A full-corpus run then
died with:

```text
could not resize shared memory segment "/PostgreSQL.407775454" to 33554432
bytes: No space left on device
```

which reads like a full disk and is not one: PostgreSQL puts parallel-query and
hash-join workspace in `/dev/shm`.

The gap is small but it is the specific failure the script exists to prevent, and
it fails in the most misleading direction — a local run going red on a green
tree, with an error naming the wrong resource.

**Disposition: FIXED.** `--shm-size=1g` added to the PostgreSQL branch, with the
reason in a comment, in all six copies of the script. The other engines' branches
were left alone: none of them showed the symptom, and adding a knob nothing needs
would be its own kind of noise.

---

Part of the [fhir-databases specification](index.md).
