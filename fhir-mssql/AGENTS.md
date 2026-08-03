# Working in fhir-mssql

**Engine:** Microsoft SQL Server · **Conformance level:** Scaffold (`C0.8`)

This port is one of six in a monorepo. Operational guidance is shared and lives
at the root:

- **[`../AGENTS.md`](../AGENTS.md)** — read this first. The five rules, the
  layout, the commit conventions.
- **[`../AGENTS/`](../AGENTS/index.md)** — topic guides:
  [spec workflow](../AGENTS/spec-workflow.md) ·
  [rust](../AGENTS/rust.md) ·
  [testing](../AGENTS/testing.md) ·
  [databases](../AGENTS/databases.md) ·
  [documentation](../AGENTS/documentation.md) ·
  [security](../AGENTS/security.md) ·
  [release](../AGENTS/release.md)

Normative behaviour is the monorepo core plus this port's annex:

- **[`../spec/index.md`](../spec/databases/index.md)** — sections 0–16, shared.
- **[`spec/index.md`](spec/index.md)** — this port's index and departures.
- Annex: 14-mssql-dialect.md — **describes MySQL, F-16**

## Specific to this port

**Scaffold.** The T-SQL `ddl.rs` is real and deliberate, but there is **no
store** — `store/src/` holds only `lib.rs` and `chain.rs`, and there are no
store tests. Two traps: the dialect annex is the MySQL annex unedited
(**F-16**), and CI plus `scripts/db.sh` provision **MySQL 8.4**, while the
one live DDL test skips silently without `FHIR_MSSQL_TEST_DSN` that the
pipeline never sets (**F-06**). A green run here means nothing.

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
