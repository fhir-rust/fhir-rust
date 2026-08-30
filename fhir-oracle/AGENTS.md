# Working in fhir-oracle

**Engine:** Oracle Database · **Conformance level:** Store (`C0.8`)

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
- Annex: [14-oracle-dialect.md](spec/14-oracle-dialect.md) — rewritten from the
  `X15.6` checklist (**F-16** fixed); `M14.28`–`M14.32` cover the DDL port

## Specific to this port

**The DDL is Oracle and has been executed; the store now connects and its
full surface is live-tested, with one confirmed gap.** `ddl.rs` was the
MySQL emitter until 2026-08-03 (**F-08**, closed). The full R5 schema — 158
resources, 9,636 statements — now installs on Oracle 26ai with 0 invalid
objects and 0 unindexable search targets. `crates/fhir-oracle-store/src/`
connected to a live `gvenzl/oracle-free:23-slim-faststart` for the first
time 2026-08-04 (**F-68**, superseding **F-66**'s "compiles but never
connected"): `tests/oracle_store.rs` runs `init`/`put`/`get`/`delete`/
`history`/`vread`/`verify_audit`/`purge`/`log_access`/`search` against it —
**7 of 7 tests pass, 0 ignored**. Getting there found and fixed five real
defects — see **F-68** in full, or the short version: Oracle folds an
unquoted username to uppercase for session identity, so the three version
namespaces must be **uppercase** Oracle users (`M14.5`, the opposite
convention from `r3`/`r4`/`r5` elsewhere); `R4.5`'s presumed mechanism (`SET
TRANSACTION READ ONLY`) fails outright with `ORA-01466` (`M14.19` — this is
an **open, confirmed gap**, not merely unverified); a double
schema-qualification bug (`ORA-00926`); a timestamp-binding bug
(`ORA-01843`); and a boolean bound as text in token search (`ORA-01722`,
`M14.34`).

Conformance level moved from Scaffold to **Store**. Not yet Reference: no
`concurrency.rs` verifies `H5.4` under contention (the `SELECT … FOR UPDATE`
mechanism is present, untested against racing writers), no `redaction.rs`,
and `R4.5` has no working answer at all. (`upgrade`/`backfill_norm` left
this list 2026-08-09: `tests/upgrade.rs`, 9 live tests — resumable DDL
`M14.35`, chunked meta `M14.36`, ROWID-keyset backfill `M14.37` — closing
**F-15**'s last port. Run the live suite with `--test-threads=1`: every
test shares the one uppercase `R5` schema, `M14.5`.) The eleven
`#[ignore]`d MySQL-asserting tests in `ddl.rs` were replaced by one
consolidated `no_mysql_or_postgres_constructs_survive` test closing
**F-08** — none remain ignored (`M14.25`).

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
commit (`W16.7`), verified with `../scripts/check-shared-core.sh` (**F-10** fixed).

Dialect differences belong in exactly two places: `map/src/ddl.rs` and the
`store` crate — and, when they change what the core requires, in a numbered
`M14.x` departure in the annex (`C0.12`).

## Running the live suite

```sh
DYLD_LIBRARY_PATH=~/lib scripts/db.sh up      # start the pinned engine container
DYLD_LIBRARY_PATH=~/lib scripts/db.sh test    # up, then the live suite
DYLD_LIBRARY_PATH=~/lib scripts/db.sh down
```

Unlike the other five ports, this one also needs **Oracle Instant Client** on
the host — the `oracle` crate `dlopen`s `libclntsh` at connection time, not
build time, so `cargo check`/`cargo build` work without it but every live
test fails `DPI-1047` without it. See `scripts/db.sh`'s header comment for
the (direct, no-login) download. `db.sh up`'s `ready()` check detects a
missing client and fails fast with a named error rather than polling to a
generic timeout.

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
