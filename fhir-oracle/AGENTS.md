# Working in fhir-oracle

**Engine:** Oracle Database · **Conformance level:** Scaffold (`C0.8`)

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
- Annex: [14-oracle-dialect.md](spec/14-oracle-dialect.md) — rewritten from the
  `X15.6` checklist (**F-16** fixed); `M14.28`–`M14.32` cover the DDL port

## Specific to this port

**The DDL is Oracle and has been executed; the runtime does not exist.**
`ddl.rs` was the MySQL emitter until 2026-08-03 (**F-08**, closed). The full R5
schema — 158 resources, 9,636 statements — now installs on Oracle 26ai with
0 invalid objects and 0 unindexable search targets.

What is still missing, and why the level has not moved: there is no
store crate and no driver, so nothing has been written through the schema by
this port. The eleven MySQL-asserting tests in `ddl.rs` are still `#[ignore]`d
and still need replacing (`M14.25`, `T11.14`).

The engine floor is Oracle 12.2 (`M14.2`) under a 63-byte identifier budget;
**F-09** is closed.

**The trap here, concretely.** A guard that reads correctly can still be inert.
The append-only trigger's first version used `NVL(SYS_CONTEXT(…), '')`, and
because Oracle treats the empty string as NULL, `NULL != 'x'` is NULL rather
than TRUE — so `DELETE` on history was permitted with no error at all
(`M14.29a`). Execute the forbidden operation. Do not review it.

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
