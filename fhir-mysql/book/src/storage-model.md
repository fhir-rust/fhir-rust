# The storage model

The full normative rules are in `spec/03-storage-model.md`; this chapter is the
tour.

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

## Types

Booleans, integers, and decimals map to `boolean`, `integer`/`bigint`,
and `numeric` (decimal scale survives round trip). FHIR temporals are
stored **verbatim as text** — `"2026-07"` is a legal FHIR date no native
type can hold — with a derived `*_sort` column (`date`/`timestamptz`) for
ordering and search. References split into `…_ref_type` / `…_ref_id`
(joinable) with `…_ref_url` for absolute, urn, and fragment forms.
Choice elements (`value[x]`) become one column set per allowed type; the
open ~54-type choices are force-split into their own tables to respect
PostgreSQL's column limit.

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
