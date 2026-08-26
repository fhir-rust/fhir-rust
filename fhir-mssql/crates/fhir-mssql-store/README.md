# fhir-mssql-store

The storage layer for `fhir-mssql` — live-verified against `azure-sql-edge`.

Part of **`fhir-mssql`**, which stores FHIR® R3, R4 and R5 resources in SQL Server 2022 as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

> **This port has a store, and it is live-verified.** `connect`/`init`/`put`/
> `get`/`delete`/`history`/`vread`/`verify_audit`/`purge`/`log_access`/
> `search`/`search_full`/`search_page` — 24 tests, 0 ignored, green against
> live `azure-sql-edge` (**F-65**). `R4.5` (torn reads under concurrent
> writers) was confirmed violated live and then fixed the same day: `SET
> TRANSACTION ISOLATION LEVEL SNAPSHOT`, backed by `ALLOW_SNAPSHOT_ISOLATION`
> on a dedicated database — `READ_COMMITTED_SNAPSHOT` alone was tried first
> and found insufficient. `O10.7`'s trust/no-trust mechanism is proven
> correct live (`tests/ssl_live.rs`), but the TLS library underneath it
> carries unpatched CVEs with no available fix — see **F-67**.
>
> **What is still missing:** `conditional_create_audited`, `put_audited`,
> `transact_audited`, `upgrade`, `backfill_norm`.

## What is here

`mssql.rs` (the store), `mssql_search.rs` (the search query builder),
`pool.rs` (a hand-written `bb8::ManageConnection` for `tiberius`, which ships
no pool of its own), plus `lib.rs` and the shared `chain.rs`.

Live-verifying this store for the first time found and fixed five real
defects — see `audit.md` **F-65** for the full account: a cross-column
collation conflict that broke every chained reference search,
`verify_audit` never checking the keyed tag it wrote, `connect` returning
`Ok` for an unreachable server, `purge` double-counting erased versions, and
the `R4.5` torn read above.

## Why it is published anyway

A conformance level is a claim about what has been verified, not about what
the code contains. This crate now has live evidence behind nearly all of its
surface. Read the module docs and the [dialect annex](../../spec/14-mssql-dialect.md)
before depending on an operation this README does not name above.

## Tests

64 tests, all of them about DDL emission (`ddl.rs`, in the sibling `-map`
crate). This crate's own live suite: 24 tests (13 `mssql_store.rs`, 2
`concurrency.rs`, 2 `redaction.rs`, 6 `roundtrip_types.rs`, 1 `ssl_live.rs`),
0 ignored, all against a real SQL Server.

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
not about what its code contains. This crate is part of a port at **Store**
level (`C0.8`): a T-SQL DDL emitter and a store, both live-verified against
`azure-sql-edge`. Not **Reference**: no `conditional_create_audited`,
`put_audited`, `transact_audited`, `upgrade`, or `backfill_norm`, and `O10.7`
is diagnosed but not satisfiable with the current TLS dependency (**F-67**).

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
