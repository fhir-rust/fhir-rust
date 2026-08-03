# fhir-oracle-store

A placeholder for the storage layer this port does not have yet.

Part of **`fhir-oracle`**, which stores FHIR R3, R4 and R5 resources in Oracle Database as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

> **This port has no store. Its DDL is Oracle and has been executed** — the full R5 schema, 9,636 statements, installs on Oracle 26ai with 0 invalid objects (**F-08** closed, 2026-08-03). What is missing is the runtime: no driver, no store, so nothing has been written through the schema.

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

53 tests, eleven of which are `#[ignore]`d because they assert MySQL behaviour.

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
| [Dialect annex](../../spec/14-oracle-dialect.md) | every declared departure from the shared core, by number |
| [Specification](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/index.md) | the normative core, shared by all six ports |
| [Conformance matrix](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/conformance-matrix.md) | what each port actually satisfies today |
| [Audit register](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/audit.md) | every known divergence, with evidence |

## Status, honestly

A conformance level is a claim about what has been **verified for this port**,
not about what its code contains. This crate is part of a port at
**Scaffold** level (`C0.8`): the DDL emitter is real Oracle and has been executed by hand against 26ai, but no test in this port runs it, and `C0.9` requires the level be justified by tests that run. **Schema** needs a live test — which needs an Oracle driver decision. No store.

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.
