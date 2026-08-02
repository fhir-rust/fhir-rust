//! The FHIR store this service puts an HTTP surface on.
//!
//! The storage crates in the sibling repos are libraries by design — each of
//! those projects removed its own server and CLI so that embedding stays cheap.
//! This project is the HTTP surface they deliberately do not carry, which is
//! why the wiring lives here rather than there.
//!
//! Backends are selected by configuration rather than by feature flag, so a
//! deployment can change engine without a rebuild. Only SQLite is wired today;
//! the MySQL and MariaDB stores are still being written, and this module names
//! that rather than pretending the choice already exists.

use std::sync::Arc;
use std::sync::OnceLock;

use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::sqlite::SqliteStore;

/// Every FHIR version this process serves, keyed by schema name (`r3`/`r4`/`r5`).
///
/// A `OnceLock` rather than Loco's shared state: the stores are opened once at
/// boot and never replaced, and threading a handle through every controller
/// signature buys nothing when the value is immutable for the process lifetime.
static STORES: OnceLock<Versions> = OnceLock::new();

pub struct Versions {
    inner: std::collections::BTreeMap<String, Arc<SqliteStore>>,
}

impl Versions {
    /// The store for one FHIR version, or `None` if it is not mounted.
    #[must_use]
    pub fn get(&self, version: &str) -> Option<&Arc<SqliteStore>> {
        self.inner.get(version)
    }

    /// Which versions are mounted, in order.
    #[must_use]
    pub fn mounted(&self) -> Vec<&str> {
        self.inner.keys().map(String::as_str).collect()
    }
}

/// Open every installed version found beside `db_path`.
///
/// A version is mounted only when its schema is actually installed. An attached
/// but empty database would otherwise advertise a CapabilityStatement for
/// resources that cannot be read — a server lying about what it can do is worse
/// than one that admits it serves nothing.
pub async fn init(db_path: &str, assets_dir: &str) -> Result<&'static Versions, String> {
    let mut inner = std::collections::BTreeMap::new();
    for version in ["r3", "r4", "r5"] {
        let asset =
            std::path::Path::new(assets_dir).join(format!("fhir-sqlite-relmap-{version}.json.gz"));
        let Ok(bytes) = std::fs::read(&asset) else {
            continue;
        };
        let map = RelMap::from_gz_bytes(&bytes).map_err(|e| format!("{}: {e}", asset.display()))?;
        let store = SqliteStore::open(db_path, Arc::new(map))
            .await
            .map_err(|e| format!("opening {db_path}: {e}"))?;
        match store.installed_checksum().await {
            Ok(Some(_)) => {
                inner.insert(version.to_string(), Arc::new(store));
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
