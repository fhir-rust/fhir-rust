# fhir-mariadb-store

The storage layer: driver, transactions, search, and the tamper-evident audit chain.

Part of **`fhir-mariadb`**, which stores FHIR R3, R4 and R5 resources in MariaDB 11.4 as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

## Install

```toml
[dependencies]
fhir-mariadb-store = "0.4.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

## Quick start

```rust
use std::sync::Arc;
use fhir_mariadb_store::{Audit, mariadb::MariaDbStore};

// open against MariaDB 11.4 — see the port README for the connection form
let store = ...;
store.init("r5-baseline").await?;          // ~7,400 tables, one transaction

let patient = serde_json::json!({
    "resourceType": "Patient", "id": "example",
    "name": [{ "family": "\u{c6}r\u{f8}" }], "birthDate": "1974-12"
});
store.put(&patient, &Audit::cli()).await?;

let back = store.get("Patient", "example").await?.unwrap();
assert_eq!(back, patient);                 // losslessly, including "1974-12"

// Accent- and case-insensitive by construction: "aero" finds "\u{c6}r\u{f8}".
let hits = store.search("Patient", &[("name".into(), "aero".into())], 50, 0).await?;
```

## What is implemented

all core reads and writes, search with cursor paging, erasure, audit verification.

`transact_audited`, `conditional_create`, `conditional_delete`, `emit_checkpoint`, `chain_witness` and `resign_history` are absent — see the [conformance matrix](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/conformance-matrix.md) for the operation-by-operation table.

## Operations

| Group | Methods |
| --- | --- |
| Schema | `init`, `upgrade`, `installed_checksum`, `backfill_norm`, `table_count`, `drop_schema` |
| Read/write | `put`, `get`, `delete`, `history`, `vread` |
| Search | `search`, `search_full`, `search_page` (cursor) |
| Audit | `log_access`, `verify_audit`, `history_canon` |

## Testing

102 tests, green against a live MariaDB 11.4.

`cargo test` passing means less than it looks. Most of what this layer
guarantees is a *database* guarantee — snapshot isolation, row locks, the
append-only trigger, index-using search plans — and none of it is exercised
without a server. The live suite is the real gate.

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
| [Dialect annex](../../spec/14-mariadb-dialect.md) | every declared departure from the shared core, by number |
| [Specification](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/index.md) | the normative core, shared by all six ports |
| [Conformance matrix](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/conformance-matrix.md) | what each port actually satisfies today |
| [Audit register](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/audit.md) | every known divergence, with evidence |

## Status, honestly

A conformance level is a claim about what has been **verified for this port**,
not about what its code contains. This crate is part of a port at
**Store** level: native store and search; 102 tests across the same four suites, green against a live MariaDB 11.4.

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.
