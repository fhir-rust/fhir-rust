# Working in fhir-postgresql

**Engine:** PostgreSQL 18 · **Conformance level:** Reference (`C0.8`)

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
- Annex: [`14-postgresql-dialect.md`](spec/14-postgresql-dialect.md) — 28
  requirements against the `X15.6` checklist (**F-14** fixed)

## Specific to this port

This is the **reference port**: the only one whose test suite substantiates
its level. **F-07** is fixed — the pre-image was derived in SQL
(`(($1::text)::jsonb)::text`) and this was the only map crate without
`canon.rs`; both are resolved, and `chain_portability.rs` recomputes a chain
from the exported rows the way a foreign verifier would. Closing it was a chain
**format change**: a database written earlier needs a reload.

Its outstanding defect is **F-17**, the unverified TLS default (`M14.27`),
which is an owner decision because changing it is breaking.

## The rule that catches people here

The pure-Rust core — `map/src/{model,shred,reconstruct,value,fold,canon,error}.rs`
and all of `gen/src` — is **identical across all six ports** (`X15.1`). Editing
it here alone is a divergence, not a fix; apply the change to all six in one
commit (`W16.7`), verified with `../scripts/check-shared-core.sh` (**F-10** fixed).

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

**Pushing:** still ask first — see [`../CLAUDE.md`](../CLAUDE.md#commit-and-push)
for the current, narrower reason (F-37, not the old six-remotes one).
