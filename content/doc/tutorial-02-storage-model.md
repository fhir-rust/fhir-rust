# Tutorial 2 — the storage model

What a FHIR resource actually becomes. This is the idea everything else rests
on; once it clicks, the SQL, the search, and the porting all follow.

Normative reference: [`spec/03-storage-model.md`](../spec/databases/03-storage-model.md).

## The rule in one sentence

**Scalars become columns; repeating elements become tables.**

Everything else — nesting, choices, references, extensions, recursion — is a
consequence of applying that consistently.

## Worked example

```json
{
  "resourceType": "Patient",
  "id": "example",
  "active": true,
  "birthDate": "1974-12",
  "maritalStatus": { "text": "Married" },
  "name": [
    { "family": "Ærø", "given": ["Anna", "Marie"] },
    { "family": "Nielsen" }
  ],
  "managingOrganization": { "reference": "Organization/hl7" }
}
```

becomes:

```
patient
  id       version_id  last_updated  active  birth_date  birth_date_sort  marital_status_text  managing_organization_ref_type  managing_organization_ref_id
  example  1           2026-07-31…   true    1974-12     1974-12-01       Married              Organization                    hl7

patient_name
  rid      ords   family    family_norm
  example  {1}    Ærø       aero
  example  {2}    Nielsen   nielsen

patient_name_given
  rid      ords    value   value_norm
  example  {1,1}   Anna    anna
  example  {1,2}   Marie   marie
```

Five things to read off that.

## 1. Non-repeating complex elements flatten

`maritalStatus` is a `CodeableConcept` — a complex type — but it does not
repeat, so it does not get a table. Its scalar leaves become **prefixed columns**
on the enclosing table: `marital_status_text` (`M3.5`).

Only its *repeating* descendants would open tables. `maritalStatus.coding` is
`0..*`, so there is a `patient_marital_status_coding` table too.

There are no shared "coding" tables. Each usage site owns its rows, which is why
`SELECT * FROM patient_marital_status_coding` means exactly one thing.

Two exceptions force a table for a non-repeating element:

- A flattened expansion wider than 150 columns — this catches open `value[x]`
  choices with their ~54 types, and keeps every table well below the engine's
  column limit (`G2.6`).
- Backbone elements a `contentReference` targets cyclically.

## 2. `ords` is the position path

`ords` is the 1-based index at **each repeating ancestor crossing**, from the
resource root down to the element.

```
{1}     the first  name
{2}     the second name
{1,1}   the first  given of the first name
{1,2}   the second given of the first name
```

An array rather than one column per level, and that choice is what makes
recursion work. `Questionnaire.item.item.item…` is unbounded in depth, and it
shares **one** table at every depth — recursion just appears as a longer path.
23 R5 resource types own such tables.

Three properties of the value domain matter, and together they rule out every
obvious optimization (`M3.4a`):

- **Negative ordinals occur.** When two cyclic `contentReference` referrers
  share a table, the second negates its ordinals so paths stay unambiguous. The
  domain is `-32767..=-1 ∪ 1..=32767`; `0` never appears.
- **The empty path `{}` is valid and common.** Resource-level extensions and
  element ids land there, and it is reconstruction's base-row sentinel.
- **Depth is unbounded** for recursive types.

The database never orders, compares, subscripts, or unnests `ords`. It enforces
uniqueness as part of the primary key `(rid, ords)` and hands the value back;
everything else happens in Rust. That is why engines without an array type can
store it as text (`M3.4b`) — PostgreSQL uses `smallint[]`, everyone else stores
the literal `{1,2}`.

## 3. Primitives keep their lexical form

`birth_date` holds `"1974-12"`, verbatim, as text. `birth_date_sort` holds
`1974-12-01` as a real date, derived at write time.

FHIR partial dates — `"1974"`, `"1974-12"` — are not representable in a native
date type without inventing precision the source did not have. So the pattern is
**store the text, derive the sort column** (`M3.6`). Range search and ordering
use the derived column; round-trip uses the stored one.

Decimals work the same way and for the same reason. `1.50` is not `1.5`
clinically, and neither `DECIMAL(65,30)` (which returns
`1.500000000000000000000000000000`) nor `REAL` (which cannot represent either
distinctly) can hold the difference. So `Numeric` binds to a text type on every
engine, with a derived sort column for range queries (`M3.6a`).

## 4. References are parsed, not stored as strings

`"Organization/hl7"` becomes `managing_organization_ref_type = 'Organization'`
and `managing_organization_ref_id = 'hl7'` — joinable columns (`M3.9`). Absolute
or otherwise non-relative references go verbatim into `…_ref_url`. The parse is
reversible: the original string reconstructs exactly.

**There are no cross-resource foreign keys** (`M3.10`). FHIR permits dangling
references, and enforcing them would make load order matter and reject
real-world data. An advisory integrity report replaces the constraint.

The foreign keys that *do* exist are `rid` from every child table to its base
table, with `ON DELETE CASCADE`, so rewriting a resource clears its rows.

## 5. `_norm` columns back the search

Every string search target gets a `_norm` companion holding the folded value
(`P6.6`) — see [tutorial 1](tutorial-01-getting-started.md#step-5--search) and
the [folding spec](../spec/databases/locale-accent-folding.md).

The column is declared with a **binary, NO PAD** collation (`M3.6b`), which
matters twice: `:exact` matching and primary-key identity both need `'Smith'` to
differ from `'Smith '`, and prefix search needs codepoint ordering to be sound.

## Choices

`value[x]` becomes one column per allowed type, plus a `CHECK` that at most one
is populated (`M3.8`):

```
observation
  value_boolean   value_string   value_integer   …
```

Complex alternatives get child tables instead — `observation_value_quantity`.
For the widest choices this is what trips the 150-column split rule.

## Extensions

Extensions are the intricate part, and they are stored **relationally**, not as
JSON. One table per resource type, holding typed leaf rows (`M3.11`):

```
patient_ext(rid, path, ords, modifier, ext_ord, url, leaf, v_kind, v_text, v_num, v_bool)
```

- `path` + `ords` locate the attach point — `""` for the resource itself.
- `ext_ord` is the index in the extension array; `modifier` separates
  `modifierExtension`.
- `url` is the extension's url, denormalized so you can query by it.
- `leaf` addresses one scalar inside the extension's content as a dotted path
  whose all-digit segments are array indexes:
  `valueCodeableConcept.coding.0.code`. Nested extensions are just longer leaves
  — `extension.0.valueString`.
- `v_kind` ∈ `s`/`n`/`b`/`z` tags the JSON scalar kind; numbers keep their
  lexical form in `v_text` and a queryable numeric in `v_num`.

One uniform encoding covers every extension value type, including arbitrarily
nested complex ones, with no JSON and no per-type tables. Primitive extensions
(`_birthDate`) reuse it with the primitive's path, and element ids ride along as
`ext_ord = 0, leaf = 'id'` rows (`M3.12`).

## The three places JSON survives

Fully relational storage has exactly three sanctioned exceptions, and each has a
reason:

| Where | Why |
| --- | --- |
| `<resource>_history.resource` | write-once audit data, read only by vread/history/audit. Normalizing every historical version would multiply the hardest part of the system for no query benefit (`H5.1`). |
| `<resource>_contained.resource` | contained resources are anonymous whole resources of unknowable type (`M3.13`). |
| `Bundle.entry.resource`, `Parameters.parameter.resource` | same reason. |

Even these are stored in a **text** column, never a JSON-typed one (`M3.6c`),
because a JSON column re-normalizes what it is given — reordering keys,
rewriting number spellings — and the history hash chain commits to exact bytes.

## The type cycle

The FHIR type graph has one genuine datatype cycle: `Reference.identifier` is an
`Identifier`, and `Identifier.assigner` is a `Reference`. Static expansion cuts
it at the element that would re-enter an in-expansion type, and stores anything
below the cut as leaf rows in a `<resource>_deep` table using the same encoding
as extensions (`M3.14`). Lossless, relational, and vanishingly rare in real
data.

## Identifier names

`Patient.name.given` → `patient_name_given`. Element paths snake_case; table
names concatenate resource and path (`G2.3`).

Names are budgeted to **63 bytes** on every port, which is PostgreSQL's limit
and tighter than any other target's. That is deliberate: a name generated once
is legal everywhere, and two ports' schemas can be compared name-for-name
(`X15.3`). Where a name would exceed the budget the generator abbreviates
deterministically and, on residual collision, appends a 6-hex-digit hash of the
full path — and records the mapping, so two paths can never collide silently
(`G2.4`).

## Next

- [Tutorial 3 — querying with SQL](tutorial-03-querying-sql.md)
- [Storage model reference](storage-model.md) for looking things up
- [`spec/03-storage-model.md`](../spec/databases/03-storage-model.md) for the normative
  text
