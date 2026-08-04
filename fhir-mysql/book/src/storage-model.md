# The storage model

The full normative rules are in
[`spec/03-storage-model.md`](../../../spec/databases/03-storage-model.md); the
MySQL-specific departures are numbered `M14.x` in the
[dialect annex](../../spec/14-mysql-dialect.md). This chapter is the tour, for
this engine specifically — not the PostgreSQL original it was copied from
(audit **F-56**).

## Base tables and child tables

Every resource type has a base table named after it (`patient`, in the `r5`
database) with `` `id` VARCHAR(64) COLLATE utf8mb4_0900_bin NOT NULL PRIMARY
KEY ``, `version_id BIGINT`, `last_updated DATETIME(6)`, and a typed column for
every scalar element. `utf8mb4_0900_bin` on `id` is deliberate, not
decorative: it is MySQL 8's NO PAD binary collation, so `'a' <> 'a '`, unlike
the tempting `utf8mb4_bin`, which pads and would let two distinct ids compare
equal (`M14.0d`). Every **repeating** element gets a child table;
non-repeating complex elements flatten into their parent as prefixed columns
(`maritalStatus.text` → `marital_status_text`).

The **kind** of child table decides its primary key, and the two kinds are not
the same shape:

- **`Elem` tables** (one array level, no unbounded text in the key) keep a
  natural key: `` `rid` VARCHAR(64) NOT NULL, `ords` VARBINARY(255) NOT NULL,
  PRIMARY KEY (`rid`, `ords`) ``, plus a `FOREIGN KEY (rid) … ON DELETE
  CASCADE`.
- **`Ext` and `Deep` tables** — extensions and the datatype-cycle spillover,
  below — cannot: their natural key is `(rid, path, ords, ..., leaf)`, and
  `path`/`leaf` are unbounded `TEXT`. InnoDB caps an index key at 3072 bytes
  and cannot key an unbounded column at all without a prefix, and a prefix
  index does not enforce uniqueness over the full value — two rows differing
  only past the prefix would collide and silently lose data. So these tables
  carry a surrogate primary key instead: `` `key_hash` BINARY(32) NOT NULL
  PRIMARY KEY ``, a SHA-256 computed in Rust over the canonically joined
  natural-key components (`M14.12`). The natural columns are still present as
  ordinary columns, with a non-unique `KEY (rid)` to serve the read path, which
  only ever filters `WHERE rid = ?`.

`ords` is `VARBINARY(255)`, not an array type — MySQL has none. It holds the
same array-literal image every port writes: `{1,2}`, `{}`, `{-1,3}`
(`fmt_ords`/`parse_ords`, unchanged from the shared core, `M14.9`). Row `{2,1}`
in `patient_name_given` is the first given name of the second name. Because
the path is a string, recursive elements (`Questionnaire.item.item…`) share
one table at any depth — recursion is just a longer image. When a resource has
*two* recursive routes into the same table (`QuestionnaireResponse`'s
`item.item` and `item.answer.item`), the second pushes negated ordinals so
paths can never collide. `VARBINARY(255)` bounds nesting to roughly 36 levels
at the worst case (`-32767,` per level); going deeper fails loudly at shred
time rather than truncating and colliding two rows (`M14.11`).

**No PostgreSQL array idioms survive.** `ords = '{1}'` still works — it is a
string-equality comparison — but the subscript form (`ords[1] = 1`, used to
match any descendant of the first instance) does not: MySQL has no array
subscripting. The nearest equivalent is a prefix match on the stored image;
see [Querying](querying.md#addressing-a-repeating-element) (`M14.13`).

## Types

`ColTy` maps to MySQL as follows (`M14.14`):

| FHIR primitive | `ColTy` | MySQL column |
| --- | --- | --- |
| `boolean` | `Bool` | `TINYINT(1)` |
| `integer`/`positiveInt`/`unsignedInt` | `Int` | `INT` |
| `integer64` (R5) | `BigInt` | `BIGINT` |
| `decimal` | `Numeric` | `TEXT` |
| `code`/`id`/`uri`/`string`/… | `Text` | `TEXT` |
| exact-match identity columns | `TextC` | `TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_bin` |
| `date` | — | `DATE` (sort column) |
| `dateTime`/`instant` | — | `DATETIME(6)` (sort column) |
| history `resource` | `Jsonb` | `LONGTEXT` |

Two of these are not what a MySQL default would give you, and both are
correctness requirements, not style:

- **`decimal` is `TEXT`, never `DECIMAL` or `DOUBLE`.** `M3.6` requires a
  decimal's original textual precision survive round-trip; `DECIMAL(65,30)`
  returns `1.50` as `1.500000000000000000000000000000` — a fixed declared
  scale cannot preserve a per-value lexical form, and `DOUBLE` is worse.
  Unlike `date`/`dateTime`, a decimal gets **no separate materialized sort
  column** in the generated map (`Prim::Decimal::sort_ty()` returns `None`) —
  the dialect annex proposed one (`M14.15`) but the shipped code instead casts
  inline at query time, `CAST(col AS DECIMAL(65,30))`, for range predicates
  (`mysql_search.rs`). The stored text is read back verbatim either way.
- **`TextC` is an explicitly *binary* collation**, not MySQL's default. MySQL's
  default collation is accent- and case-insensitive, so a plain `TEXT` column
  would silently acquire fuzzy equality — breaking `:exact` matching and
  primary-key identity. `utf8mb4_0900_bin` is chosen over the more obvious
  `utf8mb4_bin` because the latter is **PAD SPACE**: `'Smith' = 'Smith '`
  would evaluate true (`M14.0d`, `M14.7`).

`date`/`dateTime`/`instant` are stored **verbatim as `TEXT`** — `"2026-07"` is
a legal FHIR date no native type can hold — with a derived, indexed
`*_sort DATE`/`DATETIME(6)` column for ordering and range search;
`Timestamptz` is `DATETIME(6)` rather than `TIMESTAMP` because `TIMESTAMP`
converts on the session time zone and its range ends in 2038 (`M14.16`).
References split into `…_ref_type` / `…_ref_id` (joinable) with `…_ref_url`
for absolute, urn, and fragment forms. Choice elements (`value[x]`) become one
column set per allowed type; the open ~54-type choices are force-split into
their own tables to respect the column limit of every supported engine — 150
columns (`SPLIT_WIDTH`), set once in the shared generator and identical in all
six ports. It is not derived from MySQL's own (much higher) limit; it sits
well below every supported engine's, which is the property that matters
(`M14.1`).

Every table also stays within MySQL's 65,535-byte row-size limit: `TEXT` and
`BLOB` columns cost only their in-row pointer, so a 150-column table of `TEXT`
is safe, but a local optimization that swaps in `VARCHAR(n)` would not be
(`M14.18`).

## Extensions, without a JSON column

Extensions, primitive extensions (`_birthDate`), and element ids live in one
`<resource>_ext` table as **typed leaf rows**: attach path + ordinals,
extension array index, url, and a dotted leaf path inside the extension
content whose numeric segments are array indexes
(`valueCodeableConcept.coding.0.code`). Arbitrarily nested extensions and
every value type flatten into the same encoding — queryable, indexable,
lossless, and (per `M14.12` above) keyed by a `BINARY(32)` hash rather than the
natural columns.

The one true datatype cycle in FHIR (`Reference.identifier.assigner` →
`Reference` → …) is cut at the re-entry point, and anything below it spills
into a `<resource>_deep` leaf table with the same encoding and the same
surrogate-key treatment.

## The one JSON column that remains

MySQL's native `JSON` type exists and is deliberately **not used** for the
signed `resource` column (`M14.0g`, `M14.14`). `<resource>_history` stores
full-resource snapshots as `LONGTEXT` — write-once audit data serving
`vread`/`history` — because the hash chain commits to bytes canonicalized in
Rust (`canon::canonicalize`, RFC 8785 deliberately **not** used — see
`M14.19`/`M14.20`); a native `JSON` column re-normalizes on write, so the
bytes read back would not be the bytes the chain signed. `contained`
resources and inline resources (`Bundle.entry.resource`) are likewise stored
whole, also as `LONGTEXT` — they are anonymous resources of unknowable type,
so normalizing them buys nothing.
