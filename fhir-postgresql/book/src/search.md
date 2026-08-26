# Search

`fhir-postgresql-gen` compiles the official SearchParameter definitions
against the generated schema at asset-build time. Measured against R5's
1,972 SearchParameters after **F-38** (a value-restriction bug that had
silently over-matched on 51 of them was found and fixed;
[`spec/databases/audit.md`](../../../spec/databases/audit.md) has the detail),
**1,823 (92.4%)** resolve to a concrete `(table, column)` target, each backed
by a plain B-tree index — this port emits no GIN, GiST, or trigram indexes.
The remainder (composites, specials, `exists()`-style expressions, and the
four parameters that F-38 demoted from a wrong answer to an honest "not
supported") are recorded with the reason and reported as unsupported —
`store.search(...)` returns `StoreError::Unsupported`, naming the parameter
or modifier, rather than silently dropping a restriction (`C0.11`).

This chapter describes `Store::search`, `::search_full`, and `::search_page`
(`crates/fhir-postgresql-store/src/lib.rs`); there is no HTTP query string
here, only `&[(String, String)]` pairs the caller builds.

Supported semantics, verified live in `tests/search_semantics.rs`:

- **token** — `gender=female`, `identifier=http://sys|MRN-1`, bare `|code`
  and `system|` forms; boolean tokens (`active=true`).
- **string** — prefix by default, `:exact`, `:contains`; multi-part
  elements (HumanName, Address) match any part. Matching is insensitive to
  **case, accents, and Unicode composition**: `family=muller` finds
  `Müller`, and both the precomposed and decomposed spellings of `é` find
  each other. `:exact` is the exception, and deliberately so — it compares
  the literal stored string, accents included. See
  [Querying](querying.md#search-compares-the-folded-column-not-the-one-you-see)
  for the generated SQL this compiles to.
- **date** — `eq ne lt gt ge le sa eb` prefixes with FHIR® precision ranges
  (`birthdate=1980` matches `"1980-11"`); Period elements use overlap
  semantics.
- **number / quantity** — `value-quantity=gt100`,
  `120.5|http://unitsofmeasure.org|mm[Hg]`.
- **reference** — `subject=Patient/123`, bare ids, absolute URLs; and
  single-hop **chains** with an explicit type:
  `[("subject:Patient.family".into(), "Smith".into())]`. A second hop
  (`.` twice) is refused as unsupported rather than silently truncated.
- **OR** within a parameter (`code=a,b`, comma-separated), **AND** across
  parameters (multiple entries in the slice).

Everything a query value becomes is bound as a SQL parameter — user input is
never interpolated into SQL text (`search_semantics.rs`'s own module doc
states this as the property under test).

## What is a function argument, not a search parameter

Unlike a FHIR REST query string, paging, sorting, and totals are typed
arguments to `search_full`/`search_page`, not strings parsed out of the
parameter list:

| FHIR REST concept | Here |
| --- | --- |
| `_count`, `_offset` | `count: i64`, `offset: i64` arguments |
| `_sort` | `sort: &[search::SortKey]` — base-table columns only (`_id`, `_lastUpdated`, or a base-table search parameter); a child-table sort key is refused, not silently misordered |
| `_total=accurate` | `want_total: bool` |
| keyset paging | `after_id: Option<&str>` on `search_page`, ordering by `id` |

There is no ceiling on `count` enforced here — `search_page` binds whatever
`i64` it is given. A caller exposing this over HTTP (`fhir-loco`, or your own
service) is where a request-size limit belongs, the same way the
[trust boundary](trust-boundary.md) draws that line for every other
perimeter concern.

## `_include` is a primitive, not a search parameter — and `_revinclude` does not exist

`store.search(...)` does not accept `_include` or `_revinclude` as an entry
in `params`; passing either fails as an unsupported parameter, because
neither names a real SearchParameter. What exists instead:

- **`refs_of(rtype, ids, param)`** resolves one compiled *reference* search
  parameter across a set of ids to the `(type, id)` pairs it points at — the
  primitive `_include` needs. It is a separate call your code makes after
  `search`, not a modifier on it.
- **`get_all(items)`** reconstructs several resources in one snapshot, which
  is how you would materialize what `refs_of` found alongside the original
  page.
- There is **no reverse-reference lookup** in this crate — nothing finds
  "resources that reference this id" the way `_revinclude` requires. A
  caller wanting that has to run its own `search` with a `subject=` (or
  equivalent reference) parameter per result, or add the capability; it is
  not here today.

This is a correction from an earlier version of this chapter, which listed
`_include=Type:param` and `_revinclude=Type:param` as ordinary search result
parameters. They are not — verified against `crates/fhir-postgresql-store/src/search.rs`
and `lib.rs`, which contain exactly one function (`refs_of`) in this family.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
