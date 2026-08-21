# Working in fhir-mssql

**Engine:** Microsoft SQL Server · **Conformance level:** Store (`C0.8`)

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
- Annex: [`14-mssql-dialect.md`](spec/14-mssql-dialect.md) — SQL Server's own,
  rewritten (**F-16** fixed)

## Specific to this port

**Store.** `store/src/mssql.rs` and `mssql_search.rs` are a real `tiberius`
store with search, live-verified against `azure-sql-edge` by 33 tests, 0
`#[ignore]`d (**F-65**; **F-15** closed here, 9 more tests in
`tests/upgrade.rs`). `R4.5` (stable reads under concurrent writers) needed
two tries — `READ_COMMITTED_SNAPSHOT` alone was tried live and still tore;
`get` issuing `SET TRANSACTION ISOLATION LEVEL SNAPSHOT`, backed by
`ALLOW_SNAPSHOT_ISOLATION` on a dedicated `fhir_mssql` database
(`scripts/db.sh`'s `post_ready`), is what actually fixed it. See `M14.25` in
the annex before touching `get`'s transaction handling or `db.sh`'s database
setup. `upgrade` is one transaction, unlike `fhir-mysql`/`fhir-mariadb` — a
failed one rolls back rather than half-applying (`M14.35`) — and its
destructive table drops MUST go children-before-base or SQL Server refuses
with error 3726 (`M14.36`, found live). No `conditional_create_audited`,
`put_audited`, or `transact_audited`. `O10.7`'s trust mechanism is confirmed working
live (`tests/ssl_live.rs`) but not claimed — the driver's TLS chain carries
four unpatched advisories now confirmed reaching this shipping crate, and
`native-tls` fails the handshake on this host (**F-67**; `M14.34`). CI now provisions SQL Server 2022 and
`FHIR_MSSQL_REQUIRE_DB=1` makes an absent database fail rather than skip
(**F-06** fixed).

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
export FHIR_MSSQL_TEST_DSN='...'   # scripts/db.sh up prints this
cargo test -p fhir-mssql-store -- --test-threads=1   # see mssql_store.rs's module doc
scripts/db.sh down
```

`cargo test` alone passes with no database, because the corpus- and
database-driven tests self-skip. Most of what this library guarantees is a
database guarantee, so the live suite is the gate that means something.
`--test-threads=1` is not optional for the store suite: running its tests
concurrently deadlocks `azure-sql-edge` under heavy concurrent DDL (SQL Server
error 1205) — a container-load artifact, not a bug in what any single test
exercises.

## Status

- [`../spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md) — what this
  port actually satisfies, requirement by requirement.
- [`../spec/audit.md`](../spec/databases/audit.md) — open findings.
- [`tasks.md`](tasks.md) — the work breakdown.
- [`plan.md`](plan.md) — design decisions and their reasons.

**Pushing:** still ask first — see [`../CLAUDE.md`](../CLAUDE.md#commit-and-push)
for the current, narrower reason (F-37, not the old six-remotes one).
