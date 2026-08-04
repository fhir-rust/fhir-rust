# Querying with SQL

Loaded FHIR data is ordinary relational data. Every table and column name
below is real — checked against the generated R5 map, not written from memory:

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

`r5` here is not a schema in the PostgreSQL sense — it is the name a separate
SQLite database file was `ATTACH`ed under (`M14.16`). `SqliteStore` does this
attaching for you on first use; if you open a raw `sqlite3` session on the
main file directly, attach it yourself first:

```sql
ATTACH DATABASE 'clinic-r5.sqlite' AS r5;
```

Two databases attached to the same connection can be joined freely in one
query — the storage model reserves that for a version boundary you cross on
purpose, not something the schema itself forbids. What genuinely cannot cross
an attached-database boundary is a **foreign key** (`REFERENCES` resolves
only within its own database), which is why `patient.id` and
`observation.subject_ref_id` are joined by a plain `ON`, with no declared FK
between them — exactly as in every other port, since a reference in FHIR is
not an integrity constraint (`M3.10`).

## `ords`: an index path, stored as text

`ords = '{1}'` addresses the first instance of a repeating element — `n.ords
= '{1}'` above picks each patient's first name. That much survives from the
PostgreSQL original unchanged.

**What does not survive: `ords[1] = 1` to match any descendant of the first
instance.** PostgreSQL stores `ords` as `smallint[]` and can subscript it;
this port stores the identical literal image (`{1,2}`, `{}`, `{-1,3}`) as
plain `TEXT`, because SQLite has no array type and the database never orders,
compares, or unnests `ords` anyway — see [The storage
model](storage-model.md). A `TEXT` column has no subscript operator. The
nearest equivalent is a prefix match on the text image itself:

```sql
-- Any descendant of the first instance of a repeating element
SELECT * FROM r5.patient_contact_telecom WHERE ords LIKE '{1,%';
```

This is a real behavioural regression from the PostgreSQL original, not a
cosmetic rename (`M14.9`), and it is the only place in the schema where array
subscripting mattered.

## Other tips

- Temporal comparisons belong on the `*_sort` columns (e.g.
  `birth_date_sort`); the lexical column (`birth_date`) preserves what the
  client sent, partial dates included. The same split exists for `decimal`
  columns — compare on `<col>_sort`, read from `<col>` — see [The storage
  model](storage-model.md#types).
- There is no `fhir-sqlite transform` command or any other CLI to print a
  resource's row layout — this crate has no binary (`C0.17`). To see what a
  resource actually produces, either query the tables directly, as above, or
  inspect the generated map in Rust: `RelMap::bundled("r5")?.resources["Patient"]`
  carries every table and column, and each column records the FHIRPath it was
  generated from (see [Architecture](architecture.md)).
- Write queries against one version's tables at a time. `r4` and `r5` name
  tables identically wherever the two specifications agree, but nothing
  enforces that identity across a query — a join across `r4.patient` and
  `r5.observation` would silently mix two unrelated numbering spaces for
  `id`.
