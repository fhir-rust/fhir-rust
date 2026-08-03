# Architecture

Five crates:

- **fhir-oracle-map** — the relational map model (a compact, serialized
  description of every table, column, and element) and the generic
  engine: `shred` (JSON → rows) and `reconstruct` (rows → JSON), plus the
  DDL emitter. Reconstruction audits row consumption — every stored row
  must be used exactly once, so schema drift or corruption surfaces as an
  integrity error instead of silent data loss.
- **fhir-oracle-gen** — reads a FHIR specification package
  (StructureDefinitions + SearchParameters) and builds the map:
  identifier fitting under a 63-byte budget (below every supported
  engine's own limit, so one map serves all six), width-based
  force-splitting, cycle detection (type cycles spill; contentReference
  recursion shares tables via ordinal sign lanes), and the search
  compiler that resolves FHIRPath expressions by walking the map tree.
- **fhir-oracle-store** — **not written**; this port has no driver and no store. Transactional writes with
  history append, optimistic concurrency, multi-op transactions,
  pipelined multi-table reads, search execution, install/upgrade. All
  values would bind as text, preserving the
  engine's lexical-fidelity guarantees.
- **fhir-oracle-server** — axum. The FHIR RESTful API, bundle processing,
  generated CapabilityStatements, request ids, metrics.
- **fhir-oracle** — the CLI binary tying it together.

The decisive design choice is **metadata over codegen**: rather than
generating Rust for 3 versions × ~150 resource types, the generator
emits data (the map) and one engine interprets it. The engine is a few
thousand lines, tested once, correct for every resource type — and the
map doubles as documentation, carrying the FHIR path of every column.

Design decisions D1–D14, risks, and milestones live in `plan.md`; the
normative behaviour is `spec/index.md`.
