//! A connection pool via the `oracle` crate's own `pool` module.
//!
//! # Live-verified, 2026-08-04 (**F-68**)
//!
//! Written 2026-08-04 with no Oracle Instant Client available in the build
//! environment, and for a time unverified beyond a clean compile (**F-66**).
//! Instant Client is now installed and `tests/oracle_store.rs` builds a pool
//! and checks connections out of it on every test run, live against
//! `gvenzl/oracle-free:23-slim-faststart` — 7 of 7 tests pass, 0 ignored.
//!
//! Unlike `tiberius` (`fhir-mssql`), which ships no pool at all, the `oracle`
//! crate wraps OCI's own session pooling directly — `oracle::pool::Pool` — so
//! there is no hand-written `bb8::ManageConnection` here. What this module
//! adds is only the DSN-to-builder translation and the error-type bridge.
//!
//! Every operation on `oracle::Connection` is **synchronous** (it calls into
//! ODPI-C, which calls into OCI, which blocks the OS thread on network I/O).
//! `fhir_oracle_store::oracle` wraps every call in `tokio::task::spawn_blocking`,
//! the same shape `fhir-sqlite` uses for `rusqlite` — the only other sync
//! driver among the six ports.

use std::sync::Arc;

use oracle::pool::{Pool, PoolBuilder};

pub type OraclePool = Arc<Pool>;

/// Build a pool from Oracle's three-part credential, not a single DSN string.
///
/// Unlike the other five ports, there is no one-string connection form to
/// parse: `oracle::pool::PoolBuilder::new` takes `username`, `password`, and
/// `connect_string` (an Easy Connect string, `host:port/service_name`, or a
/// TNS alias) separately. `connect` in `oracle.rs` accepts them the same way
/// rather than inventing a fourth ADO-style dialect this project would then
/// have to document.
///
/// # Errors
/// If the pool cannot be built — including, on this host, "library not
/// loaded" for `libclntsh`, since no Oracle Instant Client is installed.
pub fn connect_pool(
    username: &str,
    password: &str,
    connect_string: &str,
) -> Result<OraclePool, crate::StoreError> {
    let mut builder = PoolBuilder::new(username, password, connect_string);
    // Small and fixed rather than tuned: this is unverified code with no
    // engine to tune against, and a wrong guess here is a performance
    // question, not a correctness one — unlike the query and DDL text, which
    // is why those get the annex's full attention and this does not.
    builder.min_connections(1).max_connections(10);
    let pool = builder
        .build()
        .map_err(|e| crate::StoreError::Pool(e.to_string()))?;
    Ok(Arc::new(pool))
}
