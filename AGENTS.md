# Working in this repository

Operational guidance for contributors, human and agent. This file says **how to
work**; [`spec/`](spec/index.md) says **what must be true**. When they seem to
conflict, the spec wins and this file is wrong.

Read this file first, then the one topic file in [`AGENTS/`](AGENTS/) that
covers what you are about to touch.

## What this is

A monorepo of six FHIR-to-relational database libraries. Each stores FHIR R3,
R4, and R5 resources as **real relational tables** — typed columns, child
tables, constraints — generated from the FHIR specification itself, and gives
them back losslessly.

```
fhir-postgresql   PostgreSQL 18    Reference — full store, full test suite
fhir-sqlite       SQLite 3         Store
fhir-mysql        MySQL 8.4        Store
fhir-mariadb      MariaDB 11.4     Store
fhir-mssql        SQL Server       Scaffold — DDL only, no store
fhir-oracle       Oracle           Scaffold — DDL is still MySQL's
```

Those are conformance levels (`C0.8`), and they are load-bearing: what a port is
allowed to claim depends on which one it has earned. The
[conformance matrix](spec/conformance-matrix.md) is the detail.

Each port is a **self-contained Cargo workspace** of three crates:

| Crate | Contains | Dialect-specific? |
| --- | --- | --- |
| `fhir-<engine>-map` | the relational map, shred, reconstruct, fold, canon, **ddl** | only `ddl.rs` |
| `fhir-<engine>-gen` | FHIR spec packages → map + DDL | no |
| `fhir-<engine>-store` | driver, transactions, search, hash chain | yes |

There is **no server crate and no CLI crate** anywhere, in any port, despite
what the READMEs say. See `C0.17`/`C0.18`.

## The five rules

1. **The spec is one file, at the root.** `/spec` holds every normative
   requirement that is not about a specific SQL dialect. Do not copy a section
   into a port (`W16.5`) — that duplication is what this revision removed.
2. **Change shared code in every port, in one commit** (`W16.7`). The pure-Rust
   core — `model.rs`, `shred.rs`, `reconstruct.rs`, `value.rs`, `fold.rs`,
   `canon.rs`, `error.rs`, and all of `gen/` — is identical across all six ports
   and must stay that way (`X15.1`). Check with
   `./scripts/check-shared-core.sh`; CI runs it too.
3. **A dialect difference goes in the annex, by number** (`C0.12`). If the port
   cannot do what the core requires, write an `M14.x` departure that names the
   requirement it amends. An undeclared departure is a defect, not an
   amendment.
4. **Do not claim above the port's level** (`C0.11`). All six READMEs did until
   2026-07-31 (**F-01**); the per-port books still do.
5. **Say what you did not verify.** A skipped test, an unset DSN, an untried
   engine — all of it goes in the commit message and, if it persists, in
   [`spec/audit.md`](spec/audit.md). `T11.12` exists because a silent skip reads
   as a pass.

## Where things live

| You want | Go to |
| --- | --- |
| What must be true | [`spec/index.md`](spec/index.md) |
| How ports may differ | [`spec/15-portability-and-dialects.md`](spec/15-portability-and-dialects.md) |
| What is currently broken | [`spec/audit.md`](spec/audit.md) |
| Which port does what | [`spec/conformance-matrix.md`](spec/conformance-matrix.md) |
| Tutorials and examples | [`doc/`](doc/index.md) |
| A port's design decisions | `fhir-<engine>/plan.md` |
| A port's work breakdown | `fhir-<engine>/tasks.md` |
| A port's dialect departures | `fhir-<engine>/spec/14-<engine>-dialect.md` |

## Topic guides

Read the one that matches your change.

- **[AGENTS/spec-workflow.md](AGENTS/spec-workflow.md)** — specification-driven
  development: how a change moves spec → tasks → code → test, and how to amend
  a requirement.
- **[AGENTS/rust.md](AGENTS/rust.md)** — code conventions, the shared-core rule,
  error handling, comment style.
- **[AGENTS/testing.md](AGENTS/testing.md)** — the test taxonomy, live database
  suites, fuzzing, and the rules against decorative tests.
- **[AGENTS/databases.md](AGENTS/databases.md)** — the six engines, running one
  locally, and what porting to a seventh involves.
- **[AGENTS/documentation.md](AGENTS/documentation.md)** — README, book, and
  `doc/` conventions, and the substitution trap that produced **F-01**.
- **[AGENTS/security.md](AGENTS/security.md)** — PHI, the audit chain, keys, the
  trust boundary, and what never goes in a log.
- **[AGENTS/release.md](AGENTS/release.md)** — versioning, supply-chain gates,
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

`fhir-mssql` now provisions SQL Server 2022, and its live test **fails rather
than skips** when `FHIR_MSSQL_REQUIRE_DB=1`. `fhir-oracle` has no live gate at
all — deliberately, because it has no Oracle DDL, no driver, and no store, and a
gate against a substitute engine is worse than none (**F-06**).

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
  dialect annexes describing engines they did not target. Three of those are
  fixed; `fhir-oracle`'s `ddl.rs` and every port's `book/` are not.
- Do not add a test that cannot fail. Verify it by mutation (`T11.10`).
- Do not delete an audit finding because the text that stated it was rewritten.
  Findings close when they are fixed.
- Do not push. Every port's `origin` is still the ancestor project's repository
  (**F-11**).
