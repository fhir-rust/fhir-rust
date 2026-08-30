# Working in fhir-loco

**What this is:** the HTTP surface — a FHIR® RESTful API server (Loco.rs,
Axum, Tokio, Hyper) mounted over one of the six database ports. It inherited
the name `fhir-store` briefly before being renamed (**F-37**); the name
`fhir-store` was reused a few days later for an unrelated shared library —
see the root [`CLAUDE.md`](../CLAUDE.md) if that history is confusing, because
it is confusing on purpose to nobody, just an artifact of two separate
decisions landing close together.

This crate is one of four families in the monorepo. Operational guidance is
shared and lives at the root:

- **[`../AGENTS.md`](../AGENTS.md)** — read this first. The five rules, the
  layout, the commit conventions.
- **[`../agents/`](../agents/index.md)** — topic guides: [rust](../agents/rust.md) ·
  [testing](../agents/testing.md) · [databases](../agents/databases.md) ·
  [documentation](../agents/documentation.md) · [security](../agents/security.md) ·
  [release](../agents/release.md).

Normative behaviour is this crate's own specification, not the database one:

- **[`spec/index.md`](spec/index.md)** — ids `SV1.x`–`SV4.x`, added
  2026-08-03. Every externally visible promise (status codes, headers,
  CapabilityStatement content, auth) is a numbered requirement there, not a
  list that drifts.
- The six database ports it mounts have their own specs under
  `../spec/databases/`; this crate depends on them as libraries (`C0.17`,
  `C0.18` — they carry no server of their own, deliberately) and must not
  reach past their public API to assume internal behaviour.

## Specific to this crate

**The database libraries are embedded, not called over a network.**
`fhir-sqlite-store`/`-map` and `fhir-postgresql-store`/`-map` are **path**
dependencies (`Cargo.toml`, pinned to a `version` too, since publishing
requires one) — this crate's own build graph includes their source directly.
That has a real consequence for dependency updates: a version bump in either
port's `Cargo.toml` can require regenerating *this* crate's `Cargo.lock` too,
and nothing automated catches that across the directory boundary (found the
hard way — see `fhir-loco`'s `CHANGELOG.md` 0.3.1 entry).

**No concurrency limit is enforced** (`SV4.2`) — a Loco 1.0.1 framework
limit, stated at its own requirement id rather than left to a stale list.
`$export` (`SV2.15`) serves no `_since` and no compartment exports; the
served slice's edges are in that requirement, not restated here where they
would drift.

**Auth is PASETO v4.public** (`SV3.2`–`SV3.6`), not JWT — deliberate: PASETO
fixes the algorithm per version, so there is no `alg` field to confuse and no
"alg: none" class of bug. This service only ever *verifies*; it holds no key
that could mint a token.

## Running the tests

```sh
cargo test
```

The request suite runs against `fhir-sqlite` in-process — no service
containers, no DSN. `tests/pg_backend.rs` and any PostgreSQL-specific
coverage need that port's own container; see `../fhir-postgresql/scripts/db.sh`.

## Status

- [`spec/index.md`](spec/index.md) — this crate's own requirements; the
  place to check before assuming a behavior is a bug rather than a stated
  limit.
- [`../spec/databases/audit.md`](../spec/databases/audit.md) — findings that
  named this crate before it had a spec of its own (search for `fhir-loco`).
- [`tasks.md`](tasks.md) — the work breakdown.
- [`plan.md`](plan.md) — design decisions and their reasons.

**Pushing:** still ask first — see [`../CLAUDE.md`](../CLAUDE.md#commit-and-push)
for the current, narrower reason (F-37, not the old six-remotes one).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
