# CLAUDE.md — fhir-mysql

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps.

**Engine:** MySQL 8.4 · **Level:** Store

Three things to know before editing anything here:

1. The pure-Rust core in `map/src` and `gen/src` is byte-identical across all
   six ports. Change it in all six or not at all (`X15.1`, `W16.7`).
2. No optimistic concurrency at all (no `put_audited`, no `expected_version`),
   no `transact_audited` or conditional operations. `upgrade` and
   `backfill_norm` **do** exist now (**F-15** closed here) — note `M14.36`:
   MySQL has no idempotent `CREATE INDEX` as emitted, so the reconcile step
   filters against `information_schema`, and both filters run *after* the adds.
   The live suites need a DSN — `scripts/db.sh up` prints it; they found F-20,
   F-21, F-23, F-24 and F-28.
3. Normative behaviour is [`../spec/`](../spec/index.md), not this directory.
   Check [`../spec/audit.md`](../spec/audit.md) before reporting a defect — it
   may already be tracked.
