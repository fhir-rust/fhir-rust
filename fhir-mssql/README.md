# fhir-mssql — FHIR in SQL Server, relationally

Store [FHIR](https://hl7.org/fhir/) resources in Microsoft SQL Server as **real
relational tables** — typed columns, child tables, foreign keys, and check
constraints — not JSON blobs.

> ## ⚠ Status: Scaffold. There is no store yet.
>
> **What works:** the FHIR-specification generator, the shred/reconstruct
> engine, and a **T-SQL DDL emitter** verified to install on a real server. That
> emitter is genuine work — bracketed identifiers, `BIT`,
> `NVARCHAR(450) COLLATE Latin1_General_100_BIN2`, `DATETIME2(6)`,
> `INSTEAD OF` triggers, `sys.objects` idempotence guards — and running it
> against an engine found four real bugs the shape-asserting unit tests could
> not.
>
> **What does not exist:** the store. `crates/fhir-mssql-store/src/` contains
> `lib.rs` and no implementation — 48 lines that re-export the shared audit
> chain (`fhir-store`, **F-45**) and define an error type — and there are no
> store tests.
> You cannot write or read a resource with this crate today.
>
> **What was wrong until 2026-07-31**, worth knowing if you have read an older
> copy of this file: it claimed 7,399 FHIR example resources round-tripped
> through live SQL Server and that `fhir-mssql serve` mounted a REST API.
> Neither was ever true here — the text was the `fhir-postgresql` README with
> the engine name substituted. Tracked as [`audit.md`](../spec/databases/audit.md)
> **F-01**. The [conformance matrix](../spec/databases/conformance-matrix.md) is the
> status document to trust.

Supported FHIR versions in the generator and DDL: **R5 (5.0.0), R4 (4.0.1),
R3 (3.0.2)**, each in its own SQL Server schema.

## What you can do with it today

Generate and inspect the schema, and round-trip resources **in memory** — the
engine is shared with the other ports and is not dialect-specific:

```rust
use std::sync::Arc;
use fhir_mssql_map::model::RelMap;
use fhir_mssql_map::{ddl, shred};

let map = Arc::new(RelMap::bundled("r5")?);   // compiled in (feature `r5`)

// The T-SQL this port emits.
for stmt in ddl::ddl(&map) {
    println!("{stmt};");
}

// Shredding and reconstruction work now, with no database involved.
let rows = shred(map.resources.get("Patient").unwrap(), &patient)?;
```

To exercise the DDL against a real server:

```sh
scripts/db.sh up      # SQL Server 2022 in a container
scripts/db.sh test    # installs the generated schema, checks the triggers
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

In order, from [`tasks.md`](tasks.md) and the annex:

1. **A `tiberius` store.** The dependency is already in the workspace and proven
   against the server by the DDL test.
2. **A search builder** — `OFFSET n ROWS FETCH NEXT m ROWS ONLY` rather than
   `LIMIT`, `@P1` placeholders, no `NULLS LAST`.
3. **The unindexable-column decision**, above.
4. **Snapshot isolation and write serialization**, both undecided (`M14.25`,
   `M14.26`).
5. **Verification against full SQL Server**, not only `azure-sql-edge`.

## Documentation

- [`spec/index.md`](spec/index.md) — this port's spec index and departures; the
  normative core is shared at [`../spec/`](../spec/databases/index.md).
- [`../spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md) — what this
  port actually satisfies, requirement by requirement.
- [`../doc/`](../doc/index.md) — monorepo tutorials and examples. Use
  `fhir-postgresql` or `fhir-sqlite` to follow them; this port has no store.
- **[The book](book/src/SUMMARY.md)** — inherited from the PostgreSQL original
  and **not yet rewritten for this engine**; read it with that in mind.
- [`plan.md`](plan.md) · [`tasks.md`](tasks.md) · [`CHANGELOG.md`](CHANGELOG.md)

## License

MIT OR Apache-2.0.
