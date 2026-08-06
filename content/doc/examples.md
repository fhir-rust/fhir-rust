# Examples

Short recipes against the API as it exists. Mostly `fhir-sqlite`, because it
needs no server; the shape is the same on every port.

There is **no CLI and no server crate** in this monorepo (`C0.18`), so every
example here is library code.

## Open a store

```rust
use std::sync::Arc;
use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::sqlite::SqliteStore;

let map   = Arc::new(RelMap::bundled("r5")?);   // compiled in (feature `r5`)
let store = SqliteStore::open("clinic.sqlite", map).await?;
store.init("r5-baseline").await?;
```

PostgreSQL:

<!-- not compiled: This one is `fhir-postgresql`, not `fhir-sqlite` like the rest of the page — a different crate and a different connect signature. -->
```rust,ignore
use fhir_postgresql_store::{Store, pg_config};

let cfg   = pg_config(Some("host=localhost user=you dbname=clinic"))?;
let store = Store::connect(cfg, map).await?;
store.init("r5-baseline").await?;
```

`Store::connect` reads the SSL policy from the environment (`PGSSLMODE`,
`PGSSLROOTCERT`). `verify-full` is the documented production setting (`O10.7`).

## Install only if not already installed

```rust
match store.installed_checksum().await? {
    Some(c) if c == want => { /* already there */ }
    Some(other) => anyhow::bail!("schema was installed from {other}, not {want}"),
    None => { store.init(want).await?; }
}
```

`init` does this itself (`G2.5`) — it no-ops on a matching checksum and refuses
on a mismatch rather than half-migrating. The explicit form is for when you want
to report the state rather than act on it.

## Bulk load NDJSON

```rust
use std::io::BufRead;

let audit = fhir_sqlite_store::Audit::cli();
let file  = std::io::BufReader::new(std::fs::File::open("export.ndjson")?);

let (mut ok, mut failed) = (0u64, 0u64);
for (n, line) in file.lines().enumerate() {
    let line = line?;
    if line.trim().is_empty() { continue; }
    let resource: serde_json::Value = serde_json::from_str(&line)?;
    match store.put(&resource, &audit).await {
        Ok(_)  => ok += 1,
        Err(e) => { failed += 1; eprintln!("line {}: {e}", n + 1); }
    }
}
println!("{ok} loaded, {failed} failed");
if failed > 0 { std::process::exit(1); }
```

Report per-resource errors and exit non-zero. A load that swallows failures and
reports success is the same class of problem as a test that skips silently.

Each `put` is one transaction (`R4.4`). There is no batch mode that trades that
away.

## Round-trip without a database

Shred and reconstruct are pure functions over the map, so you can check fidelity
with no store at all:

<!-- not compiled: Continues the previous block; `resource` and `recon_in` come from there. -->
```rust,ignore
use fhir_sqlite_map::{shred, reconstruct};
use fhir_sqlite_map::reconstruct::ReconIn;

let rm  = map.resource("Patient").expect("Patient in map");
let out = shred(rm, &resource)?;
// … feed out's rows back in as a ReconIn …
let back = reconstruct(rm, &recon_in, Some("example"))?;
assert_eq!(back, resource);
```

This is what the corpus and property tests do (`T11.1`), and it is the fastest
way to check that a resource shape survives before involving an engine.

## Inspect what a resource shreds into

```rust
// `RelMap` has no `resource()` accessor and no `tables` field — the tables
// belong to each `ResourceMap`, which is what the shredder walks.
let rm = map.resources.get("Patient").expect("Patient is in the map");
let out = shred(rm, &patient)?;

// `ShredOut` is a flat `rows: Vec<Row>`, each row naming its table by index —
// not a per-table grouping. Extensions, deep spills and contained resources
// are separate vectors beside it.
let mut per_table = std::collections::BTreeMap::<u32, usize>::new();
for row in &out.rows {
    *per_table.entry(row.table).or_default() += 1;
}
for (table_idx, n) in per_table {
    println!("{} — {n} rows", rm.tables[table_idx as usize].name);
}
println!(
    "{} extension, {} deep, {} contained",
    out.ext.len(),
    out.deep.len(),
    out.contained.len()
);
```

Useful when a round-trip fails: the residue reported by reconstruction
(`R4.7`) tells you a row went unconsumed, and this tells you which table it
was in.

## Optimistic concurrency

```rust
let got = store.get("Patient", "example").await?.expect("exists");
let current = got["meta"]["versionId"].as_str().unwrap().parse::<i64>()?;

let mut updated = got.clone();
updated["active"] = serde_json::json!(false);

match store.put_audited(&updated, Some(current), &audit).await {
    Ok(out) => println!("now v{}", out.version_id),
    // There is no `is_conflict()`; the variant is matched directly.
    Err(StoreError::Conflict { expected, found }) => {
        println!("someone else wrote first: expected v{expected}, found v{found}");
    }
    Err(e) => return Err(e.into()),
}
```

`expected_version` is the precondition. N racing updates must produce exactly
one success and N−1 conflicts (`T11.6`).

## Conditional create

```rust
use fhir_sqlite_store::CondCreate;

let criteria = [("identifier".to_string(), "http://acme.org/mrn|12345".to_string())];
// The resource type is its own argument, and `Created` carries the whole
// `PutOutcome` rather than an id. The "too many matches" variant is
// `Multiple`, and it carries nothing — the count is not part of the 412.
match store
    .conditional_create_audited("Patient", &criteria, &patient, &audit)
    .await?
{
    CondCreate::Created(out) => println!("created {} at v{}", out.id, out.version_id),
    CondCreate::Existing(id) => println!("already there: {id}"),
    CondCreate::Multiple => println!("criteria match more than one — refusing"),
}
```

N racing conditional creates with identical criteria must produce exactly one
resource. `Ambiguous` is not an error to swallow: it means the criteria do not
identify a single record, and creating anyway would duplicate a patient.

Available on `fhir-postgresql` and `fhir-sqlite` only.

## Search with paging

```rust
let params = [("name".to_string(), "smi".to_string())];

let mut offset = 0;
loop {
    let page = store.search("Patient", &params, 100, offset).await?;
    if page.is_empty() { break; }
    for id in &page { println!("{id}"); }
    offset += page.len() as i64;
}
```

For large result sets prefer `search_page`, which is cursor-based. Offset paging
re-scans, and the results shift under concurrent writes.

## Verify the audit chain

```rust
let breaks = store.verify_audit().await?;
if breaks.is_empty() {
    println!("all chains verify");
} else {
    for b in &breaks {
        println!("{}/{} v{} — break under {}", b.rtype, b.id, b.version_id, b.algorithm);
    }
    std::process::exit(1);
}
```

Each algorithm is reported separately (`M3.16a`), never reduced to one verdict,
so a reader can rely on whichever their regime recognises.

**A break is not the only interesting outcome.** A missing tag, a tag naming a
key this process does not hold, and a malformed tag are each reported as what
they are and are *not* tampering (`M3.16b`). Treating a key-distribution problem
as a forgery burns an incident response.

## Emit a checkpoint

> **`fhir-postgresql` only.** `chain_witness` exists on that port and no other
> — the [conformance matrix](../spec/databases/conformance-matrix.md) row says
> `• — — — — —`. On the other five this does not compile; `fhir-sqlite` emits
> checkpoints but has no witness (`M3.16c`).

```rust,ignore
// fhir-postgresql only — see the note above.
let witness = store.chain_witness().await?;
println!("{witness}");
```

Record it somewhere the database cannot reach (`M3.16c`). A chain missing its
most recent version verifies perfectly — only an off-box value catches that.

## Erasure

```rust
// The reason travels on the Audit, not as a separate argument.
let audit = Audit::cli().with_reason(Some("GDPR Art.17 request #4711".into()));
store.purge("Patient", "example", &audit).await?;
```

Removes history rows and leaves a tombstone recording who, what, when, why, and
which chain it terminated (`M3.18`). Emit a checkpoint immediately afterwards:
it is what separates a recorded intentional removal from the unrecorded kind.

## Several FHIR versions in one process

```rust
async fn open(version: &str) -> anyhow::Result<SqliteStore> {
    // The map for each version ships inside the crate behind its own feature.
    let map = Arc::new(RelMap::bundled(version)?);
    let store = SqliteStore::open(format!("clinic-{version}.sqlite"), map).await?;
    store.init(&format!("{version}-baseline")).await?;
    Ok(store)
}

let (r5, r4, r3) = (open("r5").await?, open("r4").await?, open("r3").await?);
```

Versions are independent and never share a table (`S1.2`). SQLite gives each its
own file; PostgreSQL and SQL Server use schemas; MySQL and MariaDB use
databases.

## Advisory referential integrity

There are no cross-resource foreign keys (`M3.10`) — FHIR permits dangling
references. To find them, query:

```sql
SELECT o.id, o.subject_ref_type, o.subject_ref_id
  FROM observation o
  LEFT JOIN patient p ON p.id = o.subject_ref_id
 WHERE o.subject_ref_type = 'Patient'
   AND p.id IS NULL;
```

A report, not a constraint. Enforcing it would make load order matter and reject
real-world data.

## Check the fold

```rust
use fhir_sqlite_map::fold::fold;

assert_eq!(fold("Ærø"),      "aero");   // needs the L6 expansion
assert_eq!(fold("Muñoz"),    "munoz");  // NFD alone suffices
assert_eq!(fold("Straße"),   "strasse");
assert_eq!(fold(&fold("Ærø")), "aero"); // idempotent — L5
```

Identical on every port (`X15.4`). If you are debugging a search that should
match and does not, fold both sides by hand first — it is usually the answer.
