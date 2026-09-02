# Audit findings

Non-normative. This is the register of known divergences between what the
specification requires, what the documentation claims, and what the code does.
Every finding carries evidence a reader can check.

A finding stays here until it is fixed or the spec is amended to match reality.
Deleting a finding because it is inconvenient, or because the text that stated
it was rewritten, is the failure mode this file exists to prevent.

**Audit date:** 2026-07-31. **Remediation pass:** 2026-07-31.
**Documentation and publish-readiness pass:** 2026-08-01 (**F-30** to **F-34**).
**Comprehensive re-audit:** 2026-08-06 (**F-73** to **F-84** — see [that
pass's own section](#the-2026-08-06-pass) below).
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

Of the original twenty-nine findings, **all twenty-nine are now fixed, closed,
or resolved** (measured 2026-08-06 against the summary table below; an earlier
revision of this paragraph said "twenty-three fixed and five open" long after
the five had closed — **F-73**). Eleven were found during the remediation pass rather than the
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

**F-15 is closed on all six ports** (as of 2026-08-09), and so is **F-07**.
Every port has `upgrade` and `backfill_norm`, each verified against a live
engine; `fhir-oracle` was the last, built as **F-47**'s step 1. Closing F-07
also emptied the shared-core gate's exemption list — **100 files identical
across all six ports, nothing excused** (75→100 when the gate widened under
**F-48**; an earlier revision of this paragraph said 65).

What is left open, as of 2026-08-10: **F-51** (narrowed by **F-68**, closed
2026-08-29) and **F-67** (the TLS advisory exposure in `fhir-mssql-store`,
which was a risk-acceptance decision, then closed by a driver swap
2026-08-29). **F-47** left this list
2026-08-10: its six-step physical-schema migration ran to completion (the
entry has the step-by-step account), and step 5 surfaced and fixed **F-85**
on the way. (An earlier revision of this list also carried **F-58**, which
had already closed 2026-08-09 — `fhir-loco`'s remaining feature work,
`_include`/`_revinclude` since served (`SV2.16`), transaction Bundles,
type-/system-level history, multi-port wiring, is tracked in that crate's
`tasks.md`, not by this register.)

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
| [F-15](#f-15) | Low | `_norm` backfill is unavailable on four ports after a fold change | **fixed** on all six ports, each live-verified; oracle last, 2026-08-09 (**F-47** step 1) |
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
| [F-47](#f-47) | Low | `path` is bound to unbounded types on mssql and oracle, so `U12` is unsatisfied; fixing it is a physical-schema migration for all six (the `v_kind` half proved moot, 2026-08-10) | **fixed** 2026-08-10 — the six-step migration ran to completion (2026-08-09/10): `U12a`'s `path_bound` recorded in every asset and enforced at shred; mssql and oracle bounded on fresh installs and live-verified converting existing ones; matrix `U12` row `•` across all six. Step 5 also surfaced and fixed **F-85** |
| [F-48](#f-48) | Low | the shared-core gate did not watch `gen/tests/`, and could not while its normalization was line-based — rustfmt wraps by crate-name *length* | **fixed** 2026-08-02 — token-based verdict, 75→100 files |
| [F-49](#f-49) | **High** | No workflow in this repository runs: all 20-odd sit under `<family>/.github/workflows/`, which GitHub does not read. Every "gated in CI" claim is unverified | **fixed** 2026-08-06 — root gates first (`gates.yml`), then every family's CI consolidated to root files with paths filters and working-directory defaults; first hosted run pending a push |
| [F-50](#f-50) | Medium | The `U2a` reference rule attached an adjunct to `c_url`, which no index uses, while every port indexes `(c_type, c_id)` — 453 of R5's 1,947 search targets unindexable on Oracle | **fixed** 2026-08-02 — all six; gaps now 0 |
| [F-51](#f-51) | Medium | `fhir-oracle`'s DDL was executed by hand, not by a test, so `C0.9` keeps the port at Scaffold; a live test needs an Oracle driver decision | **fixed** 2026-08-29 — the "driver decision" turned out to already be decided by precedent: `fhir-oracle-store` (F-68) already proves the `oracle` crate + Instant Client works live, so no architectural choice remained, only mechanical work. `tests/oracle_ddl.rs` (`fhir-oracle-map`) installs a sampled schema live, verified against `gvenzl/oracle-free`, wired into `fhir-oracle-ci.yml` |
| [F-52](#f-52) | **High** | The repository's only live database test was flaky — its cleanup dropped tables before foreign keys and discarded the error, so failures were misattributed to a correct `CREATE TABLE` | **fixed** 2026-08-03 — 5/5 runs green |
| [F-53](#f-53) | Medium | Every store crate's module doc called itself "the PostgreSQL layer" and described operations the two scaffolds do not have — F-01 in `src/` | **fixed** 2026-08-03 — all six |
| [F-54](#f-54) | **High** | `fhir-mysql` and `fhir-mariadb` carried PHI over an unencrypted database link with no way to encrypt it — the `minimal` Cargo feature excluded TLS entirely | **fixed** 2026-08-03 — `SslMode`, verifying default, live-verified on both engines |
| [F-55](#f-55) | **High** | `scripts/db.sh` resolved the FHIR® packages through the ancestor project's path and one developer's home directory in all six ports, so the live corpus suite could never find its inputs | **fixed** 2026-08-03 — 1,200 live round-trips now green on PostgreSQL 18 |
| [F-56](#f-56) | **High** | Every port's `book/` describes PostgreSQL and a REST server — F-01 in the long-form documentation, incl. telling a SQLite operator to back up with `pg_dump` | **fixed** 2026-08-03 — engine substitution corrected throughout; REST text now attributed to `fhir-loco` in all six banners |
| [F-57](#f-57) | Medium | `fhir-loco`'s CapabilityStatement declared a read-only server while the router served `POST`/`PUT`/`DELETE`, and named its software `fhir-store` | **fixed** 2026-08-03 — mutation-verified agreement test added |
| [F-58](#f-58) | Medium | `fhir-loco` is the service §10/§12 specify; five obligations remained unmet, incl. no stated requirement for the listener's own TLS | **closed** 2026-08-09 — every named gap served or resolved: `SV2.14` conditional create and `SV4.3` admin plane (2026-08-07), `SV3.11` posture stated and enforced (2026-08-07), `SV2.15` system-level async Bulk Data `$export` (2026-08-09, owner-directed). `SV4.2`'s two missing halves remain a recorded Loco 1.0.1 framework limit, tracked in that crate's spec rather than here |
| [F-59](#f-59) | **High** | `fhir-loco/config/production.yaml` was an empty file, so `LOCO_ENV=production` refused to boot — the only environment it exists to run in | **fixed** 2026-08-03 — real config, 3 mutation-verified tests |
| [F-60](#f-60) | Medium | No example in `doc/` or `README.md` is compiled by anything; one calls a `fhir-postgresql`-only API from a SQLite tutorial | **fixed** 2026-08-03 — `scripts/check-doc-examples.sh` added; it found six real defects incl. an unparseable block, all 24 now compile |
| [F-61](#f-61) | Medium | All six `plan.md` describe PostgreSQL, a CLI, and a `-server` crate; three of the five crates they list have never existed | **fixed** 2026-08-03 — all six corrected, banners added |
| [F-62](#f-62) | **High** | Every port's `CHANGELOG.md` is `fhir-postgresql`'s; the two scaffolds announce a TLS security fix for a connector they do not have | **partly fixed** 2026-08-03 — banners + the security entry annotated in place; per-port history is an owner decision |
| [F-63](#f-63) | Medium | Status text in `doc/faq.md`, `doc/choosing-an-engine.md` and `agents/release.md` had decayed — incl. "is this a FHIR server? No" and a fixed finding cited as blocking | **fixed** 2026-08-03 |
| [F-64](#f-64) | **High** | Every non-PostgreSQL `doc/benchmarks.md` presented `fhir-postgresql`'s measured numbers as its own, incl. a live round trip and bulk-load benchmark for the two ports with no store at all | **fixed** 2026-08-03 — corrected in all five; real harness for sqlite/mysql/mariadb is a recorded gap |
| [F-65](#f-65) | **High** | `fhir-mssql` gained a real store, live-verified; running it found five real defects, incl. a torn read (`R4.5`) and `verify_audit` never checking the keyed tag | **fixed** 2026-08-04 — all five; the port is Store level, 33 live tests, 0 ignored |
| [F-66](#f-66) | — | Scope note: `fhir-oracle`'s store was written with no Instant Client on the host and had never connected to a database | **superseded** by **F-68** — the store has now run live |
| [F-67](#f-67) | **High** | Four TLS advisories `deny.toml` excused as dev-dependency-only now reach the shipping `fhir-mssql-store`; `native-tls` fails the handshake on this host | **fixed** 2026-08-29 — owner accepted the risk formally 2026-08-28 (keep shipping on upstream `tiberius`, document loudly, do not chase a replacement, after pricing three alternatives — `M14.34`), then closed it outright the next day by switching the driver to `mssql`, a `tiberius` fork the owner publishes to carry security fixes forward. `cargo tree` confirms none of the four advisory packages remain; `deny.toml`'s ignore list is empty; full live suite (41 tests) re-verified against `azure-sql-edge` |
| [F-68](#f-68) | — | `fhir-oracle` connected live and reached Store level; four real defects found and fixed doing it; `R4.5` regressed from "believed addressable" to a confirmed open gap | **recorded** 2026-08-04 — defects fixed; `R4.5` remains open (`M14.19` needs a new answer) |
| [F-69](#f-69) | Medium | `scripts/db.sh up` silently exited 1 with zero output on a fresh checkout, in all six ports | **fixed** 2026-08-04 |
| [F-70](#f-70) | Medium | `fhir-store`'s `ChainKey`/`KeyRing::from_env` hardcoded `FHIR_SQLITE_*` names, so five ports' documented chain-key variables silently did nothing | **fixed** 2026-08-04 — `from_env(prefix)`, live-verified on PostgreSQL 18; docs corrected in all six |
| [F-71](#f-71) | **High** | `fhir-sqlite`: `active=true` token search silently matched zero rows — TEXT/INTEGER affinity never converts the word `"true"` | **fixed** 2026-08-04 — `bool_token_as_bind`, regression test added |
| [F-72](#f-72) | Medium | Root `CLAUDE.md` described `fhir-store/` as the HTTP surface, carried an obsolete nested-repo warning, and said both former scaffolds "have no store" | **fixed** 2026-08-04 |
| [F-73](#f-73) | Medium | This register and its intro fell behind their own findings: no summary rows for F-65–F-72, "what remains" listed five closed findings as open, counts stale | **fixed** 2026-08-06 |
| [F-74](#f-74) | Medium | Conformance-matrix cells stale: `put_audited` conflation, "no store" notes for two Store-level ports, the CI-gate note, `W16.15`, two contradictory mssql test counts | **fixed** 2026-08-06 |
| [F-75](#f-75) | **High** | `fhir-mssql`/`fhir-oracle` `CHANGELOG.md` still open "no store and no driver" and point readers to an *Unreleased* section that claims a `serve` binary, a CLI, and live PostgreSQL 18 | **fixed** 2026-08-06 |
| [F-76](#f-76) | Medium | Four crates.io-facing `Cargo.toml` descriptions still say "SCAFFOLD"/"emits MySQL"; `spec/publishing.md` P-1 rests on the same dead premise | **fixed** 2026-08-06 |
| [F-77](#f-77) | Medium | `spec/databases/` sections still describe the pre-Store world (index "DDL only, no store"/"scaffold only"; §3's Oracle `⚠` table; §16's `W16.x` "currently" claims; citations naming fixed findings as open) | **fixed** 2026-08-06 |
| [F-78](#f-78) | **High** | `fhir-mysql`/`fhir-mariadb` `tasks.md` tick `T33`/`T34` — conditional-op atomicity and the audited-write envelope — for operations that do not exist in either crate | **fixed** 2026-08-06 — unticked, restated |
| [F-79](#f-79) | Medium | Closed findings not propagated: `upgrade`/`backfill_norm` still narrated as missing in three ports' docs (F-15), `Prefer` TLS default still narrated (F-17), oracle's "eleven `#[ignore]`d tests" (F-08), `fhir-loco`'s "two scaffolds" | **fixed** 2026-08-06 |
| [F-80](#f-80) | Medium | F-27's class-1 disposition — delete the misattributed REST/CLI entries — was recorded 2026-08-03 and never executed; class-3 residue also survived in all four store ports' `tasks.md` | **fixed** 2026-08-06 — executed |
| [F-81](#f-81) | Medium | Six ports' `plan.md` decision entries are status-bearing and wrong — worst: `fhir-oracle` D18 asserts `R4.5` is handled by a mechanism Oracle rejects (`ORA-01466`), and D20 asserts TLS neither former scaffold has | **fixed** 2026-08-06 |
| [F-82](#f-82) | Medium | `fhir-loco/tasks.md` predated the crate's own spec: zero `SV` ids, shipped features omitted, three provably-obsolete open items (git remotes, shared history, the fixed T70 fold) | **fixed** 2026-08-06 — replaced |
| [F-83](#f-83) | Low | `fhir-oracle`'s book lacks the F-56 banner that root `CLAUDE.md` says all six books carry | **fixed** 2026-08-06 |
| [F-84](#f-84) | Medium | **All six** ports' `publish.yml` iterate a `fhir-<engine>-server` crate and a CLI crate, and all six `release.yml` build a `fhir-<engine>` binary — none of which has ever existed (wider than first recorded: not just the two former scaffolds) | **fixed** 2026-08-06 — publish loops corrected in all six; the six binary-release workflows deleted outright |
| [F-85](#f-85) | Medium | `fhir-oracle` refused every root-level extension outright (`ORA-01400`): the empty attach path binds as NULL (`''` is NULL) against `"path" CLOB NOT NULL` — a US-Core-style Patient could not be stored at all | **fixed** 2026-08-10 — bounded `"path"` is nullable (`M14.39`), NULL means the empty path; fresh installs via `create_table`, existing ones via F-47 step 5's conversion; live-verified both ways |
| [F-86](#f-86) | Medium | The `fhir/` model family (every release, R2–R6 and R4B) rejects FHIR JSON's null-padded primitive arrays (`"event": [null]` beside `_event`): repeating primitives are `Vec<T>` and cannot hold a placeholder position | **fixed** 2026-08-10 (owner-directed: a dedicated container) — `0..*` primitives are now `fhir_core::PrimVec<T>` (`R6.7a`), a transparent `Vec<Option<T>>` whose `None` is the extension-only placeholder; the nine R4B corpus examples round-trip and left the allowlist at the gate's own demand. Stated residual: `1..*` primitives keep `Vec1` and its type-level non-emptiness, so an ext-only position there stays unrepresentable (loudly refused, F-87; no corpus example uses it) |
| [F-87](#f-87) | **High** | A choice element (`timing[x]` and kin) whose content fails to parse is **silently dropped** — the resource deserializes "successfully" minus the element, data loss masquerading as success | **fixed** 2026-08-10, same day — every choice-bearing struct now deserializes through a generated shadow whose choice fields are non-`Option` `choice::Slot`s, so a present-but-invalid element errors loudly; all six release crates, five corpora green |
| [F-88](#f-88) | **High** | The consolidated port workflows (F-49) left three per-job settings unrooted, and the first hosted runs exposed all three: `cargo-deny` ran at the repo root and errored; the spec/corpus env paths pointed at the root while the fetch steps wrote under the port directory, so **every spec-dependent live test silently skipped and the "live gate" was green while testing nothing** (T11.12's nightmare at workflow scale); and the plaintext pg job lacked an explicit `PGSSLMODE`, so the store's secure-by-default `require` (O10.7) correctly refused it — surfaced only by the two new `history_page` tests, the sole live tests that actually connected | **fixed** 2026-08-10 — paths re-rooted and deny given its manifest in all six workflows; the plaintext job says `disable` explicitly with the TLS-only job carrying `require`; and a vacuity guard now fails the pg live step if anything skipped on CI |
| [F-90](#f-90) | **High** | The full R3/R4/R5 schemas **do not install** on stock MySQL 8.4 / MariaDB 11.4: InnoDB's create-time row-size check (`ERROR 1118`, > 8126 bytes) charges ~41 bytes per `TEXT` column (measured by bisection: 195 fit, 196 fail) and the widest generated tables carry up to 232–257 columns with ~190–211 `TEXT`s — the open-typed `value[x]` splats (`parameters_parameter_value`, `task_input_value`, the `StructureDefinition` element `defaultValue`/`fixed`/`pattern`/`example` tables) and `explanation_of_benefit`'s base | **fixed** — closed in full 2026-08-12, as this row's own narrative records (an earlier revision of this cell still began "open", contradicting its ending — the F-73 failure mode, corrected 2026-08-26). Found by the first full-schema CI install (`DDL_FULL=1` is CI-only; local suites sample and the 2026-08-03 "green against live MySQL 8.4" predates the widest tables' exercise). Unmasked by F-89's harness fix. The dialect cannot fix it alone (tables are map-shaped and the stores write by table name); the recommended fix is a byte-aware force-split in the shared generator (`SPLIT_WIDTH` is column-count only), budgeted for the tightest engine (~7,500 conservative bytes), which changes table shapes in all six ports — owner-directed 2026-08-11: the byte-aware force-split landed at the shared generator (`G2.6a`), trigger 6,600 / budget 7,900 charged bytes, widest resulting table 6,611; all assets and fixtures regenerated, `row_budget.rs` gates the artifacts. **Live-verified 2026-08-11**: full R3/R4/R5 installs green on MySQL 8.4 and MariaDB 11.4, and both workflows fully green for the first time (unmasking F-91 on the way). **Closed in full 2026-08-12**: the `O10.4b` moved-column guard landed in all six stores and is live-verified on **all six engines** — oracle's included, on its live job's first hosted run (2026-08-12, the F-06 gate restored with a real engine) |
| [F-91](#f-91) | Medium | The mysql/mariadb **store suites never ran in CI**: the DDL step ahead of them failed (F-90) and cargo stops at the first failing test binary, while the step still called itself "expected to skip until T64" — stale twice over, since the stores have spoken their own engines for weeks. Their first real execution (2026-08-11, the moment F-90's fix let the DDL step pass) refused the service container's self-signed certificate under the verifying default (`UnknownIssuer`) — the store keeping exactly the promise F-54 measures, against a job that had never declared its TLS intent | **fixed** 2026-08-11 — both live jobs declare `FHIR_*_SSL_MODE: DISABLED` (plaintext by design, mirroring the pg job's explicit `PGSSLMODE: disable`, F-88), the step is named honestly, and the stale T64 rationale for the missing TLS-only job is rewritten: what is actually missing is a `require_secure_transport=ON` server for the plaintext-refusal half |
| [F-92](#f-92) | Medium | F-91's genre, two more members, found by checking that the O10.4b tests actually *ran* in the green jobs: **mariadb's main store suite** (`mariadb_store.rs`, 13 tests) gated on `FHIR_MYSQL_TEST_DSN` — the mysql port's variable — so it skip-passed silently in CI while the header even said "skips silently"; and **mssql's live job had no store-suite step at all** — the workflow predates the store (F-65) and still said "cannot be written honestly until there is a store", so the store suite including F-47's 12 upgrade tests had never run hosted | **fixed** 2026-08-12 — the env var renamed to `FHIR_MARIADB_TEST_DSN` (14 sites, one file), the mssql step added with TLS intent already declared in the DSN (`TrustServerCertificate=true`, the F-91 lesson), and both stale rationales rewritten. **Verified** the same day: mariadb's 13-test suite ran green in its first genuine CI execution, and mssql's store suite ran green including the O10.4b tests |
| [F-93](#f-93) | Medium | `fhir-oracle`'s `O10.4c` re-shred **never passed a hosted run**, despite landing in a commit whose message said "live-verified" (2026-08-22) — that commit's own CI run was red, and every run since failed the two re-shred tests the same way. Root cause: `recon_with_map` ended with a hygiene `rollback()`, and the re-shred's byte-identical verify calls it *inside* the per-resource write transaction — the verify read the uncommitted new-shape rows, the rollback then silently discarded the delete and both re-inserts, the `commit()` committed nothing, and the leftover guard correctly reported the old data still in place ("re-shred left data behind"). Fixing that unmasked a second defect the rollback had been hiding: `drop_schema` was **map-scoped** — unlike `fhir-mssql`'s catalog-driven `sys.tables` sweep — so a table the connected map did not name (a relocated-column table from an earlier run) survived with its rows but without its FKs, and the next re-shred collided with the residue (ORA-00001 on `(rid, ords)`) | **fixed** 2026-08-26 — both rollbacks removed from `recon_with_map` (callers own their transactions; the function's doc now states why it must never end the caller's transaction), and `drop_schema` sweeps `user_tables` (`M14.5` makes the connecting user exactly this store's world). **Live-verified** on Oracle Database Free 23: the upgrade suite 16/16 **twice consecutively** — the second pass is the point, it exercises the residue class — plus the store suite 7/7 and `root_extension`; the first passing runs these two tests have ever had, hosted or local |
| [F-94](#f-94) | Low | Dependabot alerts on `main` surfaced `GHSA-rhfx-m35p-ff5j` (`lru` `IterMut` violates Stacked Borrows, an internal-pointer soundness defect) reaching `fhir-mysql-store` and `fhir-mariadb-store` as normal dependencies via `mysql_async 0.34.2`'s pinned `lru = "^0.12"`. Separately, `fhir-mysql/deny.toml` and `fhir-mariadb/deny.toml` carried an `ignore` for `RUSTSEC-2025-0134` (`rustls-pemfile` unmaintained) that `cargo deny` was silently no longer able to match — the crate it excused had already left the dependency tree, unnoticed, so the exception was dead and nobody knew | **fixed** 2026-08-27 — `mysql_async` bumped `0.34` → `0.37` in both ports (the minimal version whose own manifest requires `lru ^0.18`, clearing the advisory; `0.35`/`0.36` still required `^0.12`/`^0.14` and would not have). Verified with `cargo check --all-targets --locked` and the offline unit suite (50 tests, both ports) before the bump was accepted, not after; `cargo deny check advisories` reports clean with no `advisory-not-detected` warning. The dead `RUSTSEC-2025-0134` ignore was removed rather than left stale, with a dated comment explaining why the entry disappeared rather than leaving future silence. Unrelated to **F-67**: that finding is `fhir-mssql`'s `rustls-webpki` chain via `tiberius`, a different port and a different dependency — accepted formally 2026-08-28, then closed 2026-08-29 by switching the driver |
| [F-95](#f-95) | **High** | **F-94's own fix broke hosted CI on both ports it touched, and the verification that shipped it did not catch it.** The `mysql_async 0.34 -> 0.37` bump was verified with `cargo check --all-targets --locked` and `cargo test --locked --lib --bins` — unit tests only. `--lib --bins` excludes everything under `tests/`, which is exactly where `ssl_live.rs` lives, and that file only runs at all inside the live-database CI job (it self-skips without a DSN). The first hosted run after the push found it: `tls_is_configurable_and_verification_is_not_a_no_op` panicked in both `fhir-mysql-store` and `fhir-mariadb-store` — `rustls::crypto::CryptoProvider::get_default()` finding none installed, because `mysql_async 0.37` split its `rustls-tls` feature from its crypto backend (`aws-lc-rs` or `ring`), where `0.34`'s did not need the split | **fixed** 2026-08-28 — `ring` added to both ports' `mysql_async` feature list (pure Rust, no C/cmake toolchain, matching the existing comment's stated reason for choosing rustls over native-tls in the first place). Verified by reproducing the exact panic live against each port's own dev container before the fix, and its disappearance after — not inferred from the diff. Then the full store suite re-run for both, all binaries, no truncation: `mysql_store`/`concurrency`/`redaction`/`roundtrip_types`/`ssl_default`/`ssl_live`/`upgrade`, 44 tests, 0 failed. `cargo deny check advisories` still clean. **The verification-scope lesson, stated for next time:** a dependency bump's own tests are not enough evidence when the crate ships integration tests that need infrastructure the bump's own CI job doesn't provide — `cargo test --workspace` unit-only is what F-90/F-91/F-92's "does it actually run" lesson was already about, applied here to a different kind of change and missed anyway |
| [F-96](#f-96) | Medium | **Unrelated to F-94/F-95, found the same audit pass**: `fhir-postgresql` and `fhir-loco` (which depends on it) both failed hosted `cargo deny` with `error[yanked]: detected yanked crate` on `chacha20 0.10.1` — pulled in via `rand 0.10.2 -> postgres-protocol -> tokio-postgres`, a chain neither this session nor **F-94** touched. crates.io yanked `0.10.1` upstream after both lockfiles had already pinned it; `yanked = "deny"` in each port's `deny.toml` (the same policy line **F-94** relies on) is what caught it | **fixed** 2026-08-28 — `cargo update -p chacha20` in both workspaces (`0.10.1` → `0.10.2`, the fix `cargo deny`'s own error message named). Verified: `cargo deny check advisories` clean in both, `cargo check --all-targets --locked` green in both |
| [F-97](#f-97) | Medium | Surfaced closing **F-51**: the append-only trigger's `M3.17`/`M3.18` enforcement (`M14.29`, already known to have failed open once — `M14.29a`) and the `Bool` CHECK's `M14.8` enforcement were verified only by SQL-text unit tests, never against a live server. A CHECK clause that parses but never fires would pass the existing unit test the same way the trigger's `M14.29a` bug passed a read-through | **fixed** 2026-08-29 — `tests/oracle_constraints.rs` (`fhir-oracle-map`): a live `UPDATE`/undeclared `DELETE` against a seeded row, asserting the exact `ORA-20001`/`ORA-20002` errors (not merely `is_err()`, which is the distinction `M14.29a`'s own bug hid), the declared-erasure escape hatch confirmed to still work, and a live `INSERT` of `2` into a `NUMBER(1) CHECK (... IN (0,1))` column asserting `ORA-02290`. **Found and fixed in the same pass, not shipped and left flaky:** libtest runs a binary's `#[test]` functions concurrently by default, and the first version had both tests provision the same throwaway user — reproduced 3 of 3 failures before splitting each test onto its own user (`TRIGTEST`, `BOOLTEST`), 0 of many after. Wired into `fhir-oracle-ci.yml` beside **F-51**'s DDL-install step |
| [F-98](#f-98) | Medium | `scripts/check-published-match.sh` reports "ok" for a crate whose source has genuinely diverged from what it published, when the divergence is a workspace-inherited dependency requirement (`sha2.workspace = true` etc.) — its `--exclude Cargo.toml` comparison relies on `Cargo.toml.orig`, which preserves the unresolved `.workspace = true` reference rather than the literal version crates.io actually receives | **open** — found 2026-08-29 bumping `sha2`/`sha3` in `fhir-postgresql`/`fhir-sqlite`: the gate said "34 matched, 0 mismatched" while the crates.io API confirmed `fhir-postgresql-map` 0.6.0's published manifest declares `sha2 ^0.10`, which the tree's new `sha2 = "0.11"` workspace requirement no longer matches. Worked around by bumping the affected crates to a patch version regardless of what the gate reported (commit `ca34cdf`), not by trusting it. Not fixed: a correct fix needs to compare the crate's *normalized*, packaged `Cargo.toml` (which does resolve `.workspace = true` to a literal) rather than `Cargo.toml.orig`, without reintroducing the cosmetic-reordering false positives `Cargo.toml.orig` was chosen to avoid |
| [F-99](#f-99) | Medium | `fhir-postgresql-store`'s `checkpoints_are_logged_on_their_own_target_without_phi` test (`tests/audit.rs`) fails reproducibly — 2/2 hosted runs — on the Dependabot PR bumping `deadpool-postgres` 0.14.1 → 0.14.2 (PR #59), while `main`'s own last five hosted runs of the same job are all green | **open** — found 2026-08-31 triaging Dependabot PRs. The test captures `tracing` output via a thread-local `set_default` subscriber around `store.emit_checkpoint("test").await`, then asserts the capture contains `"audit_checkpoint"`; both runs it failed with an *empty* capture (`chain_witness()` itself still succeeds — only the 4 other tests in the same binary are unaffected, ruling out a general connectivity break). Leading hypothesis, not confirmed at the mechanism level: `deadpool-postgres` 0.14.2's changelog headline is "Coalesce concurrent statement preparations… tasks racing to prepare the same query now share a single `PREPARE`" — new inter-task coordination on exactly the code path `chain_witness()` calls, a plausible way for the actual query (and whatever thread ends up running it) to fall outside the test's thread-local subscriber scope. Each test in the file opens its own `Store`/pool (`test_store()`, no cross-test sharing), which rules out the simplest version of that theory and is why the mechanism is not fully pinned down. Not fixed: needs either confirming/ruling out the coalescing path by reading `deadpool-postgres` 0.14.2's source directly, or rewriting the test to capture via a global (non-thread-local) subscriber so it survives whichever task the emission runs on. The PR is not merged pending this. |
| [F-89](#f-89) | Medium | The mysql/mariadb DDL test harness was unportable and **masked real errors**: it passed MariaDB's `--skip-ssl-verify-server-cert` to whatever client exists (Oracle's mysql 8 client rejects it), assumed a utf8mb4 default charset (the runner's client defaults utf8mb3 → ERROR 1253 on the collation probe), and on any early client exit reported the stdin `Broken pipe` instead of reading the client's stderr — hiding whatever the real failure was | **fixed** 2026-08-10 — client-flavor-gated TLS flag, explicit `--default-character-set=utf8mb4`, and EPIPE falls through to collect stderr, so the next failure names itself |

## What remains, and why

*Rewritten 2026-08-06 (**F-73**): the previous revision of this table listed
F-04, F-08, F-15, F-17, and F-27 as open with rationales that predated their
own closures — every one of them contradicted the summary table above.*

| Finding | Why it is not fixed here |
| --- | --- |
| ~~F-47~~ | Closed 2026-08-10: the six-step migration ran to completion — oracle's `upgrade` (step 1, closing F-15), the `U12a` bound decision (step 2, corrected by measurement), the six-port shared-core `path_bound` (step 3), mssql's transactional conversion (step 4), oracle's resumable add-copy-drop-rename conversion (step 5, surfacing and fixing **F-85**), and the matrix flip (step 6). |
| ~~F-49~~ | Closed 2026-08-06: every family's CI now lives at the repository root — `fhir-ci.yml`, `fhir-security.yml`, one `<port>-ci.yml` per port, `fhir-loco-ci.yml` (rewritten: the old file was loco-rs template boilerplate provisioning Redis/PostgreSQL this app never used), and a new `fhir-store-ci.yml` for the one family that never had CI anywhere. Each carries a paths filter and a `working-directory` default; artifact paths were re-rooted. The inert per-family CI files are deleted. Deliberately **not** consolidated: the six ports' `publish.yml` (publishing is owner-gated; their fictional crate lists are fixed, F-84) and `fhir/release.yml` (cargo-dist manages it; moot until a release is cut). Honesty note: no hosted run has executed yet — this host cannot run GitHub Actions — so the matrix keeps `~` until the first push turns them green. |
| ~~F-51~~ | Closed 2026-08-29: `tests/oracle_ddl.rs` now installs and verifies a sampled schema live on every run, not just by hand; the full-R5 install remains verified only by **F-08**'s hand-run transcript, a separate, narrower gap the entry names. |
| ~~F-58~~ | Closed 2026-08-09: `SV2.14`, `SV3.11`, `SV4.3` (2026-08-07) and `SV2.15` `$export` (2026-08-09) are all served; `SV4.2`'s concurrency-limit halves stay recorded in `fhir-loco/spec/04` as a framework limit, which is a constraint, not a gap this register tracks. |
| ~~F-67~~ | Closed 2026-08-29: the driver switched from `tiberius` to `mssql`, a fork maintained to carry the TLS fixes forward — none of the four advisory packages remain in the dependency tree. |
| ~~F-98~~ | Closed by **F-102**: the normalized-`Cargo.toml` comparison is implemented, and running it found the predicted blind spot was real — twelve currently-published crates, not a hypothetical. |
| ~~F-102~~ | Closed 2026-09-02: all twelve live violations remediated and republished; `check-published-match.sh` unscoped reports `34 matched, 0 mismatched`. |

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
monorepo now has [`AGENTS.md`](../../AGENTS.md) with [`agents/`](../../agents/) topic
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

**Disposition: fixed on all six ports.** `fhir-oracle`, the last, closed
2026-08-09 (**F-47** step 1) — see the end of this entry.

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

**`fhir-mssql` now has it too**, verified against live `azure-sql-edge`. Nine
tests in `crates/fhir-mssql-store/tests/upgrade.rs`, the same suite shape as
the other three ports' plus one this port's destructive-DDL story earns:
`destructive_changes_succeed_with_the_flag`, confirming a permitted drop
actually happens and sticks — not merely that a drop without the flag is
refused, which the other ports' suites stop at.

Two things surfaced running it live are this port's own, not choices, and are
now normative (`spec/14-mssql-dialect.md`):

| | |
|---|---|
| `M14.35` | **The upgrade is one transaction.** Unlike MySQL/MariaDB, T-SQL DDL participates in a transaction like any other statement, so the whole additive-plus-destructive apply runs inside `BEGIN TRANSACTION`/`COMMIT TRANSACTION`, `ROLLBACK` on the first failure. A partial upgrade — which `fhir-mysql-store`'s own doc comment records as unpreventable on that engine — is prevented here, not merely reported. `backfill_norm` runs afterward, outside the transaction, in its own bounded batches. |
| `M14.36` | **Destructive table drops must be ordered children before their base table.** Every non-`Base` table carries `FOREIGN KEY (rid) REFERENCES base(id)`, and SQL Server refuses `DROP TABLE` on a table something still references — error 3726 — regardless of `ON DELETE CASCADE`, which governs `DELETE`, not `DROP TABLE`. Found live: `destructive_changes_succeed_with_the_flag` failed on its first run with `Could not drop object 'basic' because it is referenced by a FOREIGN KEY constraint`, because the destructive diff dropped tables in `HashMap` iteration order. Fixed by partitioning drops into non-`Base` and `Base` buckets and emitting the former first. |

`init` also needed a plumbing fix first: it had recorded only a bare
`checksum` key, never the map asset itself, so there was nothing for `upgrade`
to diff against on any schema installed before this pass. `init` now stores
`map_asset`/`fhir_version` alongside `checksum`, the same three keys
sqlite/mysql/mariadb record (under their own key names — this port keeps
`checksum` rather than adopting their `map_checksum`, since nothing here read
that name and there was no reason to rename it). As with SQLite, a database
installed *before* this revision has no `map_asset` and `upgrade` refuses it by
name (`an_install_without_a_stored_map_asset_says_so`) — the finding is closed
going forward, not retroactively.

**Mutation-verified** (`T11.10`): skipping the backfill makes the seeded
patient unfindable by their own name, the same class of check as the other
three ports.

**`fhir-oracle` now has it too** — the last port, closing this finding
everywhere (2026-08-09, **F-47** step 1). Nine tests in
`crates/fhir-oracle-store/tests/upgrade.rs`, the same suite shape as mssql's
including `destructive_changes_succeed_with_the_flag`, green against live
`gvenzl/oracle-free:23-slim-faststart` (run `--test-threads=1`: every test
shares the one uppercase `R5` schema, `M14.5`). Three things are this
engine's own, now normative in its annex (`spec/14-oracle-dialect.md`):

| | |
|---|---|
| `M14.35` | **The upgrade is resumable, not transactional.** Oracle DDL implicitly commits, so a failed upgrade half-applies and that cannot be prevented — instead every statement the upgrade emits is wrapped in a PL/SQL block that swallows exactly the already-applied codes (`ORA-00955` name in use, `ORA-01430` column exists, `ORA-01408` already indexed; `ORA-00942`/`ORA-00904` for drops) and re-raises everything else, so the recovery for a partial upgrade is rerunning `upgrade`. The third answer to one problem: mssql's `M14.35` is one transaction, mysql's `M14.35` is reported-partial. |
| `M14.36` | **The map asset cannot bind as one string.** ~1 MB of hex fails with `ORA-01461` even though the target column is a `CLOB` — the bind is what overflows — so meta values past the limit are stored as ≤3,000-char `<key>.<i>` chunk rows under a `chunks:N` sentinel and reassembled on read. Found live: `init` hit this the moment it first tried to store the asset. |
| `M14.37` | **The backfill pages by ROWID keyset.** The fold source column is a `CLOB`, which can be neither `DISTINCT`ed nor `=`-compared (`ORA-00932`/`ORA-22848`), so the values-based loop the other five ports share is illegal here. Batches select `WHERE dst IS NULL` ordered by `ROWID`, update by `ROWID`, and commit per batch; an empty fold result is skipped because `''` is NULL on this engine (`M14.29a`) and writing one would leave the row eligible forever. |

**Mutation-verified** (`T11.10`): with the backfill call inside `upgrade`
disabled, `rows_written_before_the_folded_column_are_backfilled` fails — the
seeded patient "Ámélie" becomes unfindable by `amelie`, which is literally
this finding — and the suite is green again with it restored. As on every
other port, an install predating the stored map asset is refused by name
rather than guessed at: the finding is closed going forward, not
retroactively.

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
(2026-07-27)`, and [`agents/release.md`](../../agents/release.md) asserted "All
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
   exactly how `fhir-r1`'s README drift and five changed `license` lines
   survived.
2. **`fhir/`'s job omits five published crates** — `fhir-r1`, `-7`,
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
The reason it survived is the one `agents/release.md` names: every local build
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
`0.0.0`: `fhir-r1`'s `README.md` had gained a "What is actually
available" section not in the published copy, and all five reservation crates
had a `license` line changed by the quintuple harmonization earlier the same
day. Every one is the same defect — a changed tree on a published number.

Versions bumped so the tree stops claiming numbers it no longer matches:
`fhir-derive-macros` to `1.2.0` (behaviour added, nothing altered) with its six
dependency pins, and `fhir-r1`, `-7`, `-8`, `-9`, `-10` to `0.0.1`. The
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

**Corrections (2026-08-10, found preparing step 3 by reading the emitted DDL
and shredder rather than this table).** Three claims above and in the schedule
are stale or wrong:

- **`v_kind` is bounded everywhere already** — `char(1)`/`CHAR(1)` on
  postgresql/mysql/mariadb/mssql, `CHAR(1 CHAR)` on oracle (fixed by
  **F-08**'s rebuild after this table was written), `TEXT` on sqlite where
  `TEXT` indexes fine. The `v_kind` half of this finding is moot; no port
  converts anything.
- **`path` has no adjunct columns on any port** (the ext/deep adjunct set is
  `url`/`v_text`/`leaf`), so step 4's "drop `path`'s adjunct columns" has
  nothing to drop.
- **`path` is not statically bounded.** The attach path accumulates through
  cyclic contentReference recursion — measured: shredding a
  `QuestionnaireResponse` with items nested five deep stores an ext row with
  `path = "item.item.item.item.item"`, one segment per level, no FHIR depth
  limit. `U12a` was amended same day: `path_bound` is a **declared capacity
  limit** (computed with each cycle traversed at most eight times, enforced
  loudly at shred time), precedented by the model's own 32,767-entry `ords`
  limit and oracle's `RAW(255)` ords image (`M14.13`). The finding's real
  surface is exactly `path` on mssql and oracle.

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
longest one seen in R3–R5. (The second question is answered as of step 2:
it is the longest seen, made a bound by `U12a` — recorded in the asset with
64-char-step headroom, enforced at shred time, never narrowed by `upgrade`.)

**Migration schedule (2026-08-09, owner-directed).** Sequenced; each step is
one session-sized commit, and nothing later starts before the step before it
is live-verified:

1. **Prerequisite: `fhir-oracle` gains `upgrade`/`backfill_norm`** — the
   F-15 remainder. Every later step assumes a working upgrade path on all
   six ports, and oracle is the port with the harder half of this migration.
   **Done 2026-08-09**: 9 live tests, mutation-checked, three new annex
   requirements (`M14.35`–`M14.37`) — the account is in F-15's entry.
2. **The bound decision, as spec text.** The generator's maximum path length
   is the longest seen in R3–R6, not a bound; the migration needs a *bound*.
   Amend `U12` (and `M14` annexes where the type is named): `path` binds to
   a map-level `path_bound` — the longest path in that release's map,
   rounded up with headroom and **recorded in the map asset**, so `G2.2`
   determinism holds and a future release cannot silently shrink it.
   `v_kind` binds to one character everywhere (only oracle's `CLOB` is
   wrong today).
   **Done 2026-08-09**: `U12a` defines `path_bound` — longest attach path
   in the release map, rounded up to the next multiple of 64, floor 128,
   recorded in the asset; widening additive, narrowing refused, an
   over-bound shred fails loudly — and fixes `v_kind` at one character
   (`z`/`b`/`n`/`s` are the only values the core writes). Target bindings:
   `M14.37` (mssql, one transactional conversion) and `M14.38` (oracle,
   add-copy-drop-rename with the half-applied story stated). Grounding:
   the longest fully qualified element path measured across the bundled
   maps is 131 chars (R4/R5), 121 (R3).
   **Amended 2026-08-10** (see the corrections block above): the bound is
   a declared capacity limit — cyclic contentReference recursion grows
   `path` per nesting level, so `U12a` now computes with cycles capped at
   eight traversals and shred refuses past the bound; `v_kind` is out of
   the migration entirely (bounded everywhere already); there are no
   `path` adjuncts to drop in step 4.
3. **The shared-core change, all six ports in one commit** (`X15.1`):
   `model.rs` carries the bound, `gen` computes it, `create_table`'s
   hardcoded `path`/`v_kind` arms become map-driven; assets regenerated in
   all six (`regen-assets`, content-compared per F-41). Physical DDL
   changes only on mssql (`NVARCHAR(MAX)` → `NVARCHAR(bound)`) and oracle
   (`CLOB` → `VARCHAR2(bound CHAR)`; the `v_kind` conversion listed here
   earlier is moot, 2026-08-10 correction); the other four keep `TEXT`
   and carry only the asset-version diff through `upgrade`.
   **Done 2026-08-10**: `ResourceMap.path_bound` (serde-defaulted, so old
   stored assets decode as `0` = legacy), `record_path_bound` in the
   shared gen (cycle cap 8, round-to-64, floor 128), shred refuses an
   over-bound attach path by name ("declared capacity"), and mssql/oracle
   `create_table` emit `NVARCHAR(path_bound)`/`VARCHAR2(path_bound CHAR)`
   when the asset records a bound, the legacy types when it does not
   (`G2.2`: the schema follows the asset). Assets regenerated in all six:
   the recorded bounds are **192 (R3), 192 (R4), 384 (R5)** — R5's deeper
   cyclic structures cost the extra step. Verified: shared core identical
   (105 files), every port's workspace green including the full 7,399-
   example corpus round-trips (no real example is refused), the new shared
   `path_bound.rs` suite in all six (bound shape + eight-levels-fit +
   loud over-bound refusal), oracle's live suite 80/80 with fresh installs
   now `VARCHAR2(384 CHAR)`, and `fhir-loco` green over the new sqlite
   asset. Fresh installs on mssql/oracle are bounded as of this step;
   converting *existing* installs is exactly steps 4–5.
4. **`fhir-mssql` upgrade path, live-verified**: pre-check
   `MAX(LEN(path))` against the bound, `ALTER COLUMN` inside the
   transactional upgrade (`M14.35`). (The "add the index / drop the
   adjuncts" tail this step first carried is amended, 2026-08-10: no
   search filters `path` yet so the index MAY wait, and `path` never had
   adjuncts to drop.)
   **Done 2026-08-10**: `convert_path_columns`, catalog-driven (what the
   deployment actually has, not what the asset says), inside the upgrade
   transaction. `MAX`→bound converts after a data pre-check that refuses,
   naming rows and longest length, if anything stored exceeds the bound;
   widening a bounded column is additive; narrowing one refuses (`U12a`:
   a recorded bound never shrinks in place); conversions count in
   `UpgradeReport.additive`. Live-verified against `azure-sql-edge`:
   `tests/upgrade.rs` grew 9→12 (pre-U12a install converted with data
   surviving and a second pass converting zero — which doubles as the
   proof the first pass was real; over-bound row refuses, rolls the whole
   upgrade back, and succeeds after cleanup; narrowing refuses), full
   port live suite 36/36 serial, mutation-checked (`T11.10`: disabling
   the conversion call fails the pre-U12a test).
5. **`fhir-oracle` conversion path, live-verified**: Oracle cannot alter
   `CLOB` to `VARCHAR2` in place — add-column, copy, drop, rename, each
   statement autocommitting (no transactional DDL, `M14`), with the
   half-applied-upgrade story the annex must state before the code exists.
   **Done 2026-08-10**: `convert_path_columns`, a catalog-driven state
   machine per table (`user_tab_columns` says which prefix of the sequence
   already ran; a rerun finishes the rest) — data pre-check refusing named
   over-bound rows, add, resumable copy, drop, rename; widening additive,
   narrowing refused; the replacement column nullable (`M14.39` — designing
   this step surfaced **F-85**, root-level extensions refused outright,
   fixed the same day). Live-verified: `tests/upgrade.rs` 9→12 including a
   real partial-failure rerun (the refusal leaves earlier tables
   converted, autocommitted, and the retry completes the rest — the
   resumability the annex promised), plus `tests/root_extension.rs`; full
   port live suite 84/84 serial; mutation-checked (`T11.10`: stubbing the
   conversion call fails the pre-U12a test).
6. **Close**: `U12` cells flip in the matrix, per-port `tasks.md` entries
   tick with their live evidence, this finding closes.
   **Done 2026-08-10.** The matrix carries a `U12`/`U12a` row, `•` on all
   six: the four `TEXT` ports satisfy it natively, mssql and oracle by the
   step 3–5 work, every claim live-verified. Both port `tasks.md` entries
   are ticked with their evidence. **The finding is closed** — the whole
   schedule ran 2026-08-09 → 2026-08-10, and its one unplanned dividend is
   **F-85**, found and fixed inside step 5.

Steps 1, 4 and 5 need live engines; 2 and 3 do not. The four stores that
never had the defect carry only step 3's asset bump — the price `X15.1`
sets for moving all six together.

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

**It will not do anything until the repository catches up** — that was true
when written, and stopped being true with commit `60bfcbe` ("Commit the
repository gates, so they can run"): `scripts/`, `doc/`, `LICENSE.md`,
`index.md`, and `spec/publishing.md` are all tracked now, the tree is clean,
and `gates.yml` runs the shared-core and doc-example checks on every push and
pull request. `W16.6` is satisfied for `X15.1`. *(Disposition updated
2026-08-06, **F-73** — the paragraph above described the pre-commit state in
the present tense long after it had changed.)*

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

**Fixed 2026-08-29.** The "real decision" this finding described turned out
to already be decided, by evidence this repository had produced itself and
not connected to this finding: **F-68**, four items later in this same
register, already proved `fhir-oracle-store` connecting live via the `oracle`
crate and Oracle Instant Client — option (a) above, chosen and working,
months before this finding was revisited. There was no remaining
architectural question, only the mechanical work of giving the *map* crate
(which had never depended on a driver, unlike the store) its own copy of
that same proven dependency.

Added `oracle` as a dev-dependency of `fhir-oracle-map` and
`tests/oracle_ddl.rs`, on the model `mssql_ddl.rs` set: install a sampled
schema (`Patient`, `Observation`), assert the statement count, apply every
statement, and count `USER_TABLES`/`USER_TRIGGERS` afterward rather than
trust a lack of errors. **One genuine Oracle-specific complication the SQL
Server model does not have:** Oracle unifies user and schema (`M14.5`), so
"install into a fresh schema" means "create a fresh database user" — a
SYSTEM-level privilege no regular test login holds. The test therefore
connects twice, as SYSTEM to provision a throwaway `DDLTEST` user and then
as that user to install and verify, mirroring in Rust exactly what this
port's own `scripts/db.sh` (`post_ready`) and `fhir-oracle-ci.yml` ("Create
the version users") already do in shell for the `R3`/`R4`/`R5` users the
*store's* tests use — a different, dedicated user, so this test cannot
collide with those.

**Live-verified before being trusted, not after:** run twice consecutively
against a real `gvenzl/oracle-free` container to confirm the drop-and-recreate
cleanup is not flaky the way `mssql_ddl.rs`'s own history warns it can be (166
statements, 105 tables, 2 triggers, identically, both times); the skip path
confirmed silent without `FHIR_ORACLE_REQUIRE_DB`; the fail-loud path
confirmed to panic rather than skip with it set, against a genuinely
unreachable connect string; `--release` and `cargo clippy -- -D warnings`
both clean. Wired into `fhir-oracle-ci.yml`'s existing live-database job,
ahead of the version-user creation it does not depend on.

**What this does and does not establish.** The port's Schema-level claim
(`C0.8`, `C0.9`) is now justified by a test that runs, not a transcript —
that is what this finding asked for, and it is done. **Not established:**
the full ~9,636-statement R5 install remains verified only by the hand-run
transcript **F-08** left behind (this test samples two resource types, the
same trade-off `mssql_ddl.rs` makes, for speed rather than exhaustiveness);
and the two hand-only behaviours named above — the append-only trigger
actually *refusing* a forbidden `UPDATE`/`DELETE` (`M14.29`), and the `Bool`
CHECK actually *rejecting* `2` (`M14.8`) — are untouched by this fix. This
test counts that triggers exist; it does not exercise what they do. Checked,
not assumed: neither behaviour appears anywhere in `fhir-oracle-store`'s own
live suite either — `grep -rl "M14.29\|M14.8" fhir-oracle-store/tests/`
finds nothing. **Filed and fixed the same day as F-97.**

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
a **local** gate is useful here without CI — `agents/release.md` step 0c invokes
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

**Status text across `doc/` and `agents/` had decayed past the point of being
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
| `agents/release.md` | "one port blocked on a High finding: `fhir-oracle` has **F-08**" | **F-08** is fixed. No port has an open High finding of its own — but **every** port is blocked by **F-49**, which that paragraph did not mention |

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


## F-64

**Every non-PostgreSQL port's `doc/benchmarks.md` presented `fhir-postgresql`'s
measured numbers as its own — load throughput, read latency, and a gated
benchmark test that does not exist.** Severity: **High**. Violates `W16.10`
directly: "Measured numbers MUST name what measured them and when. A
throughput figure inherited by substitution is not a measurement of the port
that now carries it." `T11.5`'s regression gate is also unmet in five ports,
since none has the `bench.rs` it would compare against. *Found checking whether
`doc/benchmarks.md` existed after **F-61** (`plan.md`) referenced it from all
six ports.*

`W16.10` is worth pausing on: it names this exact failure and was already in
the specification. `agents/testing.md` even quotes it verbatim. The requirement
being written down did not stop it from happening, because nothing checked
`doc/benchmarks.md` against it — the same gap **F-60** found in the doc
examples one document over.

All five non-PostgreSQL files were `fhir-postgresql`'s, with the crate name
substituted in exactly **two of five** places — the same partial-substitution
signature as every other file in this family of findings (F-01, F-27, F-53,
F-56, F-61, F-62). The other three were untouched:

```
- `fhir-mssql init` for full R5 (7,355 tables + 9,168 indexes...): 5.8-9.5 s,
  staged-schema install (see spec G2.5)...
- Live PostgreSQL put→get round trip of the same corpus: 7,396/7,396 lossless...
  101 s total... roughly 13 ms per resource...
Gated benchmark: `FHIR_MSSQL_BENCH=100000 FHIR_MSSQL_TEST_DB=… cargo test
--release -p fhir-mssql-store --test bench -- --nocapture`.
- Load: 100,000 resources... in 16.3 s — 6,146 resources/s...
- Read: 1.18 ms average...
```

**`fhir-mssql` has no store and no driver.** Neither does `fhir-oracle`. Both
files claimed a live put/get round trip, a 16.3s bulk load at 6,146
resources/s, and a 1.18ms average read — specific, decimal-precision numbers,
for an operation that has never once been performed in either port.

**`fhir-sqlite`, `fhir-mysql` and `fhir-mariadb` do have stores**, and the
numbers were still not theirs. Two defects, verified rather than assumed:

1. `bench.rs` — the file the "Gated benchmark" line invokes — exists in
   **`fhir-postgresql` only**. The other five's invocation,
   `FHIR_<PORT>_BENCH=100000 … --test bench`, would fail with "no test target
   named `bench`".
2. "staged-schema install" contradicts **F-27**, which already established that
   none of the three actually stages a schema — each applies statements
   directly, and the phrase survived in this file because F-27's correction
   was scoped to `tasks.md` and never re-checked here.

**Fixed by correcting, not deleting.** What is genuinely shared — the schema
scale table and the search-compilation percentage, both derived from the
generator that is byte-identical across all six ports (`X15.1`) — is kept.
Everything about install timing, live round-trip, and bulk load is now stated
as **not measured for this port**, with the borrowed number named as
`fhir-postgresql`'s rather than silently deleted, so a future reader does not
wonder whether the section was always empty.

`fhir-oracle`'s file additionally gained real numbers it didn't have before:
the **actual** F-08 install result — 9,636 statements, 7,358 tables, 0 invalid
objects — replacing the fictional one.

**Not fixed: no benchmark harness was built.** Measuring real load/read/index
numbers for `fhir-sqlite`, `fhir-mysql` and `fhir-mariadb` is real engineering
work — a `bench.rs` per port — and is recorded as a gap rather than attempted
here, for the same reason **F-60** didn't build a doc-compile CI job in the
same pass it fixed the examples: doing it properly is a separate piece of work
from stopping the bleeding.

## F-65

**`fhir-mssql` gained a real store this pass** — `connect`, `init`, `put`,
`get`, `delete`, `history`, `vread`, `verify_audit`, `purge`, `log_access`,
`search`/`search_full`/`search_page` — live-verified against `azure-sql-edge`
by a new `mssql_store.rs`, `concurrency.rs`, `redaction.rs`, and
`roundtrip_types.rs`. Five defects surfaced by running that work live; all
five are now fixed, the fifth (`R4.5`) in a follow-up pass — see the amendment
below. Recorded here per `W16.10`, the same reason **F-64** records numbers
rather than assuming implementation once existed means it was correct.

**Fixed:**

1. **Cross-column collation conflict (468) in any chained reference search.**
   `ColTy::Text` (`*_ref_id`, `*_ref_type`, `*_ref_url`) rendered as
   `NVARCHAR(MAX)` with the *database* default collation, while `id`/`rid`
   (`ColTy::TextC`) are always `Latin1_General_100_BIN2`. Comparing them —
   exactly what a `subject:Patient.family=…` chain does — made SQL Server
   refuse the query outright: "Cannot resolve the collation conflict between
   SQL_Latin1_General_CP1_CI_AS and Latin1_General_100_BIN2." Fixed by giving
   `Text`, `Numeric`, and `Jsonb` the same `BIN2` collation as `TextC` in
   `ddl.rs`'s `col_sql`, so every unbounded column in the schema compares
   consistently rather than three columns being special-cased.
2. **`verify_audit` never read `row_mac` back.** The column was written on
   every insert and never checked, so a keyed tag being wrong reported no
   `hmac-sha256` break at all — confirmed live: tampering an `actor` column
   correctly broke both hash chains but the HMAC check was silently absent.
   Ported the MySQL store's `check_mac`/countersign-table logic verbatim
   (`fhir_store::chain::{KeyRing, MacCheck}` is shared).
3. **`connect` returned `Ok` for an unreachable server.** `bb8::Builder::build`
   does not itself open a connection; a bad DSN (dead port) only failed on
   first use. Fixed by borrowing and releasing one connection inside
   `connect_pool` before returning.
4. **`purge`'s `versions_erased` was double the true count.** `INSTEAD OF
   DELETE` (`M14.19`) issues its own nested `DELETE`, and
   `ExecuteResult::total()` summed both statements' `DONE` tokens — measured
   live as `6` for `3` real history rows. Fixed by `SELECT COUNT(*)` before
   the delete instead of trusting the post-delete row count.

**Originally reported not fixed — `R4.5` (snapshot reads) violated under
concurrent writers,** confirmed by a live torn read: a reader observed one
write's `active` value alongside a different write's `name`/`telecom`. `get`
wraps its multi-table read in `BEGIN`/`ROLLBACK TRANSACTION`, which is not
snapshot isolation — this engine's default `READ COMMITTED` gives each
statement inside that transaction the latest committed data as of when *it*
runs, not one instant for the whole transaction, unlike PostgreSQL's or
MySQL's default. `M14.25` already named this "undecided" before any store
existed; running the reproduction (`tests/concurrency.rs`,
`reads_never_tear_under_concurrent_writes`) turned it into a confirmed
violation.

Closing it needed a database this port could run `ALTER DATABASE` against —
every DSN used so far, including `scripts/db.sh`'s `dsn_line`, omitted
`database=` and landed in `master`, which SQL Server refuses the option on
outright: `Option 'READ_COMMITTED_SNAPSHOT' cannot be set in database
'master'.` That was a provisioning gap, not a code change, so it was recorded
here rather than patched around in the same pass.

**Amendment, same day: fixed, in a follow-up pass, after the first fix
attempt was tried live and found insufficient.** `scripts/db.sh`'s `post_ready`
now creates a dedicated `fhir_mssql` database on container start-up, before
any pooled connection exists (so `ALTER DATABASE` never has to wait out an
active transaction), and `dsn_line` targets it with `database=fhir_mssql`.

The obvious fix — enable `READ_COMMITTED_SNAPSHOT` alone — was tried first and
**did not** stop the torn read when run against the live reproduction: RCSI
gives each individual *statement* inside a `READ COMMITTED` transaction its
own snapshot, not the whole *transaction* one shared snapshot, which is what
`get`'s multi-statement read actually needs. The fix that measurably works —
confirmed by the same test passing five consecutive runs after the change,
having failed every run before it — is `get` issuing `SET TRANSACTION
ISOLATION LEVEL SNAPSHOT` immediately before its `BEGIN TRANSACTION`, backed
by `ALLOW_SNAPSHOT_ISOLATION` enabled on the dedicated database. Because
`SET TRANSACTION ISOLATION LEVEL` is session-scoped rather than
transaction-scoped, `get` resets it back to `READ COMMITTED` before releasing
the connection to the pool, on every exit path — the same leak discipline
`purge`'s `SESSION_CONTEXT` erasure flag already needed, for the same reason.

The `#[ignore]` was removed from `reads_never_tear_under_concurrent_writes`
once it passed; the full live suite (23 tests) is green with none ignored.
Both the "RCSI alone is insufficient" finding and the working fix are
reproducible: `scripts/db.sh up` provisions the database correctly from a
clean state, and the test fails again if `SET TRANSACTION ISOLATION LEVEL
SNAPSHOT` is removed from `get` without removing `ALLOW_SNAPSHOT_ISOLATION`
too.

**One more pool-safety gap closed in passing, not itself live-reproduced.**
`get`'s original shape returned early via `?` from inside its per-table read
loop on any query error, skipping the rollback and (now) the isolation-level
reset — an open transaction, `SNAPSHOT`-isolated, would have gone back to the
pool for an unrelated later caller to inherit. The read loop is now a
separate function (`read_resource_rows`) whose `Result` is captured before
the rollback/reset run unconditionally, on the error path as well as the
success one. No live reproduction exists for this one specifically — it
would need a query mid-loop to fail, which nothing in this pass's test suite
forces — so it is recorded as a defect found by reasoning about the new
isolation-level state this fix introduced, not by running something and
watching it fail.

`scripts/db.sh`'s readiness check was also found broken while doing this
work, unrelated to `R4.5` itself: `ready`/`post_ready` used to shell out to
`sqlcmd` inside the container, and the `azure-sql-edge` image this port uses
locally on arm64 ships no client tools at all — neither `/opt/mssql-tools` nor
`/opt/mssql-tools18` exists in it. `wait_ready` therefore timed out and `up`
exited nonzero on **every** local run, masked because the invocations used to
verify this session's earlier work all piped `db.sh up`'s output through
something like `tail`, which swallows the exit code. Fixed by replacing the
`sqlcmd` calls with a tiny ephemeral `tiberius` probe generated into
`target/` (mirroring the sqlite branch's `$WRAPPER` pattern, not a workspace
member). `scripts/db.sh client` still shells out to `sqlcmd` and is not
fixed — nothing in the test suite calls it.

*Found live-testing the search and test-suite work this pass; fixed in a
follow-up pass the same day. See `spec/14-mssql-dialect.md` `M14.25` for the
fuller account.*

## F-66

**`fhir-oracle-store` now contains a full store implementation —
`connect`, `init`, `put`, `get`, `delete`, `history`, `vread`, `verify_audit`,
`purge`, `log_access`, `search`/`search_full`/`search_page`, plus a search
builder (`oracle_search.rs`) and a connection pool (`pool.rs`) — written with
**no Oracle Instant Client available on the build host**, and it must not be
read as evidence of anything beyond what is stated below. Severity: **N/A**
(a scope note, not a defect) — recorded per `W16.10` for the same reason
**F-64** and **F-65** record what was and was not verified, and because a
future reader finding ~2,000 lines of Oracle store code with no test result
attached needs to know why before trusting any of it.

**Context: the user's explicit choice.** Mid-session, asked how to handle
Oracle given the missing driver, the options offered were (1) skip Oracle and
leave it at Scaffold, (2) write the store anyway with no way to compile or
run it, or (3) have the user install Instant Client first. The user chose
(2) — write it anyway, unverified — over the recommended (1), after an
explicit warning that it "will very likely fail to build here... and cannot
be tested against the live oracle-probe container from this environment even
if it did build," and that doing so "contradicts this session's own
standard: every claim verified by running something." That warning's premise
turned out to be half wrong, in the more favorable direction — see below —
but the user's choice to proceed was made before that was known, on the
information available at the time.

**The premise — "this will not even compile" — was checked by running it,
and was wrong.** `cargo check -p fhir-oracle-store` and
`cargo build -p fhir-oracle-store` both **succeed** with no Instant Client
installed. The `oracle` crate wraps ODPI-C, which loads `libclntsh` via
`dlopen` at *connection* time, not link time, so nothing before an actual
`Connection::connect` or `Pool::get` call needs the library at all. A minimal
probe built against the same `oracle 0.6.3` confirms exactly where the real
wall is:

```
error: DPI Error: DPI-1047: Cannot locate a 64-bit Oracle Client library:
"dlopen(libclntsh.dylib, ...)" ... See https://oracle.github.io/odpi/doc/installation.html#macos for help
```

So the accurate claim is narrower and stronger than "written but never
compiled": **this crate compiles and type-checks against the real shared
core and the real `oracle` crate API, and has never once connected to a
database.** Every doc comment written before this was discovered — and there
were several, across `lib.rs`, `oracle.rs`, `oracle_search.rs`, `pool.rs`,
and both `Cargo.toml` files — originally said "never compiled, never run"
and was corrected in the same pass, once running `cargo build` made the
original claim false. Left uncorrected, it would have been exactly the kind
of confident-but-wrong text this project's audit exists to catch, just
authored by this session instead of an earlier one.

**What a clean compile is evidence of, and what it is not.** It confirms:
every SQL string is at least well-formed Rust (no argument-count or
mismatched-type errors), every column index and type dispatch in `cell_text`
matches the map's own `ColTy`, and the calls into `shred.rs`/`reconstruct.rs`/
`canon.rs`/`value.rs` — the shared core, identical across all six ports
(`X15.1`) — use their *real* signatures rather than guessed ones. Getting
those signatures right took several rounds of reading `fhir-mssql-store`'s
actual (live-verified) source and the shared modules directly, because the
first draft guessed wrong on `ReconIn`'s shape, `LeafVal::from_cols`'s
signature, `canon::canonicalize` vs. a guessed `canon::canon`, and
`ShredError`'s variants — all caught by the compiler, none of which a
"written but never compiled" file would have had the chance to catch.

It confirms **nothing** about: whether any SQL statement is syntactically
valid *Oracle* SQL (`ddl.rs`'s own history includes `ORA-02438`, an
empty-string-is-NULL trigger that failed open, and 453 unindexable targets
that were shared-core defects — all found only by executing DDL, never by
reading it); whether the `oracle` crate's actual runtime behavior matches
this file's assumptions about it (the `Bound`/`ToSql`/`FromSql` NULL-typing
scheme in particular, adapted from a defect class **found live** in
`fhir-mssql-store` — see **F-65** — but never itself exercised here); whether
`:1`-style positional placeholders bind in the order this code assumes;
whether `SET TRANSACTION READ ONLY` actually gives `get`'s multi-table read
one snapshot, the same unverified claim `fhir-mssql`'s analogous comment
turned out to be wrong about (**F-65**) until a live test proved it one way;
or whether the `CLIENT_INFO` erasure escape hatch works at all, given that
its SQL Server analogue's first version **failed open** (`M14.29a`) and was
only caught by executing a forbidden `DELETE`, not by reading the trigger.

**Known, stated gaps within the unverified code itself**, beyond "nothing
has run":

1. `oracle_search.rs`'s `U6` confirming comparison (`DBMS_LOB.COMPARE`
   against the source `CLOB` after a digest-adjunct hit) is not implemented
   — a digest collision would go uncaught. Stated in that file's module doc
   rather than silently omitted.
2. Booleans are bound as `i64` 0/1 rather than the `oracle` crate's native
   `bool` binding, because that binding targets Oracle's `BOOLEAN` type
   (23ai+) while this port's schema uses `NUMBER(1)` at the 12.2 floor
   (`M14.2`, `M14.4`) — reasoned from reading the driver source, not
   confirmed by sending one.
3. `drop_schema` queries `user_constraints` assuming the connecting user owns
   exactly this schema's objects (per `M14.5`'s "three users, one per
   version" decision) — untested against an actual multi-user Oracle
   install.

**Conformance impact: none, deliberately.** `fhir-oracle`'s level stays
**Scaffold** (`C0.8`) — unchanged by this finding. `C0.9` requires a level be
justified by tests that ran; a clean `cargo build` is real evidence of a
narrower claim (the code matches real APIs) and not the claim `C0.9` is
about. `fhir-oracle-map/src/ddl.rs` remains the only thing in this port
verified against a live Oracle (**F-08**), and `mod oracle`/`mod
oracle_search`/`mod pool` in `fhir-oracle-store` MUST NOT be cited as
evidence of anything until someone with Instant Client available compiles,
connects, and runs the live suite the way `fhir-mssql-store` and
`fhir-mysql-store` already have.

*Found writing and then attempting to verify the Oracle store this pass, at
the user's explicit direction after being warned it would likely not even
compile.*

## F-67

**`fhir-mssql`'s `deny.toml` justified ignoring four TLS-library vulnerability
advisories on the grounds that they reached only a dev-dependency — and that
became false the moment this port gained a store, unnoticed until this
finding.** Severity: **High**. `O10.7` is about protecting PHI in transit,
this port stores patient data, and the crate now shipping the vulnerable code
is `fhir-mssql-store` itself.

`RUSTSEC-2025-0134` (`rustls-pemfile` unmaintained) and `RUSTSEC-2026-0098`,
`-0099`, `-0104` (three `rustls-webpki 0.101.7` certificate-handling defects)
were recorded 2026-08-02 with the reasoning "`tiberius` is a dev-dependency of
`fhir-mssql-map`... verified absent from `cargo tree -e normal`... nothing
that ships is affected." That was true when written. `fhir-mssql-store`
(**F-65**) was built two tasks later in the same session and depends on
`tiberius` as a normal dependency; nobody re-ran `cargo tree -e normal`
against the new crate, so `deny.toml`'s comments kept asserting a scope that
no longer held. `cargo tree -p fhir-mssql-store -e normal`, run while
investigating `O10.7` for this finding, shows `rustls-webpki 0.101.7` and
`rustls-pemfile 1.0.4` both present in the shipping dependency tree.

**Two things resolved, one escalated, one remains open:**

1. **Resolved: the verification *mechanism* itself works.**
   `tests/ssl_live.rs`, new this pass, proves live that
   `TrustServerCertificate=false` rejects `azure-sql-edge`'s self-signed
   certificate — deterministically, reproduced across repeated connections —
   while `=true` accepts the identical certificate. The rejection is stricter
   than an ordinary chain-of-trust failure: `rustls-webpki` refuses the
   certificate's X.509 structure outright (`invalid peer certificate:
   Other(UnsupportedCertVersion)`), found by running the connection attempt
   with `RUST_LOG=trace` and reading the handshake.
2. **Resolved (negative result): `native-tls` was tried as the escape `M14.34`
   itself suggested, and does not work here.** Substituted for `rustls` in
   both the real workspace and a standalone probe, it fails the TLS handshake
   outright against this exact server — `Error forming TLS connection:
   connection closed via error` — even under blind trust
   (`TrustConfig::TrustAll`). `cargo tree`/`cargo deny check advisories`
   confirm the vulnerable chain genuinely disappears under `native-tls` (not
   merely hidden), so the finding is specifically "works elsewhere, not on
   this host" — plausibly Apple's deprecated Secure Transport backend
   refusing whatever certificate format `azure-sql-edge` generates, which
   `rustls-webpki` also, separately, cannot fully parse. Reverted; the driver
   is `rustls` again, unchanged from before this investigation.
3. **Escalated, corrected, not fixed: the four advisories now demonstrably
   reach a shipping crate,** contradicting the exception list's own stated
   scope. `deny.toml`'s `ignore` comments are rewritten with the true current
   dependency graph and this history; the advisories remain ignored because
   there is no available fix (confirmed: `tiberius 0.12.3` is still the
   newest release, checked against the crates.io index the same day), not
   because the exposure is now understood to be small.
4. **Decided, 2026-08-28: accept the risk formally, corrected scope on
   record, keep shipping.** Not a placeholder — reached after investigating
   two alternatives first (a from-scratch driver, and a fork of the one
   upstream fix that exists, `prisma/tiberius#419`), both priced and both
   set aside; the full account, with real numbers, is in `M14.34` in
   `fhir-mssql/spec/14-mssql-dialect.md`. What was **not** chosen: finding or
   funding a different TDS driver (priced at 1–2 weeks for a fork plus an
   open-ended maintenance tail, or 3–4.5 months from scratch — and neither
   comes out ahead of the flawed-but-battle-tested incumbent on the trust
   axis), or stating the TLS story unresolved (rejected because the
   trust/no-trust *mechanism* is proven correct, and a blanket "unresolved"
   would understate that).

`O10.7` therefore stays unclaimed for this port, but the claim is now
precisely diagnosed rather than an open question: the trust/no-trust logic is
proven correct, and the certificate-parsing code underneath it is proven to
carry unpatched CVEs — two independent facts, both true at once, and
conflating them (as "just say verification works" would) is exactly the kind
of half-true claim this project's audit process exists to catch. What
changed today is that the residual risk this diagnosis leaves behind is now
an owned decision rather than an open one.

**5. Closed, 2026-08-29 — resolved outright, one day after point 4's risk
acceptance, by an option that did not exist when the acceptance was made.**
The owner published `mssql` (github.com/joelparkerhenderson/mssql-rust), a
fork of `tiberius` maintained specifically to carry forward the security
fixes tiberius itself never shipped past 0.12.3. This is, in substance, the
"fork carrying #419's fix" option `M14.34` priced and set aside — but as
this project's own crate rather than a private or third-party fork, the
open-ended maintenance tail that made that option unattractive is simply
this project's ongoing maintenance surface, not a new liability, and the
"zero maintainer review" problem that disqualified `#419` itself does not
apply to work reviewed as this project's own.

`fhir-mssql`'s `Cargo.toml` now depends on `mssql` (aliased `mssql-driver`,
since `fhir-mssql-store` already has a local module named `mssql`) instead
of `tiberius`, with no other source change needed — the fork kept
`tiberius`'s public API. The resolved chain is `tokio-rustls 0.26.4` →
`rustls 0.23.43` → `rustls-webpki 0.103.15`; `cargo tree` confirms none of
`rustls-webpki 0.101.7`, `rustls-pemfile`, or any of the four advisory
packages remain anywhere in the workspace. `deny.toml`'s ignore list is
empty; `cargo deny check` passes clean. The full live suite —
`mssql_store.rs` (13), `concurrency.rs` (2), `redaction.rs` (2),
`roundtrip_types.rs` (6), `ssl_live.rs` (1), `upgrade.rs` (16),
`mssql_ddl.rs` (1), 41 in all — passes against `azure-sql-edge` under the
new driver (`upgrade.rs` needs `--test-threads=1`; the schema it shares
across tests deadlocks under default parallelism, a pre-existing test-suite
property unrelated to the driver). `O10.7` is now claimed, not merely
diagnosed. **This closes the audit register's last open finding — as of
this entry, no row in this table is open.**

*Found investigating `O10.7` this pass, triggered by writing
`tests/ssl_live.rs` and then checking what the store's own dependency tree
actually contains rather than trusting a two-day-old comment.*

## F-68

**`fhir-oracle-store` has now connected to a live Oracle database, run its
full CRUD/history/search/audit surface against it, and found and fixed four
real defects — superseding F-66's "compiles but has never connected"
premise, which was accurate when written and is now obsolete.** Severity:
**N/A** for this finding itself (a status update, recorded per `W16.10`
because F-66 is cited elsewhere as a reason not to trust this store, and that
reason no longer applies in its original form); the four defects it
describes were each real bugs, three of them High (data written incorrectly
or not at all) and one Medium (a feature — token search on a boolean field —
simply not working).

**What changed: Oracle Instant Client, which F-66 and the annex's `M14.23`
both treated as an open, potentially unresolvable blocker, turned out to be a
direct, no-login download.** `instantclient-basiclite-macos-arm64.dmg` from
`download.oracle.com` — no OTN click-through, no account — mounts and its
dylibs copy straight to `~/lib`, a default ODPI-C search path. Once present,
`gvenzl/oracle-free:23-slim-faststart` (the same arm64 image `M14.23a`
measured booting to "DATABASE IS READY TO USE!" in ~13 seconds) gave this
port its first live database, and the store built in F-66 connected to it on
the first attempt.

**Four defects found by running it, none of them visible from reading the
code:**

1. **Uppercase schema case-folding (`M14.5`).** Oracle folds an *unquoted*
   username to uppercase for session identity — `SELECT USER FROM DUAL`
   returns `"R5"` — regardless of how `CREATE USER` quoted the object name at
   creation. A user created as `CREATE USER "r5" IDENTIFIED BY ...` (quoted,
   to preserve the lowercase spelling `M14.5` assumed) could still log in as
   `"r5"`, but every DDL statement addressed to `"r5".*` then failed
   `ORA-01031: insufficient privileges`, because the session that ran it was
   really `"R5"`. Fixed by creating users **unquoted** (naturally uppercase)
   and setting the map's `schema` field to match, uppercase — the opposite
   convention from every other port's lowercase schema names, and now
   recorded in `M14.5` and `tests/oracle_store.rs`'s `sampled()` helper.
2. **`R4.5`'s presumed answer doesn't work (`M14.19`).** The annex named
   `SET TRANSACTION READ ONLY` as "the likely answer" for snapshot reads,
   unverified. Tried live, it failed every read with
   `ORA-01466: unable to read data - table definition has changed` —
   reproduced independently with a minimal 3-statement probe (`CREATE TABLE`
   + commit, then on the *same session* `SET TRANSACTION READ ONLY` +
   `SELECT`), confirming this is a genuine Oracle behavior (any session that
   has ever executed DDL is poisoned for later read-only or serializable
   transactions) rather than a bug in how the store called it. The call was
   removed rather than shipped broken. **`R4.5` regresses from "an
   unverified but plausible design" to a confirmed, open gap** — `get` now
   has no snapshot-isolation protection at all on this engine. This is the
   opposite direction from `fhir-mssql`'s `R4.5` story (**F-65**, where the
   first attempt was insufficient but a second one worked); here the
   candidate mechanism itself doesn't apply, and no replacement is proposed.
3. **Double schema-qualification (`ORA-00926`).** `insert_row` took a
   separate `schema` argument *and*, at several call sites, an
   already-schema-qualified table string, producing SQL like
   `INSERT INTO "R5"."R5"."patient_history" (...) — Missing VALUES or SET
   keyword`. Fixed by changing `insert_row` to take one pre-qualified
   `target: &str` and fixing all nine call sites; its error message now
   includes the failed SQL text, to make this class of bug faster to
   root-cause next time.
4. **Timestamp/date binding relied on session NLS settings
   (`ORA-01843: An invalid month was specified`).** Values were bound as
   plain strings, leaning on Oracle's implicit string-to-`TIMESTAMP`/`DATE`
   conversion — which uses the session's `NLS_TIMESTAMP_FORMAT`, not ISO
   8601, unlike SQL Server's `DATETIME2` which accepts ISO 8601 regardless of
   session settings. Fixed by adding real `chrono`-typed bind variants
   (`Bound::Timestamp`, `Bound::CalDate`) with an explicit
   `"%Y-%m-%dT%H:%M:%S%.f"` parse.

**A fifth defect surfaced by the checked-in test suite itself, after the
above four were fixed and a real `tests/oracle_store.rs` (not the `/tmp`
scratch harness used to find 1–4) was written and run:**

5. **Token search cannot bind a boolean as text
   (`ORA-01722: unable to convert string value containing 't' to a number`).**
   `search_by_token_and_family_name` searching `active=true` against
   `Patient.active` — `NUMBER(1)`, `ColTy::Bool` — bound the raw string
   `"true"`. Unlike SQL Server/MySQL, Oracle refuses to implicitly convert
   `'true'`/`'false'` to a number. Fixed by adding a `Bind::I64` variant to
   `oracle_search.rs` and a `col_is_bool` lookup so `target_pred`'s `Token`
   branch binds `0`/`1` instead of a string whenever the target column is
   `ColTy::Bool`, threaded through a new `cols` parameter on `target_pred`
   and its sole caller `param_predicate`.

A sixth, cosmetic finding belongs to the test fixture, not the store: the
first run of `put_then_get_round_trips_a_resource` reported `"9.60"`
round-tripping as `"9.6"`, which looked like the `M3.6` decimal-precision
defect this whole test exists to catch. It was the test's own fixture: built
with `serde_json::json!`, whose `9.60` is collapsed to an `f64` literal by
Rust's own parser before `serde_json` ever sees text — the exact trap
`fhir-mssql-store`'s test file already documents avoiding. Fixed by building
the fixture with `serde_json::from_str` instead. Recorded here because it is
the second time in this session a live test's first failure was misdiagnosed
against the store before the fixture was found guilty (`fhir-mssql`'s
`ssl_live.rs` had a milder version, misreading a `bb8` timeout as a
certificate error) — worth a general note that a live test's first failure
should be traced to its actual source before it is written down as a store
defect.

**Conformance impact.** `fhir-oracle` moves from **Scaffold** to **Store**
(`C0.8`, `C0.9`): a real, checked-in `tests/oracle_store.rs` (7 tests, `T11.2`)
now runs against a real `scripts/db.sh`-managed Oracle instance and passes
green with 0 ignored, replacing the `/tmp/oraclelive` and `/tmp/oracle-probe`
scratch harnesses used to find defects 1–4 as the basis for any claim.
`R4.5` is now `!` rather than `—` in the [conformance
matrix](conformance-matrix.md) — a confirmed gap, not merely an untested
one, and MUST be treated as one until `M14.19` names a working mechanism.
`H5.4` is `?`: `SELECT … FOR UPDATE` is in the code (`M14.20` discharged),
but no `concurrency.rs` exists yet for this port to prove it holds under
contention. See `fhir-oracle/spec/14-oracle-dialect.md` for the annex
updates this finding requires (`M14.5`, `M14.19`, `M14.20`) and
`fhir-oracle/CLAUDE.md`/`AGENTS.md`/`README.md`/`tasks.md` for the
corresponding documentation pass.

*Found installing Oracle Instant Client and running the F-66 store live for
the first time this pass, then writing a real test suite for it and running
that too.*

## F-69

**`scripts/db.sh up` silently exits 1 with zero output — no stdout, no
stderr, no diagnostic of any kind — the first time it runs against a fresh
`target/` directory, in all six ports.** Severity: **Medium**. It is not a
data-safety defect, but it is exactly the class this project's audit exists
to catch: the documented, agent-facing workflow (`scripts/db.sh up`, then
`scripts/db.sh test`) simply does not work the first time, and fails in the
one way hardest to diagnose — total silence.

**Root cause.** `spec_exports()`, present verbatim in all six `scripts/
db.sh` (F-39's fix touched this same function for a related but different
defect and did not introduce this one — it predates F-39):

```sh
spec_exports() {
  local spec
  if spec="$(find_spec)"; then
    echo "export $SPEC_ENV=$spec"
    [ -d "$CORPUS_DIR" ] && echo "export $CORPUS_ENV=$CORPUS_DIR"
  fi
}
```

When `find_spec` succeeds (the FHIR spec checkout is found — true in this
repository) but `$CORPUS_DIR` (`target/test-corpus`, the symlink tree
`scripts/db.sh corpus` lays out) does not yet exist, `[ -d "$CORPUS_DIR" ]`
is false and `&&` short-circuits, making that line — the last one
executed — return non-zero. Bash functions return the exit status of their
last command, so `spec_exports` itself returns 1.

Every `up()` calls `spec_exports` as its own unguarded last statement:

```sh
  echo "to use it in this shell:"
  dsn_line
  spec_exports
}
```

So `up()` also returns 1. Under this script's `set -euo pipefail`, any
caller that invokes `up` as a bare statement — the top-level dispatcher
(`up) up ;;`) and `run_tests`'s `up >/dev/null` alike — dies immediately on
that non-zero status. Because the failing test produces no output of its own
and `set -e` prints no message either, the visible result is: every `export
FHIR_*` line prints correctly, and then the process exits 1 with nothing
further — not even from `run_tests`, which never gets far enough to reach
`corpus_ok`/`corpus`/`cargo test`.

**Reproduced live, in two ports, from the actual current repository
state** (not a constructed scenario): neither `fhir-oracle/target/
test-corpus` nor `fhir-mssql/target/test-corpus` currently exists, so
`DYLD_LIBRARY_PATH=~/lib ./scripts/db.sh up` in `fhir-oracle` — and, by the
same code path, `./scripts/db.sh up` in `fhir-mssql` — both exit 1 despite
printing a complete, correct `export FHIR_*_TEST_*=...` block immediately
before doing so. `bash -x` confirms no command after the final `echo`
produces any output before the script terminates.

This is why it was found now rather than earlier in this session: `fhir-
mssql`'s live suite was mostly exercised via `cargo test` directly, with the
DSN exported by hand, not through `scripts/db.sh test`'s own exit code.
`fhir-oracle`'s **F-68** work was the first time this session actually
relied on `scripts/db.sh test` end-to-end and checked whether it succeeded.

**Fixed** by adding an explicit `return 0` after the `if` block in
`spec_exports`, in all six `scripts/db.sh` — the function is purely
informational (it prints `export` lines for a human to eval, or for
`eval "$(spec_exports)"` to consume) and was never meant to signal failure
to its caller; the missing corpus directory is an expected, common state
`run_tests`'s own `corpus_ok || corpus` a few lines later exists to handle,
not a fatal error one function-call earlier.

**Verified.** `fhir-oracle`: `scripts/db.sh test -p fhir-oracle-store --test
oracle_store -- --test-threads=1` against a `target/` with no
`test-corpus` directory now runs to completion — 7 of 7 tests pass, exit 0
— where it previously exited 1 with no output at all. `bash -n` confirms all
six edited scripts remain syntactically valid.

**Conformance impact: none directly** — this is a developer-workflow defect
in a script `C0.9` does not itself require, not a change to any store's
behaviour. It does mean that any conformance claim in this session
established by *running* `scripts/db.sh test` (rather than `cargo test`
directly with hand-exported variables) should be re-read as "worked after
this fix", not "always worked" — none currently rests on the broken window,
since the defect always killed the run before any test could execute, and a
killed run cannot itself have produced a false positive.

*Found running `fhir-oracle`'s `scripts/db.sh test` for the first time this
pass, to verify the exact command this pass's documentation updates were
about to recommend actually works — it did not, silently, until this fix.*

## F-70

**`fhir_store::chain::ChainKey::from_env()`/`KeyRing::from_env()` — shared
code (`fhir-store`, one crate every port depends on, not six copies)
hardcoded the literal variable names `FHIR_SQLITE_CHAIN_KEY`/`_ID`/
`_RETIRED`, regardless of which port called them.** Severity: **Medium**.
Setting `FHIR_POSTGRESQL_CHAIN_KEY` compiled, looked correct, and did
nothing: `from_env()` found no `FHIR_SQLITE_CHAIN_KEY`, returned an empty
`KeyRing`, and every history row signed with a bare SHA-256/SHA-3-256 hash
instead of the intended HMAC — silently weaker tamper-evidence than a
deployment believed it configured (`PR12.x`, the guarantee `with_chain_keys`
exists to provide).

**Blast radius, found independently by two documentation-verification
agents in the same pass** (the `fhir-postgresql` and `fhir-mariadb`
book-rewrite agents launched this session, working from separate source
trees with no shared context): every port's own `CHANGELOG.md` and
`doc/containers.md` — including `fhir-sqlite`'s own — documented a
port-specific name as if it "enabled the keyed-MAC audit tests". Checking
which ports actually call `from_env()` at all turned up a narrower truth:
only `fhir-postgresql`'s store ever did. The other five ports' keyed-MAC
test coverage (where it exists — `sqlite_store.rs`, `mysql_store.rs`,
`mariadb_store.rs`, `mssql_store.rs` each build a `KeyRing` directly with a
fixed test key) was never gated by an environment variable at all; only
`fhir-oracle` has no keyed-MAC test coverage yet (`T11.8` gap). So the
`doc/containers.md` claim was doubly wrong on five of six ports: wrong name,
*and* describing a toggle that does not exist.

**Fixed.** `ChainKey::from_env`/`KeyRing::from_env` now take a `prefix: &str`
argument and read `<prefix>_CHAIN_KEY`/`_ID`/`_RETIRED` — the caller names
itself, since the shared function cannot know which port it is linked into.
`fhir-postgresql-store`'s sole call site now passes `"FHIR_POSTGRESQL"`.
Live-verified against a real PostgreSQL 18: `cargo test -p
fhir-postgresql-store --test audit` with `FHIR_POSTGRESQL_CHAIN_KEY` set
now genuinely exercises the keyed branch (`mac_breaks == 1`, previously
unreachable because the ring was silently empty), and with it unset the
unkeyed branch still passes — both directions, 5/5 tests either way. Full
`cargo test --workspace` in `fhir-postgresql` is green (22 test binaries, 0
failed), and all five other ports still `cargo check` clean against the
new `fhir-store` (none of their own code called the old zero-argument
signature).

**Documentation corrected to match**: `fhir-sqlite`, `fhir-mysql`,
`fhir-mariadb`, and `fhir-mssql`'s `doc/containers.md` no longer claim a
`CHAIN_KEY` environment variable exists — each now states plainly that their
keyed-MAC coverage runs unconditionally in test code, not from the
environment. `fhir-oracle`'s notes it has no keyed-MAC test at all yet.
`fhir-postgresql`'s row needed no change — it is now the one port where the
claim is true. `fhir-oracle/book/src/trust-boundary.md` (written earlier
this session, before this fix existed) already routed around the bug by
constructing a `KeyRing` explicitly; left as-is since that pattern remains
correct and is now merely no longer the *only* option.

*Found independently by the `fhir-postgresql` and `fhir-mariadb`
book-documentation-rewrite agents this session, while checking every env var
a chapter named against the shared `chain.rs` source rather than trusting
the port's own `CHANGELOG.md`; fixed and live-verified the same pass.*

## F-71

**`fhir-sqlite`: `active=true` — FHIR's own boolean-token search
spelling — silently matched zero rows against a `Patient.active` (or any
other `ColTy::Bool`) column.** Severity: **High** for a search feature: not
an error, not a degraded result, a token search that returns nothing for a
value that is present, with no indication anything went wrong.

**Root cause.** `sqlite_search.rs`'s `target_pred` bound every token value —
including a bare boolean code — as a `String` parameter (`rusqlite`'s TEXT
binding). SQLite's comparison-affinity rule only promotes a bound TEXT value
to NUMERIC when the text *looks like* a number: the literal word `"true"`
does not, so it stayed TEXT and was compared against the column's INTEGER
storage class (`ColTy::Bool` binds `INTEGER` on this port) — TEXT and
INTEGER never compare equal in SQLite regardless of value, so `active =
'true'` matched nothing. Binding `"1"` instead works, because `"1"` *does*
look like a number and SQLite's affinity rule converts it before comparing.

This is a different failure shape from the analogous Oracle defect found
earlier this session (**F-68**, part 5: `ORA-01722`, a hard error) — SQLite
does not error, it just quietly returns an empty result set, which is worse
for a caller to notice.

**Fixed.** Added `col_is_bool` (a `ColTy::Bool` lookup against the target
table's columns, threaded into `target_pred` via a new `cols` parameter, the
same shape as the Oracle fix) and `bool_token_as_bind`, which maps
`"true"`/`"false"` to `"1"`/`"0"` before binding in the `TargetKind::Token`
bare-value path — the only path booleans reach, since FHIR never models a
boolean search parameter with a `system` component. Verified with a new
test, `boolean_token_search_finds_a_true_value`
(`crates/fhir-sqlite-store/tests/sqlite_store.rs`): puts a `Patient` with
`active: true`, asserts `active=true` finds it and `active=false` does not.
Full `cargo test -p fhir-sqlite-store` is green afterward — the fix did not
disturb the 27 other tests in the same binary.

*Found by the `fhir-sqlite` book-documentation-rewrite agent this session,
while compiling and running the search examples it was about to write down
as fact rather than assuming they worked; fixed the same pass.*

## F-72

**The root `CLAUDE.md`'s description of `fhir-store/` was stale and
self-contradictory: it named the wrong crate as the HTTP surface, repeated
a nested-repository warning (F-37) that no longer applies to the directory
now holding that name, and said both `fhir-mssql` and `fhir-oracle` "have no
store" — both false, and the second false in the most consequential way, on
the file every agent working in this repository is told to read first.**
Severity: **Medium** — no data-safety impact, but this is the orientation
document, and being wrong here compounds: an agent trusting it would look
for the REST server inside `fhir-store/` (empty) rather than `fhir-loco/`
(where it is), and would tell a user `fhir-mssql`/`fhir-oracle` have no
store months after both did.

**The `fhir-store` name has meant two different things at two different
times, and the file described the earlier one.** `fhir-store/` was
originally the HTTP surface's home; it had its own untracked `.git` with no
remote, a genuine defect (**F-37**), fixed 2026-08-02 by removing the nested
repo and committing its source directly — the same day, that directory was
renamed `fhir-loco` (**F-45**). The name `fhir-store` was then reused,
within the same finding (**F-45**), for an unrelated extraction: a small
shared library (the tamper-evident audit chain, `Audit`, `AccessRecord`, and
the value types every port returns), pulled out of ~860 lines duplicated six
times. Root `CLAUDE.md` kept describing the *first* meaning — "the HTTP
surface... has no spec at all" in its orientation paragraph, and the F-37
nested-repository warning verbatim in its commit-and-push section — after
both had stopped being true of the crate holding that name. The file's own
later section (`## Traps specific to this repository`) already correctly
named `fhir-loco` as the real server, so the document contradicted itself
by the time a reader reached paragraph three.

Verified before fixing, not assumed: `ls -la fhir-store/.git` (no such
file), `git status --short fhir-store/` (ordinary tracked-file entries, no
`??`), and `git ls-files fhir-store/ | wc -l` (7 files, normally tracked) —
2026-08-04. `fhir-mssql`/`fhir-oracle` "no store" was checked against
findings **F-65**/**F-68**, both closed this session.

**Fixed.** Root `CLAUDE.md`'s orientation paragraph, "traps" section, scope
discipline note, and commit-and-push section all corrected — see the file's
own current text rather than duplicating it here, per this project's own
rule against maintaining the same fact in two places.

*Found while doing a documentation-accuracy pass on `fhir-store/` at the
user's request, expecting a routine README check and instead finding the
crate's own name was actively misleading relative to the repository's most
important orientation file.*

---

## The 2026-08-06 pass

A comprehensive re-audit of the whole tree — four parallel read-only sweeps
(the six ports' `plan.md`/`tasks.md`, the `spec/` tree itself, `fhir-loco`
and `fhir-store`, and the `fhir/` model family) plus mechanical gates
(`check-shared-core.sh`: 100 files identical, 0 exempt; a full relative-link
check over 289 markdown files: two real breakages, both in `doc/benchmarks.md`
files). The model family's findings are recorded in **`fhir/tasks.md`**
(Phase B) rather than here — a different family, its own register.

The pattern of this pass inverts the original audit's. The 2026-07-31 findings
were mostly *over*-claims — documentation asserting work that did not exist.
This pass's are mostly **closure-propagation failures in both directions**:
the two newest ports' docs still deny stores that exist (F-75, F-76, F-77,
F-79), the registers that closed those findings were themselves never brought
forward (F-73, F-74), and one recorded disposition — F-27's class 1 — was
written down and then not executed (F-80). The one finding class that is
neither: two ticked store-layer *guarantees* for code that was never written
(F-78), which is the original audit's failure mode, found in the two files it
had partially cleaned.

## F-73

**This register fell behind its own findings.** Severity: **Medium** — the
file exists to be the one place a reader can trust, and it disagreed with
itself. The summary table stopped at F-64 while detail sections ran to F-72;
the intro said "twenty-three fixed and five remain open" of the original
twenty-nine when all twenty-nine had closed; "What remains, and why" gave
paragraph-length rationales for not fixing F-04, F-08, F-15, F-17, and F-27 —
all five closed, four of them with dates, in the summary table above it; the
"65 files identical" count predated the gate's widening to 100 (F-48); and
F-49's row and disposition still said `scripts/` was untracked after commit
`60bfcbe` committed it and made the root gates live. **Fixed** in this
revision — every item above corrected in place, each with a note naming this
finding where the stale text was load-bearing.

## F-74

**The conformance matrix — the document this repository tells readers to
trust over every README — carried nine stale or self-contradictory cells.**
Severity: **Medium**. Verified against code, not against other documents:

1. The `put`/`put_audited` and `delete`/`delete_audited` rows showed `•` for
   all six ports, but the `_audited` variants exist **only** in
   `fhir-postgresql` and `fhir-sqlite` (zero grep hits in the other four) —
   the same file's own port paragraphs say so; the row conflated two
   operations and reported the union.
2. Four notes still said the former scaffolds have no store (`U6`/`U7`
   "neither scaffold has a store", `P6.4a` "awaits a store", `U4a`, `O10.7`
   "Oracle (no store)") — `fhir-oracle-store` is 2,598 lines with a search
   builder, `fhir-mssql-store` 3,296, and the matrix's own level rows call
   both Store.
3. `X15.1`'s note: "**Not** gated in CI — no workflow in this repository runs
   at all" — `gates.yml` runs `check-shared-core.sh --diff` on every push,
   and the same table's `W16.6` row already said so.
4. `W16.15` showed `!` citing F-11, which the same file's sources record as
   resolved (`origin` is `git@github.com:fhir-rust/fhir-rust.git`).
5. `T11.2` said mssql "23 of 23" while the level row says 33 — 23 is the
   pre-`upgrade.rs`/`ssl_live.rs` count. 33 verified per file.
6. `fhir-postgresql`'s basis said "8 test files"; there are 10
   (`chain_portability.rs`, `live.rs`, `m2_semantics.rs`,
   `search_semantics.rs`, `ssl_default.rs` were unlisted).
7. The header said "Measured 2026-07-31" over a body measuring through
   2026-08-04.
8. The closing "What would move `fhir-postgresql` up" bullet said the TLS
   default was still an open owner decision (F-17) while the `O10.7` row said
   the default verifies "since F-17".
9. One challenge did **not** survive verification, recorded because the
   audit itself made it: `search_page`'s pg `•` was suspected offset-only,
   but the function takes `after_id: Option<&str>` — a keyset cursor — so
   the cell stands; the port's `tasks.md`, which listed cursor paging as
   remaining, was the stale document.

Also stale in the same direction: test-count units mixed whole-port and
store-crate numbers with nothing saying which. **Fixed** 2026-08-06 in the
matrix revision accompanying this one.

## F-75

**`fhir-mssql/CHANGELOG.md` and `fhir-oracle/CHANGELOG.md` still opened with
"This port has no store and no driver", and directed readers to the one
section claimed to be true of the port — *Unreleased* — which is unmodified
`fhir-postgresql` text claiming a `fhir-<engine> serve` REST server, a CLI,
7,399 live corpus round-trips, and "live PostgreSQL 18".** Severity:
**High** — the banner's whole function was to separate inherited fiction from
port truth, and it pointed at fiction as the truth. Line 6 ("no store and no
driver") and line 32 ("Nothing here has ever *written* a chain, having no
store") have been false since **F-65**/**F-68**; line 17 says what is true "is
at the top of the file under *Unreleased*" while that heading sits at line
325, the bottom. The same "no store and no driver" framing survived at
`doc/benchmarks.md:5` in both ports, alongside a broken link
(`../spec/databases/audit.md` — resolves inside the port's own `spec/`; the
register is two levels up). **Fixed** 2026-08-06: banners and framing
corrected to date the store work, the *Unreleased* sections retitled as
inherited 2.0.0-dev notes with their server/CLI bullets struck, benchmarks
framing and links repaired. The per-port-history restructure remains F-62's
owner decision.

## F-76

**Four crates.io-facing `description` fields still call their crates
scaffolds.** Severity: **Medium** — `description` is the string a registry
renders. `fhir-oracle-map`: "the DDL emitter still emits MySQL, not Oracle"
(false since **F-08**, 2026-08-03); `fhir-oracle-gen`: "the DDL it emits is
MySQL"; `fhir-mssql-gen`: "no store exists to consume it" (false since
**F-65**); `fhir-mssql-map`: "T-SQL DDL emitted but never run against a green
CI gate" (`tests/upgrade.rs` runs it live). `spec/publishing.md` compounds it:
its P-1 section ("The scaffold ports ship a store that does not exist"),
its description table, a dangling "P-8" citation with no P-8 section, and a
2026-08-01 assessment date all predate the store work. **Fixed** 2026-08-06 —
descriptions rewritten, publishing.md P-1 restated as resolved with the
current gaps named.

## F-77

**The specification tree still describes the pre-Store world in six places.**
Severity: **Medium**. `spec/databases/index.md:131-132` — mssql "DDL only, no
store", oracle "scaffold only". `03-storage-model.md:366-383` — the Oracle
column all `⚠`, "verbatim copy of MySQL's", "the port is Scaffold level",
and "`fhir-postgresql` still derives the chain pre-image with
`(($1::text)::jsonb)::text`" (F-07 fixed; the same passage says so eight lines
later). `13-compliance-mapping.md:72` — "the Oracle scaffold".
`15-portability-and-dialects.md:150-152` — `X15.12` "Nothing in the current
tree tests `X15.1`…" (the shared-core gate exists and runs in CI).
`16-repository-and-release.md` — the layout diagram misplaces the core at
`spec/`, and `W16.3`/`W16.4`/`W16.6`/`W16.8`/`W16.9`/`W16.15` all state their
fixed defects in the present tense ("Six store crates currently describe
themselves as…"). `00-conformance.md`, `10-operations.md`, and
`11-conformance-testing.md` cite F-04, F-05, and F-06 as if open. Also
`index.md`'s Contents jumps §13 → §15 with no note that §14 is per-port.
**Fixed** 2026-08-06 in the section revisions accompanying this finding.

## F-78

**`fhir-mysql/tasks.md` and `fhir-mariadb/tasks.md` tick two store-layer
guarantees for code that does not exist.** Severity: **High** — unlike the
class-1 REST fiction (F-27), nothing about these is misattributed to another
crate; they are claims about *this* store's data-safety surface, `[x]`, with
acceptance text citing tests. `T33` (atomic conditional-op interactions "via
`pg_advisory_xact_lock`") — neither port has `conditional_create` or
`conditional_delete` in any form; the matrix row is `—`. `T34` (audit
envelope "threaded through `put_audited`/`delete_audited`/`transact_audited`/
`conditional_*_audited`") — none of the four functions exists in either crate
(zero grep hits; the plain `put`/`delete` do write audit rows and chain
links, which is what made the tick look plausible). **Fixed** 2026-08-06:
both entries unticked in both files and restated to what the code does.

## F-79

**Five closed findings were never propagated to the documents that cite
them.** Severity: **Medium**, both directions of wrong. (1) **F-15 closed**:
`upgrade` + `backfill_norm` exist and are live-verified in sqlite, mysql, and
mariadb (`sqlite.rs:390/632`, `mysql.rs:298/565`, `mariadb.rs:298/565`, eight
`tests/upgrade.rs` tests each in sqlite/mysql/mariadb, nine in mssql) — yet
`fhir-sqlite/tasks.md` listed `init --upgrade` as remaining twice while its
own T90a recorded it done; mysql and mariadb `tasks.md` and `README.md` said
the same; `fhir-sqlite/spec/index.md` still said "Unmet: `O10.4a` (no
`upgrade`)". (2) **F-17 closed**: `fhir-postgresql/tasks.md` and
`fhir-postgresql/spec/index.md` still said the default is `Prefer`.
(3) **F-08 closed**: `fhir-oracle/README.md:58` and
`fhir-oracle-store/README.md:48` still said "eleven `#[ignore]`d MySQL tests
still need replacing" — zero `#[ignore]` attributes exist in the port; the
replacement is recorded in `ddl.rs`'s own doc comment. (4) **F-65/F-68**:
`fhir-loco/README.md:14` still said "the two scaffolds". (5) **F-54 closed**:
`fhir-mysql/spec/14-mysql-dialect.md:69-70` still mandated preserving
`SslPolicy` and a plaintext-refusal guard this port deliberately replaced
with `SslMode`. **Fixed** 2026-08-06 across the named files.

## F-80

**F-27's class-1 disposition was recorded and never executed.** Severity:
**Medium** — the register said "deleting them is right… Each store port's
`M4` section now says where the server is", and the M4 sections were indeed
rewritten; but the several dozen individual `[x]` entries asserting a
`fhir-<engine> serve` binary, CLI flags, admin endpoints, Prometheus
histograms, and REST test suites remained in all four store ports'
`tasks.md`, ticked. Class-3 residue survived alongside it: "the chain is
computed in SQL" (T42, all four files, contradicting F-07 and each file's own
later entries), a ~40-line PostgreSQL search-tuning entry (T43: `_norm` SQL
functions, `text_pattern_ops`, `plan_cache_mode`, and `fhir-postgresql`'s own
`search_semantics.rs` cited as this port's evidence), `REPEATABLE READ READ
ONLY` and `pg_advisory_xact_lock` in ports without them, citations of test
files that exist nowhere (`audit_async.rs`, `edge_limits.rs`,
`validate_tests`), a claimed top-level `assets/` (real location:
`crates/<port>-map/assets/`), a twice-repeated broken annex path
(`spec/14-14-<engine>-dialect.md`), and the obsolete git-remote/`688641a`
blocks (F-11). **Fixed** 2026-08-06: class-1 entries replaced with one-line
tombstones naming this finding and `fhir-loco`, class-3 entries rewritten in
each port's own vocabulary, dead references corrected — in
`fhir-postgresql`, `fhir-sqlite`, `fhir-mysql`, and `fhir-mariadb`
`tasks.md`. `fhir-sqlite` also gained the missing task entry for **F-71**.

## F-81

**Every port's `plan.md` carries inherited decision entries that are
status-bearing and false, under a banner that only licenses them as
history.** Severity: **Medium**, with one High-adjacent instance:
`fhir-oracle/plan.md` D18 asserted `R4.5` is satisfied by "`REPEATABLE READ
READ ONLY`" in the port where `R4.5` is a *confirmed open gap* — the
mechanism Oracle actually offers failed live with `ORA-01466` and was removed
(**F-68**). Also: D20 claimed rustls/`sslmode`/a startup bind guard in ports
with no such machinery (mssql's real TLS state is **F-67**, a standing risk;
oracle's is undecided, `M14.22`); oracle D5 said "no driver yet" (the
`oracle` ODPI-C crate is the driver — a substitution artifact nobody
re-read); D11 claimed ETag optimistic concurrency that contradicts each
port's own `tasks.md`; D6 assigned axum to ports that have no HTTP; pg's D15/
D16 claimed trusted-proxy principal extraction and audit-mode flags that
exist nowhere; R7 kept PostgreSQL `unaccent` mitigations under a disclaimer
saying they do not apply; R9/M1 kept `vacuum` and "live-PG" wording.
**Fixed** 2026-08-06 across all six `plan.md` files.

## F-82

**`fhir-loco/tasks.md` predated the crate's own specification and
contradicted the tree in both directions.** Severity: **Medium**. Written
2026-07-31; the spec (`SV1`–`SV4`, 45 requirements) landed 2026-08-03 and the
file cited no `SV` id. It omitted the CapabilityStatement endpoint and the
entire PASETO auth layer — both shipped and tested — while keeping three
provably-obsolete open items: "Git remotes are wrong" (F-11, resolved), the
shared-history/`688641a` note, and "T70 accent folding misses Nordic
letters" — fixed in `fold.rs` (`'æ' => "ae"`, with `fold("Ærø") == "aero"`
asserted by test, in the shared core of all six ports). Its port-status note
("MySQL and MariaDB have native stores now") understated reality by four
ports. **Fixed** 2026-08-06 — replaced with an SV-cited list of what is
served, the four genuine HTTP gaps, and the multi-port wiring picture.

## F-83

**`fhir-oracle/book/src/introduction.md` has no banner, while root
`CLAUDE.md` (F-56) says every book opens with one.** Severity: **Low** — the
chapter's own prose is honest (it names the library/server split and
`fhir-loco`), so this is a broken promise about form, not content. The other
five books carry the banner, including `fhir-sqlite`'s (an earlier draft of
this finding wrongly counted sqlite as missing too — its banner is titled
"About this book"). **Fixed** 2026-08-06 — banner added, matching the
sibling books' wording.

## F-84

**All six ports' `publish.yml` iterate a `fhir-<engine>-server` crate and a
`fhir-<engine>` CLI crate, and all six `release.yml` build a `fhir-<engine>`
binary — none of which has ever existed, as each port's own `tasks.md` and
`plan.md` correctly state.** Severity: **Medium**, tempered by the fact that
these workflows were inert (F-49) — the fiction could not execute — but the
F-27 class-1 cleanup missed CI config, and any consolidation or future
publish would have failed on a nonexistent crate. *(As first recorded this
named only the two former scaffolds; executing the F-49 consolidation showed
the same lines in all six — the file was copied per port, like everything
F-27 catalogued.)* **Fixed** 2026-08-06 with F-49's consolidation: the six
publish loops now name only the three real crates, each with a comment citing
this finding, and the six binary-release workflows are deleted outright — a
release pipeline for a binary that violates `C0.18` by existing is not
machinery worth keeping inert.

## F-85

**`fhir-oracle` refused every root-level extension.** Severity: Medium.
Violates `db:R4.1` (any valid resource stores losslessly). Found 2026-08-10,
by measurement, while designing **F-47** step 5's `path` conversion — not by
any of the port's seven live store tests, none of which stored a root-level
extension.

A root-level extension — `{"resourceType": "Patient", "extension": […]}`,
the shape every US Core profile uses — shreds to an ext row whose attach
path is `""`. This engine binds `''` as NULL (`M14.29a`'s root cause), and
the `"path"` column was `CLOB NOT NULL`, so the insert failed outright:

```text
ORA-01400: cannot insert NULL into ("R5"."patient_ext"."path")
```

The read side had already half-anticipated the answer: it maps a NULL
`path` back to `""` (`unwrap_or_default`). What was missing was the write
side's half — the column must be **nullable on this engine**, because
`NOT NULL` here forbids a value every other port stores routinely. That is
now `M14.39`: NULL is the empty attach path's stored form; the columns
(`ext` and `deep` both, for one rule) are nullable where every other
port's annex says `NOT NULL`.

**Fixed both ways, live-verified (`tests/root_extension.rs`,
`tests/upgrade.rs`):** a fresh install gets the nullable bounded column
from `create_table` (F-47 step 3's arm, corrected); an existing install
gets it from F-47 step 5's conversion, whose replacement column is
nullable — and the step-5 test's sharpest assertion is exactly this
finding's payoff: after upgrading a legacy schema, the root-level
extension that ORA-01400'd before now round-trips. The legacy DDL arm
(`path_bound = 0`) deliberately keeps `CLOB NOT NULL`: it reproduces the
historical schema *including this defect*, because `G2.2` says an old
asset reinstalls the schema it always made — the upgrade, not the
emitter, is what fixes deployments.

`leaf` was checked for the same exposure and does not have it: an empty
leaf arises only from a spilled scalar, and every spilled FHIR datatype
is an object.

## F-86

**The model family rejects null-padded primitive arrays.** Severity: Medium.
Family: `fhir/` (every release crate — verified on R5 and R4B; the
representation is shared). Violates the losslessness the model claims for
valid FHIR JSON. Found 2026-08-10, by `fhir-r4b`'s full-corpus gate — the
first corpus to exercise the form.

FHIR JSON represents a repeating primitive with extensions as two parallel
arrays, and a position that carries only an extension is a **null** in the
value array: `"event": [null]` beside `"_event": [{…}]`. That is valid —
HL7®'s own R4B examples use it (nine of them). The model's repeating
primitives are `Vec<T>`, so the null cannot deserialize (`invalid type:
null, expected a string`) and, worse, cannot be *represented*: there is no
way to hold "no value at index 0" in a `Vec<types::DateTime>`. The R3/R4/R5
corpora pass only because HL7's copies of the same examples omit the value
array entirely (`"_event"` alone), which the parallel-`Vec` layout happens
to accept.

Fixing this is a representation decision — `Vec<Option<T>>` for repeating
primitives, or a dedicated container — that changes the generated API in
all six release crates at once. Recorded, not rushed: the R4B corpus gate
carries the nine affected examples as named known failures citing this
finding ("a bug with a note attached, not an exemption", R13.2).

Since F-87's same-day fix, the failure mode is honest: such a document is
**refused with a loud error** rather than parsed minus its element. The
ext-only primitive-choice form (`_valueX` with no `valueX`) is likewise
refused by name — it is this finding's other unrepresentable shape.

**Fixed 2026-08-10, the same day, owner-directed: a dedicated container.**
`fhir_core::PrimVec<T>` (spec `R6.7a`, `R9.1a`) is the value array as the
wire defines it — transparently `Vec<Option<T>>`, `None` the
extension-only placeholder, serialized back as the same `null` so the wire
form round-trips exactly. Every `0..*` primitive field in all six release
crates now uses it: the emitter switches on the repeating-extension
sibling, the `Builder`/emptiness derives treat `PrimVec` like `Vec`, and
construction stays ergonomic (`vec![…].into()`, `.values()` skips
placeholders). Five trees regenerated; R5's hand tree converted
field-by-field with the drift gate as referee — it caught the first splice
being wrong in *both* directions (missed `Coded<…>` repeating codes;
over-converted same-named complex fields in `TestScript`, `ValueSet`,
`DeviceDefinition`) and the comparator learned that rustfmt's trailing
comma in a wrapped generic is formatting, not drift. The gate then
demanded the R4B allowlist shrink: all nine null-padded examples
round-trip, so `KNOWN_FAILURES` is empty again.

**Stated residual** (also in `R6.7a`): `1..*` repeating primitives keep
`vec1::Vec1<T>` and its compile-time non-emptiness, so an extension-only
position in a *required* repeating primitive remains unrepresentable. No
example in any official corpus uses that shape; if one ever arrives it is
refused loudly (F-87), never silently dropped.

## F-87

**A choice element that fails to parse is silently dropped.** Severity:
**High** — this is data loss masquerading as success, in a clinical data
model. Family: `fhir/` (the choice machinery is shared by every release).
Found 2026-08-10, in the same nine examples as F-86.

The probe that proves it, run against `fhir-r4b`: a `Timing` whose `event`
is null-padded fails to deserialize **as a type** (`invalid type: null` —
F-86), but the *resource* containing it in `timing[x]` deserializes
without error and simply lacks the element; re-serialization then emits
the resource minus its `timingTiming`. A round-trip that quietly deletes
a dosing schedule is the exact failure mode a health-data model exists to
prevent — worse than refusing the document outright, because nothing
tells the caller anything happened.

The fix is behavioural, not representational, and is independent of
F-86's: the generated choice deserialization must propagate an inner
parse error instead of treating it as "variant not present". F-86 decides
what parses; F-87 decides that what does not parse **errors**.

**Fixed 2026-08-10, the same day.** The root cause was pinned by a layered
probe: the choice enum's own `Deserialize` propagates errors correctly;
serde's flatten machinery is what turns any error inside a flattened
`Option<T>` into `None` — and the swallow is `Option`-specific (a
flattened non-`Option` field propagates). So the fix removes `Option` from
the deserialization path without touching the public API: each release
crate gains `choice::Slot<T>(pub Option<T>)`; `#[derive(FhirChoice)]` now
emits a `Deserialize` for `Slot<Enum>` alongside the enum's own — absence
of every variant key is the one legitimate `None`, a present-but-invalid
payload propagates its error, and a consumed-but-unbuildable element (a
`_valueX` extension without its value, previously also dropped silently)
refuses by name. The generator emits, for every choice-bearing struct, a
private shadow struct (same fields, choice fields as bare `Slot`s) and
routes the real struct's derive through it with `#[serde(from = "…De")]` —
public field types unchanged. Applied to all six release crates: five
trees regenerated; R5's hand-documented tree spliced from its **own
committed** field types (a first splice from the generated tree
miscompiled exactly where R5's drift is sanctioned — `Coded<E>` versus
plain codes — which is the drift gate's point). Verified: the probe that
found the bug now errors loudly while the valid form round-trips
unchanged; default and all-features suites green; the R2 spec suite and
the full R3, R4, R5 and R4B corpora re-run green — the nine R4B
known-failure examples now fail **deserialization** (the honest outcome)
instead of silently losing their `timing[x]`, and remain allowlisted
under F-86 until the representation gap closes.

## F-88

**The consolidated port workflows (F-49) left three per-job settings
unrooted, and the first hosted runs exposed all three at once.** Severity:
**High**. Family: databases, all six ports (CI only — no source defect).

Consolidating every family's CI to the repository root (`F-49`) moved
`working-directory` from implicit (each port's own workflow file, checked
out at its own root) to explicit (`defaults.run.working-directory` per
job) — and three settings depended on the old implicit rooting in ways
nobody had re-checked against the new explicit one:

1. **`cargo deny` ran at the repository root** rather than the port's own
   `deny.toml`, so it errored outright instead of checking the manifest
   that actually matters.
2. **The spec/corpus environment paths pointed at the repository root**
   while the fetch steps that populate them wrote under the port
   directory — so every spec-dependent live test found nothing at the
   path it looked for and self-skipped. This is `T11.12`'s failure mode
   at workflow scale: the "live gate" reported green while testing
   nothing, on every port, on the first hosted run.
3. **The plaintext PostgreSQL job carried no explicit `PGSSLMODE`**, so
   the store's secure-by-default `require` (`O10.7`, **F-17**) correctly
   refused a plaintext connection it was never told to accept — surfaced
   only because the two new `history_page` tests happened to be the sole
   live tests that actually connected that run; everything else had
   already gone silent under defect 2.

**Fixed 2026-08-10.** Paths re-rooted to each port's own directory and
`cargo deny` given its manifest explicitly, in all six workflows; the
plaintext job now says `PGSSLMODE=disable` explicitly, with the TLS-only
job carrying `require`; and a vacuity guard added to the PostgreSQL live
step fails the build outright if anything self-skipped, so defect 2's
exact failure mode — a green run that tested nothing — cannot recur
unnoticed.

*Found investigating why the first hosted run of the consolidated
workflows looked green everywhere and proved almost nothing — three
independent settings, each individually plausible, compounding into one
gate that passed without exercising the thing it existed to check.*

## F-89

**The MySQL/MariaDB DDL test harness was unportable, and its failure mode
was to mask the real error rather than report it.** Severity: **Medium**.
Family: `fhir-mysql`, `fhir-mariadb`.

Three independent portability assumptions, each wrong on at least one
runner:

- It passed MariaDB's `--skip-ssl-verify-server-cert` flag to whichever
  `mysql` client happened to be on `PATH` — Oracle's own MySQL 8 client
  rejects a flag that is specifically MariaDB's, so the harness failed
  before it reached the schema it was meant to test.
- It assumed a `utf8mb4` default client charset. A runner whose client
  defaults to `utf8mb3` failed the collation probe with `ERROR 1253`, a
  message about a comparison the harness never intended to make.
- On any early client exit, it reported the shell pipeline's own `Broken
  pipe` (from writing SQL into a process that had already exited) instead
  of reading the client's own `stderr` — so whichever of the two defects
  above actually fired, the error a maintainer saw named neither.

**Fixed 2026-08-10.** The TLS flag is now gated on which client flavor is
actually present; the client is invoked with an explicit
`--default-character-set=utf8mb4`; and an early client exit falls through
to collect and report `stderr` instead of the pipe error, so the next
failure — whatever it turns out to be — names itself instead of hiding
behind a shell artifact.

*Found the same pass as F-88, running the newly consolidated workflows
for the first time on a runner whose defaults differed from the machine
the harness was originally written against.*

## F-90

**The full R3/R4/R5 schemas do not install on stock MySQL 8.4 / MariaDB
11.4.** Severity: **High**. Family: databases, all six ports (the split
structure is shared shape). Found 2026-08-10 by the first full-schema CI
install — `DDL_FULL=1` is CI-only, local suites sample the first 25
resources alphabetically, and the 2026-08-03 "green against live MySQL
8.4" predates the widest tables' exercise. Unmasked by F-89's harness
fix, which let the client's stderr through.

InnoDB refuses a table at CREATE time (`ERROR 1118`) once its charged
row size passes 8126 bytes, charging ~41 bytes per TEXT-family column —
measured by bisection against a live server: 195 TEXT columns install,
196 do not. The generator's only width bound was `G2.6`'s column count
(`SPLIT_WIDTH = 150`), which two shapes defeat: sibling expansions each
under the threshold sum without limit (`explanation_of_benefit`'s base,
232 columns), and a split-out choice table carries every variant's
columns inline (the open-typed `value[x]` splats —
`parameters_parameter_value`, `task_input_value`, the
`StructureDefinition` element `defaultValue`/`fixed`/`pattern`/`example`
tables — ~190–211 TEXTs each). The dialect cannot fix it alone: the
tables are map-shaped and the stores write by table name.

**Fix landed 2026-08-11, at the shared generator (`G2.6a`).** The
builder now carries a per-column byte model of the tightest engine
(TEXT-family 41, integers/dates at their charge, adjunct-only types 0
since InnoDB ports never render them) and a running-accumulation
force-split: once a table's charge would pass the 6,600-byte trigger,
every further expansion that can own a table is forced into one —
threaded through backbone, contentReference, choice (whole and
per-variant), and typed builds. The finished map, after the search
phase adds its fold columns, is asserted under a 7,900-byte budget:
generation fails loudly, the install never does. The widest resulting
table charges 6,611 bytes (r5 `explanation_of_benefit`). All 18 map
assets and the six fuzz fixtures were regenerated; `gen/tests/
row_budget.rs` re-checks the bundled artifacts in every port, and the
shared-core gate covers the new file (110 files identical).

**Sequenced residual, F-47-style:**

1. ~~Generator fix + regenerated artifacts~~ — live-verified
   2026-08-11: full R3/R4/R5 installs green on MySQL 8.4 (397s of
   CREATEs) and MariaDB 11.4, and the mysql/mariadb workflows fully
   green for the first time in the project's history. The first real
   store-suite execution the fix unmasked is **F-91**.
2. ~~Upgrade guard~~ — landed 2026-08-12 as **`O10.4b`** (a moved
   column is not a drop), in all six stores: `moved_columns` detects a
   dropped column — or a column of a dropped table — whose element
   path reappears in a different table, the store checks the source
   for data in its own dialect, and a data-bearing move refuses by
   name **independent of `allow_destructive`**, naming the
   disposition (re-put the affected resource types, or reload); an
   empty source proceeds through the ordinary destructive gate. The
   guard runs before that gate, because "rerun with
   --allow-destructive" is exactly the wrong advice for a relocation.
   Two live tests per port (refusal despite the flag; empty source
   proceeds); sqlite's ran green locally, the server ports' run in
   their CI live jobs. A resource-level re-shred migration
   (reconstruct under the stored old map, shred under the new)
   was owner-directed 2026-08-12 and is being landed F-47-style, one
   port per verified step, as **`O10.4c`**:

   1. ~~`fhir-sqlite`~~ — landed 2026-08-12: `upgrade_with` +
      `UpgradeOpts.reshred_moved` (plain `upgrade` unchanged);
      `fetch_recon_input` factored from `get` so reconstruction can run
      under the stored old map; the re-shred sits inside the single
      upgrade transaction between adds and drops, preserves
      `version_id`/`last_updated`, writes no history entry, verifies
      each resource byte-identical, and re-runs the moved-data check
      before the drops. Tested against the *real* pre-G2.6a r5 map
      (committed as a fixture from `fb8f27e`): 331 relocated columns,
      a `valueReference` carried across, refusal still fires without
      the opt-in, rerun re-shreds nothing. `UpgradeReport.reshredded`
      and `UpgradeOpts` live in `fhir-store` (0.2.0 — the new field is
      semver-breaking; six dependents bumped).
   2. ~~`fhir-postgresql`~~ — landed and **live-verified** 2026-08-21
      against PostgreSQL 18 in the `scripts/db.sh` podman container: `upgrade_with` + `UpgradeOpts.reshred_moved`
      (plain `upgrade` unchanged); `get_in_map` factored from `get_in`
      so reconstruction can run under the stored old map;
      `insert_shredded` gained an explicit `last_updated` so a carried
      resource keeps its timestamp. The dialect story is **`M14.29`**:
      one transaction per resource, resumable rather than atomic,
      because this port chunks its DDL to stay inside a lock budget.
      Its cost is stated rather than buried — between the additive DDL
      and the last resource carried, an un-carried resource
      under-returns the moved element, a window SQLite's single
      transaction does not have. Two tests added on the synthetic
      relocation the two `O10.4b` tests already use; PostgreSQL has no
      real pre-G2.6a fixture, because that force-split was driven by
      InnoDB's row limit and relocated nothing here.

      Both new tests **failed on their first real run**, and the cause
      is worth recording because it invalidated an assumption the two
      existing `O10.4b` tests had been carrying since they were
      written. `with_multiple_birth_moved` moved one column between
      tables in the map — enough for the DDL diff to report a
      relocation, and so enough for a test that only checks that the
      upgrade refuses. It was **not** a map anything could be written
      through: `shred` routes an element by `Elem.table` in the node
      arena, not by which table lists the column, so it kept sending
      `multipleBirthBoolean` at the base table and the insert panicked
      on a column no longer there. `O10.4c` is the first caller that
      shreds through a moved map, which is why it was the first to
      notice. The helper now moves every variant of the choice — a
      force-split choice owns its table for all of them — and repoints
      the element. The full store suite is green against the container:
      29 tests, 11 binaries.

      **The same latent defect was in `fhir-mysql` and `fhir-mariadb`,
      and their copies of the helper said so out loud**: "the map is
      deliberately not shred-consistent afterwards — `upgrade` only
      reads table shapes, and nothing is written through it". True when
      it was written, and false the moment `O10.4c` wrote through it.
      Both are fixed the same way. No other caller shreds through a
      hand-modified map, so those three ports' `O10.4b` tests were the
      whole blast radius.
   3. ~~`fhir-mysql` and `fhir-mariadb`~~ — landed and
      **live-verified** 2026-08-21 against MySQL 8.4 and MariaDB in
      their `scripts/db.sh` podman containers, 44 tests green in each.
      One logical change across the pair: `write_shredded` and
      `recon_with_map` factored out of `put` and `get` so the migration
      writes and reads through the same paths every other operation
      uses, then `upgrade_with` between the additive and destructive
      DDL. The dialect story is **`M14.38`** in both — reported-partial,
      because `M14.22` already is: these engines commit DDL implicitly,
      so the *schema* change cannot be transactional. What InnoDB can
      still give is that no single resource is half-carried, and
      `M14.38` states that alongside the read window it shares with
      PostgreSQL.

      One live-only defect, found on the first run: the port reads
      `last_updated` back in order to preserve it, and the driver hands
      back a temporal value rather than text, so the tuple conversion
      panicked. Every other reader of that column in the file already
      wrapped it in `DATE_FORMAT`; this one now does too. Compiling
      could not have found it.
   4. ~~`fhir-mssql`~~ — landed and **live-verified** 2026-08-22
      against SQL Server in its `scripts/db.sh` podman container, 38
      tests green. It is the one server port whose story is
      **all-or-nothing** (`M14.39`): T-SQL's DDL is transactional and
      the upgrade already ran as a single transaction, so the re-shred
      goes inside it and there is no window in which an un-carried
      resource under-returns the moved element. Its read path needed no
      surgery — `read_resource_rows` already took a map explicitly.
   5. ~~`fhir-oracle`~~ — landed and **live-verified** 2026-08-22
      against `gvenzl/oracle-free`, 16-test upgrade suite green on the
      first run. Story is **resumable** (`M14.40`), matching what this
      port's upgrade already was: every DDL statement commits
      implicitly and tolerates "already applied", so the re-shred
      commits per resource to match.

   **F-90's O10.4c migration is complete across all six ports.** The
   four failure stories are genuinely different and each is written
   down at the port's own id rather than generalised: sqlite
   all-or-nothing (`M14.31`), mssql all-or-nothing (`M14.39`),
   postgresql resumable-with-a-read-window (`M14.29`), mysql/mariadb
   reported-partial (`M14.38` in each), oracle resumable (`M14.40`).
   Every one of them states the read window where it exists, because
   `O10.4` asks for a failure story and a story with only the
   reassuring half satisfies that on paper and not in fact.

   A defect in step 1 was found while starting step 2 and is fixed
   here: the `fhir-store` 0.2.0 bump reached all six ports'
   manifests but only `fhir-sqlite`'s `Cargo.lock`. The other five
   still pinned 0.1.1, so `cargo check --locked` — what every port's
   `msrv` job runs — would have failed on all five. Regenerated.

## F-91

**The mysql/mariadb store suites never ran in CI, and their first run
found the job's TLS story missing.** Severity: Medium (test coverage —
the store itself behaved correctly). Found 2026-08-11, by F-90's fix:
the DDL step ahead of the store step had failed on every hosted run, and
cargo stops at the first failing test binary, so `concurrency.rs` (first
alphabetically) and everything after it had never executed here. The
step's own name still said "expected to skip until T64", which had been
false since the stores were ported to their native drivers — the
2026-08-03 "green against live MySQL 8.4 / MariaDB 11.4, 102 tests" was
measured locally and stayed unexamined in CI behind the mask.

The failure itself was the store working: `connect` verifies TLS by
default (F-54), the service containers present auto-generated
self-signed certificates, and the suite refused them with
`UnknownIssuer`. The job, unlike the pg one after F-88, had never
declared whether its link is plaintext-by-design.

**Fixed 2026-08-11**: both live jobs set `FHIR_MYSQL_SSL_MODE` /
`FHIR_MARIADB_SSL_MODE: DISABLED` with the same comment discipline as
pg's `PGSSLMODE: disable`; `ssl_live.rs` is unaffected — it constructs
its modes programmatically and still proves verification fails against
the same self-signed server. The step is renamed to what it does, and
the workflow's rationale for the still-missing TLS-only job no longer
cites T64: the honest gap is a `require_secure_transport=ON` server, so
the plaintext-refusal half of `O10.7` gets exercised on these engines
too.

## F-92

**Two more suites that were never really running.** Severity: Medium
(test coverage). Found 2026-08-12 by the discipline F-91 taught: after
a green run, check the tests you care about actually executed. The
O10.4b moved-column tests were confirmed running on pg and mysql — and
the same check exposed two ports where they could not have:

- **fhir-mariadb**: the main store suite, `mariadb_store.rs` (13
  tests), read `FHIR_MYSQL_TEST_DSN` — the *mysql* port's variable, a
  copy-substitution miss of the F-01 genre — while the CI job sets
  `FHIR_MARIADB_TEST_DSN`. Every test printed "skipping" and passed.
  The file's own header said "Skips silently", which was truer than
  intended. The port's other suites (concurrency, upgrade, redaction,
  ssl_live) used the right variable and did run.
- **fhir-mssql**: the live job had no store-suite step at all. The
  workflow was written when the port had no store and still said
  "cannot be written honestly until there is a store" — stale since
  **F-65** built one and F-47 gave it 12 live upgrade tests, all of
  which had only ever run locally.

**Fixed 2026-08-12**: the variable renamed (14 sites, one file); the
step added, with TLS intent already declared in the job's DSN
(`TrustServerCertificate=true` — the F-91 lesson applied rather than
relearned); both stale workflow rationales rewritten to name their real
remaining gaps. Verification is the next push executing both suites in
CI for the first time — and whatever that first execution unmasks is
the next finding, not a regression of this one.

---

## F-93

**Medium.** Found 2026-08-26 by the CI-watch loop, working backward from the
run history: `fhir-oracle CI` had not been green since 2026-08-22 04:22, and
the first red was the run for the commit that *added* the `O10.4c` re-shred —
a commit whose message claimed it live-verified. The claim was true of a
local run and false of every hosted one, which is this register's oldest
genre (F-27, F-92): verification asserted, never re-checked where it counts.

**The first defect.** `recon_with_map` — the read path the re-shred uses both
to reconstruct under the old map and to verify the rewrite under the new
one — ended with `let _ = conn.rollback();` (and carried a second one on its
not-found early return): hygiene, so a pooled connection went back clean.
Under `get` that is harmless; the reads lock nothing. But the re-shred's
verify runs **inside the per-resource write transaction**, so the sequence
was: `DELETE` the base row, re-insert base and children under the new shape,
read it all back (visible — same transaction), compare canonically (equal),
*rollback everything*, then `commit()` a transaction that no longer had
anything in it. The old row's data was still in place, and the leftover
guard — checked before any column is dropped, exactly as designed — refused
with "re-shred left data behind … rerun to resume". Diagnosed by bracketing
the commit with probes: the moved-table row was visible to the verify and
already gone one statement later, on the same connection, before the commit.

**The second defect, unmasked by fixing the first.** With commits real, the
suite failed differently on its second consecutive run: `ORA-00001` on
`patient_multiple_birth_moved (rid, ords)`. `drop_schema` iterated the
*connected map's* tables — and a relocated-column table belongs to no
original map, so it survived the drop with its rows but not its FKs (those
died with the parent's `CASCADE CONSTRAINTS`), and the next run's re-shred
collided with the residue. `fhir-mssql`'s `drop_schema` had always been
catalog-driven (`sys.tables` for the schema); the oracle port now matches,
sweeping `user_tables` — `M14.5` puts each FHIR version in its own Oracle
user, so the connecting user's tables are exactly this store's world.

**Verification (2026-08-26, Oracle Database Free 23):** the upgrade suite
16/16 **twice consecutively** — the second pass is the point; it exercises
the residue class the first defect had been hiding — plus `oracle_store`
7/7 and `root_extension`, `fmt` and `clippy -D warnings` clean.

**Identified during analysis, not yet exercised, left open deliberately:**
resuming a re-shred that died mid-run looks wedged for already-carried
resources. On a rerun, the stored map is still the old one, so a carried
resource is reconstructed through the old map — which reads the base column
the carry emptied — while the verify reads the moved table that holds the
value; the two cannot match, and the per-resource rollback (correctly)
refuses forever. No data is lost — that guard is doing its job — but "rerun
to resume" would then be a promise the carried rows break. Untested; whoever
next touches the re-shred should write the kill-mid-run test before
believing either outcome.

**One more thing this diagnosis re-learned, recorded because it will happen
again:** part of an earlier session's investigation ran the suite without
the `FHIR_ORACLE_TEST_*` variables set and read six consecutive "ok" results
as passes. They were skips — the suites self-skip without credentials, and a
skipping test is indistinguishable from a passing one in the summary line
(T11.12, F-91's lesson, relearned live during the very finding that cites
them).

## F-94

GitHub's push output on `ecfe339` reported "5 vulnerabilities (1 high, 4
low)". `gh api repos/fhir-rust/fhir-rust/dependabot/alerts` gave the exact
five: three were `rustls-webpki` in `fhir-mssql/Cargo.lock` — **F-67**,
already tracked, already open, already awaiting the owner's decision, and
correctly left untouched here. The other two named a crate this register had
not seen before: `lru`, `GHSA-rhfx-m35p-ff5j`, one alert each in
`fhir-mysql/Cargo.lock` and `fhir-mariadb/Cargo.lock`.

`cargo tree -i lru` in both ports showed the same shape: `lru 0.12.5`, a
normal (not dev) dependency of `mysql_async 0.34.2`, which is a normal
dependency of the store crate itself. `mysql_async`'s own manifest pins
`lru = "^0.12"`, so `cargo update -p lru` alone cannot move it — confirmed by
trying: `cargo update -p lru --precise 0.16.3` (the first patched version)
fails with "candidate versions found which didn't match: 0.16.3 ... required
by ... mysql_async v0.34.2". The advisory can only close by moving
`mysql_async` itself.

Checked before choosing a target rather than jumping to latest: `mysql_async`
0.35.0 and 0.35.1 still require `lru ^0.12`; 0.36.0 moved to `^0.14`, still
short of the `0.16.3` fix; 0.37.0 requires `^0.18`, clearing it. `0.37.0` is
also `mysql_async`'s current latest release, so this is not settling for an
intermediate — it is the only version in the 0.35–0.37 line that fixes
anything, and it happens to be the newest.

Applied identically to both ports (`mysql_async = "0.37"` in each workspace
manifest, `cargo update -p mysql_async --precise 0.37.0`), then verified
before being trusted rather than after: `cargo check --all-targets --locked`
green in both, the offline unit suite green in `fhir-mysql-store` (50 tests
across `fold`, `value`, `ssl`, `mysql` — no database needed, per `T11.12`'s
discipline about tests that require nothing to pass vacuously), and
`cargo deny --all-features check advisories` reporting `advisories ok` with
no warning in either port.

**A second, smaller thing the bump exposed.** Each port's `deny.toml` carried
an `ignore` for `RUSTSEC-2025-0134` (`rustls-pemfile`, unmaintained), reasoned
at length when it was written. Running `cargo deny` after the bump printed
`warning[advisory-not-detected]: advisory was not encountered ... no crate
matched advisory criteria` — `rustls-pemfile` had already dropped out of the
dependency tree as a side effect of the `mysql_async` update (confirmed:
`grep -c 'name = "rustls-pemfile"'` returns `0` in both lockfiles), and the
ignore rule had gone silently dead. `cargo deny` warns rather than fails on
this, so nothing would have caught it without a human reading the warning
line. Removed the entry in both `deny.toml` files with a dated comment
recording why the exception disappeared, rather than leaving the drop silent
or the stale reasoning in place.

**A process finding worth generalizing, not just this instance:** an unmatched
`ignore` entry is evidence in the *opposite* direction from an unpatched
advisory — it means a stated risk acceptance no longer describes anything
real — and `cargo deny`'s default posture (warn, not fail) means it can sit
unnoticed indefinitely. Nothing in this repository's CI currently fails a
build on `advisory-not-detected`; whether it should is a smaller, separate
question from this finding, noted rather than decided here.

## F-95

The push that fixed F-94 (`259e209`, an unrelated CI workflow change layered
on top) turned four hosted jobs red: `fhir-mysql CI`'s and `fhir-mariadb
CI`'s live-database jobs, both failing at the same step, `Store live suite`.
`gh run view --json jobs` narrowed it to one test each:
`tls_is_configurable_and_verification_is_not_a_no_op`, in `tests/ssl_live.rs`
in both ports.

The panic named its own cause: `rustls::crypto::CryptoProvider::get_default()`
found nothing installed. `mysql_async 0.34`'s `rustls-tls` feature apparently
selected a crypto backend implicitly; `0.37`'s manifest, fetched and read
rather than guessed at, shows the split explicitly —
`default-rustls-ring = ["default-rustls-no-provider", "ring"]` and the
`aws-lc-rs` equivalent both layer on top of a `default-rustls-no-provider`
base that `rustls-tls` alone resolves to. Enabling `rustls-tls` without also
naming a provider feature is valid to cargo and broken at first real use — the
exact shape of defect a compiler cannot catch and only an exercised code path
finds.

**Why this got past the F-94 verification that immediately preceded it.**
That entry's own text says what was run: `cargo check --all-targets --locked`
and `cargo test --locked --lib --bins`. `--lib --bins` is precise about what
it excludes — everything under `tests/`, which is where integration tests
live in a cargo package, and `ssl_live.rs` is one. It cannot fail an
invocation that never compiles it. Worse, even `cargo test` without `--lib
--bins` would not have caught this in an ordinary developer run: `ssl_live.rs`
self-skips without `FHIR_MYSQL_TEST_DSN`/`FHIR_MARIADB_TEST_DSN` set, so it
only ever executes inside the live-database CI job, against a real service
container. No unit test, and no offline integration test, exercises this
path — by design, since it is inherently about a live TLS handshake. The
hosted live job was always the first and only place this could be caught, and
it was not consulted before the push.

**Fixed** by reproducing before trusting the fix, not the reverse. Brought up
each port's own dev container (`fhir-mariadb`'s and `fhir-mysql`'s
`scripts/db.sh up`, already running from earlier work), ran
`cargo test --release -p fhir-<port>-store --test ssl_live` against it with
the pre-fix manifest, and got the identical panic locally, live, before
changing anything — confirming the local containers reproduce what hosted CI
saw rather than assuming they would. Added the `ring` feature (both
manifests): pure Rust, no C compiler or cmake, chosen over `aws-lc-rs` to keep
faith with the adjacent comment's existing reason for preferring rustls over
native-tls at all. Re-ran the same command against the same running
containers: both tests green. Then, because a fix verified only on the test
that caught it is exactly F-91/F-92's failure shape one level up, ran every
test binary in both stores with no output truncation this time —
`mysql_store`, `concurrency`, `redaction`, `roundtrip_types`, `ssl_default`,
`ssl_live`, `upgrade` — 44 tests, 0 failed, and re-checked
`cargo deny --all-features check advisories` clean in both.

**The lesson, stated so the next dependency bump does not relearn it:** a
crate that ships integration tests gated on live infrastructure has coverage
its own default `cargo test` cannot demonstrate, and a change to that crate's
dependency graph is not verified until the gated tests have actually run
against the infrastructure they need — this repository's own container
scripts make that a two-command check (`scripts/db.sh up`, then the live test
target), and the excuse of "unit tests were clean" was not good enough here.

## F-96

Found in the same pass, unrelated to F-94/F-95: `fhir Security Audit`'s
`cargo-deny` matrix also failed on `fhir-postgresql` and `fhir-loco` (which
depends on it), with `error[yanked]: detected yanked crate (try `cargo update
-p chacha20`)`. `chacha20 0.10.1` reaches both through
`rand 0.10.2 -> postgres-protocol -> tokio-postgres -> fhir-postgresql-store`,
a chain neither this session's mysql/mariadb work nor **F-94** touched at
all — crates.io yanked the version out from under two lockfiles that had
already pinned it, and `yanked = "deny"` (the same `deny.toml` line that makes
**F-94**'s dead-ignore detection possible) is what caught it on the next push
rather than the one that actually introduced the pin.

**Fixed** by doing exactly what `cargo deny`'s own error message named:
`cargo update -p chacha20` in both `fhir-postgresql` and `fhir-loco`
(`0.10.1` → `0.10.2`). Verified `cargo deny --all-features check advisories`
clean in both afterward, and `cargo check --all-targets --locked` green in
both.

## F-97

**F-51**'s own closure said it plainly rather than leaving it implicit: two
behaviours this port relies on were verified only by hand, or only by a
unit test checking the *shape* of generated SQL text, never by exercising
the real engine. Both are exactly the class of defect a text-shape check
cannot catch — `M14.29a` already lived through this once: the trigger's
first version "was written, installed, and observed letting an ordinary
DELETE through with no error," because `NVL(x, '') != 'y'` evaluates to
NULL rather than TRUE when Oracle folds the empty string to NULL, and the
`ELSIF` simply never fired. It read as correct. A `CHECK` clause that
parses but never actually constrains anything would pass `ddl.rs`'s own
unit test (which asserts the clause's text is present) the identical way.

Checked before writing a test, not assumed: neither behaviour appeared
anywhere in `fhir-oracle-store`'s live suite either (`grep -rl
"M14.29\|M14.8" fhir-oracle-store/tests/` — nothing). Both were genuinely,
completely untested against a real server.

**Fixed** with `crates/fhir-oracle-map/tests/oracle_constraints.rs`, live
against `gvenzl/oracle-free`, reusing `oracle_ddl.rs`'s admin-connection
pattern (`M14.5`: provisioning a fresh schema means provisioning a fresh
user). Both constraints are generic enough not to need the full generated
Patient schema — `append_only_triggers` takes only a schema and table name,
and the boolean CHECK only needs one `Bool` column — so each test builds a
minimal synthetic table and applies the exact PL/SQL or DDL the generator
emits, isolated from shredding concerns this finding was never about.

- **The trigger test** seeds one row, then asserts an `UPDATE` is rejected
  with exactly `ORA-20001` and an undeclared `DELETE` with exactly
  `ORA-20002` — not merely `is_err()`, which is precisely the distinction
  `M14.29a`'s bug hid: a silently-succeeding forbidden DELETE and a
  correctly-rejected one both "work" if only the boolean is checked.
  Confirms the row survives the rejected DELETE, then confirms the
  declared-erasure escape hatch (`DBMS_APPLICATION_INFO.SET_CLIENT_INFO`,
  set and cleared in the same transaction as `M14.29` requires) still
  removes it.
- **The CHECK test** inserts `0` and `1` (must succeed), then `2` (must fail
  with exactly `ORA-02290`, Oracle's constraint-violation code) into a
  column built with the same `CHECK ("col" IN (0, 1))` clause
  `create_table` emits.

**Found and fixed in the same pass, not shipped with a known flaw:** the
first version of this test file gave both `#[test]` functions the same
throwaway user, on the assumption tests in one binary run one at a time.
They do not — libtest parallelizes within a binary by default — and both
tests provisioning (drop-then-create) the same Oracle user at once is
exactly the "flaky live gate is worse than a failing one" trap
`mssql_ddl.rs`'s own history already warns about. Reproduced deterministically,
3 of 3 runs failing the same way, before splitting each test onto its own
user (`TRIGTEST`, `BOOLTEST`); 0 failures in 5 repeated runs after, plus the
skip and fail-loud paths both re-confirmed against a genuinely unreachable
connect string, `--release`, and `cargo clippy -- -D warnings` clean.
Wired into `fhir-oracle-ci.yml` beside **F-51**'s DDL-install step.

## F-98

**`scripts/check-published-match.sh` can report "ok" for a crate whose
source has genuinely diverged from what crates.io received, when the
divergence is a workspace-inherited dependency version.** Severity:
**Medium** — this is the exact gate `agents/release.md` calls "the gate
that matters most" (`O10.11`), and it has a real blind spot, not a
hypothetical one.

The script's own header comment explains why it excludes `Cargo.toml` from
its packaged-file diff: "cargo normalizes this; the verbatim manifest is
preserved beside it as `Cargo.toml.orig`, which IS compared, so manifest
changes are still caught." That reasoning holds for a dependency declared
directly in a member crate's own `Cargo.toml` — but not for one declared
`sha2.workspace = true`, whose *resolved* version lives only in the
workspace root's `[workspace.dependencies]` table, a file that is never
itself part of any member crate's published tarball. `Cargo.toml.orig`
preserves the member's own manifest exactly as written — still
`sha2.workspace = true` — regardless of what the workspace root's
requirement says today or said at publish time. The normalized `Cargo.toml`
that `cargo package` actually generates (and that crates.io serves to
consumers) *does* flatten `.workspace = true` to a literal version, but
that file is precisely the one the script excludes.

**Found while bumping `sha2` and `sha3` in `fhir-postgresql` and
`fhir-sqlite`** (closing the underlying dependency-update work Dependabot
had proposed as three overlapping PRs). After changing the workspace-root
requirement from `sha2 = "0.10"` to `sha2 = "0.11"`,
`scripts/check-published-match.sh` reported `34 matched, 0 mismatched` —
vacuously true only in the sense that it compared the wrong file. The
crates.io API, queried directly, told a different story:
`GET /api/v1/crates/fhir-postgresql-map/0.6.0/dependencies` returns
`sha2 ^0.10` for the version already published — a requirement the local
tree's new `sha2 = "0.11"` no longer satisfies. Confirmed for all six
affected member crates (`fhir-postgresql-map`/`-gen`/`-store`,
`fhir-sqlite-map`/`-gen`/`-store`) the same way, all published at `0.6.0`
per the API's `max_version`, none of them flagged by the script.

**What this means in practice: the script's "ok" is not proof for any
crate whose dependency versions are declared via workspace inheritance —
which, in this repository, is all of them** (every port's `map`/`gen`/
`store` crates declare their crates.io dependencies as `foo.workspace =
true`, per `X15.1`'s shared-core convention). A change to a workspace-root
`[workspace.dependencies]` version can silently pass this gate while
genuinely violating `O10.11`.

**Not fixed here, worked around instead:** the six affected crates plus
`fhir-loco` (whose `Cargo.lock` also needed regenerating as a companion,
for an unrelated but compounding reason — see the commit) were bumped to a
patch version (`0.6.1`/`0.3.1`) regardless of what the gate reported,
restoring genuine compliance without relying on the tool to confirm it
(commit `ca34cdf`). A correct fix needs the script to additionally compare
each crate's *normalized* `Cargo.toml` — the one `cargo package` actually
produces, with `.workspace = true` resolved — while still tolerating
cargo's own cosmetic normalization (key reordering, quoting) so the fix
doesn't reintroduce the false positives `Cargo.toml.orig` was chosen to
avoid in the first place. That is real design work, not a one-line patch,
and is left open rather than attempted under time pressure.

*Found investigating a routine dependency bump, by cross-checking the
gate's "ok" against the crates.io API directly rather than trusting it —
exactly the discipline `agents/release.md` asks for and this finding
exists because, this once, it was actually applied.*

**Closed by F-102**: the normalized-`Cargo.toml` comparison this entry
deferred is implemented, verified against synthetic cosmetic-reorder and
real-divergence cases before being trusted against crates.io, and running it
for real found the blind spot was not hypothetical — see F-102 for what it
found and the deeper, related defect (no CI job called this script at all)
that let it stand undetected.

---

## F-99

**`fhir-postgresql-store`'s `checkpoints_are_logged_on_their_own_target_without_phi`
test fails reproducibly against `deadpool-postgres` 0.14.2, and the PR
proposing that bump is held pending it.** Severity: **Medium** — the test
itself is sound (it is the one guarding that a checkpoint's audit-log line
never carries PHI, `M3.17`-adjacent), but it now fails to observe *any*
output at all, which is a test-technique fragility, not evidence the
guarantee itself broke.

Found triaging Dependabot PR #59 (`deadpool-postgres` 0.14.1 → 0.14.2 in
`fhir-postgresql`). Two independent hosted runs of that PR's `Live-database
tests (PostgreSQL 18)` job both failed the same way:

```
thread 'checkpoints_are_logged_on_their_own_target_without_phi' panicked at
crates/fhir-postgresql-store/tests/audit.rs:425:5:
checkpoint must land on its own target:
```

The empty string after the colon is the captured log itself — the
assertion's own `{logged}` interpolation. The test captures `tracing`
output by installing a custom writer as the thread's default subscriber
(`tracing::subscriber::set_default`, scoped to the calling thread only) for
the duration of `store.emit_checkpoint("test").await`, then asserts the
capture contains `"audit_checkpoint"` (the event's `target`). Both runs
captured nothing, yet `chain_witness()` — the fallible call inside
`emit_checkpoint` — did not itself error (a `chain_witness()` failure
would emit via `tracing::error!` instead, still inside the same capture
window, still absent either way). The other four tests in the same binary,
sharing the same connection style, passed both times — ruling out a
general PostgreSQL-18-job or connectivity regression and pointing at this
one test's capture technique specifically.

**Leading hypothesis, not confirmed at the mechanism level:**
`deadpool-postgres` 0.14.2's own changelog headline is directly on point:
"Coalesce concurrent statement preparations. Tasks racing to prepare the
same query now share a single `PREPARE` instead of each sending their own."
That is new inter-task coordination sitting exactly on the code path
`chain_witness()` exercises — a plausible way for the query that actually
runs (and whatever polls it to completion) to end up outside the calling
task, and therefore outside a thread-local subscriber's scope, in a way
0.14.1 did not. Checked and ruled out the simplest version of that theory:
each test in `tests/audit.rs` opens its own `Store` via `test_store()`,
so there is no connection pool shared *across* tests for a coalesced
prepare to race against — if the hypothesis holds, the coalescing must be
happening within this one test's own single call, which is not yet
confirmed by reading `deadpool-postgres` 0.14.2's source directly.

**Not fixed here.** Two honest paths forward, neither attempted under time
pressure: (1) read the `deadpool-postgres` 0.14.2 diff far enough to
confirm or rule out the coalescing path for a single caller with no
concurrent racer, or (2) stop relying on a thread-local subscriber for
this test — capture via a process-global subscriber (installed once, e.g.
with `tracing_subscriber`'s reload layer) so the assertion survives
regardless of which task or thread ends up running the pooled query. PR
#59 is left open rather than merged past a failure that was reproduced,
not merely observed once.

**Update, closed while continuing the CI-watch backlog:** path (1) is now
settled, and it clears `deadpool-postgres` rather than convicting it. Both
`deadpool-postgres-0.14.1` and `-0.14.2`, and their `deadpool` (0.12.3 →
0.13.1) and `deadpool-runtime` (0.1.4 → 0.3.1) companions that the bump also
carries, were diffed directly from the local registry cache rather than
inferred from a changelog headline. The `statement_cache.rs` extraction that
motivated the leading hypothesis replaces a `RwLock<HashMap<Key, Statement>>`
with a `RwLock<HashMap<Key, Arc<OnceCell<Statement>>>>`: for a single caller
with no concurrent racer for the same key — which every test in
`tests/audit.rs` is, since each opens its own fresh `Store` and pool via
`test_store()` — `OnceCell::get_or_try_init` runs its `init` closure inline,
on the calling task, exactly as the old code's direct
`client.prepare_typed(...).await` did. Nothing in the diff of any of the
three crates spawns a task or a blocking thread on this path (the one
`spawn_blocking` added to `deadpool-runtime` 0.3.1 is unused by `deadpool`'s
and `deadpool-postgres`'s own source, confirmed by `grep`, not assumed). The
coalescing theory is therefore ruled out on direct evidence, not just
"checked and ruled out the simplest version" as the entry above left it.

**What the evidence does point to.** `emit_checkpoint`'s `tracing::info!` is
one physical callsite, and it is reached from three places: this test's
direct call, and internally from both `resign_history` (exercised by
`resigning_refuses_tampered_history_and_frees_the_old_key`) and the erasure
path (exercised via `audit_trail_is_complete_and_tamper_evident`) — all five
tests in this binary run in parallel, in the same process, on separate OS
threads. `tracing`'s per-callsite interest cache is decided globally the
first time a callsite fires anywhere in the process; a thread-local
`tracing::subscriber::set_default` guard — which is what this test used —
does not trigger the cache rebuild that installing a global default does.
Whichever of those three call paths reached the callsite *first* in a given
process, with no subscriber active, would cache "nobody's interested" for
the rest of that run, starving every later call on any thread including one
with an active `set_default` guard — the exact "captured nothing" signature
both hosted runs showed, on a callsite none of the other four tests'
assertions depend on. This is consistent with everything observed but was
not reproduced locally: 8 runs of the live suite at `--test-threads=2`
against the exact `-0.14.2`/`0.13.1`/`0.3.1` trio, and 3 more against the
original `-0.14.1`/`0.12.3`/`0.1.4` trio, all passed — the race evidently
needs scheduling characteristics this 16-core laptop's container did not
reproduce, most likely GitHub's runner having far fewer cores. Recorded as
the leading mechanism, not a certainty, for the same reason this entry
already models: state what was checked and what remains inferred.

**Fixed regardless of which mechanism is the true one**, per path (2):
`tests/audit.rs` now arms a single process-wide `tracing_subscriber::fmt`
default via `tracing::subscriber::set_global_default`, behind a `OnceLock`,
called from `test_store()` — every test's first real step — rather than
scoped with `set_default` around one call in one test. This is
mechanism-agnostic: it removes the thread-local/interest-cache hazard
outright rather than resolving which of this test's three callers would
have raced. Verified: `cargo fmt --check` and `cargo clippy --tests -D
warnings` clean; the full `fhir-postgresql-store` live suite (26 tests
across `audit`, `concurrency`, `history_page`, `live`, `m2_semantics`,
`redaction`, `search_semantics`, `ssl_default`, `upgrade`) green against a
local PostgreSQL 18 container, both with the original lockfile and with
`deadpool-postgres` bumped to `0.14.2` locally to match PR #59 exactly; the
targeted `audit` suite alone re-run 9 times total under the `-0.14.2` trio
with no failure. PR #59 is unblocked once this fix lands on `main` and its
next hosted run picks it up.

## F-100

**`AI_STATEMENT.md` — the document §8 of itself holds up as the record of
this project's "confident prose that nothing substantiates" failure mode —
had two instances of exactly that failure mode inside it.** Found while
reconciling the document against a governance change (2026-09-02: the owner
authorized Claude to execute `cargo publish`), not while looking for this
specifically; both predate that change and are unrelated to it.

1. **§4 banned naming a tool as co-author of anything in this repository,
   flatly, since version 1.0.0 (2026-08-26).** `git log --all -i --grep
   "co-authored-by" --oneline | wc -l` returns 185 — commits both before and
   after 1.0.0's issue date carry a `Co-Authored-By: Claude Sonnet 5` (or
   `dependabot[bot]`) trailer. Git's author/committer field was never a
   tool — verified on several commits directly (`git show -s --format="author:
   %an <%ae>%ncommitter: %cn <%ce>"`) — but the trailer itself does name a
   tool as co-author, in commit messages, in this repository, which is the
   exact thing the sentence said does not happen.
2. **§12 claimed "Nothing is signed. No commit or tag signature exists."**
   `git config commit.gpgsign` is `true`; `git log --show-signature` reports
   "Good git signature" (SSH, ED25519, the maintainer's key) on commits and
   tags alike. Signing has been active since 2026-08-27 — a date this
   document's own §13 ("revised off-cycle when ... a claim in this document
   stops being true") should have caught, and did not for six days.

**Fixed, not merely noted**, in the same pass: `AI_STATEMENT.md` bumped to
1.1.0 (2026-09-02). §4 now describes the trailer as disclosure rather than
authorship (git author/committer, and accountability, are unmoved by it) and
explains why the 1.0.0 wording was wrong rather than silently dropping it.
§12's signing bullet corrected to state what signing actually verifies (the
git identity) and what it does not (tool involvement, which is what the
trailer is for). §5/§6 gained the `cargo publish`-execution row the
governance change itself required, kept explicitly distinct from the release
*decision*, which stays the maintainer's alone. `CONTRIBUTING.md`'s "Using AI
tools" section, which told contributors to put AI disclosure "in the
description, not in commit trailers" — the same claim in a second document —
was corrected in the same pass rather than left to contradict the just-fixed
`AI_STATEMENT.md` §4.

**Why this is worth its own finding rather than a quiet edit:** a governance
document that misdescribes the practice it governs is a worse failure than
having no such document, because a reader — a downstream integrator, an
auditor, a contributor — has no way to detect the gap from the document
alone; only comparing it against the tree, as this repository's own culture
insists on doing for every other claim, surfaced it here. The document
survives that comparison now; it did not before this finding.

## F-101

**The exact `mysql_async` version F-94 bumped to was yanked from crates.io
after that bump landed.** Same class as F-96 (`chacha20`, also yanked
post-pin): `cargo deny`'s `yanked = "deny"` caught it on the merge commit for
PR #59 (an unrelated `deadpool-postgres` bump), failing `fhir-mysql` and
`fhir-mariadb`'s security-audit jobs on `main` — `mysql_async 0.37.0`, pinned
by F-94 2026-08-31, shows `yanked=true` via the crates.io API as of this
finding.

**Fixed** the same way as F-96: `cargo update -p mysql_async --precise
0.37.1` in both ports (manifest requirement `"0.37"` already covers it, no
`Cargo.toml` change needed). Checked before bumping rather than assumed:
`0.37.1`'s own dependency manifest still requires `lru ^0.18`
(crates.io `/dependencies` API), so F-94's advisory fix is not undone.
Verified in both ports: `cargo deny --all-features check advisories` →
`advisories ok`; `cargo check --all-targets --locked` clean; offline
`--lib --bins` suites green (44 + 6 tests each); and — because F-95's own
lesson is that a `mysql_async` bump's TLS behaviour is only proven live, not
by `--lib --bins` — the full live suite against local containers (MySQL 8.4,
MariaDB 11.4), `--test-threads=1`, including `ssl_live.rs`'s two tests
actually executing (not self-skipping) and passing: 27 tests in
`fhir-mysql-store`, 27 in `fhir-mariadb-store`, 0 failed. `fmt`/`clippy -D
warnings` clean in both.

## F-102

**F-98's own predicted consequence — "a workspace-root dependency version
bump can silently pass the gate while genuinely violating O10.11" — had
already happened, in twelve currently-published crates, and the reason is
worse than F-98 alone: no CI job in this repository ever ran the improved
script F-98 called for.** Severity: **High** — this is the gate
`agents/release.md` calls "the gate that matters most," found not enforcing
itself, on crates already on crates.io.

**Found continuing the CI-watch backlog after F-101**, implementing F-98's
own deferred fix (compare the *normalized* `Cargo.toml`, not just
`Cargo.toml.orig`, as parsed TOML rather than diffed text so cargo's cosmetic
reordering/requoting isn't mistaken for content). Two things surfaced
immediately on running the improved script for real, neither of which F-98
anticipated:

1. **No workflow in this repository calls `scripts/check-published-match.sh`
   — not once, anywhere.** `grep -rl check-published-match.sh .github/`
   returns nothing. Each of the six database ports instead carries its own
   `published-versions` job with an inline, independently-duplicated
   ~20-line check — and that inline check diffs `src/` only, never looking
   at `Cargo.toml` (normalized or `.orig`) at all: a strictly bigger blind
   spot than F-98's, on the exact same gate, in every port's own hosted CI.
   `fhir-ci.yml` has an equivalent inline job with the identical `src/`-only
   defect. `fhir-loco-ci.yml` and `fhir-store-ci.yml` have no such job at
   all — not a narrower one, none. The thorough script has existed since
   F-35 and has apparently never been run as part of any push.
2. **Running it for real found twelve already-published crates whose
   published version does not match the source that claims it today**,
   across both families:

   | Crate | Published | What moved since | Caught by |
   | --- | --- | --- | --- |
   | `fhir-mariadb-store` | 0.6.0 | `hmac` 0.12→0.13 (workspace) | normalized `Cargo.toml` only (F-98's exact case) |
   | `fhir-mssql-store` | 0.6.0 | `hmac` 0.12→0.13, `sha3` 0.10→0.12 (workspace) | normalized `Cargo.toml` only |
   | `fhir-mysql-map`/`-gen`/`-store` | 0.6.0 | `sha2` 0.10→0.11 (workspace) | normalized `Cargo.toml` only |
   | `fhir-postgresql-store` | 0.6.1 | this session's own F-99 fix (`tests/audit.rs`, packaged) + `getrandom` 0.3→0.4 (workspace, via the F-99/PR#59 `deadpool-postgres` bump) | packaged files **and** manifest — this one predates neither F-98 nor F-102, it is *today's* |
   | `fhir-loco` | 0.3.1 | `rstest` 0.25→0.26 (direct dependency) + `AGENTS.md`/`CLAUDE.md` added | already visible via `Cargo.toml.orig` — this one only ever needed the existing script *run* |
   | `fhir-store` | 0.3.0 | `AGENTS.md`/`CLAUDE.md` added, README's own version example left at `"0.1"` | packaged files only |
   | `fhir-core` | 3.3.0 | `convert_case` 0.11→0.12 (direct dependency) | already visible via `Cargo.toml.orig` |
   | `fhir-r5`, `fhir-r6` | 4.2.0 | `convert_case` 0.11→0.12 (direct dependency) | already visible via `Cargo.toml.orig` |
   | `fhir` | 4.2.0 | `convert_case` 0.11→0.12 (direct dependency) | already visible via `Cargo.toml.orig` |

   Five of these (`fhir-loco`, `fhir-core`, `fhir-r5`, `fhir-r6`, `fhir`) were
   never actually invisible to the *existing* script — `Cargo.toml.orig`
   already carries a direct (non-workspace) dependency bump. They were
   invisible to **CI**, because nothing runs the script. The other seven are
   F-98's exact predicted case: a workspace-inherited version moved and
   `Cargo.toml.orig` still reads `foo.workspace = true`, unchanged.
   `fhir-r2`/`r3`/`r4`/`r4b` and the four `-map`/`-gen` crates not listed
   above were checked and are genuinely unaffected — this is not "everything
   published is wrong," it is these twelve specifically, with evidence for
   each.

**Fixed:**

- `scripts/check-published-match.sh`: added the normalized-`Cargo.toml`
  comparison F-98 deferred (`tomllib`-parsed dict equality, not a text diff —
  verified against a synthetic cosmetic-reorder case, which correctly reports
  a match, and a synthetic version-change case, which correctly reports a
  mismatch, before trusting it against real crates.io data). Also gained a
  `<name>...` argument so a caller can scope it to one family's own crates
  by exact name (a plain prefix was tried first and rejected: "fhir" as a
  prefix would also match "fhir-postgresql" and everything else here, since
  every crate in this repository is named "fhir-something").
- All six ports' `published-versions` jobs, `fhir-ci.yml`'s equivalent, and
  two new jobs in `fhir-loco-ci.yml` and `fhir-store-ci.yml` (which had none)
  now call the shared script instead of an inline, independently-duplicated,
  narrower check. Verified locally by simulating each job's exact
  invocation from its own `working-directory` default before trusting the
  YAML: `cd fhir-postgresql && ../scripts/check-published-match.sh --diff
  fhir-postgresql-map fhir-postgresql-gen fhir-postgresql-store` and the
  equivalent for every other family, each reproducing the same result the
  full run found. `python3 -c "import yaml; yaml.safe_load(...)"` on every
  edited workflow file.

**Remediation of the twelve live violations — versions bumped, per
`agents/release.md` §§1–4, under the 2026-09-02 release-readiness
delegation (`GOVERNANCE.md`, `AI_STATEMENT.md` §§5–6):**

| Crate(s) | New version | Verified |
| --- | --- | --- |
| `fhir-postgresql-map`/`-gen`/`-store` | 0.6.2 | fmt/clippy/deny clean; full live suite (26 tests) re-run against PostgreSQL 18 |
| `fhir-mysql-map`/`-gen`/`-store` | 0.6.1 | same, live suite (27 tests) against MySQL 8.4, TLS tests executing |
| `fhir-mariadb-map`/`-gen`/`-store` | 0.6.1 | same, live suite (27 tests) against MariaDB 11.4, TLS tests executing |
| `fhir-mssql-map`/`-gen`/`-store` | 0.6.1 | same, live suite (40 tests) freshly re-run against `azure-sql-edge` (this host's arm64 substitute — `mcr.microsoft.com/mssql/server` segfaults under emulation here) |
| `fhir-loco` | 0.3.2 | fmt/clippy clean, full test suite (41 tests) |
| `fhir-store` | 0.3.1 | fmt/clippy/deny clean, full test suite (14 tests) |
| `fhir-core` | 3.3.1 | fmt/clippy clean, 45 unit + 13 doctests |
| `fhir-r5`, `fhir-r6` | 4.2.1 | fmt/clippy clean, 816 and 861 tests respectively |
| `fhir` | 4.2.1 | fmt/clippy clean, full suite with `r2 r3 r4 r4b r5 r6` features |

`check-published-match.sh` re-run after each bump: correct vacuous OK
(ahead of the published version, nothing to compare yet) in every case.

**A thirteenth thing found finishing this, not part of the original
twelve:** bumping `fhir-store` broke `cargo check --locked` in every one of
the six ports plus `fhir-loco` — each embeds `fhir-store` as a path
dependency and their own `Cargo.lock` still pointed at 0.3.0. Caught live
on hosted CI (`fhir-mssql`/`fhir-oracle`/`fhir-sqlite`/`fhir-loco`/
`fhir-postgresql` CI all failed "cannot update the lock file ... because
--locked was passed"), not locally first — the same class of gap
`fhir-loco`'s own 0.3.1 changelog entry had already named ("a sibling
workspace's lockfile also needs regenerating" — dependabot cannot see it,
and neither, this time, did the session that made the change, until CI
said so). Fixed by regenerating all seven `Cargo.lock` files; `fhir-sqlite`
and `fhir-oracle` needed no version bump of their own, since only
`Cargo.lock` moved — dependency-resolution drift, which
`check-published-match.sh` already and correctly excludes from
comparison, not source drift.

**CLOSED IN FULL, 2026-09-02.** Hosted CI confirmed green on the final
state (every port's own CI, `repository gates`, both security-audit
workflows, and `fhir CI`'s `Publish dry-run` job all passed on the commit
carrying the lockfile fix). All eighteen affected crates then published to
crates.io in dependency order (`fhir-store`; `fhir-core` → `fhir-r5`,
`fhir-r6` → `fhir`; each port's `map` → `gen` → `store`; `fhir-loco` last),
under the 2026-09-02 release-readiness delegation. `check-published-match.sh`
re-run against the live registry immediately after, with no filter — the
whole tree, not a scoped subset — reports:

```
34 matched, 0 mismatched, 0 skipped.
OK: every published version matches its source (34 compared).
```

Ten annotated, signed tags pushed (`fhir-store-v0.3.1`, `fhir-core-v3.3.1`,
`fhir-r5-v4.2.1`, `fhir-r6-v4.2.1`, `fhir-v4.2.1`, `fhir-postgresql-v0.6.2`,
`fhir-mysql-v0.6.1`, `fhir-mariadb-v0.6.1`, `fhir-mssql-v0.6.1`,
`fhir-loco-v0.3.2`) with matching GitHub Releases. Do not reopen for a new
divergence found later — that is a new finding, per this register's own
convention (F-90's close-out already establishes it).

---

Part of the [fhir-databases specification](index.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
