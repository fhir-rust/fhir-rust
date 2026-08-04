# Querying with SQL

Loaded FHIR data is ordinary relational data. Every FHIR version installs into
its own MySQL *database* (`r5`, `r4`, `r3` — MySQL has no separate schema
concept, so a database is the namespace, `M14.21`); the examples below assume
`USE r5;` or a `r5.`-qualified name.

```sql
-- Patients and their observation counts
SELECT n.family, count(o.id) AS observations
  FROM r5.patient p
  JOIN r5.patient_name n ON n.rid = p.id AND n.ords = '{1}'
  LEFT JOIN r5.observation o
    ON o.subject_ref_type = 'Patient' AND o.subject_ref_id = p.id
 GROUP BY n.family
 ORDER BY observations DESC;

-- Blood-pressure observations by LOINC code, with values
SELECT o.id, o.value_quantity_value, o.value_quantity_code
  FROM r5.observation o
  JOIN r5.observation_code_coding c
    ON c.rid = o.id AND c.system = 'http://loinc.org' AND c.code = '85354-9';

-- Search an extension by url and value
SELECT rid FROM r5.patient_ext
 WHERE url = 'http://hl7.org/fhir/StructureDefinition/patient-birthPlace'
   AND leaf = 'valueAddress.city' AND v_text = 'Springfield';
```

## Addressing a repeating element

`ords = '{1}'` addresses the first instance of a repeating element — that form
survives unchanged from the PostgreSQL original.

**The subscript form does not survive.** PostgreSQL's `ords[1] = 1`, matching
any descendant of the first instance, relies on an array type MySQL does not
have: here `ords` is `VARBINARY(255)`, holding the literal text image
(`{1,2}`, `{-1,3}`, …), not a typed array (`M14.8`, `M14.13`). The nearest
equivalent is a prefix match on that image:

```sql
-- Any descendant of the first instance of a repeating element
SELECT * FROM r5.patient_name_given
 WHERE ords LIKE '{1,%';
```

This is a prefix match on stored text, not a typed subscript — it is
correct because `fmt_ords` always writes the array with a leading `{`, so
`{1,` cannot appear anywhere but the start of an ordinal path beginning with
`1`. It does not generalize to matching, say, "second element at any depth";
for that, match a `%,1}` or `%,1,%` fragment against the specific shape you
need.

## Other tips

- Temporal comparisons belong on the `*_sort` columns
  (`birth_date_sort DATE`, `issued_sort DATETIME(6)`, …); the lexical column
  (`birth_date TEXT`, …) preserves what the client sent, including partial
  dates like `"2026-07"` that no native date type can hold.
- Decimal comparisons have no materialized sort column to query — the store's
  own search builder casts inline, `CAST(value_quantity_value AS
  DECIMAL(65,30))`, because a per-value lexical form cannot be captured by a
  fixed declared scale (`M3.6`, `M14.15`). A hand-written query needing a
  numeric range should do the same rather than sorting the `TEXT` column
  lexicographically, under which `"9" > "10"`.
- There is no `fhir-mysql transform` command and no CLI in this workspace
  (`C0.17`) — the fastest way to learn a table's layout is to read the
  generated map asset (`crates/fhir-mysql-map/assets/*.json.gz`), which
  carries a FHIR-path annotation for every table and column, or to call
  `fhir_mysql_map::shred::shred` on a sample resource in a small Rust program
  and inspect the rows it returns.
- Write queries against one version database at a time; `r4` and `r5` name
  tables identically where the specs agree, but nothing joins across them.
