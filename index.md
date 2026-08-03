# fhir-rust — documentation index

Every entry point in the repository, in one place. If you know what you want,
this is the fastest route; if you do not, start with
[`README.md`](README.md).

The repository holds four families: the **model** crate ([`fhir/`](fhir/)), the
**persistence core** ([`fhir-store/`](fhir-store/)), the six **database** ports
(`fhir-<engine>/`), and the **HTTP surface** ([`fhir-loco/`](fhir-loco/)). Most
of this page is about the database ports, which are the largest; the other three
have their own sections at the end. [`spec/index.md`](spec/index.md) is the root of every specification and
says which one governs which code.

## By what you are doing

### Evaluating

| | |
| --- | --- |
| [README](README.md) | what this is, in five minutes |
| [Choosing an engine](doc/choosing-an-engine.md) | which of the six, and what each costs |
| [Conformance matrix](spec/databases/conformance-matrix.md) | what each port actually satisfies today |
| [The storage model](doc/storage-model.md) | how a FHIR resource becomes tables |
| [Trust boundary](doc/trust-boundary.md) | what is guaranteed, what your deployment must add |
| [FAQ](doc/faq.md) | the questions that come up first |

### Building something

| | |
| --- | --- |
| [Tutorial 1 — first store](doc/tutorial-01-getting-started.md) | install a schema, write and read a resource |
| [Tutorial 2 — the schema](doc/tutorial-02-storage-model.md) | base tables, child tables, `ords`, extensions |
| [Tutorial 3 — SQL](doc/tutorial-03-querying-sql.md) | query the relational schema directly |
| [Tutorial 4 — FHIR search](doc/tutorial-04-search.md) | search parameters, modifiers, the fold |
| [Tutorial 5 — history and audit](doc/tutorial-05-history-and-audit.md) | versions, attribution, the hash chain |
| [Tutorial 6 — porting](doc/tutorial-06-porting.md) | adding a seventh database |
| [Examples](doc/examples.md) | short, runnable recipes |

### Contributing

| | |
| --- | --- |
| [AGENTS.md](AGENTS.md) | how to work here — start here |
| [Topic guides](AGENTS/index.md) | spec workflow, rust, testing, databases, docs, security, release |
| [CLAUDE.md](CLAUDE.md) | agent-specific notes and traps |
| [Audit findings](spec/databases/audit.md) | what is currently broken, with evidence |

### Implementing or auditing

| | |
| --- | --- |
| [Specification root](spec/index.md) | all four families, precedence, id namespaces |
| [Publishing readiness](spec/publishing.md) | what blocks all 33 crates from crates.io |
| [Database core](spec/databases/index.md) | the normative core, sections 0–16 |
| [Compliance mapping](spec/databases/13-compliance-mapping.md) | regulation → requirement → evidence |
| [Conformance matrix](spec/databases/conformance-matrix.md) | per-port status |

## The database specification

One copy, shared by all six ports. Requirement ids are permanent (`C0.5`).

| | Section | Prefix | Subject |
| --- | --- | --- | --- |
| 0 | [Conformance](spec/databases/00-conformance.md) | `C0.x` | keywords, id grammar, levels, retired sections |
| 1 | [Scope](spec/databases/01-scope.md) | `S1.x` | FHIR versions, resource coverage, engine floors |
| 2 | [Schema generation](spec/databases/02-schema-generation.md) | `G2.x` | determinism, identifiers, install |
| 3 | [Storage model](spec/databases/03-storage-model.md) | `M3.x` | tables, types, extensions, audit, hash chain |
| 4 | [Shredding and reconstruction](spec/databases/04-shredding-and-reconstruction.md) | `R4.x` | lossless round-trip, snapshot reads |
| 5 | [Versioning and history](spec/databases/05-versioning-and-history.md) | `H5.x` | versions, soft delete, vread |
| 6 | [Search](spec/databases/06-search.md) | `P6.x` | parameters, folding, indexes, bounded cost |
| 7 | — | — | *retired: REST API* |
| 8 | — | — | *retired: CLI* |
| 9 | [Validation](spec/databases/09-validation.md) | `V9.x` | structural, strict, terminology gap |
| 10 | [Operations](spec/databases/10-operations.md) | `O10.x` | logging, migrations, TLS, supply chain |
| 11 | [Conformance testing](spec/databases/11-conformance-testing.md) | `T11.x` | what must be tested, and how honestly |
| 12 | [Trust, principal, audit](spec/databases/12-trust-principal-and-audit.md) | `PR12.x` | attribution, disclosure logging |
| 13 | [Compliance mapping](spec/databases/13-compliance-mapping.md) | — | HIPAA, GDPR, ONC, IEC 62304 |
| 14 | *per port* | `M14.x` | that engine's departures |
| 15 | [Portability and dialects](spec/databases/15-portability-and-dialects.md) | `X15.x` | what is shared, what an annex must say |
| 16 | [Repository and release](spec/databases/16-repository-and-release.md) | `W16.x` | layout, SSOT, versioning |
| — | [Locale and accent folding](spec/databases/locale-accent-folding.md) | `L1`–`L16` | the fold, normatively |
| — | [Search adjuncts](spec/databases/unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) | `U1`–`U13` | bounded and checksum adjuncts for any unindexable search target |

Non-normative companions: [conformance matrix](spec/databases/conformance-matrix.md) ·
[audit findings](spec/databases/audit.md).

`R4.x` is the one prefix this specification shares with the model crate's, where
it means something unrelated. [`spec/index.md`](spec/index.md#the-r4-collision--read-this-before-citing-r4x)
has the disambiguation rule.

## The ports

Each has its own index, dialect annex, book, and work breakdown.

| Port | Spec | Guide | Book | Plan | Tasks |
| --- | --- | --- | --- | --- | --- |
| `fhir-postgresql` | [spec](fhir-postgresql/spec/index.md) · [annex](fhir-postgresql/spec/14-postgresql-dialect.md) | [AGENTS](fhir-postgresql/AGENTS.md) | [book](fhir-postgresql/book/src/SUMMARY.md) | [plan](fhir-postgresql/plan.md) | [tasks](fhir-postgresql/tasks.md) |
| `fhir-sqlite` | [spec](fhir-sqlite/spec/index.md) · [annex](fhir-sqlite/spec/14-sqlite-dialect.md) | [AGENTS](fhir-sqlite/AGENTS.md) | [book](fhir-sqlite/book/src/SUMMARY.md) | [plan](fhir-sqlite/plan.md) | [tasks](fhir-sqlite/tasks.md) |
| `fhir-mysql` | [spec](fhir-mysql/spec/index.md) · [annex](fhir-mysql/spec/14-mysql-dialect.md) | [AGENTS](fhir-mysql/AGENTS.md) | [book](fhir-mysql/book/src/SUMMARY.md) | [plan](fhir-mysql/plan.md) | [tasks](fhir-mysql/tasks.md) |
| `fhir-mariadb` | [spec](fhir-mariadb/spec/index.md) · [annex](fhir-mariadb/spec/14-mariadb-dialect.md) | [AGENTS](fhir-mariadb/AGENTS.md) | [book](fhir-mariadb/book/src/SUMMARY.md) | [plan](fhir-mariadb/plan.md) | [tasks](fhir-mariadb/tasks.md) |
| `fhir-mssql` | [spec](fhir-mssql/spec/index.md) · [annex](fhir-mssql/spec/14-mssql-dialect.md) | [AGENTS](fhir-mssql/AGENTS.md) | [book](fhir-mssql/book/src/SUMMARY.md) | [plan](fhir-mssql/plan.md) | [tasks](fhir-mssql/tasks.md) |
| `fhir-oracle` | [spec](fhir-oracle/spec/index.md) · [annex](fhir-oracle/spec/14-oracle-dialect.md) | [AGENTS](fhir-oracle/AGENTS.md) | [book](fhir-oracle/book/src/SUMMARY.md) | [plan](fhir-oracle/plan.md) | [tasks](fhir-oracle/tasks.md) |

All six annexes are now real — the MSSQL and Oracle ones were the MySQL annex
with three lines changed until 2026-07-31, and both were rewritten from the
`X15.6` checklist ([`audit.md`](spec/databases/audit.md) **F-16**). All six are still
marked *proposed* (`X15.9`), so none may be cited as evidence for a conformance
level yet.

## The model crate

[`fhir/`](fhir/) — the FHIR data model and the generator that produces it from
the official specification JSON. No database, no I/O; the database ports do not
depend on it.

| | |
| --- | --- |
| [README](fhir/README.md) | what it is, and how to depend on it |
| [Specification](fhir/spec/index.md) | 14 sections, ids `R1.x`–`R14.x` |
| [Assurance](fhir/spec/13-assurance.md) | what must hold before it is trusted clinically |
| [Cross-release conversion](fhir/spec/14-cross-release-conversion.md) | moving a resource between releases, and what that costs |
| [AGENTS](fhir/AGENTS.md) · [topic guides](fhir/AGENTS/architecture.md) | how to work in it |
| [CHANGELOG](fhir/CHANGELOG.md) | |

Its `R4.x` ids are **not** the database specification's `R4.x`; see
[`spec/index.md`](spec/index.md#the-r4-collision--read-this-before-citing-r4x).

## The persistence core

[`fhir-store/`](fhir-store/) — the engine-agnostic half of storage: the
tamper-evident audit chain (`M3.16`), the attribution and disclosure records
(`PR12.x`), and the value types every port's operations return. No driver, no
socket.

| | |
| --- | --- |
| [README](fhir-store/README.md) | what is in it, and what stays in a port |
| Governed by | [`spec/databases/`](spec/databases/index.md) — it is the ports' own core, not a separate spec |

It exists because `chain.rs` was 618 lines byte-identical in all six ports and
the shared-core gate did not watch it (**F-45**).

## The HTTP surface

[`fhir-loco/`](fhir-loco/) — a FHIR RESTful API server (Axum, Loco) over
`fhir-sqlite`. The database ports carry no server of their own by design
(`C0.17`), and this is where that surface lives.

| | |
| --- | --- |
| [README](fhir-loco/README.md) | endpoints, running it, what belongs where |
| [tasks](fhir-loco/tasks.md) | its work breakdown |
| Specification | [`fhir-loco/spec/index.md`](fhir-loco/spec/index.md) — 4 sections, ids `SV1.x`–`SV4.x` |

## Reading order, if you have an hour

1. [README](README.md) — 5 min
2. [Storage model](doc/storage-model.md) — 15 min, the one idea everything rests on
3. [Tutorial 1](doc/tutorial-01-getting-started.md) — 15 min, hands on
4. [Conformance matrix](spec/databases/conformance-matrix.md) — 10 min, what is actually true
5. [Audit findings](spec/databases/audit.md) — 15 min, what is not
