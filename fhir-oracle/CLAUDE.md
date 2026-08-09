# CLAUDE.md — fhir-oracle

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps.

**Engine:** Oracle Database · **Level:** Store (`C0.8`)

Five things to know before editing anything here:

1. The pure-Rust core in `map/src` and `gen/src` is identical across all six
   ports, modulo whitespace. Change it in all six or not at all (`X15.1`,
   `W16.7`); check with `../scripts/check-shared-core.sh`.
2. `ddl.rs` **is** an Oracle emitter now (**F-08** closed): the full R5 schema —
   9,636 statements — installs on 26ai with 0 invalid objects.
3. `crates/fhir-oracle-store/src/` **connects and has been live-tested**
   (**F-68**, superseding **F-66**'s "compiles but never connected"):
   `tests/oracle_store.rs` runs the full CRUD/history/search/audit surface
   against a live `gvenzl/oracle-free:23-slim-faststart` — 7 of 7 tests
   pass, 0 ignored — and `tests/upgrade.rs` adds 9 more (`upgrade`/
   `backfill_norm`, 2026-08-09, closing **F-15**'s last port; run the live
   suite with `--test-threads=1`, every test shares the one `R5` schema).
   Getting there found and fixed five real defects — the
   sharpest one to remember: **Oracle folds an unquoted username to
   uppercase for session identity**, so `RelMap.schema` MUST be uppercase
   (`M14.5`) — the opposite of `r3`/`r4`/`r5` everywhere else. `R4.5` is an
   **open, confirmed gap**: the one candidate mechanism (`SET TRANSACTION
   READ ONLY`) was tried live and fails outright (`ORA-01466`); do not
   assume it works because it appears sound on paper (`M14.19`).
4. Normative behaviour is [`../spec/`](../spec/databases/index.md), not this directory.
   Check [`../spec/audit.md`](../spec/databases/audit.md) before reporting a defect — it
   may already be tracked.
5. Running the live suite needs **Oracle Instant Client** on the host, not
   just the container — `DYLD_LIBRARY_PATH=~/lib scripts/db.sh test`. See
   `AGENTS.md`'s "Running the live suite" and `scripts/db.sh`'s header
   comment for the (direct, no-login) download.

**If you touch the append-only trigger, execute a forbidden `DELETE`.** Do not
review it. Its first version failed open because Oracle treats `''` as NULL, so
`NVL(x, '') != 'y'` is NULL rather than TRUE and the guard never fired
(`M14.29a`). It looked correct.

**If you touch `SET TRANSACTION READ ONLY` or any other snapshot-isolation
idea for `R4.5`, run it live before trusting it.** The obvious candidate
already failed once, live, in a way no amount of reading would have caught
(`ORA-01466` on any session that has ever executed DDL) — see `M14.19` and
**F-68**.
