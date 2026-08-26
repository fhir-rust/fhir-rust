# Benchmarks

**Read this section before any number below.**

One port has been measured: `fhir-postgresql`, on a development machine, on
2026-07-24. The other five have not. Numbers derived from the *generator* —
schema scale and search-parameter compilation — are legitimately shared across
all six, because the generator is byte-identical in all of them (`X15.1`);
numbers derived from a *store* are not, and are not presented here for any port
that has not produced them.

This distinction is written down because the project got it wrong once, at
scale. Every non-PostgreSQL port's benchmark page used to carry
`fhir-postgresql`'s measured numbers with the engine name substituted —
including a live round-trip and a 6,146 resources/s bulk load for two ports that
had no store at all, and a gated benchmark command naming a test target that
existed in one port only. That is finding **F-64** (High, fixed 2026-08-03), and
the requirement it broke, `W16.10`, was already in the specification at the time:

> Measured numbers MUST name what measured them and when. A throughput figure
> inherited by substitution is not a measurement of the port that now carries
> it.

Per-port pages live at `fhir-<engine>/doc/benchmarks.md`. This page is the
repository-level summary.

## Measured — `fhir-postgresql`

**Apple Silicon development machine, PostgreSQL 18.4 local, release builds,
2026-07-24.** Working numbers for risk tracking, not tuned results, and **not
produced by hosted CI** — which under `C0.9` makes them weaker evidence than a
green pipeline.

### Schema install and scale

| Version | Resources | Tables | Data columns | Map asset (gz) |
| --- | --- | --- | --- | --- |
| R3 3.0.2 | 117 | 3,827 | 30,246 | 503 KB |
| R4 4.0.1 | 146 | 5,672 | 43,777 | 734 KB |
| R5 5.0.0 | 158 | 7,355 | 58,405 | 984 KB |

- Full R5 install — 7,355 tables and 9,168 indexes, of which 1,813 are generated
  search indexes: **5.8–9.5 s**, staged-schema install (`G2.5`). A naive single
  transaction exhausts `max_locks_per_transaction`; the staging-and-rename
  design avoids requiring any server configuration.
- Chunked `drop_schema` of the same: **~5 s**.

*The table above is generator-derived and holds for all six ports. The install
timings are PostgreSQL's alone.*

### Round-trip correctness and cost

- In-memory shred → reconstruct over all official specification examples:
  **7,399 / 7,399 lossless** across R3 (1,664), R4 (2,911), R5 (2,824), in
  **~5.6 s** in release mode.
- Live PostgreSQL `put` → `get` of the same corpus: **7,396 / 7,396 lossless**
  (three examples lack ids and are skipped) in **101 s**, including three full
  schema installs — roughly **13 ms per resource** for write + read +
  reconstruct, unindexed and before any batching of the read path.

### Bulk load and reads

Gated benchmark, `fhir-postgresql` only — this is the test target that does not
exist in the other five ports:

```sh
FHIR_POSTGRESQL_BENCH=100000 FHIR_POSTGRESQL_TEST_DB=… \
  cargo test --release -p fhir-postgresql-store --test bench -- --nocapture
```

- **Load: 100,000 resources** (50k `Patient` + 50k `Observation`) in **16.3 s** —
  **6,146 resources/s** — through full shredding and transactional `put` with
  history append, 12 concurrent workers over a 16-connection pool.
- **Read: 1.18 ms average** for a full multi-table reconstruction, over a
  500-read sample.
- **`EXPLAIN` audit:** canonical token, reference and date-range searches all
  plan index scans. The test fails on any sequential scan, which makes this the
  one performance property that is actually gated rather than reported.

### Search compilation

- R5: **1,870 of 1,972 SearchParameters compiled (94.8 %)**. Every uncompiled
  parameter records its reason in the map asset — composites, specials, and
  `exists()`-style expressions.

*Generator-derived; holds for all six ports.*

## Not measured

Stated as a list rather than left for a reader to infer:

- **Five of the six ports have no store benchmark at all** — sqlite, mysql,
  mariadb, mssql, oracle. None has a `bench.rs`. Building a shared harness is a
  recorded gap from F-64's fix, not an oversight discovered here.
- **The comparison this project's argument rests on has never been run.** The
  case for relational storage is that JSON storage makes writing easy and
  querying painful. Nothing here measures a normalized query against the *same
  data in a JSONB column in the same database on the same hardware*, which is
  the only measurement that tests the claim. Until it exists, "SQL that reads
  like the domain" is an argument from ergonomics, not from performance.
- **Scale beyond 100k resources.** The harness extends by environment variable;
  nobody has run it further.
- **Latency distribution under mixed read/write load**, and **search throughput
  under concurrency**. Averages are reported above; percentiles are not, and an
  average read latency is a weak number for a clinical workload.
- **Any number from hosted CI.** Every figure on this page comes from one
  developer machine.
- **`T11.5`'s regression gate** is unmet in five ports, since a regression gate
  needs a baseline to compare against.

## Methodology, if you want to reproduce or extend this

1. Bring up the pinned engine: `cd fhir-postgresql && scripts/db.sh up`, then
   `scripts/db.sh corpus` to lay out the specification example corpus.
2. Build in release mode. Debug numbers for this workload are not merely slower;
   they are a different shape, because shredding is allocation-heavy.
3. State the machine, the engine version, and the date, in that order, beside
   any number you publish — `W16.10` requires it, and F-64 is what happens when
   it is not done.
4. A number that came from a port other than the one it is written under is a
   defect, not an estimate.
