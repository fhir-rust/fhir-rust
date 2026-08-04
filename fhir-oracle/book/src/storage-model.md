# The storage model

The full normative rules are in
[`spec/03-storage-model.md`](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/03-storage-model.md);
this chapter is the tour, and this port's dialect departures are numbered
`M14.x` in the [Oracle annex](../../spec/14-oracle-dialect.md).

## Base tables and child tables

Every resource type has a base table named after it (`"R5"."patient"`) with
`"id" VARCHAR2(64 CHAR) PRIMARY KEY`, `"version_id"`, `"last_updated"`, and a
typed column for every scalar element. Every **repeating** element gets a
child table; non-repeating complex elements flatten into their parent as
prefixed columns (`maritalStatus.text` → `marital_status_text`).

Child tables carry:

- `"rid"` — the owning resource id (`VARCHAR2(64 CHAR)`, FK, cascade on
  delete),
- `"ords" RAW(255)` — the shared text-image ordinal path through repeating
  ancestors, stored as raw bytes rather than an array (Oracle has no native
  array column type) — `M14.13`,
- typed columns for everything reachable without crossing another repeating
  element.

`ords` is the key idea: `patient_name_given`'s row for path `{2,1}` is the
first given name of the second name. Because the path is a single encoded
value rather than a fixed set of columns, recursive elements
(`Questionnaire.item.item…`) share one table at any depth — recursion is
just a longer path. When a resource has *two* recursive routes into the same
table (QuestionnaireResponse's `item.item` and `item.answer.item`), the
second pushes negated ordinals so paths can never collide. `M14.14` requires
this encoding to preserve negative ordinals verbatim and to store the empty
path (`{}`) as a value distinct from `NULL` — worth stating explicitly on
this engine, since Oracle traditionally treats the empty *string* as `NULL`,
and `ords` is a `NOT NULL` primary-key column in which the empty path is
frequent.

## Types

| FHIR type | Oracle column | Note |
| --- | --- | --- |
| `boolean` | `NUMBER(1)` + `CHECK (col IN (0,1))` | No native boolean before 23ai; this port targets the 12.2 floor (`M14.4`, `M14.8`) |
| `integer` | `NUMBER(10)` | |
| `integer64` (R5) | `NUMBER(19)` | |
| `decimal` | `VARCHAR2(64 CHAR)` | **Not** `NUMBER` — Oracle's `NUMBER` normalizes `1.50` to `1.5`, which would violate the requirement that decimal scale survive round-trip (`M3.6a`, `M14.7`) |
| `date` / `dateTime` / `instant` | `DATE` / `TIMESTAMP(6)` for the derived sort column; the lexical value itself is stored as text | FHIR dates have no length bound (`"2026-07"` is legal and no native type holds it), so the canonical value is text with a derived `*_sort` column for ordering and search |
| `string` (short, bounded) | `VARCHAR2(n CHAR)` | |
| `string` (unbounded) | `CLOB`, plus a bounded (`VARCHAR2(450 CHAR)`) and a digest (`RAW(32)`, SHA-256) adjunct column | A `CLOB` on this engine cannot be indexed or `=`-compared at all — see below |

References split into `…_ref_type` / `…_ref_id` (joinable) with
`…_ref_url` for absolute, urn, and fragment forms. Choice elements
(`value[x]`) become one column set per allowed type; the open ~54-type
choices are force-split into their own tables to respect the column limit of
every supported engine. The threshold is **150 columns**, set once in the
shared generator (`SPLIT_WIDTH`) and identical in all six ports — which is
why one generated map serves them all. It is not derived from Oracle's own
limit specifically; it sits well below every supported engine's, which is
the property that matters.

## Why unbounded text needs two extra columns here

`VARCHAR2` maxes at **4000 bytes** on the `STANDARD` setting this port
targets — and because the default character set is `AL32UTF8` (up to 4 bytes
per character), a `VARCHAR2(4000 CHAR)` declaration can silently hold as few
as **1000** arbitrary Unicode characters, not 4000. A FHIR `string` has no
length bound, so anything past that ceiling is a `CLOB` — and a `CLOB`
cannot be indexed, compared with `=`, or joined the way character data can
(`ORA-22848`, `ORA-02327`, both confirmed live).

Every port that hits this wall solves it the same way (`U1`–`U10`, shared
core): a `CLOB`-typed column gets a **bounded companion**
(`<col>_idx VARCHAR2(450 CHAR)`, first 450 characters, for prefix/range
search) and a **digest companion** (`<col>_h RAW(32)`, a SHA-256 of the
folded value, for equality search). Oracle needs both more than most of its
siblings do: SQL Server's analogous `NVARCHAR(MAX)` at least answers `=`, so
only the index is missing there. An Oracle `CLOB` answers no comparison at
all, so without the digest column an equality search over unbounded text
would simply fail rather than scan slowly.

## Extensions without a JSON blob

Extensions, primitive extensions (`_birthDate`), and element ids live in one
`<resource>_ext` table as **typed leaf rows**: attach path + ordinals,
extension array index, url, and a dotted leaf path inside the extension
content whose numeric segments are array indexes
(`valueCodeableConcept.coding.0.code`). Arbitrarily nested extensions and
every value type flatten into the same encoding — queryable, indexable
(via the same bounded/digest adjunct pair where the leaf value is
unbounded text), lossless.

The one true datatype cycle in FHIR (`Reference.identifier.assigner` →
Reference → …) is cut at the re-entry point and anything below it spills
into a `<resource>_deep` leaf table with the same encoding.

## The sanctioned `CLOB`

`<resource>_history` stores full-resource snapshots as `CLOB` (write-once
audit data serving `vread`/`history` — never re-parsed for search, so the
comparison limitation does not apply), and `contained` resources plus inline
resources (`Bundle.entry.resource`) are stored whole — they are anonymous
resources of unknowable type, so normalizing them buys nothing. This engine
never uses Oracle's native `JSON` type for any of it: `JSON` re-normalizes on
write, which would make the stored bytes differ from the bytes the audit
hash chain signed (`M3.6c`, `M14.12`).
