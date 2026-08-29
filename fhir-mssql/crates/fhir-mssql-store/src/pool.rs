//! A connection pool for `mssql`, which ships none of its own.
//!
//! `mysql_async` and `tokio-postgres` (via `deadpool-postgres`) both arrive
//! with pooling built in; `mssql` does not, and no `bb8`/`deadpool`
//! integration for it is mature enough to depend on for a store handling PHI.
//! `bb8` itself is driver-agnostic — [`ManageConnection`] is the entire
//! integration surface — so this is a from-scratch implementation rather than
//! a third-party one, and it is small enough to read in one sitting.

use async_trait::async_trait;
use bb8::ManageConnection;
use mssql_driver::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub type Connection = Client<Compat<TcpStream>>;
pub type Pool = bb8::Pool<ConnectionManager>;

#[derive(Debug, Clone)]
pub struct ConnectionManager {
    config: Config,
}

impl ConnectionManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = mssql_driver::error::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let tcp = TcpStream::connect(self.config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        Client::connect(self.config.clone(), tcp.compat_write()).await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        // `SELECT 1` rather than trusting the connection silently: a
        // half-closed TCP connection can look fine until the next real query
        // fails partway through a transaction the pool has already handed to a
        // caller.
        conn.simple_query("SELECT 1").await?.into_row().await?;
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        // mssql surfaces a broken connection through the next call's Result
        // rather than through a synchronous flag, so there is nothing to check
        // here without doing the async work `is_valid` already does. bb8 calls
        // this only as a fast pre-check; returning `false` defers entirely to
        // `is_valid`.
        false
    }
}

/// Build a pool from an ADO connection string.
///
/// `bb8::Builder::build` does not itself open a connection — found by running
/// `connect` against a deliberately dead port, which returned `Ok` and only
/// failed on first use. A store that constructs successfully and only fails
/// at first use is a worse diagnostic than a connection error at startup, so
/// this borrows and immediately releases one connection before returning,
/// turning an unreachable server into a `connect`-time error rather than a
/// surprise on whichever request happens to run first.
///
/// # Errors
/// If the connection string does not parse, or the pool cannot establish its
/// first connection.
pub async fn connect_pool(ado: &str) -> Result<Pool, crate::StoreError> {
    let config = Config::from_ado_string(ado)
        .map_err(|e| crate::StoreError::Other(format!("bad DSN: {e}")))?;
    let pool = bb8::Pool::builder()
        .build(ConnectionManager::new(config))
        .await
        .map_err(|e| crate::StoreError::Pool(e.to_string()))?;
    pool.get()
        .await
        .map_err(|e| crate::StoreError::Pool(e.to_string()))?;
    Ok(pool)
}
