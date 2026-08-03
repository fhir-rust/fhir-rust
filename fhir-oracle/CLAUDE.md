# CLAUDE.md — fhir-oracle

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps.

**Engine:** Oracle Database · **Level:** Scaffold (`C0.8`)

Three things to know before editing anything here:

1. The pure-Rust core in `map/src` and `gen/src` is identical across all six
   ports, modulo whitespace. Change it in all six or not at all (`X15.1`,
   `W16.7`); check with `../scripts/check-shared-core.sh`.
2. `ddl.rs` **is** an Oracle emitter now (**F-08** closed): the full R5 schema —
   9,636 statements — installs on 26ai with 0 invalid objects. The level is
   still **Scaffold**, though: it was verified by hand, and `C0.9` counts only
   tests that run (**F-51**).
3. Normative behaviour is [`../spec/`](../spec/databases/index.md), not this directory.
   Check [`../spec/audit.md`](../spec/databases/audit.md) before reporting a defect — it
   may already be tracked.

**If you touch the append-only trigger, execute a forbidden `DELETE`.** Do not
review it. Its first version failed open because Oracle treats `''` as NULL, so
`NVL(x, '') != 'y'` is NULL rather than TRUE and the guard never fired
(`M14.29a`). It looked correct.
