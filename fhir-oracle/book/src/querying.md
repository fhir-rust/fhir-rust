# Querying with SQL

Loaded FHIR® data is ordinary relational data — connect as one of the version
users (`R5`, say) and query its own schema directly:

```sql
-- Patients and their observation counts
SELECT n."family", COUNT(o."id") AS observations
  FROM "R5"."patient" p
  JOIN "R5"."patient_name" n ON n."rid" = p."id"
  LEFT JOIN "R5"."observation" o
    ON o."subject_ref_type" = 'Patient' AND o."subject_ref_id" = p."id"
 GROUP BY n."family"
 ORDER BY observations DESC;

-- Blood-pressure observations by LOINC code, with values
SELECT o."id", o."value_quantity_value", o."value_quantity_code"
  FROM "R5"."observation" o
  JOIN "R5"."observation_code_coding" c
    ON c."rid" = o."id" AND c."system" = 'http://loinc.org' AND c."code" = '85354-9';

-- The first given name of the second recorded family name — see below for
-- what "first" and "second" mean on this engine
SELECT n."family", g."value"
  FROM "R5"."patient_name" n
  JOIN "R5"."patient_name_given" g ON g."rid" = n."rid"
 WHERE n."rid" = 'example';
```

## `ords`: a `RAW`, not an array

Every other port stores the ordinal path as a native array
(`smallint[]` on PostgreSQL, for instance). Oracle has no array column type,
so `ords` here is `RAW(255)` holding the same shared text image, encoded —
not a value you compare with `=` against a literal you typed by hand. To
address "the first instance of a repeating element" from SQL, join through
`rid` and use `MIN`/row-limiting on the natural order rather than filtering
`ords` directly:

```sql
-- The first given name, however many a patient has
SELECT g."value" FROM "R5"."patient_name_given" g
 WHERE g."rid" = 'example'
 ORDER BY g."ords" FETCH FIRST 1 ROW ONLY;
```

If you need to decode or construct an `ords` value directly, do it in Rust
against `fhir_oracle_map::shred`/`reconstruct` — the encoding is shared,
engine-independent code, not something to reverse-engineer per query.

## Searching unbounded text (extensions, long strings) from raw SQL

Columns wide enough to need a `CLOB` (`Text`-typed columns generally, and the
extension tables' `path`/`url`/`leaf`/`v_text`/`v_num` columns specifically)
cannot be filtered with `=` at all on this engine (`ORA-22848`). Two
generated companion columns exist for exactly this — see [The storage
model](storage-model.md):

```sql
-- Prefix search via the bounded adjunct (first 450 characters, case- and
-- accent-folded the same way the Rust search compiler folds it)
SELECT "rid" FROM "R5"."patient_ext"
 WHERE "url_idx" LIKE 'http://hl7.org/fhir/StructureDefinition/patient-birth%';
```

Equality search goes through the SHA-256 digest adjunct (`<col>_h`,
`RAW(32)`), which you cannot hand-write in plain SQL — it has to be computed
the same way the application computes it (`fold::digest` in the shared
core), or the comparison will simply never match. This is the one place
where "just write the SQL yourself" stops being practical, and the search
compiler (see [Search](search.md)) exists precisely so nobody has to.

## Tips

- Temporal comparisons belong on the derived `*_sort` columns
  (`DATE`/`TIMESTAMP(6)`); the lexical column preserves what the client sent
  verbatim, including partial precision (`"2026-07"`).
- Write queries against one version user at a time; `R4` and `R5` name
  tables identically where the specs agree, but nothing joins across them —
  they are separate Oracle users, and Oracle does not offer cross-schema
  joins any more conveniently than cross-database ones.
- The generated map assets carry a FHIR-path annotation for every table and
  column — reading `fhir_oracle_map::model::RelMap` in a Rust REPL or test is
  the fastest way to learn a table's layout without guessing from SQL alone.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
