# Working in fhir-mysql

**Engine:** MySQL 8.4 · **Conformance level:** Store (`C0.8`)

This port is one of six in a monorepo. Operational guidance is shared and lives
at the root:

- **[`../AGENTS.md`](../AGENTS.md)** — read this first. The five rules, the
  layout, the commit conventions.
- **[`../agents/`](../agents/index.md)** — topic guides:
  [spec workflow](../agents/spec-workflow.md) ·
  [rust](../agents/rust.md) ·
  [testing](../agents/testing.md) ·
  [databases](../agents/databases.md) ·
  [documentation](../agents/documentation.md) ·
  [security](../agents/security.md) ·
  [release](../agents/release.md)

Normative behaviour is the monorepo core plus this port's annex:

- **[`../spec/index.md`](../spec/databases/index.md)** — sections 0–16, shared.
- **[`spec/index.md`](spec/index.md)** — this port's index and departures.
- Annex: 14-mysql-dialect.md

## Specific to this port

Independent from `fhir-mariadb` despite the shared ancestry
(`M14.0a`–`M14.0c`): neither must read the other's schema, and neither should
hold back syntax the other lacks. `TextC` binds to `utf8mb4_0900_bin` here.
`upgrade` and `backfill_norm` exist (**F-15** closed here, verified against
live MySQL 8.4); no `transact_audited` or conditional operations yet. When
touching the upgrade path, read `M14.34`–`M14.36` first: the meta column is
`LONGTEXT` because a `TEXT` one silently truncates the map asset, DDL is not
transactional so a failed upgrade is reported rather than rolled back, and the
reconcile filters run **after** the adds.

## The rule that catches people here

The pure-Rust core — `map/src/{model,shred,reconstruct,value,fold,canon,error}.rs`
and all of `gen/src` — is **identical across all six ports** (`X15.1`). Editing
it here alone is a divergence, not a fix; apply the change to all six in one
commit (`W16.7`). Nothing in CI checks this yet (**F-10**).

Dialect differences belong in exactly two places: `map/src/ddl.rs` and the
`store` crate — and, when they change what the core requires, in a numbered
`M14.x` departure in the annex (`C0.12`).

## Running the live suite

```sh
scripts/db.sh up      # start the pinned engine container
scripts/db.sh test    # up, then the live suite
scripts/db.sh down
```

`cargo test` alone passes with no database, because the corpus- and
database-driven tests self-skip. Most of what this library guarantees is a
database guarantee, so the live suite is the gate that means something.

## Status

- [`../spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md) — what this
  port actually satisfies, requirement by requirement.
- [`../spec/audit.md`](../spec/databases/audit.md) — open findings.
- [`tasks.md`](tasks.md) — the work breakdown.
- [`plan.md`](plan.md) — design decisions and their reasons.

**Do not push:** `origin` is still the ancestor project's repository (**F-11**).
