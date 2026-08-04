# Architecture

**Three crates.** Earlier text in this book described five — adding a
`fhir-sqlite-server` (axum) and a `fhir-sqlite` CLI binary that do not exist
in this workspace and are not planned (`C0.17`, `C0.18`). A server can be
built over this library — [`fhir-loco`](../../../fhir-loco/) is that server —
but it is a separate crate in a separate workspace, and none of its code
lives here.

- **`fhir-sqlite-map`** — the relational map model (a serialized description
  of every table, column, and element), the generic engine (`shred`: JSON →
  rows, `reconstruct`: rows → JSON), the Rust-side JSON canonicalizer
  (`canon`, see [The storage model](storage-model.md)), the accent/case
  folder (`fold`), and this engine's DDL emitter
  (`ddl.rs`). Everything in this crate except `ddl.rs` is byte-identical
  (modulo whitespace) across all six ports (`X15.1`) — it operates on Rust
  types and never emits SQL. Reconstruction audits row consumption: every
  stored row must be used exactly once, so schema drift or corruption
  surfaces as an integrity error instead of silently dropped data.
- **`fhir-sqlite-gen`** — reads a FHIR specification package
  (StructureDefinitions + SearchParameters) and *builds* the map:
  identifier fitting under a 63-character budget (`PG_MAX_IDENT`, tighter
  than every supported engine's own limit, so one generated map serves all
  six — `M14.1`), width-based force-splitting past 150 columns
  (`SPLIT_WIDTH`), cycle detection (type cycles spill into a `_deep` table;
  `contentReference` recursion shares one table via signed ordinal lanes),
  and the search-parameter compiler that resolves FHIRPath-derived
  expressions by walking the map tree. This crate, including its tests, is
  also identical across all six ports — the DDL it hands to `ddl.rs` is
  dialect-neutral data, not SQL.
- **`fhir-sqlite-store`** — the one crate that is actually SQLite-specific.
  `SqliteStore` wraps a single `rusqlite::Connection` behind a
  `tokio::sync::Mutex`, with every database call executed inside
  `tokio::task::spawn_blocking` (`rusqlite` is synchronous; nothing here
  talks to SQLite off the blocking pool). It owns transactional writes with
  history append and the hash chain, optimistic concurrency
  (`put_audited`/`StoreError::Conflict`), single-hop reference-chain search
  (`sqlite_search.rs`, a fork of the PostgreSQL store's builder rather than
  shared code — see [Search](search.md)), install/upgrade/backfill, and the
  audit primitives covered in [The trust boundary](trust-boundary.md). It
  depends on and re-exports [`fhir-store`](../../../fhir-store/) for the
  parts that are not SQLite-specific at all — `Audit`, `AccessRecord`, and
  the chain primitives (`chain::preimage`, `chain::link`, `chain::KeyRing`) —
  so `fhir_sqlite_store::Audit` resolves without an extra dependency, and a
  fix to the chain math benefits every port from one crate rather than six
  copies.

## One connection, one mutex, one writer

`SqliteStore` holds exactly one `rusqlite::Connection`. Two tasks calling
methods on the *same* `SqliteStore` handle are serialized by its internal
mutex and can never interleave — which is convenient, but also means testing
concurrent behaviour through one handle would prove nothing: it could not
fail however broken the code was. `tests/concurrency.rs` therefore opens a
**second** `SqliteStore` on the same file, which is a second SQLite
connection and an ordinary deployment shape (two processes, or two pooled
connections in one). There, SQLite's actual single-writer lock and whatever
transaction a reader holds are the only things standing between a reader and
a torn read — see `R4.5` in [The trust boundary](trust-boundary.md).

## The decisive design choice: metadata over codegen

Rather than generating Rust source for 3 FHIR versions × ~150 resource types,
`fhir-sqlite-gen` emits **data** — the relational map — and one generic
engine interprets it at runtime. The engine (`shred`/`reconstruct` in
`fhir-sqlite-map`) is a few thousand lines, tested once, and correct for
every resource type it is handed; the map doubles as documentation, carrying
the FHIRPath of every table and column it generated (`RelMap::bundled("r5")?.resources["Patient"]`
is inspectable directly in Rust — see [Querying with SQL](querying.md)).

This is also why the map crate, not the gen crate, is where a dialect lives:
`ddl.rs` translates the same map into SQLite's `CREATE TABLE`/`CREATE INDEX`
statements, and `sqlite_search.rs` (in the store crate) translates compiled
search targets into SQLite's dialect of `WHERE` clauses — everything else the
generator produces is consumed identically by all six engines.

Design decisions and their reasons live in `plan.md`; work-item status is
`tasks.md` (but see the root [`CLAUDE.md`](../../../CLAUDE.md) on trusting
that file); normative behaviour is [`spec/index.md`](../../spec/index.md), the
shared core plus this port's [dialect annex](../../spec/14-sqlite-dialect.md).
