//! The FHIR store this service puts an HTTP surface on.
//!
//! The storage crates in this monorepo are libraries by design — each port
//! removed its own server and CLI so that embedding stays cheap. This project
//! is the HTTP surface they deliberately do not carry, which is why the wiring
//! lives here rather than there.
//!
//! Backends are selected by configuration rather than by feature flag, so a
//! deployment can change engine without a rebuild: `FHIR_LOCO_BACKEND` is
//! `sqlite` (the default) or `postgresql`. One backend per process — the
//! stores are opened once at boot and never replaced.
//!
//! [`AnyStore`] is an enum, not a trait object: the two stores' surfaces are
//! call-compatible but not identical types, the set of backends is closed and
//! small, and an enum keeps every forwarded call monomorphic and every
//! signature honest. The value types the wrapper returns (`HistEntry`,
//! `SearchOutcome`, `PutOutcome`, …) are `fhir-store`'s, which both ports
//! re-export — one struct, two paths — so nothing is converted on the way
//! through. The one genuinely shared piece of *map* data is handled at mount
//! time: the relational map's model is token-identical across ports
//! (`X15.1`), so the postgres map's bytes parse as `fhir_sqlite_map`'s
//! `RelMap` too, and [`AnyStore::map`] serves that copy for the controllers
//! that introspect search parameters.

use std::sync::Arc;
use std::sync::OnceLock;

use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::sqlite::SqliteStore;
use fhir_sqlite_store::{
    AccessRecord, Audit, CondCreate, CondDelete, HistEntry, PutOutcome, ResourceStatus,
    SearchOutcome,
};
use serde_json::Value;

/// Every FHIR version this process serves, keyed by schema name (`r3`/`r4`/`r5`).
///
/// A `OnceLock` rather than Loco's shared state: the stores are opened once at
/// boot and never replaced, and threading a handle through every controller
/// signature buys nothing when the value is immutable for the process
/// lifetime. (The old "revisit when a second backend is mounted" note is
/// resolved by *when* the choice happens: the backend is picked at boot, so
/// one immutable map of versions still describes the whole process.)
static STORES: OnceLock<Versions> = OnceLock::new();

/// What a store call failed with, in the only distinctions the HTTP layer
/// acts on. Both ports' `StoreError` types map into this — `Unsupported` and
/// `Conflict` keep their meaning, everything else is `Other` and stays out of
/// response bodies.
#[derive(Debug)]
pub enum StoreFailure {
    Unsupported(String),
    Conflict { expected: i64, found: i64 },
    Other(String),
}

impl std::fmt::Display for StoreFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(m) => write!(f, "unsupported: {m}"),
            Self::Conflict { expected, found } => {
                write!(f, "version conflict: expected {expected}, found {found}")
            }
            Self::Other(m) => write!(f, "{m}"),
        }
    }
}

impl From<fhir_sqlite_store::StoreError> for StoreFailure {
    fn from(e: fhir_sqlite_store::StoreError) -> Self {
        use fhir_sqlite_store::StoreError as E;
        match e {
            E::Unsupported(m) => Self::Unsupported(m),
            E::Conflict { expected, found } => Self::Conflict { expected, found },
            other => Self::Other(other.to_string()),
        }
    }
}

impl From<fhir_postgresql_store::StoreError> for StoreFailure {
    fn from(e: fhir_postgresql_store::StoreError) -> Self {
        use fhir_postgresql_store::StoreError as E;
        match e {
            E::Unsupported(m) => Self::Unsupported(m),
            E::Conflict { expected, found } => Self::Conflict { expected, found },
            other => Self::Other(other.to_string()),
        }
    }
}

/// One mounted store, whatever engine backs it.
#[derive(Clone)]
pub enum AnyStore {
    Sqlite(Arc<SqliteStore>),
    /// The postgres store plus the map re-parsed as `fhir_sqlite_map`'s
    /// `RelMap` (same bytes, token-identical model), so [`Self::map`] has one
    /// return type.
    Postgres(Arc<fhir_postgresql_store::Store>, Arc<RelMap>),
}

macro_rules! forward {
    ($self:ident, $($call:tt)+) => {
        match $self {
            AnyStore::Sqlite(s) => s.$($call)+.await.map_err(StoreFailure::from),
            AnyStore::Postgres(s, _) => s.$($call)+.await.map_err(StoreFailure::from),
        }
    };
}

impl AnyStore {
    /// The relational map: the sqlite store's own, or the mount-time copy for
    /// postgres.
    #[must_use]
    pub fn map(&self) -> &RelMap {
        match self {
            Self::Sqlite(s) => s.map(),
            Self::Postgres(_, m) => m,
        }
    }

    pub async fn status(&self, rtype: &str, id: &str) -> Result<ResourceStatus, StoreFailure> {
        forward!(self, status(rtype, id))
    }

    pub async fn get(&self, rtype: &str, id: &str) -> Result<Option<Value>, StoreFailure> {
        match self {
            // sqlite returns the bare resource; postgres returns `Got`
            // (resource + version). This layer re-reads versions through
            // `status`, so the bare resource is the common shape.
            Self::Sqlite(s) => s.get(rtype, id).await.map_err(StoreFailure::from),
            Self::Postgres(s, _) => s
                .get(rtype, id)
                .await
                .map(|o| o.map(|g| g.resource))
                .map_err(StoreFailure::from),
        }
    }

    pub async fn vread(
        &self,
        rtype: &str,
        id: &str,
        version_id: i64,
    ) -> Result<Option<HistEntry>, StoreFailure> {
        forward!(self, vread(rtype, id, version_id))
    }

    pub async fn history(&self, rtype: &str, id: &str) -> Result<Vec<HistEntry>, StoreFailure> {
        forward!(self, history(rtype, id))
    }

    pub async fn history_page(
        &self,
        rtype: Option<&str>,
        count: i64,
        since: Option<&str>,
    ) -> Result<Vec<(String, String, HistEntry)>, StoreFailure> {
        forward!(self, history_page(rtype, count, since))
    }

    pub async fn search(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
    ) -> Result<Vec<String>, StoreFailure> {
        forward!(self, search(rtype, params, count, offset))
    }

    /// `search_full` without a sort argument: this layer never sorts, and the
    /// two ports' `SortKey` types are distinct, so the empty slice lives here.
    pub async fn search_full(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        want_total: bool,
    ) -> Result<SearchOutcome, StoreFailure> {
        match self {
            Self::Sqlite(s) => s
                .search_full(rtype, params, count, offset, &[], want_total)
                .await
                .map_err(StoreFailure::from),
            Self::Postgres(s, _) => s
                .search_full(rtype, params, count, offset, &[], want_total)
                .await
                .map_err(StoreFailure::from),
        }
    }

    pub async fn search_page(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        want_total: bool,
        after_id: Option<&str>,
    ) -> Result<SearchOutcome, StoreFailure> {
        match self {
            Self::Sqlite(s) => s
                .search_page(rtype, params, count, offset, &[], want_total, after_id)
                .await
                .map_err(StoreFailure::from),
            Self::Postgres(s, _) => s
                .search_page(rtype, params, count, offset, &[], want_total, after_id)
                .await
                .map_err(StoreFailure::from),
        }
    }

    pub async fn refs_of(
        &self,
        rtype: &str,
        ids: &[String],
        param: &str,
    ) -> Result<Vec<(String, String)>, StoreFailure> {
        forward!(self, refs_of(rtype, ids, param))
    }

    pub async fn put_audited(
        &self,
        resource: &Value,
        expected_version: Option<i64>,
        audit: &Audit,
    ) -> Result<PutOutcome, StoreFailure> {
        forward!(self, put_audited(resource, expected_version, audit))
    }

    pub async fn delete_audited(
        &self,
        rtype: &str,
        id: &str,
        audit: &Audit,
    ) -> Result<bool, StoreFailure> {
        forward!(self, delete_audited(rtype, id, audit))
    }

    pub async fn conditional_create_audited(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
        resource: &Value,
        audit: &Audit,
    ) -> Result<CondCreate, StoreFailure> {
        forward!(
            self,
            conditional_create_audited(rtype, criteria, resource, audit)
        )
    }

    pub async fn conditional_delete_audited(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
        audit: &Audit,
    ) -> Result<CondDelete, StoreFailure> {
        forward!(self, conditional_delete_audited(rtype, criteria, audit))
    }

    pub async fn log_access(&self, rec: &AccessRecord) -> Result<(), StoreFailure> {
        forward!(self, log_access(rec))
    }

    pub async fn installed_checksum(&self) -> Result<Option<String>, StoreFailure> {
        forward!(self, installed_checksum())
    }

    /// Disclosure-log length — test observability, both backends.
    pub async fn access_log_len(&self) -> Result<i64, StoreFailure> {
        forward!(self, access_log_len())
    }
}

pub struct Versions {
    inner: std::collections::BTreeMap<String, AnyStore>,
}

impl Versions {
    /// The store for one FHIR version, or `None` if it is not mounted.
    #[must_use]
    pub fn get(&self, version: &str) -> Option<&AnyStore> {
        self.inner.get(version)
    }

    /// Which versions are mounted, in order.
    #[must_use]
    pub fn mounted(&self) -> Vec<&str> {
        self.inner.keys().map(String::as_str).collect()
    }
}

/// Which backend to open, resolved from configuration by the caller.
pub enum BackendConfig<'a> {
    Sqlite {
        db_path: &'a str,
        assets_dir: &'a str,
    },
    Postgres {
        /// A `tokio_postgres` connection string (`host=… user=…` or a URL).
        /// TLS policy comes from `PGSSLMODE`, exactly as the store documents
        /// (`O10.7`).
        dsn: &'a str,
        assets_dir: &'a str,
    },
}

/// Open every installed version the configured backend holds.
///
/// A version is mounted only when its schema is actually installed
/// (`installed_checksum` returns `Some`). An empty database would otherwise
/// advertise a CapabilityStatement for resources that cannot be read — a
/// server lying about what it can do is worse than one that admits it serves
/// nothing.
pub async fn init(cfg: BackendConfig<'_>) -> Result<&'static Versions, String> {
    let mut inner = std::collections::BTreeMap::new();
    for version in ["r3", "r4", "r5"] {
        let store = match &cfg {
            BackendConfig::Sqlite {
                db_path,
                assets_dir,
            } => {
                let asset = std::path::Path::new(assets_dir)
                    .join(format!("fhir-sqlite-relmap-{version}.json.gz"));
                let Ok(bytes) = std::fs::read(&asset) else {
                    continue;
                };
                let map = RelMap::from_gz_bytes(&bytes)
                    .map_err(|e| format!("{}: {e}", asset.display()))?;
                let store = SqliteStore::open(db_path, Arc::new(map))
                    .await
                    .map_err(|e| format!("opening {db_path}: {e}"))?;
                AnyStore::Sqlite(Arc::new(store))
            }
            BackendConfig::Postgres { dsn, assets_dir } => {
                let asset = std::path::Path::new(assets_dir)
                    .join(format!("fhir-postgresql-relmap-{version}.json.gz"));
                let Ok(bytes) = std::fs::read(&asset) else {
                    continue;
                };
                let pg_map = fhir_postgresql_map::model::RelMap::from_gz_bytes(&bytes)
                    .map_err(|e| format!("{}: {e}", asset.display()))?;
                // Same bytes, token-identical model (X15.1): the sqlite-map
                // parse is the copy `AnyStore::map` serves.
                let map = RelMap::from_gz_bytes(&bytes)
                    .map_err(|e| format!("{}: {e}", asset.display()))?;
                let pg_cfg: tokio_postgres::Config =
                    dsn.parse().map_err(|e| format!("FHIR_LOCO_PG_DSN: {e}"))?;
                let store = fhir_postgresql_store::Store::connect(pg_cfg, Arc::new(pg_map))
                    .await
                    .map_err(|e| format!("connecting postgres: {e}"))?;
                AnyStore::Postgres(Arc::new(store), Arc::new(map))
            }
        };
        match store.installed_checksum().await {
            Ok(Some(_)) => {
                inner.insert(version.to_string(), store);
            }
            Ok(None) => tracing::info!(version, "schema not installed; not mounting"),
            Err(e) => return Err(format!("probing {version}: {e}")),
        }
    }
    Ok(STORES.get_or_init(|| Versions { inner }))
}

/// The mounted versions, or `None` before [`init`] has run.
#[must_use]
pub fn versions() -> Option<&'static Versions> {
    STORES.get()
}
