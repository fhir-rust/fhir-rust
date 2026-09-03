# fhir-store

The **engine-agnostic half of FHIR® persistence**: everything about storing FHIR
that is not specific to one SQL engine.

It opens no sockets, speaks no HTTP, and links no database driver. What it holds
is the part every port needs and none of them should own privately.

## Install

```toml
[dependencies]
fhir-store = "0.3.1"
```

## What is in here

| | |
| --- | --- |
| `chain` | the tamper-evident audit chain — SHA-256 + SHA3-256, optional HMAC (`M3.16`) |
| `Audit`, `AccessRecord` | who is responsible for a change, and who read what (`PR12.1`–`PR12.4`) |
| `PutOutcome`, `Got`, `HistEntry`, `ResourceStatus` | what a write, a read and a history return |
| `SearchOutcome`, `CondCreate`, `CondDelete` | search and conditional-operation results |
| `TxOp`, `TxOutcome` | transaction bundle operations |
| `PurgeReport`, `UpgradeReport`, `ChainBreak` | erasure, migration and chain-verification reports |

## Why it exists

Six ports — `fhir-postgresql`, `fhir-sqlite`, `fhir-mysql`, `fhir-mariadb`,
`fhir-mssql`, `fhir-oracle` — each carried their own copy of this code.
`chain.rs` alone was **618 lines, byte-identical in all six**.

Worse, it was unwatched: `scripts/check-shared-core.sh` gates the map and the
generator and stopped at the store, so the store's engine-agnostic half drifted
without a gate. That is why closing audit finding **F-07** — the hash-chain
pre-image was derived from PostgreSQL's `jsonb`, so chains were not portable
between engines — had to be applied six times by hand.

## What stays in a port

The things that genuinely differ:

- the driver and its connection handling,
- transaction and locking syntax,
- placeholder syntax and value binding,
- the search-SQL builder,
- `ddl.rs`, which decides the emitted schema,
- `StoreError`, which wraps that port's own `ShredError` and so cannot be
  lifted here.

## What this crate does not do

**It does not authenticate.** That is the perimeter's job, and this crate holds
the boundary rather than crossing it. But "authentication is elsewhere" must not
become "the record of who did what is nowhere" (§12) — so `Audit` carries
attribution, `AccessRecord` carries disclosure, and `chain` makes both
tamper-evident.

The RESTful surface is [`fhir-loco`](https://crates.io/crates/fhir-loco).

## Further reading

| | |
| --- | --- |
| [Specification](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/index.md) | the normative core, shared by all six ports |
| [§12 trust, principal, audit](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/12-trust-principal-and-audit.md) | what attribution must guarantee |
| [Audit register](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/audit.md) | every known divergence, with evidence |

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
