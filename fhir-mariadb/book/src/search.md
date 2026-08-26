# Search

fhir-mariadb compiles the official SearchParameter definitions against the
generated schema at asset-build time — in the generator, shared byte-for-byte
across all six ports (`X15.1`), so this figure is the same for every one of
them: **92.4%** of R5's parameters resolve to concrete (table, column)
targets, each backed by a generated index (`P6.1`, corrected under audit
**F-38** from a prior 94.8% that silently dropped a `where()` value
restriction on 51 parameters). The remainder (composites, specials,
`exists()`-style expressions) are recorded with the reason and reported as
unsupported — the compiler never guesses. There is no query-time fallback for
an unsupported parameter; a search naming one is rejected as
`StoreError::Unsupported`, naming the parameter, never silently ignored
(`A7.11`).

Supported semantics:

- **token** — `gender=female`, `identifier=http://sys|MRN-1`, bare `|code`
  and `system|` forms; boolean tokens (`active=true`).
- **string** — prefix by default, `:exact`, `:contains`; multi-part
  elements (HumanName, Address) match any part. Matching is insensitive to
  **case, accents, and Unicode composition**: `family=muller` finds
  `Müller`, and both the precomposed and decomposed spellings of `é` find
  each other. `:exact` is the exception, and deliberately so — it compares
  the literal stored string, accents included.
- **date** — `eq ne lt gt ge le sa eb` prefixes with FHIR® precision
  ranges (`birthdate=1980` matches `"1980-11"`); Period elements use
  overlap semantics.
- **number / quantity** — `value-quantity=gt100`,
  `120.5|http://unitsofmeasure.org|mm[Hg]`.
- **reference** — `subject=Patient/123`, bare ids, absolute URLs; and
  single-hop **chains** with an explicit type:
  `Observation?subject:Patient.family=Smith`.
- **OR** within a parameter (`code=a,b`), **AND** across parameters.
- Result handling: `_id` and `_lastUpdated` are recognized specially rather
  than compiled as ordinary parameters; `_sort` works on base-table
  parameters only (`-` prefix for descending; sorting on a child-table
  parameter is a named `Unsupported` error, not a silent no-op, because a
  wrong order is worse than a refusal); an optional total count runs as its
  own query against the same `WHERE` clause, so a caller can ask for it (a
  `_total=accurate` equivalent) without paying for it on every page; and
  paging takes both an explicit count/offset **and** an optional keyset
  cursor — the last id already seen, which adds a `WHERE id > ?` predicate
  ahead of the ordinary `ORDER BY … LIMIT ? OFFSET ?`. The two compose but
  are not the same tool: pass the cursor with `offset = 0` for genuine
  index-friendly keyset paging through a large result set, or use `offset`
  alone for conventional page-N paging, whose cost grows with how deep the
  page is.
- **Not implemented at all: `_include` and `_revinclude`.** There is no
  reference-expansion code anywhere in this store — searching still returns
  only matching resources of the requested type, never resources they (or
  things referencing them) point at. A caller that wants included resources
  has to issue its own follow-up `get`s.

Everything above is store-level search: `Vec<String>` of matching ids (plus
an optional total), not a FHIR `Bundle`. Turning that into `Bundle.entry`,
`link` relations, and an `_include`d resource's own entry is presentation
work this library does not do — `fhir-loco`'s job, not this crate's
(`C0.17`).

Everything a query sends is bound as a SQL parameter — user input is
never interpolated into SQL text.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
