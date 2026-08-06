# Benchmarks

**This page was `fhir-postgresql`'s, with the crate name substituted in two of
five places** (audit **F-64**). Two of its four sections described a live
store this port did not have when they were written; `fhir-oracle` has had a
real store since 2026-08-04 (**F-68**, live-verified against
`gvenzl/oracle-free`), but **no benchmark has been run against it** — the
`bench.rs` test and load benchmark remain `fhir-postgresql`'s alone.

What is genuinely shared — the schema scale and search-compilation figures,
which come from the generator that is byte-identical across all six ports
(`X15.1`) — is kept.

## Schema scale (risk R1)

| Version | Resources | Tables | Data columns | Map asset (gz) |
| --- | --- | --- | --- | --- |
| R3 3.0.2 | 117 | 3,827 | 30,246 | 503 KB |
| R4 4.0.1 | 146 | 5,672 | 43,777 | 734 KB |
| R5 5.0.0 | 158 | 7,355 | 58,405 | 984 KB |

The DDL emitter is real (**F-08**): the full R5 schema — 158 resources, 9,636
statements — installed on Oracle AI Database 26ai Free with **0 invalid
objects**: 7,358 tables (the 7,355 above, plus 3 fixed schema-wide tables:
`fhir_oracle_meta`, the access log, the countersign table), 9,479 indexes, 158
triggers, 21,540 check constraints, 7,039 foreign keys. That install has not
been timed — it was verified for correctness, not for speed, and no timing
harness has been pointed at the store (**F-68**) yet.

## Search compilation (M3)

- R5: **92.4% of SearchParameters compile** since **F-38** removed 51
  compilations that silently dropped a `where()` value restriction (this
  page previously carried the pre-F-38 94.8%); every uncompiled parameter
  records its reason in the map asset (composites, specials, exists()-style
  expressions). Shared logic (`gen/`), so this figure is identical in every
  port.

## Round-trip correctness (R4.2)

- In-memory shred→reconstruct, all official spec examples
  (examples-json.zip): **7,399/7,399 lossless** across R3 (1,664),
  R4 (2,911), R5 (2,824). Shared shred/reconstruct engine (`X15.1`); this
  needs no database and has been run.
- **There is no timed live put→get round trip for this port.** The 101s/13ms
  figure this page carried, and the "Live PostgreSQL" label on it, were
  `fhir-postgresql`'s own `live.rs` result. `fhir-oracle`'s store does
  round-trip live (`tests/oracle_store.rs`, **F-68**) but nothing has timed it.

## Bulk load, reads, and index audit

**None of this applies to this port.** The gated-benchmark invocation and
every number the section here used to carry — 16.3s load, 6,146 resources/s,
1.18ms reads, `FHIR_ORACLE_BENCH=100000 … --test bench` — were
`fhir-postgresql`'s, with the crate name swapped into an invocation that does
not exist. There is no `bench.rs` in this workspace; the store (**F-68**) has
never been load-benchmarked.

## Not yet measured

Everything above "install a schema on a live server" (**F-08**) and "in-memory
round-trip" (shared). The store exists now (**F-68**) — what is missing is a
benchmark harness pointed at it, a recorded gap under **F-64** (see
[`audit.md`](../../spec/databases/audit.md)).
