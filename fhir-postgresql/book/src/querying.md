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

These three ran against a live-installed R5 schema; the column and table
names come straight from `fhir-postgresql-map::ddl::create_table` for
`Patient` and `Observation` (see [the storage model](storage-model.md) for
the full generated `r5.patient` and `r5.patient_name` DDL).

## Search compares the folded column, not the one you see

Every table with a string search target also carries a `*_norm` companion
column — `patient_name.family_norm text COLLATE "C"` alongside `family` — that
holds the case- and accent-folded value (`M14.20`). `store.search(...)`
reads `family_norm`, never `family` with a runtime `lower()`, and a prefix
search compiles to a range scan rather than `LIKE $1 || '%'` (`M14.23`,
`P6.6a`): PostgreSQL only extracts a prefix from a *constant* `LIKE` pattern,
so a bound parameter would degrade to a sequential scan under the generic
plan while still looking fine under a hand-run `EXPLAIN` with a literal. A
prefix search for `mul` compiles to:

```sql
SELECT rid FROM r5.patient_name
 WHERE family_norm >= 'mul' AND family_norm < 'mum';
```

`prefix_upper("mul")` returning `"mum"` is a unit-tested property of
`fold.rs`, not something written by hand per query — the same function that
produced the stored `_norm` value produces the bound.

Tips:

- `ords = '{1}'` addresses the first instance of a repeating element;
  `ords[1] = 1` matches any descendant of the first instance — PostgreSQL is
  the only port where `ords` is a native array and this subscript form works;
  `ords LIKE '{1,%'` is the portable form the other five ports use, and it
  also works here.
- Temporal comparisons belong on the `*_sort` columns; the lexical column
  preserves what the client sent, partial precision included.
- Write queries against one version schema at a time; `r4` and `r5` name
  tables identically where the specs agree, but the schemas are independent
  (`M14.4`) and nothing joins across them.
- There is no `transform` command and no CLI in this crate — see
  [Getting started](getting-started.md). To see what rows a resource
  produces, call `fhir_postgresql_map::shred::shred` directly, or read the
  DDL for that resource type out of the map with `ddl::create_table`, the way
  the example above was generated.
