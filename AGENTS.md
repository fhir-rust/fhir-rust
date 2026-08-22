# Working in this repository

Operational guidance for contributors, human and agent. This file says **how to
work**; [`spec/`](spec/index.md) says **what must be true**. When they seem to
conflict, the spec wins and this file is wrong.

Read this file first, then the one topic file in [`agents/`](agents/) that
covers what you are about to touch.

## What this is

A FHIR monorepo with **four families**. They share a domain and this file's
discipline; they do not share requirement numbers, release cadence, or a
conformance model.

| Family | Directory | Its spec | Its own guide |
| --- | --- | --- | --- |
| Model | [`fhir/`](fhir/) | [`fhir/spec/`](fhir/spec/index.md), ids `R1.x`–`R14.x` | [`fhir/AGENTS.md`](fhir/AGENTS.md) |
| Persistence core | [`fhir-store/`](fhir-store/) | [`spec/databases/`](spec/databases/index.md) — `M3.16`, `PR12.x` | **this file** |
| Databases | `fhir-<engine>/` ×6 | [`spec/databases/`](spec/databases/index.md) | **this file** |
| HTTP surface | [`fhir-loco/`](fhir-loco/) | [`fhir-loco/spec/`](fhir-loco/spec/index.md), ids `SV1.x`–`SV4.x` | **this file** | |

They stack downward only: the model knows nothing about databases, the
persistence core links no driver, the ports carry no HTTP or CLI, and the server
adds only status codes. Nothing lower may
depend on something higher, and **no family's requirements bind another** — see
[`spec/index.md`](spec/index.md) for the precedence rule.

**Before citing `R4.x`, read [the collision
note](spec/index.md#the-r4-collision--read-this-before-citing-r4x).**
`R4.1`–`R4.7` exist in both the model and database specs and mean unrelated
things. Qualify the citation (`db:R4.2`, `model:R4.2`); neither family may
renumber (`C0.5`).

The rest of this file is about the **database family**, the largest of the
three. For the model crate work from [`fhir/AGENTS.md`](fhir/AGENTS.md) and
[`fhir/agents/`](fhir/agents/architecture.md) instead.

## The database ports

Six libraries. Each stores FHIR R3, R4, and R5 resources as **real relational
tables** — typed columns, child tables, constraints — generated from the FHIR
specification itself, and gives them back losslessly.

```
fhir-postgresql   PostgreSQL 18    Reference — full store, full test suite
fhir-sqlite       SQLite 3         Store
fhir-mysql        MySQL 8.4        Store
fhir-mariadb      MariaDB 11.4     Store
fhir-mssql        SQL Server       Store — live-verified, incl. upgrade (F-65)
fhir-oracle       Oracle           Store — live-verified incl. upgrade (F-15); R4.5 open (F-68)
```

Those are conformance levels (`C0.8`), and they are load-bearing: what a port is
allowed to claim depends on which one it has earned. The
[conformance matrix](spec/databases/conformance-matrix.md) is the detail.

Each port is a **self-contained Cargo workspace** of three crates:

| Crate | Contains | Dialect-specific? |
| --- | --- | --- |
| `fhir-<engine>-map` | the relational map, shred, reconstruct, fold, canon, **ddl** | only `ddl.rs` |
| `fhir-<engine>-gen` | FHIR spec packages → map + DDL | no |
| `fhir-<engine>-store` | driver, transactions, search-SQL builder | yes |

Each store crate also **depends on [`fhir-store`](fhir-store/)** for the
engine-agnostic half — the audit chain, `Audit`, `AccessRecord`, and the result
types — and re-exports it, so `fhir_sqlite_store::Audit` still resolves. That is
one crate rather than six copies: `chain.rs` alone was 618 lines byte-identical
in all six, and the shared-core gate did not watch it (**F-45**).

A consequence worth knowing: a change to the shared half now needs a
`fhir-store` release before a port can take it. That version coupling is the
price of the duplication being impossible rather than merely visible.

There is **no server crate and no CLI crate** in any port (`C0.17`, `C0.18`).
The REST server is a separate crate, [`fhir-loco`](fhir-loco/) — Loco.rs, Axum,
Tokio, Hyper — mounted over `fhir-sqlite`. Endpoint work goes there, not into a
port; a port's job is the store API the server calls.

That is settled, and it settled **F-27** class 1: the ports' REST milestones
were misattributed rather than unfinished.

## The five rules

1. **The spec is one copy, at the root.** `/spec/databases` holds every
   normative requirement that is not about a specific SQL dialect. Do not copy a
   section into a port (`W16.5`) — that duplication is what this revision
   removed.
2. **Change shared code in every port, in one commit** (`W16.7`). The pure-Rust
   core — `model.rs`, `shred.rs`, `reconstruct.rs`, `value.rs`, `fold.rs`,
   `canon.rs`, `error.rs`, and all of `gen/` **including its tests** — is
   identical across all six ports and must stay that way (`X15.1`). Check with
   `./scripts/check-shared-core.sh` — 100 files. It compares tokens rather than
   lines (`X15.1a`), because rustfmt wraps by crate-name length and a line-based
   gate reports that as a divergence nobody can fix.
   **Run it yourself before pushing.** `.github/workflows/gates.yml` at the
   root runs this and `scripts/check-doc-examples.sh` in CI, and since the
   F-49 consolidation (2026-08-06) every family's CI lives beside it at the
   root too — `fhir-ci.yml`, one `<port>-ci.yml` per port, `fhir-loco-ci.yml`,
   `fhir-store-ci.yml` — each path-filtered to its family.
3. **A dialect difference goes in the annex, by number** (`C0.12`). If the port
   cannot do what the core requires, write an `M14.x` departure that names the
   requirement it amends. An undeclared departure is a defect, not an
   amendment.
4. **Do not claim above the port's level** (`C0.11`). All six READMEs did until
   2026-07-31 (**F-01**); the per-port books still do.
5. **Say what you did not verify.** A skipped test, an unset DSN, an untried
   engine — all of it goes in the commit message and, if it persists, in
   [`spec/audit.md`](spec/databases/audit.md). `T11.12` exists because a silent skip reads
   as a pass.

## Where things live

| You want | Go to |
| --- | --- |
| Which spec governs what | [`spec/index.md`](spec/index.md) |
| What must be true of a port | [`spec/databases/index.md`](spec/databases/index.md) |
| What must be true of the model crate | [`fhir/spec/index.md`](fhir/spec/index.md) |
| How ports may differ | [`spec/databases/15-portability-and-dialects.md`](spec/databases/15-portability-and-dialects.md) |
| What is currently broken | [`spec/audit.md`](spec/databases/audit.md) |
| Which port does what | [`spec/conformance-matrix.md`](spec/databases/conformance-matrix.md) |
| Tutorials and examples | [`doc/`](doc/index.md) |
| A port's design decisions | `fhir-<engine>/plan.md` |
| A port's work breakdown | `fhir-<engine>/tasks.md` |
| A port's dialect departures | `fhir-<engine>/spec/14-<engine>-dialect.md` |

## Topic guides

Read the one that matches your change.

- **[agents/spec-workflow.md](agents/spec-workflow.md)** — specification-driven
  development: how a change moves spec → tasks → code → test, and how to amend
  a requirement.
- **[agents/rust.md](agents/rust.md)** — code conventions, the shared-core rule,
  error handling, comment style.
- **[agents/testing.md](agents/testing.md)** — the test taxonomy, live database
  suites, fuzzing, and the rules against decorative tests.
- **[agents/databases.md](agents/databases.md)** — the six engines, running one
  locally, and what porting to a seventh involves.
- **[agents/documentation.md](agents/documentation.md)** — README, book, and
  `doc/` conventions, and the substitution trap that produced **F-01**.
- **[agents/security.md](agents/security.md)** — PHI, the audit chain, keys, the
  trust boundary, and what never goes in a log.
- **[agents/release.md](agents/release.md)** — versioning, supply-chain gates,
  and why a published version must match its source.

## Before you start

```sh
cd fhir-postgresql              # or any port
cargo build
cargo test                      # passes with no database — see below
scripts/db.sh up                # start the port's engine in a container
scripts/db.sh test              # run the live suite against it
```

`cargo test` passing means little on its own. Most of what these libraries
guarantee is a database guarantee — snapshot isolation, row locks, the
append-only trigger, index-using search plans — and none of it is exercised
without a server. The live suite is the real gate.

Every live test in the five server ports finds its own server: the port's
`*_TEST_DSN` when set, otherwise the `scripts/db.sh` container if it is
listening. `./scripts/db.sh up` and then a plain `cargo test` is the whole
local workflow, and `FHIR_<PORT>_REQUIRE_DB=1` — which every live CI job now
sets — turns a skip into a failure, so a job that reached no server is red
rather than green (`T11.12`, `T11.13`). See [`agents/testing.md`](agents/testing.md).

## Commit conventions

- One logical change per commit. A shared-core change touching six ports is one
  logical change (rule 2), not six commits.
- Reference requirement ids: `fix(sqlite): fold NFD before lowercase (L4)`.
- If a commit amends the spec to match code, say which port's behaviour drove it
  (`C0.22`). A silent generalization and a rubber stamp look identical
  afterwards.
- If a commit closes an audit finding, say so: `closes F-07`.

## What not to do

- Do not "fix" a divergence between two ports by making one match the other
  without checking which is right. Both may be wrong; the core says which.
- Do not text-substitute an engine name through a document. That is exactly how
  **F-01**, **F-08**, and **F-16** happened — six READMEs, a `ddl.rs`, and two
  dialect annexes describing engines they did not target. All are now fixed,
  including `fhir-oracle`'s `ddl.rs` and every port's `book/` (rewritten from
  copy-substituted templates to source-verified content 2026-08-04) — but the
  failure mode recurs easily, so keep checking for it rather than assuming
  it is settled forever.
- Do not add a test that cannot fail. Verify it by mutation (`T11.10`).
- Do not delete an audit finding because the text that stated it was rewritten.
  Findings close when they are fixed.
- Ask before pushing — but not for either reason this line used to give. The
  six ports no longer have their own `origin`s (**F-11** is resolved: they are
  directories in one repository, remote `fhir-rust/fhir-rust`). The
  nested-repository warning about `fhir-store/` (**F-37**) is also resolved,
  and for a more confusing reason than F-11: F-37 was about a *different*
  directory that used to carry this name — the HTTP surface, since renamed
  `fhir-loco/` (**F-45**) — not the persistence-core crate that holds the
  name today, which has never had a nested `.git` (verified 2026-08-04,
  **F-72**). Ask anyway; it is still someone else's decision when to push.
