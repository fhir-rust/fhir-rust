# Tasks — fhir-loco

An executable list. The crate's own [spec](spec/index.md) (`SV` ids,
specified 2026-08-03) is normative for behaviour; the
[conformance matrix](../spec/databases/conformance-matrix.md) governs the
ports this service sits over.

## Done

- [x] **FHIR REST CRUD over `fhir-sqlite`.** read/vread/create/update/delete/
  search/instance-history at `/{version}/{rtype}…`
  (`src/controllers/fhir.rs`), verified end to end by 10 request tests
  (`tests/requests/`), including `create_read_update_delete_round_trip`,
  `history_shows_every_version_including_the_deletion`, and
  `search_returns_a_bundle_and_respects_paging`.
- [x] **CapabilityStatement.** `GET /{version}/metadata` (`SV2.8`–`SV2.11`),
  with a mutation-verified agreement test between what it declares and what
  the router serves (**F-57**).
- [x] **Auth: PASETO v4.public** (`SV3.2`–`SV3.6`). Verifying key from
  `FHIR_LOCO_PASETO_PUBLIC_KEY` (`src/auth.rs`); writes are refused without
  a token (`a_write_without_a_token_is_refused`).
- [x] **`If-Match`/ETag optimistic concurrency on update** (`SV2.5`), and
  deleted-vs-never-existed as 410 vs 404 (`SV2.4`).
- [x] **`config/production.yaml` is real configuration**, not the empty file
  that refused to boot (**F-59**, three mutation-verified tests).

## Open

- [x] **Conditional create over HTTP** (`SV2.14`) — *done 2026-08-07*.
  `POST` with `If-None-Exist` calls the store's `conditional_create_audited`
  (search-then-create indivisible under the write gate); all four outcomes
  preserved at the HTTP boundary — created `201`, one match returned
  unchanged `200`, ambiguous `412`, unreadable header `400` — and the
  CapabilityStatement declares `conditionalCreate`. Tests:
  `conditional_create_serves_all_four_outcomes`,
  `metadata_declares_conditional_create`. Closes one of **F-58**'s five
  named gaps; `conditional_delete_audited` still has no HTTP verb (a DELETE
  with criteria is a separate spec decision, not part of `SV2.14`).
- [x] **`_include` / `_revinclude`** (`SV2.16`) — *done 2026-08-10*.
  Forward includes ride the store's `refs_of`, reverse ones its reference
  search (`Type/id` values); entries carry `search.mode`
  (`match`/`include`), the included set dedups and never repeats a match,
  and every invalid form — wrong source type, non-reference parameter,
  `:iterate`, unknown type — is **refused by name**, never dropped. Cap:
  more than 1,000 included resources refuses with `too-costly`. The
  CapabilityStatement declares `searchInclude` exactly from the compiled
  map (`searchRevInclude` deliberately undeclared — untyped reference
  columns, see `SV2.16`). Tests:
  `include_and_revinclude_resolve_references`,
  `includes_are_refused_by_name_never_dropped`,
  `metadata_declares_search_includes`.
- [ ] **Transaction Bundles.** `fhir-sqlite`'s `transact_audited`
  deliberately returns `Unsupported` — atomicity by compensation was
  rejected because it would claim an atomicity it does not have. Needs a
  store that can hold one transaction across the operations, or a
  documented refusal.
- [ ] **Type-level and system-level `_history`.** Only instance-level exists.
- [x] **`$export`** (`SV2.15`) — *done 2026-08-09, owner-directed (option
  A)*. System-level async Bulk Data: kick-off (202 + `Content-Location`,
  `Prefer: respond-async` required), status polling with `X-Progress`, a
  manifest whose per-type counts the NDJSON files must match, authenticated
  disclosure-logged file fetches, `DELETE` cancel/cleanup, TTL expiry of
  exported files, and the CapabilityStatement operation declaration.
  `_since` and compartment exports refused by name (the honest edges are in
  `SV2.15` itself). `src/controllers/export.rs`; tests:
  `export_serves_the_async_bulk_data_contract`,
  `export_kickoff_refuses_rather_than_ignores`,
  `metadata_declares_the_export_operation`. F-58's last feature gap.
- [x] **Listener TLS posture** (`SV3.11`) — *stated and enforced 2026-08-07*.
  The requirement now lives in this crate's own spec (service obligations
  moved here with the `SV` restatement, not §10): loopback bind, or TLS
  terminated upstream and acknowledged via
  `FHIR_LOCO_TLS_TERMINATED_UPSTREAM=true` — a non-loopback plaintext bind
  **refuses to boot** (`auth::listener_posture`, checked in `before_run`).
  Tests: `loopback_binds_need_no_acknowledgement`,
  `a_non_loopback_plaintext_bind_refuses_without_the_acknowledgement`.
  In-process TLS termination (rustls on this listener) remains unbuilt and
  undemanded; the requirement obliges the posture decision, not a stack.
- [ ] **`SV4.2` edge concurrency limits.** Body limit (`32mb`) and timeout
  (`30s`) are set in production config; Loco 1.0.1 exposes neither a
  concurrency limit nor an in-flight cap, so those two halves are unmet.
  One of **F-58**'s five gaps.
- [x] **`SV4.3` admin plane** — *done 2026-08-07*. A second listener
  (`FHIR_LOCO_ADMIN_BIND`; off unless set) serves `/health`, `/ready` (a
  mounted store — separate from liveness because liveness-green-over-a-dead-
  store was this crate's original boot bug), and `/metrics` (Prometheus text:
  status-class counters and a fixed-bucket latency histogram, so p99 is
  answerable — no metrics crate, atomics on the request path). Every FHIR
  request is timed by an `after_routes` middleware layer. `src/admin.rs`;
  tests: the four `admin::tests` incl.
  `ready_refuses_without_a_mounted_store` and
  `buckets_are_cumulative_and_p99_answerable`.
- [ ] **Multi-port wiring.** Only `fhir-sqlite` is wired (`Cargo.toml`). All
  six ports now have stores, but the HTTP-facing surface this crate calls —
  `status`, `get_versioned`, `get_all`, `put_audited` — exists in full only
  in `fhir-sqlite`; `fhir-postgresql` has all but `get_versioned`, and
  mysql/mariadb/mssql/oracle have none of the four. `store::init` also
  still holds the stores in a process-global `OnceLock` (`src/store.rs`),
  which wants revisiting when a second backend is mounted.

The [conformance matrix](../spec/databases/conformance-matrix.md) is the status
document to trust. This file is a plan; it is not evidence (`C0.9`).
