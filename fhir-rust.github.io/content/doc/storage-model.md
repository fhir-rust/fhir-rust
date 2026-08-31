# The storage model — reference

Lookup reference for the schema. [Tutorial 2](tutorial-02-storage-model.md) is
the version to read through; this is the version to come back to.

Normative text: [`spec/03-storage-model.md`](../spec/databases/03-storage-model.md).

## Table kinds

| Kind | Name | Key | Holds |
| --- | --- | --- | --- |
| Base | `<resource>` | `id` | one row per resource; every scalar element |
| Elem | `<resource>_<path>` | `(rid, ords)` | one row per instance of a repeating element |
| Ext | `<resource>_ext` | `(rid, path, ords, modifier, ext_ord, leaf)` | every extension leaf value |
| Deep | `<resource>_deep` | `(rid, path, ords, leaf)` | values below a type-cycle cut |
| Contained | `<resource>_contained` | `(rid, ord)` | contained resources, as text |
| History | `<resource>_history` | `(id, version_id)` | every version, plus audit envelope |

## System columns

**Base** (`M3.2`)

| Column | `ColTy` | Meaning |
| --- | --- | --- |
| `id` | `Text` | resource id, primary key |
| `version_id` | `BigInt` | monotonic per resource, starts at 1 |
| `last_updated` | `Timestamptz` | |

**Child** (`M3.4`)

| Column | Meaning |
| --- | --- |
| `rid` | root resource id; references the base table, `ON DELETE CASCADE` |
| `ords` | 1-based index at each repeating ancestor crossing |

**History** (`H5.1`, `M3.15`, `M3.16`)

| Column | Meaning |
| --- | --- |
| `id`, `version_id`, `last_updated` | |
| `op` | `C`, `U`, or `D` |
| `resource` | whole version, `Jsonb` bound to a text type |
| `actor`, `actor_source`, `client`, `request_id`, `reason` | the audit envelope |
| `prev_hash`, per-algorithm digest columns, tag columns | the chain |

## `ColTy` → FHIR® primitive

| FHIR | `ColTy` | Notes |
| --- | --- | --- |
| boolean | `Bool` | |
| integer, unsignedInt, positiveInt | `Int` | |
| integer64 (R5) | `BigInt` | |
| decimal | `Numeric` | text-bound; lexical precision preserved (`M3.6a`) |
| string, code, id, markdown, uri, url, canonical, oid, uuid, xhtml, base64Binary | `Text` | |
| date | `Text` + `<name>_sort` `Date` | verbatim + derived |
| dateTime, instant | `Text` + `<name>_sort` `Timestamptz` | verbatim + derived |
| time | `Text` | fractional-second fidelity |

Engine bindings: [choosing an engine](choosing-an-engine.md#what-each-engine-costs-you).

## Derived and companion columns

| Suffix | Purpose | Requirement |
| --- | --- | --- |
| `_sort` | typed, indexed companion to a lexical temporal or numeric column | `M3.6` |
| `_norm` | folded companion to a string search column | `P6.6` |
| `_ref_type`, `_ref_id`, `_ref_url` | parsed reference | `M3.9` |

## `ords`

Format: `{}`, `{1}`, `{1,2}`, `{-1,3}` — the literal produced by `fmt_ords`
(`M3.4b`), identical on every engine.

| Property | Consequence |
| --- | --- |
| Domain `-32767..=-1 ∪ 1..=32767`; `0` never occurs | no unsigned or magnitude-only encoding |
| `{}` valid, frequent, and `NOT NULL` | must be distinguishable from null |
| Depth unbounded for recursive types | no fixed-width encoding |

The database only enforces uniqueness and returns the value. No `ORDER BY`, no
subscript, no `unnest` — which is why a text binding suffices where there is no
array type.

Query idiom: `ords = '{1}'` everywhere; `ords[1] = 1` on PostgreSQL only;
`ords LIKE '{1,%'` is the portable equivalent.

## Extension encoding

`<resource>_ext(rid, path, ords, modifier, ext_ord, url, leaf, v_kind, v_text,
v_num, v_bool)` (`M3.11`)

| Column | Meaning |
| --- | --- |
| `path` | dotted JSON-name path to the attach point; `""` for the resource |
| `ords` | ordinals at each repeating crossing on that path |
| `modifier` | distinguishes `modifierExtension` |
| `ext_ord` | 1-based index in the extension array; `0` for element ids |
| `url` | top-level extension url, denormalized for querying |
| `leaf` | dotted path to one scalar; all-digit segments are 0-based indexes |
| `v_kind` | `s` string · `n` number · `b` boolean · `z` null |
| `v_text` | the value, lexically |
| `v_num` | queryable numeric, when `v_kind = 'n'` |
| `v_bool` | when `v_kind = 'b'` |

Examples of `leaf`:

```
valueString
valueCodeableConcept.coding.0.code
extension.0.valueString              nested extension
id                                   with ext_ord = 0 — an element id
```

Primitive extensions (`_birthDate`) reuse the same table with the primitive's
path (`M3.12`).

## The three JSON exceptions

| Where | Why |
| --- | --- |
| `<resource>_history.resource` | write-once audit data (`H5.1`) |
| `<resource>_contained.resource` | anonymous whole resources of unknowable type (`M3.13`) |
| `Bundle.entry.resource`, `Parameters.parameter.resource` | same |

All bound to a **text** type, never a JSON type — a JSON column re-normalizes,
and the hash chain commits to exact bytes (`M3.6c`).

## Table-forcing rules

A non-repeating element normally flattens (`M3.5`). It is forced into its own
table, with fixed ordinal 1, when:

1. the flattened expansion would exceed 150 columns (`G2.6`) — this catches open
   `value[x]` choices with ~54 types;
2. it is a backbone element targeted cyclically by a `contentReference`.

## Identifiers

| Rule | Value |
| --- | --- |
| Case conversion | snake_case (`birthDate` → `birth_date`) |
| Table name | resource + path (`Patient.name.given` → `patient_name_given`) |
| Budget | 63 bytes on every port (`X15.3`) |
| Overflow | deterministic abbreviation, then 6-hex-digit hash of the full path |
| Collision | impossible by construction; the mapping is recorded (`G2.4`) |

## Indexes emitted

`P6.4`: every base-table search column · every child table's foreign key and
`ords` · every reference `(ref_type, ref_id)` · every token `(system, code)`.

Where an engine cannot index a column as bound, the port must narrow the type or
add an indexable derived column rather than drop the index (`P6.4a`).

## Scale

| | R5 | R4 | R3 |
| --- | --- | --- | --- |
| Tables | 7,355 | ~6,000 | ~4,700 |
| Recursive-table resource types | 23 | 18 | 13 |
| Max non-recursive nesting depth | 6 | 6 | 6 |

Example corpus: 7,399 resources across all three versions, all round-tripping
losslessly — measured on `fhir-postgresql`.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
