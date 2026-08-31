# The fhir-databases specification

This is the **normative core** for every database port in this monorepo: one
copy of every requirement that is not about a particular SQL dialect, cited by
stable number, implemented six times and verified six times.

It is one of the repository's specification families, not the whole of it —
[`spec/index.md`](../index.md) is the root index, and it is the one to read
first if you are not certain which family governs the code you are changing. It
also records the one requirement-id prefix (`R4`) that this specification shares
with another family, and how to cite it unambiguously.

Requirements use RFC 2119 keywords (MUST, SHOULD, MAY) as defined in
[§0 Conformance](00-conformance.md).

Behaviour is defined here first, then implemented and verified. When code and
spec disagree, reconcile them — do not let them drift. Operational guidance for
contributors lives in [`AGENTS.md`](../../AGENTS.md); this directory defines
**what must be true**, not how to work.

## Why one core

Sections 1–13 were, until this revision, copied into all six database projects.
The copies were byte-identical apart from the product name — verified by diff,
0–2 differing lines per file across 78 files — which is duplication with no
compensating benefit and an obvious failure mode: a requirement amended in one
port and not the other five. This directory holds the one copy. A port states
only where it *departs*, in its own dialect annex, and a departure has to be
written down to exist.

```
spec/
  index.md                       the monorepo root index: all three families
  databases/                     the normative core — this directory
    00-conformance.md ..  16-repository-and-release.md
    locale-accent-folding.md
    unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md

fhir-postgresql/spec/index.md    port index: links this core
fhir-sqlite/spec/
  index.md                       port index
  14-sqlite-dialect.md           the port's own departures (M14.x)
```

## Precedence

1. The core (this directory) is normative for every port.
2. A port's dialect annex is normative **only where it explicitly amends a core
   requirement by number**. `M14.6 amends M3.4` is a departure; prose that
   merely differs is not.
3. Where a port has no annex text on a subject, the core governs unmodified.
4. Nothing in a README, book chapter, `plan.md`, `tasks.md`, or code comment is
   normative. Those describe; this decides.

An undeclared departure is a defect in the port, not an amendment to the core.
See [§15 Portability and dialects](15-portability-and-dialects.md) for what an
annex must contain.

## Contents

### Framework

- **0.** [Conformance](00-conformance.md) — normative language, requirement-id
  grammar, conformance profiles, retired sections, how to amend.

### The core requirements

- **1.** [Scope](01-scope.md) — `S1.x`
- **2.** [Schema generation](02-schema-generation.md) — `G2.x`
- **3.** [Storage model](03-storage-model.md) — `M3.x`
- **4.** [Shredding and reconstruction](04-shredding-and-reconstruction.md) — `R4.x`
- **5.** [Versioning and history](05-versioning-and-history.md) — `H5.x`
- **6.** [Search](06-search.md) — `P6.x`
- **9.** [Validation](09-validation.md) — `V9.x`
- **10.** [Operations](10-operations.md) — `O10.x`
- **11.** [Conformance testing](11-conformance-testing.md) — `T11.x`
- **12.** [Trust, principal, and audit](12-trust-principal-and-audit.md) — `PR12.x`
- **13.** [Compliance mapping](13-compliance-mapping.md) — table, not requirements

Section **14** is not in this directory by design: it is the per-port dialect
annex (`M14.x`), one `spec/14-<engine>-dialect.md` in each port — see
[§15](15-portability-and-dialects.md) `X15.6` for what an annex must contain.

Sections **7** (REST API) and **8** (CLI) are retired: these are embeddable
libraries, and neither an HTTP server nor a command-line tool is in scope. The
numbering keeps the gap rather than renumbering, so a requirement id like `M9.2`
still means what it meant. Several retired ids are still cited from §11 and §13;
[§0 Conformance](00-conformance.md#retired-sections) records exactly which, and
[`audit.md`](audit.md) tracks the unresolved references as finding **F-04**.

### Monorepo framework

- **15.** [Portability and dialects](15-portability-and-dialects.md) — `X15.x`.
  What every port shares by construction, what a dialect annex must contain,
  and what cross-engine interoperability means.
- **16.** [Repository and release](16-repository-and-release.md) — `W16.x`.
  Project layout, crate naming, versioning, and what must be identical across
  ports versus what may differ.

### Cross-cutting

- [Unbounded string search: bounded adjunct and checksum
  adjunct](unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md)
  — normative for making any **search-reachable** column searchable where the
  engine cannot index or compare it as bound (`P6.4a`, `P6.9`). Numbered `U<n>`:
  `U1a` generalizes the trigger beyond strings to CLOBs, BLOBs and the
  fixed-shape extension columns, `U2a` matches the adjuncts to the operations a
  search actually performs, `U4a` fixes the digest at SHA-256 in 32 binary
  bytes, and `U11`–`U13` say which columns the generator must walk.
  Unnumbered as a section because it changes the **generated map**, which is
  upstream of every dialect and shared verbatim across all six ports (`X15.1`).
  It is what closes SQL Server's `P6.4a` departure and unblocks the Oracle DDL
  emitter.
- [Locale and accent folding](locale-accent-folding.md) — normative for the fold
  that backs case- and accent-insensitive search (`P6.6`). Unnumbered because it
  cuts across sections rather than sitting between two of them, and shared
  verbatim by every port: the fold is pure Rust, so it is identical across
  dialects by construction (`X15.4`).

### Status, not requirements

- [Conformance matrix](conformance-matrix.md) — which port satisfies which core
  requirement today. Non-normative; it records reality, not intent.
- [Audit](audit.md) — the findings register. Every known divergence between this
  spec, the documentation, and the code, with evidence and a disposition.

## The ports

| Port | Engine | Store status |
| --- | --- | --- |
| [`fhir-postgresql`](../../fhir-postgresql/spec/index.md) | PostgreSQL 18 | reference implementation |
| [`fhir-sqlite`](../../fhir-sqlite/spec/index.md) | SQLite 3 | native store, embeddable |
| [`fhir-mysql`](../../fhir-mysql/spec/index.md) | MySQL 8.4 | native store |
| [`fhir-mariadb`](../../fhir-mariadb/spec/index.md) | MariaDB 11.4 | native store |
| [`fhir-mssql`](../../fhir-mssql/spec/index.md) | SQL Server | native store (**F-65**) |
| [`fhir-oracle`](../../fhir-oracle/spec/index.md) | Oracle Database | native store (**F-68**), `upgrade`/backfill since 2026-08-09 (**F-15**); `R4.5` open |

The [conformance matrix](conformance-matrix.md) is the detailed version of that
last column, and is the one to trust.

## Reading this specification

Each file is a section of one specification, split so that a section can be
read, reviewed, and cited on its own. Requirement numbers are stable across the
split and across the hoist into this directory: `M3.16b` is `M3.16b` wherever it
moved to, and a citation written against a per-project copy still resolves.
