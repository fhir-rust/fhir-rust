# CLAUDE.md — fhir-oracle

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps.

**Engine:** Oracle Database · **Level:** Scaffold

Three things to know before editing anything here:

1. The pure-Rust core in `map/src` and `gen/src` is byte-identical across all
   six ports. Change it in all six or not at all (`X15.1`, `W16.7`).
2. Scaffold: `ddl.rs` is still the MySQL emitter (**F-08**) and there is no
   store or driver. Do not read its output as Oracle DDL.
3. Normative behaviour is [`../spec/`](../spec/index.md), not this directory.
   Check [`../spec/audit.md`](../spec/audit.md) before reporting a defect — it
   may already be tracked.
