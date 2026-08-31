# Tutorial 1 — your first store

About 15 minutes. No database server required: we use `fhir-sqlite`, which
bundles its engine.

By the end you will have installed a full FHIR® R5 relational schema, written a
resource, read it back byte-for-byte, and found it by an accent-insensitive
search.

## Setup

```toml
# Cargo.toml
[dependencies]
fhir-sqlite-map   = { path = "…/fhir-databases/fhir-sqlite/crates/fhir-sqlite-map" }
fhir-sqlite-store = { path = "…/fhir-databases/fhir-sqlite/crates/fhir-sqlite-store" }
tokio      = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

Two crates, and you only ever call the second. `-map` gives you the relational
map type; `-store` gives you the store. (`-gen` is the third crate in the
workspace and you will not need it — it regenerates the map from the FHIR
specification packages, and its output is already committed.)

## Step 1 — load the relational map

The map is the compiled form of a FHIR version: every resource type, every
element, which table and column each becomes. It ships gzipped under `assets/`,
so nothing here needs the FHIR specification packages or a network (`G2.1`).

```rust
use std::sync::Arc;
use fhir_sqlite_map::model::RelMap;

let map = Arc::new(RelMap::bundled("r5")?);   // compiled in
```

Three versions are available — `r5`, `r4`, `r3` — and they are independent
(`S1.2`). A process can hold all three; each gets its own database file.

## Step 2 — open and install

```rust
use fhir_sqlite_store::sqlite::SqliteStore;

let store = SqliteStore::open("clinic.sqlite", map).await?;
let statements = store.init("r5-baseline").await?;
println!("installed {statements} statements");
```

`init` creates the schema — around **7,400 tables** for R5. That number is not a
typo and not a design accident: every repeating element of every resource type
gets its own table, which is what makes the data queryable as SQL rather than as
JSON paths. Tutorial 2 explains the shape.

Three properties worth knowing now:

- **`init` is idempotent** (`G2.5`). It records the checksum you passed and
  no-ops if you run it again with the same one. Run it with a *different* one
  against an installed schema and it refuses, rather than half-migrating.
- **It is all-or-nothing.** SQLite's DDL is transactional, so a failed install
  leaves the schema exactly as it was. (PostgreSQL cannot do this in one
  transaction — 7,355 tables exhaust its lock budget — so it stages the install
  under a temporary schema and renames it into place. Same guarantee, different
  route.)
- **The checksum string is yours to choose**, but it should identify the
  artifact you installed from. `assets/CHECKSUMS.txt` has the real ones.

## Step 3 — write a resource

```rust
use fhir_sqlite_store::Audit;

let patient = serde_json::json!({
    "resourceType": "Patient",
    "id": "example",
    "name": [{ "family": "Ærø", "given": ["Anna", "Marie"] }],
    "birthDate": "1974-12",
    "active": true
});

let put = store.put(&patient, &Audit::cli()).await?;
println!("version {}", put.version_id);
```

Two things just happened that are easy to miss.

**The resource was shredded, not stored.** It is now rows in `patient`,
`patient_name`, and `patient_name_given` — typed columns, not JSON. Nothing in
the database holds the document you passed in.

**You had to say who did it.** `Audit::cli()` is one of three deliberate
choices:

```text
Audit::principal(actor, source)  // an authenticated identity, from your perimeter
Audit::cli()                     // a local operator ran this
Audit::unattributed()            // nobody did, and the record will say so
```

There is no default (`PR12.3a`). An API that accepted a write with no
attribution and recorded something plausible would turn a deployment mistake
into a permanent false record, and a false attribution survives review in a way
a missing one does not.

## Step 4 — read it back

```rust
let back = store.get("Patient", "example").await?.unwrap();
assert_eq!(back, patient);
```

That assertion is the whole point of the project (`R4.2`). The resource was
decomposed into rows across three tables and rebuilt from them, and the result
is the same JSON — including:

- **`"1974-12"`**, a partial date. It cannot live in a `DATE` column without
  becoming `1974-12-01`, so it is stored verbatim as text with a derived sort
  column alongside for range queries (`M3.6`).
- **Array order.** `["Anna", "Marie"]` comes back in that order, from a table
  where row order is arbitrary, because the order is carried in the `ords`
  column rather than in the storage.
- **Decimal precision**, if you had used one. `1.50` stays `1.50`, which is why
  decimals are not stored in `DECIMAL` or `REAL` columns (`M3.6a`).

Try adding an element FHIR does not define, and the write fails naming the path
(`R4.3`). Silent data loss is disqualifying in a clinical system, so anything
the map does not recognise is an error rather than a dropped field.

## Step 5 — search

```rust
let hits = store
    .search("Patient", &[("name".into(), "aero".into())], 50, 0)
    .await?;
assert_eq!(hits, vec!["example".to_string()]);
```

`"aero"` found `"Ærø"`. Not by a `LIKE` heuristic — by a **fold** applied
identically on both sides.

Every string search column has a companion `_norm` column holding the folded
value, computed in Rust at write time: decompose to NFD, drop combining marks,
lowercase, drop marks again, then expand the letters that have no decomposition
(`æ`→`ae`, `ø`→`o`, `ß`→`ss`, and a dozen more). The search term goes through
the same function. So there is exactly **one** definition of "the same string"
in the system, rather than one in SQL and one in Rust that must agree for every
codepoint in Unicode (`P6.6`, `L1`).

Doing it in Rust rather than in SQL also means it works identically on all six
engines, with no extension to install and no dependence on the database's
collation tables or Unicode version (`X15.4`).

The two extra arguments are `count` and `offset`. `count` defaults to 50 and
caps at 1000 (`P6.3`).

## Step 6 — see the history

```rust
let mut updated = patient.clone();
updated["active"] = serde_json::json!(false);
store.put(&updated, &Audit::cli()).await?;

for entry in store.history("Patient", "example").await? {
    println!("v{} {} at {}", entry.version_id, entry.op, entry.last_updated);
}
// v1 C at …
// v2 U at …
```

Every write appends a history row carrying the whole resource, who did it, and a
hash chained to the previous version's (`H5.1`, `M3.15`, `M3.16`). Delete is
soft: the base rows go, the history stays readable.

Tutorial 5 covers what that chain does and does not protect against — the honest
version, which is narrower than it first sounds.

## What you have

```
clinic.sqlite            ← what you opened
fhir-r5.sqlite           ← the actual schema, one file per FHIR version
```

Inspect it with any SQLite client:

```sh
sqlite3 fhir-r5.sqlite "SELECT id, birth_date, active FROM patient;"
sqlite3 fhir-r5.sqlite "SELECT rid, ords, family FROM patient_name;"
sqlite3 fhir-r5.sqlite "SELECT rid, ords, value FROM patient_name_given;"
```

Those are ordinary tables with ordinary columns. That is the product.

## Next

- [Tutorial 2 — the storage model](tutorial-02-storage-model.md): what `ords`
  is, why `patient_name_given` exists, and how extensions fit.
- [Tutorial 3 — querying with SQL](tutorial-03-querying-sql.md).
- Using PostgreSQL instead? The API differs only at the constructor —
  `Store::connect(cfg, map)` — and everything above applies unchanged.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
