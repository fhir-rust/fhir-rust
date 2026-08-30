# CLAUDE.md — fhir-loco

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps — this crate is
the center of the repository's most confusing naming history (**F-37**,
**F-45**); read that section before assuming this directory or the sibling
`fhir-store/` is the one you think it is.

**What it is:** the HTTP surface — a FHIR® RESTful API server (Loco.rs) over
one of the six embeddable database ports. **Level:** governed by its own
spec (`SV1.x`–`SV4.x`), not `C0.8`'s four database-port levels.

Two things to know before editing anything here:

1. **The database crates it mounts are path dependencies, not registry-only
   ones** — `fhir-sqlite-store`/`-map` and `fhir-postgresql-store`/`-map`
   participate directly in this crate's own build graph. A dependency bump
   in either port can require regenerating *this* crate's `Cargo.lock` too;
   nothing checks for that automatically across the directory boundary.
2. Only the six ports are embeddable libraries with no server of their own
   (`C0.17`, `C0.18`) — this crate is deliberately the one exception. Do not
   "simplify" by adding a `serve` command to a port instead of routing the
   change through here.

Normative behaviour is [`spec/index.md`](spec/index.md) for this crate's own
surface, and [`../spec/databases/`](../spec/databases/index.md) for whatever
port is mounted underneath. Check
[`../spec/databases/audit.md`](../spec/databases/audit.md) before reporting a
defect — it may already be tracked.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
