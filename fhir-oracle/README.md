# fhir-oracle — FHIR in Oracle Database, relationally

The intent: store [FHIR](https://hl7.org/fhir/) resources in Oracle Database as
**real relational tables** — typed columns, child tables, foreign keys, and
check constraints — not JSON blobs.

> ## Status: Store. The schema is real and executed; the store now connects and its full surface is live-tested — with one confirmed gap.
>
> This was the last of the six ports to reach Store level, and it did so the
> same way `fhir-mssql` did: by connecting a real store to a real server and
> running it (**F-68**), not by reading the code more carefully.
>
> **What works:** the FHIR-specification generator, the shred/reconstruct engine
> — both shared, engine-independent Rust, as correct here as in any other port —
> **the DDL emitter** (since 2026-08-03), and, since 2026-08-04, **the store**:
> `connect`/`init`/`put`/`get`/`delete`/`history`/`vread`/`verify_audit`/
> `purge`/`log_access`/`search`, live-tested against
> `gvenzl/oracle-free:23-slim-faststart` in `tests/oracle_store.rs` —
> **7 of 7 tests pass, 0 ignored.**
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
> **Getting the store connected found and fixed five real defects**, none
> visible from reading the code — full account in `audit.md` **F-68**:
> Oracle folds an unquoted username to uppercase for session identity, so the
> three version namespaces must be uppercase Oracle users, not the lowercase
> `r3`/`r4`/`r5` every other port uses; `R4.5`'s presumed mechanism
> (`SET TRANSACTION READ ONLY`) fails every read with `ORA-01466` on any
> session that has run DDL; two `insert_row` call sites double-qualified the
> schema (`ORA-00926`); timestamp/date values bound as plain strings relied
> on session NLS settings (`ORA-01843`); and token search bound a boolean as
> text (`ORA-01722`).
>
> **What is still missing:**
>
> - **`R4.5` (snapshot reads under concurrent writers) has no working
>   mechanism.** The one candidate this port's annex named was tried live and
>   removed after it broke every read. `get` currently reads with no
>   protection against torn reads under concurrent writers. This is an open,
>   confirmed gap, not merely an unverified one — see `M14.19`.
> - **No concurrency test.** `H5.4` (serialized `version_id`) is implemented
>   via `SELECT … FOR UPDATE`, but no test races concurrent writers against
>   it the way `fhir-mssql`'s and `fhir-mysql`'s `concurrency.rs` do.
> - **No `redaction.rs`.** (`upgrade` and `backfill_norm` left this list
>   2026-08-09 — `tests/upgrade.rs`, 9 live tests, closing **F-15**'s last
>   port; see `M14.35`–`M14.37` for what Oracle made different about them.)
> - **Transport security is undecided** (`O10.7`, `M14.22`) — the live tests
>   above connect over a plain local port with no encryption configured
>   either way.
> - The eleven `#[ignore]`d MySQL-asserting tests in `ddl.rs` are **gone** —
>   replaced with Oracle-asserting ones under **F-08** (an earlier revision of
>   this line said they still needed replacing after they had been, F-79).
> - **There is no live CI gate.** It provisioned **MySQL** and was removed
>   rather than repointed (**F-06**); nothing in this repository runs in CI at
>   all — see **F-49**. `scripts/db.sh` now exists for local use (`up`/`down`/
>   `status`), generating an ephemeral probe crate the same way `fhir-mssql`
>   does, but nothing invokes it automatically.
>
> **Before you install it:** the three version namespaces are three Oracle
> **users**, and they must be **uppercase and unquoted** — `CREATE USER R5
> IDENTIFIED BY ...`, not `r5` or `"r5"` (`M14.5`). This port does **not**
> create them (`M14.28`). Provision `R3`, `R4` and `R5` with quotas first
> (`CREATE SESSION, CREATE TABLE, CREATE TRIGGER, CREATE PROCEDURE, CREATE
> SEQUENCE, UNLIMITED TABLESPACE`); a deployment that cannot create users
> cannot install this port.
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

Store resources in a real Oracle database — Instant Client on the host is
required (see `scripts/db.sh`'s header comment for the download):

```sh
DYLD_LIBRARY_PATH=~/lib scripts/db.sh up      # prints FHIR_ORACLE_TEST_*
DYLD_LIBRARY_PATH=~/lib cargo test -p fhir-oracle-store --test oracle_store
```

```rust
use std::sync::Arc;
use fhir_oracle_map::model::RelMap;
use fhir_oracle_store::oracle::OracleStore;

let mut map = RelMap::bundled("r5")?;
map.schema = "R5".to_string();               // uppercase — see M14.5 below
let store = OracleStore::connect("r5", "password", "localhost:1521/FHIR", Arc::new(map)).await?;
store.init("checksum").await?;
let put = store.put(&patient_json, &fhir_oracle_store::Audit::cli()).await?;
```

## What has to be decided

The [dialect annex](spec/14-oracle-dialect.md) is written as a decision list.
Most of it is now decided and live-verified; transport security and install
atomicity are not.

**Decided: the engine floor is Oracle 12.2.** This one could not be left open,
because the generator had already assumed an answer. Oracle identifiers were
**30 bytes before 12.2** and 128 after, so the shared 63-byte budget is legal on
12.2+ and *silently truncating* below it — which would collapse distinct table
names into one, exactly the collision the identifier budget exists to prevent.
The port had inherited the constant without inheriting a reason (**F-09**).

**Decided and live-verified:**

| Question | Answer |
| --- | --- |
| **`VARCHAR2` vs `CLOB`** | `VARCHAR2` maxes at 4000 bytes; longer values are `CLOB`, indexed and compared through the `U1`–`U10` bounded/digest adjunct pair — a plain `CLOB` cannot be indexed or `=`-compared at all. |
| Namespaces | Three Oracle **users**, one per version — and they MUST be **uppercase and unquoted**. Oracle folds an unquoted `CREATE USER` to uppercase for session identity regardless of how the map's schema is spelled; a lowercase schema fails `ORA-01031` against every statement (`M14.5`). |
| `Bool` | `NUMBER(1)` + `CHECK`, binding as `i64` 0/1 — including in search predicates, which MUST NOT bind the string `"true"`/`"false"` (`ORA-01722`, `M14.34`). |
| `Timestamptz` / `Date` | `TIMESTAMP(6)` / `DATE`, bound through a typed `chrono` value — a plain string relies on session NLS settings and fails `ORA-01843` (`M14.34`). |
| Idempotence | A PL/SQL block swallowing ORA-00955, which makes every statement a block. |
| Erasure flag | `SYS_CONTEXT` with an **application context** — a heavier dependency than any other port's, needing its own database object and trusted package. |
| A driver | The `oracle` crate (ODPI-C/OCI), synchronous, wrapped in `spawn_blocking`. Runs on arm64: `gvenzl/oracle-free:23-slim-faststart` boots in ~13 seconds. |

**Still open:**

| Question | Why it is not mechanical |
| --- | --- |
| **`R4.5` snapshot reads** | The presumed answer, `SET TRANSACTION READ ONLY`, was tried live and **fails** — `ORA-01466` on any session that has run DDL. No replacement is proposed yet (`M14.19`). |
| `TextC` | Collation and `NLS_SORT`; note `CHAR` is blank-padded (`M14.10`). |
| `ords` | `VARCHAR2` or `RAW`. Watch Oracle's `''`-is-`NULL` rule against a `NOT NULL` key column. |
| Install atomicity at scale | `G2.5` — the live install so far has been a single hand-run script, not tested for partial failure (`M14.18`). |
| Transport security | Oracle Net encryption and/or TLS; undecided (`O10.7`, `M14.22`). |

## What does not need porting

Worth stating, because it is most of the system and it is already correct here:
the relational map, shredding, reconstruction, the accent fold, canonical JSON,
and the entire generator are **byte-identical across all six ports** and operate
on Rust types without ever emitting SQL. A CI gate checks that they have not
diverged.

`ddl.rs` and the store are both now written and live-verified. What remains
is narrower: `R4.5`, a concurrency test, redaction, and transport
security — see "Still open" above. (Upgrade/backfill left this list
2026-08-09.)

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
