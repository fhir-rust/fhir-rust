# Benchmarks

**This page was `fhir-postgresql`'s, with the crate name substituted in two of
five places** (audit **F-64**). The install-time and load-benchmark numbers
below were `fhir-postgresql`'s own measurements, presented as this port's —
including a `bench.rs` gated benchmark this port does not have, and a claim of
"staged-schema install" this port does not do (**F-27** already found the
second half of that same substitution, in `tasks.md`).

What is corrected below is stated as corrected; what is genuinely shared — the
schema scale and search-compilation figures, which come from the generator
that is byte-identical across all six ports (`X15.1`) — is kept.

## Schema scale (risk R1)

| Version | Resources | Tables | Data columns | Map asset (gz) |
| --- | --- | --- | --- | --- |
| R3 3.0.2 | 117 | 3,827 | 30,246 | 503 KB |
| R4 4.0.1 | 146 | 5,672 | 43,777 | 734 KB |
| R5 5.0.0 | 158 | 7,355 | 58,405 | 984 KB |

- Install time for full R5 (7,355 tables + 9,168 indexes, of which 1,813 are
  generated search indexes) has **not been measured for this port**. The
  5.8–9.5s figure this page carried was `fhir-postgresql`'s, install strategy
  and all: this port installs via a direct statement-by-statement install, not
  atomic — MariaDB has no transactional DDL, so the staged-schema-then-rename
  dance PostgreSQL uses has no equivalent (corrected under **F-27**).
  `max_locks_per_transaction` is a PostgreSQL setting this port's engine does
  not have.

## Search compilation (M3)

- R5: **1,870 of 1,972 SearchParameters compiled (94.8%)**; every
  uncompiled parameter records its reason in the map asset (composites,
  specials, exists()-style expressions).

## Round-trip correctness (R4.2)

- In-memory shred→reconstruct, all official spec examples
  (examples-json.zip): **7,399/7,399 lossless** across R3 (1,664),
  R4 (2,911), R5 (2,824). Verified this session (**F-64**); wall-clock time
  depends on the machine running it and is not restated here as a number
  belonging to a specific release build.
- **No live corpus round-trip benchmark exists for this port.** The 101s/13ms
  figure this page carried was `fhir-postgresql`'s own `live.rs` result.
  `fhir-mariadb` has no equivalent harness — its live suite tests correctness
  (`concurrency.rs`, `redaction.rs`, `upgrade.rs`), not throughput.

## Bulk load, reads, and index audit (T15/T28)

**There is no `bench.rs` in this port.** The gated-benchmark invocation and
every number in this section — 16.3s load, 6,146 resources/s, 1.18ms reads —
were `fhir-postgresql`'s, with `FHIR_POSTGRESQL_BENCH` substituted for
`FHIR_MARIADB_BENCH`. `cargo test --release -p fhir-mariadb-store --test bench`
does not exist and would not run.

An EXPLAIN-style index-usage audit is real work this port has not done.

## Not yet measured

Everything a `bench.rs` would answer: load throughput, read latency, and
index-usage audit under this port's own engine. Building that harness is real
work, tracked as a gap rather than assumed done.
