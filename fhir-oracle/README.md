# fhir-oracle — FHIR in Oracle Database, relationally

The intent: store [FHIR](https://hl7.org/fhir/) resources in Oracle Database as
**real relational tables** — typed columns, child tables, foreign keys, and
check constraints — not JSON blobs.

> ## ⚠ Status: Scaffold. Nothing in this port is Oracle yet.
>
> This is the earliest of the six ports, and the README says so plainly because
> until 2026-07-31 it did the opposite.
>
> **What works:** the FHIR-specification generator and the shred/reconstruct
> engine. Both are shared, engine-independent Rust and are as correct here as in
> any other port — resources round-trip losslessly **in memory**.
>
> **What is not Oracle:**
>
> - **`ddl.rs` is still the MySQL emitter.** It produces `TEXT`, `TINYINT(1)`,
>   `DATETIME(6)`, `LONGTEXT`, and `COLLATE utf8mb4_0900_bin` — none of which
>   exist in Oracle — and its comments still discuss MySQL's 2038 `TIMESTAMP`
>   range. Its eleven MySQL-asserting tests are `#[ignore]`d so that a green run
>   cannot be mistaken for Oracle conformance. Tracked as
>   [`audit.md`](../spec/audit.md) **F-08**.
> - **There is no store**, and no driver in the workspace.
> - **There are no map tests** — `crates/fhir-oracle-map/tests/` does not exist.
> - **There is no local database script and no live CI gate.** Both existed and
>   both provisioned **MySQL**, so a green run proved nothing; they have been
>   removed rather than repointed, because there is nothing yet to point them at
>   (**F-06**).
>
> **What was wrong until 2026-07-31:** this README claimed 7,399 FHIR example
> resources round-tripped through live Oracle and that `fhir-oracle serve`
> mounted a REST API. Neither was ever true — the text was the
> `fhir-postgresql` README with the engine name substituted (**F-01**). The
> [conformance matrix](../spec/conformance-matrix.md) is the status document to
> trust.

## What you can do with it today

Round-trip resources in memory, using the shared engine:

```rust
use std::sync::Arc;
use fhir_oracle_map::model::RelMap;
use fhir_oracle_map::shred;

let bytes = std::fs::read("assets/fhir-oracle-relmap-r5.json.gz")?;
let map = Arc::new(RelMap::from_gz_bytes(&bytes)?);
let rows = shred(map.resources.get("Patient").unwrap(), &patient)?;
```

```sh
cargo test --workspace   # unit and round-trip tests; no engine needed
```

Do **not** call `ddl::ddl(&map)` expecting Oracle DDL. It emits MySQL.

## What has to be decided

The [dialect annex](spec/14-oracle-dialect.md) is written as a decision list
rather than a specification, because that is the honest shape for a port where
nothing has been decided. It settles one item and opens the rest deliberately.

**Decided: the engine floor is Oracle 12.2.** This one could not be left open,
because the generator had already assumed an answer. Oracle identifiers were
**30 bytes before 12.2** and 128 after, so the shared 63-byte budget is legal on
12.2+ and *silently truncating* below it — which would collapse distinct table
names into one, exactly the collision the identifier budget exists to prevent.
The port had inherited the constant without inheriting a reason (**F-09**).

**Open, and each one is real work:**

| Question | Why it is not mechanical |
| --- | --- |
| **`VARCHAR2` vs `CLOB`** | `VARCHAR2` maxes at 4000 bytes; longer values must be `CLOB`, and **a `CLOB` cannot be indexed or compared with `=`**. This is the SQL Server port's `NVARCHAR(MAX)` problem, sharper: `NVARCHAR(MAX)` still compares, so those searches merely scan — an Oracle `CLOB` would make some searches *not work at all*. A FHIR `string` has no length bound, so this cannot be resolved by declaring one. |
| Namespaces | Oracle conflates user and schema: three users, or one user with prefixed tables that spend bytes from an already-tight budget? |
| `Bool` | No boolean type before 23ai. `NUMBER(1)` + `CHECK` is the intended answer; requiring 23ai was considered and rejected. |
| Idempotence | **No `IF NOT EXISTS` anywhere.** Every statement becomes a PL/SQL block swallowing ORA-00955, which also complicates install atomicity. |
| Erasure flag | No session variable. `SYS_CONTEXT` with an **application context** — a heavier dependency than any other port's, needing its own database object and trusted package. |
| `ords` | `VARCHAR2` or `RAW`. Watch Oracle's `''`-is-`NULL` rule against a `NOT NULL` key column. |
| A driver | Undecided — and blocked on whether an Oracle Database Free image runs on arm64 at all, which decides whether live verification can be promised on a developer machine. |

## What does not need porting

Worth stating, because it is most of the system and it is already correct here:
the relational map, shredding, reconstruction, the accent fold, canonical JSON,
and the entire generator are **byte-identical across all six ports** and operate
on Rust types without ever emitting SQL. A CI gate checks that they have not
diverged.

The work is `ddl.rs` and a store. That is a real body of work, and it is bounded.

## Documentation

- [`spec/index.md`](spec/index.md) — this port's spec index; the normative core
  is shared at [`../spec/`](../spec/index.md).
- [`spec/14-oracle-dialect.md`](spec/14-oracle-dialect.md) — the decision list.
- [`../spec/conformance-matrix.md`](../spec/conformance-matrix.md) — what this
  port actually satisfies.
- [`../doc/tutorial-06-porting.md`](../doc/tutorial-06-porting.md) — the porting
  guide, written partly from this port's mistakes.
- **[The book](book/src/SUMMARY.md)** — inherited from the PostgreSQL original
  and **not rewritten for this engine**; read it with that in mind.
- [`plan.md`](plan.md) · [`tasks.md`](tasks.md) · [`CHANGELOG.md`](CHANGELOG.md)

## License

MIT OR Apache-2.0.
