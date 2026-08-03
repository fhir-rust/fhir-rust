# fhir-postgresql specification

**Engine:** PostgreSQL 18 · **Conformance level:** Reference (`C0.8`)

Normative behaviour for this port is the monorepo core, plus this port's
departures.

- **The core** — [`../../spec/index.md`](../../spec/databases/index.md). Sections 0–16,
  engine-neutral, shared by all six ports. Requirement ids (`M3.16b`, `PR12.6`,
  `T11.12`) mean the same thing here as anywhere.
- **This port's departures** —
  [`14-postgresql-dialect.md`](14-postgresql-dialect.md), numbered `M14.x`.
  **Status: proposed** (`X15.9`), so it MUST NOT be cited as evidence for a
  conformance level until ratified.

Nothing in this port's `README.md`, `book/`, `plan.md`, or `tasks.md` is
normative (`index.md` precedence rule 4). Operational guidance for contributors
is in [`../../AGENTS.md`](../../AGENTS.md).

## The annex, in summary

`fhir-postgresql` is the original: sections 1–13 were written against PostgreSQL
and *were* the specification, so for most of this project's life there was
nothing for an annex to differ from. Consolidating the core removed that
exemption, and the annex now exists (**F-14**, fixed).

What it records, against the `X15.6` checklist:

| `X15.6` item | PostgreSQL |
| --- | --- |
| Engine floor (`S1.4`) | 18 |
| Namespace (`S1.2`) | `CREATE SCHEMA r5` |
| `ColTy` binding (`M3.6`) | `boolean`, `integer`, `bigint`, `numeric`, `text`, `text COLLATE "C"`, `date`, `timestamptz`, `jsonb` |
| `ords` binding (`M3.4a`) | `smallint[]`, the only port with a native array type |
| Install atomicity (`G2.5`) | staged schema `r5__init` in chunked transactions, renamed into place — 7,355 tables exceed `max_locks_per_transaction` in one transaction |
| Snapshot isolation (`R4.5`) | `REPEATABLE READ READ ONLY` |
| Write serialization (`H5.4`) | `SELECT … FOR UPDATE` on the base row before the history append |
| Append-only (`M3.17`) | `BEFORE UPDATE OR DELETE` trigger raising an exception |
| Index limits (`P6.4a`) | none — `text` indexes without a key-length cap |
| Paging / placeholders | `LIMIT`/`OFFSET`, `$n` |
| Transport (`O10.7`) | rustls; `PGSSLMODE`/DSN, `PGSSLROOTCERT` for the anchor. Three effective modes, defaulting to unverified — a departure (`M14.27`) |
| Unmet core requirements | `M3.6c` (`M14.13`), `X15.2` (`M14.12`), `O10.7` default (`M14.27`) |

Writing it surfaced two departures that were invisible while this port defined
the spec: the `jsonb` binding, and a TLS default that does not verify.

## Open findings against this port

- **F-07 (High)** — the chain pre-image is still derived in SQL:
  `(($1::text)::jsonb)::text` at
  `crates/fhir-postgresql-store/src/lib.rs:291`. `fhir-postgresql-map` is the
  only map crate without `canon.rs`. Consequence: a PostgreSQL chain cannot be
  verified by any other port (`X15.11`), and the bytes signed are defined by a
  PostgreSQL version rather than by this specification (`X15.2`). Recorded as
  `M14.12`; fixing it is a chain-format change and needs `M3.16e` treatment.
- **F-17 (Medium)** — `SslPolicy` defaults to `Prefer`, which does not verify
  the server certificate, against `O10.7`. Recorded as `M14.27`. Deployments
  MUST set `PGSSLMODE=verify-full` until the default changes.
- **F-18 (Low)** — `ddl.rs` still emits a `fhir_postgresql_norm(text)` SQL
  function that nothing calls, a residue of the pre-`P6.6` design that `L3`
  prohibits. Recorded as `M14.21`.
- **F-02**, **F-11**, **F-15** — shared with the other ports; see the register.
- **F-14** — no dialect annex. **Fixed:** the annex now exists.

This port is Reference level because its test suite substantiates it:
`concurrency.rs`, `audit.rs`, `redaction.rs`, `upgrade.rs`, `live.rs`,
`m2_semantics.rs`, `search_semantics.rs`, and `bench.rs` run against live
PostgreSQL 18 in CI. It is the only port of which that is true, which is what
the [conformance matrix](../../spec/databases/conformance-matrix.md) is for.

## Contents of the core

| | Section | Prefix |
| --- | --- | --- |
| 0 | [Conformance](../../spec/databases/00-conformance.md) | `C0.x` |
| 1 | [Scope](../../spec/databases/01-scope.md) | `S1.x` |
| 2 | [Schema generation](../../spec/databases/02-schema-generation.md) | `G2.x` |
| 3 | [Storage model](../../spec/databases/03-storage-model.md) | `M3.x` |
| 4 | [Shredding and reconstruction](../../spec/databases/04-shredding-and-reconstruction.md) | `R4.x` |
| 5 | [Versioning and history](../../spec/databases/05-versioning-and-history.md) | `H5.x` |
| 6 | [Search](../../spec/databases/06-search.md) | `P6.x` |
| 9 | [Validation](../../spec/databases/09-validation.md) | `V9.x` |
| 10 | [Operations](../../spec/databases/10-operations.md) | `O10.x` |
| 11 | [Conformance testing](../../spec/databases/11-conformance-testing.md) | `T11.x` |
| 12 | [Trust, principal, and audit](../../spec/databases/12-trust-principal-and-audit.md) | `PR12.x` |
| 13 | [Compliance mapping](../../spec/databases/13-compliance-mapping.md) | — |
| 15 | [Portability and dialects](../../spec/databases/15-portability-and-dialects.md) | `X15.x` |
| 16 | [Repository and release](../../spec/databases/16-repository-and-release.md) | `W16.x` |
| — | [Locale and accent folding](../../spec/databases/locale-accent-folding.md) | `Lx` |

Sections 7 and 8 are retired (`C0.15`).
