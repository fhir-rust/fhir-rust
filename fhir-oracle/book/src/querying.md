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

- `ords = '{1}'` addresses the first instance of a repeating element;
  `ords[1] = 1` matches any descendant of the first instance.
- Temporal comparisons belong on the `*_sort` columns; the lexical column
  preserves what the client sent.
- `fhir-oracle transform <file>` prints the exact rows any resource produces —
  the fastest way to learn a table layout. The generated map assets also
  carry a FHIR-path annotation for every table and column.
- Write queries against one version schema at a time; `r4` and `r5` name
  tables identically where the specs agree.
