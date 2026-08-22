//! fhir-mssql-store: the SQL Server layer. Applies generated DDL, writes
//! shredded resources transactionally with history, and reads rows back for
//! reconstruction.
//!
//! Written 2026-08-04 against a live `azure-sql-edge` container (`mssql_ddl.rs`
//! already proved the DDL installs; this proves the store built on it works).
//! `conditional_create_audited` and the schema-upgrade path are not yet
//! implemented — see the module docs on [`mssql`] for exactly what is and is
//! not covered, and `C0.9`: the level claimed for this port is only what the
//! live suite actually exercises.
//!
//! `tiberius` has no built-in connection pool or typed transaction API, unlike
//! `mysql_async` or `tokio-postgres`. [`pool`] supplies the former with a
//! from-scratch `bb8::ManageConnection`; transactions are plain
//! `BEGIN`/`COMMIT`/`ROLLBACK TRANSACTION` statements, since T-SQL has no
//! richer primitive to bind to.

/// The tamper-evident audit chain, shared by every port (`M3.16`).
pub use fhir_store::chain;
pub mod mssql;
pub mod mssql_search;
pub mod pool;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("pool: {0}")]
    Pool(String),
    /// A database failure, as the engine reported it.
    #[error("mssql: {0}")]
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
    PutOutcome, ResourceStatus, SearchOutcome, TxOp, TxOutcome, UpgradeOpts, UpgradeReport,
};
