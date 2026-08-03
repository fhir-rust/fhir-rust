# fhir-oracle — FHIR in Oracle Database, relationally

The intent: store [FHIR](https://hl7.org/fhir/) resources in Oracle Database as
**real relational tables** — typed columns, child tables, foreign keys, and
check constraints — not JSON blobs.

> ## ⚠ Status: Scaffold. The schema is real and has been executed; no test runs it, and there is no runtime.
>
> This is the earliest of the six ports, and the README says so plainly because
> until 2026-07-31 it did the opposite.
>
> **What works:** the FHIR-specification generator, the shred/reconstruct engine
> — both shared, engine-independent Rust, as correct here as in any other port —
> and, since 2026-08-03, **the DDL emitter**.
>
> The full R5 schema installs on Oracle:
>
> | | |
> | --- | --- |
> | statements applied | 9,636 |
> | tables / indexes | 7,358 / 9,479 |
> | triggers / checks / foreign keys | 158 / 21,540 / 7,039 |
> | **invalid objects** | **0** |
> | unindexable search targets | **0** of 1,947 |
>
> Verified against Oracle AI Database 26ai Free (23.26.2.0.0), in one pass
> (**F-08** closed).
>
> **What is still missing:**
>
> - **There is no store and no driver.** Nothing has been *written* through this
>   schema by the port — only by hand, to prove the append-only guards fire. That
>   gap is why no round-trip claim appears on this page.
> - **No test runs the DDL.** The install above was done by hand with `sqlplus`.
>   `C0.9` counts only tests that run, so the level stays **Scaffold** until a
>   live test exists (**F-51**).
> - **The eleven MySQL-asserting tests in `ddl.rs` are still `#[ignore]`d** and
>   still need replacing (`M14.25`).
> - **There is no local database script and no live CI gate.** Both existed and
>   both provisioned **MySQL**, so a green run proved nothing; they were removed
>   rather than repointed (**F-06**). Nothing in this repository runs in CI at
>   all — see **F-49**.
>
> **Before you install it:** the three version namespaces are three Oracle
> **users** (`M14.5`), and this port does **not** create them (`M14.28`).
> Provision `r3`, `r4` and `r5` with quotas first; a deployment that cannot
> create users cannot install this port.
>
> **What was wrong until 2026-07-31:** this README claimed 7,399 FHIR example
> resources round-tripped through live Oracle and that `fhir-oracle serve`
> mounted a REST API. Neither was ever true — the text was the
> `fhir-postgresql` README with the engine name substituted (**F-01**). The
> [conformance matrix](../spec/databases/conformance-matrix.md) is the status document to
> trust.

## What you can do with it today

Round-trip resources in memory, using the shared engine:

```rust
use std::sync::Arc;
use fhir_oracle_map::model::RelMap;
use fhir_oracle_map::shred;

let map = Arc::new(RelMap::bundled("r5")?);   // compiled in (feature `r5`)
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
  is shared at [`../spec/`](../spec/databases/index.md).
- [`spec/14-oracle-dialect.md`](spec/14-oracle-dialect.md) — the decision list.
- [`../spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md) — what this
  port actually satisfies.
- [`../doc/tutorial-06-porting.md`](../doc/tutorial-06-porting.md) — the porting
  guide, written partly from this port's mistakes.
- **[The book](book/src/SUMMARY.md)** — inherited from the PostgreSQL original
  and **not rewritten for this engine**; read it with that in mind.
- [`plan.md`](plan.md) · [`tasks.md`](tasks.md) · [`CHANGELOG.md`](CHANGELOG.md)

## License

MIT OR Apache-2.0.
