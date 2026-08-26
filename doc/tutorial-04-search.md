# Tutorial 4 — FHIR® search

FHIR search parameters compile to SQL predicates over the normalized columns.
No query engine, no interpreter — the generator turns each parameter into a
predicate template at build time, and search binds values into it.

Normative reference: [`spec/06-search.md`](../spec/databases/06-search.md).

## The call

```rust
let ids = store.search(
    "Patient",
    &[("name".into(), "smith".into()),
      ("birthdate".into(), "ge1970".into())],
    50,   // count
    0,    // offset
).await?;
```

Parameters are `(name, value)` pairs exactly as they would appear in a FHIR
query string. `search` returns ids; `search_full` returns reconstructed
resources; `search_page` returns a cursor-paged result.

## Parameter types

| Type | Example | Compiles to |
| --- | --- | --- |
| string | `name=smith` | prefix range on `family_norm` |
| token | `identifier=http://acme|12345` | `(system, code)` equality |
| date | `birthdate=ge1970-01-01` | range on `birth_date_sort` |
| number | `probability=gt0.8` | range on the derived sort column |
| quantity | `value-quantity=5.4|http://unitsofmeasure.org|mg` | value + system + code |
| reference | `subject=Patient/123` | `(ref_type, ref_id)` equality |
| uri | `url=http://example.org/fhir/ValueSet/x` | equality |

Composite and special parameters may be deferred; the generated documentation
records support per parameter (`P6.1`). For R5, 94.8% of parameters compile —
measured on `fhir-postgresql`.

## String search

Default is **case- and accent-insensitive prefix match**:

```text
("name", "smi")      // matches Smith, Smythe-Jones, ŚMIGŁY
("name", "aero")     // matches Ærø
("name", "munoz")    // matches Muñoz
```

Two modifiers change it:

```text
("name:exact",    "Ærø")   // literal string — compares the stored column
("name:contains", "mit")   // unanchored — folds both sides, and scans
```

`:exact` deliberately compares the *stored* column rather than the folded one,
because it is defined as the literal string. `:contains` folds both sides and
remains a scan, as an unanchored match must.

### Why "aero" finds "Ærø"

Because both sides go through the same function. Each string search column has a
`_norm` companion holding the folded value, computed **in Rust at write time**;
the search term is folded by the same function before binding (`P6.6`).

The fold: decompose to NFD → drop combining marks → lowercase → decompose and
drop marks again → expand the letters that have no decomposition.

That fourth step is not redundant — lowercasing can *introduce* a mark, since
`İ` (U+0130) lowercases to `i` plus a combining dot. The fifth handles the
letters NFD cannot reach, because they are single codepoints carrying a stroke
or a ligature rather than a base plus a mark: `æ`→`ae`, `œ`→`oe`, `ø`→`o`,
`đ`/`ð`→`d`, `ł`→`l`, `ß`→`ss`, `þ`→`th`, and others (`L6`).

`å` never needed step 5 — it is `a` plus a combining ring, which step 2 handles.
That distinction is exactly why the two steps are separate.

What the fold deliberately does **not** do (`L8`, `L9`):

- **No transliteration between scripts.** Greek and Cyrillic fold their combining
  marks like any other script (`ό`→`ο`, `й`→`и`), which is accent-insensitive
  search working consistently. Romanising them would make "the same string" a
  property of a romanisation policy rather than of the text.
- **No locale sensitivity.** Turkish `İ` folds to `i`, not to dotless `ı`. A
  locale-sensitive fold would make stored values depend on server configuration,
  so the same database would answer differently after a config change.

Because the fold is pure Rust, it is byte-identical on all six engines, needs no
database extension, and does not depend on any engine's collation tables or
Unicode version (`X15.4`).

## Date search

FHIR prefixes, with precision-aware ranges (`P6.2`):

```text
("birthdate", "1974")          // eq — the whole year
("birthdate", "ge1974-12")     // ge — from 1974-12-01 onward
("date",      "sa2026-01-01")  // sa — starts after
```

`eq`, `ne`, `lt`, `gt`, `ge`, `le`, `sa`, `eb` are all supported, and they
compare against the derived `_sort` columns. Precision matters: `eq1974` is a
range over the year, not equality against a padded date.

## Token search

```text
("identifier", "http://acme.org/mrn|12345")  // system and code
("identifier", "|12345")                     // code, no system
("identifier", "12345")                      // code, any system
("gender",     "female")                     // a plain code
```

Indexed on `(system, code)` pairs (`P6.4`).

## Reference search

```text
("subject", "Patient/123")   // type and id
("subject", "123")           // id, any type
```

Indexed on `(ref_type, ref_id)`.

## Result parameters

```text
("_count", "100")           // default 50, max 1000
("_sort",  "birthdate")     // any searchable parameter
("_id",    "example")
("_lastUpdated", "gt2026-01-01")
("_total", "accurate")      // or "estimate"
```

Paging is by **opaque cursor** rather than offset, via `search_page` (`P6.3`).
Offset paging is available and is the wrong default for large result sets, for
the usual reason.

## Bounded cost is a requirement, not a nicety

`P6.7` requires that a single search have bounded cost, and it has a clause that
is really a patient-safety rule:

> `_include`/`_revinclude` expansion MUST be capped and, when the cap truncates,
> MUST add a warning. Silent truncation of clinical results is a patient-safety
> defect, not a performance trade.

A clinician who searches for a patient's medications and gets 50 of 78 with no
indication that 28 are missing has been given a wrong answer that looks like a
right one. Whatever caps you configure, the truncation must be visible.

Result materialization also batches — one query per resource type, not one per
resource.

## Unsupported parameters

A parameter that does not compile returns a warning and is ignored, per FHIR's
lenient handling; under strict handling it errors (`P6.5`). It is never silently
dropped, because a search that quietly ignores a filter returns *more* than
asked for, and in a clinical context that is the dangerous direction.

## Injection

Every user-supplied value binds as a parameter (`P6.8`) — including in
`LIMIT`/`OFFSET`, sort direction, and cursor decoding. The only interpolated
fragments are table and column names taken from the generated relational map and
quoted by the dialect's own rule; a value never goes through that path.

There is a committed fuzz corpus for this at
`fuzz/seeds/search_sql/injection.txt`, and `T11.9` requires it be run rather
than merely committed.

## Under the hood

```rust
// This search…
store.search("Patient", &[("name".into(), "smi".into())], 50, 0).await?;

// …becomes roughly this, with the term bound, never interpolated:
//   SELECT DISTINCT n.rid
//     FROM patient_name n
//    WHERE n.family_norm >= $1 AND n.family_norm < $2
//    LIMIT $3 OFFSET $4
// with $1 = fold("smi") and $2 = the computed upper bound.
```

A **range**, not `LIKE $1 || '%'` (`P6.6a`). Planners extract a prefix from a
constant pattern only, so the `LIKE` form degrades to a sequential scan in the
generic plan while looking correct in any hand-run `EXPLAIN` with a literal.
That bug is invisible in exactly the way that matters: it shows up as latency
under load, never as a wrong answer.

## Next

- [Tutorial 5 — history and audit](tutorial-05-history-and-audit.md)
- [The folding specification](../spec/databases/locale-accent-folding.md) — normative, and
  worth reading before changing anything about matching

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
