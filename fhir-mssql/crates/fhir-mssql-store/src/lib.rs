//! fhir-mssql-store: the SQL Server layer — **not yet written**.
//!
//! **There is no store here yet.** This crate is a placeholder: it re-exports
//! the shared audit chain and defines the error type, and that is all. No
//! driver, no connection, no operations — see `C0.8`, and the port README.

/// The tamper-evident audit chain, shared by every port (`M3.16`).
pub use fhir_store::chain;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("pool: {0}")]
    Pool(String),
    /// A database failure, as the engine reported it.
    #[error("mysql: {0}")]
    Db(String),
    #[error("shred: {0}")]
    Shred(#[from] fhir_mssql_map::ShredError),
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
