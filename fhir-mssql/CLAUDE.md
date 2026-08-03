# CLAUDE.md — fhir-mssql

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps.

**Engine:** Microsoft SQL Server · **Level:** Scaffold

Three things to know before editing anything here:

1. The pure-Rust core in `map/src` and `gen/src` is byte-identical across all
   six ports. Change it in all six or not at all (`X15.1`, `W16.7`).
2. Scaffold: there is **no store**. The T-SQL `ddl.rs` is real and CI now
   provisions SQL Server 2022, but no green run exists yet to cite (`C0.9`).
3. Normative behaviour is [`../spec/`](../spec/databases/index.md), not this directory.
   Check [`../spec/audit.md`](../spec/databases/audit.md) before reporting a defect — it
   may already be tracked.
