# fhir-mysql specification

**Engine:** MySQL 8.4 (floor 8.0, InnoDB, `utf8mb4`) · **Conformance level:**
Store (`C0.8`)

Normative behaviour for this port is the monorepo core, plus this port's
departures.

- **The core** — [`../../spec/index.md`](../../spec/index.md). Sections 0–16,
  engine-neutral, shared by all six ports. Requirement ids (`M3.16b`, `PR12.6`,
  `T11.12`) mean the same thing here as anywhere.
- **This port's departures** — [`14-mysql-dialect.md`](14-mysql-dialect.md),
  numbered `M14.x`. **Status: proposed** (`X15.9`), so it MUST NOT be cited as
  evidence for a conformance level until ratified.

A departure is normative only where it amends a core requirement **by number**
(precedence rule 2). Nothing in this port's `README.md`, `book/`, `plan.md`, or
`tasks.md` is normative. Contributor guidance: [`../../AGENTS.md`](../../AGENTS.md).

> **Note on the annex's preamble.** It opens "Sections 1–13 were inherited
> verbatim from the PostgreSQL original and still describe PostgreSQL". That is
> no longer the case: the core has been consolidated and rewritten in
> engine-neutral terms. Its `M14.x` departures stand.

## Relationship to fhir-mariadb

`fhir-mariadb` began as a fork of this port and the two are **independent**
(`M14.0a`–`M14.0c`). Neither is required to read the other's schema, and neither
should hold back syntax the other lacks. What they MUST continue to share is
*behaviour*, which is the core plus `X15.1`, `X15.4`, and `X15.5` — not SQL.

The two do differ concretely where it matters: `TextC` binds to
`utf8mb4_0900_bin` here and `utf8mb4_nopad_bin` on MariaDB, which are the two
engines' spellings of the same NO PAD binary property `M3.6b` requires.

## What this port departs on

| `X15.6` item | MySQL |
| --- | --- |
| Engine floor (`S1.4`) | 8.0 — window functions, `LATERAL`, `JSON_TABLE`, `SIGNAL` |
| Namespace (`S1.2`) | one database per version (`CREATE DATABASE r5`) |
| `ColTy` binding (`M3.6`) | `TINYINT(1)`, `INT`, `BIGINT`, `TEXT`, `TEXT COLLATE utf8mb4_0900_bin`, `DATE`, `DATETIME(6)`, `LONGTEXT` |
| `Numeric` (`M3.6a`) | `TEXT` — `DECIMAL(65,30)` returns `1.50` as `1.500000000000000000000000000000` |
| `Timestamptz` | `DATETIME(6)`, not `TIMESTAMP`: `TIMESTAMP` converts on session time zone and its range ends in 2038 |
| `Jsonb` (`M3.6c`) | `LONGTEXT`, not `JSON`: a `JSON` column re-normalizes, so the bytes read back would not be the bytes signed |
| `ords` binding (`M3.4a`) | `TEXT` holding the shared array literal |
| Index limits (`P6.4a`) | InnoDB key-length cap; see the annex's index section |
| Unmet core requirements | `O10.4a` (no `upgrade`), `M3.16c`/`M3.16d`, `transact_audited`, conditional operations |

## Open findings against this port

- **F-15** — no `upgrade`, so the corrected fold is a full reload here.
- **F-01** — the README carries the PostgreSQL reference's measured results.
- **F-02**, **F-11** — shared; see the [register](../../spec/audit.md).

## Contents of the core

| | Section | Prefix |
| --- | --- | --- |
| 0 | [Conformance](../../spec/00-conformance.md) | `C0.x` |
| 1 | [Scope](../../spec/01-scope.md) | `S1.x` |
| 2 | [Schema generation](../../spec/02-schema-generation.md) | `G2.x` |
| 3 | [Storage model](../../spec/03-storage-model.md) | `M3.x` |
| 4 | [Shredding and reconstruction](../../spec/04-shredding-and-reconstruction.md) | `R4.x` |
| 5 | [Versioning and history](../../spec/05-versioning-and-history.md) | `H5.x` |
| 6 | [Search](../../spec/06-search.md) | `P6.x` |
| 9 | [Validation](../../spec/09-validation.md) | `V9.x` |
| 10 | [Operations](../../spec/10-operations.md) | `O10.x` |
| 11 | [Conformance testing](../../spec/11-conformance-testing.md) | `T11.x` |
| 12 | [Trust, principal, and audit](../../spec/12-trust-principal-and-audit.md) | `PR12.x` |
| 13 | [Compliance mapping](../../spec/13-compliance-mapping.md) | — |
| **14** | [**MySQL dialect**](14-mysql-dialect.md) | `M14.x` |
| 15 | [Portability and dialects](../../spec/15-portability-and-dialects.md) | `X15.x` |
| 16 | [Repository and release](../../spec/16-repository-and-release.md) | `W16.x` |
| — | [Locale and accent folding](../../spec/locale-accent-folding.md) | `Lx` |

Sections 7 and 8 are retired (`C0.15`).
