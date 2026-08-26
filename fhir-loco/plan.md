# Plan — `fhir-loco`

Design decisions and the open ones. The specification
([`spec/index.md`](spec/index.md), `SV` ids) is normative for behaviour; this
file records *why*, and drafts the decisions not yet made. Companion:
[`tasks.md`](tasks.md).

## Decisions made (recorded from the shipped code)

- **D1 — Loco.rs over bare axum.** The framework supplies boot phases,
  config-per-environment, a test harness, and a background-worker queue; the
  FHIR® controller is plain axum underneath.
- **D2 — mounted over `fhir-sqlite`.** One embedded store, no DSN, no service
  container in CI. Multi-port wiring is open (see `tasks.md`): the
  HTTP-facing store surface this crate calls exists in full only in sqlite.
- **D3 — PASETO v4.public, no unauthenticated mode** (`SV3.2`–`SV3.6`). A
  misconfigured deployment stops at boot rather than trusting callers.
- **D4 — plain HTTP behind a TLS-terminating proxy**, enforced (`SV3.11`): a
  non-loopback bind refuses to boot without
  `FHIR_LOCO_TLS_TERMINATED_UPSTREAM=true`.
- **D5 — the admin plane is a separate listener** (`SV4.3`,
  `FHIR_LOCO_ADMIN_BIND`), carries no FHIR route, and never may.

## Open decision: `$export` (SV2.15, restating `M8`)

The last substantial F-58 gap, and one of the three §13 compliance rows that
depend on this crate. FHIR Bulk Data `$export` is an **async protocol**:
kick-off (`GET …/$export`, `Prefer: respond-async`) answers `202` with a
`Content-Location`; the client polls that status endpoint; completion lists
NDJSON files the client fetches; `DELETE` on the status URL cancels.

### Option A — conformant async Bulk Data, smallest honest slice

System-level `$export` (all types) and `Patient`-type `$export` only, `_type`
and `_since` filters, one NDJSON file per resource type.

- **Job orchestration:** Loco's background-worker queue — already in the
  framework (`connect_workers`), currently unused. A job row carries the
  criteria and status; the worker iterates the store.
- **Store surface:** already sufficient for the slice — `get_all` for the
  scan, search on `_lastUpdated` for `_since`. No new store operation, so no
  six-port shared-core change.
- **Files:** written under a configured export directory, named by job id;
  fetched through an authenticated route (`requiresAccessToken: true` — the
  PASETO gate the rest of the API already enforces). A retention sweep
  deletes expired jobs — the genuinely new operational surface, because
  exported PHI sitting on disk is a disclosure with a lifetime, and the
  disclosure log (`log_access`) must record each file fetch.
- **Sizing:** a multi-session epic — spec section first (the `SV2.15`
  amendment defining exactly the slice), then job model, worker, routes,
  request tests, and the §13 row updates.

### Option B — synchronous bounded export

Stream NDJSON directly from a `GET` for small stores. **Rejected as
`$export`:** the Bulk Data protocol *is* the async contract; a synchronous
endpoint under that name would claim conformance it does not have (`C0.11`'s
shape). If ever wanted, it must be named something else.

### Option C — defer, keep the honest gap

`SV2.15` stays a negative requirement; the compliance rows keep citing it.
Zero cost, and the right answer for as long as no deployment needs bulk
egress.

**Decided 2026-08-09: the owner chose A — built the same day** (`SV2.15`,
closing F-58's last feature gap). Two deviations from the draft, recorded:
the job runs on a plain spawned task with an in-process registry rather
than Loco's worker queue — the queue adds config and backend surface for
no gain while this server is single-process over an embedded store;
revisit if it ever is not — and iteration pages `search_page`'s keyset
cursor rather than `get_all`, which turned out to be a multi-get, not a
scan (the draft was wrong about that). The retention sweep, PASETO on
every export URL, and per-fetch disclosure logging shipped as drafted;
`_since` and compartment exports are refused by name, per the slice.

## Open constraint: edge concurrency limits (SV4.2)

Loco 1.0.1 exposes neither a concurrency limit nor an in-flight cap — a
framework limit, recorded in `spec/04-operations.md`. Path forward when it
matters: a `tower` `ConcurrencyLimitLayer`/`LoadShedLayer` added in
`after_routes`, the same hook the metrics middleware uses. Not built now:
shedding behaviour needs a load target to tune against, and an untuned limit
is a new outage mode.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
