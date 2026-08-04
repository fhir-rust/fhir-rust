# The storage model

The full normative rules are in `spec/03-storage-model.md` (the shared core)
and this port's annex, `spec/14-mariadb-dialect.md`; this chapter is the tour,
and every type name below is `col_sql`'s actual MariaDB output (`M14.14`), not
PostgreSQL's.

## Base tables and child tables

Every resource type has a base table named after it (`r5.patient`, a database
in MariaDB's sense — see [FHIR versions](fhir-versions.md)) with
`` `id` VARCHAR(64) COLLATE utf8mb4_nopad_bin `` as its primary key,
`version_id`, `last_updated`, and a typed column for every scalar element.
`id` is bounded rather than `TEXT` because the FHIR `id` type is constrained
to `[A-Za-z0-9\-\.]{1,64}`, so 64 characters is exact and spec-faithful
rather than an arbitrary cap, and a bounded, indexable primary key is what
every foreign key below points at. Every **repeating** element gets a child
table; non-repeating complex elements flatten into their parent as prefixed
columns (`maritalStatus.text` → `marital_status_text`).

Child tables carry:

- `rid` — the owning resource id (FK, cascade on delete),
- `` `ords` VARBINARY(255) `` — the 1-based index path through repeating
  ancestors, stored as the same array-literal image every port uses
  (`{1,2}`, `{}`, `{-1,3}`), just typed as a fixed-width binary column rather
  than PostgreSQL's `smallint[]` — MariaDB has no array type. The database
  never orders, subscripts, or unnests this column; it only enforces
  uniqueness and returns it intact (`M14.9`), which is what makes the
  bounded encoding sufficient. `VARBINARY(255)` admits roughly 36 levels of
  nesting at the worst case; a deeper path fails loudly at shred time rather
  than truncating (`M14.11`).
- typed columns for everything reachable without crossing another
  repeating element.

`ords` is the key idea: `patient_name_given` row `{2,1}` is the first given
name of the second name. Because the path is an array image, recursive
elements (`Questionnaire.item.item…`) share one table at any depth —
recursion is just longer paths. When a resource has *two* recursive routes
into the same table (QuestionnaireResponse's `item.item` and
`item.answer.item`), the second pushes negated ordinals so paths can never
collide.

**`Ext` and `Deep` tables key on a surrogate, not the natural columns.**
Their natural primary key includes `path` and `leaf`, both unbounded text;
InnoDB caps an index key at 3072 bytes and cannot index unbounded `TEXT` at
all without a lossy prefix. This port computes a `BINARY(32)` SHA-256 over
the joined natural-key components in Rust and declares that the primary key,
keeping the natural columns as ordinary, non-unique-indexed data (`M14.12`).
`Elem` tables do not need this — their key is `(rid, ords)`, which fits
`VARCHAR(64)` + `VARBINARY(255)` comfortably.

## Types

| FHIR / map type | MariaDB column |
| --- | --- |
| `boolean` | `TINYINT(1)` |
| `integer` | `INT`; `integer64` (R5) is `BIGINT` |
| `decimal` | `TEXT` (verbatim lexical form) **+** a derived `DOUBLE` `*_sort` column |
| exact-match text (ids, codes) | `TEXT COLLATE utf8mb4_nopad_bin` |
| ordinary text | `TEXT` (`utf8mb4`) |
| date / dateTime / instant | `TEXT` (verbatim) **+** a derived `DATE` or `DATETIME(6)` `*_sort` column |

Booleans and integers are native types, but **decimals are not `DECIMAL`**.
`DECIMAL(65,30)` returns `1.50` as `1.500000000000000000000000000000` — a
fixed declared scale cannot preserve a per-value lexical form, and
round-trip fidelity (`M3.6`, `R4.2`) is the invariant this project exists to
keep. So `decimal` follows the same pattern FHIR temporals already use: the
verbatim text survives in one column, and a derived, indexed sort column
(`DOUBLE` here, not PostgreSQL's native `numeric`) serves range search and
ordering (`M14.15`). Retrieval always reads the verbatim column.

Exact-match text — resource ids, `TextC` columns — is bound to
`utf8mb4_nopad_bin`, MariaDB's own **NO PAD** binary collation, not the more
obvious `utf8mb4_bin`. `utf8mb4_bin` is **PAD SPACE**, under which
`'Smith' = 'Smith '` evaluates true, which would silently widen `:exact`
string matching and weaken primary-key identity (`M14.6`, `M14.7`). MySQL
spells the same NO-PAD property `utf8mb4_0900_bin`; MariaDB accepts that name
too, but it is not native here.

References split into `…_ref_type` / `…_ref_id` (joinable) with `…_ref_url`
for absolute, urn, and fragment forms. Choice elements (`value[x]`) become one
column set per allowed type; the open ~54-type choices are force-split into
their own tables to respect the column limit of every supported engine. The
threshold is **150 columns**, set once in the shared generator (`SPLIT_WIDTH`)
and identical in all six ports — which is why one generated map serves them
all. It is not derived from this engine's limit specifically; MariaDB's
per-table column ceiling is 1017, far above the threshold, and every
generated table stays within MariaDB's separate 65,535-byte row-size limit
too, because `TEXT`/`BLOB` columns cost only their in-row pointer (`M14.18`).

## Extensions without a JSON column

Extensions, primitive extensions (`_birthDate`), and element ids live in one
`<resource>_ext` table as **typed leaf rows**: attach path + ordinals,
extension array index, url, and a dotted leaf path inside the extension
content whose numeric segments are array indexes
(`valueCodeableConcept.coding.0.code`). Arbitrarily nested extensions and
every value type flatten into the same encoding — queryable, indexable,
lossless. `Ext` carries the same surrogate-key treatment as `Deep`, above.

The one true datatype cycle in FHIR (`Reference.identifier.assigner` →
Reference → …) is cut at the re-entry point and anything below it spills into
a `<resource>_deep` leaf table with the same encoding.

## The one JSON-shaped column, and it is not MariaDB's `JSON` type

`<resource>_history` stores full-resource snapshots (write-once audit data
serving vread/history), and `contained` resources plus inline resources
(`Bundle.entry.resource`) are stored whole — they are anonymous resources of
unknowable type, so normalizing them buys nothing.

That column is declared `LONGTEXT`, deliberately **not** MariaDB's native
`JSON` type. The history hash chain commits to a canonical rendering of the
resource, computed once in Rust (`canon::canonicalize`) and used identically
by the writer and the verifier (`M14.19`/`M14.20`). MariaDB's `JSON` type
also normalizes and reorders keys on the way in, but by its own rules, not
this port's — so a `JSON` column would silently rewrite the bytes a chain had
already signed, and every chain would fail verification the first time it was
checked. `LONGTEXT` stores exactly the bytes Rust computed, unmodified.
(`LONGTEXT` rather than `TEXT` for the same reason the map asset needs it in
`fhir_mariadb_meta`: `TEXT` caps at 65,535 bytes and a real resource's
canonical form can exceed that.)
