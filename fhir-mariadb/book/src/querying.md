# Querying with SQL

Loaded FHIR data is ordinary relational data:

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

Tips:

- `ords = '{1}'` addresses the first instance of a repeating element exactly,
  and survives unchanged from the PostgreSQL idiom this book started from.
  **`ords[1] = 1` does not** — MariaDB has no array type, `ords` is a
  `VARBINARY(255)` column holding the literal image (`{1,2}`, `{}`,
  `{-1,3}`), and there is no subscript operator over it. The nearest
  equivalent for "any descendant of the first instance" is a prefix match on
  that stored image: `ords LIKE '{1,%'`. This is the one user-visible
  regression from the PostgreSQL storage model, and it is deliberate — see
  `M14.13` in the [dialect annex](../../spec/14-mariadb-dialect.md).
- Temporal and decimal comparisons belong on the derived `*_sort` columns
  (`DATE`/`DATETIME(6)`/`DOUBLE`); the lexical `TEXT` column preserves what
  the client sent, verbatim, and is what a reconstruction reads.
- There is no `fhir-mariadb transform` or any other CLI command — this
  library ships no binary (`C0.17`). To learn a table layout, read the
  generated map asset directly (`fhir-mariadb-map/assets/*.json.gz`, decoded
  with `RelMap::from_gz_bytes`): every table and column carries a FHIR-path
  annotation.
- Write queries against one FHIR-version database at a time; `r4` and `r5`
  name tables identically where the specs agree. MariaDB has no schema
  concept separate from a database, so `r5.patient` is `database.table`, not
  `schema.table` (`M14.21`) — the qualified-name shape reads the same either
  way.
