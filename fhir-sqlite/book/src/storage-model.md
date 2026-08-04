# The storage model

The normative rules are the shared core, [`spec/03-storage-model.md`](../../../spec/databases/03-storage-model.md),
plus this port's departures in [`spec/14-sqlite-dialect.md`](../../spec/14-sqlite-dialect.md).
This chapter is the tour, and — unlike the PostgreSQL original this book was
copied from — every type below is what SQLite actually stores (`M14.10`), not
what the core spec describes in the abstract.

## Base tables and child tables

Every resource type has a base table named after it (`patient`, inside the
attached `r5` database) with `id TEXT PRIMARY KEY`, `version_id`,
`last_updated`, and a typed column for every scalar element. Every
**repeating** element gets a child table; non-repeating complex elements
flatten into their parent as prefixed columns (`maritalStatus.text` →
`marital_status_text`).

Child tables carry:

- `rid` — the owning resource id (`FOREIGN KEY … ON DELETE CASCADE`, which is
  why `PRAGMA foreign_keys = ON` in `open` is load-bearing rather than
  hygiene — see [Getting started](getting-started.md)),
- `ords` — the 1-based index path through repeating ancestors,
- typed columns for everything reachable without crossing another repeating
  element.

`ords` is the key idea: `patient_name_given` row `{2,1}` is the first given
name of the second name. Because the path is a value rather than a fixed set
of ordinal columns, recursive elements (`Questionnaire.item.item…`) share one
table at any depth — recursion is just longer paths. When a resource has
*two* recursive routes into the same table (`QuestionnaireResponse`'s
`item.item` and `item.answer.item`), the second pushes negated ordinals so
paths can never collide.

**`ords` is `TEXT` here, not an array type.** PostgreSQL stores it as
`smallint[]`; SQLite has no array type, so the port keeps the same literal
image — `{1,2}`, `{}`, `{-1,3}` — in a `TEXT` column (`M14.6`, `M14.7`). This
is not a shortcut: the database never orders, subscripts, or unnests `ords`
anywhere in the tree — child tables correlate on `rid` alone, and
reconstruction is order-insensitive by construction, keying rows into hash
maps before a resource is rebuilt. The only thing SQL does with `ords` is
enforce uniqueness as part of a primary key and hand it back intact. One
consequence is user-visible and is covered in [Querying with
SQL](querying.md): PostgreSQL's `ords[1] = 1` subscript idiom has no
equivalent here, because there is no subscript operator on `TEXT`.

## Types

SQLite is dynamically typed — every `ColTy` below maps to a storage
*affinity*, not a constraint the engine enforces (`M14.10`). The store, not
the database, is what rejects a value of the wrong shape.

| FHIR-level type | `ColTy` | SQLite column | Note |
| --- | --- | --- | --- |
| `boolean` | `Bool` | `INTEGER` (0/1) | not `BOOLEAN` — SQLite has no boolean affinity |
| `integer`, `positiveInt`, … | `Int` | `INTEGER` | |
| `integer64` (R5) | `BigInt` | `INTEGER` | SQLite's `INTEGER` is 64-bit; no separate `bigint` needed |
| `decimal` | `Numeric` | `TEXT` **+** a derived `<col>_sort REAL` column | see below |
| `string`, `code`, `id`, … | `Text` | `TEXT` | |
| the folded `<col>_norm` companion column (search only) | `TextC` | `TEXT COLLATE BINARY` | binary, NO PAD comparison (`M3.6b`); ordering by raw codepoint is what makes the prefix-range search trick in [Search](search.md) sound |
| `date` | `Date` | `TEXT` (verbatim) **+** `<col>_sort TEXT` | fixed-width ISO-8601, not a native date type |
| `dateTime`, `instant` | `Timestamptz` | `TEXT` (verbatim) **+** `<col>_sort TEXT` | fixed-width ISO-8601 UTC |
| history snapshots, `contained` | `Jsonb` | `TEXT` | canonicalized in Rust, never by a database JSON function — see below |

**A decimal cannot be `REAL`.** `M3.6` requires a decimal's original textual
precision to survive round-trip — `"1.50"` must come back as `"1.50"`, not
`"1.5"` — and binary floating point cannot represent that distinction, let
alone hold a 30-significant-digit clinical quantity exactly. So `Numeric`
columns store the verbatim lexical form as `TEXT`, and a parallel, indexed
`REAL` column (e.g. `value_quantity_value_sort`) exists purely for range
search (`gt`, `lt`, `ge`, `le`) and ordering — reads always come from the
`TEXT` column (`M14.11`). This is the same pattern the *original* PostgreSQL
storage model already used for dates; this port just extends it to decimals
too, rather than introducing a second convention. The open cost: the `REAL`
sort column is subject to binary rounding past the 17th significant digit —
immaterial for clinical quantities, but a real inexactness the PostgreSQL
original does not have.

**Dates are the same idea, one column simpler.** The verbatim text (`"1974-12"`,
a legal FHIR partial date no native type could hold) lives in the plain
column; a normalized, fixed-width `TEXT` sort column
(`YYYY-MM-DD` or `YYYY-MM-DDTHH:MM:SS.ffffffZ`, always UTC) makes
lexicographic comparison equal chronological comparison, so range search can
still use an index (`M14.12`).

References split into `…_ref_type` / `…_ref_id` (joinable) with
`…_ref_url` for absolute, urn, and fragment forms. Choice elements
(`value[x]`) become one column set per allowed type; the open ~54-type
choices are force-split into their own tables to respect the column limit.
That threshold is **150 columns** (`SPLIT_WIDTH`), set once in the shared
generator and identical across all six ports — it is not derived from
SQLite's own column limit (`SQLITE_MAX_COLUMN`, 2,000 by default, per
`M14.1`), which is one reason a single generated map can serve every engine.

## Extensions without a document column

Extensions, primitive extensions (`_birthDate`), and element ids live in one
`<resource>_ext` table as **typed leaf rows**: attach path + ordinals,
extension array index, url, and a dotted leaf path inside the extension
content whose numeric segments are array indexes
(`valueCodeableConcept.coding.0.code`). Arbitrarily nested extensions and
every value type flatten into the same encoding — queryable, indexable,
lossless, and stored in the same `TEXT`/`INTEGER`/`REAL` columns as
everything else, not JSON.

The one true datatype cycle in FHIR (`Reference.identifier.assigner` →
`Reference` → …) is cut at the re-entry point, and anything below it spills
into a `<resource>_deep` leaf table with the same encoding.

## History and contained resources: the one place text holds a whole document

`<resource>_history` stores full-resource snapshots (write-once audit data
serving `vread`/`history`), and `contained` resources plus inline resources
(`Bundle.entry.resource`) are stored whole — they are anonymous resources of
unknowable type, so normalizing them buys nothing.

The PostgreSQL original stores these as `jsonb` and computes the hash chain's
pre-image *in SQL*, over PostgreSQL's own canonical `jsonb::text` rendering.
SQLite has no comparable JSON type — its `json()` function preserves
insertion order and minifies differently — so that approach does not port.
Instead, canonicalization happens **in Rust**, once, via `canon::canonicalize`
in `fhir-sqlite-map`, and the `<resource>_history.resource` column is
declared plain `TEXT` holding those exact bytes (`M14.14`, `M14.15`):

- object keys sorted by UTF-8 byte order,
- numbers emitted as their parsed lexeme verbatim (via `serde_json`'s
  `arbitrary_precision` feature — `1.50` and `0.1` survive unchanged; RFC 8785
  is deliberately **not** used, because it serializes numbers as IEEE-754
  doubles and would destroy exactly the precision `M3.6` requires),
- minimal, deterministic string escaping, no insignificant whitespace.

The same function signs and verifies, so a chain computed this way is
**portable**: the same resource yields the same pre-image whichever of the
six engines stores it, which is not true of the PostgreSQL original's
SQL-side canonicalization. It is also a breaking change to any chain computed
before this port existed.
