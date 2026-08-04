# Search

fhir-sqlite compiles the official SearchParameter definitions against the
generated schema at asset-build time — this compiler lives in
`fhir-sqlite-gen`, byte-identical across all six ports (`X15.1`), so the
figure below is not specific to SQLite. As of the last regeneration
(**F-38**/**F-40**), **92.4%** of R5's 1,972 parameters resolve to a concrete
`(table, column)` target — 1,823 of them, down from a pre-fix 94.8% (1,870)
after the generator stopped silently dropping a `where()` value restriction
on four colliding codes rather than mis-resolving it (`P6.1`). Every
uncompiled parameter is recorded with its reason (composites, `special`s,
`exists()`-style expressions) rather than guessed at; a query naming one
fails with `StoreError::Unsupported`, never a wrong answer.

Turning a compiled target into SQL is **not** shared: `sqlite_search.rs` is a
fork of the PostgreSQL store's search builder, not the same code running
against a different dialect (about 18 points genuinely differ — placeholder
syntax, casts, `LIKE` versus `ILIKE`). What is shared is the *decision* of
what each parameter means; what is dialect-specific is the SQL that
implements it.

## Supported semantics, verified against `sqlite_search.rs` and its callers

- **token** — `gender=female`, `identifier=http://sys|MRN-1`, bare `|code`
  and `system|` forms.
- **string** — prefix match by default, `:exact`, `:contains`; multi-part
  elements (HumanName, Address) match any part, one row per table.
  Non-`:exact` matching is insensitive to **case, accents, and Unicode
  composition** — `family=muller` finds `Müller`, and both the precomposed
  and decomposed spellings of `é` find each other — because it compares a
  folded companion column populated by `fold::fold` at write time (see [The
  storage model](storage-model.md)), not by comparing at query time. `:exact`
  is the deliberate exception: it compares the literal stored string, accents
  and case included. All three confirmed by running the store directly
  (`family:exact=Smith` matches, `family:exact=smith` does not,
  `family:contains=mit` matches).
- **date** — `eq ne lt gt ge le sa eb` prefixes with FHIR precision ranges
  (`birthdate=1980` matches a stored `"1980-11"`); `Period`-valued elements
  use overlap semantics against both bounds, with an open-ended period
  treated as extending to `9999-12-31T23:59:59.999999Z` (the sentinel
  `M14.12` documents).
- **number / quantity** — `value-quantity=gt100`,
  `120.5|http://unitsofmeasure.org|mm[Hg]`. Comparisons run as
  `CAST(col AS REAL) op CAST(? AS REAL)`, deliberately: the column holds the
  exact lexical form `M3.6` requires (`"9"` must not sort before `"10"` as
  text), so numeric range search casts explicitly rather than relying on
  affinity. **This costs the index** — a `CAST` on the indexed column turns a
  range scan into a full table scan — which is the stated price of
  correctness until `M14.11`'s derived `REAL` sort columns exist for every
  numeric target, not only `decimal`.
- **reference** — `subject=Patient/123`, bare ids, absolute/urn/fragment
  URLs; and single-hop **chains** with an explicit type —
  `Observation?subject:Patient.family=Smith` — confirmed working by running
  it directly. A second hop (`a:B.c:D.e=f`) is rejected with
  `StoreError::Unsupported`, not silently truncated; the code enforces this
  by depth, not by convention.
- **OR** within a parameter via a comma list (`code=a,b`); **AND** across
  parameters.
- `_id` and `_lastUpdated` are recognized directly by `build_search_sql`.
  Everything else that is not a compiled `SearchDef` code — including result
  parameters like `_count`, `_sort`, `_total`, `_include`, `_revinclude`, and
  `_offset` — is **not** a string the store parses out of the parameter list.
  They are ordinary typed arguments on
  [`search_full`/`search_page`](architecture.md): `count`, `offset`,
  `sort: &[SortKey]`, `want_total: bool`, and a keyset `after_id: Option<&str>`
  for paging past a given id. `_include`/`_revinclude` resolution is a
  separate call, `refs_of(rtype, ids, param)`, which a caller composes with a
  search rather than the search performing it inline. Turning HTTP query
  syntax into these calls is `fhir-loco`'s job, not this crate's — see
  [Architecture](architecture.md).

Everything a query sends is bound as a SQL parameter (`?N`); user input is
never interpolated into SQL text — the fuzz target `fuzz/fuzz_targets/search_sql.rs`
exists specifically to keep that true under adversarial input.

## A gap found while verifying this chapter

**Boolean token search does not accept FHIR's own spelling.** `active=true`
finds nothing, even against a `Patient` stored with `"active": true`.
Booleans shred to a plain `INTEGER` (0/1) column (`M14.10`), and a token
value crosses as bound `TEXT`; SQLite's comparison-affinity rule converts a
bound value to numeric affinity only when it *looks like* a number, and
`"true"` does not, so `code_c = ?` compares the text `"true"` against the
integer `1` and never matches. `active=1` does match, which is not valid
FHIR token syntax. This is the same class of defect the conformance matrix
records against `fhir-oracle` (`ORA-01722`, binding `"true"` against a
`NUMBER(1)` column, fixed there with an `i64` bind path) — confirmed here by
running the store directly, not inferred from the Oracle case. It is not
listed as a fix in the [dialect annex](../../spec/14-sqlite-dialect.md) or the
[conformance matrix](../../../spec/databases/conformance-matrix.md) as of
this writing, so treat it as an open, unfiled gap rather than a documented
limitation.
