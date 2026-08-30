# CLAUDE.md — fhir-store

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps — especially the
name-collision history: this directory is **not** the HTTP surface (that is
[`fhir-loco`](../fhir-loco/)), despite briefly having been, under a different
name, before **F-45**.

**What it is:** a shared library — the tamper-evident audit chain and the
engine-agnostic value types every port's operations return. **Level:** N/A
(`C0.8`'s four levels describe a database port; this crate underlies all six
but is not itself leveled).

Two things to know before editing anything here:

1. **This is the single copy, not one of six.** `chain.rs` used to be ~618
   lines duplicated byte-for-byte in every port (**F-45** extracted it here).
   `scripts/check-shared-core.sh` does not watch this crate — it watches
   `map/`/`gen/` in each port — so there is no gate keeping this file in sync
   with anything, because there is nothing left for it to drift from.
2. `#![forbid(unsafe_code)]` at the crate root, no I/O, no dependency on any
   port (the dependency direction runs the other way). The hash chain is
   unkeyed by default and documented as such — the keyed HMAC tag is what
   actually resists a privileged attacker, not the digest alone.

Normative behaviour is [`../spec/databases/`](../spec/databases/index.md),
not a spec of its own — this crate has none. Check
[`../spec/databases/audit.md`](../spec/databases/audit.md) before reporting a
defect — it may already be tracked.
