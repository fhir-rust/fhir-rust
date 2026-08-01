# 4. Shredding and reconstruction

- **R4.1** Shredding (JSON → rows) and reconstruction (rows → JSON) are driven
  by the generated relational map through one generic engine; no per-resource
  handwritten code.
- **R4.2** Round-trip MUST be lossless: for any valid resource,
  `reconstruct(shred(r))` is semantically identical JSON — same values
  (including decimal precision and partial dates), same array order, key order
  not significant. This invariant is enforced by property tests over spec
  examples and generated resources.

  This is the requirement the whole design exists to satisfy, and the one a port
  may never trade away (`C0.13`). A relational schema that cannot give the
  resource back is a lossy archive with good query performance.

- **R4.3** Unknown elements (not in the version's spec) MUST be rejected with an
  error naming the path — never silently dropped.
- **R4.4** A resource write (shred + delete-old-rows + insert) MUST be a single
  transaction.
- **R4.5** A resource **read** MUST likewise see a single snapshot. A read
  touches one base table and many child tables; issued as independent
  statements, a concurrent write between them would reconstruct a resource that
  never existed — base columns from one version, child rows from the next. Every
  multi-statement read (`get`, `export`, search result materialization) MUST
  therefore run inside one transaction at an isolation level that gives it a
  stable snapshot, read-only where the engine can express it. This is a
  correctness requirement, not a tuning knob.

  The isolation level that delivers it is a dialect concern (`X15.7`) and the
  names differ: PostgreSQL's `REPEATABLE READ READ ONLY`, MySQL's and MariaDB's
  `REPEATABLE READ`, SQL Server's `SNAPSHOT` (which must be enabled on the
  database), SQLite's `BEGIN DEFERRED` under WAL. A port MUST name the mechanism
  in its annex, and MUST NOT settle for an engine default that provides less —
  `READ COMMITTED` re-reads on every statement and is exactly the failure this
  requirement describes.

- **R4.6** Resource ids MUST satisfy the FHIR `id` production
  (`[A-Za-z0-9\-\.]{1,64}`) wherever they enter the system. An id that does not
  is an error, never a stored row.
- **R4.7** Reconstruction MUST **audit row consumption**: every row fetched for
  a resource must be consumed exactly once, and a residue MUST be reported as an
  integrity error naming the count, never silently discarded.

  This is what makes a lossy storage binding loud rather than quiet. A port that
  encodes `ords` or a decimal in a way that collides two distinct values will
  produce a resource that is missing something, and the residue is the only
  signal that says so before a clinician notices.

---

Part of the [fhir-databases specification](index.md).
