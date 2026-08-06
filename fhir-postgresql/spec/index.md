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
| Transport (`O10.7`) | rustls; `PGSSLMODE`/DSN, `PGSSLROOTCERT` for the anchor. Defaults to `Require`, which verifies (**F-17** fixed; `tests/ssl_default.rs`) |
| Unmet core requirements | `M3.6c` (`M14.13`) |

Writing it surfaced two departures that were invisible while this port defined
the spec: the `jsonb` binding, and a TLS default that does not verify.

## Open findings against this port

- **F-07 (High)** — **Fixed:** the chain pre-image is computed in Rust. The
  stored normalized form is canonicalized by `fhir_postgresql_map::canon`
  (`canon_of`, `crates/fhir-postgresql-store/src/lib.rs:238`) and chained by
  the shared `fhir-store::chain`, so the signed bytes are defined by this
  specification, not by a PostgreSQL version (`X15.2`), and
  `tests/chain_portability.rs` proves an outside verifier can recompute a
  chain. It was a chain-format change: a database written before it needs a
  reload, not a migration (`M14.12`).
- **F-17 (Medium)** — **Fixed:** `SslPolicy` now defaults to `Require`, which
  verifies the server certificate, meeting `O10.7`'s verifying default. Pinned
  by `crates/fhir-postgresql-store/tests/ssl_default.rs`; a breaking change,
  recorded in the CHANGELOG (Unreleased). `M14.27` records the history.
- **F-18 (Low)** — `ddl.rs` still emits a `fhir_postgresql_norm(text)` SQL
  function that nothing calls, a residue of the pre-`P6.6` design that `L3`
  prohibits. Recorded as `M14.21`.
- **F-02** — shared with the other ports; see the register. **F-11** is
  resolved (monorepo merge, one remote) and **F-15** is closed everywhere it
  can be — `upgrade`/`backfill_norm` exist in sqlite, mysql, mariadb and
  mssql; oracle has no `upgrade` yet.
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
