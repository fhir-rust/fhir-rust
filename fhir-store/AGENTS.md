# Working in fhir-store

**What this is:** the engine-agnostic half of FHIR® persistence — the
tamper-evident audit chain (`chain.rs`), the attribution and disclosure
records it commits to, and the value types every database port's operations
return. Extracted from ~860 duplicated lines across the six ports (**F-45**).
It is **not** the HTTP surface (that is [`fhir-loco`](../fhir-loco/)) and it
is not a database port itself — see the root [`CLAUDE.md`](../CLAUDE.md) for
why this name has been reused twice in this repository's history and what
that means for a reader.

This crate is one of four families in the monorepo. Operational guidance is
shared and lives at the root:

- **[`../AGENTS.md`](../AGENTS.md)** — read this first. The five rules, the
  layout, the commit conventions.
- **[`../agents/`](../agents/index.md)** — topic guides: [rust](../agents/rust.md) ·
  [testing](../agents/testing.md) · [documentation](../agents/documentation.md) ·
  [security](../agents/security.md) · [release](../agents/release.md).

Normative behaviour is the database specification, not a spec of its own:
this crate has none (root `CLAUDE.md` says so explicitly, since an earlier
draft of this file almost invented one). What `chain.rs` implements is
governed by `spec/databases/03-hash-chains.md` and `M3.15`–`M3.18` in the
shared core spec, same as every port.

## Specific to this crate

**`chain.rs` is shared-core-adjacent, but not covered by
`check-shared-core.sh`.** That gate watches `map/` and `gen/` in each port;
this crate is what closing **F-45** moved the *store's* shared half into, and
it is a single copy by construction now rather than six kept in sync by
discipline. There is nothing to diverge into, which is a stronger guarantee
than a gate — but if a future extraction adds a second engine-agnostic crate,
do not assume this one's history means the same discipline applies
automatically.

**No I/O, no unsafe, no dependency on any port.** `#![forbid(unsafe_code)]`
at the crate root (`R13.14`), and the six ports depend on *this* crate, never
the other way — a `fhir-postgresql`-specific import here would be a layering
violation, not a convenience.

**The hash chain is unkeyed by default, and that is a documented boundary,
not an oversight.** The digests are computed over a published pre-image; an
attacker with database write access can recompute them. The keyed HMAC tag
(`M3.16b`) is the mechanism that actually resists that attacker — see
`chain.rs`'s own doc comments before assuming the unkeyed digest alone proves
anything against a privileged adversary.

## Running the tests

```sh
cargo test
```

No server, no container, no `scripts/db.sh` — this crate touches no
database. All 14 tests in `chain.rs` run offline, every time.

## Status

- [`../spec/databases/audit.md`](../spec/databases/audit.md) — open findings
  (search for `fhir-store`, not this directory's own register — it has none).
- [`../spec/databases/conformance-matrix.md`](../spec/databases/conformance-matrix.md)
  — this crate underlies every port's row but is not itself leveled (`C0.8`
  defines conformance for a *port*, not its shared library).
- [`CHANGELOG.md`](CHANGELOG.md) — this crate's own release history.

**Pushing:** still ask first — see [`../CLAUDE.md`](../CLAUDE.md#commit-and-push)
for the current, narrower reason (F-37, not the old six-remotes one).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
