# 10. Operations

Requirements marked **[service]** describe a service layer that does not exist
in this monorepo (`C0.17`). They are retained because the obligations bind
whatever service is eventually built, and a port is not non-conformant for
lacking them.

- **O10.1** **[service]** A service exposes liveness and readiness endpoints off
  the FHIR base paths, and Prometheus metrics on a separate configurable port
  (request counts/latencies by route, pool stats, per-resource-type row counts).
- **O10.2** Structured logging via `tracing` (JSON in production); every
  operation gets a correlation id. **Logs MUST NOT contain resource content
  (PHI) at default level.** This half is in force for the library: the store
  logs, and a store that logs a patient's name at `debug` has put PHI in a
  system with different retention, different access control, and different
  export paths from the database. `T11.7` is its test.
- **O10.3** **[service]** Connection pooling; pool exhaustion returns 503 with
  `Retry-After`, never queues unboundedly. Statement timeouts are set per pool
  connection. The statement-timeout half is in force wherever a port opens
  connections: an unbounded statement is an unbounded hold on a snapshot
  (`R4.5`) and on a row lock (`H5.4`).
- **O10.4** Schema migrations: a metadata table records artifact versions;
  `init --upgrade` applies generated migration DDL between artifact versions
  transactionally where possible, and refuses destructive changes without an
  explicit acknowledgement. Every release documents its migration.
- **O10.4a** A migration that changes **stored derived values** MUST provide a
  backfill, and MUST NOT be deployable without one. The folded `_norm` columns
  (`P6.6`) are the live case: shipping a corrected fold against a database
  written under the old one leaves stored values that match neither the old
  spelling nor the new, so searches that worked before the fix stop working
  after it. Deploying such a change without backfilling is worse than not
  fixing the bug.
- **O10.5** **[service]** TLS: production deployments terminate TLS at a
  fronting proxy, or in-process behind a `tls` feature (rustls). A service binds
  localhost by default; binding non-loopback requires explicit acknowledgement.
  Authentication and authorization (SMART on FHIR, OAuth) are explicitly out of
  scope for a service core and delegated to the deployment perimeter; the spec
  requires documenting this boundary.
- **O10.6** Backup and restore is the engine's own (`pg_dump`/PITR, a copied
  SQLite file, `mysqldump`/binlog, `BACKUP DATABASE`, RMAN). The documentation
  MUST state the invariant that a consistent snapshot is always a valid store,
  and MUST name the engine's mechanism rather than another engine's.
- **O10.7** **The database connection carries PHI and MUST be encrypted**,
  except where the connection does not leave the process. A port connecting over
  a network MUST support the engine's TLS modes, MUST default to verifying the
  server certificate, and MUST document which mode is the production setting.
  Starting a networked service over an unencrypted database connection MUST
  refuse without an explicit override: the two halves of the trust boundary are
  decided together or not at all.

  An embedded engine (SQLite) has no connection to encrypt, and its annex MUST
  say so rather than leave a TLS requirement standing unmet. Its equivalent
  obligation is at-rest: the database file carries PHI and its permissions and
  storage encryption are the deployment's to set.

- **O10.8** **[service]** Resource limits are enforced at the edge, not only at
  the pool: a per-request timeout, a bounded concurrency limit, and a maximum
  in-flight request count, all configurable, all shedding load as 503 with
  `Retry-After` rather than queueing. Pool size is configurable, not a
  compiled-in constant.
- **O10.9** **[service]** Metrics and health endpoints MUST be servable on a
  separate bind address from the FHIR API, so operational endpoints are not
  exposed to the same network as clinical data. Latency MUST be reported as a
  histogram, not a running total, so p99 is answerable.
- **O10.10** Releases ship supply-chain evidence: `cargo deny` (advisories,
  licenses, bans) and `cargo audit` in CI, a CycloneDX SBOM per release
  artifact, and checksums for every published binary. This is the IEC 62304 /
  FDA cybersecurity expectation for a component handling clinical data, and it
  is cheap to keep green from the start.
- **O10.11** A published version MUST match the source that claims it. A
  crates.io version is immutable, so a tree carrying an already-published
  version number MUST be byte-identical to what was published, and CI MUST fail
  otherwise.

  Without the check the divergence is invisible: every local build resolves the
  path dependency and never fetches the registry copy, so the tree stays green
  while the artifact someone downloads is different code. It surfaces only when
  a third party packages a dependent, as an error about code they did not write.
  For a component handling clinical data, "the released artifact is the reviewed
  source" is the claim the whole audit trail rests on — `O10.10`'s SBOM
  describes the artifact, and it is worth nothing if the artifact is not the
  source.

- **O10.12** A port's local development script and its CI pipeline MUST
  provision the **engine that port targets**, at the version its annex declares.
  A pipeline that starts a substitute engine produces green builds and no
  evidence (`C0.10`), and is worse than having no live gate, because the summary
  says the gate passed. Originating defect **F-06**, fixed; the per-family
  workflows were also inert in the monorepo until consolidated to the root
  (**F-49**, 2026-08-06) — the [conformance matrix](conformance-matrix.md)
  `O10.12` row records the current state.

---

Part of the [fhir-databases specification](index.md).
