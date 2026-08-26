# The storage model

The full normative rules are in [`spec/03-storage-model.md`](../../../spec/databases/03-storage-model.md)
plus this port's departures in [`spec/14-mssql-dialect.md`](../../spec/14-mssql-dialect.md);
this chapter is the tour, and its column types are checked against
`crates/fhir-mssql-map/src/ddl.rs`.

## Base tables and child tables

Every resource type has a base table named after it (`r5.patient`) with
`[id] NVARCHAR(64) COLLATE Latin1_General_100_BIN2 NOT NULL PRIMARY KEY`,
`[version_id] BIGINT`, `[last_updated] DATETIME2(6)`, and a typed column for
every scalar element. 64 is not a guess: the FHIR® `id` production is
`[A-Za-z0-9\-\.]{1,64}` (`R4.6`, `M14.12`), and `BIN2` rather than the
server's default collation because the default is case- and
accent-insensitive — fine for a folded search column, wrong for a primary
key. Every **repeating** element gets a child table; non-repeating complex
elements flatten into their parent as prefixed columns
(`maritalStatus.text` → `marital_status_text`).

Child tables carry:

- `[rid]` — the owning resource id, same type as `[id]` (FK, cascade on
  delete),
- `[ords] VARBINARY(255) NOT NULL` — the 1-based index path through
  repeating ancestors,
- typed columns for everything reachable without crossing another repeating
  element.

`ords` is the key idea, and it is this port's one genuinely unusual binding
(`M14.13`): every other port stores the same text image — `{1,2}`, `{}`,
`{-1,3}` — in a native array or character type; SQL Server stores it as the
**raw ASCII bytes of that same text**, because `VARBINARY` keeps one byte
per character against the 900-byte index key budget (`M14.15`) and compares
exactly. `fmt_ords`/`parse_ords` are unmodified, shared code (`X15.1`), so a
database compares value-for-value against any other port's — only the
column type differs. `patient_name_given` row `{2,1}` is the first given
name of the second name. Because the path is an array, recursive elements
(`Questionnaire.item.item…`) share one table at any depth — recursion is
just longer paths. When a resource has *two* recursive routes into the same
table (QuestionnaireResponse's `item.item` and `item.answer.item`), the
second pushes negated ordinals so paths can never collide.

## Types

`col_sql` binds `ColTy` (`M3.6`, `M14.6`) as:

| `ColTy` | SQL Server | Note |
| --- | --- | --- |
| `Bool` | `BIT` | |
| `Int` | `INT` | |
| `BigInt` | `BIGINT` | R5's `integer64` |
| `Numeric` | `NVARCHAR(MAX)` | **not** `DECIMAL` — see below |
| `Text` | `NVARCHAR(MAX)` | FHIR temporals, long strings |
| `TextC` | `NVARCHAR(450) COLLATE Latin1_General_100_BIN2` | folded/exact-comparable strings and ids |
| `Date` | `DATE` | derived sort column |
| `Timestamptz` | `DATETIME2(6)` | derived sort column, **not** `DATETIME` |
| `Jsonb` | `NVARCHAR(MAX)` | history snapshots, contained resources — never SQL Server's `JSON` type |

Two of these are the ones worth understanding, not just memorizing:

- **`Numeric` is `NVARCHAR(MAX)`, not `DECIMAL`** (`M3.6a`, `M14.8`).
  `DECIMAL(38,10)` returns `1.50` as `1.5000000000` — a fixed declared scale
  cannot preserve the lexical precision a client actually sent, and `M3.6`
  requires that survive round-trip. Range search runs against a derived
  numeric comparison (`CAST(… AS FLOAT)` in the search builder, never
  `DECIMAL`), not against this column directly.
- **FHIR temporals are `NVARCHAR(MAX)` too**, stored verbatim as text —
  `"2026-07"` is a legal FHIR date no native `DATE`/`DATETIME2` can hold —
  with a derived `*_sort` column (`Date`/`Timestamptz`) for ordering and
  range search. `Timestamptz` is `DATETIME2(6)`, never `DATETIME`, which
  rounds to 1/300th of a second and would silently alter a value the hash
  chain commits to; it is never `DATETIMEOFFSET` either, because every value
  is normalized to UTC in Rust before binding.

References split into `…_ref_type` / `…_ref_id` (joinable, both `TextC`)
with `…_ref_url` (`Text`) for absolute, urn, and fragment forms. Choice
elements (`value[x]`) become one column set per allowed type; the open
~54-type choices are force-split into their own tables to respect the
column limit of every supported engine. The threshold is **150 columns**
(`SPLIT_WIDTH`), set once in the shared generator and identical in all six
ports — not derived from SQL Server's own (much higher) limit, which is
what lets one generated map serve all six.

### The indexing gap this creates

A column bound to `NVARCHAR(MAX)` cannot be part of an index key at all on
this engine (`M14.15`, `M14.16`). `TextC` is fine — it is `NVARCHAR(450)`,
under the 900-byte key limit — so the folded companion column every
non-`:exact` string search actually compares indexes normally. What does
not index is a token's `system`/`code`, which are `Text`: those searches
are correct and scan. The generated map already carries the intended fix, a
**bounded adjunct** (`<col>_idx`, `NVARCHAR(450)`) and a **checksum
adjunct** (`<col>_h`, `BINARY(32)`) for every `string`-search-parameter
target (`M14.32`/`M14.33`, `U1`–`U10`) — installed and confirmed on a live
SQL Server (3,713 columns, `tests/adjuncts_in_ddl.rs`) — but no store,
including this one, wires them into its `TargetKind` yet, so the schema
carries them unused. This port MUST NOT claim `P6.4a` until it does
(`M14.16`).

## Extensions without a JSON column

Extensions, primitive extensions (`_birthDate`), and element ids live in
one `<resource>_ext` table as **typed leaf rows**: `[rid]`, `[path]`,
`[ords]`, extension array index, `[url]`, and a dotted `[leaf]` path inside
the extension content whose numeric segments are array indexes
(`valueCodeableConcept.coding.0.code`), plus `[v_kind]`/`[v_text]`/`[v_num]`/
`[v_bool]` typed value columns. Arbitrarily nested extensions and every
value type flatten into the same encoding — queryable, indexable, lossless.
`Ext` and `Deep` tables key on a `BINARY(32)` hash surrogate
(`surrogate_key` in `mssql.rs`) rather than their natural key, because that
key includes `NVARCHAR(MAX)` columns that cannot be a primary key at all on
this engine — the same reason MySQL needed one, and it matters more here
since 900 bytes is tighter than MySQL's 3072 (`M14.15`).

The one true datatype cycle in FHIR (`Reference.identifier.assigner` →
`Reference` → …) is cut at the re-entry point and anything below it spills
into a `<resource>_deep` leaf table with the same encoding.

## The sanctioned JSON text

`<resource>_history` stores full-resource snapshots as `NVARCHAR(MAX)`
(write-once audit data serving `vread`/`history`), and `contained`
resources plus inline resources (`Bundle.entry.resource`) are stored
whole — they are anonymous resources of unknowable type, so normalizing
them buys nothing. None of it uses SQL Server's `JSON` functions or type:
the hash chain's pre-image is canonicalized in Rust (`canon.rs`, `X15.2`),
and a column that re-normalized what it was given would make the bytes
read back differ from the bytes the chain signed (`M14.11`).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
