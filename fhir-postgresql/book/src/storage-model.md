# The storage model

The full normative rules are in
[`spec/databases/03-storage-model.md`](../../../spec/databases/03-storage-model.md);
this port's departures and bindings are in the
[dialect annex](../../spec/14-postgresql-dialect.md). This chapter is the tour.

## Base tables and child tables

Every resource type has a base table named after it (`r5.patient`) with
`id text PRIMARY KEY`, `version_id`, `last_updated`, and a typed column
for every scalar element. Every **repeating** element gets a child table;
non-repeating complex elements flatten into their parent as prefixed
columns (`maritalStatus.text` → `marital_status_text`).

Child tables carry:

- `rid` — the owning resource id (FK, cascade on delete),
- `ords smallint[]` — the 1-based index path through repeating ancestors,
- typed columns for everything reachable without crossing another
  repeating element.

`ords` is the key idea: `patient_name_given` row `{2,1}` is the first
given name of the second name. Because the path is an array, recursive
elements (`Questionnaire.item.item…`) share one table at any depth —
recursion is just longer paths. When a resource has *two* recursive routes
into the same table (QuestionnaireResponse's `item.item` and
`item.answer.item`), the second pushes negated ordinals so paths can never
collide.

This is what `init` (`store.init(...)`, [Getting started](getting-started.md))
actually emits for `r5.patient` and its `name` child table — generated
directly from `fhir-postgresql-map::ddl::create_table` against the bundled R5
map, not hand-typed:

```sql
CREATE TABLE "r5"."patient" (
  "id" text PRIMARY KEY,
  "version_id" bigint NOT NULL,
  "last_updated" timestamptz NOT NULL,
  "meta_version_id" text,
  "meta_last_updated" text,
  "meta_last_updated_sort" timestamptz,
  "active" boolean,
  "gender" text,
  "birth_date" text,
  "birth_date_sort" date,
  "deceased_boolean" boolean,
  "deceased_date_time" text,
  "deceased_date_time_sort" timestamptz,
  "marital_status_text" text,
  "managing_organization_ref_type" text,
  "managing_organization_ref_id" text,
  "managing_organization_ref_url" text
  -- … one column per scalar and flattened non-repeating element
);

CREATE TABLE "r5"."patient_name" (
  "rid" text NOT NULL REFERENCES "r5"."patient" ("id") ON DELETE CASCADE,
  "ords" smallint[] NOT NULL,
  "use" text,
  "text" text,
  "family" text,
  "period_start" text,
  "period_start_sort" timestamptz,
  "period_end" text,
  "period_end_sort" timestamptz,
  "family_norm" text COLLATE "C",
  "text_norm" text COLLATE "C",
  PRIMARY KEY ("rid", "ords")
);
```

Two things worth noticing that generated SQL, rather than prose, makes
plain: **there are no `CHECK` constraints and no PostgreSQL `enum` types**
anywhere in `ddl.rs` — `gender` is `text`, not `patient_gender_enum`.
Required-binding value sets are not enforced by the schema (see the
[trust boundary](trust-boundary.md)). And `family_norm`/`text_norm` are the
materialized, case- and accent-folded companion columns search actually
compares (`M14.20`) — every non-`:exact` string search reads these, never
`family`/`text` with a runtime `lower()`.

## Types

Booleans, integers, and decimals map to `boolean`, `integer`/`bigint`,
and `numeric` (decimal scale survives round trip). FHIR® temporals are
stored **verbatim as text** — `"2026-07"` is a legal FHIR date no native
type can hold — with a derived `*_sort` column (`date`/`timestamptz`) for
ordering and search. References split into `…_ref_type` / `…_ref_id`
(joinable) with `…_ref_url` for absolute, urn, and fragment forms.
Choice elements (`value[x]`) become one column set per allowed type; a wide
choice (`Extension.value[x]`, `~54` allowed types) is force-split into its
own child table once its columns would push the parent past `SPLIT_WIDTH`
(150 columns, `M14.2`) — a conservative shared budget the six ports agree on,
not PostgreSQL's own 1,600-column ceiling, which this budget stays far below
on purpose so the same generated schema stays legal on the narrower engines
too.

## Extensions without JSONB

Extensions, primitive extensions (`_birthDate`), and element ids live in
one `<resource>_ext` table as **typed leaf rows**: attach path + ordinals,
extension array index, url, and a dotted leaf path inside the extension
content whose numeric segments are array indexes
(`valueCodeableConcept.coding.0.code`). Arbitrarily nested extensions and
every value type flatten into the same encoding — queryable, indexable,
lossless.

The one true datatype cycle in FHIR (`Reference.identifier.assigner` →
Reference → …) is cut at the re-entry point and anything below it spills
into a `<resource>_deep` leaf table with the same encoding.

## The sanctioned JSONB

`<resource>_history` stores full-resource snapshots (write-once audit
data serving vread/history), and `contained` resources plus inline
resources (`Bundle.entry.resource`) are stored whole — they are anonymous
resources of unknowable type, so normalizing them buys nothing.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
