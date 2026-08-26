# Search

The official SearchParameter definitions are compiled against the generated
schema at asset-build time — shared, engine-independent code
(`fhir-oracle-gen`), identical across all six ports. As of the last
correction (`F-38`), **92.4%** of R5's search parameters resolve to concrete
(table, column) targets, each eligible for a generated index; it was
previously reported as 94.8%, but 51 of those silently dropped a `where()`
value restriction, which is a real behavioral bug, not a rounding
difference. The remainder (composites, specials, `exists()`-style
expressions) are recorded with the reason and reported as unsupported at
compile time — this port's search builder (`oracle_search.rs`) never
guesses at a target it wasn't given.

`oracle_search.rs` implements a predicate for every `TargetKind` the
compiler can emit — string, token, date, number, quantity, reference, uri —
so the semantics below are implemented in the sense that the code path
exists and is exercised by unit tests. **Only two of them are live-verified
against a real Oracle**, by `tests/oracle_store.rs::search_by_token_and_family_name`
(`F-68`): a fold-insensitive `family` (string) search, and a boolean `token`
search (`active=true`). Treat the rest as implemented-but-not-yet-run-live
until a broader live search suite exists.

Supported semantics:

- **token** — `gender=female`, `identifier=http://sys|MRN-1`, bare `|code`
  and `system|` forms; boolean tokens (`active=true`). A bare boolean token
  binds as `0`/`1` against the column's native `NUMBER(1)` type — Oracle,
  unlike SQL Server/MySQL, refuses to implicitly convert the string
  `"true"`/`"false"` to a number, so this had to be a distinct bind path
  (`M14.34`, found live).
- **string** — prefix by default, `:exact`, `:contains`; multi-part elements
  (HumanName, Address) match any part. Matching is insensitive to **case,
  accents, and Unicode composition**: `family=muller` finds `Müller`, and
  both the precomposed and decomposed spellings of `é` find each other.
  `:exact` is the exception, and deliberately so — it compares against the
  SHA-256 digest of the literal stored string, accents included.
- **date** — `eq ne lt gt ge le sa eb` prefixes with FHIR® precision ranges
  (`birthdate=1980` matches `"1980-11"`); Period elements use overlap
  semantics.
- **number / quantity** — `value-quantity=gt100`,
  `120.5|http://unitsofmeasure.org|mm[Hg]`.
- **reference** — `subject=Patient/123`, bare ids, absolute URLs; and
  single-hop **chains** with an explicit type:
  `Observation?subject:Patient.family=Smith`.
- **OR** within a parameter (`code=a,b`), **AND** across parameters.
- Result parameters: `_id`, `_lastUpdated`, `_count`, `_sort` (base-table
  parameters, `-` for descending), keyset `_cursor` paging via
  `search_page` — implemented, but no live test yet requests a second page
  with a cursor.

Everything a query sends is bound as a positional SQL parameter (`:1`,
`:2`, …, per `M14.21`) — user input is never interpolated into SQL text.

## Where equality goes through a digest, not the column

`code`/`system` on token targets, and any unbounded (`CLOB`-typed) string
target, cannot be `=`-compared directly on this engine at all (`M14.9`).
Equality against those goes through the SHA-256 digest adjunct column
(`<col>_h`) instead, computed the same way on write and on search
(`fold::digest`, `U4a`). This is stronger than a workaround: without it,
those searches would not merely be slow, they would be **impossible** to
express as SQL at all against a raw `CLOB` column.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
