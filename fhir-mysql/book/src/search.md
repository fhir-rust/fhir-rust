# Search

fhir-mysql compiles the official SearchParameter definitions against the
generated schema at asset-build time, in `fhir-mysql-gen`'s `search.rs` —
shared code, identical across all six ports. The shipped R5 map asset compiles
**1,870 of 1,972 SearchParameters (94.8%)**. That figure is stale in one
respect worth stating plainly: a later audit pass (**F-38**) found the
compiler silently dropping a `where()` value restriction on 51 of those, so
the *correct* rate is **92.4%** — the shipped asset has not yet been
regenerated to reflect the fix, so what is actually installed today is still
the 94.8%-with-a-bug version, not the corrected one. Either way, the
remainder — composites, specials, `exists()`-style expressions — are recorded
in the map with the reason they were skipped, and reported as unsupported
rather than silently ignored or guessed at (`P6.5`).

This is a library API, not a query-string parser: there is no `_count=` or
`_sort=` text this crate reads. `MySqlStore::search_page` (and the narrower
`search`/`search_full`) take result control as typed Rust arguments —
`count: i64`, `offset: i64`, `sort: &[SortKey]`, `want_total: bool`,
`after_id: Option<&str>` — and a caller sitting in front of an HTTP request
(such as `fhir-loco`) is what would translate `_count`, `_sort`, and a cursor
token into them.

**Content** parameters — the filters a FHIR® search actually searches on — are
a `&[(String, String)]` of parameter-code/value pairs, and this part is close
to the wire syntax:

- **token** — `gender=female`, `identifier=http://sys|MRN-1`, bare `|code`
  and `system|` forms.
- **string** — prefix by default, `:exact`, `:contains`; multi-part elements
  (HumanName, Address) match any part. Matching is insensitive to **case,
  accents, and Unicode composition** — folding happens in Rust
  (`fold::fold`), never in a SQL collation (`M14.5`, `M14.6`): `family=muller`
  finds `Müller`, and both the precomposed and decomposed spellings of `é`
  find each other. `:exact` is the exception, deliberately: it compares the
  literal stored string, accents included, against the `TextC` column.
- **date** — `eq ne lt gt ge le sa eb ap` prefixes, compared on the derived
  `*_sort` column with FHIR precision ranges (`birthdate=1980` matches
  `"1980-11"`).
- **number** — prefixed, compared via `CAST(col AS DECIMAL(65,30))` against
  the verbatim `TEXT` column, never lexicographically (`"9" > "10"` as text,
  which is exactly the bug this avoids).
- **quantity** — `[prefix]value[|system|unit]`, e.g.
  `value-quantity=gt120.5|http://unitsofmeasure.org|mm[Hg]`; system and unit
  are optional and, when given, added as equality predicates on the
  quantity's own system/code columns.
- **reference** — `subject=Patient/123`, bare ids, absolute URLs; and
  single-hop **chains** with an explicit target type:
  `subject:Patient.family=Smith`. Chains deeper than one hop are refused with
  `Unsupported`, not silently truncated.
- **`_id`** and **`_lastUpdated`** are handled the same way as any other
  content parameter — comma-separated values OR together, multiple distinct
  parameter codes AND together (`code=a,b` is an OR within `code`; `code=a` +
  `status=final` in the same call is an AND across them).

**Not implemented: `_include` and `_revinclude`.** The core spec asks for
them (`P6.3`, single hop); this port's `mysql_search.rs` has no code path for
either, and no test exercises them. This is an open gap, not a documented
`M14.x` departure — treat any claim otherwise as wrong until it is closed.

Paging is keyset-based: pass the previous page's last id as `after_id` to
narrow the *page* query; `want_total` (when set) runs a separate `COUNT(*)`
against the same predicate, unnarrowed by the cursor, so a client can always
tell how far it has left to page. There is no `_total=estimate` mode — a
requested total is always an exact count.

Everything a value contributes to the query is bound as a parameter
(`?`), never interpolated into SQL text — `search_values_are_bound_never_interpolated`
in `mysql_store.rs` is the test that would fail if this regressed, and a fuzz
target (`fuzz/fuzz_targets/search_sql.rs`) exists for the same invariant.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
