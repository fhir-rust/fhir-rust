# CLAUDE.md — fhir-postgresql

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps.

**Engine:** PostgreSQL 18 · **Level:** Reference

Three things to know before editing anything here:

1. The pure-Rust core in `map/src` and `gen/src` is byte-identical across all
   six ports. Change it in all six or not at all (`X15.1`, `W16.7`).
2. Its `README.md` and dialect annex are now accurate. The hash-chain
   pre-image is computed in Rust as of **F-07** (`M14.12`) — writer and verifier
   share `canon_of`, and `chain_portability.rs` proves an outside verifier can
   recompute a chain. That was a **format change**: a database written before it
   needs a reload, not a migration. The TLS default is fixed too: connections
   verify by default (`SslPolicy::Require`, **F-17** fixed 2026-08-03,
   `tests/ssl_default.rs`, `M14.27`) — an earlier revision of this line still
   called it open.
3. Normative behaviour is [`../spec/`](../spec/databases/index.md), not this directory.
   Check [`../spec/audit.md`](../spec/databases/audit.md) before reporting a defect — it
   may already be tracked.
