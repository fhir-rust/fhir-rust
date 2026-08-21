# Databases and dialects

## The six engines

| Port | Engine | Floor | Driver | Level |
| --- | --- | --- | --- | --- |
| `fhir-postgresql` | PostgreSQL 18 | 18 | `tokio-postgres` + `deadpool` | Reference |
| `fhir-sqlite` | SQLite 3 | bundled | `rusqlite` (`bundled`) | Store |
| `fhir-mysql` | MySQL 8.4 | 8.0 | `mysql_async` | Store |
| `fhir-mariadb` | MariaDB 11.4 | 10.6 | `mysql_async` | Store |
| `fhir-mssql` | SQL Server | undeclared | `tiberius` (rustls) | Scaffold |
| `fhir-oracle` | Oracle | **undeclared** | **none chosen** | Scaffold |

`fhir-sqlite` bundles SQLite rather than linking the host's, because the
generated DDL depends on version-specific behaviour and the engine should be
pinned the same way the container images are.

## Regenerating the map assets

The maps in `assets/*.json.gz` are committed (`G2.1`) so a build never needs the
FHIR packages. They are produced by, and only by:

```sh
cargo run -p fhir-<engine>-gen --bin regen-assets            # rewrite them
cargo run -p fhir-<engine>-gen --bin regen-assets -- --check # gate: writes nothing
```

`--check` exits non-zero when the committed maps are not what the generator
produces. Run it after **any** change to `gen/src` — a generator change with
stale assets means the tree and the artifact disagree, which is how `where()`
restrictions stayed dropped in six shipped maps (**F-38**, **F-40**).

The packages are found via `FHIR_<ENGINE>_SPEC_DIR`, else
`fhir/doc/fhir-specifications` in this monorepo. Both the tool and
`gen/tests/assets_current.rs` skip loudly when they are absent, because absent
input is not drift.

**Regenerating is not a file edit.** It changes `map_checksum`, which is what a
store records at `init` and compares at `upgrade`. Measure before you commit:
if the column set is unchanged, an installed database stays structurally valid
and only the checksum moves; if columns appear, it is a schema migration
(`L12`, `O10.4a`) and four ports have no `upgrade` to carry one (**F-15**).

## Running one locally

```sh
cd fhir-<engine>
scripts/db.sh up        # start the pinned image, wait until it answers
scripts/db.sh dsn       # print the export line for a manual run
scripts/db.sh client    # interactive client inside the container
scripts/db.sh test      # up, then the live suite
scripts/db.sh corpus    # lay out the FHIR spec/example corpora
scripts/db.sh down      # stop and remove; data is not persisted
```

Podman if installed, Docker otherwise. Nothing writes outside the container or
`target/`.

`fhir-mssql/scripts/db.sh` now starts SQL Server 2022 (set
`FHIR_MSSQL_IMAGE=mcr.microsoft.com/azure-sql-edge` on Apple silicon).
`fhir-oracle/scripts/db.sh` **refuses**, with an explanation and exit code 1,
because that port has no Oracle DDL, no driver, and no store — starting a
substitute is what it did until 2026-07-31, and it made every local run look
like evidence (**F-06**, `O10.12`).

## Where dialect lives

Exactly two places (`X15.1`):

**`map/src/ddl.rs`** — the SQL the generator emits. The `col_sql` function is
the whole type binding:

```rust
pub fn col_sql(ty: ColTy) -> &'static str {
    match ty {
        ColTy::Bool        => "BIT",              // SQL Server
        ColTy::Numeric     => "NVARCHAR(MAX)",    // not DECIMAL — M3.6a
        ColTy::TextC       => "NVARCHAR(450) COLLATE Latin1_General_100_BIN2",
        ColTy::Timestamptz => "DATETIME2(6)",     // not DATETIME — rounds to 1/300 s
        ColTy::Jsonb       => "NVARCHAR(MAX)",    // not JSON — M3.6c
        …
    }
}
```

**`store/`** — driver, transaction control, placeholder syntax, search SQL.

Everything above these speaks `ColTy` and Rust types. If a change seems to need
a third dialect-aware place, that is a missing abstraction — raise it rather
than adding an exception.

## The three bindings that are always wrong the obvious way

Each has bitten a port already, and each is a numbered requirement now.

**`Numeric` is not a decimal type** (`M3.6a`). `M3.6` requires a decimal's
original textual precision to survive round-trip. `DECIMAL(65,30)` returns
`1.50` as `1.500000000000000000000000000000`; `REAL` cannot hold `1.50`
distinctly from `1.5` at all. Bind to text, and serve range search from a
derived sort column.

**`TextC` must be binary and NO PAD** (`M3.6b`). It backs `:exact` matching and
key identity. SQL Server's default collation is case- *and* accent-insensitive,
so a column left at the default silently acquires fuzzy equality — the opposite
of what is wanted. And under a PAD SPACE collation `'Smith' = 'Smith '` is true,
which widens `:exact` and weakens primary keys. Hence `utf8mb4_0900_bin` on
MySQL, `utf8mb4_nopad_bin` on MariaDB, `Latin1_General_100_BIN2` on SQL Server,
`COLLATE BINARY` on SQLite.

**`Jsonb` must not be a JSON type** (`M3.6c`). The history hash chain commits to
bytes canonicalized in Rust (`X15.2`). A JSON column re-normalizes what it is
given, so the bytes read back would differ from the bytes signed and **every
chain would fail verification**. Bind to text.

PostgreSQL still binds `jsonb` (`M14.13`), but since **F-07** the pre-image is
canonicalized in Rust from the stored bytes, so no digest depends on how the
column renders. What remains is narrower: `jsonb` can alter a value on the way
*in* (`1e2` is stored as `100`), so that port signs the value as stored.

## `ords`, which looks hard and is not

`ords` is a path of ordinals — `{1}`, `{}`, `{-1,3}` — identifying an element's
position through every repeating ancestor. Only PostgreSQL has a native array
type; the rest store the literal as text (`M3.4b`), and that is sufficient
because **the database never orders, compares, subscripts, or unnests it**.
There is no `ORDER BY ords`, no `@>`, no `unnest`; child tables correlate on
`rid` alone; reconstruction is order-insensitive because rows land in hash maps
keyed on `ords` before anything is rebuilt. The database enforces uniqueness as
part of a primary key and hands the value back.

Three properties of the value domain must survive any binding (`M3.4a`), and
together they rule out every clever encoding:

- **Negative ordinals occur.** Two cyclic `contentReference` referrers sharing a
  table negate the second's ordinals. Domain is `-32767..=-1 ∪ 1..=32767`; `0`
  never appears. Unsigned or magnitude-only encodings are wrong.
- **The empty path is valid and common.** Resource-level extensions and element
  ids shred with `ords = {}` into a `NOT NULL` primary-key column, and the empty
  key is reconstruction's base-row sentinel.
- **Depth is unbounded.** 23 R5 resource types own recursive tables
  (`Questionnaire.item.item`, `ImplementationGuide`, `StructureMap`, …) whose
  depth is data-dependent. A fixed-width encoding covers ~99.9% of tables and
  still fails.

One user-facing thing regresses on a text binding, and the book must say so
(`M14.9` in the SQLite annex): `ords = '{1}'` still works, but PostgreSQL's
`ords[1] = 1` subscript does not. `ords LIKE '{1,%'` is the nearest equivalent
and is a prefix match on the text image, not a typed subscript.

## Porting to a seventh engine

1. **Copy a port whose shape matches** — `fhir-mysql` for a client/server SQL
   engine, `fhir-sqlite` for an embedded one.
2. **Rename crates and directories** (`W16.2`), and fix every `description`
   (`W16.3`) — not fixing them is **F-02**, present in all six today.
3. **Write `ddl.rs`.** This is the real work. Start at `col_sql` and the three
   bindings above.
4. **Write the annex** against the twelve-item `X15.6` checklist, before the
   store. This is the step `fhir-mssql` and `fhir-oracle` skipped; the result
   was two annexes specifying MySQL, since rewritten (**F-16**). Read
   [`fhir-oracle`'s](../fhir-oracle/spec/14-oracle-dialect.md) for what an annex
   looks like when nothing has been decided yet — a decision list, not a blank.
5. **Point CI and `scripts/db.sh` at the real engine** (`O10.12`).
6. **Then the store**, following the parallel-module pattern.
7. **Do not touch the shared core** (`X15.1`). If you think you must, you have
   found either a missing `ColTy` or a genuine core amendment; both are
   conversations before they are commits.
8. **Rewrite the documentation** rather than substituting the engine name
   through it (`W16.8`). That substitution is **F-01**, and it is the most
   serious finding in the register.

Steps 4 and 8 are the ones that get skipped, and they are the two that produce
documents making confident false claims about clinical software.
