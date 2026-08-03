# Benchmarks

**This page was `fhir-postgresql`'s, with the crate name substituted in two of
five places** (audit **F-64**). Two of its four sections describe a live store
this port does not have and never had — `fhir-mssql` has no store and no
driver (`C0.8`) — via a `bench.rs` test and a load benchmark that are
`fhir-postgresql`'s alone.

What is genuinely shared — the schema scale and search-compilation figures,
which come from the generator that is byte-identical across all six ports
(`X15.1`) — is kept.

## Schema scale (risk R1)

| Version | Resources | Tables | Data columns | Map asset (gz) |
| --- | --- | --- | --- | --- |
| R3 3.0.2 | 117 | 3,827 | 30,246 | 503 KB |
| R4 4.0.1 | 146 | 5,672 | 43,777 | 734 KB |
| R5 5.0.0 | 158 | 7,355 | 58,405 | 984 KB |

The DDL emitter is real (**F-08**): the generated schema installs on a live
SQL Server 2022 (`tests/mssql_ddl.rs`) — 131 statements, 102 tables, 4
triggers for a two-resource sample, and 0 errors. Full-R5 install time has
**not been measured**: there is no store to time the operation from a
caller's perspective, only DDL application.

## Search compilation (M3)

- R5: **1,870 of 1,972 SearchParameters compiled (94.8%)**; every
  uncompiled parameter records its reason in the map asset (composites,
  specials, exists()-style expressions). Shared logic (`gen/`), so this
  figure is identical in every port.

## Round-trip correctness (R4.2)

- In-memory shred→reconstruct, all official spec examples
  (examples-json.zip): **7,399/7,399 lossless** across R3 (1,664),
  R4 (2,911), R5 (2,824). Shared shred/reconstruct engine (`X15.1`); this
  needs no database and has been run.
- **There is no live put→get round trip for this port.** The 101s/13ms figure
  this page carried, and the "Live PostgreSQL" label on it, were
  `fhir-postgresql`'s own `live.rs` result. `fhir-mssql` has no store to run
  one against.

## Bulk load, reads, and index audit

**None of this applies to this port.** The gated-benchmark invocation and
every number the section here used to carry — 16.3s load, 6,146 resources/s,
1.18ms reads, `FHIR_MSSQL_BENCH=100000 … --test bench` — were
`fhir-postgresql`'s, with the crate name swapped into an invocation that does
not exist. There is no store to load, and no `bench.rs` in this workspace.

## Not yet measured

Everything above "install a schema on a live server" (**F-08**) and "in-memory
round-trip" (shared). A store is the prerequisite for the rest, and there is
none — see [`audit.md`](../spec/databases/audit.md) **F-51**.
