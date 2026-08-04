# Querying with T-SQL

Loaded FHIR data is ordinary relational data. Identifiers below are
**bracket-quoted**, never double-quoted (`M14.5`): a double quote is a
string delimiter here unless the session has `QUOTED_IDENTIFIER ON`, which
is the default but is session state, and a schema must not depend on
session state.

```sql
-- Patients and their observation counts
SELECT n.[family], COUNT(o.[id]) AS observations
  FROM [r5].[patient] p
  JOIN [r5].[patient_name] n
    ON n.[rid] = p.[id] AND n.[ords] = CONVERT(VARBINARY(255), '{1}')
  LEFT JOIN [r5].[observation] o
    ON o.[subject_ref_type] = 'Patient' AND o.[subject_ref_id] = p.[id]
 GROUP BY n.[family]
 ORDER BY observations DESC;

-- Blood-pressure observations by LOINC code, with values
SELECT o.[id], o.[value_quantity_value], o.[value_quantity_code]
  FROM [r5].[observation] o
  JOIN [r5].[observation_code_coding] c
    ON c.[rid] = o.[id]
   AND c.[system] = N'http://loinc.org' AND c.[code] = N'85354-9';

-- Search an extension by url and value
SELECT [rid] FROM [r5].[patient_ext]
 WHERE [url] = N'http://hl7.org/fhir/StructureDefinition/patient-birthPlace'
   AND [leaf] = 'valueAddress.city' AND [v_text] = N'Springfield';

-- Page through a result set the way search_page does (M14.22)
SELECT [id] FROM [r5].[patient]
 ORDER BY [id] ASC
 OFFSET 20 ROWS FETCH NEXT 10 ROWS ONLY;
```

These are illustrative, hand-written against the same column names
`mssql.rs`/`mssql_search.rs` bind, not captured from a live run in this
documentation pass — treat them as a starting point, not a golden file.

Tips:

- **`ords` is `VARBINARY(255)`, not a text or array column** (`M14.13`) —
  the only one of the six ports that binds it as bytes. It holds the exact
  ASCII bytes of the same text image every other port stores (`{1}`, `{}`,
  `{-1,3}`), so `CONVERT(VARBINARY(255), '<image>')` — a plain, non-`N`
  string literal, since the image is ASCII-only — reproduces what was
  written. There is no array-index operator to reach for the way PostgreSQL
  has `ords[1] = 1`; matching "any descendant of the first instance" needs a
  `LIKE`/prefix comparison against the byte image instead.
- Temporal comparisons belong on the `*_sort` columns (`DATE`/`DATETIME2(6)`);
  the lexical `NVARCHAR(MAX)` column preserves what the client sent and does
  not sort correctly on its own (partial dates like `"2026-07"` are not
  fixed-width).
- **A token's `[system]`/`[code]` columns cannot be indexed** on this
  engine — they are `NVARCHAR(MAX)` (`M14.16`) — so an equality filter on
  them, as in the LOINC example above, is correct but scans. `TextC`
  columns (450 chars, the folded/exact string columns) index normally.
- There is no `fhir-mssql transform` command or any other CLI (`C0.17`,
  `C0.18`); this crate is a library. To learn a table layout, read the
  generated map asset directly — `crates/fhir-mssql-map/assets/fhir-mssql-relmap-r5.json.gz`
  carries a FHIR-path annotation for every table and column — or install the
  schema with `scripts/db.sh up && scripts/db.sh client` and query
  `sys.columns`/`sys.tables` in the target schema.
- Write queries against one version schema at a time; `r4` and `r5` name
  tables identically where the specs agree, but nothing enforces that a
  query only touches one — a cross-schema join compiles and returns
  meaningless results if the two versions modeled the same path
  differently.
