# fhir-mssql-store

A placeholder for the storage layer this port does not have yet.

Part of **`fhir-mssql`**, which stores FHIR R3, R4 and R5 resources in SQL Server 2022 as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

> **This port has no store.** It emits T-SQL DDL and nothing else — no driver, no transactions, no search, no way to read or write a resource.

## What is here

`lib.rs` and `chain.rs`, and no implementation — 869 lines against the reference
port's 3,959. Nothing can be read or written through this crate.

It exists so the port's crate layout matches its five siblings, and so the
shared types a store will need (`Audit`, `AccessRecord`, `ChainBreak`, the key
ring) have a home. When a store is written it goes here, and this README stops
saying so.

## Why it is published anyway

Publishing a scaffold is a deliberate decision, and the risk is that the *name*
implies a working FHIR store to someone who has not opened the code. The crate
description says "scaffold: no store yet" for that reason, and so does this
paragraph. Do not depend on it expecting storage.

## Tests

64 tests, all of them about DDL emission.

## Trust boundary

These are components, not certified systems.

| Guaranteed here | Your deployment must provide |
| --- | --- |
| Attribution on every write | Authentication |
| A disclosure record on every read | Authorization, scopes, consent |
| Tamper-evident history (SHA-256 + SHA3-256, optional HMAC) | TLS termination |
| Append-only history enforced in the database | Rate limiting per identity |
| Erasure that leaves a verifiable tombstone | Terminology validation |
| No PHI in logs at default level | Everything else a certified system needs |

They cannot make a deployment compliant; they are built so as not to be the
reason one cannot be.

## Further reading

| | |
| --- | --- |
| [Port README](../../README.md) | this engine's overview, quick start, and trust boundary |
| [Dialect annex](../../spec/14-mssql-dialect.md) | every declared departure from the shared core, by number |
| [Specification](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/index.md) | the normative core, shared by all six ports |
| [Conformance matrix](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/conformance-matrix.md) | what each port actually satisfies today |
| [Audit register](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/audit.md) | every known divergence, with evidence |

## Status, honestly

A conformance level is a claim about what has been **verified for this port**,
not about what its code contains. This crate is part of a port at
**Scaffold** level: a T-SQL DDL emitter only; CI provisions SQL Server 2022 and fails rather than skips without it, so **Schema** level is reachable as soon as one green run exists to cite.

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.
