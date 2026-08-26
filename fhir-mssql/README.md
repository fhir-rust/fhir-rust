# fhir-mssql — FHIR® in SQL Server, relationally

Store [FHIR](https://hl7.org/fhir/) resources in Microsoft SQL Server as **real
relational tables** — typed columns, child tables, foreign keys, and check
constraints — not JSON blobs.

> ## ⚠ Status: Store
>
> **What works:** a real `tiberius` store — `connect`, `init`, `put`, `get`,
> `delete`, `history`, `vread`, `search`/`search_full`/`search_page`,
> `verify_audit`, `purge`, `log_access`, `upgrade`, `backfill_norm` —
> live-verified against a real server by 33 tests (`mssql_store.rs`,
> `concurrency.rs`, `redaction.rs`, `roundtrip_types.rs`, `ssl_live.rs`,
> `upgrade.rs`), **0 `#[ignore]`d**, plus the T-SQL DDL emitter that came
> before it. You can write and read a resource with this crate today.
> Five real defects surfaced by running the work live were found and fixed —
> a cross-column collation conflict that broke every chained reference
> search, a keyed audit tag that was written but never checked, a `connect`
> that reported success against an unreachable server, a doubled erasure
> count, and a torn read under concurrent writers (`R4.5`) — see
> [`audit.md`](../spec/databases/audit.md) **F-65**.
>
> The `R4.5` fix took two tries, worth knowing about because the first one is
> the answer that reads right and isn't: `READ_COMMITTED_SNAPSHOT` alone was
> enabled and, run against the live reproduction, still tore — it gives each
> *statement* inside a transaction its own snapshot, not the whole
> *transaction* one. `SET TRANSACTION ISOLATION LEVEL SNAPSHOT`, backed by
> `ALLOW_SNAPSHOT_ISOLATION` on a dedicated database (`scripts/db.sh` now
> provisions one — `master` refuses the option), is what actually stopped it.
>
> **`O10.7` is diagnosed, not satisfied.** `tests/ssl_live.rs` confirms live
> that certificate verification is a real mechanism, not a no-op —
> `TrustServerCertificate=false` reproducibly rejects `azure-sql-edge`'s
> self-signed certificate. But that same investigation found the driver's TLS
> dependency chain carries four unpatched advisories (three CVEs in
> `rustls-webpki`, one unmaintained-crate warning) that now reach this
> *shipping* store crate — `deny.toml` had been ignoring them on the
> assumption they were dev-only, true when written and false since this port
> gained a store two tasks later. `native-tls` was tried as an escape and
> fails the TLS handshake outright on this host. See `M14.34` and
> [`audit.md`](../spec/databases/audit.md) **F-67**.
>
> **What does not exist:** `conditional_create_audited`, `put_audited`
> (optimistic concurrency), `transact_audited`. `upgrade` and `backfill_norm`
> now exist (closes this port's share of **F-15**) — live-verified against
> `azure-sql-edge` by 9 more tests (`tests/upgrade.rs`), 0 `#[ignore]`d,
> bringing the store's live total to 33. Unlike `fhir-mysql`/`fhir-mariadb`,
> `upgrade` is genuinely atomic: T-SQL DDL is transactional, so the whole
> additive-plus-destructive apply runs inside one `BEGIN TRANSACTION` and
> rolls back on the first failure rather than leaving a half-upgraded schema.
> A live-only defect surfaced writing this: dropping a resource's tables in
> arbitrary order failed with SQL Server error 3726 (`DROP TABLE` blocked by
> a `FOREIGN KEY` still referencing it) — fixed by dropping child tables
> before their base table (`M14.36`).
> Verified against `azure-sql-edge` only, not full SQL Server (`M14.31`).
>
> **What was wrong until 2026-07-31**, worth knowing if you have read an older
> copy of this file: it claimed 7,399 FHIR example resources round-tripped
> through live SQL Server and that `fhir-mssql serve` mounted a REST API.
> Neither was true at the time — the text was the `fhir-postgresql` README with
> the engine name substituted. Tracked as [`audit.md`](../spec/databases/audit.md)
> **F-01**. The [conformance matrix](../spec/databases/conformance-matrix.md) is the
> status document to trust.

Supported FHIR versions in the generator and DDL: **R5 (5.0.0), R4 (4.0.1),
R3 (3.0.2)**, each in its own SQL Server schema.

## What you can do with it today

Store and retrieve a resource against a real server:

```rust
use std::sync::Arc;
use fhir_mssql_map::model::RelMap;
use fhir_mssql_store::mssql::MsSqlStore;

let map = Arc::new(RelMap::bundled("r5")?);   // compiled in (feature `r5`)
let dsn = "server=tcp:127.0.0.1,1433;user=sa;password=…;TrustServerCertificate=true";
let store = MsSqlStore::connect(dsn, map).await?;
store.init("my-checksum").await?;

let put = store.put(&patient_json, &Audit::default()).await?;
let back = store.get("Patient", &put.id).await?;
let hits = store.search("Patient", &[("family".into(), "Aero".into())], 10, 0).await?;
```

To exercise the DDL, or the full live suite, against a real server:

```sh
scripts/db.sh up      # SQL Server 2022 in a container
scripts/db.sh test    # installs the generated schema, checks the triggers
export FHIR_MSSQL_TEST_DSN='...'   # scripts/db.sh up prints this
cargo test -p fhir-mssql-store -- --test-threads=1   # see mssql_store.rs's module doc
scripts/db.sh down
```

On Apple silicon set `FHIR_MSSQL_IMAGE=mcr.microsoft.com/azure-sql-edge`, the
arm64 build — a **subset** of the product, so good evidence rather than a
conformance claim.

## What is specific to SQL Server

Full detail in the [dialect annex](spec/14-mssql-dialect.md); the decisions
that shaped the DDL:

- **Bracketed identifiers**, never double quotes. Double quotes work only under
  `QUOTED_IDENTIFIER ON`, which is the default but is *session state*, and a
  schema must not depend on session state.
- **`NVARCHAR` throughout, never `VARCHAR`.** SQL Server's `VARCHAR` is a
  single-byte code page unless the column carries a UTF-8 collation; FHIR text
  is Unicode, and losing a patient name to a code page is not a trade worth
  making for storage.
- **`TextC` is `NVARCHAR(450) COLLATE Latin1_General_100_BIN2`.** The server's
  default collation is case- **and** accent-insensitive, so a column left at the
  default would silently acquire fuzzy equality — enough to break `:exact`
  matching and key identity. 450 × 2 bytes = 900, exactly the index key limit.
- **`DATETIME2(6)`, not `DATETIME`,** which rounds to 1/300th of a second and
  would silently alter a timestamp the hash chain commits to.
- **`ords` is `VARBINARY(255)`** holding the shared text image — the only port
  that uses bytes, because it keeps one byte per character against the key
  budget.
- **900-byte index keys**, tighter than MySQL's 3072, so `Ext` and `Deep` keep a
  hash surrogate key.
- **`CREATE OR ALTER TRIGGER … INSTEAD OF`** for append-only history: one
  idempotent statement, with no `DROP`-then-`CREATE` window in which history is
  unguarded.

### The known gap in search

A token's `system` and `code` are `NVARCHAR(MAX)`, which **cannot be part of an
index key**, so they are dropped from their index and those searches scan. They
remain correct; they are not fast. The intended fix is a persisted computed
column holding the leading 450 characters, indexed in its place — indexable
without truncating what is stored — and it belongs in the generated map rather
than in the DDL. Recorded as a departure (`M14.16`), not hidden.

## What has to happen next

In rough order, from the [conformance matrix](../spec/databases/conformance-matrix.md)
and the annex:

1. **Decide `M14.34`'s residual risk** — the verification mechanism works;
   the TLS library underneath it does not have a fix available today. Accept
   the risk formally, fund a different driver, or state the transport story
   is unresolved.
2. **The unindexable-column decision**, above (`M14.16`).
3. **Verification against full SQL Server**, not only `azure-sql-edge`
   (`M14.31`).

`upgrade` and `backfill_norm` are done — every Store-level port now has them
(closes this port's share of **F-15**).

## Documentation

- [`spec/index.md`](spec/index.md) — this port's spec index and departures; the
  normative core is shared at [`../spec/`](../spec/databases/index.md).
- [`../spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md) — what this
  port actually satisfies, requirement by requirement.
- [`../doc/`](../doc/index.md) — monorepo tutorials and examples, written
  against `fhir-postgresql`/`fhir-sqlite`; not yet re-checked against this
  port's store.
- **[The book](book/src/SUMMARY.md)** — inherited from the PostgreSQL original
  and **not yet rewritten for this engine**; read it with that in mind.
- [`plan.md`](plan.md) · [`tasks.md`](tasks.md) · [`CHANGELOG.md`](CHANGELOG.md)

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` —
your choice of any one of the five; the reasoning and full texts are in
the repository's [LICENSE.md](../LICENSE.md) and [LICENSES/](../LICENSES/).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
