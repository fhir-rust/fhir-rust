# fhir-oracle-store

The storage layer for `fhir-oracle` — live-verified, with one confirmed gap.

Part of **`fhir-oracle`**, which stores FHIR R3, R4 and R5 resources in Oracle Database as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

> **This port has a store, and it connects.** `connect`/`init`/`put`/`get`/
> `delete`/`history`/`vread`/`verify_audit`/`purge`/`log_access`/`search` —
> live-tested against `gvenzl/oracle-free:23-slim-faststart` in
> `tests/oracle_store.rs`: **7 of 7 tests pass, 0 ignored** (**F-68**,
> 2026-08-04). The DDL underneath it is the same Oracle emitter, executed
> since 2026-08-03 (**F-08**): the full R5 schema, 9,636 statements, installs
> on Oracle 26ai with 0 invalid objects.
>
> **What is still missing:** `R4.5` (snapshot reads under concurrent
> writers) has no working mechanism — the one candidate this port's annex
> named was tried live and removed after it broke every read (`ORA-01466`).
> There is no concurrency test verifying `H5.4` under contention, no
> redaction test, and no `upgrade`/`backfill_norm`. Needs Oracle Instant
> Client on the host to run at all — see the port README.

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
`R4.5` mechanism failure named above.

## Why it is published anyway

A conformance level is a claim about what has been verified, not about what
the code contains. This crate now has real, live evidence behind most of its
surface, but not all of it — `R4.5` in particular is a named, open gap, not
merely an absence. Read the module docs and the [dialect annex](../../spec/14-oracle-dialect.md)
before depending on any operation this README does not name above.

## Tests

53 tests, eleven of which are `#[ignore]`d because they assert MySQL
behaviour (`ddl.rs`, in the sibling `-map` crate). This crate's own
`tests/oracle_store.rs`: 7 tests, 0 ignored, all live against a real Oracle.

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
tests pass, 0 ignored. Not **Reference**: `R4.5` has no working mechanism,
and `H5.4` is unverified under concurrent writers.

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.
