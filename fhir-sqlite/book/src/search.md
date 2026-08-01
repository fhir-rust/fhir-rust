# Search

fhir-sqlite compiles the official SearchParameter definitions against the
generated schema at asset-build time: 94.8% of R5's 1,972 parameters
resolve to concrete (table, column) targets, each backed by a generated
index. The remainder (composites, specials, `exists()`-style expressions)
are recorded with the reason and reported as unsupported — the server
never guesses.

Supported semantics:

- **token** — `gender=female`, `identifier=http://sys|MRN-1`, bare `|code`
  and `system|` forms; boolean tokens (`active=true`).
- **string** — prefix by default, `:exact`, `:contains`; multi-part
  elements (HumanName, Address) match any part. Matching is insensitive to
  **case, accents, and Unicode composition**: `family=muller` finds
  `Müller`, and both the precomposed and decomposed spellings of `é` find
  each other. `:exact` is the exception, and deliberately so — it compares
  the literal stored string, accents included.
- **date** — `eq ne lt gt ge le sa eb` prefixes with FHIR precision
  ranges (`birthdate=1980` matches `"1980-11"`); Period elements use
  overlap semantics.
- **number / quantity** — `value-quantity=gt100`,
  `120.5|http://unitsofmeasure.org|mm[Hg]`.
- **reference** — `subject=Patient/123`, bare ids, absolute URLs; and
  single-hop **chains** with an explicit type:
  `Observation?subject:Patient.family=Smith`.
- **OR** within a parameter (`code=a,b`), **AND** across parameters.
- Result parameters: `_id`, `_lastUpdated`, `_count` (≤1000), `_sort`
  (base-table parameters, `-` for descending), `_total=accurate`,
  `_include=Type:param`, `_revinclude=Type:param`, keyset `_cursor`
  paging (automatic in `next` links when unsorted), `_offset`.

Everything a query sends is bound as a SQL parameter — user input is
never interpolated into SQL text.
