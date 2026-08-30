# CLAUDE.md — fhir-sqlite

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps.

**Engine:** SQLite 3 · **Level:** Store

Three things to know before editing anything here:

1. The pure-Rust core in `map/src` and `gen/src` is byte-identical across all
   six ports. Change it in all six or not at all (`X15.1`, `W16.7`).
2. `transact_audited` returns `Unsupported` deliberately. `upgrade` and
   `backfill_norm` now exist (**F-15** closed on all six ports, oracle last,
   2026-08-09); note `M14.32` — SQLite has no `ADD COLUMN IF NOT EXISTS`, so
   the audit envelope is diffed, never reconciled as PostgreSQL does.
   The suite needs no server: run it. It found F-20 to F-23.
3. Normative behaviour is [`../spec/`](../spec/databases/index.md), not this directory.
   Check [`../spec/audit.md`](../spec/databases/audit.md) before reporting a defect — it
   may already be tracked.
