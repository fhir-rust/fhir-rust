//! fhir-oracle-store: the Oracle layer.
//!
//! # Live-verified, 2026-08-04 — Store level (`C0.8`)
//!
//! Written 2026-08-04 with no Oracle Instant Client on the build host, and
//! for a time unverified beyond a clean `cargo build` (**F-66**). Instant
//! Client for macOS arm64 turned out to be a direct, no-login download; once
//! installed, this store connected to a live
//! `gvenzl/oracle-free:23-slim-faststart` and its full CRUD/history/search/
//! audit surface was run against it in `tests/oracle_store.rs` (**F-68**,
//! `T11.2`): **7 of 7 tests pass, 0 ignored.** Running it live found and
//! fixed five real defects — uppercase schema case-folding, `R4.5`'s
//! candidate mechanism failing outright, a double schema-qualification bug,
//! a timestamp-binding bug, and a boolean bound as text in token search —
//! see `oracle.rs`'s module doc and `audit.md` **F-68** for the account.
//!
//! **What this does *not* mean:** `R4.5` (snapshot reads under concurrent
//! writers) has no working mechanism on this port — the candidate named in
//! `M14.19` was tried live and removed after it broke every read
//! (`ORA-01466`) — so `get` is not protected against torn reads today. There
//! is also no `concurrency.rs` exercising `H5.4` under contention, no
//! `redaction.rs`, and no `upgrade`/`backfill_norm`. See the [conformance
//! matrix](../../../spec/databases/conformance-matrix.md) for the precise,
//! row-by-row claim — this module doc is a summary, not the source of truth.
//!
//! See `oracle.rs`'s own module doc for the architecture decisions behind
//! the implementation.

/// The tamper-evident audit chain, shared by every port (`M3.16`).
pub use fhir_store::chain;
pub mod oracle;
pub mod oracle_search;
pub mod pool;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("pool: {0}")]
    Pool(String),
    /// A database failure, as the engine reported it.
    #[error("oracle: {0}")]
    Db(String),
    #[error("shred: {0}")]
    Shred(#[from] fhir_oracle_map::ShredError),
    /// Optimistic-concurrency failure: the caller's expected version does
    /// not match the stored one (HTTP 412 at the API layer).
    #[error("version conflict: expected {expected}, found {found}")]
    Conflict { expected: i64, found: i64 },
    /// A client-safe rejection: the request asked for something this server
    /// does not do, described in terms of the request itself (a parameter
    /// name, a modifier, a sort key). Safe to return verbatim — it names
    /// what the caller sent, never what is stored (spec A7.11).
    #[error("{0}")]
    Unsupported(String),
    /// An internal failure. The text is diagnostics for the operator and may
    /// mention schema or values, so it belongs in the log behind an incident
    /// id, never in a response body.
    #[error("{0}")]
    Other(String),
}

/// The engine-agnostic value types, shared by every port.
///
/// Re-exported rather than redefined: these were duplicated in all six ports
/// until **F-45**, and a re-export means a caller written against one port
/// compiles against another without a conversion.
pub use fhir_store::{
    AccessRecord, Audit, ChainBreak, CondCreate, CondDelete, Got, HistEntry, PurgeReport,
    PutOutcome, ResourceStatus, SearchOutcome, TxOp, TxOutcome, UpgradeReport,
};
