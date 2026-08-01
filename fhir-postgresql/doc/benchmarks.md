# Benchmarks

Measured on the development machine (Apple Silicon, PostgreSQL 18.4 local,
release builds), 2026-07-24. These are working numbers for the plan's risk
tracking, not tuned results; the regression-gate methodology lands with task
T28.

## Schema scale (risk R1)

| Version | Resources | Tables | Data columns | Map asset (gz) |
| --- | --- | --- | --- | --- |
| R3 3.0.2 | 117 | 3,827 | 30,246 | 503 KB |
| R4 4.0.1 | 146 | 5,672 | 43,777 | 734 KB |
| R5 5.0.0 | 158 | 7,355 | 58,405 | 984 KB |

- `fhir-postgresql init` for full R5 (7,355 tables + 9,168 indexes, of which 1,813
  are generated search indexes): **5.8–9.5 s**, staged-schema install (see
  spec G2.5). A naive single transaction exhausts
  `max_locks_per_transaction`; the staging + rename design avoids any
  server configuration requirement.
- Chunked `drop_schema` of the same: ~5 s.

## Search compilation (M3)

- R5: **1,870 of 1,972 SearchParameters compiled (94.8%)**; every
  uncompiled parameter records its reason in the map asset (composites,
  specials, exists()-style expressions).

## Round-trip correctness (R4.2)

- In-memory shred→reconstruct, all official spec examples
  (examples-json.zip): **7,399/7,399 lossless** across R3 (1,664),
  R4 (2,911), R5 (2,824). ~5.6 s total in release mode.
- Live PostgreSQL put→get round trip of the same corpus:
  **7,396/7,396 lossless** (3 examples lack ids and are skipped),
  **101 s** total including three full schema installs — roughly
  **13 ms per resource** for write + read + reconstruct, unindexed and
  before any batching of the read path.

## Bulk load, reads, and index audit (T15/T28)

Gated benchmark: `FHIR_POSTGRESQL_BENCH=100000 FHIR_POSTGRESQL_TEST_DB=… cargo test
--release -p fhir-postgresql-store --test bench -- --nocapture`.

- **Load: 100,000 resources (50k Patient + 50k Observation) in 16.3 s —
  6,146 resources/s** through full shredding, transactional put with
  history append, 12 concurrent workers over a 16-connection pool.
- **Read: 1.18 ms average** for a full multi-table reconstruction
  (500-read sample over the loaded data).
- **EXPLAIN audit**: canonical token (child-table identifier), reference
  (base-table subject), and date-range searches all plan index scans; the
  test fails on any sequential scan.

## Not yet measured

Million-resource scale (the 100k harness extends by env var), latency
distribution under mixed read/write load, and search throughput under
concurrency.
