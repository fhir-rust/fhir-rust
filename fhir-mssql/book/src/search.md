# Search

`fhir-mssql-store`'s `search`/`search_full`/`search_page` (`mssql.rs`) share
one query builder, `mssql_search::build_search_sql`, forked from the MySQL
builder rather than parameterizing a shared one (`M14.0a`) — the dialects
diverge at enough points that a shared builder would be more coupling than
any engine gains. All three take raw `(name, value)` pairs; there is no
HTTP query-string parsing here — that belongs to a caller such as
`fhir-loco`, not this crate.

```rust,ignore
let ids: Vec<String> = store.search("Patient", &[("family".into(), "Aero".into())], 10, 0).await?;
let full: SearchOutcome = store.search_full("Patient", &params, 10, 0, &sort, true).await?; // .ids, .total
let page: SearchOutcome = store.search_page("Patient", &params, 10, 0, &sort, true, after_id).await?;
```

The generator compiles the official SearchParameter definitions against the
generated schema at asset-build time — shared code (`gen/`), so the same
figure holds in every port: **1,870 of R5's 1,972 parameters (94.8%)**
resolve to concrete `(table, column)` targets (`doc/benchmarks.md`). The
remainder (composites, specials, `exists()`-style expressions) are recorded
with the reason and reported unsupported rather than guessed. That figure
predates a correction the conformance matrix records under `P6.1`: **F-38**
found 51 of those compiled parameters had silently dropped a `where()`
value restriction, narrowing the true figure to 92.4% — fixed in the
generator, but the map assets bundled with this crate have not been
regenerated since, so what actually ships today is still the 94.8%
compilation, uncorrected. Do not cite 92.4% as this port's shipped
behaviour until the assets catch up.

Supported semantics, checked against `target_pred` in `mssql_search.rs`:

- **token** — `gender=female`, `identifier=http://sys|MRN-1`, bare `|code`
  and `system|` forms; boolean tokens (`active=true`) work because this
  engine coerces a bound `'true'`/`'false'` string to `BIT` — Oracle does
  not, which is how that gap was found (**F-68**).
- **string** — prefix by default (a `>=`/`<` range scan against the folded
  `TextC` column, not a leading `LIKE '…%'`, so a bound parameter still
  seeks the index under a generic plan), `:exact` (compares the literal
  stored value, case- and accent-sensitive), `:contains` (`LIKE '%…%' ESCAPE
  '\'`, so it always scans). Multi-part elements (HumanName, Address) match
  any part. Non-`:exact` matching is case-, accent-, and
  composition-insensitive because it compares the value Rust's `fold::fold`
  already normalized (`P6.6`) — the SQL layer never re-implements folding.
  A map generated before folding existed has no folded companion column;
  the builder falls back to a bare `LIKE`, which is collation-insensitive
  but not the same fold, and the fix is regenerating the map, not patching
  the query.
- **date** — `eq ne lt gt ge le sa eb` prefixes against a derived sort
  column (`Date`/`Timestamptz`, never the lexical text column), with FHIR®
  precision ranges (`birthdate=1980` matches `"1980-11"`); Period elements
  compare a second, end column with overlap semantics.
- **number / quantity** — `value-quantity=gt100`,
  `120.5|http://unitsofmeasure.org|mm[Hg]`. Compared as `CAST(… AS FLOAT)`
  on both sides, never `DECIMAL` (`M14.8`): the stored column holds the
  exact lexical form and a fixed declared scale would both lose precision
  at the comparison boundary and re-introduce the defect `Numeric` exists
  to avoid.
- **reference** — `subject=Patient/123`, bare ids, absolute/urn/fragment
  URLs (matched against a separate `…_ref_url` column); and single-hop
  **chains** with an explicit type: `subject:Patient.family=Smith` compiled
  as a correlated `EXISTS` against the target type's base table. Chains
  deeper than one hop return `Unsupported` rather than being silently
  truncated.
- **OR** within a parameter (`code=a,b`), **AND** across parameters.
- Result parameters actually implemented: `_id`, `_lastUpdated`, `_count`,
  `_offset`, `_sort` (base-table columns only — sorting on a child-table
  parameter returns an error rather than a wrong order, since `sort_col`
  has nowhere to put a `JOIN` in this builder), and `_total` via
  `want_total`. **Not implemented: `_include`, `_revinclude`.** Nothing in
  `mssql_search.rs` or `mssql.rs` builds an inclusion query; do not claim
  them for this port.
- Keyset paging is `search_page`'s `after_id: Option<&str>` parameter,
  compiled as `p.[id] > @Pn` added to the page query only — never to the
  `_total` count, so a `_total` that shrank as a caller paged would make
  paging impossible to drive. Building a `next` link from it is a caller's
  job, not this crate's.

Everything a value binds to is a `@P`*n* parameter — user input is never
interpolated into SQL text (checked in the search-builder's own binding
tests). One SQL-Server-specific finding worth knowing if you touch
`_sort`: unlike MySQL, this engine's `ORDER BY` rejects a column named
twice ("A column has been specified more than once in the order by list")
rather than tolerating it, so the builder's own `_id` tiebreaker is only
appended when the caller's sort keys do not already include one — found by
running an `_id`-sorted search live, not by reading a manual.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
