# Introduction

fhir-mariadb stores [FHIR](https://hl7.org/fhir/) resources in PostgreSQL 18 as
**real relational tables** — typed columns, child tables, primary and
foreign keys — not JSON or JSONB blobs, and serves them back through the
standard FHIR RESTful API.

Two claims define the project, and both are enforced by tests:

1. **Losslessness.** Any valid FHIR resource that goes in comes back
   semantically identical — array order, decimal precision, partial dates,
   extensions, and all. The entire official example corpus for R3, R4, and
   R5 (7,399 resources) round-trips through live PostgreSQL, and ten
   thousand generated property-test cases besides.
2. **Relational honesty.** Live data lives in typed columns you can query,
   join, index, and constrain with ordinary SQL. The only JSONB in the
   system holds write-once history snapshots and anonymous contained
   resources — never data the schema claims to model.

The trade fhir-mariadb makes is generation over convention: the schema (7,355
tables for R5) is generated from the FHIR specification itself, and a
single generic engine shreds and reconstructs every resource type by
walking the generated map. Nothing about a specific resource is
hand-written.

## Why not JSONB?

JSONB storage makes writing FHIR trivial and everything downstream harder:
queries become path-spelunking, the planner sees no per-column statistics,
value typing is enforced nowhere, and analytical SQL reads like an
apology. For a clinical system the important operations are reads,
searches, joins, and audits — exactly what normalized storage is good at.

## Status

Functional end to end and pre-release. See `tasks.md` in the repository
for the milestone ledger and `doc/benchmarks.md` for measured numbers
(6,146 resources/s bulk load; 1.18 ms average reconstruction reads;
index-verified searches at 100k resources).
