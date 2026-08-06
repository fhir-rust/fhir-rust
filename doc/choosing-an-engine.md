# Choosing an engine

Six ports, one specification. They differ in two files — `map/src/ddl.rs` and
the `store` crate — and in how much has been verified.

## The short answer

| If you want | Use | Because |
| --- | --- | --- |
| Production today | **`fhir-postgresql`** | the only port whose test suite substantiates its claims |
| Embedded, no server | **`fhir-sqlite`** | one file, bundled engine, always-runnable tests |
| An existing MySQL/MariaDB estate | `fhir-mysql`, `fhir-mariadb` | native stores and search, live CI gates |
| SQL Server | `fhir-mssql` | native store and search, live-verified incl. `upgrade` (**F-65**); note the TLS advisory risk (**F-67**) |
| Oracle | `fhir-oracle`, cautiously | native store and search, live-verified (**F-68**) — but no `upgrade`, no concurrency/redaction tests, and `R4.5` snapshot reads are a confirmed open gap |

## Status in detail

| | pg | sqlite | mysql | mariadb | mssql | oracle |
| --- | :-: | :-: | :-: | :-: | :-: | :-: |
| Conformance level | Reference | Store | Store | Store | Store | Store |
| Dialect annex exists and is real | • | • | • | • | • | • |
| Store implementation | • | • | • | • | • | • |
| Search | • | • | • | • | • | • |
| History, audit, chain | • | • | • | • | • | • |
| Transaction bundles | • | ~ | — | — | — | — |
| Conditional create/delete | • | • | — | — | — | — |
| `upgrade` / backfill | • | • | • | • | • | — |
| `chain_witness`, re-sign | • | — | — | — | — | — |
| Concurrency / redaction / audit **tests** | • | • | • | • | • | — |
| CI runs the right engine | ~ | ~ | ~ | ~ | ~ | — |
| Dialect annex describes the right engine | • | • | • | • | • | • |

(The CI row is `~` everywhere because per-family workflows are inert in the
monorepo — the workflow files provision the right engines but nothing runs
them until they are consolidated rootward, **F-49**.)

Full detail: [conformance matrix](../spec/databases/conformance-matrix.md).

The row that surprises people is the tests row. `fhir-sqlite`, `fhir-mysql` and
`fhir-mariadb` have working stores built on the same shared engine as
PostgreSQL's, and each now carries `concurrency.rs`, `redaction.rs`,
`roundtrip_types.rs` and `upgrade.rs` — 102 to 105 tests apiece, green against
their live engines (measured 2026-08-03).

What they still lack is a dedicated **audit** suite of PostgreSQL's depth, and
none has been run repeatedly enough to claim determinism (`T11.15`). This
paragraph said "none has a concurrency, redaction, or audit test" until
2026-08-03, which had stopped being true (**F-63**).

## What each engine costs you

The `ColTy` bindings, which is where the engines actually differ:

| `ColTy` | PostgreSQL | SQLite | MySQL | MariaDB | SQL Server |
| --- | --- | --- | --- | --- | --- |
| `Bool` | `boolean` | `INTEGER` | `TINYINT(1)` | `TINYINT(1)` | `BIT` |
| `Numeric` | `numeric` | `TEXT` | `TEXT` | `TEXT` | `NVARCHAR(MAX)` |
| `Text` | `text` | `TEXT` | `TEXT` | `TEXT` | `NVARCHAR(MAX)` |
| `TextC` | `text COLLATE "C"` | `TEXT COLLATE BINARY` | `…utf8mb4_0900_bin` | `…utf8mb4_nopad_bin` | `NVARCHAR(450) …BIN2` |
| `Date` | `date` | `TEXT` ISO | `DATE` | `DATE` | `DATE` |
| `Timestamptz` | `timestamptz` | `TEXT` ISO UTC | `DATETIME(6)` | `DATETIME(6)` | `DATETIME2(6)` |
| `Jsonb` | `jsonb` | `TEXT` | `LONGTEXT` | `LONGTEXT` | `NVARCHAR(MAX)` |
| `ords` | `smallint[]` | `TEXT` | `TEXT` | `TEXT` | `VARBINARY(255)` |

### PostgreSQL

**The reference.** Everything is implemented and the test suite proves it:
`concurrency.rs`, `audit.rs`, `redaction.rs`, `upgrade.rs`, `live.rs`,
`m2_semantics.rs`, `search_semantics.rs`, `bench.rs`, against live PostgreSQL 18
in CI. Measured: 7,399 example resources round-trip losslessly, 94.8% of R5
search parameters compile, 6,146 resources/sec bulk load, 1.18 ms reads.

The only engine with a **native array type**, so `ords` is `smallint[]` and
PostgreSQL-only subscript idioms (`ords[1] = 1`) work here and nowhere else.

Costs: a server, and `max_locks_per_transaction` means installing 7,355 tables
needs a staged schema and a rename rather than one transaction.

The hash-chain pre-image used to be derived in SQL here, so a PostgreSQL chain
could not be verified by another port. That was **F-07** and it is **fixed**:
`canon.rs` is shared and identical in all six, so a chain written by any port
verifies in any other ([`audit.md`](../spec/databases/audit.md)).

### SQLite

**The embeddable one.** No server, one file per FHIR version, engine bundled and
pinned rather than whatever the host ships. Its tests need no environment
variables and always run — which, as its own test header notes, means a green
run there proves more than a green run in the inherited PostgreSQL suites.

Costs: `transact_audited` returns `Unsupported` deliberately (a compensating
unwind is not atomic, and refusing is better than pretending); no `upgrade`;
`chain_witness` and re-signing unimplemented; numeric range search works via
`CAST(… AS REAL)`, which is correct but gives up the index. And the `ords`
subscript idiom the book teaches does not work on a `TEXT` column.

Concurrency is SQLite's, so WAL mode and a single writer.

### MySQL and MariaDB

Native stores with search, live CI gates on `mysql:8.4` and `mariadb:11.4`.
Deliberately **independent** despite the shared ancestry (`M14.0a`–`M14.0c`) —
neither must read the other's schema, and neither holds back syntax the other
lacks.

The visible difference is the collation: `utf8mb4_0900_bin` on MySQL,
`utf8mb4_nopad_bin` on MariaDB. Both are their engine's spelling of the NO PAD
binary property `M3.6b` requires; `utf8mb4_bin` would be wrong, because it is
PAD SPACE and would make `'Smith' = 'Smith '` true.

Costs: no `transact_audited`, no conditional operations, no
checkpoint (`upgrade`/`backfill_norm` exist and are live-verified — **F-15**
closed here). `DATETIME(6)` rather than `TIMESTAMP`, because `TIMESTAMP` converts
on session time zone and its range ends in 2038.

### SQL Server

**Store level since 2026-08-04** (**F-65** — an earlier revision of this
section began "Not usable yet"). A real `tiberius` store with search,
live-verified against `azure-sql-edge` by 33 tests, 0 ignored, including
`upgrade`/`backfill_norm` (this port's `upgrade` is genuinely one
transaction — T-SQL DDL is transactional, `M14.35`). `R4.5` snapshot reads
needed two live attempts: `READ_COMMITTED_SNAPSHOT` alone still tore;
`SET TRANSACTION ISOLATION LEVEL SNAPSHOT` on a dedicated database is what
works. What to weigh before choosing it:

- **The TLS advisory risk (`F-67`)**: three unpatched `rustls-webpki` CVEs
  reach the shipping store crate, and `native-tls` fails the handshake — a
  standing risk awaiting an owner decision, so `O10.7` is `!` in the matrix.
- Verified only against `azure-sql-edge`, not full SQL Server (`M14.31`).
- A token's `system`/`code` are `NVARCHAR(MAX)` and are dropped from their
  index, so those searches scan. The intended fix is a persisted computed
  column holding the leading 450 characters (`P6.4a`).
- No `put_audited`, `transact_audited`, or conditional operations.

### Oracle

**Store level since 2026-08-04** (**F-68** — an earlier revision of this
section said "Scaffold only, and nothing in it is Oracle"). `ddl.rs` is a
real Oracle emitter now (**F-08** fixed): the full R5 schema — 9,636
statements — installs on 26ai with 0 invalid objects, and the store (the
`oracle` ODPI-C crate, synchronous, wrapped in `spawn_blocking`) runs its
CRUD/history/search/audit surface live, 7/7 tests, 0 ignored. The caveats
are sharper here than anywhere else:

- **`R4.5` snapshot reads are a confirmed open gap**, not an unverified one:
  `SET TRANSACTION READ ONLY` fails with `ORA-01466` on any session that has
  run DDL, so `get` currently reads with no snapshot protection.
- No `upgrade`/`backfill_norm`, no concurrency or redaction tests, transport
  security undecided (`M14.22`), and no CI gate — the fake MySQL gate was
  removed rather than repointed (**F-06**), and `scripts/db.sh` is the local
  gate.

Open Oracle questions that make this more than mechanical: no boolean type
before 23ai; `VARCHAR2` capped at 4000 bytes with longer values becoming `CLOB`,
which cannot be indexed or compared like text; no `IF NOT EXISTS` anywhere, so
idempotence needs a PL/SQL block swallowing ORA-00955; `SYS_CONTEXT` in place of
a session GUC for the erasure flag; and whether an Oracle Database Free image
runs on arm64 at all, which decides whether live verification can be promised.

## What does not vary

Whichever you choose, these are identical by construction (`X15.1`):

- The relational map, shredding, and reconstruction — including lossless
  round-trip (`R4.2`).
- The fold: `fold("Ærø") == "aero"` on every engine, with no extension to
  install and no dependence on the engine's collation tables or Unicode version
  (`X15.4`).
- Generated identifiers: every port budgets to 63 bytes, the tightest target, so
  a name generated once is legal everywhere and two schemas are comparable
  name-for-name (`X15.3`).
- The `ords` stored image, whatever type holds it (`X15.5`).
- The canonical JSON the hash chain commits to (`X15.2`) — with no exception
  since **F-07** was fixed; `canon.rs` is identical in all six.

## Migrating between engines

There is no migration tool. The route is export from one and load into the
other, and it works because the *logical* content of a store is engine-
independent (`X15.10`): the same resource shredded by two ports produces the same
logical rows under the same identifiers.

What does not carry across is the hash chain. A chain verified under one port
should be verified there, before the export, and the destination starts a new
chain — reported as beginning where it begins, never backfilled (`M3.16e`).
