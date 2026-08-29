# Architecture

Three crates — no server crate, no CLI binary (`C0.17`, `C0.18`):

- **fhir-oracle-map** — the relational map model (a compact, serialized
  description of every table, column, and element) and the generic engine:
  `shred` (JSON → rows) and `reconstruct` (rows → JSON), plus the DDL
  emitter (`ddl.rs`, the one file in this crate that is genuinely Oracle —
  everything else is shared, byte-identical-modulo-whitespace across all six
  ports, `X15.1`). Reconstruction audits row consumption — every stored row
  must be used exactly once, so schema drift or corruption surfaces as an
  integrity error instead of silent data loss.
- **fhir-oracle-gen** — reads a FHIR® specification package
  (StructureDefinitions + SearchParameters) and builds the map: identifier
  fitting under a 63-byte budget (safe on the 12.2+ floor's 128-byte limit,
  but not derived from it — the same 63 is used on every port, so one map
  serves all six), width-based force-splitting, cycle detection (type cycles
  spill into `_deep` tables; recursive elements share tables via ordinal
  sign lanes), and the search compiler that resolves FHIRPath expressions by
  walking the map tree.
- **fhir-oracle-store** — the driver and the operations:
  `connect`/`init`/`put`/`get`/`delete`/`history`/`vread`/`search`/
  `search_full`/`search_page`/`verify_audit`/`purge`/`log_access`. Uses the
  `oracle` crate (ODPI-C/OCI bindings), synchronous under the hood and
  wrapped in `tokio::task::spawn_blocking`. Live-verified against a real
  Oracle for the first time 2026-08-04 — see [Introduction](introduction.md)
  and the [dialect annex](../../spec/14-oracle-dialect.md)'s `M14.34` for
  what running it live actually found.

There is no `fhir-oracle-server` and no `fhir-oracle` CLI binary. If an
earlier version of this book described one — bundle processing, generated
`CapabilityStatement`s, request ids — that described `fhir-postgresql`'s
book text copied here with the name substituted (`F-56`); no port in this
workspace has ever had a server or CLI crate. The actual HTTP surface, where
one exists, is [`fhir-loco`](https://github.com/fhir-rust/fhir-rust/tree/main/fhir-loco),
a separate crate currently mounted over `fhir-sqlite`, not this port.

## Why the driver is synchronous, unlike five of the six ports

`oracle::Connection` calls into ODPI-C, which calls into OCI, which blocks
the calling OS thread on network I/O — there is no async Oracle driver for
Rust. `fhir-sqlite` is the only other port with this shape (`rusqlite` is
also synchronous); both wrap every public method's body in one
`tokio::task::spawn_blocking` call rather than pretending to be async.

Two consequences worth knowing before reading the store's source:

- **The connection pool is the driver's own**, `oracle::pool::Pool` — unlike
  `fhir-mssql`, which had to hand-write a `bb8::ManageConnection` because
  `mssql` (its TDS driver) ships no pool at all.
- **Row locking uses `SELECT … FOR UPDATE`**, Oracle's native syntax,
  instead of the `WITH (UPDLOCK, ROWLOCK)` hint SQL Server needs for the
  same guarantee.

## The decisive design choice

**Metadata over codegen**: rather than generating Rust for 3 versions ×
~150 resource types, the generator emits data (the map) and one engine
interprets it. The engine is a few thousand lines, tested once, correct for
every resource type — and the map doubles as documentation, carrying the
FHIR path of every column.

Design decisions, risks, and milestones live in `plan.md`; the normative
behaviour is [`spec/index.md`](../../spec/index.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
