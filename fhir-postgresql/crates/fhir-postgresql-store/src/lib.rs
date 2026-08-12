//! fhir-postgresql-store: the PostgreSQL layer. Applies generated DDL, writes
//! shredded resources transactionally with history, and reads rows back for
//! reconstruction.//!
//! Values cross the wire as text with explicit casts (`($n::text)::numeric`),
//! which keeps the lexical fidelity `M3.6`/`R4.2` require — decimal scale and
//! partial dates — intact in both directions.

/// The tamper-evident audit chain, shared by every port (`M3.16`).
pub use fhir_store::chain;

/// The engine-agnostic value types, shared by every port.
///
/// Re-exported rather than redefined: these were duplicated in all six ports
/// until **F-45**, and a re-export means a caller written against one port
/// compiles against another without a conversion.
pub use fhir_store::{
    AccessRecord, Audit, ChainBreak, CondCreate, CondDelete, Got, HistEntry, PurgeReport,
    PutOutcome, ResourceStatus, SearchOutcome, TxOp, TxOutcome, UpgradeReport,
};
pub mod search;

use std::collections::BTreeSet;
use std::sync::Arc;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use fhir_postgresql_map::model::{ColTy, RelMap, ResourceMap, TableKind};
use fhir_postgresql_map::reconstruct::{InRow, ReconIn, reconstruct};
use fhir_postgresql_map::shred::{DeepRow, ExtRow, ShredOut, SqlVal, shred};
use fhir_postgresql_map::value::LeafVal;
use serde_json::Value;
use thiserror::Error;
use tokio_postgres::NoTls;
use tokio_postgres::types::ToSql;

#[derive(Debug, Error)]
pub enum StoreError {
    /// A database failure.
    ///
    /// Displayed through [`pg_detail`] rather than as `{0}`:
    /// `tokio_postgres::Error`'s own `Display` is the bare string
    /// `"db error"`, and everything an operator needs — the SQLSTATE, the
    /// message, the hint — hangs off `source()`. Logging the outer error
    /// alone throws away the entire diagnosis.
    #[error("postgres: {}", pg_detail(.0))]
    Pg(#[from] tokio_postgres::Error),
    #[error("pool: {0}")]
    Pool(String),
    #[error("shred: {0}")]
    Shred(#[from] fhir_postgresql_map::ShredError),
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

/// The useful half of a `tokio_postgres::Error`: SQLSTATE, message, and hint
/// from the `DbError` behind `source()`.
///
/// This text is for logs, never for a response body (spec A7.11) — it can
/// name schema objects and, in a constraint violation, values.
fn pg_detail(e: &tokio_postgres::Error) -> String {
    match e.as_db_error() {
        Some(db) => {
            let mut out = format!("[{}] {}", db.code().code(), db.message());
            if let Some(d) = db.detail() {
                out.push_str(&format!(" — {d}"));
            }
            if let Some(h) = db.hint() {
                out.push_str(&format!(" (hint: {h})"));
            }
            out
        }
        // Transport, TLS, and protocol failures have no DbError; walk the
        // chain so the cause is not lost either.
        None => {
            let mut out = e.to_string();
            let mut src = std::error::Error::source(e);
            while let Some(s) = src {
                out.push_str(&format!(": {s}"));
                src = s.source();
            }
            out
        }
    }
}

impl From<deadpool_postgres::PoolError> for StoreError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        StoreError::Pool(e.to_string())
    }
}

/// Open the read snapshot every multi-statement read runs in (spec R4.5).
///
/// `REPEATABLE READ` gives all statements in the transaction one snapshot;
/// `READ ONLY` lets PostgreSQL skip taking an xid and makes the intent
/// unmistakable to anyone reading a `pg_stat_activity` dump.
async fn snapshot(
    client: &mut deadpool_postgres::Client,
) -> Result<deadpool_postgres::Transaction<'_>, StoreError> {
    client
        .build_transaction()
        .isolation_level(tokio_postgres::IsolationLevel::RepeatableRead)
        .read_only(true)
        .start()
        .await
        .map_err(StoreError::Pg)
}

/// Append one history row with its audit envelope, hash links, and keyed
/// tag (spec M3.15, M3.16, M3.16a, M3.16b).
///
/// Digests are computed **here, not in SQL**. They are unkeyed over a public
/// pre-image, so a database that computes them holds everything needed to
/// forge them — and, more decisively, a MAC can only be introduced where the
/// database is not. See [`crate::chain`] for what each layer does and does
/// not buy.
#[allow(clippy::too_many_arguments)]
async fn append_history(
    tx: &tokio_postgres::Transaction<'_>,
    schema: &str,
    hist: &str,
    id: &str,
    version: i64,
    op: &str,
    resource_json: Option<&String>,
    audit: &Audit,
    keys: &crate::chain::KeyRing,
) -> Result<(), StoreError> {
    // One read gathers everything the chain commits to. It is safe to split
    // this from the insert because the write path already holds a
    // `SELECT … FOR UPDATE` row lock on this resource's base row, so no other
    // writer can append a version in between.
    //
    // The timestamp is rendered in UTC with an explicit format rather than
    // `::text`: the default rendering follows the session's TimeZone, so a
    // verifier connecting from another zone would recompute different bytes
    // and report every row as broken.
    let row = tx
        .query_one(
            &format!(
                "SELECT now() AS ts, \
                        to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS.US') AS ts_utc, \
                        (($1::text)::jsonb)::text AS canon, \
                        prev.\"row_hash\", prev.\"row_hash_sha3\" \
                 FROM (SELECT 1) one \
                 LEFT JOIN LATERAL ( \
                     SELECT h.\"row_hash\", h.\"row_hash_sha3\" FROM \"{schema}\".\"{hist}\" h \
                     WHERE h.\"id\" = $2::text ORDER BY h.\"version_id\" DESC LIMIT 1 \
                 ) prev ON true"
            ),
            &[&resource_json, &id],
        )
        .await?;
    let ts: std::time::SystemTime = row.get(0);
    let ts_utc: String = row.get(1);
    // The *stored* normalized form, not the submitted text: jsonb reorders
    // keys and rewrites number spellings, so hashing the input would make
    // every chain fail the moment it was checked against what was saved.
    //
    // That stored form is then canonicalized **in Rust** (`X15.2`, `M14.12`),
    // which is what audit F-07 was about. Until then the bytes signed were
    // whatever `jsonb::text` rendered — a form defined by a PostgreSQL version
    // rather than by this specification, and unverifiable by any other port.
    //
    // Writer and verifier both canonicalize the same stored bytes, so they
    // agree by construction rather than by both happening to call the same SQL
    // cast. This is what `M14.13` means by fixing the pre-image making the
    // column type stop mattering.
    let canon: Option<String> = row
        .get::<_, Option<String>>(2)
        .map(|stored| canon_of(&stored));
    let prev_sha256: Option<Vec<u8>> = row.get(3);
    let prev_sha3: Option<Vec<u8>> = row.get(4);

    let pre = crate::chain::preimage(id, version, &ts_utc, op, canon.as_deref(), &audit.actor);
    let (row_hash, row_hash_sha3) =
        crate::chain::link(prev_sha256.as_deref(), prev_sha3.as_deref(), &pre);
    let row_mac = keys
        .signing()
        .map(|k| crate::chain::mac(k, prev_sha256.as_deref(), &pre));

    tx.execute(
        &format!(
            "INSERT INTO \"{schema}\".\"{hist}\" \
               (\"id\", \"version_id\", \"last_updated\", \"op\", \"resource\", \
                \"actor\", \"actor_source\", \"client\", \"request_id\", \"reason\", \
                \"prev_hash\", \"row_hash\", \"prev_hash_sha3\", \"row_hash_sha3\", \
                \"row_mac\") \
             VALUES ($1::text, $2::bigint, $3::timestamptz, $4::text, ($5::text)::jsonb, \
                     $6::text, $7::text, $8::text, $9::text, $10::text, \
                     $11::bytea, $12::bytea, $13::bytea, $14::bytea, $15::text)"
        ),
        &[
            &id,
            &version,
            &ts,
            &op,
            &resource_json,
            &audit.actor,
            &audit.actor_source,
            &audit.client,
            &audit.request_id,
            &audit.reason,
            &prev_sha256,
            &row_hash,
            &prev_sha3,
            &row_hash_sha3,
            &row_mac,
        ],
    )
    .await?;
    Ok(())
}

/// The canonical bytes the chain commits to, computed in Rust (`X15.2`).
///
/// Takes the resource **as stored** — PostgreSQL's `jsonb` rendering — and puts
/// it through the shared `canon::canonicalize`, the same function the other five
/// ports use. One function, called by both the writer and `verify_audit`, so
/// the two cannot drift into disagreeing about what was signed.
///
/// Text that does not parse as JSON is passed through unchanged rather than
/// panicking: it cannot have come from a `jsonb` column, so the only way to see
/// it is a column that is not what this code believes it to be — and a chain
/// break is the right report for that, not a crash in a verifier.
fn canon_of(stored: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(stored) {
        Ok(v) => fhir_postgresql_map::canon::canonicalize(&v),
        Err(_) => stored.to_string(),
    }
}

/// `O10.4b`: dropped columns — or columns of dropped tables — whose element
/// path reappears in a *different* table of the new map. That is the shape a
/// relocation (`G2.6a`'s force-split, **F-90**) takes in the generic diff,
/// and it must be told apart from a genuine removal before the destructive
/// gate can be trusted. Returns `(old table, column, new table)`.
fn moved_columns(old_map: &RelMap, new_map: &RelMap) -> Vec<(String, String, String)> {
    use std::collections::{HashMap, HashSet};
    let mut new_paths: HashMap<&str, &str> = HashMap::new();
    let mut new_cols_by_table: HashMap<&str, HashSet<&str>> = HashMap::new();
    for rm in new_map.resources.values() {
        for t in &rm.tables {
            let set = new_cols_by_table.entry(t.name.as_str()).or_default();
            for c in &t.cols {
                set.insert(c.name.as_str());
                if !c.path.is_empty() {
                    new_paths.insert(c.path.as_str(), t.name.as_str());
                }
            }
        }
    }
    let mut moved = Vec::new();
    for rm in old_map.resources.values() {
        for t in &rm.tables {
            let kept = new_cols_by_table.get(t.name.as_str());
            for c in &t.cols {
                let dropped = kept.is_none_or(|set| !set.contains(c.name.as_str()));
                if !dropped || c.path.is_empty() {
                    continue;
                }
                if let Some(&nt) = new_paths.get(c.path.as_str())
                    && nt != t.name
                {
                    moved.push((t.name.clone(), c.name.clone(), nt.to_string()));
                }
            }
        }
    }
    moved
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hex_decode(s: &str) -> Result<Vec<u8>, StoreError> {
    if !s.len().is_multiple_of(2) {
        return Err(StoreError::Other("bad hex asset".into()));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|_| StoreError::Other("bad hex asset".into()))
        })
        .collect()
}

/// The advisory-lock key for one set of conditional criteria.
///
/// Criteria are order-insensitive as far as FHIR is concerned, so they are
/// sorted before hashing: `identifier=x&name=y` and `name=y&identifier=x`
/// select the same resources and must contend for the same lock.
fn criteria_lock_key(schema: &str, rtype: &str, criteria: &[(String, String)]) -> i64 {
    use sha2::{Digest, Sha256};
    let mut sorted: Vec<&(String, String)> = criteria.iter().collect();
    sorted.sort();
    let mut h = Sha256::new();
    h.update(schema.as_bytes());
    h.update([0]);
    h.update(rtype.as_bytes());
    for (k, v) in sorted {
        h.update([0]);
        h.update(k.as_bytes());
        h.update([1]);
        h.update(v.as_bytes());
    }
    let d = h.finalize();
    i64::from_be_bytes(d[..8].try_into().expect("32-byte digest"))
}

fn hist_entry(row: tokio_postgres::Row) -> Result<HistEntry, StoreError> {
    let op: String = row.get(2);
    let resource: Option<String> = row.get(3);
    Ok(HistEntry {
        version_id: row.get(0),
        last_updated: row.get(1),
        op: op.chars().next().unwrap_or('?'),
        resource: resource
            .map(|t| serde_json::from_str(&t).map_err(|e| StoreError::Other(e.to_string())))
            .transpose()?,
    })
}

pub struct Store {
    pool: Pool,
    map: Arc<RelMap>,
    /// Keys for the tamper-evidence MAC (M3.16b). Loaded once, from the
    /// environment; never written to the database and never logged.
    keys: crate::chain::KeyRing,
}

/// How the connection to PostgreSQL is protected (spec O10.7).
///
/// The link between fhir-postgresql and its database carries whole resources — it is
/// exactly as sensitive as the link to the client, and until now it was
/// unconditionally plaintext.
///
/// fhir-postgresql deliberately deviates from libpq in one direction only: libpq's
/// `require` encrypts without validating the server certificate, which stops
/// a passive eavesdropper but not an active one. Here `require` validates.
/// `verify-ca` and `verify-full` are accepted as synonyms for it, because
/// rustls always checks the hostname — there is no "CA but not hostname"
/// mode to offer, and the stricter reading is the safe one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SslPolicy {
    /// No TLS. Only appropriate for a loopback connection on a host where
    /// nothing else runs.
    Disable,
    /// Use TLS if the server offers it, and validate nothing. Does not survive
    /// an active attacker: a server that declines TLS gets a plaintext link,
    /// and one that offers a forged certificate gets an encrypted one to the
    /// attacker. This is libpq's default; it is **not** this port's.
    Prefer,
    /// Require TLS, and validate the server certificate and hostname.
    ///
    /// The default, because `O10.7` says a port MUST default to verifying and
    /// this connection carries PHI (**F-17**).
    ///
    /// It was `Prefer` until 2026-08-03, on the reasoning that changing it
    /// would break deployments relying on libpq-compatible behaviour. The
    /// database crates have never been published — `spec/publishing.md` records
    /// all eighteen names as still available — so there were no such
    /// deployments, and the objection was to a cost nobody was paying. Before a
    /// first release is the cheapest moment this could ever be changed.
    ///
    /// This is also stricter than libpq's `require`, which encrypts without
    /// validating anything; `verify-ca` and `verify-full` collapse into it
    /// too, erring toward the safe side (`M14.27`).
    #[default]
    Require,
}

impl SslPolicy {
    /// Parse a libpq `sslmode` value.
    ///
    /// # Errors
    /// Returns an error for a value libpq defines but fhir-postgresql does not
    /// implement, rather than silently choosing a weaker mode.
    pub fn parse(s: &str) -> Result<Self, StoreError> {
        match s.trim().to_ascii_lowercase().as_str() {
            "disable" => Ok(Self::Disable),
            "prefer" | "allow" => Ok(Self::Prefer),
            "require" | "verify-ca" | "verify-full" => Ok(Self::Require),
            other => Err(StoreError::Other(format!(
                "unknown sslmode {other:?}; expected disable, prefer, require, \
                 verify-ca, or verify-full"
            ))),
        }
    }

    /// The policy from `PGSSLMODE`, or the default.
    ///
    /// # Errors
    /// Propagates a malformed `PGSSLMODE`.
    pub fn from_env() -> Result<Self, StoreError> {
        match std::env::var("PGSSLMODE") {
            Ok(v) => Self::parse(&v),
            Err(_) => Ok(Self::default()),
        }
    }

    /// Whether this policy leaves PHI on the wire in the clear.
    #[must_use]
    pub fn is_encrypted(self) -> bool {
        self == Self::Require
    }
}

/// Connection pool size, configurable rather than compiled in (spec O10.8).
///
/// An explicit value wins over `FHIR_POSTGRESQL_POOL_SIZE`, which wins over the
/// default: a flag the operator typed should not be silently overridden by an
/// environment variable they inherited.
fn pool_size(explicit: Option<usize>) -> usize {
    explicit
        .filter(|n| *n > 0)
        .or_else(|| {
            std::env::var("FHIR_POSTGRESQL_POOL_SIZE")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .filter(|n| *n > 0)
        })
        .unwrap_or(16)
}

/// Trust anchors for the database connection: `PGSSLROOTCERT` when set,
/// otherwise the platform store.
fn root_store() -> Result<rustls::RootCertStore, StoreError> {
    let mut roots = rustls::RootCertStore::empty();
    if let Ok(path) = std::env::var("PGSSLROOTCERT") {
        // Parsed via rustls-pki-types rather than rustls-pemfile: the latter
        // is unmaintained (RUSTSEC-2025-0134) and was only ever a thin
        // wrapper over this same code.
        use rustls::pki_types::CertificateDer;
        use rustls::pki_types::pem::PemObject;
        let mut added = 0usize;
        for cert in CertificateDer::pem_file_iter(&path)
            .map_err(|e| StoreError::Other(format!("PGSSLROOTCERT {path}: {e}")))?
        {
            let cert = cert.map_err(|e| StoreError::Other(format!("PGSSLROOTCERT {path}: {e}")))?;
            roots
                .add(cert)
                .map_err(|e| StoreError::Other(format!("PGSSLROOTCERT {path}: {e}")))?;
            added += 1;
        }
        if added == 0 {
            return Err(StoreError::Other(format!(
                "PGSSLROOTCERT {path} contains no certificates"
            )));
        }
        return Ok(roots);
    }
    let native = rustls_native_certs::load_native_certs();
    if native.certs.is_empty() {
        let first = native.errors.first().map_or_else(
            || "no certificates found".to_string(),
            std::string::ToString::to_string,
        );
        return Err(StoreError::Other(format!(
            "no platform trust anchors for the database connection: {first}; \
             set PGSSLROOTCERT or sslmode=disable"
        )));
    }
    roots.add_parsable_certificates(native.certs);
    Ok(roots)
}

/// Build a tokio-postgres config from the standard PG* environment
/// variables, or parse an explicit DSN.
pub fn pg_config(dsn: Option<&str>) -> Result<tokio_postgres::Config, StoreError> {
    if let Some(dsn) = dsn {
        return dsn
            .parse::<tokio_postgres::Config>()
            .map_err(StoreError::Pg);
    }
    let mut cfg = tokio_postgres::Config::new();
    let user = std::env::var("PGUSER")
        .ok()
        .or_else(|| std::env::var("USER").ok())
        .unwrap_or_else(|| "postgres".into());
    cfg.host(std::env::var("PGHOST").as_deref().unwrap_or("localhost"));
    cfg.port(
        std::env::var("PGPORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(5432),
    );
    cfg.dbname(std::env::var("PGDATABASE").as_deref().unwrap_or(&user));
    if let Ok(pw) = std::env::var("PGPASSWORD") {
        cfg.password(&pw);
    }
    cfg.user(&user);
    // Runaway statements must die server-side; overridable, never unset.
    let stmt_ms: u64 = std::env::var("FHIR_POSTGRESQL_STATEMENT_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30_000);
    cfg.options(format!("-c statement_timeout={stmt_ms}"));
    Ok(cfg)
}

impl Store {
    /// Connect using the SSL policy from `PGSSLMODE` (default `prefer`).
    pub async fn connect(
        cfg: tokio_postgres::Config,
        map: Arc<RelMap>,
    ) -> Result<Self, StoreError> {
        Self::connect_with(cfg, map, SslPolicy::from_env()?).await
    }

    /// Connect with an explicit SSL policy (spec O10.7).
    pub async fn connect_with(
        cfg: tokio_postgres::Config,
        map: Arc<RelMap>,
        ssl: SslPolicy,
    ) -> Result<Self, StoreError> {
        Self::connect_full(cfg, map, ssl, None).await
    }

    /// Connect with an explicit SSL policy and pool size (spec O10.7, O10.8).
    pub async fn connect_full(
        mut cfg: tokio_postgres::Config,
        map: Arc<RelMap>,
        ssl: SslPolicy,
        pool: Option<usize>,
    ) -> Result<Self, StoreError> {
        let mgr_cfg = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = match ssl {
            SslPolicy::Disable => {
                cfg.ssl_mode(tokio_postgres::config::SslMode::Disable);
                Manager::from_config(cfg, NoTls, mgr_cfg)
            }
            SslPolicy::Prefer | SslPolicy::Require => {
                cfg.ssl_mode(if ssl == SslPolicy::Require {
                    tokio_postgres::config::SslMode::Require
                } else {
                    tokio_postgres::config::SslMode::Prefer
                });
                let tls_cfg = rustls::ClientConfig::builder()
                    .with_root_certificates(root_store()?)
                    .with_no_client_auth();
                let connector = tokio_postgres_rustls::MakeRustlsConnect::new(tls_cfg);
                Manager::from_config(cfg, connector, mgr_cfg)
            }
        };
        // A bounded wait: exhaustion surfaces as 503 + Retry-After at the
        // API layer instead of queueing unboundedly (spec O10.3).
        let pool = Pool::builder(mgr)
            .max_size(pool_size(pool))
            .wait_timeout(Some(std::time::Duration::from_secs(2)))
            .runtime(deadpool_postgres::Runtime::Tokio1)
            .build()
            .map_err(|e| StoreError::Pool(e.to_string()))?;
        let keys = crate::chain::KeyRing::from_env("FHIR_POSTGRESQL").map_err(StoreError::Other)?;
        Ok(Store { pool, map, keys })
    }

    /// Replace the chain key ring, rather than taking it from the process
    /// environment.
    ///
    /// The environment is a deployment concern; a caller that wants to hold
    /// keys another way — a secrets manager, a test — should not have to
    /// mutate process-global state to do it. Mutating `FHIR_POSTGRESQL_CHAIN_KEY` is
    /// also not thread-safe, so tests that did so raced whatever else was
    /// running.
    #[must_use]
    pub fn with_chain_keys(mut self, keys: crate::chain::KeyRing) -> Self {
        self.keys = keys;
        self
    }

    /// How the tamper-evidence chain is being kept: the signing key's id, or
    /// `None` when unkeyed. Never exposes the key itself.
    #[must_use]
    pub fn chain_key_id(&self) -> Option<&str> {
        self.keys.signing().map(crate::chain::ChainKey::id)
    }

    pub fn map(&self) -> &RelMap {
        &self.map
    }

    fn rm(&self, rtype: &str) -> Result<&ResourceMap, StoreError> {
        self.map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Other(format!("unknown resource type {rtype:?}")))
    }

    /// Apply the generated DDL. Refuses to touch a schema installed from a
    /// different map; a schema installed from the same map is a no-op.
    pub async fn init(&self, checksum: &str) -> Result<bool, StoreError> {
        let mut client = self.pool.get().await?;
        let s = &self.map.schema;
        let existing = client
            .query_opt(
                &format!(
                    "SELECT \"value\" FROM \"{s}\".\"fhir_postgresql_meta\" WHERE \"key\" = 'map_checksum'"
                ),
                &[],
            )
            .await;
        if let Ok(Some(row)) = existing {
            let v: String = row.get(0);
            if v == checksum {
                return Ok(false);
            }
            return Err(StoreError::Other(format!(
                "schema {s} was installed from a different map (checksum {v}); refusing"
            )));
        }
        // Creating thousands of tables in one transaction would exhaust the
        // server's lock table, so stage the install under a temporary schema
        // in chunked transactions and rename it into place atomically.
        let staging = format!("{s}__init");
        client
            .batch_execute(&format!("DROP SCHEMA IF EXISTS \"{staging}\" CASCADE"))
            .await?;
        let statements = fhir_postgresql_map::ddl::ddl_in(&self.map, &staging);
        for chunk in statements.chunks(200) {
            let tx = client.transaction().await?;
            tx.batch_execute(&chunk.join(";\n")).await?;
            tx.commit().await?;
        }
        let asset_hex = hex_encode(
            &self
                .map
                .to_gz_bytes()
                .map_err(|e| StoreError::Other(e.to_string()))?,
        );
        client
            .execute(
                &format!(
                    "INSERT INTO \"{staging}\".\"fhir_postgresql_meta\" (\"key\", \"value\") \
                     VALUES ('map_checksum', $1), ('fhir_version', $2), ('map_asset', $3)"
                ),
                &[&checksum, &self.map.fhir_version.as_str(), &asset_hex],
            )
            .await?;
        client
            .batch_execute(&format!("ALTER SCHEMA \"{staging}\" RENAME TO \"{s}\""))
            .await?;
        Ok(true)
    }

    /// Upgrade an installed schema to this store's map: additive changes
    /// (new tables, new columns, new indexes) apply automatically;
    /// destructive ones (dropped tables/columns/indexes) require
    /// `allow_destructive`. Column type changes always refuse — those need
    /// a manual migration.
    pub async fn upgrade(
        &self,
        checksum: &str,
        allow_destructive: bool,
    ) -> Result<UpgradeReport, StoreError> {
        let s = &self.map.schema;
        let mut client = self.pool.get().await?;
        let old_hex: String = client
            .query_opt(
                &format!(
                    "SELECT \"value\" FROM \"{s}\".\"fhir_postgresql_meta\" WHERE \"key\" = 'map_asset'"
                ),
                &[],
            )
            .await
            .map_err(|_| StoreError::Other(format!("schema {s} is not installed")))?
            .ok_or_else(|| {
                StoreError::Other(
                    "installed schema predates upgrade support (no stored map asset)".into(),
                )
            })?
            .get(0);
        let old_bytes = hex_decode(&old_hex)?;
        let old_map = RelMap::from_gz_bytes(&old_bytes)
            .map_err(|e| StoreError::Other(format!("stored map asset unreadable: {e}")))?;

        // Diff tables and columns by name across all resources.
        use std::collections::HashMap;
        let mut adds: Vec<String> = Vec::new();
        // Objects the per-resource diff cannot see, because they are not in
        // the relational map: the access log, the append-only guard, and the
        // history audit envelope. Every statement is idempotent, so these are
        // *reconciled* rather than diffed — applied every time, counted
        // never, which keeps "a re-upgrade is a no-op" true.
        let mut reconcile: Vec<String> = fhir_postgresql_map::ddl::schema_wide_objects(s);
        for rm in self.map.resources.values() {
            if let Some((_, hist)) = rm.find_table(TableKind::History) {
                reconcile.extend(fhir_postgresql_map::ddl::history_audit_columns(
                    s, &hist.name,
                ));
                reconcile.push(fhir_postgresql_map::ddl::append_only_trigger(s, &hist.name));
            }
        }
        let mut destructive: Vec<String> = Vec::new();
        let mut old_tables: HashMap<&str, &fhir_postgresql_map::model::Table> = HashMap::new();
        for rm in old_map.resources.values() {
            for t in &rm.tables {
                old_tables.insert(t.name.as_str(), t);
            }
        }
        let mut new_names: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for rm in self.map.resources.values() {
            for t in &rm.tables {
                new_names.insert(t.name.as_str());
                match old_tables.get(t.name.as_str()) {
                    None => adds.push(fhir_postgresql_map::ddl::create_table(s, rm, t)),
                    Some(old_t) => {
                        let old_cols: HashMap<&str, ColTy> =
                            old_t.cols.iter().map(|c| (c.name.as_str(), c.ty)).collect();
                        let new_col_names: std::collections::HashSet<&str> =
                            t.cols.iter().map(|c| c.name.as_str()).collect();
                        for c in &t.cols {
                            match old_cols.get(c.name.as_str()) {
                                None => adds.push(format!(
                                    "ALTER TABLE \"{s}\".\"{}\" ADD COLUMN \"{}\" {}",
                                    t.name,
                                    c.name,
                                    fhir_postgresql_map::ddl::col_sql(c.ty)
                                )),
                                Some(old_ty) if *old_ty != c.ty => {
                                    return Err(StoreError::Other(format!(
                                        "column {}.{} changed type {:?} → {:?}; manual migration required",
                                        t.name, c.name, old_ty, c.ty
                                    )));
                                }
                                Some(_) => {}
                            }
                        }
                        for name in old_cols.keys() {
                            if !new_col_names.contains(name) {
                                destructive.push(format!(
                                    "ALTER TABLE \"{s}\".\"{}\" DROP COLUMN \"{name}\"",
                                    t.name
                                ));
                            }
                        }
                    }
                }
            }
        }
        for name in old_tables.keys() {
            if !new_names.contains(name) {
                destructive.push(format!("DROP TABLE \"{s}\".\"{name}\" CASCADE"));
            }
        }
        // O10.4b: a moved column is not a drop. A map change that relocates
        // an element between tables (G2.6a's force-split, F-90) reaches this
        // diff as an ADD plus a DROP, and `allow_destructive` was defined
        // for abandoning data, not relocating it. Refuse a data-bearing
        // move by name, independent of the flag; an empty source proceeds.
        // Checked before the destructive gate: "rerun with
        // --allow-destructive" is the wrong advice for a relocation.
        let moved = moved_columns(&old_map, &self.map);
        let mut data_bearing: Vec<String> = Vec::new();
        for (t, c, nt) in &moved {
            let row = client
                .query_one(
                    &format!(
                        "SELECT EXISTS(SELECT 1 FROM \"{s}\".\"{t}\" WHERE \"{c}\" IS NOT NULL)"
                    ),
                    &[],
                )
                .await?;
            if row.get::<_, bool>(0) {
                data_bearing.push(format!("{t}.{c} → {nt}"));
            }
        }
        if !data_bearing.is_empty() {
            return Err(StoreError::Other(format!(
                "upgrade refuses {} moved column(s) holding data (O10.4b, F-90): {}. \
                 --allow-destructive does not cover relocation; re-put the affected \
                 resource types through this artifact, or reload",
                data_bearing.len(),
                data_bearing.join(", ")
            )));
        }
        // Index diff by full statement text.
        let old_ix: std::collections::HashSet<String> = old_map
            .resources
            .values()
            .flat_map(|rm| fhir_postgresql_map::ddl::search_indexes(s, rm))
            .collect();
        for rm in self.map.resources.values() {
            for stmt in fhir_postgresql_map::ddl::search_indexes(s, rm) {
                if !old_ix.contains(&stmt) {
                    adds.push(stmt);
                }
            }
        }

        if !destructive.is_empty() && !allow_destructive {
            return Err(StoreError::Other(format!(
                "upgrade requires {} destructive change(s); rerun with --allow-destructive (first: {})",
                destructive.len(),
                destructive.first().expect("non-empty")
            )));
        }
        // Adds first: reconciliation touches history tables, and a resource
        // type new in this artifact has no tables until `adds` creates them.
        // Reconciling first would `ALTER TABLE` something that does not exist
        // yet.
        let all: Vec<&String> = adds
            .iter()
            .chain(reconcile.iter())
            .chain(destructive.iter())
            .collect();
        for chunk in all.chunks(100) {
            let tx = client.transaction().await?;
            let joined: Vec<String> = chunk.iter().map(|x| x.to_string()).collect();
            tx.batch_execute(&joined.join(";\n")).await?;
            tx.commit().await?;
        }
        let new_hex = hex_encode(
            &self
                .map
                .to_gz_bytes()
                .map_err(|e| StoreError::Other(e.to_string()))?,
        );
        client
            .execute(
                &format!(
                    "UPDATE \"{s}\".\"fhir_postgresql_meta\" SET \"value\" = CASE \"key\" \
                     WHEN 'map_checksum' THEN $1 WHEN 'map_asset' THEN $2 \
                     WHEN 'fhir_version' THEN $3 ELSE \"value\" END \
                     WHERE \"key\" IN ('map_checksum', 'map_asset', 'fhir_version')"
                ),
                &[&checksum, &new_hex, &self.map.fhir_version.as_str()],
            )
            .await?;
        let folded = self.backfill_norm(&mut client).await?;
        Ok(UpgradeReport {
            additive: adds.len(),
            destructive: destructive.len(),
            folded,
        })
    }

    /// Populate folded search columns (P6.6) for rows written before the
    /// column existed, returning how many values were folded.
    ///
    /// An upgrade that added the column would otherwise leave it NULL on every
    /// existing row, and a string search compares the folded column — so the
    /// resources would silently stop matching. Silent under-return is the one
    /// failure mode a clinical search must not have, so the backfill runs as
    /// part of the upgrade rather than as a step an operator can forget.
    ///
    /// Folds distinct *values* rather than rows (a surname repeats across
    /// patients), in bounded batches, and is resumable: each pass only looks
    /// at rows still NULL, so an interrupted run resumes where it stopped.
    async fn backfill_norm(
        &self,
        client: &mut deadpool_postgres::Client,
    ) -> Result<usize, StoreError> {
        const BATCH: usize = 1000;
        let s = &self.map.schema;
        let mut total = 0usize;
        for rm in self.map.resources.values() {
            for t in &rm.tables {
                for (src, dst) in &t.norm_cols {
                    let (tn, mut done) = (&t.name, false);
                    while !done {
                        let rows = client
                            .query(
                                &format!(
                                    "SELECT DISTINCT \"{src}\" FROM \"{s}\".\"{tn}\" \
                                     WHERE \"{dst}\" IS NULL AND \"{src}\" IS NOT NULL \
                                     LIMIT {BATCH}"
                                ),
                                &[],
                            )
                            .await?;
                        if rows.is_empty() {
                            done = true;
                            continue;
                        }
                        let vals: Vec<String> =
                            rows.iter().map(|r| r.get::<_, String>(0)).collect();
                        let folded: Vec<String> = vals
                            .iter()
                            .map(|v| fhir_postgresql_map::fold::fold(v))
                            .collect();
                        client
                            .execute(
                                &format!(
                                    "UPDATE \"{s}\".\"{tn}\" AS t SET \"{dst}\" = v.f \
                                     FROM (SELECT unnest($1::text[]) AS s, \
                                                  unnest($2::text[]) AS f) v \
                                     WHERE t.\"{src}\" = v.s AND t.\"{dst}\" IS NULL"
                                ),
                                &[&vals, &folded],
                            )
                            .await?;
                        total += vals.len();
                        done = vals.len() < BATCH;
                    }
                }
            }
        }
        Ok(total)
    }

    /// Remove this version's schema entirely (tables dropped in chunks to
    /// stay inside the server's lock budget). Destructive; caller confirms.
    pub async fn drop_schema(&self) -> Result<(), StoreError> {
        let client = self.pool.get().await?;
        for schema in [
            self.map.schema.clone(),
            format!("{}__init", self.map.schema),
        ] {
            let rows = client
                .query(
                    "SELECT tablename FROM pg_tables WHERE schemaname = $1",
                    &[&schema],
                )
                .await?;
            for chunk in rows.chunks(50) {
                let stmts: Vec<String> = chunk
                    .iter()
                    .map(|r| {
                        let t: String = r.get(0);
                        format!("DROP TABLE IF EXISTS \"{schema}\".\"{t}\" CASCADE")
                    })
                    .collect();
                client.batch_execute(&stmts.join(";\n")).await?;
            }
            client
                .batch_execute(&format!("DROP SCHEMA IF EXISTS \"{schema}\" CASCADE"))
                .await?;
        }
        Ok(())
    }

    /// Create-or-update one resource in a single transaction, appending
    /// history. The resource must carry an id.
    pub async fn put(&self, resource: &Value) -> Result<PutOutcome, StoreError> {
        self.put_if(resource, None).await
    }

    /// Like [`Store::put_if`], recording who is responsible (spec M3.15).
    pub async fn put_audited(
        &self,
        resource: &Value,
        expected_version: Option<i64>,
        audit: &Audit,
    ) -> Result<PutOutcome, StoreError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let out = self
            .put_in_audited(&tx, resource, expected_version, audit)
            .await?;
        tx.commit().await?;
        Ok(out)
    }

    /// Like [`Store::delete`], recording who is responsible.
    pub async fn delete_audited(
        &self,
        rtype: &str,
        id: &str,
        audit: &Audit,
    ) -> Result<bool, StoreError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let existed = self.delete_in_audited(&tx, rtype, id, audit).await?;
        tx.commit().await?;
        Ok(existed)
    }

    /// Like [`Store::put`], but honoring an If-Match expectation: the write
    /// only proceeds when the stored version equals `expected_version`
    /// (0 = "must not exist yet").
    pub async fn put_if(
        &self,
        resource: &Value,
        expected_version: Option<i64>,
    ) -> Result<PutOutcome, StoreError> {
        self.put_audited(resource, expected_version, &Audit::unattributed())
            .await
    }

    /// Run several writes as one all-or-nothing database transaction
    /// (FHIR transaction Bundles). Outcomes are returned in op order.
    pub async fn transact(&self, ops: &[TxOp]) -> Result<Vec<TxOutcome>, StoreError> {
        self.transact_audited(ops, &Audit::unattributed()).await
    }

    /// [`Store::transact`], attributing every entry in the bundle to one
    /// principal — a transaction is one act by one actor.
    pub async fn transact_audited(
        &self,
        ops: &[TxOp],
        audit: &Audit,
    ) -> Result<Vec<TxOutcome>, StoreError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let mut outcomes = Vec::with_capacity(ops.len());
        for op in ops {
            match op {
                TxOp::Put { resource, expected } => {
                    outcomes.push(TxOutcome::Put(
                        self.put_in_audited(&tx, resource, *expected, audit).await?,
                    ));
                }
                TxOp::Delete { rtype, id } => {
                    outcomes.push(TxOutcome::Delete(
                        self.delete_in_audited(&tx, rtype, id, audit).await?,
                    ));
                }
            }
        }
        tx.commit().await?;
        Ok(outcomes)
    }

    /// One create-or-update inside a caller-managed transaction.
    async fn put_in_audited(
        &self,
        tx: &tokio_postgres::Transaction<'_>,
        resource: &Value,
        expected_version: Option<i64>,
        audit: &Audit,
    ) -> Result<PutOutcome, StoreError> {
        let rtype = resource
            .get("resourceType")
            .and_then(Value::as_str)
            .ok_or_else(|| StoreError::Other("missing resourceType".into()))?
            .to_string();
        let rm = self.rm(&rtype)?;
        let out = shred(rm, resource)?;
        let id = out
            .id
            .clone()
            .ok_or_else(|| StoreError::Other("resource has no id".into()))?;
        let s = &self.map.schema;
        let base = &rm.base_table().name;
        let json = serde_json::to_string(resource).map_err(|e| StoreError::Other(e.to_string()))?;

        let old: Option<i64> = tx
            .query_opt(
                &format!(
                    "SELECT \"version_id\" FROM \"{s}\".\"{base}\" WHERE \"id\" = $1 FOR UPDATE"
                ),
                &[&id],
            )
            .await?
            .map(|r| r.get(0));
        if let Some(expected) = expected_version {
            let found = old.unwrap_or(0);
            if found != expected {
                return Err(StoreError::Conflict { expected, found });
            }
        }
        if old.is_some() {
            tx.execute(
                &format!("DELETE FROM \"{s}\".\"{base}\" WHERE \"id\" = $1"),
                &[&id],
            )
            .await?;
        }
        let hist = rm
            .find_table(TableKind::History)
            .expect("history table")
            .1
            .name
            .clone();
        // Version numbers continue past deletes, so derive from history (a
        // deleted id keeps its history rows but has no base row). The
        // history primary key backstops any create/create race on a
        // deleted id.
        let last_any: Option<i64> = tx
            .query_one(
                &format!("SELECT max(\"version_id\") FROM \"{s}\".\"{hist}\" WHERE \"id\" = $1"),
                &[&id],
            )
            .await?
            .get(0);
        let version = old.unwrap_or(0).max(last_any.unwrap_or(0)) + 1;
        insert_shredded(tx, &self.map, rm, &id, version, &out).await?;
        let op = if old.is_some() { "U" } else { "C" };
        append_history(
            tx,
            s,
            &hist,
            &id,
            version,
            op,
            Some(&json),
            audit,
            &self.keys,
        )
        .await?;
        Ok(PutOutcome {
            id,
            version_id: version,
            created: old.is_none(),
        })
    }

    /// Read the current version, reconstructed from the relational tables.
    ///
    /// The read spans one base table and every child table, so it MUST see a
    /// single snapshot (spec R4.5): a concurrent write between the statements
    /// would otherwise reconstruct a resource that never existed — base
    /// columns from one version, child rows from the next.
    pub async fn get(&self, rtype: &str, id: &str) -> Result<Option<Got>, StoreError> {
        let mut client = self.pool.get().await?;
        let tx = snapshot(&mut client).await?;
        let got = self.get_in(&tx, rtype, id).await?;
        // Read-only: commit and rollback are equivalent, and commit is the
        // cheaper signal that the snapshot is no longer needed.
        tx.commit().await?;
        Ok(got)
    }

    /// One reconstruction inside a caller-supplied snapshot. Callers that
    /// read several resources (search materialization, export) share one
    /// transaction so the whole page is consistent, not merely each row.
    pub async fn get_in(
        &self,
        tx: &tokio_postgres::Transaction<'_>,
        rtype: &str,
        id: &str,
    ) -> Result<Option<Got>, StoreError> {
        let rm = self.rm(rtype)?;
        let s = &self.map.schema;
        let client = tx;

        let base = rm.base_table();
        let base_cols: String = base
            .cols
            .iter()
            .map(|c| format!(", \"{}\"::text", c.name))
            .collect();
        let row = client
            .query_opt(
                &format!(
                    "SELECT \"version_id\"{base_cols} FROM \"{s}\".\"{}\" WHERE \"id\" = $1",
                    base.name
                ),
                &[&id],
            )
            .await?;
        let Some(brow) = row else { return Ok(None) };
        let version_id: i64 = brow.get(0);

        let mut input = ReconIn {
            tables: vec![Vec::new(); rm.tables.len()],
            ..Default::default()
        };
        let mut bcols = std::collections::HashMap::new();
        for (i, c) in base.cols.iter().enumerate() {
            if let Some(v) = brow.get::<_, Option<String>>(i + 1) {
                bcols.insert(c.name.clone(), v);
            }
        }
        input.tables[0].push(InRow {
            ords: Vec::new(),
            cols: bcols,
        });

        // Pipeline all child-table reads on one connection.
        let client = &client;
        let mut futs = Vec::new();
        for (ti, t) in rm.tables.iter().enumerate() {
            if ti == 0 {
                continue;
            }
            let sql = match t.kind {
                TableKind::Elem => {
                    let cols: String = t
                        .cols
                        .iter()
                        .map(|c| format!(", \"{}\"::text", c.name))
                        .collect();
                    format!(
                        "SELECT \"ords\"::text{cols} FROM \"{s}\".\"{}\" WHERE \"rid\" = $1",
                        t.name
                    )
                }
                TableKind::Ext => format!(
                    "SELECT \"path\", \"ords\"::text, \"modifier\", \"ext_ord\", \"url\", \"leaf\", \
                     \"v_kind\", coalesce(\"v_text\", \"v_num\"::text, \"v_bool\"::text) \
                     FROM \"{s}\".\"{}\" WHERE \"rid\" = $1",
                    t.name
                ),
                TableKind::Deep => format!(
                    "SELECT \"path\", \"ords\"::text, \"leaf\", \
                     \"v_kind\", coalesce(\"v_text\", \"v_num\"::text, \"v_bool\"::text) \
                     FROM \"{s}\".\"{}\" WHERE \"rid\" = $1",
                    t.name
                ),
                TableKind::Contained => format!(
                    "SELECT \"ord\", \"resource\"::text FROM \"{s}\".\"{}\" WHERE \"rid\" = $1",
                    t.name
                ),
                TableKind::Base | TableKind::History => continue,
            };
            futs.push(async move {
                let rows = client.query(&sql, &[&id]).await?;
                Ok::<_, tokio_postgres::Error>((ti, rows))
            });
        }
        let results = futures_join_all(futs).await;
        for res in results {
            let (ti, rows) = res?;
            let t = &rm.tables[ti];
            match t.kind {
                TableKind::Elem => {
                    for r in rows {
                        let ords = parse_ords(r.get::<_, String>(0).as_str())?;
                        let mut cols = std::collections::HashMap::new();
                        for (i, c) in t.cols.iter().enumerate() {
                            if let Some(v) = r.get::<_, Option<String>>(i + 1) {
                                cols.insert(c.name.clone(), v);
                            }
                        }
                        input.tables[ti].push(InRow { ords, cols });
                    }
                }
                TableKind::Ext => {
                    for r in rows {
                        let kind: String = r.get(6);
                        let text: Option<String> = r.get(7);
                        input.ext.push(ExtRow {
                            path: r.get(0),
                            ords: parse_ords(r.get::<_, String>(1).as_str())?,
                            modifier: r.get(2),
                            ext_ord: r.get(3),
                            url: r.get(4),
                            leaf: r.get(5),
                            val: LeafVal::from_cols(&kind, text.as_deref())?,
                        });
                    }
                }
                TableKind::Deep => {
                    for r in rows {
                        let kind: String = r.get(3);
                        let text: Option<String> = r.get(4);
                        input.deep.push(DeepRow {
                            path: r.get(0),
                            ords: parse_ords(r.get::<_, String>(1).as_str())?,
                            leaf: r.get(2),
                            val: LeafVal::from_cols(&kind, text.as_deref())?,
                        });
                    }
                }
                TableKind::Contained => {
                    for r in rows {
                        let ord: i16 = r.get(0);
                        let text: String = r.get(1);
                        let v: Value = serde_json::from_str(&text)
                            .map_err(|e| StoreError::Other(e.to_string()))?;
                        input.contained.push((ord, v));
                    }
                }
                _ => unreachable!(),
            }
        }

        let resource = reconstruct(rm, &input, Some(id))?;
        Ok(Some(Got {
            resource,
            version_id,
        }))
    }

    /// Reconstruct several resources in **one** snapshot (spec R4.5).
    ///
    /// Search and export materialize a whole page this way, so the page is
    /// internally consistent rather than merely each row being consistent
    /// with itself. Results are returned in the order asked for; `None` means
    /// the id was absent from the snapshot (a legal outcome for a search hit
    /// deleted between the id query and materialization, and for a dangling
    /// `_include` reference).
    pub async fn get_all(
        &self,
        items: &[(String, String)],
    ) -> Result<Vec<Option<Got>>, StoreError> {
        if items.is_empty() {
            return Ok(Vec::new());
        }
        let mut client = self.pool.get().await?;
        let tx = snapshot(&mut client).await?;
        let mut out = Vec::with_capacity(items.len());
        for (rtype, id) in items {
            out.push(self.get_in(&tx, rtype, id).await?);
        }
        tx.commit().await?;
        Ok(out)
    }

    /// Whether an id is active, deleted, or unknown — the read path's 404
    /// vs 410 distinction.
    pub async fn status(&self, rtype: &str, id: &str) -> Result<ResourceStatus, StoreError> {
        let rm = self.rm(rtype)?;
        let s = &self.map.schema;
        let base = &rm.base_table().name;
        let hist = &rm.find_table(TableKind::History).expect("history").1.name;
        // Base and history are two statements: without one snapshot, a delete
        // landing between them reports Unknown (404) for a resource whose
        // history says Deleted (410).
        let mut client = self.pool.get().await?;
        let client = snapshot(&mut client).await?;
        if let Some(row) = client
            .query_opt(
                &format!("SELECT \"version_id\" FROM \"{s}\".\"{base}\" WHERE \"id\" = $1"),
                &[&id],
            )
            .await?
        {
            return Ok(ResourceStatus::Active(row.get(0)));
        }
        let last: Option<i64> = client
            .query_one(
                &format!("SELECT max(\"version_id\") FROM \"{s}\".\"{hist}\" WHERE \"id\" = $1"),
                &[&id],
            )
            .await?
            .get(0);
        Ok(match last {
            Some(v) => ResourceStatus::Deleted(v),
            None => ResourceStatus::Unknown,
        })
    }

    /// One historical version, straight from the history archive.
    /// `resource` is None for delete markers.
    pub async fn vread(
        &self,
        rtype: &str,
        id: &str,
        version_id: i64,
    ) -> Result<Option<HistEntry>, StoreError> {
        let rm = self.rm(rtype)?;
        let s = &self.map.schema;
        let hist = &rm.find_table(TableKind::History).expect("history").1.name;
        let client = self.pool.get().await?;
        let row = client
            .query_opt(
                &format!(
                    "SELECT \"version_id\", \"last_updated\"::text, \"op\", \"resource\"::text \
                     FROM \"{s}\".\"{hist}\" WHERE \"id\" = $1 AND \"version_id\" = $2"
                ),
                &[&id, &version_id],
            )
            .await?;
        row.map(hist_entry).transpose()
    }

    /// The full history of one id, newest first.
    /// The stored `map_checksum`, or `None` when the schema (or its meta
    /// table) is not installed. The mount probe `fhir-loco` uses: `Some`
    /// means "installed, serve it", `None` means "do not mount".
    pub async fn installed_checksum(&self) -> Result<Option<String>, StoreError> {
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        let reg: Option<String> = client
            .query_one(
                "SELECT to_regclass($1)::text",
                &[&format!("\"{s}\".\"fhir_postgresql_meta\"")],
            )
            .await?
            .get(0);
        if reg.is_none() {
            return Ok(None);
        }
        let row = client
            .query_opt(
                &format!(
                    "SELECT \"value\" FROM \"{s}\".\"fhir_postgresql_meta\" \
                     WHERE \"key\" = 'map_checksum'"
                ),
                &[],
            )
            .await?;
        Ok(row.map(|r| r.get(0)))
    }

    /// History across one type, or every mapped type (`rtype` `None`),
    /// newest first — the store half of type-/system-level `_history`
    /// (`fhir-loco`'s `SV2.17`), same semantics as `fhir-sqlite`'s.
    ///
    /// Returns at most `count` `(rtype, id, entry)` rows, ordered by
    /// `last_updated` then `version_id`, both descending. `since` keeps
    /// versions written **at or after** that instant, cast by the engine
    /// (`$1::timestamptz`), so an unparseable instant errors rather than
    /// comparing as text. Entries merge across per-type queries by the
    /// text image of `last_updated` — one session, one time zone, one
    /// format, so text order is time order. No cursor: the newest `count`
    /// entries, an honest slice rather than an approximate page.
    pub async fn history_page(
        &self,
        rtype: Option<&str>,
        count: i64,
        since: Option<&str>,
    ) -> Result<Vec<(String, String, HistEntry)>, StoreError> {
        let types: Vec<&str> = match rtype {
            Some(t) => {
                // Validate up front so an unknown type refuses by name.
                let _ = self.rm(t)?;
                vec![t]
            }
            None => self.map.resources.keys().map(String::as_str).collect(),
        };
        let count = count.clamp(1, 1000);
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        let mut out: Vec<(String, String, HistEntry)> = Vec::new();
        for t in types {
            let rm = self.rm(t)?;
            let hist = &rm.find_table(TableKind::History).expect("history").1.name;
            let filter = if since.is_some() {
                " WHERE \"last_updated\" >= ($1::text)::timestamptz"
            } else {
                ""
            };
            let sql = format!(
                "SELECT \"id\", \"version_id\", \"last_updated\"::text, \"op\", \
                 \"resource\"::text FROM \"{s}\".\"{hist}\"{filter} \
                 ORDER BY \"last_updated\" DESC, \"version_id\" DESC LIMIT {count}"
            );
            let rows = match since {
                Some(v) => client.query(&sql, &[&v]).await?,
                None => client.query(&sql, &[]).await?,
            };
            for row in rows {
                let id: String = row.get(0);
                let version_id: i64 = row.get(1);
                let last_updated: String = row.get(2);
                let op: String = row.get(3);
                let raw: Option<String> = row.get(4);
                let resource = match raw {
                    Some(txt) => Some(serde_json::from_str(&txt).map_err(|e| {
                        StoreError::Other(format!("history {t}/{id}/{version_id}: {e}"))
                    })?),
                    None => None,
                };
                out.push((
                    t.to_string(),
                    id,
                    HistEntry {
                        version_id,
                        last_updated,
                        op: op.chars().next().unwrap_or('?'),
                        resource,
                    },
                ));
            }
        }
        out.sort_by(|a, b| {
            (b.2.last_updated.as_str(), b.2.version_id)
                .cmp(&(a.2.last_updated.as_str(), a.2.version_id))
        });
        out.truncate(usize::try_from(count).unwrap_or(usize::MAX));
        Ok(out)
    }

    pub async fn history(&self, rtype: &str, id: &str) -> Result<Vec<HistEntry>, StoreError> {
        let rm = self.rm(rtype)?;
        let s = &self.map.schema;
        let hist = &rm.find_table(TableKind::History).expect("history").1.name;
        let client = self.pool.get().await?;
        let rows = client
            .query(
                &format!(
                    "SELECT \"version_id\", \"last_updated\"::text, \"op\", \"resource\"::text \
                     FROM \"{s}\".\"{hist}\" WHERE \"id\" = $1 ORDER BY \"version_id\" DESC"
                ),
                &[&id],
            )
            .await?;
        rows.into_iter().map(hist_entry).collect()
    }

    /// Conditional create (`If-None-Exist`), atomic against concurrent
    /// requests with the same criteria (spec A7.10).
    ///
    /// Searching and then writing in two steps is a race: two concurrent
    /// conditional creates with identical criteria both find nothing and both
    /// create, which is how a patient ends up in the chart twice. The
    /// criteria are hashed into a transaction-scoped advisory lock, so
    /// same-criteria requests serialize while unrelated ones proceed freely,
    /// and the match and the write share one transaction.
    pub async fn conditional_create(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
        resource: &Value,
    ) -> Result<CondCreate, StoreError> {
        self.conditional_create_audited(rtype, criteria, resource, &Audit::unattributed())
            .await
    }

    /// [`Store::conditional_create`], recording who is responsible.
    pub async fn conditional_create_audited(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
        resource: &Value,
        audit: &Audit,
    ) -> Result<CondCreate, StoreError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let ids = self.locked_match(&tx, rtype, criteria).await?;
        let out = match ids.len() {
            0 => CondCreate::Created(self.put_in_audited(&tx, resource, Some(0), audit).await?),
            1 => CondCreate::Existing(ids.into_iter().next().expect("one")),
            _ => CondCreate::Multiple,
        };
        tx.commit().await?;
        Ok(out)
    }

    /// Conditional delete, atomic on the same terms as
    /// [`Store::conditional_create`].
    pub async fn conditional_delete(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
    ) -> Result<CondDelete, StoreError> {
        self.conditional_delete_audited(rtype, criteria, &Audit::unattributed())
            .await
    }

    /// [`Store::conditional_delete`], recording who is responsible.
    pub async fn conditional_delete_audited(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
        audit: &Audit,
    ) -> Result<CondDelete, StoreError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let ids = self.locked_match(&tx, rtype, criteria).await?;
        let out = match ids.len() {
            0 => CondDelete::NoMatch,
            1 => {
                self.delete_in_audited(&tx, rtype, &ids[0], audit).await?;
                CondDelete::Deleted
            }
            _ => CondDelete::Multiple,
        };
        tx.commit().await?;
        Ok(out)
    }

    /// Take the criteria lock, then match — inside the caller's transaction.
    /// At most two ids are fetched: the interactions only distinguish none,
    /// one, and more than one.
    async fn locked_match(
        &self,
        tx: &tokio_postgres::Transaction<'_>,
        rtype: &str,
        criteria: &[(String, String)],
    ) -> Result<Vec<String>, StoreError> {
        let rm = self.rm(rtype)?;
        let q = search::build_search_sql(&self.map, rm, criteria, 2, 0, &[], None)?;
        tx.execute(
            "SELECT pg_advisory_xact_lock($1)",
            &[&criteria_lock_key(&self.map.schema, rtype, criteria)],
        )
        .await?;
        let refs: Vec<&(dyn ToSql + Sync)> =
            q.binds.iter().map(|b| b as &(dyn ToSql + Sync)).collect();
        Ok(tx
            .query(&q.sql, &refs)
            .await?
            .iter()
            .map(|r| r.get(0))
            .collect())
    }

    /// Delete: removes current rows, appends a delete marker to history.
    pub async fn delete(&self, rtype: &str, id: &str) -> Result<bool, StoreError> {
        self.delete_audited(rtype, id, &Audit::unattributed()).await
    }

    async fn delete_in_audited(
        &self,
        tx: &tokio_postgres::Transaction<'_>,
        rtype: &str,
        id: &str,
        audit: &Audit,
    ) -> Result<bool, StoreError> {
        let rm = self.rm(rtype)?;
        let s = &self.map.schema;
        let base = &rm.base_table().name;
        let hist = rm
            .find_table(TableKind::History)
            .expect("history")
            .1
            .name
            .clone();
        let old: Option<i64> = tx
            .query_opt(
                &format!(
                    "SELECT \"version_id\" FROM \"{s}\".\"{base}\" WHERE \"id\" = $1 FOR UPDATE"
                ),
                &[&id],
            )
            .await?
            .map(|r| r.get(0));
        let Some(old) = old else {
            return Ok(false);
        };
        tx.execute(
            &format!("DELETE FROM \"{s}\".\"{base}\" WHERE \"id\" = $1"),
            &[&id],
        )
        .await?;
        let version = old + 1;
        append_history(tx, s, &hist, id, version, "D", None, audit, &self.keys).await?;
        Ok(true)
    }

    /// Execute a search over compiled parameters. `params` are raw
    /// (name-or-name:modifier, value) pairs; returns matching ids in
    /// id order.
    pub async fn search(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
    ) -> Result<Vec<String>, StoreError> {
        Ok(self
            .search_full(rtype, params, count, offset, &[], false)
            .await?
            .ids)
    }

    /// Search with sort keys and an optional accurate total.
    pub async fn search_full(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        sort: &[search::SortKey],
        want_total: bool,
    ) -> Result<SearchOutcome, StoreError> {
        self.search_page(rtype, params, count, offset, sort, want_total, None)
            .await
    }

    /// Search with an optional keyset cursor (`after_id`) for stable
    /// paging under the default id ordering.
    #[allow(clippy::too_many_arguments)]
    pub async fn search_page(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        sort: &[search::SortKey],
        want_total: bool,
        after_id: Option<&str>,
    ) -> Result<SearchOutcome, StoreError> {
        let rm = self.rm(rtype)?;
        let q = search::build_search_sql(&self.map, rm, params, count, offset, sort, after_id)?;
        // Page and count are two statements; one snapshot keeps `_total`
        // consistent with the page it describes (spec R4.5).
        let mut client = self.pool.get().await?;
        let client = snapshot(&mut client).await?;
        let refs: Vec<&(dyn ToSql + Sync)> =
            q.binds.iter().map(|b| b as &(dyn ToSql + Sync)).collect();
        let rows = client.query(&q.sql, &refs).await?;
        let ids = rows.iter().map(|r| r.get(0)).collect();
        let total = if want_total {
            // The count query shares only the WHERE binds.
            Some(
                client
                    .query_one(&q.count_sql, &refs[..q.count_binds])
                    .await?
                    .get(0),
            )
        } else {
            None
        };
        Ok(SearchOutcome { ids, total })
    }

    /// Append one disclosure record (spec PR12.5).
    ///
    /// Reads are the interactions an audit asks about first, and they leave
    /// no other trace: nothing in the resource changes when someone looks at
    /// it.
    pub async fn log_access(&self, rec: &AccessRecord) -> Result<(), StoreError> {
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        client
            .execute(
                &format!(
                    "INSERT INTO \"{s}\".\"fhir_postgresql_access_log\" \
                       (\"request_id\", \"actor\", \"actor_source\", \"client\", \
                        \"interaction\", \"rtype\", \"id\", \"version_id\", \
                        \"outcome\", \"result_count\", \"reason\") \
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
                ),
                &[
                    &rec.audit.request_id,
                    &rec.audit.actor,
                    &rec.audit.actor_source,
                    &rec.audit.client,
                    &rec.interaction,
                    &rec.rtype,
                    &rec.id,
                    &rec.version_id,
                    &rec.outcome,
                    &rec.result_count,
                    &rec.audit.reason,
                ],
            )
            .await?;
        Ok(())
    }

    /// Append many access records in one statement (PR12.6).
    ///
    /// One `INSERT` per disclosure costs a pool connection and a round trip on
    /// the read path, which is the price of the synchronous mode. Batching
    /// amortizes that: the arrays are unnested server-side, so a hundred
    /// records cost one round trip instead of a hundred.
    ///
    /// All-or-nothing by construction — a single statement either appends
    /// every record or none — so a partially written batch cannot leave the
    /// log claiming fewer disclosures than happened.
    pub async fn log_access_batch(&self, recs: &[AccessRecord]) -> Result<(), StoreError> {
        if recs.is_empty() {
            return Ok(());
        }
        let s = &self.map.schema;
        let request_id: Vec<Option<&str>> =
            recs.iter().map(|r| r.audit.request_id.as_deref()).collect();
        let actor: Vec<&str> = recs.iter().map(|r| r.audit.actor.as_str()).collect();
        let actor_source: Vec<Option<&str>> = recs
            .iter()
            .map(|r| r.audit.actor_source.as_deref())
            .collect();
        let client: Vec<Option<&str>> = recs.iter().map(|r| r.audit.client.as_deref()).collect();
        let interaction: Vec<&str> = recs.iter().map(|r| r.interaction.as_str()).collect();
        let rtype: Vec<Option<&str>> = recs.iter().map(|r| r.rtype.as_deref()).collect();
        let id: Vec<Option<&str>> = recs.iter().map(|r| r.id.as_deref()).collect();
        let version_id: Vec<Option<i64>> = recs.iter().map(|r| r.version_id).collect();
        let outcome: Vec<&str> = recs.iter().map(|r| r.outcome.as_str()).collect();
        let result_count: Vec<Option<i64>> = recs.iter().map(|r| r.result_count).collect();
        let reason: Vec<Option<&str>> = recs.iter().map(|r| r.audit.reason.as_deref()).collect();
        let client_conn = self.pool.get().await?;
        client_conn
            .execute(
                &format!(
                    "INSERT INTO \"{s}\".\"fhir_postgresql_access_log\" \
                       (\"request_id\", \"actor\", \"actor_source\", \"client\", \
                        \"interaction\", \"rtype\", \"id\", \"version_id\", \
                        \"outcome\", \"result_count\", \"reason\") \
                     SELECT * FROM unnest($1::text[], $2::text[], $3::text[], \
                       $4::text[], $5::text[], $6::text[], $7::text[], \
                       $8::bigint[], $9::text[], $10::bigint[], $11::text[])"
                ),
                &[
                    &request_id,
                    &actor,
                    &actor_source,
                    &client,
                    &interaction,
                    &rtype,
                    &id,
                    &version_id,
                    &outcome,
                    &result_count,
                    &reason,
                ],
            )
            .await?;
        Ok(())
    }

    /// Erase one resource and its history (GDPR Art. 17, spec M3.18).
    ///
    /// This is the one sanctioned exception to append-only history, and it is
    /// explicit rather than quiet: the resource's history rows are removed and
    /// replaced by a single tombstone recording who erased it, when, why, and
    /// the `row_hash` the chain ended on. An erased record therefore leaves a
    /// *verifiable hole* — `verify-audit` can still show that a chain existed
    /// and was deliberately terminated — rather than looking like a chain that
    /// never happened.
    ///
    /// What this cannot do is un-say the data: backups, replicas, and WAL
    /// archives still hold it until they age out. The book says so plainly;
    /// a deployment promising erasure has to mean the whole estate.
    pub async fn purge(
        &self,
        rtype: &str,
        id: &str,
        audit: &Audit,
    ) -> Result<PurgeReport, StoreError> {
        let rm = self.rm(rtype)?;
        let s = &self.map.schema;
        let base = &rm.base_table().name;
        let hist = rm
            .find_table(TableKind::History)
            .expect("history")
            .1
            .name
            .clone();
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        // Scoped to this transaction: the guard refuses history DELETEs
        // everywhere else (M3.17).
        tx.batch_execute("SET LOCAL fhir_postgresql.erasure = 'on'")
            .await?;

        let last = tx
            .query_opt(
                &format!(
                    "SELECT \"version_id\", \"row_hash\", \"row_hash_sha3\" \
                     FROM \"{s}\".\"{hist}\" \
                     WHERE \"id\" = $1 ORDER BY \"version_id\" DESC LIMIT 1"
                ),
                &[&id],
            )
            .await?;
        let Some(last) = last else {
            return Ok(PurgeReport {
                versions_erased: 0,
                existed: false,
            });
        };
        let last_version: i64 = last.get(0);
        let terminated_hash: Option<Vec<u8>> = last.get(1);
        // Both chains are terminated, not just SHA-256. A tombstone that
        // recorded one and left the other null would leave the second chain
        // with a hole at exactly the point an auditor looks hardest.
        let terminated_sha3: Option<Vec<u8>> = last.get(2);

        // Current rows first: the child tables cascade from the base row.
        tx.execute(
            &format!("DELETE FROM \"{s}\".\"{base}\" WHERE \"id\" = $1"),
            &[&id],
        )
        .await?;
        let erased = tx
            .execute(
                &format!("DELETE FROM \"{s}\".\"{hist}\" WHERE \"id\" = $1"),
                &[&id],
            )
            .await?;

        // The tombstone: op 'X', no resource, terminating both chains and
        // carrying its own keyed tag, so an erasure is itself attested.
        let ts_utc: String = tx
            .query_one(
                "SELECT to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS.US')",
                &[],
            )
            .await?
            .get(0);
        let tomb_version = last_version + 1;
        let pre = crate::chain::preimage(id, tomb_version, &ts_utc, "X", None, &audit.actor);
        let (tomb_256, tomb_3) =
            crate::chain::link(terminated_hash.as_deref(), terminated_sha3.as_deref(), &pre);
        let tomb_mac = self
            .keys
            .signing()
            .map(|k| crate::chain::mac(k, terminated_hash.as_deref(), &pre));
        tx.execute(
            &format!(
                "INSERT INTO \"{s}\".\"{hist}\" \
                   (\"id\", \"version_id\", \"last_updated\", \"op\", \"resource\", \
                    \"actor\", \"actor_source\", \"client\", \"request_id\", \"reason\", \
                    \"prev_hash\", \"row_hash\", \"prev_hash_sha3\", \"row_hash_sha3\", \
                    \"row_mac\") \
                 VALUES ($1::text, $2::bigint, now(), 'X', NULL, \
                         $3::text, $4::text, $5::text, $6::text, $7::text, \
                         $8::bytea, $9::bytea, $10::bytea, $11::bytea, $12::text)"
            ),
            &[
                &id,
                &tomb_version,
                &audit.actor,
                &audit.actor_source,
                &audit.client,
                &audit.request_id,
                &audit.reason,
                &terminated_hash,
                &tomb_256,
                &terminated_sha3,
                &tomb_3,
                &tomb_mac,
            ],
        )
        .await?;
        tx.commit().await?;
        tracing::warn!(
            rtype,
            id,
            actor = %audit.actor,
            reason = audit.reason.as_deref().unwrap_or("(none)"),
            versions = erased,
            "erased a resource and its history (GDPR Art. 17)"
        );
        // Checkpoint immediately after the one sanctioned deletion (M3.16c).
        // A witness taken here is what separates a recorded, intentional
        // removal from the unrecorded kind: without it, both look alike to
        // anyone comparing checkpoints later.
        self.emit_checkpoint("after-erasure").await;
        Ok(PurgeReport {
            versions_erased: erased,
            existed: true,
        })
    }

    /// Recompute every history hash chain and report the first break in each
    /// (spec M3.16).
    ///
    /// Recomputation happens in SQL with the same expression the writer used,
    /// so this checks the stored bytes rather than a Rust-side idea of them.
    /// Rows written before the audit columns existed carry a null `row_hash`;
    /// they are reported as the point the chain begins, not as tampering —
    /// claiming a break where there is only history would train an operator
    /// to ignore the report.
    /// A digest over every chain head in the schema — the witness value
    /// (spec M3.16b).
    ///
    /// Record this somewhere the database cannot reach. The MAC stops a row
    /// being *rewritten*, but an attacker with SQL write access can still
    /// delete rows wholesale, and a chain that no longer contains a version
    /// cannot report its absence. Comparing today's witness against one
    /// recorded yesterday is what makes truncation and mass deletion visible.
    ///
    /// The value covers `(resource type, id, last version, its two digests)`
    /// for every chain, ordered, so it changes if any chain loses a version,
    /// gains one, or has its head altered. Keyed when a key is configured, so
    /// the witness itself cannot be recomputed by whoever holds only the data.
    pub async fn chain_witness(&self) -> Result<String, StoreError> {
        use sha2::{Digest as _, Sha256};
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        let mut acc = Sha256::new();
        let mut chains = 0u64;
        for rm in self.map.resources.values() {
            let Some((_, hist)) = rm.find_table(TableKind::History) else {
                continue;
            };
            let rows = client
                .query(
                    &format!(
                        "SELECT DISTINCT ON (\"id\") \"id\", \"version_id\", \
                                \"row_hash\", \"row_hash_sha3\" \
                         FROM \"{s}\".\"{}\" \
                         ORDER BY \"id\", \"version_id\" DESC",
                        hist.name
                    ),
                    &[],
                )
                .await?;
            for row in rows {
                let id: String = row.get(0);
                let version: i64 = row.get(1);
                let h256: Option<Vec<u8>> = row.get(2);
                let h3: Option<Vec<u8>> = row.get(3);
                acc.update(rm.name.as_bytes());
                acc.update(b"|");
                acc.update(id.as_bytes());
                acc.update(b"|");
                acc.update(version.to_string().as_bytes());
                acc.update(b"|");
                acc.update(h256.unwrap_or_default());
                acc.update(b"|");
                acc.update(h3.unwrap_or_default());
                acc.update(b"\n");
                chains += 1;
            }
        }
        let digest = acc.finalize();
        let body = format!("{chains}:{}", hex_encode(&digest));
        Ok(match self.keys.signing() {
            Some(k) => crate::chain::mac(k, None, body.as_bytes()),
            None => body,
        })
    }

    /// Counter-sign every history row under the current signing key
    /// (spec M3.16d), returning how many rows were signed.
    ///
    /// For retiring a key you can no longer trust — a suspected compromise —
    /// where keeping it loadable is not an option. Rotation alone is
    /// additive and needs none of this: old rows stay verifiable as long as
    /// the old key is kept.
    ///
    /// **Verification runs first, and any finding aborts the whole
    /// operation.** Re-signing rows that do not currently verify would
    /// launder forged history into the new key's authority, which is the
    /// one thing this must never do. It is a single transaction, so a
    /// partial re-signing cannot be left behind either.
    ///
    /// Counter-signatures are appended, never written over `row_mac`. The
    /// original tag is evidence: replacing it would destroy the record of
    /// what the retired key attested, and leave no way to distinguish a
    /// legitimate re-signing from a forged one.
    ///
    /// # Errors
    /// If no key is configured, if any chain fails verification, or on a
    /// database error.
    pub async fn resign_history(&self, audit: &Audit, reason: &str) -> Result<u64, StoreError> {
        let Some(key) = self.keys.signing() else {
            return Err(StoreError::Other(
                "re-signing needs a signing key; pass --chain-key-file".into(),
            ));
        };
        let breaks = self.verify_audit().await?;
        if !breaks.is_empty() {
            return Err(StoreError::Other(format!(
                "refusing to re-sign: {} chain break(s) found, first {}/{} version {} [{}]: {}. \
                 Re-signing unverified history would give forged rows the new key's authority.",
                breaks.len(),
                breaks[0].rtype,
                breaks[0].id,
                breaks[0].version_id,
                breaks[0].algorithm,
                breaks[0].detail
            )));
        }

        let s = &self.map.schema;
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let mut signed = 0u64;
        for rm in self.map.resources.values() {
            let Some((_, hist)) = rm.find_table(TableKind::History) else {
                continue;
            };
            let rows = tx
                .query(
                    &format!(
                        "SELECT \"id\", \"version_id\", \
                                to_char(\"last_updated\" AT TIME ZONE 'UTC', \
                                        'YYYY-MM-DD HH24:MI:SS.US'), \
                                \"op\", (\"resource\")::text, \"actor\", \"prev_hash\" \
                         FROM \"{s}\".\"{}\" ORDER BY \"id\", \"version_id\"",
                        hist.name
                    ),
                    &[],
                )
                .await?;
            for row in rows {
                let id: String = row.get(0);
                let version_id: i64 = row.get(1);
                let ts_utc: String = row.get(2);
                let op: String = row.get(3);
                let resource: Option<String> = row.get(4);
                let actor: String = row.get(5);
                let prev: Option<Vec<u8>> = row.get(6);
                let pre = crate::chain::preimage(
                    &id,
                    version_id,
                    &ts_utc,
                    &op,
                    resource.as_deref(),
                    &actor,
                );
                let tag = crate::chain::mac(key, prev.as_deref(), &pre);
                tx.execute(
                    &format!(
                        "INSERT INTO \"{s}\".\"fhir_postgresql_countersign\" \
                           (\"rtype\", \"id\", \"version_id\", \"row_mac\", \"actor\", \"reason\") \
                         VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING"
                    ),
                    &[&rm.name, &id, &version_id, &tag, &audit.actor, &reason],
                )
                .await?;
                signed += 1;
            }
        }
        tx.commit().await?;
        tracing::warn!(
            key_id = key.id(),
            actor = %audit.actor,
            %reason,
            rows = signed,
            "counter-signed history under a new key"
        );
        self.emit_checkpoint("after-resign").await;
        Ok(signed)
    }

    /// Compute a checkpoint and log it on the `audit_checkpoint` target
    /// (spec M3.16c).
    ///
    /// A deployment already shipping logs gets an external witness for free.
    /// The dedicated target lets an operator route and retain these on their
    /// own schedule, and the line carries only counts and digests — no PHI —
    /// so it may be kept far longer than ordinary application logs, and
    /// somewhere patient data must not go.
    ///
    /// This is only a witness if the logs land where the database cannot
    /// reach. Logs shipped off-host qualify; a log table in this same
    /// database does not, and nothing here can enforce that.
    pub async fn emit_checkpoint(&self, reason: &str) {
        match self.chain_witness().await {
            Ok(witness) => tracing::info!(
                target: "audit_checkpoint",
                schema = %self.map.schema,
                fhir_version = %self.map.fhir_version,
                keyed = self.keys.signing().is_some(),
                %reason,
                %witness,
                "chain checkpoint"
            ),
            // Never fatal: a checkpoint that cannot be taken must not stop
            // the server serving, but must be loud enough to notice.
            Err(e) => tracing::error!(
                target: "audit_checkpoint",
                schema = %self.map.schema,
                error = %e,
                %reason,
                "chain checkpoint failed"
            ),
        }
    }

    pub async fn verify_audit(&self) -> Result<Vec<ChainBreak>, StoreError> {
        use crate::chain::{self, MacCheck};
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        let mut breaks = Vec::new();
        for rm in self.map.resources.values() {
            let Some((_, hist)) = rm.find_table(TableKind::History) else {
                continue;
            };
            // Rows in chain order. Recomputation happens in this process,
            // with the same `chain::preimage` the writer used, so the two
            // cannot drift into disagreeing about what was signed — and so
            // the database is never told the format.
            let rows = client
                .query(
                    &format!(
                        "SELECT \"id\", \"version_id\", \
                                to_char(\"last_updated\" AT TIME ZONE 'UTC', \
                                        'YYYY-MM-DD HH24:MI:SS.US'), \
                                \"op\", (\"resource\")::text, \"actor\", \
                                \"prev_hash\", \"row_hash\", \
                                \"prev_hash_sha3\", \"row_hash_sha3\", \"row_mac\" \
                         FROM \"{s}\".\"{}\" ORDER BY \"id\", \"version_id\"",
                        hist.name
                    ),
                    &[],
                )
                .await?;
            let countersigns: std::collections::HashMap<(String, i64), String> = client
                .query(
                    &format!(
                        "SELECT \"id\", \"version_id\", \"row_mac\" \
                         FROM \"{s}\".\"fhir_postgresql_countersign\" WHERE \"rtype\" = $1"
                    ),
                    &[&rm.name],
                )
                .await
                // A schema predating the table verifies exactly as before.
                .unwrap_or_default()
                .into_iter()
                .map(|r| ((r.get(0), r.get(1)), r.get(2)))
                .collect();
            let mut prev_id = String::new();
            let (mut prior_256, mut prior_3): (Option<Vec<u8>>, Option<Vec<u8>>) = (None, None);
            for row in rows {
                let id: String = row.get(0);
                let version_id: i64 = row.get(1);
                let ts_utc: String = row.get(2);
                let op: String = row.get(3);
                let resource: Option<String> = row.get(4);
                let actor: String = row.get(5);
                let prev_hash: Option<Vec<u8>> = row.get(6);
                let row_hash: Option<Vec<u8>> = row.get(7);
                let prev_sha3: Option<Vec<u8>> = row.get(8);
                let row_sha3: Option<Vec<u8>> = row.get(9);
                let row_mac: Option<String> = row.get(10);

                if id != prev_id {
                    prev_id.clone_from(&id);
                    prior_256 = None;
                    prior_3 = None;
                }
                // A tombstone terminates a chain rather than continuing it
                // (M3.18); it is not a break.
                if op == "X" {
                    prior_256 = row_hash;
                    prior_3 = row_sha3;
                    continue;
                }
                // Rows written before the audit columns existed carry no
                // digest. That is where the chain begins, not tampering —
                // claiming a break where there is only history would train an
                // operator to ignore the report.
                let (Some(stored_256), Some(stored_3)) = (&row_hash, &row_sha3) else {
                    prior_256 = row_hash;
                    prior_3 = row_sha3;
                    continue;
                };

                // Canonicalized in Rust, exactly as the writer did (`M14.12`).
                let canon = resource.as_deref().map(canon_of);
                let pre = chain::preimage(&id, version_id, &ts_utc, &op, canon.as_deref(), &actor);
                let (want_256, want_3) =
                    chain::link(prior_256.as_deref(), prior_3.as_deref(), &pre);

                for (algorithm, stored, want, stored_link, prior) in [
                    ("sha256", stored_256, &want_256, &prev_hash, &prior_256),
                    ("sha3-256", stored_3, &want_3, &prev_sha3, &prior_3),
                ] {
                    let bad = !chain::digests_equal(stored, want);
                    let unlinked = stored_link.as_deref() != prior.as_deref();
                    if bad || unlinked {
                        breaks.push(ChainBreak::new(
                            rm.name.clone(),
                            id.clone(),
                            version_id,
                            algorithm,
                            match (bad, unlinked) {
                                (true, true) => "row hash and link both differ",
                                (true, false) => "row contents differ from their hash",
                                _ => "link to the previous version differs",
                            },
                        ));
                    }
                }

                // Only a mismatch is a finding. The other outcomes say the
                // row could not be checked, which is a different claim and
                // must not be reported as tampering.
                let own = self
                    .keys
                    .check(row_mac.as_deref(), prior_256.as_deref(), &pre);
                // A counter-signature stands in once a key has been retired
                // (M3.16d), but only where the original tag cannot be
                // checked. A row whose own tag *mismatches* stays a finding
                // regardless of what later vouched for it — otherwise
                // re-signing would be a way to bless forged history.
                let verdict = match (&own, countersigns.get(&(id.clone(), version_id))) {
                    (MacCheck::Absent | MacCheck::Unverifiable { .. }, Some(have)) => {
                        match self.keys.signing() {
                            Some(k)
                                if chain::digests_equal(
                                    chain::mac(k, prior_256.as_deref(), &pre).as_bytes(),
                                    have.as_bytes(),
                                ) =>
                            {
                                MacCheck::Ok
                            }
                            _ => own,
                        }
                    }
                    _ => own,
                };
                match verdict {
                    MacCheck::Mismatch => breaks.push(ChainBreak::new(
                        rm.name.clone(),
                        id.clone(),
                        version_id,
                        "hmac-sha256",
                        "keyed tag does not match",
                    )),
                    MacCheck::Ok | MacCheck::Absent => {}
                    MacCheck::Unverifiable { key_id } => {
                        tracing::warn!(
                            rtype = %rm.name, %id, version_id, %key_id,
                            "row is signed with a key this process does not hold; not checked"
                        );
                    }
                    MacCheck::Malformed => tracing::warn!(
                        rtype = %rm.name, %id, version_id,
                        "row_mac is not <key-id>:<hex>; not checked"
                    ),
                }

                prior_256 = row_hash;
                prior_3 = row_sha3;
            }
        }
        Ok(breaks)
    }

    /// The audit envelope of every history row for one resource, oldest
    /// first: `(version_id, actor, actor_source, client, request_id, reason)`.
    ///
    /// This is how an operator answers "who changed this record, and why".
    #[allow(clippy::type_complexity)]
    pub async fn raw_history_audit(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<
        Vec<(
            i64,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        )>,
        StoreError,
    > {
        let rm = self.rm(rtype)?;
        let s = &self.map.schema;
        let hist = &rm.find_table(TableKind::History).expect("history").1.name;
        let client = self.pool.get().await?;
        let rows = client
            .query(
                &format!(
                    "SELECT \"version_id\", \"actor\", \"actor_source\", \"client\", \
                            \"request_id\", \"reason\" \
                     FROM \"{s}\".\"{hist}\" WHERE \"id\" = $1 ORDER BY \"version_id\""
                ),
                &[&id],
            )
            .await?;
        Ok(rows
            .iter()
            .map(|r| (r.get(0), r.get(1), r.get(2), r.get(3), r.get(4), r.get(5)))
            .collect())
    }

    /// Disclosure records for one resource, oldest first:
    /// `(actor, interaction, outcome)`.
    ///
    /// This is how an operator answers "who has looked at this patient".
    /// Total disclosure-log rows — the observability hook `fhir-loco`'s
    /// tests use, matching `fhir-sqlite`'s method of the same name.
    pub async fn access_log_len(&self) -> Result<i64, StoreError> {
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                &format!("SELECT COUNT(*) FROM \"{s}\".\"fhir_postgresql_access_log\""),
                &[],
            )
            .await?;
        Ok(row.get(0))
    }

    pub async fn access_log_for(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Vec<(String, String, String)>, StoreError> {
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        let rows = client
            .query(
                &format!(
                    "SELECT \"actor\", \"interaction\", \"outcome\" \
                     FROM \"{s}\".\"fhir_postgresql_access_log\" \
                     WHERE \"rtype\" = $1 AND \"id\" = $2 ORDER BY \"seq\""
                ),
                &[&rtype, &id],
            )
            .await?;
        Ok(rows
            .iter()
            .map(|r| (r.get(0), r.get(1), r.get(2)))
            .collect())
    }

    /// Run arbitrary SQL against this schema. **Tests only** — it exists so
    /// the audit suite can play the attacker with direct database access,
    /// which is the threat the hash chain and the append-only trigger are
    /// there to answer.
    #[doc(hidden)]
    pub async fn execute_raw_for_test(&self, sql: &str) -> Result<(), StoreError> {
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        client
            .batch_execute(&format!("SET LOCAL search_path TO \"{s}\";\n{sql}"))
            .await?;
        Ok(())
    }

    /// `EXPLAIN` a compiled search under a **forced generic plan**, returning
    /// the plan lines. **Tests only.**
    ///
    /// The generic plan is the point: it is the plan PostgreSQL reuses once a
    /// statement has been executed a few times, and it is the one that cannot
    /// see parameter values. A prefix search written as `LIKE $1` degrades to
    /// a sequential scan there while looking fine in every hand-run `EXPLAIN`
    /// with a literal — which is how the first attempt at P6.6 passed review
    /// and would still have scanned the whole table in production.
    #[doc(hidden)]
    pub async fn explain_generic_for_test(
        &self,
        rtype: &str,
        params: &[(String, String)],
    ) -> Result<Vec<String>, StoreError> {
        let rm =
            self.map.resources.get(rtype).ok_or_else(|| {
                StoreError::Unsupported(format!("unknown resource type {rtype:?}"))
            })?;
        let q = crate::search::build_search_sql(&self.map, rm, params, 100, 0, &[], None)?;
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        tx.batch_execute("SET LOCAL plan_cache_mode = force_generic_plan")
            .await?;
        let stmt = tx.prepare(&format!("EXPLAIN {}", q.sql)).await?;
        let binds: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = q
            .binds
            .iter()
            .map(|b| b as &(dyn tokio_postgres::types::ToSql + Sync))
            .collect();
        let rows = tx.query(&stmt, &binds).await?;
        Ok(rows.iter().map(|r| r.get::<_, String>(0)).collect())
    }

    /// Whether this version's schema is installed in the database.
    pub async fn installed(&self) -> Result<bool, StoreError> {
        let client = self.pool.get().await?;
        let row = client
            .query_opt(
                "SELECT 1 FROM information_schema.schemata WHERE schema_name = $1",
                &[&self.map.schema],
            )
            .await?;
        Ok(row.is_some())
    }

    /// The (type, id) pairs referenced by `param` (a compiled reference
    /// search parameter) across the given resources — the _include lookup.
    pub async fn refs_of(
        &self,
        rtype: &str,
        ids: &[String],
        param: &str,
    ) -> Result<Vec<(String, String)>, StoreError> {
        use fhir_postgresql_map::model::TargetKind;
        let rm = self.rm(rtype)?;
        let Some(def) = rm.search.iter().find(|d| d.code == param) else {
            return Err(StoreError::Other(format!(
                "unknown _include parameter {param:?}"
            )));
        };
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        let mut out = Vec::new();
        for t in &def.targets {
            let TargetKind::Reference { c_type, c_id, .. } = &t.kind else {
                continue;
            };
            let table = &rm.tables[t.table as usize].name;
            let (id_col, filter) = if t.table == 0 {
                ("\"id\"", "")
            } else {
                ("\"rid\"", "")
            };
            let _ = filter;
            let sql = format!(
                "SELECT DISTINCT \"{c_type}\", \"{c_id}\" FROM \"{s}\".\"{table}\"                  WHERE {id_col} = ANY($1) AND \"{c_type}\" IS NOT NULL AND \"{c_id}\" IS NOT NULL"
            );
            let rows = client.query(&sql, &[&ids]).await?;
            for r in rows {
                out.push((r.get(0), r.get(1)));
            }
        }
        if def.targets.is_empty() {
            return Err(StoreError::Other(format!(
                "search parameter {param:?} has no reference targets"
            )));
        }
        Ok(out)
    }

    /// Cheap connectivity probe for readiness checks.
    pub async fn ping(&self) -> Result<(), StoreError> {
        let client = self.pool.get().await?;
        client.query_one("SELECT 1", &[]).await?;
        Ok(())
    }

    /// All current resource ids of one type.
    pub async fn ids(&self, rtype: &str) -> Result<Vec<String>, StoreError> {
        let rm = self.rm(rtype)?;
        let s = &self.map.schema;
        let client = self.pool.get().await?;
        let rows = client
            .query(
                &format!(
                    "SELECT \"id\" FROM \"{s}\".\"{}\" ORDER BY \"id\"",
                    rm.base_table().name
                ),
                &[],
            )
            .await?;
        Ok(rows.iter().map(|r| r.get(0)).collect())
    }
}

/// Insert every shredded row inside the caller's transaction.
async fn insert_shredded(
    tx: &tokio_postgres::Transaction<'_>,
    map: &RelMap,
    rm: &ResourceMap,
    id: &str,
    version: i64,
    out: &ShredOut,
) -> Result<(), StoreError> {
    let s = &map.schema;

    // Group element rows by table.
    let mut by_table: Vec<Vec<&fhir_postgresql_map::shred::Row>> =
        vec![Vec::new(); rm.tables.len()];
    for row in &out.rows {
        by_table[row.table as usize].push(row);
    }

    for (ti, rows) in by_table.iter().enumerate() {
        if rows.is_empty() {
            continue;
        }
        let t = &rm.tables[ti];
        // The union of populated columns across this table's rows.
        let mut names: BTreeSet<&str> = BTreeSet::new();
        for r in rows {
            for (n, _) in &r.cols {
                names.insert(n);
            }
        }
        let names: Vec<&str> = names.into_iter().collect();
        let types: Vec<ColTy> = names
            .iter()
            .map(|n| {
                t.cols
                    .iter()
                    .find(|c| c.name == **n)
                    .map(|c| c.ty)
                    .expect("shredded column exists in table")
            })
            .collect();

        let (sys_cols, sys_vals): (&str, usize) = match t.kind {
            TableKind::Base => ("\"id\", \"version_id\", \"last_updated\"", 2),
            TableKind::Elem => ("\"rid\", \"ords\"", 2),
            _ => unreachable!("shred rows only target base/elem tables"),
        };

        // Chunk to stay far below the 65535-parameter protocol limit.
        let per_row = sys_vals + names.len();
        let chunk_rows = (30000 / per_row.max(1)).max(1);
        for chunk in rows.chunks(chunk_rows) {
            let mut sql = format!("INSERT INTO \"{s}\".\"{}\" ({sys_cols}", t.name);
            for n in &names {
                sql.push_str(&format!(", \"{n}\""));
            }
            sql.push_str(") VALUES ");
            let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
            let mut ords_bufs: Vec<String> = Vec::new();
            for r in chunk {
                ords_bufs.push(fmt_ords(&r.ords));
            }
            for (ri, r) in chunk.iter().enumerate() {
                if ri > 0 {
                    sql.push_str(", ");
                }
                match t.kind {
                    TableKind::Base => {
                        params.push(Box::new(id.to_string()));
                        let p1 = params.len();
                        params.push(Box::new(version));
                        sql.push_str(&format!("(${p1}, ${}, now()", p1 + 1));
                    }
                    TableKind::Elem => {
                        params.push(Box::new(id.to_string()));
                        let p1 = params.len();
                        params.push(Box::new(ords_bufs[ri].clone()));
                        sql.push_str(&format!("(${p1}, (${}::text)::smallint[]", p1 + 1));
                    }
                    _ => unreachable!(),
                }
                for (n, ty) in names.iter().zip(&types) {
                    let val = r.cols.iter().find(|(cn, _)| cn == *n).map(|(_, v)| v);
                    match val {
                        None => sql.push_str(", NULL"),
                        Some(v) => {
                            let image = match v {
                                SqlVal::Bool(b) => b.to_string(),
                                SqlVal::Int(n) => n.to_string(),
                                SqlVal::Num(x)
                                | SqlVal::Text(x)
                                | SqlVal::Ts(x)
                                | SqlVal::Date(x)
                                | SqlVal::Jsonb(x) => x.clone(),
                                // U4a: this path deliberately sends every
                                // value as text and casts it server-side, so
                                // the digest goes as PostgreSQL's `\x` hex
                                // input form and `::bytea` decodes it. The
                                // stored value is 32 binary bytes either way;
                                // only the wire representation is text.
                                SqlVal::Bytes(b) => {
                                    let mut s = String::with_capacity(2 + b.len() * 2);
                                    s.push_str("\\x");
                                    for byte in b {
                                        use std::fmt::Write as _;
                                        let _ = write!(s, "{byte:02x}");
                                    }
                                    s
                                }
                            };
                            params.push(Box::new(image));
                            // ($n::text)::<type> keeps the wire type text.
                            sql.push_str(&format!(
                                ", (${}::text)::{}",
                                params.len(),
                                fhir_postgresql_map::ddl::col_sql(*ty)
                            ));
                        }
                    }
                }
                sql.push(')');
            }
            let refs: Vec<&(dyn ToSql + Sync)> = params
                .iter()
                .map(|b| b.as_ref() as &(dyn ToSql + Sync))
                .collect();
            tx.execute(&sql, &refs).await?;
        }
    }

    // Extension rows.
    if !out.ext.is_empty() {
        let t = rm
            .find_table(TableKind::Ext)
            .expect("ext table")
            .1
            .name
            .clone();
        for chunk in out.ext.chunks(3000) {
            let mut sql = format!(
                "INSERT INTO \"{s}\".\"{t}\" (\"rid\", \"path\", \"ords\", \"modifier\", \"ext_ord\", \"url\", \"leaf\", \"v_kind\", \"v_text\", \"v_num\", \"v_bool\") VALUES "
            );
            let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
            for (ri, e) in chunk.iter().enumerate() {
                if ri > 0 {
                    sql.push_str(", ");
                }
                let (kind, text, num, boolv) = e.val.cols();
                let base = params.len();
                params.push(Box::new(id.to_string()));
                params.push(Box::new(e.path.clone()));
                params.push(Box::new(fmt_ords(&e.ords)));
                params.push(Box::new(e.modifier));
                params.push(Box::new(e.ext_ord));
                params.push(Box::new(e.url.clone()));
                params.push(Box::new(e.leaf.clone()));
                params.push(Box::new(kind.to_string()));
                params.push(Box::new(text.map(str::to_string)));
                params.push(Box::new(num.map(str::to_string)));
                params.push(Box::new(boolv));
                sql.push_str(&format!(
                    "(${}, ${}, (${}::text)::smallint[], ${}, ${}, ${}, ${}, ${}, ${}, (${}::text)::numeric, ${})",
                    base + 1, base + 2, base + 3, base + 4, base + 5, base + 6,
                    base + 7, base + 8, base + 9, base + 10, base + 11
                ));
            }
            let refs: Vec<&(dyn ToSql + Sync)> = params
                .iter()
                .map(|b| b.as_ref() as &(dyn ToSql + Sync))
                .collect();
            tx.execute(&sql, &refs).await?;
        }
    }

    // Spill rows.
    if !out.deep.is_empty() {
        let t = rm
            .find_table(TableKind::Deep)
            .expect("deep table")
            .1
            .name
            .clone();
        for chunk in out.deep.chunks(3000) {
            let mut sql = format!(
                "INSERT INTO \"{s}\".\"{t}\" (\"rid\", \"path\", \"ords\", \"leaf\", \"v_kind\", \"v_text\", \"v_num\", \"v_bool\") VALUES "
            );
            let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
            for (ri, d) in chunk.iter().enumerate() {
                if ri > 0 {
                    sql.push_str(", ");
                }
                let (kind, text, num, boolv) = d.val.cols();
                let base = params.len();
                params.push(Box::new(id.to_string()));
                params.push(Box::new(d.path.clone()));
                params.push(Box::new(fmt_ords(&d.ords)));
                params.push(Box::new(d.leaf.clone()));
                params.push(Box::new(kind.to_string()));
                params.push(Box::new(text.map(str::to_string)));
                params.push(Box::new(num.map(str::to_string)));
                params.push(Box::new(boolv));
                sql.push_str(&format!(
                    "(${}, ${}, (${}::text)::smallint[], ${}, ${}, ${}, (${}::text)::numeric, ${})",
                    base + 1,
                    base + 2,
                    base + 3,
                    base + 4,
                    base + 5,
                    base + 6,
                    base + 7,
                    base + 8
                ));
            }
            let refs: Vec<&(dyn ToSql + Sync)> = params
                .iter()
                .map(|b| b.as_ref() as &(dyn ToSql + Sync))
                .collect();
            tx.execute(&sql, &refs).await?;
        }
    }

    // Contained resources.
    if !out.contained.is_empty() {
        let t = rm
            .find_table(TableKind::Contained)
            .expect("contained table")
            .1
            .name
            .clone();
        for (ord, v) in &out.contained {
            let json = serde_json::to_string(v).map_err(|e| StoreError::Other(e.to_string()))?;
            tx.execute(
                &format!(
                    "INSERT INTO \"{s}\".\"{t}\" (\"rid\", \"ord\", \"resource\") VALUES ($1, $2, ($3::text)::jsonb)"
                ),
                &[&id, ord, &json],
            )
            .await?;
        }
    }
    Ok(())
}

fn fmt_ords(ords: &[i16]) -> String {
    let inner: Vec<String> = ords.iter().map(|o| o.to_string()).collect();
    format!("{{{}}}", inner.join(","))
}

fn parse_ords(s: &str) -> Result<Vec<i16>, StoreError> {
    let t = s.trim_start_matches('{').trim_end_matches('}');
    if t.is_empty() {
        return Ok(Vec::new());
    }
    t.split(',')
        .map(|x| {
            x.trim()
                .parse::<i16>()
                .map_err(|_| StoreError::Other(format!("bad ords image {s:?}")))
        })
        .collect()
}

use futures_util::future::join_all as futures_join_all;
