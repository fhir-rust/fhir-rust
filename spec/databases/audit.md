# Audit findings

Non-normative. This is the register of known divergences between what the
specification requires, what the documentation claims, and what the code does.
Every finding carries evidence a reader can check.

A finding stays here until it is fixed or the spec is amended to match reality.
Deleting a finding because it is inconvenient, or because the text that stated
it was rewritten, is the failure mode this file exists to prevent.

**Audit date:** 2026-07-31. **Remediation pass:** 2026-07-31.
**Documentation and publish-readiness pass:** 2026-08-01 (**F-30** to **F-34**).
**Scope:** all six ports at the tree's current state. **Method:** cross-port diff
of every shared file under a name-substituting normalization; read of every spec,
README, `Cargo.toml`, CI config, and `col_sql` binding; a repo-wide markdown link
check; `cargo package --list` and the crates.io API for publish readiness.

The 2026-08-01 pass added sixteen findings, and **all sixteen are fixed**
(**F-30**–**F-45**). **F-37**, carried over from the original audit, closed on
2026-08-02 as well.
It also closed **F-11**, which the merge into a monorepo resolved outright. It was driven by two changes of
context rather than by re-reading the same tree: the specification was hoisted
into `spec/databases/` so that `spec/` could hold more than one family, and
publishing every crate to crates.io became the stated goal. The first broke 247
links and made the root README's absent-crate problem visible; the second turned
metadata that had never mattered — `categories`, `version`, what `cargo package`
actually contains — into things a reader would download. The cross-family view
lives in [`spec/publishing.md`](../publishing.md); only the six ports' share of
it is recorded here.

Of the original twenty-nine findings, **twenty-three are fixed and five remain
open**; F-05 is
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
| [F-04](#f-04) | Medium | §11 and §13 cite retired §7/§8 requirements; three compliance rows rest on them | **closed** 2026-08-03 — restated as `SV` ids in `fhir-loco/spec/`; the strikes stay |
| [F-05](#f-05) | Medium | §10 and §12 specified a service no crate implemented — `fhir-loco` is that service | **closed** 2026-08-03; the gap it leaves is F-58 |
| [F-06](#f-06) | **High** | `fhir-mssql` and `fhir-oracle` live gates never ran their own engine | **fixed** |
| [F-07](#f-07) | **High** | `fhir-postgresql` derives its chain pre-image from `jsonb`; chains are not portable | **fixed**, verified live |
| [F-08](#f-08) | **High** | `fhir-oracle`'s DDL emitter was verbatim MySQL and could not produce an Oracle schema | **fixed** 2026-08-03 — full R5 schema installed on 26ai, 0 invalid objects |
| [F-09](#f-09) | Medium | `fhir-oracle` had no declared engine floor under a 63-byte identifier budget | **fixed** |
| [F-10](#f-10) | Medium | Nothing verifies that the shared Rust core has not diverged across ports | **fixed** |
| [F-11](#f-11) | Medium | All six ports share the ancestor project's git remote | **resolved** by the monorepo merge |
| [F-12](#f-12) | Medium | `spec/index.md` referenced `AGENTS.md` in every port; no such file existed | **fixed** |
| [F-13](#f-13) | Medium | Sections 1–13 were duplicated across six ports | **fixed** |
| [F-14](#f-14) | Low | `fhir-postgresql` had no dialect annex while the other five did | **fixed** |
| [F-15](#f-15) | Low | `_norm` backfill is unavailable on four ports after a fold change | **fixed** in sqlite, mysql, mariadb (live); mssql/oracle await a store |
| [F-16](#f-16) | **High** | The MSSQL and Oracle dialect annexes were the MySQL annex, unedited | **fixed** |
| [F-17](#f-17) | Medium | `fhir-postgresql` TLS defaulted to unverified, violating `O10.7` | **fixed** 2026-08-03 — default is `Require`; the "breaking" premise was false, the crates are unpublished |
| [F-18](#f-18) | Low | `fhir-postgresql` carried a SQL folding function nothing calls (never emitted — wording corrected) | **fixed** |
| [F-19](#f-19) | Low | `fhir-mssql`'s T-SQL emitter carries MySQL doc comments, one contradicting its code | **fixed** |
| [F-20](#f-20) | **High** | Booleans, integers and dates unreadable: dropped on sqlite, **panic** on mysql/mariadb | **fixed** in all three, verified live |
| [F-21](#f-21) | **High** | Reads tore under concurrent writes — no read transaction | **fixed** in sqlite, mysql, mariadb |
| [F-22](#f-22) | **High** | `fhir-sqlite` version-guarded writes enforced nothing; all racers won | **fixed** |
| [F-23](#f-23) | Medium | Shred/reconstruct errors flattened, erasing the integrity signal | **fixed** in sqlite, mysql, mariadb |
| [F-24](#f-24) | **High** | `fhir-mysql`/`fhir-mariadb` assigned `version_id` with no row lock: 1 of 8 concurrent writes succeeded | **fixed**, verified live |
| [F-25](#f-25) | **High** | `fhir-mssql`'s upgrade DDL emits `ADD COLUMN`, which T-SQL rejects: the upgrade path cannot run at all | **fixed** |
| [F-26](#f-26) | **High** | `fhir-mssql` adds `actor NOT NULL` with no default; SQL Server refuses that on any populated table | **fixed** |
| [F-27](#f-27) | **High** | All six `tasks.md` tick off a REST server and CLI no port has; the two scaffolds also tick off a store, and three describe PostgreSQL mechanisms they do not use | **fixed** 2026-08-03 — all three classes; class 1 resolved by the owner: the server is `fhir-loco` |
| [F-28](#f-28) | Medium | `schema_wide_objects` documented itself as idempotent on MySQL/MariaDB; its three indexes are not | **fixed** |
| [F-29](#f-29) | Low | `scripts/db.sh` did not give PostgreSQL the `--shm-size` CI gives it, so a local full-corpus run failed where CI passed | **fixed** |
| [F-30](#f-30) | Medium | Hoisting the core into `spec/databases/` broke 247 markdown links across 30 files | **fixed** |
| [F-31](#f-31) | **High** | Root README and index documented seven `openehr*` crates absent from this repository, and omitted `fhir/` and `fhir-store/` | **fixed** |
| [F-32](#f-32) | Medium | All six ports advertised a `command-line-utilities` category, contradicting `C0.18` | **fixed** |
| [F-33](#f-33) | **High** | A published crate could not obtain a relational map at all; assets were outside every package root | **fixed** — `RelMap::bundled()` behind `r3`/`r4`/`r5` features |
| [F-34](#f-34) | **High** | `Cargo.toml` says `0.1.0`; every `CHANGELOG.md` and `Cargo.lock` says `0.4.0` | **fixed** — owner chose `0.4.0` |
| [F-35](#f-35) | **High** | The published-vs-source gates compare `src/` only, omit five published crates, and have never run; `fhir-derive-macros` had diverged 206 lines at `1.1.0` | **fixed** — script added, versions bumped |
| [F-36](#f-36) | **High** | Four of six `search_sql` fuzz targets had never compiled, while their CI jobs claimed injection-safety coverage | **fixed** |
| [F-37](#f-37) | Medium | `fhir-store/` (now `fhir-loco/`) was a nested git repository with no remote, untracked by the monorepo | **fixed** — absorbed and committed |
| [F-38](#f-38) | **High** | `where()` value restrictions silently dropped; four parameters compiled to one identical target | **fixed**, assets regenerated via **F-40** |
| [F-39](#f-39) | **High** | 21 spec-dependent tests resolved the ancestor layout and silently skipped, including the reference port's whole store suite | **fixed** |
| [F-40](#f-40) | **High** | Nothing could reproduce the committed map assets; `G2.2` determinism was unverifiable | **fixed** — `regen-assets` added, assets regenerated |
| [F-41](#f-41) | **High** | Compressed asset bytes varied with `flate2` feature unification, making `G2.2` unsatisfiable on two ports | **fixed** — `G2.2a` added, drift now compares content |
| [F-42](#f-42) | **High** | The full-corpus round-trip skipped in all six ports; 7,399 examples per port had never been tested | **fixed** — now green in all six |
| [F-43](#f-43) | **High** | The `O10.10` supply-chain gate had never run; `fhir-store` had none, and carried three advisories in its normal tree | **fixed** — mssql documented; `fhir-store` upgraded to loco 1.0.1 |
| [F-44](#f-44) | Medium | `check-shared-core.sh` aborted under `set -u` on the empty `EXEMPT` array — only reachable once a file diverged | **fixed** |
| [F-45](#f-45) | Medium | The shared-core gate stopped at the store; `chain.rs` was 618 identical lines duplicated six times, unwatched | **fixed** — extracted to `fhir-store`, all six rewired |
| [F-46](#f-46) | Medium | `U11` cannot reach the extension and deep tables: they have no columns in the map, and the cheaper workaround contradicts `U2b` | **fixed** 2026-08-02, verified live |
| [F-47](#f-47) | Low | `path` and `v_kind` are bounded in practice but bound to unbounded types on mssql and oracle, so `U12` is unsatisfied; fixing it is a physical-schema migration for all six | open |
| [F-48](#f-48) | Low | the shared-core gate did not watch `gen/tests/`, and could not while its normalization was line-based — rustfmt wraps by crate-name *length* | **fixed** 2026-08-02 — token-based verdict, 75→100 files |
| [F-49](#f-49) | **High** | No workflow in this repository runs: all 20-odd sit under `<family>/.github/workflows/`, which GitHub does not read. Every "gated in CI" claim is unverified | open — root workflow added but inert until `scripts/` is committed; the other eight families' CI is an owner decision |
| [F-50](#f-50) | Medium | The `U2a` reference rule attached an adjunct to `c_url`, which no index uses, while every port indexes `(c_type, c_id)` — 453 of R5's 1,947 search targets unindexable on Oracle | **fixed** 2026-08-02 — all six; gaps now 0 |
| [F-51](#f-51) | Medium | `fhir-oracle`'s DDL was executed by hand, not by a test, so `C0.9` keeps the port at Scaffold; a live test needs an Oracle driver decision | open |
| [F-52](#f-52) | **High** | The repository's only live database test was flaky — its cleanup dropped tables before foreign keys and discarded the error, so failures were misattributed to a correct `CREATE TABLE` | **fixed** 2026-08-03 — 5/5 runs green |
| [F-53](#f-53) | Medium | Every store crate's module doc called itself "the PostgreSQL layer" and described operations the two scaffolds do not have — F-01 in `src/` | **fixed** 2026-08-03 — all six |
| [F-54](#f-54) | **High** | `fhir-mysql` and `fhir-mariadb` carried PHI over an unencrypted database link with no way to encrypt it — the `minimal` Cargo feature excluded TLS entirely | **fixed** 2026-08-03 — `SslMode`, verifying default, live-verified on both engines |
| [F-55](#f-55) | **High** | `scripts/db.sh` resolved the FHIR packages through the ancestor project's path and one developer's home directory in all six ports, so the live corpus suite could never find its inputs | **fixed** 2026-08-03 — 1,200 live round-trips now green on PostgreSQL 18 |
| [F-56](#f-56) | **High** | Every port's `book/` describes PostgreSQL and a REST server — F-01 in the long-form documentation, incl. telling a SQLite operator to back up with `pg_dump` | **fixed** 2026-08-03 — engine substitution corrected throughout; REST text now attributed to `fhir-loco` in all six banners |
| [F-57](#f-57) | Medium | `fhir-loco`'s CapabilityStatement declared a read-only server while the router served `POST`/`PUT`/`DELETE`, and named its software `fhir-store` | **fixed** 2026-08-03 — mutation-verified agreement test added |
| [F-58](#f-58) | Medium | `fhir-loco` is the service §10/§12 specify; five obligations remain unmet, incl. no stated requirement for the listener's own TLS | open — spec question **closed** (`fhir-loco/spec/`); the five gaps are `SV2.14`, `SV2.15`, `SV3.11`, `SV4.2`, `SV4.3` |
| [F-59](#f-59) | **High** | `fhir-loco/config/production.yaml` was an empty file, so `LOCO_ENV=production` refused to boot — the only environment it exists to run in | **fixed** 2026-08-03 — real config, 3 mutation-verified tests |
| [F-60](#f-60) | Medium | No example in `doc/` or `README.md` is compiled by anything; one calls a `fhir-postgresql`-only API from a SQLite tutorial | **fixed** 2026-08-03 — `scripts/check-doc-examples.sh` added; it found six real defects incl. an unparseable block, all 24 now compile |
| [F-61](#f-61) | Medium | All six `plan.md` describe PostgreSQL, a CLI, and a `-server` crate; three of the five crates they list have never existed | **fixed** 2026-08-03 — all six corrected, banners added |
| [F-62](#f-62) | **High** | Every port's `CHANGELOG.md` is `fhir-postgresql`'s; the two scaffolds announce a TLS security fix for a connector they do not have | **partly fixed** 2026-08-03 — banners + the security entry annotated in place; per-port history is an owner decision |
| [F-63](#f-63) | Medium | Status text in `doc/faq.md`, `doc/choosing-an-engine.md` and `AGENTS/release.md` had decayed — incl. "is this a FHIR server? No" and a fixed finding cited as blocking | **fixed** 2026-08-03 |

## What remains, and why

| Finding | Why it is not fixed here |
| --- | --- |
| **F-08** | Writing an Oracle DDL emitter. Two of its three blockers cleared this pass. The `VARCHAR2`/`CLOB` boundary — which `M14.9` said MUST be settled *before* `ddl.rs` is written — is now settled by [unbounded string search](unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) (`U1`–`U10`, `P6.9`), and an arm64 Oracle image is confirmed to exist and pull (`M14.23`). What remains is the order of work: the adjunct columns are a **map** change in `model.rs` and `gen/src`, which are shared verbatim across all six ports (`X15.1`), so that lands before any Oracle DDL. And an Oracle has still never been *started* here — it needs more memory than a default Podman machine has. |
| **F-15** | Done for `fhir-sqlite`. MySQL and MariaDB each need an `upgrade` plus a resumable `_norm` backfill (`L14`), and both first need `init` to start recording the map asset — like SQLite, they store only `map_checksum`. Each needs that engine live. `fhir-mssql` and `fhir-oracle` have no store to hang an upgrade on, so theirs arrive with **F-08** and the store work. |
| **F-17** | Changing the default from `Prefer` to `Require` is one line and is **breaking** for any deployment relying on the libpq-compatible default. That is the owner's call; the departure is recorded (`M14.27`) and the README now says to set `verify-full`. |
| **F-04** | Whether to restore §7 or amend the citing requirements is an owner decision. Reconstructing retired requirement text from its citations was considered and rejected: text nobody wrote would carry ratified authority in the section that maps to regulation. |
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
all** — `crates/*-store/src/` is now `lib.rs` alone, 48 lines that re-export the
shared audit chain and define an error type (`chain.rs` moved to `fhir-store`,
**F-45**), and neither has a store test file. Nothing round-tripped through
either engine, live or otherwise. `fhir-oracle` additionally has no map test
directory.

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
regulation.

### Reframed, 2026-08-03: this is not an independent decision

§7 and §8 were retired as "out of scope". That was right, and it was **out of
scope for the ports** — which are libraries. The owner has since confirmed the
REST API exists: it is **`fhir-loco`**. So these requirements were never
obsolete; they changed family, and nobody moved them.

The evidence that they are still live is that they are still being **cited**.
**F-57** — a real defect fixed this session, where the CapabilityStatement
declared a read-only server while the router served writes — is cited against
`A7.12`, a struck identifier. A retired requirement caught a live bug, which is
about as clear a signal as this register produces that the retirement was
filed under the wrong reason.

Measured against `fhir-loco`:

| Struck id | What it asks | `fhir-loco` |
| --- | --- | --- |
| `A7.12` | CapabilityStatement conformance | **implements it** — `GET /{version}/metadata`, corrected under **F-57** |
| `A7.11` | no submitted value echoed in an `OperationOutcome` | has an `OperationOutcome` mapper; the non-echo property is untested |
| `A7.10` | racing conditional creates yield exactly one resource | **absent** — no `If-None-Exist` handling in the router, though the store implements `conditional_create_audited` |
| `A7.8` | transport security at the service edge | **absent** — plain HTTP, TLS is expected from a proxy (**F-59**) |
| `M8` | `$export` | **absent** |

So one of the five is implemented, one is partly, and three are not. That is a
far more useful statement than "unsatisfied as written", and it is only
available now that the service has a name.

**It was the same decision as F-58**, and the owner made it on 2026-08-03:
`fhir-loco` has its own specification, at
[`fhir-loco/spec/`](../../fhir-loco/spec/index.md), ids `SV1.x`–`SV4.x`.

**The struck ids stay struck, and are restated rather than moved.** `A7.11`
becomes `SV2.7`, `A7.12` becomes `SV2.8`–`SV2.11`, `A7.10` becomes `SV2.14`,
`M8` becomes `SV2.15`, and `A7.8` becomes `SV3.11` — each citing the original.
Moving them would renumber across families, which `C0.5` forbids and which is
exactly how the `R4` collision happened. A reader tracing `A7.12` finds it in
`C0.16`, and `C0.16` now points at `fhir-loco/spec/`.

`C0.16`'s register and §13's compliance rows are updated accordingly. The three
regulated rows — HIPAA §164.312(e), ONC FHIR conformance, ONC Bulk Data — now
name a crate that can be audited rather than a service layer said not to exist.

**Closed.**

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

### Update, 2026-08-03: the service exists — it is `fhir-loco`

The owner identified it. **`fhir-loco`** (Loco.rs, Axum, Tokio, Hyper) mounts a
FHIR REST API over a store, currently `fhir-sqlite`. The premise above —
"no crate depends on `axum`" — is no longer true.

That resolves the framing question this finding opened: **"whatever service is
built" has been built**, so the `[service]` requirements now have an implementer
and are assessable rather than hypothetical. Retaining them rather than deleting
them was the right call, and this is the moment that pays off.

It does **not** mean they are met. Measured against `fhir-loco` — see **F-58**.

The `[service]` markers stay, and their meaning sharpens: they mark requirements
that bind `fhir-loco` and not the ports. A port is still not non-conformant for
lacking a `/metrics` endpoint.

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

**Update, 2026-08-02: the verification blocker is gone.** `M14.23` recorded that
an arm64 image exists and pulls; what was never known was whether it *runs*. It
does — the host VM needed 6 GiB rather than 2, and at 1.9 GiB the instance died
with `ORA-03113` after starting its listener. At 6 GiB, Oracle AI Database 26ai
Free (23.26.2.0.0) reaches `DATABASE IS READY TO USE!` and answers SQL.

Four assumptions became measurements (`M14.23b`): the 4000-byte `VARCHAR2`
ceiling, the 128-byte identifier limit, and — the two that matter beyond this
port — that a `CLOB` can be neither `=` compared (`ORA-22848`) nor indexed
(`ORA-02327`). Those two are the premise of `U1`–`U10`, and they are no longer
a citation.

A fifth changed a decision rather than confirming one: `BOOLEAN` **is** a real
column type on 26ai (`M14.23c`), so this port must declare its engine floor
before `ddl.rs` is written — 23ai+ gives `BOOLEAN`, older needs `NUMBER(1)` plus
a CHECK, and the two schemas are not interchangeable.

So both of F-08's blockers are cleared: the `VARCHAR2`/`CLOB` boundary by
`U1`–`U10` landing, and verification by an engine that now starts.

`M14.6`'s type table — "intended shape, none of it yet implemented or verified"
— has now been **executed**: all nine bindings create, plus the `U1` adjuncts
and `ords` (`M14.23d`). One row of it turns out to be misleading. Under
`AL32UTF8` a `VARCHAR2(4000 CHAR)` holds **4000 bytes, not 4000 characters** —
as few as 1000 for 4-byte codepoints — so the emitter must reckon in bytes, and
the `U1`–`U10` adjuncts are load-bearing for many more columns than the table
implies.

**`col_sql` is now ported and executed.** All eleven bindings it emits were
generated by the code, run against Oracle 26ai, and accepted (`M14.23d`).

Executing them earned its keep immediately. Ten were right; the eleventh —
`Bool` as `NUMBER(1) CHECK (VALUE IN (0,1))`, exactly as `M14.6`'s table reads
— fails with `ORA-02438`, because a `CHECK` must name the column it constrains
and `col_sql` is handed only a type. The constraint belongs to `create_table`,
and until it emits one a `Bool` column here is an unconstrained `NUMBER(1)`, so
this port MUST NOT claim `M14.8` (`M14.23e`). That is a design error in the
annex that no amount of re-reading would have found.

**The rest of the file is still MySQL**, and its header now says so
function-by-function rather than as one blanket warning.

`M14.5` — the user-versus-schema namespace question that changes every `CREATE
TABLE` — is now **decided**: three users, one per version, because the prefix
alternative spends identifier bytes `M14.2` already calls the tightest
constraint here. The `CREATE USER` privilege that costs is documented rather
than left to be discovered.

Porting `create_table` then surfaced a gap the annex never covered
(`M14.23f`). Its fixed-shape columns — `path`, `leaf`, `url`, `v_text`,
`v_kind` — are not `ColTy`-driven, so `M14.6`'s table says nothing about them.
The naive `TEXT → CLOB` translation creates, and then extension search fails:

```text
select count(*) from pext where path = 'Patient.name';
ORA-22848: cannot use CLOB type as comparison key
```

`path` is what extension search filters on. So `U1`–`U10` are load-bearing for
the extension tables as well, not only for `ColTy::Text` — and the generator
currently adds adjuncts only for `string` search targets. That is undecided and
now blocks `create_table`.

`fhir-oracle/crates/fhir-oracle-map/src/ddl.rs::col_sql` emitted `TEXT`,
`TINYINT(1)`, `DATETIME(6)`, `LONGTEXT`, and `COLLATE utf8mb4_0900_bin` — none
of which exist in Oracle — and its comments still discussed MySQL's 2038
`TIMESTAMP` range. The port's own `tasks.md` stated this plainly ("Scaffold
only… Nothing here is an Oracle schema") and `#[ignore]`d the eleven
MySQL-asserting tests, so the code was honest; the README (F-01) was not.

### Resolution, 2026-08-03: ported, and the port's own schema executed

**Fixed.** The whole emitter is Oracle. The evidence is an install, not a
review: the complete R5 schema — 158 resources, **9,636 statements** — applied
to Oracle AI Database 26ai Free (23.26.2.0.0) in one pass.

| | |
| --- | --- |
| tables | 7,358 |
| indexes | 9,479 |
| triggers | 158 |
| check constraints | 21,540 |
| foreign keys | 7,039 |
| **invalid objects** | **0** |
| unindexable search targets | **0** of 1,947 |

**Three defects that only executing it could find.** This is the part worth
keeping:

1. `NUMBER(1) CHECK (VALUE IN (0,1))` — how `M14.6`'s table reads — fails with
   `ORA-02438`, because a column check may not reference another column. The
   constraint moved to `create_table`, the only caller that knows the column
   name (`M14.23e`). That is what now earns `M14.8`.
2. **The append-only DELETE guard failed open.** MySQL's
   `COALESCE(@var, '') <> 'on'` translated to `NVL(SYS_CONTEXT(…), '')`, and in
   Oracle the empty string *is* NULL — so `NULL != 'x'` evaluates to NULL rather
   than TRUE, the `ELSIF` never fires, and an ordinary `DELETE` on history
   succeeded **with no error at all**. Sentinel is now `'unset'` (`M14.29a`).
   Re-verified by executing a forbidden DELETE: refused, row intact; then
   declared erasure: deleted.
3. `search_index_gaps` reported 453 of 1,947 targets unindexable, which turned
   out to be a shared-core defect in the `U2a` reference rule affecting all six
   ports (**F-50**).

The second is the one to remember. It is a security-relevant control that
silently permitted exactly what it exists to forbid, it came from a
mechanically faithful translation of working MySQL, and no amount of reading
would have surfaced it — which is the whole argument for `C0.9`'s "an engine has
run this port's own schema".

**New departures:** `M14.28` (no `CREATE USER`; three users are a documented
prerequisite), `M14.29`/`M14.29a` (erasure via `CLIENT_INFO`, non-empty
sentinel), `M14.30` (the `rid` index is its own statement), `M14.31`
(idempotency swallows `ORA-00955`/`-01408` only, never `OTHERS`), `M14.32`
(index the adjunct, or emit nothing and count it). `M14.27` is superseded.

**Level: unchanged at Scaffold.** The DDL is real and has been executed by the
engine it targets, which is what this finding was about — but `C0.8` defines
exactly four levels and `C0.9` requires the claimed one be justified by tests
that *run*. This was verified by hand with `sqlplus`; no test in the port does
it. **Schema** is one live test away, and that test needs an Oracle driver
(**F-51**). It is certainly not Store:
there is no driver and no store crate, so nothing has been written through this
schema by the port itself — only by hand, to prove the guards fire. The eleven
`#[ignore]`d MySQL-asserting tests still need replacing (`M14.25`, `T11.14`).

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
`init` to verify the server version and refuse below it.

*The fix left one loose end, closed 2026-08-02.* §1's engine table still read
"12.2+, undeclared … that floor has not yet been written into the annex, tracked
as F-09" — a pointer back at a finding that had already been fixed by the annex
it was pointing away from. Corrected, and the identifier fact behind it is now
measured on a live Oracle rather than cited (`M14.23b`). Requiring 23ai instead
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

Every port's `origin` was the upstream `fhirpg` repository, correct for at most
one of them. Nothing had been pushed. The related open question — whether six
products should keep a shared ancestor history, be squashed, or be re-rooted —
was recorded in the same place and was an owner decision.

**Disposition: RESOLVED by restructuring, 2026-08-01 — and the guidance it
generated is now itself wrong.** Verified: none of `fhir/`, `fhir-postgresql/`,
`fhir-sqlite/`, `fhir-mysql/`, `fhir-mariadb/`, `fhir-mssql/`, or `fhir-oracle/`
contains a `.git` directory. They are plain directories in one repository whose
single remote is:

```text
origin  git@github.com:fhir-rust/fhir-rust.git
```

There are no six remotes to disagree, so the finding as written cannot recur.
The owner question it recorded was answered by the merge into a monorepo.

Two things replace it, and neither is the original problem:

1. **That URL does not resolve anonymously** (404). So does a private
   repository, so this is not evidence it is absent — but it is not verified
   either. Tracked with the other declared-URL problems as
   [`spec/publishing.md`](../publishing.md) **P-5**.
2. **`fhir-store/` is a nested git repository with no remote at all**, and the
   parent lists it as untracked (`?? fhir-store/`). See **F-37**.

The standing "do not push" instruction in `AGENTS.md` and `CLAUDE.md` cited this
finding and its six-wrong-remotes reasoning. That reasoning is obsolete; both
files have been corrected rather than left to warn about a hazard that no longer
exists, because a caution that is visibly wrong is one contributors learn to
skip.

## F-37

**`fhir-store/` is a nested git repository the monorepo does not track.**
Severity: Medium. Violates `W16.15`. *Found while verifying F-11.*

`fhir-store/` contains its own `.git`, has **no remote configured**, and appears
in the parent's status as untracked:

```text
?? fhir-store/
```

Every other family member is an ordinary directory. This one is a repository
inside a repository, which git handles in whichever way is least useful: `git
add fhir-store` records a **gitlink** — a bare commit id pointing at a
repository that exists nowhere but this disk — rather than the files. A clone of
the monorepo would get an empty directory, and nothing would report an error.

It matters more now than it would have last week, because `fhir-store` is on the
publish list ([`spec/publishing.md`](../publishing.md) **P-3**) and its source
is not actually in the repository that would be released from.

**Disposition: FIXED, 2026-08-02 — owner absorbed it.** The nested `.git` was
removed and the directory (since renamed `fhir-loco`, **F-45**) committed to the
monorepo as `01c058e "Add fhir-loco/"`.

Verified rather than assumed: **31 files** are tracked under `fhir-loco/`, the
index contains **zero** entries with mode `160000`, and `fhir-loco/Cargo.toml`,
`src/auth.rs`, `src/app.rs`, `src/controllers/fhir.rs` and
`tests/requests/fhir.rs` all resolve in `HEAD`. A clone now gets the source
rather than an empty directory.

What the removal cost, recorded because it is unrecoverable: two commits on a
single `main` branch, **no remote**, no stashes, no tags, and nothing
committed-then-deleted. The working tree survived intact, so the loss is those
two commit messages and their timestamps, not any code. The history had never
been pushed anywhere, so nothing referenced it.

This also settles the question **P-5** answered from the other side: there is one
repository, `git@github.com:fhir-rust/fhir-rust.git`, it is what every one of
the 33 crate manifests now declares, and every directory in the tree is inside
it.

## F-12

**`spec/index.md` referenced a file that did not exist.** Severity: Medium.
**Fixed.**

Every port's `spec/index.md` contained "Operational guidance for contributors
lives in `AGENTS.md`". No `AGENTS.md` existed anywhere in the tree. The
monorepo now has [`AGENTS.md`](../../AGENTS.md) with [`AGENTS/`](../../AGENTS/) topic
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

**Disposition, 2026-08-03: FIXED. The default is `Require`.**

This sat open as an owner decision because changing it is "breaking for any
deployment relying on the libpq-compatible behaviour". That premise was never
checked, and it is false: **the database crates have never been published.**
`spec/publishing.md` records all eighteen names as still available on crates.io,
so no deployment relies on anything, and the cost the deferral protected is one
nobody is paying.

Weighed properly, the decision is not close:

- `O10.7` says a port **MUST** default to verifying. `Prefer` does not verify.
  This was not a choice between two defensible defaults; it was a standing
  violation of a MUST, and `C0.12` makes an undeclared departure a defect rather
  than an amendment.
- The connection carries PHI, and `Prefer` does not survive an active attacker:
  a server that declines TLS yields a plaintext link, one presenting a forged
  certificate yields an encrypted link to the attacker. Both look identical from
  the application's side.
- Before a first release is the cheapest moment this can ever be changed. After
  publication the same edit acquires exactly the migration cost cited as the
  reason not to make it — the argument for waiting gets *worse* with time.

Every weaker mode remains reachable; `PGSSLMODE=prefer` still selects it. What
changed is that it must now be asked for.

**What now enforces it:** `tests/ssl_default.rs`, five tests, no database
required — a security default gated behind having provisioned PostgreSQL would
be one more check that silently skips (`T11.12`). Mutation-verified per
`T11.10`: restoring `#[default]` to `Prefer` fails two of them independently,
the direct assertion and the unset-`PGSSLMODE` path.

The original defect is worth remembering for its shape: **one token** —
`#[default]` on the wrong enum variant — in a file nobody re-reads, with a
failure mode invisible from the application. That is why the test asserts
equality against `SslPolicy::Require` rather than something looser like "is not
`Disable`".

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

**Disposition, 2026-08-03: classes 2 and 3 fixed; class 1 remains the owner's.**

**Class 2 — done.** `fhir-mssql/tasks.md` and `fhir-oracle/tasks.md` were
replaced outright. Each was `fhir-mysql`'s file with the crate name substituted;
there was nothing in either to recover, because it recorded another port's
history. Each is now ~68 lines: what exists (the shared generator and engine,
and each port's own DDL emitter with its actual evidence), what does not (a
store, a driver, any write through the schema), and what has not been decided.
MySQL mentions fell from 38 and 35 to 2 each, both in the paragraph explaining
the rewrite.

**Class 3 — done, and this finding's own table was wrong.** The three store
ports' descriptions were corrected against the code rather than against this
entry:

| claim | sqlite | mysql | mariadb |
| --- | --- | --- | --- |
| `tokio-postgres` | false | false | false |
| `FOR UPDATE` | false — uses `BEGIN IMMEDIATE` | **true** | **true** |
| staged-schema install | false | false | false |
| `ILIKE` | false — `LIKE` | false | false |

The `FOR UPDATE` row above is the correction: MySQL and MariaDB both support and
**use** `SELECT … FOR UPDATE` (`mysql.rs:905`), so counting it as PostgreSQL
contamination in those two ports was wrong. In `fhir-sqlite` the string appears
only in comments explaining what it does instead. Checking the code rather than
trusting the earlier count is what surfaced this.

**One class-3 entry was a security claim, and it was the worst of them.** `T32
Encrypted database transport (O10.7)` was `[x]` in all four store ports and
described `SslPolicy`, a rustls connector, `PGSSLROOTCERT` trust anchors,
`Store::connect_with`, a `refuse_insecure_db` startup guard and a
`--allow-insecure-db` flag. Measured:

- `SslPolicy` and `connect_with` exist **only** in `fhir-postgresql`.
- `refuse_insecure_db` exists in **no port**, and there is no `serve` binary for
  it to guard (`C0.17`, `C0.18`).
- `fhir-sqlite` has no connection at all — it is a local file, so `O10.7` is
  satisfied vacuously and the entry now says so, along with what actually
  carries the risk instead (file permissions, disk encryption).
- `fhir-mysql` and `fhir-mariadb` configure no TLS. Both entries are now
  **unticked** — unlike the REST milestones this one is unambiguously planned,
  because `O10.7` requires it.

`fhir-postgresql`'s own entry additionally claimed its TLS-only live test "now
runs in CI". It does not: the job exists, but no workflow in this repository
executes (**F-49**).

### Class 1 — resolved 2026-08-03: the server exists, in `fhir-loco`

The owner settled it: **the REST server is [`fhir-loco`](../../fhir-loco/)** —
Loco.rs, Axum, Tokio, Hyper — mounted over `fhir-sqlite`.

That reframes these entries entirely. They were not "planned and unfinished";
they were **misattributed**, inherited from the ancestor project where the
server lived inside the port. So deleting them is right and unticking them would
have been *wrong*: `[ ]` asserts the port is going to grow a server, and it is
not (`C0.17`, `C0.18`). Recording the question as undecided rather than guessing
turned out to matter — both available guesses were incorrect.

Each store port's `M4` section now says where the server is and what it serves:

| Route | Methods |
| --- | --- |
| `/{version}/metadata` | `GET` |
| `/{version}/{rtype}` | `GET` (search), `POST` (create) |
| `/{version}/{rtype}/{id}` | `GET`, `PUT`, `DELETE` |
| `/{version}/{rtype}/{id}/_history` | `GET` |
| `/{version}/{rtype}/{id}/_history/{vid}` | `GET` |

`T23` (multi-version serve), `T-graceful`, `T46` (CapabilityStatement) and
`fhir-sqlite`'s `T64b` are redirected rather than deleted: the work is real, and
each now names `fhir-loco` instead of a `fhir-<engine> serve` binary that has
never existed.

**Checking `fhir-loco` against those entries found a live defect** — see
**F-57**. `tasks.md` may now be cited for REST work only by pointing at
`fhir-loco`; no port implements one.

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

## F-30

**The specification moved to `spec/databases/` and left every relative link to
it broken.** Severity: Medium. Violates `W16.8`. *Found by a repo-wide link
check while writing the monorepo spec index.*

Hoisting the core into `spec/databases/` (so that `spec/` could hold more than
one family) invalidated **247 markdown links** in 30 files: `spec/…` from the
repository root, `../spec/…` from each port, `../../spec/…` from each port's
`spec/` directory, and — in the other direction — `../AGENTS.md`,
`../doc/trust-boundary.md`, and six `../fhir-<engine>/spec/index.md` from inside
`spec/databases/index.md`, all of which now needed one more `../`.

Nothing in CI checks markdown links, which is why a move that touched no prose
broke a quarter of the cross-references in the documentation set.

**Disposition: FIXED.** All 247 rewritten, each only where the current target
did not exist and the candidate did. Re-verified: zero broken links outside
`fhir/fhir.md`, which is a 22 MB generated transcript and out of scope.

Two further link classes were corrected by hand, because they resolved but to
the wrong document: 34 port-level references to "the shared core" pointed at
`spec/index.md`, which is now the **monorepo** index, and were repointed at
`spec/databases/index.md`.

## F-31

**The root README and index documented seven crates that are not in this
repository, and omitted the two families that are.** Severity: **High**.
Violates `C0.11` and `W16.8`. *Found by the same link check.*

`README.md` carried an "Also here: openEHR" section describing `openehr`,
`openehr-store`, and five engine crates, with conformance levels ("237 of 291
requirements verified"), findings ids (**A-13**, **A-14**, **A-15**), and a
verification script. `index.md` carried the matching table. **None of those
directories exist here** — all 13 links were dead.

In the other direction, both files were silent about `fhir/` (the model crate
and its 14-section specification) and `fhir-store/` (the HTTP surface), which
are two of the repository's three families.

This is F-01's failure mode surviving in a new place: measured-sounding claims
about code the reader cannot find, in the document that introduces the project.

**Disposition: FIXED.** The openEHR sections are replaced with accurate ones for
`fhir/` and `fhir-store/`, including the fact that `fhir-store` has no
specification at all. The root of every specification is now
[`spec/index.md`](../index.md).

## F-32

**All six port workspaces advertised a command-line-utilities category, in
libraries that have no CLI.** Severity: Medium. Violates `C0.18`. *Found while
auditing publish readiness.*

Every port declared `categories = ["database", "science",
"command-line-utilities"]`. Section 8 (CLI) is retired, `C0.18` says there is no
CLI crate in any port, and `AGENTS.md` says so twice. `categories` is published
metadata: it decides which crates.io listing the crate appears in, so this is a
claim made to readers who are browsing for a command-line tool.

**Disposition: FIXED.** Now `["database", "science", "data-structures"]` in all
six; `cargo metadata` re-run on three workspaces to confirm the manifests parse.

## F-33

**A published crate could not obtain a relational map.** Severity: **High**
(raised from Medium). Violates `G2.1` in effect. *Reframed after examining what
`cargo package` actually contains.*

Originally recorded as "packaged store crates ship tests that read an asset
outside the package". True, and the smaller half.

`RelMap::from_gz_bytes` was the only constructor; the maps lived at the port
root, outside every package root, so `cargo`'s `include` could not reach them.
`cargo package --list` confirmed **zero** asset files in `fhir-sqlite-map` and
`fhir-sqlite-store`. A consumer who ran `cargo add` got an engine with nothing
to walk — no shred, no reconstruct, no `init`, no `get`.

`G2.1` says the artifacts are committed "so that builds and installs never
require the spec packages". That held for a git checkout and failed for an
install, which is the case the wording covers.

**Disposition: FIXED** — owner chose `include_bytes!` behind per-version
features, the shape the model family already uses for its release crates.

1. `assets/` moved from the port root into the **map crate**
   (`crates/fhir-<engine>-map/assets/`), because `include` cannot escape a
   package root. Every path that referenced it moved with it: the `gen` crate's
   `assets_root`, the map and store test helpers, six port READMEs, ten crate
   READMEs, the root README, and two `doc/` pages.
2. `RelMap::bundled(version)` embeds the map with `include_bytes!` behind
   features `r3`, `r4`, `r5`, with `r5` on by default. `bundled_versions()`
   reports what a given build carries; `BundledError` distinguishes "feature
   off" from "no such version" from "corrupt bytes".
3. The `store` crates forward the same three features to the map crate, since a
   store without a map is useless.

Verified: `fhir-sqlite-map` now packages **19 files, 2.6 MiB** including all
three maps and `CHECKSUMS.txt`; `bundled()` returns 117, 146 and 158 resources
for R3, R4 and R5; an unknown version errors rather than panicking; and clippy
is clean at `<default>`, `--no-default-features`, `--features r3,r4,r5`, and
`--no-default-features --features r4`.

**The honest limit of this fix.** The features gate **compilation, not
download**. A `.crate` archive is static, so all three maps travel with it
(~2.5 MB) however few are enabled. Only separate data crates would make the
download opt-in, and that is a different trade — the caveat is stated in
`bundled`'s own doc comment rather than left for a user to discover from their
lockfile.

**A correction to the crate READMEs.** They told the reader to
`std::fs::read("assets/…")` — a path that exists in a checkout and nowhere else.
Written while looking at the repository rather than at the package, which is the
same mistake as the finding, in miniature. All of them now call `bundled()`, and
both the map and store examples were compiled against the real crates to confirm
they build.

## F-34

**The port version in `Cargo.toml` is contradicted by the port's own
changelog.** Severity: **High**. Violates `O10.11`. *Found while auditing
publish readiness.*

All six `Cargo.toml` files declare `version = "0.1.0"`. All six `CHANGELOG.md`
files open with `## 0.4.0 — tamper evidence that survives the database
(2026-07-27)`, and [`AGENTS/release.md`](../../AGENTS/release.md) asserted "All
six currently sit at `0.4.0`".

**The committed lock files settle which way it went.** All six `Cargo.lock`
files pin their own three crates at `0.4.0`:

```text
fhir-postgresql  manifest=0.1.0  lock=0.4.0   MISMATCH
fhir-sqlite      manifest=0.1.0  lock=0.4.0   MISMATCH
fhir-mysql       manifest=0.1.0  lock=0.4.0   MISMATCH
fhir-mariadb     manifest=0.1.0  lock=0.4.0   MISMATCH
fhir-mssql       manifest=0.1.0  lock=0.4.0   MISMATCH
fhir-oracle      manifest=0.1.0  lock=0.4.0   MISMATCH
```

A lock file records what the manifests said when it was last resolved, so the
ports **were** at `0.4.0` and the manifest was reset to `0.1.0` without
regenerating the lock. Four sources say `0.4.0` — changelog, lock, release
guide, and the release history itself — and one says `0.1.0`. Running any cargo
command that re-resolves rewrites the lock to `0.1.0` and destroys the evidence;
that was observed during this audit and reverted deliberately.

`O10.11` exists precisely for this: a published version must match the source
that claims it, and a crates.io version is immutable. Publishing `0.1.0` would
put a crate on the registry whose changelog documents three releases that never
happened at that number.

**Disposition: FIXED — owner chose `0.4.0` (2026-08-01).** Set in all six
`[workspace.package]` blocks, and in the eighteen `[workspace.dependencies]`
sibling pins that carried the same stale `0.1.0`.

**The lock files then confirmed the diagnosis mechanically.** Re-resolving all
six workspaces after the change produced **zero** modifications to any
`Cargo.lock` — the locks already said `0.4.0`, so the manifests were the stale
side, exactly as the mismatch implied. Had `0.1.0` been correct, all six locks
would have been rewritten.

One consequence to carry forward: none of the four releases in the changelogs
(`0.1.0` through `0.4.0`) ever reached crates.io — all eighteen port crate names
are still unregistered. The first publication will therefore be `0.4.0`, with no
`0.1.x`–`0.3.x` beneath it on the registry. That is legal and not unusual, but a
reader comparing the changelog to the version history will see three entries
with no artifact, so the changelogs should say where those releases lived. See
[`spec/publishing.md`](../publishing.md) **P-2**.

## F-35

**Nothing verified that an already-published version still matches its
source, and one had already diverged.** Severity: **High**. Violates `O10.11`
and `W16.6`. *Found while auditing publish readiness; the divergence itself was
found by diffing the tree against the registry.*

`O10.11` requires that a published version match the source claiming it, and
says CI must fail otherwise.

**A correction to this finding as first written.** It said no such check
existed. That is false, and the error is worth keeping visible: every port has a
`published-versions` job in `.github/workflows/ci.yml`, and `fhir/` has an
equivalent one whose comment documents a *previous* instance of exactly this
defect ("fhir-derive-macros 1.0.1 knew releases r2/r6 here and only r3/r4/r5 on
crates.io … Hence 1.1.0"). The gates were written by someone who had already
been bitten. The finding is not that they are absent; it is that three separate
gaps let the same defect through again:

1. **They compare `src/` only** — `diff -rq .../src "$dir/src"`. A changed
   `README.md`, `Cargo.toml`, or `LICENSE` is invisible to them, which is
   exactly how `fhir-release-1`'s README drift and five changed `license` lines
   survived.
2. **`fhir/`'s job omits five published crates** — `fhir-release-1`, `-7`,
   `-8`, `-9`, `-10`, all registered at `0.0.0` and none of them checked.
3. **None of them has ever run.** Nothing in this tree has been pushed
   (**F-11**), so every one of these jobs is unexecuted. A gate that has never
   run is indistinguishable from one that does not exist, which is how it
   looked from inside the tree — and `T11.9` says as much about fuzz targets.

The divergence was found by running the check locally for the first time, and
it was not hypothetical:

| | `fhir-derive-macros` `src/lib.rs` |
| --- | --- |
| published `1.1.0` | 554 lines |
| tree, also claiming `1.1.0` | 758 lines |
| | **206 added, 2 removed** — the `qty-3` invariant support |

`Cargo.toml.orig` was byte-identical, so nothing in the metadata hinted at it.
The reason it survived is the one `AGENTS/release.md` names: every local build
resolves the **path** dependency and never fetches the registry copy, so the
workspace stayed green against 758 lines while `fhir-derive-macros = "1.1.0"`
resolves to 554 for everyone else. It surfaces only when a third party packages
a dependent, as an error about code they did not write.

This is a database-family finding by consequence rather than by location: the
ports do not depend on `fhir-derive-macros`, but `O10.11` and `W16.6` are the
database specification's requirements, and the missing gate was missing for all
six too.

**Disposition: FIXED.** `scripts/check-published-match.sh` closes all three
gaps: it covers **all 32 crates** in one run rather than eight or three, it
compares **every packaged file** rather than `src/` alone, and it is a script
rather than a CI step, so it can be run before pushing instead of only after.
It fetches the registry copy through cargo, ignores `.cargo_vcs_info.json`,
`Cargo.lock`, and the normalized `Cargo.toml` — all of which legitimately vary —
and compares `Cargo.toml.orig` and every other file, so manifest and README
changes are caught.

It does not replace the CI jobs; those should keep running, and should be
widened to the same file set and crate list when this tree is finally pushed.
Crates whose version is not yet published are **printed as skipped**, never
silently passed, and a run that compares zero crates says so explicitly rather
than reporting a green it did not earn (`T11.12`).

Running it found two more divergences beyond `fhir-derive-macros`, both at
`0.0.0`: `fhir-release-1`'s `README.md` had gained a "What is actually
available" section not in the published copy, and all five reservation crates
had a `license` line changed by the quintuple harmonization earlier the same
day. Every one is the same defect — a changed tree on a published number.

Versions bumped so the tree stops claiming numbers it no longer matches:
`fhir-derive-macros` to `1.2.0` (behaviour added, nothing altered) with its six
dependency pins, and `fhir-release-1`, `-7`, `-8`, `-9`, `-10` to `0.0.1`. The
gate is green afterwards, and reports that it is green *vacuously*.

## F-36

**Four of six `search_sql` fuzz targets had never compiled.** Severity:
**High**. Violates `T11.9`. *Reported from a build failure, then found to affect
four ports.*

The reported error was one port's:

```text
error[E0433]: cannot find `search` in `fhir_mariadb_store`
  --> fuzz_targets/search_sql.rs:63:43
```

It is the substitution failure again (**F-01**, **F-02**, **F-08**, **F-16**).
Every port's `search_sql.rs` was copied from `fhir-postgresql`, whose search
module is named `search`, and the module name was never adjusted:

| Port | Target referenced | Module actually named | |
| --- | --- | --- | --- |
| `fhir-postgresql` | `search` | `search` | ok |
| `fhir-sqlite` | `sqlite_search` | `sqlite_search` | ok — had been renamed |
| `fhir-mysql` | `search` | `mysql_search` | **broken** |
| `fhir-mariadb` | `search` | `mariadb_search` | **broken** |
| `fhir-mssql` | `search` | *none — no store* | **broken** |
| `fhir-oracle` | `search` | *none — no store* | **broken** |

What makes this `T11.9` rather than a build error is what the target claims.
Its CI job documents the property it checks — "every attacker-controlled search
value must end up in the bind list, never spliced into the SQL … Verified by
breaking it" — and that is an **injection-safety** claim. Four ports asserted it
while the target could not build. `T11.9` says a committed target that never
executes proves nothing; one that cannot compile is worse, because the CI job,
the seed corpus, and the comment all still read as coverage.

It survived because **CI has never run** on this tree (**F-11**), and fuzz
crates are deliberately outside the workspace — nightly-only `libfuzzer-sys` —
so no ordinary `cargo build` or `cargo check --workspace` reaches them.

**Disposition: FIXED.** `fhir-mysql` and `fhir-mariadb` now name their real
modules. The two scaffolds have no search module to point at, so their
`search_sql.rs`, its `[[bin]]` stanza, and its CI matrix entry were **removed**
rather than left broken, with the reason recorded in `fuzz/Cargo.toml`; the
target returns with the store. Verified by compiling all six fuzz crates on
nightly — previously four failed, now all six pass.

## F-38

**Search parameters silently dropped their `where()` value restriction, making
four of them return identical rows.** Severity: **High**. Violates `P6.1` and
`C0.11`. *Found while scoping a FHIRPath evaluator.*

`compile_alt` skipped every `where(...)` segment with one line:

```rust
if seg.starts_with("where(") {
    continue; // typed-reference restriction — lenient
}
```

The comment is true of some `where()` clauses and false of others, and the
difference decides whether skipping is lenient or wrong. Measured against R5's
`search-parameters.json`, 116 clauses:

| Form | Count | Dropping it is |
| --- | ---: | --- |
| `where(resolve() is Patient)` | 64 | **lenient** — the reference column stores the target type beside the id, so a query can re-apply it |
| `where(type='derived-from')` | 51 | **wrong** — nothing downstream knows to re-apply it |
| `where(...exists())` | 1 | refused already |

All 51 value-restricted pairs — 8 distinct parameter codes across 51
`(resource, parameter)` combinations — compiled **with targets**, so the
restriction vanished without a note.

**The consequence, verified in the shipped R5 map.** `ActivityDefinition`'s
`composed-of`, `derived-from`, `predecessor`, and `successor` all compiled to
one identical target:

```text
composed-of    [{"kind":{"Uri":{"col":"resource"}},"table":25}]
derived-from   [{"kind":{"Uri":{"col":"resource"}},"table":25}]
predecessor    [{"kind":{"Uri":{"col":"resource"}},"table":25}]
successor      [{"kind":{"Uri":{"col":"resource"}},"table":25}]
```

Four different clinical questions, one answer. Asking what a Measure is
*derived from* returns its predecessors and successors too. This is `U2`'s
argument in a second place: a search that matches on an access path without the
discriminator returns rows it was never asked for.

**Disposition: FIXED in the generator; the shipped maps are not yet
regenerated.** `compile_alt` now distinguishes the two forms and **refuses** a
value restriction rather than dropping it, so the parameter is recorded
unsupported with a reason. A search that answers a question it was not asked is
worse than one that says it cannot (`C0.11`, `T11.12`).

Regenerating R5 from the real definitions confirms the effect and its cost:

| | before | after |
| --- | ---: | ---: |
| parameters with targets | 1870 (94.8%) | 1823 (92.4%) |
| the four colliding codes | 1 identical target each | 0 targets — unsupported |

47 parameters became unsupported. Four keep targets because their expression
also has a non-`where` arm — `depends-on` still compiles `ActivityDefinition.library`
— so those now **under**-match where they used to over-match, with the dropped
arm recorded in the note. That is a changed failure mode, not an eliminated one,
and it is the honest one of the two.

**The assets have since been regenerated** (**F-40**), so the fix is delivered
rather than pending. Measuring first showed why that was safe: for the four
ports with stores the regenerated map has **exactly the same 58,642 columns**,
so no table or column changes and an installed database stays structurally
valid — only `map_checksum` and the search compilation move.

**A measurement that nearly became a false finding.** Counting "partially
compiled" parameters gives 696 (35.3%), which reads alarming. 650 of them carry
*only* "path does not start at X" notes — the expected result of a union
expression that lists arms for other resources. Just 46 have a real failure.
Classifying the notes before reporting the number is the difference between a
finding and a scare.

## F-39

**Every spec-dependent test resolved the ancestor project's directory layout,
so they silently skipped.** Severity: **High**. Violates `T11.12`. *Found while
looking for the tool that regenerates the map assets.*

Twenty-one test files located the FHIR specification packages by a hard-coded
relative path into a sibling checkout named `fhir-rust-crate`:

```rust
Some(PathBuf::from(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../fhir-rust-crate/doc/fhir-specifications"
))),
```

That directory does not exist in this monorepo — the packages are in
`fhir/doc/fhir-specifications`, and the example corpora in `fhir/tests/data`.
The resolver returns `None`, the test prints `skipping: no spec dir` and
returns, and the harness reports **ok**.

`T11.12` exists because a silent skip reads as a pass. Here it read as a pass in
the tests that matter most:

| File | What was not running |
| --- | --- |
| `*-gen/tests/roundtrip.rs` (×6) | shred/reconstruct over the FHIR example corpus, all three versions |
| `*-gen/tests/proptest_roundtrip.rs` (×6) | property-based round-trip |
| `fhir-postgresql-store/tests/` (×8) | upgrade, concurrency, audit, redaction, chain portability, search semantics, bench, M2 semantics |

The last row is the sharp one: those eight are the evidence the conformance
matrix cites for `fhir-postgresql` being the **Reference** port. They need a
database as well, so a DSN-less run would skip regardless — but the spec-dir
gate meant they could not run *even with* a database.

**Disposition: FIXED.** All 21 files now resolve `fhir/doc/fhir-specifications`
and `fhir/tests/data`. The ancestor path was removed rather than kept as a
second candidate: it cannot exist in this layout, and a dead fallback is what
made the skip look deliberate.

Verified by running what had never run:

```text
r3: 52 examples, 0 failures
r4: 48 examples, 0 failures
r5: 53 examples, 0 failures
```

153 examples round-trip cleanly, which is good news obtained the only way it
could be — by making the test capable of failing first.

**Still skipping, and saying so:** `*-gen/tests/corpus.rs` uses a third
convention, `spec-cache/` beside the workspace root, matching a CI fetch step.
It prints `skipping: corpus or spec dir missing` rather than passing quietly.
Three different resolution conventions across one test suite is its own small
defect; unifying them is deferred, not forgotten.

## F-40

**Nothing in the repository could reproduce the committed map assets.**
Severity: **High**. Violates `G2.1` and `G2.2`. *Found while trying to deliver
**F-38**'s fix.*

`G2.1` requires the maps to be "generated ... by the port's `gen` operation" and
committed under `assets/`. `G2.2` requires generation to be deterministic and
records a SHA-256 per artifact in `assets/CHECKSUMS.txt`.

`generate()` existed, as a library function called **only from tests**. There
was no `[[bin]]`, no `build.rs`, no script, and no CI step that wrote an asset.
So:

- the committed `assets/*.json.gz` could not be reproduced from the tree;
- `CHECKSUMS.txt` attested to artifacts of unknown provenance — the checksums
  verified, which proved the files had not been corrupted, not that they were
  the generator's output;
- `G2.2`'s determinism requirement was unverifiable in principle;
- and **F-38** could not be delivered at all, because its fix lives in the
  generator and every port ships a pre-generated map.

**Disposition: FIXED.** A committed tool now exists in each port:

```sh
cargo run -p fhir-<engine>-gen --bin regen-assets            # write
cargo run -p fhir-<engine>-gen --bin regen-assets -- --check # gate, writes nothing
```

`C0.18` is what makes this legitimate rather than a new CLI: it states that
`gen` "names a **library operation** in this monorepo", and `G2.1` names that
operation as what produces the assets. The logic lives in `gen/src/assets.rs`
and the binary is a thin `main`, so the writer and the checker cannot drift
apart. There is still no CLI *crate* in any port.

Both are backed by `gen/tests/assets_current.rs`, which fails when the committed
assets are not what the generator produces, and by
`regenerate_is_deterministic`, which asserts `G2.2` directly by generating
twice. Both skip **loudly** when the FHIR packages are absent, because absent
input is not drift (`T11.12`).

Determinism was verified rather than assumed: `flate2`'s `GzEncoder` writes
`MTIME = 0`, so the gzip container carries no timestamp and repeated runs are
byte-identical. Had it embedded a timestamp, `G2.2` would have been impossible
to satisfy and nobody would have known.

`assets.rs` and `bin/regen-assets.rs` are shared core and are now gated by
`scripts/check-shared-core.sh`, which went from 65 files to **75**.

### What running it found

The first `--check` reported drift in all eighteen artifacts — correctly, since
**F-38** had changed the generator. Before regenerating, the change was measured
rather than assumed:

| Port | Columns before → after | Search params with targets |
| --- | --- | --- |
| `fhir-sqlite` (and pg, mysql, mariadb) | 58,642 → 58,642 (**no schema change**) | 1870 → 1823 |
| `fhir-mssql` (and oracle) | 58,642 → 59,116 (**+474**: the `U1` adjunct pairs) | 1870 → 1823 |

That mattered for the migration question. For the four ports that have stores,
regenerating changes **no table and no column** — only `map_checksum` and the
corrected search compilation, so an installed database stays structurally
valid. The two ports whose schema does grow have no store and therefore no
database to migrate. Regeneration was safe on both counts, which is why it was
done rather than deferred again.

The regenerated checksums also confirm `U9` by construction: the four ports
that must not materialize adjuncts produce **one identical map**, and the two
that must produce a **different identical map**. Two contents, exactly matching
`ddl::TEXT_ADJUNCTS`.

## F-41

**Asset determinism depended on an unrelated dependency's feature flags.**
Severity: **High**. Violates `G2.2`. *Found by the gate added for **F-40**,
within an hour of adding it.*

`fhir-mysql-gen`'s new `assets_current` test failed under `cargo test
--workspace` while the same check passed under `cargo run -p fhir-mysql-gen`,
against assets the latter had just written. Only `fhir-mysql` and
`fhir-mariadb` were affected; the other four ports were green.

The cause is Cargo feature unification:

```text
flate2 feature "zlib"          <- mysql_async (the driver, in the store crate)
flate2 feature "miniz_oxide"   <- fhir-mysql-map (default)
```

`flate2` selects its compression backend by feature, and features unify across a
workspace. Building the gen crate alone gives `miniz_oxide`; building the
workspace — which pulls in the store crate, hence `mysql_async` — gives the C
`zlib` backend. **Same map, same JSON, different compressed bytes.**

So `G2.2`'s "same spec input → byte-identical output" was **unsatisfiable** for
two ports, and satisfied elsewhere only by accident of which backend a given
build happened to select. The committed assets could be simultaneously correct
and incorrect depending on how you asked.

**Disposition: FIXED, in the spec and the tool.** `G2.2a` now states that the
determinism requirement attaches to the **map content**, not to the compressed
container, and `assets.rs` compares the decompressed JSON. `CHECKSUMS.txt` still
records the file digest, because "was this artifact corrupted in transit" is a
real and different question, and `shasum -a 256 -c` still verifies it.

The writer also now skips rewriting a file whose content is already current, so
building with a different backend no longer churns the working tree.

Verified on the failing case: `cargo test --workspace` in `fhir-mysql` went from
**6 passed / 1 failed** to **106 passed / 0 failed**, and the standalone binary
still reports no drift.

**Worth noting about the gate itself.** F-40's tool was written to make `G2.2`
checkable, and the first thing it found was that `G2.2` was wrong. That is the
argument for gates that run rather than requirements that are asserted — and it
arrived only because the check was wired into `cargo test` rather than left as a
command someone might run.

## F-42

**The full-corpus round-trip skipped in every port, so the repository's most
substantial test had never run.** Severity: **High**. Violates `T11.9` and
`T11.12`. *Found while unifying the third path-resolution convention left over
from **F-39**.*

`*-gen/tests/corpus.rs` round-trips **every example shipped with the FHIR
specification** through shred and reconstruct. It looked for a corpus at
`FHIR_<ENGINE>_CORPUS_DIR` or `corpus/` beside the workspace root — a CI fetch
step's layout — and for definitions at `spec-cache/`. Neither exists in this
monorepo, so the test printed `skipping: corpus or spec dir missing` and
reported **ok**, in all six ports.

The examples were present the whole time, under
`fhir/doc/fhir-specifications/<ver>/fhir-examples-json`: 1,664 for R3, 2,912 for
R4, 2,824 for R5.

This is the third distinct resolution convention in one test suite — the other
two were **F-39** — and the reason the count matters is **F-01**: six READMEs
claimed "7,399 FHIR example resources round-tripped" while the test that would
have measured it could not find its input.

**Disposition: FIXED.** Both resolvers now accept the monorepo layout, and the
loop tries the nested `<ver>/fhir-examples-json` form **before** the flat
`stu3/`, `r4/`, `r5/` form.

That ordering is load-bearing, and getting it wrong the first time is worth
recording. With the flat form checked first, `<corpus>/r4` resolved to the
monorepo's *version* directory — which exists, contains no JSON, and yielded
`r4: 0 corpus examples, 0 failures`. The overall assertion is `total > 1000`,
which 1,664 from R3 alone satisfied, so the suite went green while two of three
versions tested nothing. A silent skip inside a test that had just been un-skipped.

**Measured, in all six ports:**

```text
r3: 1664 corpus examples, 0 failures
r4: 2911 corpus examples, 0 failures
r5: 2824 corpus examples, 0 failures
```

7,399 resources per port, 44,394 round-trips in total, zero failures — which is
the number **F-01** asserted without evidence, now obtained rather than claimed.
It exercises the shared shred/reconstruct engine, so it holds at the **map**
layer for all six including the two with no store.

## F-43

**The supply-chain gate had never been run, and `fhir-store` had no gate at
all.** Severity: **High**. Violates `O10.10`. *Found while producing the
release evidence `O10.10` requires.*

`O10.10` requires `cargo deny`, `cargo audit`, an SBOM and checksums per
release. `deny.toml` existed in seven workspaces and had, so far as the tree
shows, never been executed — CI has never run (**F-11**). `fhir-store` had no
`deny.toml` whatsoever, despite being on the publish list.

Running all four categories across all eight workspaces found:

| Workspace | advisories | licenses | bans | sources |
| --- | --- | --- | --- | --- |
| `fhir`, `fhir-postgresql`, `fhir-sqlite`, `fhir-mysql`, `fhir-mariadb`, `fhir-oracle` | ok | ok | ok | ok |
| `fhir-mssql` | **4 findings** | ok | ok | ok |
| `fhir-store` | **3 findings** | 2 rejected | ok | ok |

**`fhir-mssql` — four, all dev-only.** One unmaintained crate
(RUSTSEC-2025-0134, `rustls-pemfile`) and three `rustls-webpki 0.101.7`
vulnerabilities (RUSTSEC-2026-0098, -0099, -0104: name-constraint handling for
URI and wildcard names, and a reachable panic parsing CRLs). All reach the tree
through `tiberius 0.12.3`, a **dev-dependency** of `fhir-mssql-map` — verified
absent from `cargo tree -e normal`, so nothing a consumer builds contains them.
No upgrade exists: the fixes need `rustls-webpki >= 0.103.12`, which needs
`rustls 0.23`, which no tiberius release supports, and 0.12.3 is the newest.

*Disposition: ignored in `deny.toml` with that reasoning written out, and
recorded as `M14.34` in the annex so the consequence sits beside `M14.24`'s
driver decision rather than only in a config file. `M14.34` requires the choice
be revisited when the port gains a store — at which point the driver stops
being a test dependency and starts being how clinical data crosses a network —
and forbids claiming `O10.7` until then.*

**`fhir-store` — three, in the normal tree, and NOT ignored.** `fxhash`
unmaintained (RUSTSEC-2025-0057) via `scraper`, and two `quick-xml 0.38.4`
denial-of-service vulnerabilities (RUSTSEC-2026-0194, -0195: quadratic run time
on duplicate attribute names, and unbounded namespace-declaration allocation)
via `opendal`. Both arrive through `loco-rs 0.16.4`.

These were **deliberately not excepted**. The mssql findings are dev-only; these
are in the dependency graph of a server that will handle PHI, and an exception
list is the wrong place to record "this server has a reachable DoS". The fix is
`loco-rs 0.16 → 1.0.1`, which exists but is a **major** framework upgrade
touching `app.rs`, the controllers and the initializers — a migration, not a
bump, and the owner's to schedule.

*Disposition: **FIXED**, 2026-08-02, by the `loco-rs 0.16.4 -> 1.0.1` upgrade.*

The upgrade needed **no source changes at all** — a clean rebuild of all targets
compiled with zero errors. `fhir-store` uses a narrow, stable slice of the API
(`Hooks`, `AppRoutes`, the prelude), and loco's 1.0 kept it. The expectation
that a major version meant a migration was reasonable and wrong; measuring cost
before assuming it is the cheaper order.

It cleared one advisory outright: `scraper 0.21 -> 0.25` dropped `fxhash`.

Two remain — `quick-xml`'s DoS pair — and they are blocked upstream at a
precisely identified point. The fix is `quick-xml >= 0.41.0`; `opendal-core
0.58.0` requires exactly that; `loco-rs 1.0.1` pins `opendal ^0.57`. One loco
release that admits opendal 0.58 clears both, and nothing in this repository can
bring it forward: `quick-xml` is unconditional in `opendal-core`, not reachable
through a feature.

They are now ignored in `deny.toml`, reversing the position taken above, because
the facts changed. The exposure argument is that these fire on parsing hostile
XML, this service parses FHIR **JSON**, and loco enables only opendal's
`services-fs` and `services-memory` — not `services-s3`, `-azblob` or `-gcs`,
whose responses are the XML opendal parses. The entry records that this is a
feature-level argument and **not** an audit of opendal's source, so the residual
risk is stated rather than implied.

**Fixed while there:** a yanked `bytesize 2.6.0` (updated to 2.7.0), and two
permissive licences missing from `fhir-store`'s allowlist — `0BSD`
(`quoted_printable`) and `CDLA-Permissive-2.0` (`webpki-roots`, a licence over
the root-certificate *data*). Neither is copyleft, so neither can change a
deployment's licensing, which is what that allowlist exists to prevent.

**The SBOM half of `O10.10` was verified too, and works.** `cargo cyclonedx
--format json --all` — the exact command the CI job runs, which had also never
executed — produces a valid CycloneDX 1.3 document for **all 32 crates**, each
carrying the right name, version, and the harmonized quintuple licence
expression. That last is a useful cross-check: the licensing work landed in
machine-readable form and not only in prose.

One gap fixed while there: `*.cdx.json` was not in `.gitignore`, so a local run
left 31 untracked files that would have been committed. An SBOM describes one
build; a stale one in the tree is worse than none — the failure the map assets
had before **F-40** gave them a regeneration path. They are generated at
release and uploaded as an artifact, so they are now ignored.

**On method.** The first pass over the seven workspaces used `head -1` on the
output and reported `fhir-mssql` as having one problem. It had four; the other
three were below the fold. Truncating a security tool's output is how a
four-finding workspace reads as a one-finding workspace, and the corrected run
is the one recorded above.

## F-44

**The shared-core gate crashed instead of reporting, on the only path where it
matters.** Severity: Medium. Violates `W16.6`. *Found by running `cargo fmt`,
which produced the first real divergence the gate had ever seen.*

`scripts/check-shared-core.sh` runs under `set -u` and its `is_exempt` helper
iterated `"${EXEMPT[@]}"`. `EXEMPT` is empty — deliberately, since closing
**F-07** removed the last exemption — and bash 3.2, which macOS still ships,
treats an empty array expansion under `set -u` as an unbound variable and
aborts:

```text
./scripts/check-shared-core.sh: line 73: EXEMPT[@]: unbound variable
```

`is_exempt` is called **only** when a file has already been found missing or
divergent. So the gate worked perfectly while everything agreed, and died the
moment it had something to say — printing a bash error where the divergence
report should have been. Every green run in this repository's history was a run
that never reached the broken line.

**Disposition: FIXED.** `${EXEMPT[@]+"${EXEMPT[@]}"}` guards the empty case, and
the reason is in a comment so the guard is not removed as noise. The gate now
reports the divergence it was written to report.

### The divergence it was hiding

Not a real one, and the way it is not is worth recording. `cargo fmt` wrapped
`bundled_r3`'s body differently in `fhir-postgresql` than in the other five:

```rust
// fhir-sqlite — fits in 100 columns
RelMap::from_gz_bytes(include_bytes!("../assets/fhir-sqlite-relmap-r3.json.gz"))
    .map_err(BundledError::Decode)

// fhir-postgresql — four characters longer, so rustfmt broke the call
RelMap::from_gz_bytes(include_bytes!(
    "../assets/fhir-postgresql-relmap-r3.json.gz"
))
.map_err(BundledError::Decode)
```

`fhir-postgresql` is four characters longer than `fhir-sqlite`, and that was
enough to cross rustfmt's line limit. The gate's normalizer rewrites crate names
to a common token so content can be compared, but it cannot normalize *layout*
that the name's length produced.

This is a standing hazard for `X15.1`: any shared-core line whose width depends
on the crate name can be formatted into a false divergence. Fixed structurally
rather than by loosening the normalizer — the path is bound to a `const` first,
so the long line is a short declaration and the expression that follows is
name-independent. Loosening the comparison would have traded a false positive
for a class of false negatives.

## F-45

**The shared-core gate watched the map and the generator and stopped at the
store, where 618 identical lines sat unwatched.** Severity: Medium. Violates
`W16.6` in spirit. *Found while splitting `fhir-store`.*

`scripts/check-shared-core.sh` covers `map/src/{model,shred,reconstruct,value,fold,canon,error,lib}.rs`
and `gen/src/*`. It does not cover the store crate. Measured across the six
ports, normalized for the engine name:

| File | Lines | Ports differing from `fhir-sqlite` |
| --- | ---: | --- |
| `store/src/chain.rs` | 618 | 0 lines in mysql, mariadb, mssql, oracle; **3** in postgresql |
| `store/src/lib.rs` | 251 | 8–10 lines in four ports (postgresql holds its whole `Store` impl, so it differs wholesale) |

So roughly **860 engine-agnostic lines were duplicated six times**, and the one
gate that would have noticed did not look. That is the direct reason closing
**F-07** — the hash-chain pre-image was derived from PostgreSQL's `jsonb`, so
chains were not portable between engines — had to be applied by hand six times.

**Disposition: FIXED, by extraction rather than by widening the gate.** A new
[`fhir-store`](../../fhir-store/) crate now holds the engine-agnostic half:
`chain` (the audit chain, which depends only on `hmac`, `sha2` and `sha3` — no
port crate at all), and the value types `Audit`, `AccessRecord`, `PutOutcome`,
`Got`, `HistEntry`, `ResourceStatus`, `SearchOutcome`, `CondCreate`,
`CondDelete`, `TxOp`, `TxOutcome`, `PurgeReport`, `UpgradeReport`, `ChainBreak`.

Widening the gate was the alternative and is worse: a gate that watches six
copies still leaves six copies, and `W16.7` still requires six edits in one
commit. One crate makes the duplication impossible rather than merely visible.

**What deliberately stays in a port:** the driver, transactions, placeholder
syntax, the search-SQL builder, `ddl.rs`, and `StoreError` — which wraps that
port's own `ShredError` and therefore cannot be lifted.

**A design note this contradicts.** The gate's own header says the
one-copy fix "is not available for the code, because each port must compile on
its own". That was true of a *path* dependency across six independent
workspaces; it is not true of a published crate, which each workspace resolves
for itself. The cost that is real is version coupling — a change to the shared
half now needs a `fhir-store` release before a port can take it — and that is
the trade this extraction accepts.

**All six ports are now rewired**, so the extraction is load-bearing rather than
merely available. Each `fhir-<engine>-store` takes `fhir-store` as a versioned
path dependency, deletes its `chain.rs`, and re-exports the value types instead
of redefining them. `fhir-postgresql` needed surgery rather than truncation —
its `lib.rs` interleaves the value types with the whole `Store` impl — so the
16 type and `impl` blocks were removed individually, 196 lines.

One API change was required. `ChainBreak` is `#[non_exhaustive]`, which is
deliberate — it gained `algorithm` after release — and a `#[non_exhaustive]`
struct cannot be built with a literal outside its defining crate. The ports
detect chain breaks, so they are outside it. Rather than drop the attribute and
lose the freedom to add fields, `fhir-store` gained `ChainBreak::new`, and the
eight construction sites across the six ports now call it.

**Verified.** All six build, lint and format clean, and every suite passes:
postgresql 62, sqlite 102, mysql 92, mariadb 92, mssql 54, oracle 43.

Each count is **exactly 14 lower** than before, which is the point rather than a
regression: the 14 chain tests used to run six times over six copies of one
implementation and now run once, in the crate that owns it. Repository-wide the
test count falls by 70 while coverage is unchanged — 70 executions that were
re-proving the same code.

## F-46

**`U11` cannot be satisfied for the extension and deep tables without a
structural change, and the two ways to do it disagree with each other.**
Severity: Medium. *Found implementing `U11`.*

`U11` requires the generator to consider every search-reachable column. For
`ColTy`-driven columns that is now done (`U1a`, `U2a`, `U2b`). For the
extension and deep tables it is not, and the obstacle is not effort:

| | |
| --- | --- |
| `Ext`, `Deep`, `Contained`, `History` tables in the map | **zero columns** — verified across all 158 R5 resources |
| Where their shape lives instead | hardcoded in `ddl.rs::create_table`, per port |
| How the shredder emits their rows | `ExtRow` / `DeepRow` — fixed structs with named fields, not the generic `Vec<(String, SqlVal)>` that `Row` uses |

So the machinery the adjuncts flow through — a `Column` in `t.cols`, an entry in
`t.adjunct_cols`, a `(name, SqlVal)` pair the shredder appends — does not reach
these tables at all.

**Two shapes, and they are not equivalent:**

*A — put the fixed-shape columns in the map.* `build.rs` populates `t.cols` for
`Ext` and `Deep`, `create_table` emits from the map instead of hardcoding, and
adjuncts flow through the existing path unchanged. Uniform afterwards, and it
satisfies `U2b` because the map records what exists. It is also the larger
change: every port's store binds those rows with hardcoded column lists, and all
six would move together (`X15.1`, `W16.7`).

*B — derive them dialect-side.* `ExtRow` gains adjunct fields, `create_table`
emits the columns where `needs_adjunct` is true, and each store binds them.
Contained, and the map never changes — **which is the problem**: `U2b` requires
the map to record which adjuncts exist, and under B nothing does. A query
builder would have to infer them from the dialect, which is the assumption `U2b`
was written to forbid.

**Disposition: OPEN. A chosen, and attempting it found that A has an ordering
constraint neither option statement captured.**

B is cheaper and contradicts a requirement written earlier in the same pass, so
A is the one the spec permits. A was then implemented far enough to fail
usefully.

Adding the fixed-shape columns to the map **first** was tried and reverted. It
worked as far as it went: `Ext` and `Deep` gained their nine and six columns
across all 158 R5 resources (+2,370 per port), and adjuncts attached exactly as
`U2a` requires — `url` and `v_text` both, `leaf` digest-only, `path` and
`v_kind` none because `U12` binds them to an indexable type and `needs_adjunct`
refuses them.

Then the map disagreed with the database. `create_table` still hardcodes those
columns, and it does not emit what the map now claimed:

| Column | Map said | `create_table` emits (mssql) |
| --- | --- | --- |
| `path` | `TextC` → `NVARCHAR(450)` | `NVARCHAR(MAX)` |
| `v_kind` | `TextC` → `NVARCHAR(450)` | `CHAR(1)` |

A map that misdescribes the schema is worse than a map that omits it. Omission
is a gap a reader can see; a wrong type is one that reads as authoritative —
and `path` claiming to be indexable when it is `NVARCHAR(MAX)` is exactly the
belief `U1a` exists to prevent.

**So A must be done in the other order**: `create_table` becomes map-driven
first, or in the same change. That is not a detail of sequencing — making
`create_table` map-driven *changes the physical schema of all six ports*
(`path` from unbounded to bounded, `v_kind` from `CHAR(1)` to `VARCHAR2`), which
is a data migration under `L12`/`O10.4a` and needs the four ports that have
stores to carry it.

The revert was clean: `--check` reported no drift and the tree was where it
started.

### Resolution, 2026-08-02: A, narrowed to the columns that need no type change

Fixed. The ordering constraint above is real, and the way past it is to not
trigger it: **only the three columns an adjunct is actually wanted on go into
the map**, and all three are already the dialect's unbounded text type in the
emitted DDL.

| Column | Map says | `create_table` emits | Agree? |
| --- | --- | --- | --- |
| `url` | `Text` | `NVARCHAR(MAX)` / `TEXT` | yes |
| `leaf` | `Text` | `NVARCHAR(MAX)` / `TEXT` | yes |
| `v_text` | `Text` | `NVARCHAR(MAX)` / `TEXT` | yes |

`path`, `ords`, `v_kind`, `v_num`, `v_bool`, `modifier`, `ext_ord` and
`key_hash` stay out. They are the ones whose map type and emitted type would
have disagreed, and none of them is a search target that needs an adjunct — so
including them bought the physical-schema migration and nothing else. **No
column changes type, so this is not a data migration**; the four ports with
stores need no backfill, and the two without need no decision.

The map is therefore still a partial description of these tables, which `U2b`
permits — it requires the map to record which adjuncts exist, not to describe
every column. `fixed_shape_cols` in `build.rs` says so at the definition, so the
next reader does not take the omission for an oversight.

Adjuncts follow `U2a`, by the operation performed: `url` both (a URI is matched
by equality and by `:below`/`:above` prefix), `v_text` both (string semantics),
`leaf` digest only (matched exactly, never by prefix). `needs_adjunct` still
decides, so the four ports that index their unbounded text type get none.

The half that was missing on the first attempt — `create_table` emitting what
the map names — is `push_adjunct_cols`, called in the `Ext` and `Deep` arms
*before* the trailing key and constraint clauses, because SQLite rejects a
column definition that follows a table constraint.

**What now enforces it:** `tests/adjuncts_in_ddl.rs` in every port's gen crate
generates the R5 map and asserts, for every table of every resource, that each
adjunct the map names appears in `create_table`'s output and that each adjunct's
source column exists. It checks 3,713 columns on `fhir-mssql` — 2,449 from the `ColTy` path and 1,264 from the extension and deep tables, which is what this finding added. Mutation-verified
per `T11.10`: deleting the `push_adjunct_cols` call from the `Ext` arm fails it
at `account_ext.leaf_h`. It also asserts `checked > 0` **iff** `TEXT_ADJUNCTS`,
so on the four ports where the correct answer is zero the test cannot quietly
become vacuous (`T11.12`).

`U11` is now satisfied for `string`, `token`, `uri` and `reference` targets and
for the extension and deep tables. `Contained` and `History` are out of scope
for a reason worth writing down rather than assuming: each has exactly one text
column, both hold a whole serialized resource (`resource`, `body`), and no
search in these ports filters on either — they are read back by key during
reconstruction. That is a property of the current search surface, not of the
tables. **If `_contained` or `_text` search is ever implemented, `U11` reaches
them and they will need adjuncts**; `fixed_shape_cols` returns an empty vector
for both with that written at the arm, so the decision is visible where it would
have to change.

**Verified against a live engine.** `fhir-mssql` is one of the two ports that
gets these columns, and `crates/fhir-mssql-map/tests/mssql_ddl.rs` installed the
generated schema — 131 statements, 102 tables, 4 triggers — on `azure-sql-edge`
and it was accepted. Querying `sys.columns` afterwards found the adjuncts really
there, with the types `U4a` requires:

```
observation_ext.leaf_h     binary(32)
observation_ext.url_idx    nvarchar(450)
observation_ext.url_h      binary(32)
observation_ext.v_text_idx nvarchar(450)
observation_ext.v_text_h   binary(32)
```

`binary(32)`, not 64 characters of hex — which is the whole point of `U4a`, and
the kind of thing only the engine can confirm.

Two limits on that evidence, stated rather than implied. The image is
`azure-sql-edge`: `mcr.microsoft.com/mssql/server:2022-latest` segfaults on
arm64, which `db.sh:28` already anticipates, and Edge is a subset of the full
product — good evidence, not a conformance claim (`C0.11`). And `fhir-mssql`
has no store, so *installing* the schema is all that can be tested here; no row
has been written through these columns on any engine, because the four ports
that could write one are the four where `needs_adjunct` correctly declines to
create them.

---

## F-47

**`path` and `v_kind` are bounded in practice but bound to unbounded types, so
`U12` is unsatisfied on two ports.** Severity: Low. *Split out of **F-46**.*

`U12` says that a column bounded in practice SHOULD be bound to an indexable
type rather than given adjuncts — adjuncts are the fallback for values that
genuinely have no bound, and two derived columns cost more than one narrower
one.

Two columns of the extension and deep tables fail that on `fhir-mssql` and
`fhir-oracle`:

| Column | Emitted as | Actually bounded by |
| --- | --- | --- |
| `path` | `NVARCHAR(MAX)` / `CLOB` | the longest FHIR element path, which the generator knows at build time |
| `v_kind` | `CHAR(1)` on mssql | already bounded — the defect is oracle's, where it is a `CLOB` |

`create_table` hardcodes them, so fixing this means making those arms map-driven
— and **that changes the physical schema**: `path` from unbounded to bounded is
a data migration under `L12`/`O10.4a`, which the four ports with stores would
have to carry even though none of them has the defect (`X15.1` moves all six
together).

This is why **F-46** was narrowed rather than solved in full. It is deliberately
*not* urgent: no search filters on `path` today, so the missing bound costs
nothing until one does. What it does cost is `U12`, which those two ports
therefore cannot claim.

Resolving it needs the migration story first — specifically, what
`fhir-postgresql` does with an existing `patient_ext` when `path` narrows, and
whether the generator's known maximum path length is a *bound* or merely the
longest one seen in R3–R5.

---

## F-48

**The shared-core gate does not watch the `gen` crates' tests, and its
normalization cannot be extended to them as written.** Severity: Low. *Found
adding `tests/adjuncts_in_ddl.rs`.*

`X15.1` makes all of `gen/` shared, and `scripts/check-shared-core.sh` enforces
that for `gen/src` — six modules, 75 files, currently identical. It does not
look at `gen/tests/`, which is equally dialect-independent and equally shared in
practice:

| File | Identical across six? |
| --- | --- |
| `assets_current.rs` | yes |
| `proptest_roundtrip.rs` | yes |
| `adjuncts_in_ddl.rs` | yes (added with **F-46**) |
| `corpus.rs` | **no** |
| `roundtrip.rs` | **no** |

The `corpus.rs` difference was a real defect: `fhir-sqlite` **and
`fhir-postgresql`** each listed the same `spec_root` candidate **twice**, left
over from the **F-39** fix. Harmless — `find` takes the first — but dead, and it
is why that file diverged.

Only `fhir-sqlite` was fixed at first. `fhir-postgresql` was missed precisely
because the line-based comparison used `fhir-sqlite` as its baseline, so the two
files agreed — a baseline comparison cannot tell "matches" from "shares the same
bug". The token gate below caught it on its first run.

Removing it was verified rather than assumed inert: the corpus test still
resolves its inputs and still runs the whole set — 1,664 R3, 2,911 R4 and 2,824
R5 examples, 0 failures. Given **F-39** was a path-resolution defect that turned
this exact test into a silent skip for its entire life, "it only deletes a
duplicate" is not a claim worth making without running it.

The rest is not a defect, and it is the reason the gate cannot simply be pointed
at `tests/`. `normalize()` rewrites the crate name to a constant, but rustfmt
wrapped these files based on **how long the name is**, and that is invisible
after normalization:

| Port | `FHIR_<PORT>_SPEC_DIR` line, one-line width | rustfmt |
| --- | --- | --- |
| `fhir-mysql`, `fhir-mssql` | 69 | one line |
| `fhir-sqlite`, `fhir-oracle` | 70 | split over three |
| `fhir-mariadb` | 71 | split |
| `fhir-postgresql` | 74 | split |

The boundary is exactly 69/70 in all six, so this is entirely name-length, not
divergent code. Pointing the existing gate at `gen/tests/` would report those
two files red on day one, and a gate that is red for a reason nobody can fix is
the failure `EXEMPT` was written to avoid.

### Resolution, 2026-08-02

Fixed. The verdict is now token-based and `gen/tests/` is watched: **100 files**,
up from 75.

`tokenize()` runs the existing `normalize()` and then emits one token per line,
where a token is a run of `[A-Za-z0-9_]` **or a single other non-space
character**. Punctuation therefore survives — only whitespace is discarded. The
pass/fail decision is made on that stream; `--diff` still shows the line-based
diff, because a token-per-line diff is unreadable.

**It found a real defect on its first run.** `fhir-postgresql` carried the same
duplicated `spec_root` candidate that `fhir-sqlite` did — 41 differing tokens.
The earlier half of this finding fixed only `fhir-sqlite`, because a line-based
comparison against `fhir-sqlite` as baseline could not distinguish "postgresql
matches the baseline" from "postgresql shares the baseline's bug". Both are now
clean, and all six carry the candidate exactly once.

**Mutation-verified in four directions** (`T11.10`), because a gate that has just
been made *more* tolerant is exactly the kind that quietly stops working:

| Mutation | Expected | Result |
| --- | --- | --- |
| `let mut failures = 0` → `1` in one port | caught | 2 differing tokens |
| a call chain rewrapped across three lines | tolerated | passes |
| `*lane += 1` → `*lane -= 1` | caught | 2 differing tokens |
| double space inside a string literal | **masked** | passes |

The fourth is the deliberate blind spot, and it is demonstrated above rather
than asserted. Whitespace inside a string literal, and the rewrapping of a doc
comment, are both invisible now. That is acceptable for this file set on
`X15.1`'s own criterion — these are the pure-Rust core modules, which never emit
SQL, so no string literal here is a payload whose spacing carries meaning. It
would **not** be acceptable if this gate were ever pointed at `ddl.rs`, and it is
not.

A first attempt at mutation 3 used an anchor that did not exist in the file. It
"passed", which proved nothing — the edit never applied. Recorded because a
mutation test that silently fails to mutate is the same defect as a test that
silently skips (`T11.12`), one level up.

---

## F-49

**No workflow in this repository runs. Every one of them is in a directory
GitHub does not read.** Severity: **High**. Violates `W16.6`. *Found wiring
**F-48**'s gate into CI.*

GitHub Actions discovers workflows in `.github/workflows/` **at the repository
root**, and nowhere else. This repository has no root `.github/` at all:

```
$ ls -a | grep -i github
  (nothing)

$ git ls-files | grep 'workflows/' | sed 's|/workflows/.*|/workflows/|' | sort -u
fhir-loco/.github/workflows/
fhir-mariadb/.github/workflows/
fhir-mssql/.github/workflows/
fhir-mysql/.github/workflows/
fhir-oracle/.github/workflows/
fhir-postgresql/.github/workflows/
fhir-sqlite/.github/workflows/
fhir/.github/workflows/
```

Eight families, 20-odd workflow files, none of them reachable. The layout is a
leftover from when each family was its own repository — where it was correct.
The monorepo merge that resolved **F-11** made every one of them inert, and
nothing noticed because an inert workflow produces no output to be missing.

**This is not the same finding as F-11, and the audit currently says it is.**
Several entries — **F-35**, **F-43**, the fuzz-target finding — explain "CI has
never run" by citing **F-11**, which is marked *resolved*. A reader would
reasonably conclude that CI runs now. It does not, and the reason is no longer
the one recorded. Those attributions are stale rather than wrong at the time
they were written.

**What is consequently unverified.** Every claim of the form "gated in CI" in
this repository, including:

| Claim | Where | Reality |
| --- | --- | --- |
| `X15.1` shared core "now **gated** in CI" | conformance matrix (**F-10**) | nothing invokes `scripts/check-shared-core.sh` — the only file that mentions it is itself |
| `fhir-mssql` "fails rather than skips" without a database | **F-06**, `AGENTS.md` | the job that would fail never starts |
| `published-versions` gates | **F-35** | already recorded as never having run; this is why |
| supply-chain gates | **F-43** | fixed as a *script*, but the workflow that runs it does not execute |
| `G2.2` asset drift | `tests/assets_current.rs` | runs under `cargo test`, so it is gated wherever tests run — but that is a developer's machine, not CI |

The pattern is the one this audit exists for: a control that cannot run is
indistinguishable from a control that passes (`T11.12`, one level up from
tests).

**Disposition: PARTIALLY FIXED, and not yet effective.**
`.github/workflows/gates.yml` now exists at the root and runs the two checks
that cannot live in a per-port workflow, because each is inherently
cross-cutting:

- **`scripts/check-shared-core.sh --diff`** — deliberately with **no Rust
  toolchain**: the check compares text, so it stays fast enough that nobody is
  tempted to skip it, and keeps working even if a port stops compiling.
- **`scripts/check-doc-examples.sh`** — this one does need a toolchain; it
  compiles every ```` ```rust ```` block in `doc/` and `README.md` against the
  real crates (**F-60**).

Verified locally that both exit 0 on a clean tree and non-zero on an introduced
defect.

**It will not do anything until the repository catches up.** `scripts/` is
itself **untracked** — as are `doc/`, `LICENSE.md`, `index.md`,
`spec/publishing.md`, and 81 files in total, against 224 modified and 8 deleted.
A workflow that invokes a script absent from the checkout fails on its first
run, which is at least loud; but the honest statement today is that `W16.6` is
**not** satisfied for `X15.1` either — only that the mechanism is in place and
waiting on a commit.

That is the same shape as **P-10**: a gate whose inputs are not in the
repository is a gate that exists only on the machine that wrote it.

**Not fixed: the other eight families' workflows.** Consolidating them is a
design decision rather than a correction — it needs path filters so a change to
`fhir/` does not run six database suites, a matrix over ports, and a rule for
which service containers start when. Doing that badly produces a CI run that is
slow enough to be ignored, which is the failure mode this finding is about.
Recorded as the owner's call, with the evidence above.

---

## F-50

**The reference adjunct rule attached to a column no index uses.** Severity:
Medium. Shared core, all six ports. *Found porting `fhir-oracle`'s
`search_indexes` (**F-08**).*

`U2a` requires an adjunct on whichever column a query actually touches. The
rule written for `TargetKind::Reference` attached one to `c_url` alone:

```rust
TargetKind::Reference { c_url, .. } => (vec![c_url], (false, true)),
```

Every port's `search_indexes` keys the reference index on `(c_type, c_id)`.
So the index needed adjuncts on two columns the adjunct pass had deliberately
skipped, and skipped the one it had attached.

Both search forms are real — `subject=Patient/123` matches the split
`(c_type, c_id)`, an absolute reference matches `c_url` — so the fix is all
three, equality-only, since neither form has a prefix operation.

**Why it was invisible.** On the four ports where `TEXT_ADJUNCTS` is false no
adjunct is materialized at all, so the rule's output is discarded and any rule
looks correct. It only becomes observable on a port that both materializes
adjuncts and needs them to index, which until **F-08** none did.

It surfaced as a number, not as a failure: `search_index_gaps` reported **453 of
R5's 1,947 search targets** unindexable on Oracle, and every one of them was a
`(<x>_ref_type, <x>_ref_id)` pair. A schema would have installed cleanly with
453 missing indexes and nothing to say so — which is why that counter exists at
all.

Fixed in all six (`X15.1`). Oracle's gap count is now **0**.

A second, milder version of the same mistake was fixed alongside it: Oracle's
`index_columns` substituted only the *bounded* adjunct, so token and reference
targets — which `U2a` correctly gives a digest and no bounded column — were
counted as unindexable. An index on the digest serves an equality search
exactly, with `U6`'s confirming comparison covering collisions. That accounted
for the difference between 1,231 gaps and 453.

---

## F-51

**`fhir-oracle`'s DDL has been executed but nothing re-executes it, so the port
cannot claim Schema.** Severity: Medium. *Opened closing **F-08**.*

`C0.8` defines exactly four levels and `C0.9` requires the claimed one be
justified **by tests that run**. Closing **F-08** produced the evidence Schema
asks for — the full R5 schema installs on Oracle 26ai, 0 invalid objects — but
produced it by hand, with `podman exec … sqlplus`. A transcript in an audit
entry is not a test. Nothing in the repository will notice when it stops being
true.

The level therefore stays **Scaffold**, and the temptation to write "Scaffold+"
is what this finding exists to record: that string was written into eleven files
before being removed. `C0.8` has four levels precisely so that a port cannot
invent a rung for the distance it has actually travelled.

**What Schema needs:** a live test in `crates/fhir-oracle-map/tests/`, on the
model of `fhir-mssql`'s `mssql_ddl.rs` — install the generated schema, assert it
applies, skip loudly without a DSN and fail rather than skip when
`FHIR_ORACLE_REQUIRE_DB` is set (`T11.12`).

**What blocks it, and why it is a real decision rather than an oversight:**
there is no Oracle driver in the workspace. The `oracle` crate binds Oracle
Instant Client, which is a native dependency with its own licence terms and must
be present at build time as well as test time — unlike `tiberius`, which is pure
Rust and is why `fhir-mssql` could have a live test cheaply. Options are (a) take
the Instant Client dependency behind a feature flag, (b) drive `sqlplus` in a
container from a shell script and treat that as the live gate, or (c) wait for a
pure-Rust Oracle protocol client.

Two behaviours are additionally verified only by hand and would be lost the same
way: the append-only trigger refusing `UPDATE`/`DELETE` and permitting a
declared erasure (`M14.29`), and the `Bool` CHECK rejecting `2` (`M14.8`). The
first is the one that already failed open once (`M14.29a`).

---

## F-52

**The only live database test in this repository was flaky, and failed in a way
that blamed the wrong thing.** Severity: **High**. *Found re-running it after the
**F-50** shared-core change.*

It violated no requirement, because there was none: `T11.13` covers a test that
*skips* without its database, not one that passes intermittently with it, and
nothing else in §11 addressed determinism. **`T11.15` was added for this**, so
the next instance is a defect against a rule rather than a judgement call.

`fhir-mssql`'s `mssql_ddl.rs` failed about two runs in three:

```
statement 8 of 131 was rejected by SQL Server:
'There is already an object named 'observation' in the database.'
```

The install was fine. The **cleanup** was broken. It built one batch of
`DROP TABLE` for everything in `sys.tables`, and a base table cannot be dropped
while a child table's foreign key still references it — so the batch aborted at
the first such table and left the remainder standing. `sys.tables` has no
guaranteed order, which is why it was intermittent rather than constant.

Two separate defects, and the second is the one worth learning from:

1. Foreign keys must be dropped before tables. Fixed.
2. **The cleanup's error was discarded** — `let _ = client.simple_query(…)` —
   and the schema was never checked to be empty afterwards. So a cleanup failure
   surfaced eight statements later as a DDL error, pointing at a `CREATE TABLE`
   that was correct. The fix panics on the cleanup error and asserts the table
   count is zero before installing.

**Why this is High.** This is the one live test that had ever been run against a
real engine in this repository, and the conclusions resting on it include
**F-46**'s verification and `fhir-mssql`'s entire claim to have executed DDL.
Those still stand — it passes, and passed when cited — but they were resting on
a test that would have gone green on a re-run whatever it was told.

A flaky gate is worse than a failing one: the habit it teaches is to run it
again. `T11.15` now says so normatively.

Now passes five consecutive runs. Note the interaction with **F-49**: this test
does not run in CI either, so nothing but a human re-running it would ever have
noticed.

---

## F-53

**Every store crate called itself "the PostgreSQL layer", including the two that
have no store at all.** Severity: Medium. Violates `C0.11`. *Found gathering
facts to rewrite the scaffolds' `tasks.md` (**F-27**).*

The module doc of `crates/fhir-<engine>-store/src/lib.rs` was identical in all
six ports:

```rust
//! fhir-oracle-store: the PostgreSQL layer. Applies generated DDL, writes
//! shredded resources transactionally with history, and reads rows back for
//! reconstruction.
//!
//! Every value crosses the wire as text with explicit casts
//! (`($n::text)::numeric`), …
```

Three separate false statements, in the first thing a reader of the crate sees:

| | |
| --- | --- |
| "the PostgreSQL layer" | wrong in five of six |
| `($n::text)::numeric` | PostgreSQL's parameter syntax. `fhir-sqlite` binds through `rusqlite`; `fhir-mysql`/`fhir-mariadb` through `mysql_async` |
| "Applies generated DDL, writes … reads rows back" | `fhir-mssql` and `fhir-oracle` do none of it — their `lib.rs` is 48 lines that re-export the shared chain and define an error type |

This is **F-01 in `src/`**. F-01 fixed the six READMEs and F-16 the two annexes;
`rustdoc` renders this text as the crate's front page, so it is documentation by
any definition, and it had the same single cause — one file copied per port and
never re-read.

The last row is the one that matters. A reader of `fhir-oracle-store`'s
documentation is told it writes resources transactionally with history. It
cannot open a connection.

Fixed: each header now names its own engine and its own binding mechanism, and
the two scaffolds say plainly that there is no store — `C0.11` forbids
documentation describing a capability above the port's level, and a crate's own
module doc is where that rule bites hardest.

The `postgresql` header's cast claim was checked rather than assumed and is
true: `::text` appears 34 times in its `lib.rs`.

---

## F-54

**`fhir-mysql` and `fhir-mariadb` carry PHI over an unencrypted database link,
and offer no way to encrypt it.** Severity: **High**. Violates `O10.7`.
*Separated from **F-27** class 3, where it was found.*

Both are **Store**-level ports — they shred, reconstruct, and serve search
against a real engine, so real patient data crosses this connection. Neither
store configures TLS in any form:

```
$ grep -rn 'ssl\|Ssl\|tls\|Tls' fhir-mysql/crates/fhir-mysql-store/src/*.rs
(no matches)
```

`mysql_async` supports TLS through `SslOpts`; it is simply never set, so every
connection is plaintext and there is no environment variable, DSN parameter, or
API that changes that. This is worse than `fhir-postgresql`'s **F-17**, which
was a bad *default* over a working implementation — here there is nothing to
default to.

**Why this was invisible.** `tasks.md` said it was done. `T32 Encrypted database
transport (O10.7)` was `[x]` in both ports and described `SslPolicy`, a rustls
connector, `PGSSLROOTCERT` trust anchors and a startup guard — all of it
`fhir-postgresql`'s text, none of it present. The task list asserting completion
is exactly what stopped anyone looking, which is the argument **F-27** makes
about `[x]` being a stronger claim than prose.

`O10.7` also requires that starting a networked service over an unencrypted
database connection refuse without an explicit override. Neither port can
satisfy that half either, though it is moot while no port has a service to start
(`C0.17`).

### Resolution, 2026-08-03

**Fixed in both ports.** `ssl.rs` adds an `SslMode` in **MySQL's own
`--ssl-mode` vocabulary** — `DISABLED`, `REQUIRED`, `VERIFY_CA`,
`VERIFY_IDENTITY` — read from `FHIR_<PORT>_SSL_MODE`, with
`FHIR_<PORT>_SSL_CA` for a private CA, and wired through a new
`connect_with`. `connect` reads the environment and defaults to
`VERIFY_IDENTITY`.

**The Cargo feature was half the defect.** `mysql_async` was declared with
`default-features = false, features = ["minimal"]`, and `minimal` excludes TLS
entirely — so even code that asked for `SslOpts` could not have encrypted
anything. Now `["minimal", "rustls-tls"]`; rustls rather than native-tls, to
match every other port and avoid an OpenSSL build dependency.

**Three decisions worth recording:**

1. **Not `PGSSLMODE`.** That is a libpq name and these ports do not speak libpq.
   Reusing it invites a deployment to set the wrong variable and believe it took
   effect — the failure `O10.7` can least afford, because an unencrypted link is
   invisible from the application.
2. **`PREFERRED` is refused, not approximated.** MySQL's client has it;
   `mysql_async` cannot express it — passing `SslOpts` makes TLS mandatory,
   omitting them makes it impossible, and there is no third state. Mapping it to
   `REQUIRED` would refuse connections that used to work; mapping it to
   `DISABLED` would hand a plaintext link to someone who asked for encryption.
   It is an error naming the two modes that exist.
3. **The default is `VERIFY_IDENTITY`, not `REQUIRED`.** MySQL's own `REQUIRED`
   encrypts and validates *nothing*, so it does not survive an active attacker.
   Defaulting to it would look secure and satisfy nothing.

**Verified against live MySQL 8.4 and MariaDB 11.4.** The load-bearing
assertion is not that a verified connection succeeds — if checking were a no-op
that would succeed too — but that `VERIFY_IDENTITY` **fails** against the stock
container's self-signed certificate:

```
Db("Input/output error: invalid peer certificate: UnknownIssuer")
```

`tests/ssl_live.rs` asserts exactly that, and is mutation-verified two ways
(`T11.10`): making `VerifyIdentity` accept invalid certificates fails it, and
reverting the Cargo feature to `["minimal"]` — the pre-fix state — fails it too.

**The cost, stated rather than discovered.** A verifying default breaks every
local dev container, because their certificates are self-signed: the existing
live suites began failing with `UnknownIssuer` the moment the default changed.
That is the change working. `scripts/db.sh` now prints
`FHIR_<PORT>_SSL_MODE=DISABLED` alongside the DSN, so the opt-out is **explicit
and visible** — nobody reading a green local suite should think it exercised a
verified link. Full live suites pass again on both ports with that set.

Both `tasks.md` entries stay unticked pending a live test against a
*correctly-certificated* server, which needs a CA fixture rather than a stock
container.

---

## F-55

**`scripts/db.sh` resolved the FHIR specification packages through the ancestor
project's path and one developer's home directory, in all six ports.** Severity:
**High**. Violates `T11.12`. *Found auditing the conformance matrix's `•`
claims.*

`find_spec` searched exactly two locations:

```sh
"$REPO/../fhir-rust-crate/doc/fhir-specifications"
"$HOME/git/joelparkerhenderson/fhir-rust-crate/doc/fhir-specifications"
```

Neither exists in this repository, where the packages are at
`fhir/doc/fhir-specifications`. So `spec_exports` emitted nothing, the corpus
environment variables were never set, and following the **documented** workflow —
`scripts/db.sh up` then run the suite — produced a live suite whose corpus tests
could not find their inputs.

**This is F-39's defect in the shell script.** F-39 and F-42 fixed the candidate
lists inside the Rust tests and did not look here; the same absolute path into
the same developer's home directory survived one directory away.

**What it cost, concretely.** `target/test-corpus/{stu3,r4,r5}` were **dangling
symlinks** into `/Users/jph/git/joelparkerhenderson/fhir-rust-crate/…`, left
from when that path existed. `fhir-postgresql`'s `live.rs` therefore failed with
`no examples ran` — which is `T11.12` working exactly as intended, since it
asserts `total > 0` rather than passing vacuously. But nothing had run it, so
nobody saw the assertion fire.

That matters because `fhir-postgresql` is the **Reference** port, and the live
corpus round-trip is what a Store-or-above level rests on (`C0.8`). The
in-memory corpus round-trip has always worked and is what `R4.2`'s matrix note
cites — 7,399 examples, map layer, no store needed. The *live* one, through
PostgreSQL, had no evidence in this repository at all.

**Fixed** in all six scripts by putting `$REPO/../fhir/doc/fhir-specifications`
first, keeping the two legacy paths after it so an ancestor checkout still
works.

**And the staleness check that let it persist.** `run_tests` rebuilt the corpus
only when the *directory* was missing:

```sh
[ -d "$CORPUS_DIR" ] || corpus >/dev/null 2>&1 || true
```

A directory full of dangling symlinks satisfies that, so nothing ever rebuilt
it. The check now requires every link to resolve, and is non-empty-asserting so
an empty directory counts as stale too. Verified by breaking a link deliberately
and watching it rebuild.

**Verified, not assumed.** After the fix the links rebuild against the monorepo
(1,664 + 2,912 + 2,824 files) and the live suite runs: **1,200 live round-trips
through PostgreSQL 18 — 400 per release — with 0 failures**, plus the audit,
concurrency, redaction, upgrade and search suites green.

A second defect surfaced on the way and is fixed with it: `db.sh` did not set
`PGSSLMODE`, so after **F-17** made the default verify, the whole live suite
failed with `error performing TLS handshake` against a plaintext dev container.
That opt-out was added for `fhir-mysql` and `fhir-mariadb` when **F-54** landed
and missed here — the same omission, one port later.

**The other Store ports were re-measured too**, since the same broken
`find_spec` was in all six scripts. With it fixed, `fhir-mysql` and
`fhir-mariadb` each run **102** tests green against their live engines — the
conformance matrix said 97, a number that predated this session's additions and
had no date on it. Both rows now carry the measured figure and the date.

`fhir-sqlite` was further out: the matrix said **61**, the measured figure is
**105**.

That is a small correction with a general point behind it: a count in a status
document decays silently. None of these was wrong when written; each became
wrong without anyone touching it, which is the same failure mode as a stale
`[x]` (**F-27**), just less dramatic. All three rows now carry the date they
were measured, so the next reader can see how old the number is instead of
having to trust it.

---

## F-56

**Every port's `book/` still describes PostgreSQL and a REST server, in the
long-form documentation a user actually reads.** Severity: **High**. Violates
`C0.11`, `C0.17`. *Found continuing the audit sweep after **F-55**.*

This is **F-01 in the file F-01 did not cover**, one level up from **F-27**. The
READMEs were rewritten and the task lists are being rewritten; the books were
never touched. They are ten chapters per port, six ports, and they open like
this — in `fhir-sqlite`:

> fhir-sqlite stores FHIR resources in **PostgreSQL 18** as real relational
> tables … and serves them back through the standard **FHIR RESTful API**.
>
> — `book/src/introduction.md`

> You need **PostgreSQL 18** and Rust.
>
> — `book/src/getting-started.md`

> A fhir-sqlite store is **plain PostgreSQL**: `pg_dump`, physical replication…
>
> — `book/src/operations.md`

Neither sentence is true of any port but one, and the last is actively harmful:
it tells an operator their backup strategy is `pg_dump` for a database that is a
single file.

**Measured:**

| Port | chapters | files naming PostgreSQL | files describing a server |
| --- | --- | --- | --- |
| `fhir-postgresql` | 10 | 9 | 8 |
| `fhir-sqlite` | 10 | 7 | 8 |
| `fhir-mysql` | 10 | 7 | 8 |
| `fhir-mariadb` | 10 | 7 | 8 |
| `fhir-mssql` | 10 | 7 | 8 |
| `fhir-oracle` | 10 | 7 | 8 |

Two distinct problems, as with **F-27**:

1. **Engine substitution** — five ports describing PostgreSQL. Purely factual,
   fixable now.
2. **A REST server** — all six, `C0.17`. No port has one, and whether one is
   planned is the same undecided question **F-27** class 1 records.

**Why High.** A README is a page; a book is what someone reads to learn how to
operate the thing. `C0.11` forbids documentation describing a capability above
the port's level, and this describes a different product entirely. It has also
been building the whole time: `mdbook build book` runs in every port's CI job —
which, per **F-49**, has never executed.

### What is fixed, 2026-08-03

**The two chapters a reader acts on first**, in all five non-PostgreSQL ports:

- **`introduction.md`** now names the port's own engine, says plainly that the
  crate is a **library** with no `serve` and no REST API, and no longer claims
  the corpus "round-trips through live PostgreSQL" — for a port whose evidence
  is the shred/reconstruct engine, that sentence attributed the wrong proof to
  the wrong thing.
- **`getting-started.md`** replaced the invented CLI session — `init`, `load`,
  `serve`, `transform`, `search`, `get`, `export`, none of which exist in any
  port — with what is actually there: three crates and no binary. Each port
  states its own namespace mapping, which genuinely differs: a database on
  MySQL and MariaDB, a schema on SQL Server, an attached file on SQLite, and a
  **user** on Oracle (`M14.5`), which that port does not create (`M14.28`).

The Rust example is marked `rust,ignore` and says why: an uncompiled example is
how the previous page came to document a binary that was never built. The
README carries the compiled one.

**Every book now opens with a banner** naming what in it is still wrong and
pointing at the conformance matrix — the same stopgap **F-27** used, for the
same reason: the misleading has to stop before the rewriting finishes.

All six books still build (`mdbook build book`).

### The engine substitution is now fully fixed

Files naming PostgreSQL fell from **7 per port to 3**, and all three remaining
are deliberate — the banner, and text that says what the port is *not*.

Beyond the two entry chapters:

- **`operations.md`** — the backup advice was the actively harmful one, and is
  now each engine's own: a file copy or `VACUUM INTO` for SQLite, `mysqldump`
  and binlog replay for MySQL, `mariadb-dump` for MariaDB, `BACKUP DATABASE`
  and log shipping for SQL Server, RMAN and Data Guard for Oracle. The two
  scaffolds say so conditionally, because they have no store to back up.
- **`architecture.md`** — the store is no longer `tokio-postgres + deadpool` in
  all six. This is the *fourth* place that same sentence was found, after the
  READMEs (**F-01**), the task lists (**F-27** class 3) and the crate docs
  (**F-53**).
- **`trust-boundary.md`** — the backup row pointed at `pg_dump`/PITR for every
  engine.
- **`fhir-versions.md`** — "installs into its own PostgreSQL schema" is now each
  engine's real namespace: a database on MySQL and MariaDB, a schema on SQL
  Server, an attached file on SQLite, and a **user** on Oracle that the port
  does not create (`M14.5`, `M14.28`).
- **`storage-model.md`** — claimed the wide-choice split respects "PostgreSQL's
  column limit".

That last one produced a mistake worth recording. The first correction asserted
a specific limit per engine — 2,000 for SQLite, 4,096 for MySQL, and so on — and
that the threshold was "chosen from the tightest engine". Checking the code
refuted both: `SPLIT_WIDTH` is **150**, set once in the shared generator, and its
own comment derives it from *PostgreSQL's* 1600-column limit. The text now says
150, says it is identical in all six, and claims only that it sits below every
supported engine's limit — which is the property that actually matters and the
one that can be checked. Asserting five unverified numbers to fix a
PostgreSQL-centric claim would have been the same defect in a new coat.

All six books build.

### The REST half, resolved 2026-08-03

The owner settled **F-27** class 1: the server is **`fhir-loco`**, a separate
crate (Loco.rs, Axum, Tokio, Hyper) mounted over a store.

Every book — all six now, including `fhir-postgresql`, which had no banner at
all — opens by saying that any `serve` command, endpoint or status code in the
chapters below is `fhir-loco`'s behaviour and not the library's. That is the
honest fix short of rewriting eight chapters per port: the text is no longer
*false*, it is attributed.

Rewriting those chapters to describe `fhir-loco` properly is a documentation
task rather than a correctness one, and it is not this finding — the books no
longer claim a capability the crate lacks, which is what `C0.11` and `C0.17`
require.

All six build.

## F-57

**`fhir-loco`'s CapabilityStatement declared a read-only server while the router
served writes.** Severity: Medium. Violates `A7.12`. *Found checking the ports'
REST milestones against the crate that actually implements them (**F-27**
class 1).*

`GET /{version}/metadata` advertised three interactions per resource type:

```json
"interaction": [{"code":"read"}, {"code":"vread"}, {"code":"search-type"}]
```

The router has carried writes since it was written:

```rust
.add("/{version}/{rtype}", get(search).post(create))
.add("/{version}/{rtype}/{id}", get(read).put(update).delete(delete_))
```

A client doing conformance-driven discovery — reading `metadata` to decide what
it may attempt — would have concluded this server was read-only and never tried
a create, update or delete. The endpoints worked the whole time.

**`A7.12` in the other direction.** The requirement is normally read as "do not
declare what you cannot do", and every check written for it looks for
over-claiming. This under-claimed: safer, still wrong, and nothing compared the
two lists.

That is the shape `U11a` names — where two artifacts must agree, assert the
agreement, because each side is self-consistent while contradicting the other.
`metadata_declares_every_interaction_the_router_serves` now does, and is
mutation-verified: removing the three write interactions fails it and prints the
declared list.

**Second defect, same statement.** `software.name` was `"fhir-store"`. Since the
split (**F-45**) that name belongs to the engine-agnostic persistence core; the
server is `fhir-loco`. Every CapabilityStatement this service emitted identified
itself as a different crate.


## F-58

**`fhir-loco` is the service §10 and §12 specify, and does not yet meet several
of their requirements.** Severity: Medium. *Opened closing **F-05**.*

**F-05** retained the `[service]` requirements on the reasoning that "the
obligations are real and will bind whatever service is built". That service is
`fhir-loco`. Measured against it:

| Requirement | Asks for | `fhir-loco` |
| --- | --- | --- |
| FHIR status codes | `400/404/410/412/500` and `OperationOutcome` | **yes** — `200 201 204 400 401 404 410 412 500 503`, and an `OperationOutcome` mapper |
| `PR12.1`–`PR12.4` attribution | a verified principal on every write | **yes** — PASETO v4.public, no unauthenticated mode |
| `O10.8` body limit and timeout | limits at the edge | **now set** in production — `32mb`, `30s` (**F-59**) |
| `O10.8` concurrency / in-flight | shed as `503` with `Retry-After` | **unmet** — Loco 1.0.1 exposes neither |
| `O10.9` admin plane | metrics and health on a separate bind address | **unmet** — one listener, and no `/metrics` at all |
| ~~`A7.10`~~ conditional create | racing `If-None-Exist` yields exactly one resource | **unreachable** — the store implements `conditional_create_audited`, the router does not expose it |
| ~~`M8`~~ `$export` | Bulk Data | **absent** |

The first two are the load-bearing ones and they are met — a server that
attributed writes to nobody, or answered a deleted resource with `404` instead
of `410`, would be wrong in ways that corrupt data or mislead clients.

The last two rows are struck ids from the retired §7/§8 (`C0.16`) and are
included because **F-04** established they describe `fhir-loco` rather than
nothing. `A7.10` is the interesting one: the capability *exists*, in the store,
and no HTTP route reaches it — a conditional create is implemented and
unreachable.

Working the `O10.8` row found something worse than the gap it was about:
`config/production.yaml` was an **empty file**, so the service could not boot in
production at all (**F-59**). Two of `O10.8`'s limits are now set there; the
other two, and `O10.9`, are blocked on the framework rather than on effort, and
are stated as unmet rather than assumed.

**`O10.7` is a special case and is satisfied, vacuously.** It governs the
*database* link, and `fhir-loco` mounts `fhir-sqlite` — a local file, no
connection. If it is ever pointed at `fhir-postgresql`, `fhir-mysql` or
`fhir-mariadb`, `O10.7` binds for real, and those ports now default to verifying
(**F-17**, **F-54**). Nothing about the HTTP listener's own TLS is covered by
`O10.7`, and no requirement currently states it — a gap in §10 rather than in
this crate.

### Resolved 2026-08-03: `fhir-loco/spec/`

The owner settled the governance half: `fhir-loco` has its own specification,
ids `SV1.x`–`SV4.x`, four sections. The `[service]` markers in §10 and §12 stay
and now mean "binds `fhir-loco`, restated as an `SV` id" — they are **restated,
not moved**, because `C0.5` makes ids permanent and moving them across families
is how the `R4` collision happened.

**The gaps are now recorded at their own ids**, which is the point of having a
specification rather than a finding:

| Gap | Id |
| --- | --- |
| no concurrency or in-flight limit (Loco 1.0.1 exposes neither) | `SV4.2` |
| no admin plane, no `/metrics` | `SV4.3` |
| conditional create unreachable — the store implements it | `SV2.14` |
| no `$export` | `SV2.15` |
| **no requirement anywhere states an obligation for the listener's own TLS** | `SV3.11` |

That last one is worth keeping in view: it is not that `fhir-loco` fails a
requirement, it is that the requirement does not exist. `O10.7` governs the
database link. A deployment exposing this port directly carries PHI in the clear
and nothing in this repository tells it so; `SV4.4`'s loopback default is the
only mitigation, and a weak one.

**This finding stays open** for that reason — the spec question is answered, the
five gaps are not.


## F-59

**`fhir-loco` could not start in production: its production config was an empty
file.** Severity: **High**. *Found working the `O10.8` gap in **F-58**.*

`fhir-loco/config/production.yaml` was zero bytes, and committed that way. Loco
selects configuration by `LOCO_ENV` rather than merging over a default, so this
was not "starts with defaults" — it was a hard boot failure:

```
$ LOCO_ENV=production cargo run --bin fhir_loco-cli -- start
Error: YAMLFile(Error("missing field `logger`"), "config/production.yaml")
```

**The one environment this service exists to run in was the one it could not run
in.** Nothing noticed because the test suite runs as `test` and the developer
loop runs as `development`; both of those files were complete.

### What the new file sets, and why

- **`O10.8` limits, as far as the framework allows.** `limit_payload: 32mb` and
  `timeout_request: 30000`. A FHIR Bundle is legitimately large, so the body
  limit is not small — what it stops is a body that never ends.
- **`catch_panic`.** A panic must become a `500`, not a dropped connection: a
  client that cannot tell "server fault" from "network fault" may retry a write
  it should not.
- **`binding: 127.0.0.1`.** Binding `0.0.0.0` would put a FHIR API carrying PHI
  on every interface the host has. That must be a deliberate act by whoever
  deploys it, not something inherited from a file they did not read.
- **`pretty_backtrace: false`, `format: json`.** A backtrace can carry file
  paths and argument values, and this process handles PHI; the audit chain and
  access log are the intended record (`PR12.5`).
- **`cors` and `compression` off, with the reason written down.** CORS invites a
  credentialed cross-origin request against PHI, and compressing
  attacker-influenced responses is the BREACH pattern.

### What is still unmet, said rather than assumed

`O10.8` also requires a bounded concurrency limit and a maximum in-flight
request count, shedding as `503` with `Retry-After`. **Loco 1.0.1 exposes
neither**, so they are unmet and the config says so at the point where a reader
would otherwise assume coverage. Tracked in **F-58**.

`O10.9` — metrics and health on a separate bind address — is likewise unmet:
there is one listener.

### What now enforces it

`tests/config.rs`, three tests, no database. Every environment's config must
parse; production must set both limits it can; production must not print
backtraces. Mutation-verified: emptying `production.yaml` reproduces the
original error verbatim, and deleting the `limit_payload` block fails the
`O10.8` assertion.

The general lesson is the one **F-49** and **F-55** also taught: a path nothing
exercises is a path nothing has checked. Here it was an entire deployment
environment.


## F-60

**No example in `doc/` or `README.md` is compiled by anything.** Severity:
Medium. Violates `T11.9` in spirit. *Found auditing the tutorials after the
books (**F-56**).*

38 Rust blocks across `doc/*.md` and `README.md`, and nothing builds any of
them — no doctest harness, no `include_str!`, no example crate. They are marked
```` ```rust ````, which tells a reader they are code rather than pseudocode,
and nothing has ever checked that claim.

**The good news first, because it is the larger part of the finding.** The
documented API is real. Every method the tutorials call exists —
`RelMap::bundled`, `SqliteStore::open`, `init`, `put`, `put_audited`, `get`,
`search`, `history`, `vread`, `delete`, `purge`, `verify_audit`, `log_access`,
`conditional_create_audited`, `installed_checksum` — and `tutorial-04`'s search
calls match the real four-argument signature.

**Tutorial 1 was assembled into a program and run**, which is the strongest
check available and it passed:

```
installed 9480 statements
version 1
v2 U at 2026-08-03 …
v1 C at 2026-08-03 …
```

Both of its assertions held: `get` round-tripped the resource byte-for-byte, and
searching `name=aero` matched `Ærø`, so the accent fold behaves as the tutorial
says. That is a real end-to-end verification of the documented entry path.

**The one defect found.** `store.chain_witness()` appears in `doc/examples.md`
and `doc/tutorial-05`, and exists on **`fhir-postgresql` alone** — the
conformance matrix row is `• — — — — —`. Both call sites carried a trailing
comment (`// fhir-postgresql today`), which is easy to miss and easy to lose in
a copy-paste, and `tutorial-05`'s surrounding examples are written against
`fhir-sqlite`, where the line does not compile. Both are now ```` ```rust,ignore ````
with the restriction stated *above* the block rather than beside it.

`tutorial-05`'s blocks are prose fragments — they never construct a store — so
none of them compile as written. That is defensible for illustrative snippets,
but it is why the marker matters: an unmarked ```` ```rust ```` block invites a
reader to paste it.

### The gate was built, and it found six more defects

Deferring it was wrong. `scripts/check-shared-core.sh` already establishes that
a **local** gate is useful here without CI — `AGENTS/release.md` step 0c invokes
one — so "worth little until F-49" was an excuse rather than a reason.

`scripts/check-doc-examples.sh` extracts every ```` ```rust ```` block from
`doc/*.md` and `README.md`, wraps it in a preamble supplying the four names the
prose has already established (`map`, `store`, `patient`, `audit`), and compiles
it. **20 of 36 blocks failed on the first run.**

Most were harness artifacts and were fixed in the harness, not the docs — a
trailing expression colliding with the appended `Ok(())`, a missing import, and
an `anyhow::Result` context that `Box<dyn Error>` could not stand in for. Nine
were listings of parameter values or constructors rather than statements, and
are now ```` ```text ````, which is what they always were.

**Six were real, and every one of them would have failed for a reader:**

| Where | Defect |
| --- | --- |
| `examples.md` | `store.purge(rtype, id, audit, reason)` — `purge` takes **three** arguments; the reason travels on the `Audit`. The *same* bug as `tutorial-05`'s, at a second site |
| `examples.md` | `map.resource("Patient")` and `map.tables` — `RelMap` has neither; tables belong to each `ResourceMap` |
| `examples.md` | `out.tables` — `ShredOut` is a flat `rows: Vec<Row>`, each row naming its table by index |
| `examples.md` | `e.is_conflict()` — no such method; `StoreError::Conflict { expected, found }` is matched directly |
| `examples.md` | `CondCreate::Ambiguous(n)` — the variant is `Multiple` and carries nothing; and `conditional_create_audited` takes **four** arguments, with the resource type first |
| `examples.md` | **syntactically invalid** — `let bytes = std::fs::read(` with no closing paren, followed by an unrelated line. Wreckage from an earlier edit, sitting on the examples page |

The last is the one that makes the case for the gate: it is not a subtle API
drift, it is source that cannot parse, and it survived in the file the project's
own index advertises as "short, runnable recipes".

**A note on method.** While fixing the `purge` call I wrote
`with_reason("…")`, and the gate rejected it — the signature takes
`Option<String>`. I introduced a second wrong example in the act of fixing the
first, and only the compiler caught it. That is the argument for the gate in one
line.

Four blocks remain ```` ```rust,ignore ````, each with an HTML comment saying
why: one targets `fhir-postgresql` rather than `fhir-sqlite`, two continue a
previous block, and one elides a struct's fields. `ignore` is a claim that the
block is illustrative, not a way to silence the check, and the script says so
when it fails.

**Now: `OK: all 24 documentation example(s) compile.`**


## F-61

**All six `plan.md` files describe PostgreSQL, a CLI, and a server crate.**
Severity: Medium. Violates `C0.11`. *Found auditing `plan.md` as `tasks.md`'s
sibling after **F-27**.*

`tasks.md` was corrected in three passes (**F-27**). `plan.md` sits beside it,
is linked from the repository index as "a port's design decisions", and had
never been looked at. All six are **exactly 183 lines** — one template, copied.

`fhir-sqlite/plan.md` opened:

> Ground-up rewrite of fhir-sqlite: fully normalized relational storage of FHIR
> R3/R4/R5 in **PostgreSQL 18**, with a FHIR **REST server and CLI**.

Every non-PostgreSQL port named PostgreSQL ten times: `tokio-postgres +
deadpool` as the driver decision, "63-byte PostgreSQL" identifier budget,
"per-version PostgreSQL schemas", and a risk entry about `unaccent` being a
PostgreSQL extension.

All six additionally described a crate layout of five members —
`-map`, `-gen`, `-store`, `-server` (axum), and a CLI binary — and an
`M4 — REST server` milestone. Three of those five have never existed.

**Fixed.** Each plan now names its own engine, driver and namespace concept —
an attached file on SQLite, a database on MySQL and MariaDB, a schema on SQL
Server, a **user** on Oracle. The crate layout says plainly that there is no
server crate and no CLI, and `M4` records that the REST surface became
`fhir-loco`.

Two entries were **kept rather than deleted**, because the decision they record
is real even where the reasoning was another engine's:

- `D5` (driver choice) keeps the original tokio-postgres argument as history and
  names this port's actual driver.
- `R7` (`unaccent` is an extension) is why the fold is pure Rust — which is
  precisely why `P6.6` behaves identically on all six engines. Deleting it would
  discard the reason for a design that is still load-bearing.

Each file now opens with a banner saying it records *why*, not *what has been
done*, and pointing at the conformance matrix.

PostgreSQL mentions per non-PostgreSQL plan: **10 → 3**, all three deliberate
history.


## F-62

**Every port's `CHANGELOG.md` is `fhir-postgresql`'s, and two of them announce a
security fix that never shipped there.** Severity: **High**. Violates `W16.12`.
*Found auditing `CHANGELOG.md` after `plan.md` (**F-61**).*

`W16.12` is unusually direct: "A port's `CHANGELOG.md` MUST describe changes to
**that** port. An entry inherited from another port's history describes work
that was not done here."

All six are that inheritance. The sharpest instance is in `fhir-oracle` and
`fhir-mssql`, which have **no store and no driver**:

> **Fixed — PHI crossed to PostgreSQL in the clear.** The connector was
> hard-coded `NoTls`, so `sslmode=require` could not be honored. Connections now
> go through rustls, honoring `sslmode` and `PGSSLROOTCERT` (O10.7)…

There is no connector in either port. There never was. **This is a changelog
announcing a security fix that did not ship**, in a component that handles PHI,
under a release heading — and a changelog is the strongest claim a project
makes, because it says a thing *happened*. A reader auditing whether their
deployment has the TLS fix would conclude it does.

The same files also claim a live corpus round-trip "through live PostgreSQL 18",
`tokio_postgres::Error` handling, `jsonb` removal, and `fhir-<engine> init
--upgrade` — a CLI no port has.

**Why this is not simply deleted.** Two reasons, and they pull in opposite
directions from **F-27**, where the fix *was* deletion:

1. A changelog is a **historical record**. Rewriting history to say something
   else happened is a different kind of dishonesty from leaving it wrong.
2. The shared half is real. The generator, the shred/reconstruct engine and the
   fold are byte-identical across all six (`X15.1`), so those releases genuinely
   did change these crates. An entry about `canon.rs` is true everywhere.

**Fixed by marking, not rewriting.** Each of the five non-PostgreSQL changelogs
opens with a banner naming what in it is inherited and, for the two scaffolds,
stating that anything about a connector, TLS, a live round-trip, `jsonb`, or a
CLI **did not happen in this port**. The security entry is annotated in place at
both scaffolds, because a banner 100 lines up is not where a reader auditing a
CVE will be looking.

The store ports' banners are narrower: their drivers exist, so what is wrong is
the *name* — `tokio-postgres`, `PGSSLMODE`, advisory locks — rather than the
event.

`fhir-postgresql`'s own changelog is untouched and correct; it is the original.

**What remains.** A per-port changelog that is genuinely this port's history
requires deciding what the shared releases mean for each — arguably they should
carry the shared crate's version rather than a per-port one. That is the same
family of question as **F-58**, and it is the owner's.


## F-63

**Status text across `doc/` and `AGENTS/` had decayed past the point of being
wrong.** Severity: Medium. *Found sweeping the root-level documentation after
the per-port files (**F-62**).*

Not one large defect; four independent claims that were true when written and
had quietly stopped being so. They are recorded together because the cause is
one — a status sentence with no date and nothing that rechecks it — and because
that cause is the same as the stale test counts in **F-55**.

| Where | Said | Actually |
| --- | --- | --- |
| `doc/faq.md` | "**Is this a FHIR server?** No." | The ports are not; the repository has one — `fhir-loco`. A reader asking the title question about the repo got the wrong answer |
| `doc/choosing-an-engine.md` | sqlite/mysql/mariadb have "no concurrency, redaction, or audit test" | All three carry `concurrency.rs`, `redaction.rs`, `roundtrip_types.rs` and `upgrade.rs`; 102–105 tests each, green live |
| `doc/choosing-an-engine.md` (×2) | the hash-chain pre-image "is still derived in SQL" (**F-07**) | **F-07** is fixed; `canon.rs` is shared and identical in all six |
| `AGENTS/release.md` | "one port blocked on a High finding: `fhir-oracle` has **F-08**" | **F-08** is fixed. No port has an open High finding of its own — but **every** port is blocked by **F-49**, which that paragraph did not mention |

The last is the most misleading, because it understates: it named one blocked
port when all six are, for a reason discovered later and never folded back in.

**Fixed**, each with what replaced it stated rather than silently swapped. The
FAQ now answers the question it is actually asked, and says why it used to say
otherwise.

**No gate proposed.** A "recheck every status sentence" tool is not a realistic
thing to build. What is realistic, and what **F-55** already established, is
that a status claim should carry the date it was measured — three conformance
matrix rows now do. Extending that convention to prose is a habit rather than a
script.

---

Part of the [fhir-databases specification](index.md).
