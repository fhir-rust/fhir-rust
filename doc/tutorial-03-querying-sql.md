# Tutorial 3 — querying with SQL

The point of relational storage is that you can use SQL. Not JSON path
expressions dressed up as SQL — actual columns, actual joins, actual statistics
for the planner.

Assumes [tutorial 2](tutorial-02-storage-model.md).

## The shape you are querying

```
patient                    one row per resource
patient_name               one row per name             (rid, ords)
patient_name_given         one row per given name       (rid, ords)
patient_telecom            one row per telecom          (rid, ords)
patient_ext                one row per extension leaf   (rid, path, ords, …)
patient_history            one row per version
```

Every child table has `rid` (the resource id) and `ords` (the position path).
Join on `rid`; filter on `ords` when you want a specific instance.

## Basics

```sql
-- Everyone born in 1974, using the derived sort column
SELECT id, birth_date
  FROM patient
 WHERE birth_date_sort >= DATE '1974-01-01'
   AND birth_date_sort <  DATE '1975-01-01';
```

Note `birth_date_sort`, not `birth_date`. The stored column holds the lexical
form — possibly `"1974"`, possibly `"1974-12-25"` — and the derived column is
the typed, indexed one. Filter on the derived; display the stored.

```sql
-- Primary family names
SELECT p.id, n.family
  FROM patient p
  JOIN patient_name n ON n.rid = p.id AND n.ords = '{1}'
 WHERE p.active;
```

`ords = '{1}'` is "the first name". This is the idiom you will use most.

## Joining across resources

References are already parsed into columns, so this is an ordinary join:

```sql
SELECT n.family, count(o.id) AS observations
  FROM patient p
  JOIN patient_name n
    ON n.rid = p.id AND n.ords = '{1}'
  LEFT JOIN observation o
    ON o.subject_ref_type = 'Patient'
   AND o.subject_ref_id   = p.id
 GROUP BY n.family
 ORDER BY observations DESC
 LIMIT 20;
```

There is no foreign key behind that join (`M3.10`) — FHIR permits dangling
references, so enforcing one would make load order matter and reject real data.
The join still uses the index on `(ref_type, ref_id)` that the generator emits
for every reference column (`P6.4`).

## Nested repeating elements

```sql
-- All given names of every patient's first name, in order
SELECT g.rid, g.ords, g.value
  FROM patient_name_given g
 WHERE g.ords LIKE '{1,%'
 ORDER BY g.rid, g.ords;
```

Read `{1,2}` as "second given name of the first name". The path is the ordering
information, which is why array order survives a table where row order does not.

## The one idiom that differs per engine

PostgreSQL stores `ords` as `smallint[]`, so it supports subscripting:

```sql
-- PostgreSQL only
SELECT * FROM patient_name_given WHERE ords[1] = 1;
```

Every other port stores the array literal as `TEXT` (`M3.4b`), where the
equality form works verbatim but the subscript does not. The portable
equivalent:

```sql
-- everywhere
SELECT * FROM patient_name_given WHERE ords LIKE '{1,%';
```

That is a prefix match on the text image rather than a typed subscript, and it
is the only place in the whole schema where the difference is visible to a query
author (`M14.9` in the SQLite annex). If you want your SQL to run on all six
engines, use the `LIKE` form.

## Case- and accent-insensitive matching

Do not write `LOWER()` or `UNACCENT()`. Use the `_norm` columns:

```sql
SELECT rid FROM patient_name WHERE family_norm = 'aero';   -- finds 'Ærø'
```

The value in `family_norm` was folded in Rust at write time (`P6.6`), and the
fold is the system's single definition of "the same string". Rolling your own in
SQL gives you a second definition that must agree with the first for every
codepoint in Unicode, and it will not.

For a prefix search, use a **range**, not `LIKE`:

```sql
SELECT rid FROM patient_name
 WHERE family_norm >= 'smit' AND family_norm < 'smiu';
```

`LIKE $1 || '%'` looks equivalent and is not. A planner extracts a prefix from a
*constant* pattern only, so a `LIKE` against a bound parameter degrades to a
sequential scan in the generic plan — while looking perfectly indexed in any
hand-run `EXPLAIN` with a literal (`P6.6a`).

## Extensions

```sql
-- Everyone carrying the US Core race extension
SELECT DISTINCT rid
  FROM patient_ext
 WHERE url = 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-race';

-- Its text value
SELECT rid, v_text
  FROM patient_ext
 WHERE url  = 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-race'
   AND leaf = 'extension.1.valueString';
```

`url` is denormalized onto every leaf row precisely so this query is a simple
one (`M3.11`). `leaf` addresses the scalar inside the extension's content;
all-digit segments are 0-based array indexes.

Numeric extension values are in `v_num` (queryable) and `v_text` (lexical):

```sql
SELECT rid, v_num FROM observation_ext
 WHERE url = '…/some-numeric-ext' AND v_kind = 'n' AND v_num > 100;
```

## History

```sql
SELECT version_id, op, last_updated, actor, actor_source, reason
  FROM patient_history
 WHERE id = 'example'
 ORDER BY version_id;
```

`op` is `C`, `U`, or `D`. The `resource` column holds the whole version as text.
The audit envelope columns (`actor`, `actor_source`, `client`, `request_id`,
`reason`) were written by the same statement that appended the row, in the same
transaction as the data change (`M3.15`) — an audit record that can be lost
independently of the change it describes is not an audit record.

**Do not `UPDATE` or `DELETE` these tables.** A trigger raises an exception
(`M3.17`), and it is there so that escaping append-only is a deliberate DBA act
rather than an application bug.

## Reads must be one snapshot

If you are reconstructing a resource by hand rather than through `get`, wrap
the reads in one transaction at an isolation level that gives a stable snapshot
(`R4.5`):

```sql
BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY;   -- PostgreSQL
  SELECT … FROM patient           WHERE id  = 'example';
  SELECT … FROM patient_name      WHERE rid = 'example';
  SELECT … FROM patient_name_given WHERE rid = 'example';
COMMIT;
```

Issued as independent statements, a concurrent write between them reconstructs a
resource that **never existed** — base columns from one version, child rows from
the next. This is a correctness requirement, not a tuning knob, and `READ
COMMITTED` (the default on several engines) is exactly the failure mode.

Names differ: `REPEATABLE READ READ ONLY` on PostgreSQL, `REPEATABLE READ` on
MySQL and MariaDB, `SNAPSHOT` on SQL Server (which must be enabled on the
database), `BEGIN DEFERRED` under WAL on SQLite.

## Performance notes

The generator emits indexes for every base-table search column, every child
table's `(rid, ords)`, every reference `(ref_type, ref_id)` pair, and every
token `(system, code)` pair (`P6.4`). If your query is slow, check whether it is
filtering on a `_sort` or `_norm` column — the typed and folded ones are the
indexed ones.

Where an engine cannot index a column as bound — SQL Server's `NVARCHAR(MAX)`
token columns, for instance — the port is required to narrow the type or add an
indexable derived column rather than silently drop the index and scan
(`P6.4a`). SQL Server's is an open item.

## Next

- [Tutorial 4 — FHIR search](tutorial-04-search.md), which compiles to exactly
  this kind of SQL, generated
- [Examples](examples.md) for more recipes
