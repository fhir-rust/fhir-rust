# 4. Operations

## Configuration

- **SV4.1** **Every environment's configuration MUST parse.** Loco selects
  configuration by `LOCO_ENV` and does not merge over a default, so an absent or
  malformed file is a boot failure rather than a fallback.

  `config/production.yaml` was an **empty file** until 2026-08-03, so
  `LOCO_ENV=production` refused to start:

  ```text
  Error: YAMLFile(Error("missing field `logger`"), "config/production.yaml")
  ```

  The one environment this service exists to run in was the one it could not run
  in, and nothing noticed because the test suite runs as `test` and the
  developer loop as `development` (**F-59**). A test MUST assert that each
  environment's configuration loads; `tests/config.rs` does.

## Limits

- **SV4.2** Resource limits MUST be enforced at the edge, not only at the store:
  a bounded request body, a per-request timeout, a bounded concurrency limit,
  and a maximum in-flight request count, shedding as `503` with `Retry-After`
  rather than queueing. Restates `O10.8`.

  **Partly unmet.** Production sets a `32mb` body limit and a `30s` timeout.
  **Loco 1.0.1 exposes neither a concurrency limit nor an in-flight cap**, so
  those two are not configured — a framework limit, not an oversight, and it is
  recorded here rather than assumed covered.

- **SV4.3** Metrics and health endpoints MUST be servable on a **separate bind
  address** from the FHIR API, so operational endpoints are not exposed to the
  same network as clinical data. Latency MUST be reported as a histogram, not a
  running total, so p99 is answerable. Restates `O10.9`.

  **Met** (served since 2026-08-07; until then there was one listener and no
  `/metrics` at all — **F-58**). `FHIR_LOCO_ADMIN_BIND` names the admin
  socket; unset, no second listener exists, which is the same
  deliberate-exposure posture as `SV4.4`. It serves `/health` (liveness),
  `/ready` (readiness — a mounted store; the two are separate endpoints
  because liveness green over a dead store was this crate's original boot
  bug), and `/metrics` (Prometheus text: request counts by status class, and
  request duration as a fixed-bucket histogram). The admin router carries no
  FHIR route and MUST never grow one: nothing on this listener reads a
  resource.

## Binding

- **SV4.4** The default bind address MUST be loopback. Exposing a FHIR API
  carrying PHI on every interface MUST be a deliberate act by whoever deploys
  it, not a default inherited from a configuration file they did not read.

  This is the only thing standing between the absence of `SV3.11` (no listener
  TLS) and PHI on the wire, which is why it is a MUST rather than a default.

## Logging

- **SV4.5** Production logging MUST NOT print backtraces. A backtrace can carry
  file paths and argument values, and this process handles PHI. The audit chain
  and the access log are the intended record (`PR12.5`).

- **SV4.6** Production logs MUST be machine-parseable (JSON). A log a shipper
  cannot parse is a log nobody reads.

- **SV4.7** A panic MUST become a `500`, not a dropped connection. A client that
  cannot distinguish "server fault" from "network fault" may retry a write it
  should not.

## Deliberate omissions

- **SV4.8** CORS MUST NOT be enabled by default. This API is not for browsers,
  and enabling it invites a credentialed cross-origin request against PHI.

- **SV4.9** Response compression MUST NOT be enabled by default. Compressing
  attacker-influenced responses is the BREACH pattern, and it buys little on
  JSON of this size.

  Both omissions MUST be recorded **where the configuration is**, not only here.
  A disabled feature with no comment reads as an oversight, and the next person
  tidying the file will enable it.

## Supply chain

- **SV4.10** This crate is subject to the same release gates as the ports:
  `cargo deny` and `cargo audit` (`O10.10`), and a published version matching
  its source (`O10.11`).

---

Part of the [fhir-loco specification](index.md).
