# fhir-oracle-store

The storage layer for `fhir-oracle` — live-verified at Store level.

Part of **`fhir-oracle`**, which stores FHIR® R3, R4 and R5 resources in Oracle Database as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

> **This port has a store, and it connects.** `connect`/`init`/`put`/`get`/
> `delete`/`history`/`vread`/`verify_audit`/`purge`/`log_access`/`search` —
> live-tested against `gvenzl/oracle-free:23-slim-faststart` in
> `tests/oracle_store.rs`: **7 of 7 tests pass, 0 ignored** (**F-68**,
> 2026-08-04). The DDL underneath it is the same Oracle emitter, executed
> since 2026-08-03 (**F-08**): the full R5 schema, 9,636 statements, installs
> on Oracle 26ai with 0 invalid objects.
>
> `get` uses a serializable snapshot, verified after schema install and
> during concurrent writes on Oracle 26ai Free (`M14.19`). Racing writers
> (`H5.4`) and redaction remain unverified. Install Oracle Instant Client
> to run the live tests.

## What is here

Five source files — `oracle.rs` (the store), `oracle_search.rs` (the search
query builder), `pool.rs` (connection pooling via the `oracle` crate's own
`oracle::pool`), plus `lib.rs` and the shared `chain.rs`. `oracle::Connection`
is synchronous (it wraps ODPI-C/OCI), so every public method wraps its body in
`tokio::task::spawn_blocking` — the same shape `fhir-sqlite` uses for
`rusqlite`.

Getting this store to actually run, for the first time, live, found and fixed
five real defects — see the module docs in `oracle.rs`/`oracle_search.rs` and
`audit.md` **F-68** for the full account: Oracle's username case-folding
requiring an uppercase schema, a double schema-qualification bug, a
timestamp-binding bug, and a boolean bound as text in token search, plus the
post-DDL `READ ONLY` failure (`ORA-01466`).

## Why it is published anyway

The [dialect annex](../../spec/14-oracle-dialect.md) records supported
operations and verification gaps at Store level.

## Tests

On 2026-11-19, Oracle 26ai Free 23.26.2.0.0 passed all 25 store integration
tests, including the eight tests in `tests/oracle_store.rs`. Run with
`--test-threads=1` because the tests share the `R5` schema.

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
not about what its code contains. This crate is part of a port at **Store**
level (`C0.8`): the DDL emitter is real Oracle and has been executed both by
hand and by this crate's own live test suite, and the store's full CRUD/
history/search/audit surface is live-tested against a real Oracle — 7 of 7
tests passed in the original suite. The snapshot regression is covered in
`tests/oracle_store.rs`.

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only OR OGL-UK-3.0 OR CC-BY-SA-4.0` — you choose.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
