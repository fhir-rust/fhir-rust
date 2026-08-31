# Tutorial 6 — porting to a new database

What it takes to add a seventh engine, in the order that works, with the steps
that get skipped called out — because two of the six existing ports skipped them
and the results are in the [audit register](../spec/databases/audit.md).

Normative reference:
[`spec/15-portability-and-dialects.md`](../spec/databases/15-portability-and-dialects.md).

## What you are actually porting

Very little, which is the good news:

| | Changes? |
| --- | --- |
| `map/src/{model,shred,reconstruct,value,fold,canon,error}.rs` | **no** |
| `gen/src/**` (the whole generator) | **no** |
| `map/src/ddl.rs` | **yes** — the real work |
| `store/**` | **yes** |

Those first two rows are `X15.1`, and they are not aspirational: measured across
the six existing ports, every one of those files is byte-identical modulo the
crate name. They operate on Rust types and never emit SQL, so there is nothing
in them for a dialect to change.

The SQLite annex says it well: `PG_MAX_IDENT = 63` and `SPLIT_WIDTH = 150` *look*
like PostgreSQL constants but are correct for SQLite too, because both are
tighter than SQLite's limits. Identifier fitting and table splitting carry over
unmodified.

## Step 1 — copy the closest port

`fhir-mysql` for a client/server SQL engine, `fhir-sqlite` for an embedded one.
Rename directories and crates to `fhir-<engine>-{map,gen,store}` (`W16.2`).

**Fix every crate `description` while you are there** (`W16.3`). All six current
ports say "PostgreSQL storage layer for fhir-\<engine\>" — a string published to
crates.io and rendered on docs.rs, read by exactly the person who has not looked
at the code yet (**F-02**).

## Step 2 — write the annex, before the code

This is the step to do out of order — *before* implementing, not after — and it
is the one both scaffold ports skipped.

Write `spec/14-<engine>-dialect.md` against the twelve-item `X15.6` checklist:

1. Engine floor, and the dialect fact that sets it (`S1.4`)
2. Namespace mechanism (`S1.2`)
3. `ColTy` binding, justified against `M3.6a`–`M3.6c` (`M3.6`)
4. `ords` binding, and how the three value-domain properties survive (`M3.4a`)
5. Install atomicity (`G2.5`)
6. Snapshot isolation level, and any database setting it needs (`R4.5`)
7. Write serialization for `version_id` and the chain append (`H5.4`)
8. Append-only enforcement, or its absence (`M3.17`)
9. Index limits and what is done instead (`P6.4a`)
10. Paging and placeholder syntax
11. Transport security, or why there is no connection (`O10.7`)
12. Every core requirement this port does not satisfy, as a numbered departure

"Not applicable" is an acceptable answer to any of these. Silence is not:
silence and having-not-considered-it look identical on the page.

A departure must **cite what it amends** (`X15.7`) and must not restate
unchanged core requirements (`X15.8`):

```markdown
- **M14.6** `ords` MUST be a `TEXT` column, amending **M3.4**'s array type.
  The value-domain properties of M3.4a survive: negative ordinals appear
  verbatim, the empty path is the two-character string `{}`, and depth is
  unbounded because the encoding is variable-length.
```

> **What skipping this looks like.** `fhir-mssql/spec/14-mssql-dialect.md` and
> `fhir-oracle/spec/14-oracle-dialect.md` are the MySQL annex with three lines
> changed. Both are titled "14. MySQL dialect" and declare the target as "MySQL
> 8.0 or later, InnoDB, `utf8mb4`". The MSSQL one is worse than useless: the
> port's `ddl.rs` is genuine, deliberate T-SQL, so its own specification
> contradicts its own working code, and a reader following the annex would
> reject correct code. Every `M14.x` id in both files is wrong, permanently,
> because ids are never reused (`C0.5`). Tracked as **F-16**.

## Step 3 — `ddl.rs`

Start at `col_sql`. Three bindings are always wrong the obvious way:

**`Numeric` is not a decimal type** (`M3.6a`). `M3.6` requires a decimal's
original textual precision to survive round-trip. `DECIMAL(65,30)` returns
`1.50` as `1.500000000000000000000000000000`; `REAL` cannot hold `1.50`
distinctly from `1.5` at all. Bind to text, and serve range search from a
derived sort column.

**`TextC` must be binary and NO PAD** (`M3.6b`). It backs `:exact` matching and
key identity. SQL Server's default collation is case- *and* accent-insensitive,
so a column left at the default silently acquires fuzzy equality. Under a PAD
SPACE collation `'Smith' = 'Smith '` is true, which widens `:exact` and weakens
primary keys.

**`Jsonb` must not be a JSON type** (`M3.6c`). The hash chain commits to bytes
canonicalized in Rust; a JSON column re-normalizes what it is given, so the
bytes read back would differ from the bytes signed and **every chain would fail
verification**.

Then the rest: `IF NOT EXISTS` support (Oracle has none, so idempotence means a
PL/SQL block swallowing ORA-00955), trigger syntax for append-only, index key
limits, and reserved words.

> **What skipping this looks like.** `fhir-oracle`'s `ddl.rs` is still the MySQL
> emitter — `TEXT`, `TINYINT(1)`, `DATETIME(6)`, `utf8mb4_0900_bin`, none of
> which exist in Oracle. Its eleven MySQL-asserting tests are `#[ignore]`d,
> which is the right call and the reason the code is honest even though the
> README is not (**F-08**).

## Step 4 — point CI at the real engine

Before writing the store, make `scripts/db.sh` and both CI configs provision
**your** engine at the version your annex declares (`O10.12`).

> **What skipping this looks like.** `fhir-mssql` and `fhir-oracle` both start
> `mysql:8.4`, in `scripts/db.sh`, in `.github/workflows/ci.yml`, and in
> `.woodpecker/database.yaml`. Compounding it, `fhir-mssql`'s only live test
> needs `FHIR_MSSQL_TEST_DSN` and **skips silently** without it, which that
> pipeline never sets. So the T-SQL DDL has never run in CI, the build is green,
> and the summary reports a passing database gate (**F-06**).
>
> A test that skips is indistinguishable from a test that passes, in a summary
> (`T11.12`). That is a rule about tests, and here it reappeared one level up,
> about pipelines.

## Step 5 — the DDL test

Before the store: does the generated schema actually execute?

Unit assertions on statement shape catch a stray backquote. They do not catch a
reserved word, an unindexable column, or a trigger the parser rejects. Running
the generated schema through the real engine is what found the SQLite and MySQL
ports' real bugs.

Make it **fail** rather than skip when its DSN is absent in CI.

## Step 6 — the store

Follow the parallel-module pattern: `<engine>.rs` and `<engine>_search.rs`
alongside the shared `lib.rs` and `chain.rs`. Order of implementation, roughly
the order the existing ports used:

1. `open`/`connect`, `init`, `drop_schema`, `installed_checksum`
2. `put`, `get` — round-trip a resource through the live engine
3. `history`, `vread`, `delete`
4. `search`, `search_full`, `search_page`
5. `log_access`, `verify_audit`, `purge`
6. `transact_audited`, conditional operations, `emit_checkpoint`
7. `upgrade` and `backfill_norm` — needed before any fold change ships
   (`O10.4a`)

Two transaction requirements are easy to miss and hard to debug:

- **Every multi-statement read is one snapshot** (`R4.5`). A read touches one
  base table and many child tables; issued independently, a concurrent write
  between them reconstructs a resource that never existed.
- **`version_id` is assigned under a lock** that serializes writers per resource
  id (`H5.4`), because the chain digest of version *n* commits to *n−1* — a race
  produces two rows claiming the same predecessor and a chain that verifies for
  neither.

## Step 7 — tests

Port `concurrency.rs`, `redaction.rs`, and `audit.rs` from `fhir-postgresql`.
These are the tests behind `T11.6`, `T11.7`, and `T11.8` — the concurrency and
audit guarantees, which are the ones §13 maps to HIPAA §164.312(b) and (c).

Only `fhir-postgresql` has them today, which is why four ports carry `?` rather
than `•` in the matrix. Sharing a correct implementation is not evidence that
this port runs it.

And verify each test by **mutation** (`T11.10`): break what it guards, watch it
go red. This matters most for tamper evidence, where a control that cannot fail
is indistinguishable from one that works.

## Step 8 — write the documentation

Not substitute it (`W16.8`).

> **What skipping this looks like.** Every port's README carries the PostgreSQL
> reference's paragraph — "all 7,399 official FHIR® example resources round-trip
> losslessly", "94.8% of R5 search parameters compile", "`fhir-<engine> serve`
> mounts every installed version" — with the engine name swapped. Two of those
> ports have no store. None has a CLI crate. That is a claim about clinical
> software, in a product's own name, that nothing substantiates (**F-01**), and
> it is the most serious finding in the register.

State the port's conformance level (`C0.8`) and do not claim above it
(`C0.11`). A Scaffold port's README says it is a scaffold.

## Step 9 — the matrix

Add a column to [`spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md).
Use `?`, honestly, for everything your test suite does not demonstrate. `?` is
not a soft `•` — it means "plausibly satisfied by shared code, and nothing here
proves it".

## The checklist

```
[ ] crates renamed, descriptions fixed              W16.2, W16.3
[ ] annex written against the X15.6 checklist       X15.6
[ ] ddl.rs, starting at col_sql                     M3.6a-c
[ ] CI and scripts/db.sh provision the real engine  O10.12
[ ] DDL execution test — fails, not skips, in CI    T11.12
[ ] store, in the order above                       R4.5, H5.4
[ ] concurrency, redaction, audit tests ported      T11.6-8
[ ] every test verified by mutation                 T11.10
[ ] documentation written, not substituted          W16.8
[ ] conformance matrix column added, honestly       C0.9
[ ] git remote set to the new repository            W16.15
```

The two lines that decide whether the port is trustworthy are the annex and the
documentation. They are also the two with no compiler behind them, which is why
they are the two that get skipped.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
