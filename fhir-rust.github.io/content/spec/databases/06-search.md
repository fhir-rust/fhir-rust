# 6. Search

- **P6.1** All standard SearchParameters of each version MUST be compiled by the
  generator into SQL predicate templates over the normalized columns. Search
  types supported: token, string, date, number, quantity, reference, uri;
  composite and special parameters MAY be deferred (documented per parameter in
  generated docs).
- **P6.2** String search default is case-insensitive prefix match (`:exact` and
  `:contains` modifiers supported). Token search matches `system|code`
  semantics. Date search implements FHIR® range/prefix semantics (eq, ne, lt, gt,
  ge, le, sa, eb) against the `_sort` columns with precision-aware ranges.
- **P6.3** Result parameters: `_count` (default 50, max 1000), paging via opaque
  cursor, `_sort` on searchable params, `_id`, `_lastUpdated`,
  `_total=accurate|estimate`, `_include`/`_revinclude` (single hop).
- **P6.4** The generator MUST emit indexes for: every base-table search column,
  every child-table foreign key + `ords`, reference `(ref_type, ref_id)` pairs,
  and token `(system, code)` pairs.
- **P6.4a** Where an engine cannot index a column as bound — a `TEXT`/`MAX`
  column that exceeds an index key limit — the port MUST NOT silently drop the
  index and leave the search to scan. It MUST either narrow the bound type for
  that column or add an indexable derived column holding a bounded prefix, and
  MUST record which in its annex. A search parameter that compiles to SQL and
  then scans the table is `P6.1`-conformant and useless.

  [Unbounded string search](unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md)
  settles *how*, for the ports that need it.

- **P6.5** Unsupported search parameters MUST return a warning and be ignored
  per FHIR's lenient handling, or error where strict handling is requested.
- **P6.6** String search MUST be insensitive to **case, accents, and Unicode
  composition** — FHIR requires it, and a system serving Ærø, Ångström, Müller,
  Muñoz, and Ślusarczyk cannot ship a case-insensitive `LIKE` alone.

  Each string search target column gets a companion `_norm` column holding the
  folded value, **computed by the engine in Rust at write time** (NFD, drop
  combining marks, lowercase, drop marks again, then expand the letters that
  have no decomposition — see [Locale and accent folding](locale-accent-folding.md),
  which is normative for the fold). Queries fold the search term with the same
  function and compare against that column, so there is exactly one definition
  of string equality in the system rather than one in SQL and one in Rust that
  must agree for every codepoint. The column is bound to `TextC` (`M3.6b`), so
  ordering is by Unicode codepoint.

  Folding in Rust is what makes the fold portable: no port depends on an engine
  extension, an engine's collation tables, or an engine's Unicode version, and
  two ports fold a name to the same bytes by construction (`X15.4`). An earlier
  design built on PostgreSQL's `unaccent` needed an IMMUTABLE wrapper, an
  expression index the planner would not use with a parameterized pattern, and a
  deployment-time check for an extension that managed-database tenants often
  cannot install — and it would have had no counterpart on five of six engines.

- **P6.6a** A prefix search MUST be emitted as a **range predicate** — `col >=
  term AND col < upper(term)`, with the upper bound computed in Rust — not as
  `LIKE $1 || '%'`. Query planners extract a prefix from a *constant* pattern
  only, so a `LIKE` against a bound parameter degrades to a sequential scan in
  the generic plan while looking correct in any hand-run `EXPLAIN` with a
  literal.

  `:exact` compares the stored column, not the folded one, because it is defined
  as the literal string. `:contains` folds both sides and remains a scan, as an
  unanchored match must.

- **P6.7** A single search request MUST have a bounded cost. Result
  materialization MUST batch (one query per resource type, not one per
  resource); `_include`/`_revinclude` expansion MUST be capped and, when the cap
  truncates, MUST report the truncation. Silent truncation of clinical results
  is a patient-safety defect, not a performance trade.
- **P6.8** Every generated SQL predicate MUST bind its user-supplied values as
  **parameters**. A search term MUST NOT reach the engine by string
  concatenation, including in `LIMIT`/`OFFSET`, sort direction, and cursor
  decoding. Identifiers that the generator emits — table and column names — come
  from the relational map and are quoted by the dialect's own rule; they are the
  only interpolated fragments, and a value MUST NOT be routed through that path.

  The fuzz corpus committed under `fuzz/seeds/search_sql/injection.txt` is part
  of the evidence for this requirement, and `T11.9` requires it be run.

- **P6.9** A text column an engine cannot index or compare as bound MUST be
  given a **bounded adjunct** and a **checksum adjunct** in the generated map,
  per `U1`–`U10`. Both are required: a bounded adjunct cannot answer equality
  and a checksum adjunct cannot answer a prefix.

  The rules live in their own section because they change the map, which is
  shared verbatim across all six ports (`X15.1`), and because several sections
  depend on them without any one owning them.

---

Part of the [fhir-databases specification](index.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
