//! fhir-sqlite-store: the SQLite layer. Applies generated DDL, writes
//! shredded resources transactionally with history, and reads rows back for
//! reconstruction.//!
//! Values bind through `rusqlite` as text, which is what preserves the lexical
//! fidelity `M3.6`/`R4.2` require: decimal scale and partial dates survive
//! because nothing numeric ever parses them. SQLite's dynamic typing makes that
//! easy and makes the opposite mistake easy too — a column declared `TEXT` will
//! still store whatever it is given.

/// The tamper-evident audit chain, shared by every port (`M3.16`).
pub use fhir_store::chain;
pub mod sqlite;
pub mod sqlite_search;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    /// A database failure, as the engine reported it.
    #[error("sqlite: {0}")]
    Db(String),
    #[error("shred: {0}")]
    Shred(#[from] fhir_sqlite_map::ShredError),
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
