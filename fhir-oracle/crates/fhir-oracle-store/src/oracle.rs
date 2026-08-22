//! The Oracle store.
//!
//! # Live-verified, 2026-08-04 (`T11.2`, **F-68**)
//!
//! Written 2026-08-04 with no Oracle Instant Client on this build host, and
//! for a time unverified beyond a clean `cargo build` (**F-66**: ODPI-C
//! loads `libclntsh` via `dlopen` at connection time, not link time, so
//! `cargo check`/`cargo build` succeed with no client library at all; a
//! minimal probe confirmed the real wall was `DPI-1047` at `Connection::
//! connect`, not at build time).
//!
//! Instant Client for macOS arm64 downloads directly from Oracle with no
//! login required. Installed, this file connected to a live
//! `gvenzl/oracle-free:23-slim-faststart` for the first time, and
//! `tests/oracle_store.rs` now runs its full surface against it — 7 of 7
//! green, 0 ignored. Getting there found and fixed five real defects, none
//! visible from reading the code:
//!
//! 1. **Uppercase schema case-folding (`M14.5`).** Oracle folds an
//!    *unquoted* username to uppercase for session identity regardless of
//!    how `CREATE USER` quoted it; a lowercase schema made every DDL
//!    statement `ORA-01031` against a session that was really `"R5"`. Fixed
//!    by creating users unquoted and setting `RelMap.schema` to match,
//!    uppercase — the opposite convention from every other port.
//! 2. **`R4.5`'s presumed mechanism doesn't work (`M14.19`).** `SET
//!    TRANSACTION READ ONLY` failed every read with `ORA-01466: unable to
//!    read data - table definition has changed` on any session that had run
//!    DDL, reproduced with a minimal 3-statement probe. Removed rather than
//!    shipped broken — `get` below has **no snapshot-isolation protection**;
//!    `R4.5` is an open, confirmed gap on this port, not merely unverified.
//! 3. **Double schema-qualification (`ORA-00926`).** `insert_row` took a
//!    `schema` argument *and*, at several call sites, an already-qualified
//!    table string. Fixed by taking one pre-qualified `target: &str`.
//! 4. **Timestamp binding relied on session NLS settings (`ORA-01843`).**
//!    Plain-string binds leaned on Oracle's implicit conversion, which reads
//!    `NLS_TIMESTAMP_FORMAT`, not ISO 8601. Fixed with real `chrono`-typed
//!    binds (`Bound::Timestamp`, `Bound::CalDate`).
//! 5. **A boolean bound as text in token search (`ORA-01722`)**, found by
//!    `tests/oracle_store.rs` itself — see `oracle_search.rs`'s module doc.
//!
//! **This crate's conformance level is now Store (`C0.8`)** for the
//! operations below, live-tested; it is not Reference — no concurrency test
//! exists to verify `H5.4` under contention (the `SELECT … FOR UPDATE`
//! mechanism is present but unexercised by a racing-writers test), and
//! `R4.5` has no working mechanism at all, per point 2 above. See the
//! [conformance matrix](../../../spec/databases/conformance-matrix.md) for
//! the row-by-row claim.
//!
//! # What is implemented and live-tested
//!
//! `connect`, `init`, `put`, `get`, `delete`, `history`, `vread`,
//! `verify_audit`, `purge`, `log_access`, `search`/`search_full` — the same
//! surface `fhir-mssql-store` and `fhir-mysql-store` expose, all exercised by
//! `tests/oracle_store.rs`. `search_page` exists and is called transitively
//! but has no test calling it directly with a cursor. `upgrade` and
//! `backfill_norm` exist too (2026-08-09, closing this port's share of
//! **F-15**), exercised by `tests/upgrade.rs` — see their doc comments for
//! the three engine-specific rules (`M14.35`–`M14.37`). **Not written**,
//! matching those ports: `conditional_create_audited`, `put_audited`.
//!
//! # Architecture, and why it differs from every other port but SQLite
//!
//! - **The driver is synchronous.** `oracle::Connection` calls into ODPI-C,
//!   which calls into OCI, which blocks the calling OS thread on network
//!   I/O — there is no async Oracle driver for Rust. Every public method
//!   here wraps its whole body in one `tokio::task::spawn_blocking`, the same
//!   shape `fhir-sqlite` uses for `rusqlite`, the only other synchronous
//!   driver among the six ports. Unlike SQLite, Oracle is a real server with
//!   real concurrent clients, so this port pools connections — via the
//!   `oracle` crate's own `oracle::pool::Pool` (`pool.rs`), not a
//!   hand-written `bb8::ManageConnection` the way `fhir-mssql` needed for
//!   `tiberius`, which ships no pool of its own.
//! - **Transactions are Rust calls, not SQL statements.** Oracle has no
//!   `BEGIN TRANSACTION`; a transaction is implicitly open from the first
//!   DML statement on a connection and closed by `Connection::commit()` or
//!   `Connection::rollback()`.
//! - **Row locking is `SELECT … FOR UPDATE`**, Oracle's native syntax, unlike
//!   the `WITH (UPDLOCK, ROWLOCK)` hint `fhir-mssql` needs.
//! - **The erasure flag travels in `CLIENT_INFO`**
//!   (`DBMS_APPLICATION_INFO.SET_CLIENT_INFO`), per `M14.29` — Oracle has no
//!   session-variable or `SESSION_CONTEXT` equivalent reachable without a
//!   `CREATE ANY CONTEXT` privilege this port's install does not require.
//!   `CLIENT_INFO` is general-purpose and shared with monitoring tools, so —
//!   exactly as `fhir-mssql` does for `SESSION_CONTEXT` on a *pooled*
//!   connection — it MUST be set immediately before the erasing `DELETE` and
//!   cleared immediately after, on the same connection, never assumed to
//!   reset itself between pool checkouts.
//! - **`Ext`/`Deep` carry a hash surrogate key** (`M14.9`), for a sharper
//!   reason than SQL Server's: their natural key includes a `CLOB`, which on
//!   this engine cannot be indexed *or* `=`-compared at all (`ORA-02327`,
//!   `ORA-22848`), not merely excluded from a key.
//! - **Booleans bind as `NUMBER(1)` (0/1 via `i64`), never the `oracle`
//!   crate's native `bool` binding.** `impl ToSql for bool` in the driver
//!   targets Oracle's `BOOLEAN` type, which arrived in 23ai; this port's
//!   floor is 12.2 (`M14.2`, `M14.4`) and its schema's `Bool` columns are
//!   `NUMBER(1)`. Binding a Rust `bool` directly would send the wrong wire
//!   type for every boolean column this store ever writes — found reading
//!   the driver source, not by running anything, which is exactly the class
//!   of defect this file cannot promise is the only one it has.
//! - **Adjunct-aware search.** Unlike `fhir-mssql`, which deliberately lets
//!   `Text`-typed search targets scan unindexed because `NVARCHAR(MAX)` still
//!   answers `=`, an Oracle `CLOB` answers **no** comparison at all. Every
//!   comparison this port's search builder (`oracle_search.rs`) makes against
//!   a `Text`/`Jsonb`-typed column goes through the `U1`–`U10` adjunct pair
//!   instead of the source column. `U6`'s `DBMS_LOB.COMPARE` confirmation
//!   step is not implemented — see that module's doc comment.

use std::sync::Arc;

use fhir_oracle_map::model::{ColTy, RelMap};
use oracle::sql_type::{OracleType, ToSql};
use oracle::{Connection, Row, SqlValue};

use crate::StoreError;
use crate::pool::{self, OraclePool};

/// A history row's chain tip: `(version_id, sha256 link, sha3 link)`.
type ChainTip = (i64, Option<Vec<u8>>, Option<Vec<u8>>);

fn db_err(e: oracle::Error) -> StoreError {
    StoreError::Db(e.to_string())
}

fn join_err(e: tokio::task::JoinError) -> StoreError {
    StoreError::Other(format!("blocking task panicked: {e}"))
}

/// `"schema"."table"`, both double-quoted, matching `ddl.rs`.
///
/// Oracle's *unquoted* identifiers fold to uppercase; a quoted identifier is
/// exact-case and is what `ddl.rs` emits throughout, so this store must quote
/// the same way or it would address a same-named-but-different object.
fn quote_ident(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

fn qualified(schema: &str, name: &str) -> String {
    format!("{}.{}", quote_ident(schema), quote_ident(name))
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
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

/// Write one meta value in chunks small enough to bind as `VARCHAR2` in SQL
/// context (< 4000 bytes; `ORA-01461` otherwise — the map asset is around a
/// megabyte of hex, far past any string-bind limit, and chunked rows sidestep
/// the LOB-binding API entirely). The base key holds the chunk count; chunks
/// live at `<key>.<i>`.
fn write_meta_chunked(
    conn: &Connection,
    meta: &str,
    key: &str,
    value: &str,
) -> Result<(), StoreError> {
    const CHUNK: usize = 3000;
    let merge = format!(
        "MERGE INTO {meta} tgt USING (SELECT :1 AS k, :2 AS v FROM DUAL) src \
         ON (tgt.\"key\" = src.k) \
         WHEN MATCHED THEN UPDATE SET \"value\" = src.v \
         WHEN NOT MATCHED THEN INSERT (\"key\", \"value\") VALUES (src.k, src.v)"
    );
    if value.len() <= CHUNK {
        conn.execute(&merge, &[&key, &value]).map_err(db_err)?;
        return Ok(());
    }
    // Stale chunks from a longer previous value must not survive the write.
    conn.execute(
        &format!("DELETE FROM {meta} WHERE \"key\" LIKE :1 || '.%'"),
        &[&key],
    )
    .map_err(db_err)?;
    let chunks: Vec<&str> = value
        .as_bytes()
        .chunks(CHUNK)
        .map(|c| std::str::from_utf8(c).expect("hex is ASCII"))
        .collect();
    for (i, chunk) in chunks.iter().enumerate() {
        let k = format!("{key}.{i}");
        conn.execute(&merge, &[&k.as_str(), chunk])
            .map_err(db_err)?;
    }
    let count = format!("chunks:{}", chunks.len());
    conn.execute(&merge, &[&key, &count.as_str()])
        .map_err(db_err)?;
    Ok(())
}

/// Read a value `write_meta_chunked` wrote, reassembling chunks.
fn read_meta_chunked(
    conn: &Connection,
    meta: &str,
    key: &str,
) -> Result<Option<String>, StoreError> {
    let row = conn.query_row(
        &format!("SELECT \"value\" FROM {meta} WHERE \"key\" = :1"),
        &[&key],
    );
    let base: Option<String> = match row {
        Ok(r) => r.get::<usize, Option<String>>(0).unwrap_or(None),
        Err(_) => None,
    };
    let Some(base) = base else { return Ok(None) };
    let Some(n) = base.strip_prefix("chunks:") else {
        return Ok(Some(base));
    };
    let n: usize = n
        .parse()
        .map_err(|_| StoreError::Other("bad chunk count in meta".into()))?;
    let mut out = String::new();
    for i in 0..n {
        let k = format!("{key}.{i}");
        let chunk: Option<String> = match conn.query_row(
            &format!("SELECT \"value\" FROM {meta} WHERE \"key\" = :1"),
            &[&k.as_str()],
        ) {
            Ok(r) => r.get::<usize, Option<String>>(0).unwrap_or(None),
            Err(_) => None,
        };
        let chunk = chunk.ok_or_else(|| {
            StoreError::Other(format!("meta value {key:?} is missing chunk {i} of {n}"))
        })?;
        out.push_str(&chunk);
    }
    Ok(Some(out))
}

/// Wrap a statement so a rerun survives "already applied" (`M14.15`'s shape).
///
/// Oracle has no transactional DDL: a failed `upgrade` leaves everything
/// before the failure applied, and the recovery is to run it again — which
/// only works if every statement tolerates having already run. Statements
/// `ddl.rs` emits pre-wrapped (`BEGIN\n  EXECUTE IMMEDIATE …`) pass through
/// untouched: wrapping them again would nest `q'{…}'` literals, which cannot
/// nest.
fn resumable(stmt: &str, codes: &[i32]) -> String {
    if stmt.starts_with("BEGIN\n  EXECUTE IMMEDIATE") {
        return stmt.to_string();
    }
    assert!(
        !stmt.contains("}'"),
        "q'{{...}}' literal would terminate early: {stmt}"
    );
    let list = codes
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "BEGIN\n  EXECUTE IMMEDIATE q'{{{stmt}}}';\n\
         EXCEPTION WHEN OTHERS THEN\n  IF SQLCODE NOT IN ({list}) THEN RAISE; END IF;\nEND;"
    )
}

/// A pool bound to one relational map.
pub struct OracleStore {
    pool: OraclePool,
    map: Arc<RelMap>,
    keys: crate::chain::KeyRing,
}

impl OracleStore {
    /// Connect using Oracle's three-part credential (`pool.rs`).
    ///
    /// # Errors
    /// If the pool cannot be built — on this host, always: no Instant Client.
    pub async fn connect(
        username: &str,
        password: &str,
        connect_string: &str,
        map: Arc<RelMap>,
    ) -> Result<Self, StoreError> {
        let username = username.to_string();
        let password = password.to_string();
        let connect_string = connect_string.to_string();
        let pool = tokio::task::spawn_blocking(move || {
            pool::connect_pool(&username, &password, &connect_string)
        })
        .await
        .map_err(join_err)??;
        Ok(Self {
            pool,
            map,
            keys: crate::chain::KeyRing::default(),
        })
    }

    /// Sign the hash chain with these keys instead of leaving it unkeyed.
    #[must_use]
    pub fn with_chain_keys(mut self, keys: crate::chain::KeyRing) -> Self {
        self.keys = keys;
        self
    }

    /// Confirm the server is reachable.
    ///
    /// # Errors
    /// If the pool cannot produce a working connection.
    pub async fn ping(&self) -> Result<(), StoreError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            conn.query_row("SELECT 1 FROM DUAL", &[]).map_err(db_err)?;
            Ok(())
        })
        .await
        .map_err(join_err)?
    }

    /// Apply the generated DDL as a fresh install.
    ///
    /// Every statement `ddl.rs` emits for a `CREATE TABLE`/`CREATE INDEX`/
    /// trigger is already idempotent (`M14.15`, `M14.31` — a PL/SQL block
    /// swallowing `ORA-00955`/`ORA-01408`), **except** the per-resource table
    /// and index statements from `create_table`/`ddl_in`'s main loop, which
    /// are not wrapped the same way there — mirroring `fhir-mssql`'s `init`,
    /// a second call is documented as `init --upgrade`'s job, not this one's.
    ///
    /// # Errors
    /// On the first statement that fails to apply.
    pub async fn init(&self, checksum: &str) -> Result<usize, StoreError> {
        let pool = self.pool.clone();
        let map = self.map.clone();
        let checksum = checksum.to_string();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            let meta = qualified(&map.schema, "fhir_oracle_meta");
            let statements = fhir_oracle_map::ddl::ddl(&map);
            let mut applied = 0usize;
            for stmt in &statements {
                conn.execute(stmt, &[]).map_err(|e| {
                    StoreError::Other(format!(
                        "init: statement {} of {} failed: {e}\n{stmt}",
                        applied + 1,
                        statements.len()
                    ))
                })?;
                applied += 1;
            }
            // MERGE, Oracle's UPSERT, available well below the M14.2 floor.
            // The map asset itself is stored beside the checksum — it is what
            // `upgrade` diffs against, and an install that records only the
            // checksum cannot be upgraded later (the pre-F-15 mssql defect).
            let asset_hex = hex_encode(
                &map.to_gz_bytes()
                    .map_err(|e| StoreError::Other(e.to_string()))?,
            );
            for (k, v) in [
                ("checksum", checksum.as_str()),
                ("fhir_version", map.fhir_version.as_str()),
                ("map_asset", asset_hex.as_str()),
            ] {
                write_meta_chunked(&conn, &meta, k, v)?;
            }
            conn.commit().map_err(db_err)?;
            Ok(applied)
        })
        .await
        .map_err(join_err)?
    }

    /// The checksum recorded by the last successful `init`, if any.
    ///
    /// # Errors
    /// On a connection or query failure. A missing meta table is **not** an
    /// error — it means this schema has never been installed — and is folded
    /// into `Ok(None)`.
    pub async fn installed_checksum(&self) -> Result<Option<String>, StoreError> {
        let pool = self.pool.clone();
        let map = self.map.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            let meta = qualified(&map.schema, "fhir_oracle_meta");
            let row = conn.query_row(
                &format!("SELECT \"value\" FROM {meta} WHERE \"key\" = :1"),
                &[&"checksum"],
            );
            match row {
                Ok(r) => Ok(r.get::<usize, Option<String>>(0).unwrap_or(None)),
                Err(_) => Ok(None),
            }
        })
        .await
        .map_err(join_err)?
    }

    /// Upgrade an installed schema to this store's map (`O10.4a`, `L12`).
    ///
    /// Oracle has no transactional DDL: every statement autocommits, so a
    /// failure leaves everything before it applied. The recovery is to run
    /// `upgrade` again — every statement this method applies tolerates
    /// having already run (`resumable`), so a rerun completes the remainder
    /// rather than failing on what the first attempt achieved. That is this
    /// dialect's half-applied-upgrade story, and the annex states it.
    ///
    /// The reconciliation step re-applies the **whole** current DDL with
    /// every statement made resumable — new tables, their indexes, the
    /// schema-wide objects, and the `CREATE OR REPLACE` triggers all
    /// converge in one pass. On a full R5 install that is thousands of
    /// swallowed `ORA-00955`s; an upgrade is rare enough that simple and
    /// self-healing beats fast here.
    ///
    /// # Errors
    /// If the schema is not installed, predates upgrade support (no stored
    /// map asset), needs destructive changes without `allow_destructive`, a
    /// column changed type (a migration somebody must design, `L12`), or on
    /// a database failure.
    pub async fn upgrade(
        &self,
        checksum: &str,
        allow_destructive: bool,
    ) -> Result<crate::UpgradeReport, StoreError> {
        self.upgrade_with(
            checksum,
            crate::UpgradeOpts {
                allow_destructive,
                ..crate::UpgradeOpts::default()
            },
        )
        .await
    }

    /// [`upgrade`](Self::upgrade) with the full option set: `reshred_moved`
    /// additionally carries data across relocated columns (`O10.4c`) — each
    /// affected resource reconstructed under the *stored* old map, shredded
    /// under the new one, `version_id` and `last_updated` preserved, no history
    /// entry (a representation change is not a new version), verified
    /// byte-identical before anything is dropped.
    ///
    /// **Oracle's failure story is resumable** (`M14.40`), which is what this
    /// port's whole upgrade already is: every DDL statement here commits
    /// implicitly and tolerates "already applied", so the migration is a state
    /// machine that can be re-entered rather than a transaction that can be
    /// rolled back. The re-shred matches it — one commit per resource, so a
    /// failure part-way leaves each resource wholly in the old shape or wholly
    /// in the new, nothing dropped, and a rerun carries what is left.
    pub async fn upgrade_with(
        &self,
        checksum: &str,
        opts: crate::UpgradeOpts,
    ) -> Result<crate::UpgradeReport, StoreError> {
        let pool = self.pool.clone();
        let map = self.map.clone();
        let checksum = checksum.to_string();
        let (additive, destructive, reshredded) =
            tokio::task::spawn_blocking(move || -> Result<(usize, usize, usize), StoreError> {
                let conn = pool.get().map_err(db_err)?;
                let meta = qualified(&map.schema, "fhir_oracle_meta");

                // "Never installed" and "installed before the asset was
                // recorded" have different remedies; distinguish them.
                let installed: i64 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM user_tables WHERE table_name = 'fhir_oracle_meta'",
                        &[],
                    )
                    .map_err(db_err)?
                    .get(0)
                    .map_err(db_err)?;
                if installed == 0 {
                    return Err(StoreError::Other(format!(
                        "schema {} is not installed",
                        map.schema
                    )));
                }
                let old_hex = read_meta_chunked(&conn, &meta, "map_asset")?;
                let old_hex = old_hex.ok_or_else(|| {
                    StoreError::Other(
                        "installed schema predates upgrade support (no stored map asset); \
                         reinstall with `init` to make later upgrades possible"
                            .into(),
                    )
                })?;
                let old_map = RelMap::from_gz_bytes(&hex_decode(&old_hex)?)
                    .map_err(|e| StoreError::Other(format!("stored map asset unreadable: {e}")))?;

                let (adds, drops) = diff_maps(&map, &old_map)?;
                // O10.4b: a moved column is not a drop. A map change that
                // relocates an element between tables (G2.6a's force-split,
                // F-90) reaches the diff as an ADD plus a DROP, and
                // `allow_destructive` was defined for abandoning data, not
                // relocating it. Refuse a data-bearing move by name,
                // independent of the flag; an empty source proceeds. Checked
                // before the destructive gate: "rerun with allow_destructive"
                // is the wrong advice for a relocation.
                let moved = moved_columns(&old_map, &map);
                let mut data_bearing: Vec<String> = Vec::new();
                for (t, col, nt) in &moved {
                    let has: i64 = conn
                        .query_row(
                            &format!(
                                "SELECT COUNT(*) FROM (SELECT 1 FROM {} WHERE \"{col}\" \
                                 IS NOT NULL FETCH FIRST 1 ROWS ONLY)",
                                qualified(&map.schema, t)
                            ),
                            &[],
                        )
                        .map_err(db_err)?
                        .get(0)
                        .map_err(db_err)?;
                    if has != 0 {
                        data_bearing.push(format!("{t}.{col} → {nt}"));
                    }
                }
                if !data_bearing.is_empty() && !opts.reshred_moved {
                    return Err(StoreError::Other(format!(
                        "upgrade refuses {} moved column(s) holding data (O10.4b, F-90): {}. \
                         allow_destructive does not cover relocation; re-put the affected \
                         reshred_moved (O10.4c), re-put the affected resource types \
                         through this artifact, or reload",
                        data_bearing.len(),
                        data_bearing.join(", ")
                    )));
                }
                if !drops.is_empty() && !opts.allow_destructive {
                    return Err(StoreError::Other(format!(
                        "upgrade requires {} destructive change(s); rerun with allow_destructive \
                         (first: {})",
                        drops.len(),
                        drops.first().expect("non-empty")
                    )));
                }
                let (n_add, n_drop) = (adds.len(), drops.len());

                let apply = |stmt: &str, codes: &[i32]| -> Result<(), StoreError> {
                    conn.execute(&resumable(stmt, codes), &[])
                        .map(|_| ())
                        .map_err(|e| StoreError::Other(format!("upgrade: {e}\n{stmt}")))
                };

                // 1. The additive diff: new tables and new columns.
                for stmt in &adds {
                    apply(stmt, &[-955, -1430])?;
                }
                // 2. Audit-envelope columns on history tables that predate
                //    them — Oracle's ADD is not idempotent (ddl.rs's own
                //    warning), so ORA-01430 is swallowed instead of diffed.
                for rm in map.resources.values() {
                    if let Some((_, hist)) =
                        rm.find_table(fhir_oracle_map::model::TableKind::History)
                    {
                        for stmt in
                            fhir_oracle_map::ddl::history_audit_columns(&map.schema, &hist.name)
                        {
                            apply(&stmt, &[-1430])?;
                        }
                    }
                }
                // 3. Reconcile everything else by re-applying the current DDL.
                for stmt in fhir_oracle_map::ddl::ddl(&map) {
                    apply(&stmt, &[-955, -1408, -1430])?;
                }
                // 3b. O10.4c: carry data across relocated columns. After the
                //     additive and reconcile passes (the new tables must
                //     exist) and before the drops (the old columns must still
                //     be readable). One commit per resource: this port's DDL
                //     commits implicitly, so the whole upgrade is resumable
                //     rather than atomic (M14.40), and the re-shred is built
                //     the same way.
                let mut reshredded = 0usize;
                if opts.reshred_moved && !moved.is_empty() {
                    let s_q = quote_ident(&map.schema);
                    let mut rtypes: std::collections::BTreeSet<&str> =
                        std::collections::BTreeSet::new();
                    for (t, _, _) in &moved {
                        for (name, orm) in &old_map.resources {
                            if orm.tables.iter().any(|tt| &tt.name == t) {
                                rtypes.insert(name.as_str());
                            }
                        }
                    }
                    for rtype in rtypes {
                        let old_rm = &old_map.resources[rtype];
                        let Some(new_rm) = map.resources.get(rtype) else {
                            return Err(StoreError::Other(format!(
                                "{rtype} has moved columns but the new map does not carry \
                                 the resource; re-shred cannot target it"
                            )));
                        };
                        let base_raw = old_rm.base_table().name.clone();
                        if base_raw != new_rm.base_table().name {
                            return Err(StoreError::Other(format!(
                                "{rtype}: base table renamed {base_raw} → {}; re-shred \
                                 does not support that",
                                new_rm.base_table().name
                            )));
                        }
                        let base_q = quote_ident(&base_raw);
                        let ids: Vec<String> = {
                            let sql = format!("SELECT \"id\" FROM {s_q}.{base_q} ORDER BY \"id\"");
                            let rows = conn.query(&sql, &[]).map_err(db_err)?;
                            let mut out = Vec::new();
                            for r in rows {
                                let r = r.map_err(db_err)?;
                                out.push(r.get::<_, String>(0).map_err(db_err)?);
                            }
                            out
                        };
                        for id in ids {
                            let Some(value) = recon_with_map(&conn, &s_q, old_rm, &id)? else {
                                continue;
                            };
                            let row = conn.query_row(
                                &format!(
                                    "SELECT \"version_id\", \
                                     TO_CHAR(\"last_updated\", \
                                     'YYYY-MM-DD\"T\"HH24:MI:SS.FF6') \
                                     FROM {s_q}.{base_q} WHERE \"id\" = :1 FOR UPDATE"
                                ),
                                &[&id],
                            );
                            let Ok(row) = row else { continue };
                            let version_id: i64 = row.get(0).map_err(db_err)?;
                            let ts: String = row.get(1).map_err(db_err)?;
                            conn.execute(
                                &format!("DELETE FROM {s_q}.{base_q} WHERE \"id\" = :1"),
                                &[&id],
                            )
                            .map_err(db_err)?;
                            let out = fhir_oracle_map::shred::shred(new_rm, &value)?;
                            write_shredded(
                                &conn, &s_q, &base_q, new_rm, &id, version_id, &ts, &out,
                            )?;
                            let back =
                                recon_with_map(&conn, &s_q, new_rm, &id)?.ok_or_else(|| {
                                    StoreError::Other(format!(
                                        "re-shred wrote {rtype}/{id} but it did not read back"
                                    ))
                                })?;
                            if fhir_oracle_map::canon::canonicalize(&back)
                                != fhir_oracle_map::canon::canonicalize(&value)
                            {
                                conn.rollback().map_err(db_err)?;
                                return Err(StoreError::Other(format!(
                                    "re-shred verification failed for {rtype}/{id}: the \
                                     new-shape reconstruction is not byte-identical. This \
                                     resource is rolled back, {reshredded} carried before \
                                     it remain carried, and no column has been dropped \
                                     (M14.40)"
                                )));
                            }
                            conn.commit().map_err(db_err)?;
                            reshredded += 1;
                        }
                    }
                    // Every moved source must now be empty, checked before the
                    // drops so a miss stops with the data still in place.
                    for (t, col, nt) in &moved {
                        let has: i64 = conn
                            .query_row(
                                &format!(
                                    "SELECT COUNT(*) FROM {s_q}.{} WHERE {} IS NOT NULL",
                                    quote_ident(t),
                                    quote_ident(col)
                                ),
                                &[],
                            )
                            .map_err(db_err)?
                            .get(0)
                            .map_err(db_err)?;
                        if has != 0 {
                            return Err(StoreError::Other(format!(
                                "re-shred left data behind in {t}.{col} (destined for \
                                 {nt}); nothing has been dropped — rerun to resume"
                            )));
                        }
                    }
                }

                // 4. Destructive last, each tolerating "already gone".
                for stmt in &drops {
                    apply(stmt, &[-942, -904])?;
                }
                // 4b. F-47 step 5: legacy "path" columns to the recorded
                //     bound (M14.38, M14.39) — a catalog-driven state
                //     machine, resumable like everything above, run before
                //     the meta write so a refusal leaves the old asset
                //     recorded.
                let converted = convert_path_columns(&conn, &map.schema, map.path_bound())?;
                // 5. Record what is now installed.
                let new_hex = hex_encode(
                    &map.to_gz_bytes()
                        .map_err(|e| StoreError::Other(e.to_string()))?,
                );
                for (k, v) in [
                    ("checksum", checksum.as_str()),
                    ("fhir_version", map.fhir_version.as_str()),
                    ("map_asset", new_hex.as_str()),
                ] {
                    write_meta_chunked(&conn, &meta, k, v)?;
                }
                conn.commit().map_err(db_err)?;
                Ok((n_add + converted, n_drop, reshredded))
            })
            .await
            .map_err(join_err)??;

        let folded = self.backfill_norm().await?;
        Ok(crate::UpgradeReport {
            reshredded,
            additive,
            destructive,
            folded,
        })
    }

    /// Fill `_norm` columns rows written before them left NULL (`O10.4a`).
    ///
    /// ROWID-keyset batches, not DISTINCT source values: `DISTINCT` and `=`
    /// are both illegal on a `CLOB` source (ORA-00932 / ORA-22848), and the
    /// cursor guarantees progress even where a folded value is empty —
    /// Oracle stores `''` as NULL, so under a values-based loop such a row
    /// would match the `IS NULL` predicate forever. Resumable by
    /// construction: the predicate is the work list, and batches commit as
    /// they go.
    ///
    /// # Errors
    /// On a database failure; a partial backfill is resumed by calling again.
    pub async fn backfill_norm(&self) -> Result<usize, StoreError> {
        const BATCH: usize = 500;
        let pool = self.pool.clone();
        let map = self.map.clone();
        tokio::task::spawn_blocking(move || -> Result<usize, StoreError> {
            let conn = pool.get().map_err(db_err)?;
            let s = quote_ident(&map.schema);
            let mut total = 0usize;
            for rm in map.resources.values() {
                for t in &rm.tables {
                    for (src, dst) in &t.norm_cols {
                        let table = quote_ident(&t.name);
                        let src_c = quote_ident(src);
                        let dst_c = quote_ident(dst);
                        let update = format!(
                            "UPDATE {s}.{table} SET {dst_c} = :1 \
                             WHERE ROWID = CHARTOROWID(:2)"
                        );
                        let mut cursor: Option<String> = None;
                        loop {
                            let rows: Vec<(String, String)> = {
                                let sql = match &cursor {
                                    None => format!(
                                        "SELECT ROWIDTOCHAR(ROWID), {src_c} FROM {s}.{table} \
                                         WHERE {dst_c} IS NULL AND {src_c} IS NOT NULL \
                                         ORDER BY ROWID FETCH FIRST {BATCH} ROWS ONLY"
                                    ),
                                    Some(_) => format!(
                                        "SELECT ROWIDTOCHAR(ROWID), {src_c} FROM {s}.{table} \
                                         WHERE {dst_c} IS NULL AND {src_c} IS NOT NULL \
                                         AND ROWID > CHARTOROWID(:1) \
                                         ORDER BY ROWID FETCH FIRST {BATCH} ROWS ONLY"
                                    ),
                                };
                                let result = match &cursor {
                                    None => conn.query(&sql, &[]),
                                    Some(c) => conn.query(&sql, &[c]),
                                }
                                .map_err(db_err)?;
                                let mut out = Vec::new();
                                for row in result.flatten() {
                                    let rid: String = row.get(0).map_err(db_err)?;
                                    let v: String = row.get(1).map_err(db_err)?;
                                    out.push((rid, v));
                                }
                                out
                            };
                            if rows.is_empty() {
                                break;
                            }
                            let n = rows.len();
                            for (rid, v) in &rows {
                                let folded = fhir_oracle_map::fold::fold(v);
                                // An empty fold stays NULL, exactly as the
                                // write path leaves it.
                                if !folded.is_empty() {
                                    conn.execute(&update, &[&folded.as_str(), &rid.as_str()])
                                        .map_err(db_err)?;
                                    total += 1;
                                }
                            }
                            conn.commit().map_err(db_err)?;
                            cursor = Some(rows.last().expect("non-empty").0.clone());
                            if n < BATCH {
                                break;
                            }
                        }
                    }
                }
            }
            Ok(total)
        })
        .await
        .map_err(join_err)?
    }

    /// Execute one raw statement, for tests that must manufacture a state the
    /// public surface refuses to produce — an install that predates the stored
    /// map asset, for example. Not part of the store's contract.
    #[doc(hidden)]
    pub async fn exec_raw(&self, sql: &str) -> Result<(), StoreError> {
        let pool = self.pool.clone();
        let sql = sql.to_string();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            conn.execute(&sql, &[]).map_err(db_err)?;
            conn.commit().map_err(db_err)?;
            Ok(())
        })
        .await
        .map_err(join_err)?
    }

    /// Drop every table this map generated, plus the schema-wide objects.
    ///
    /// # Errors
    /// On a database failure other than "object does not exist" (tolerated,
    /// same as every other port's `drop_schema`).
    pub async fn drop_schema(&self) -> Result<(), StoreError> {
        let pool = self.pool.clone();
        let map = self.map.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            let s = quote_ident(&map.schema);
            // Constraints (the Ext/Deep -> base FKs) first, or the table
            // drops below fail on a referenced parent. `user_constraints`
            // is scoped to the connecting user; since `M14.5` puts each
            // FHIR version in its own Oracle user, this connection's own
            // constraints are exactly this schema's.
            let fks = conn.query(
                "SELECT constraint_name, table_name FROM user_constraints \
                 WHERE constraint_type = 'R'",
                &[],
            );
            if let Ok(rows) = fks {
                for row in rows.flatten() {
                    let cname: String = row.get(0).unwrap_or_default();
                    let tname: String = row.get(1).unwrap_or_default();
                    let _ = conn.execute(
                        &format!(
                            "ALTER TABLE {s}.{} DROP CONSTRAINT {}",
                            quote_ident(&tname),
                            quote_ident(&cname)
                        ),
                        &[],
                    );
                }
            }
            for rm in map.resources.values() {
                for t in &rm.tables {
                    let _ = conn.execute(
                        &format!(
                            "DROP TABLE {s}.{} CASCADE CONSTRAINTS",
                            quote_ident(&t.name)
                        ),
                        &[],
                    );
                }
            }
            for t in [
                "fhir_oracle_meta",
                "fhir_oracle_access_log",
                "fhir_oracle_countersign",
            ] {
                let _ = conn.execute(&format!("DROP TABLE {s}.{}", quote_ident(t)), &[]);
            }
            conn.commit().map_err(db_err)?;
            Ok(())
        })
        .await
        .map_err(join_err)?
    }
}

// ------------------------------------------------------------------- put

impl OracleStore {
    /// Create or replace a resource, appending one history row (spec T1/T2).
    ///
    /// # Errors
    /// If the resource fails to shred, or on a database failure.
    pub async fn put(
        &self,
        resource: &serde_json::Value,
        audit: &crate::Audit,
    ) -> Result<crate::PutOutcome, StoreError> {
        let map = self.map.clone();
        let pool = self.pool.clone();
        let keys = self.keys.clone();
        let resource = resource.clone();
        let audit = audit.clone();
        tokio::task::spawn_blocking(move || {
            let rtype = resource
                .get("resourceType")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| StoreError::Other("resource has no resourceType".into()))?
                .to_string();
            let rm = map
                .resources
                .get(&rtype)
                .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
            let out = fhir_oracle_map::shred::shred(rm, &resource)?;
            let id = out
                .id
                .clone()
                .ok_or_else(|| StoreError::Other("resource has no id".into()))?;
            let canon = fhir_oracle_map::canon::canonicalize(&resource);
            let ts = utc_micros(std::time::SystemTime::now());

            let conn = pool.get().map_err(db_err)?;
            let s = quote_ident(&map.schema);
            let base = quote_ident(&rm.base_table().name);
            let hist = qualified(
                &map.schema,
                &rm.find_table(fhir_oracle_map::model::TableKind::History)
                    .map(|(_, t)| t.name.clone())
                    .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?,
            );

            let outcome = put_in_tx(
                &conn, &keys, &s, &base, &hist, rm, &out, &id, &canon, &ts, &audit,
            );
            match &outcome {
                Ok(_) => conn.commit().map_err(db_err)?,
                Err(_) => {
                    let _ = conn.rollback();
                }
            }
            outcome
        })
        .await
        .map_err(join_err)?
    }
}

#[allow(clippy::too_many_arguments)]
fn put_in_tx(
    conn: &Connection,
    keys: &crate::chain::KeyRing,
    s: &str,
    base: &str,
    hist: &str,
    rm: &fhir_oracle_map::model::ResourceMap,
    out: &fhir_oracle_map::shred::ShredOut,
    id: &str,
    canon: &str,
    ts: &str,
    audit: &crate::Audit,
) -> Result<crate::PutOutcome, StoreError> {
    // H5.4: serialise writers for this resource id before reading the chain
    // tip. `FOR UPDATE` holds the row lock until commit/rollback, so a second
    // writer for the same id blocks here rather than racing the tip read
    // below. A create has no base row to lock; racing creates of the same id
    // still resolve on the history table's primary key.
    let _lock = conn.query_row(
        &format!("SELECT \"version_id\" FROM {s}.{base} WHERE \"id\" = :1 FOR UPDATE"),
        &[&id],
    );

    let prev_row = conn.query_row(
        &format!(
            "SELECT \"version_id\", \"row_hash\", \"row_hash_sha3\" FROM {hist} \
             WHERE \"id\" = :1 AND \"version_id\" = \
             (SELECT MAX(\"version_id\") FROM {hist} WHERE \"id\" = :1)"
        ),
        &[&id],
    );
    let (version_id, prev_256, prev_3): ChainTip = match prev_row {
        Ok(r) => (
            r.get::<usize, Option<i64>>(0).unwrap_or(None).unwrap_or(0) + 1,
            r.get::<usize, Option<Vec<u8>>>(1).unwrap_or(None),
            r.get::<usize, Option<Vec<u8>>>(2).unwrap_or(None),
        ),
        Err(_) => (1, None, None),
    };

    let existed = conn
        .query_row(
            &format!("SELECT 1 FROM {s}.{base} WHERE \"id\" = :1"),
            &[&id],
        )
        .is_ok();
    if existed {
        conn.execute(&format!("DELETE FROM {s}.{base} WHERE \"id\" = :1"), &[&id])
            .map_err(db_err)?;
    }

    write_shredded(conn, s, base, rm, id, version_id, ts, out)?;

    let op = if existed { "U" } else { "C" };
    let pre = crate::chain::preimage(id, version_id, ts, op, Some(canon), &audit.actor);
    let (row_hash, row_sha3) = crate::chain::link(prev_256.as_deref(), prev_3.as_deref(), &pre);
    let row_mac = keys
        .signing()
        .map(|k| crate::chain::mac(k, prev_256.as_deref(), &pre));

    let cols = [
        "\"id\"",
        "\"version_id\"",
        "\"last_updated\"",
        "\"op\"",
        "\"resource\"",
        "\"actor\"",
        "\"actor_source\"",
        "\"client\"",
        "\"request_id\"",
        "\"reason\"",
        "\"prev_hash\"",
        "\"row_hash\"",
        "\"prev_hash_sha3\"",
        "\"row_hash_sha3\"",
        "\"row_mac\"",
    ]
    .map(String::from)
    .to_vec();
    let vals = vec![
        Bound::Str(Some(id.to_string())),
        Bound::I64(Some(version_id)),
        Bound::Timestamp(parse_ts(ts)),
        Bound::Str(Some(op.to_string())),
        Bound::Str(Some(canon.to_string())),
        Bound::Str(Some(audit.actor.clone())),
        Bound::Str(audit.actor_source.clone()),
        Bound::Str(audit.client.clone()),
        Bound::Str(audit.request_id.clone()),
        Bound::Str(audit.reason.clone()),
        Bound::Bytes(prev_256),
        Bound::Bytes(Some(row_hash)),
        Bound::Bytes(prev_3),
        Bound::Bytes(Some(row_sha3)),
        Bound::Str(row_mac),
    ];
    insert_row(conn, hist, &cols, &vals)?;

    Ok(crate::PutOutcome {
        id: id.to_string(),
        version_id,
        created: !existed,
    })
}

// ------------------------------------------------------------------- get

impl OracleStore {
    /// Read a resource back, reconstructed from its rows.
    ///
    /// # Errors
    /// If the resource type is unknown, a stored row fails to parse, or on a
    /// database failure.
    pub async fn get(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Option<serde_json::Value>, StoreError> {
        let map = self.map.clone();
        let pool = self.pool.clone();
        let rtype = rtype.to_string();
        let id = id.to_string();
        tokio::task::spawn_blocking(move || {
            let rm = map
                .resources
                .get(&rtype)
                .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
            let conn = pool.get().map_err(db_err)?;
            let s = quote_ident(&map.schema);

            // R4.5, NOT met — `SET TRANSACTION READ ONLY` was tried here as
            // the multi-statement snapshot `M14.19` named as the likely
            // answer, and running it live (not merely compiling it) found it
            // reliably broken rather than merely unverified: on any session
            // that has executed DDL — which every pooled connection here has,
            // since `init` runs on this same pool — a later `SET TRANSACTION
            // READ ONLY` followed by any `SELECT` fails outright with
            // `ORA-01466: unable to read data - table definition has
            // changed`, reproduced with a minimal two-statement probe
            // (`CREATE TABLE` + commit, then `SET TRANSACTION READ ONLY`,
            // then `SELECT`) with no application logic involved at all. This
            // is a session-level Oracle behavior — DDL poisons that session
            // for any later read-only/serializable transaction — not a bug
            // in how this store calls it. Removed rather than left in
            // silently failing every `get`; this engine's default `READ
            // COMMITTED` still gives each individual statement its own
            // consistent read, so this is a regression from "believed
            // fixed" to "known undecided" (matching `fhir-mssql` before its
            // own `R4.5` fix — see that port's `M14.25`), not a regression
            // from a working state.
            let r = recon_with_map(&conn, &s, rm, &id)?;
            Ok(r)
        })
        .await
        .map_err(join_err)?
    }
}

// --------------------------------------------------------- history / vread

impl OracleStore {
    fn hist_target(&self, rtype: &str) -> Result<(String, String), StoreError> {
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let hist = rm
            .find_table(fhir_oracle_map::model::TableKind::History)
            .map(|(_, t)| quote_ident(&t.name))
            .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?;
        Ok((quote_ident(&self.map.schema), hist))
    }

    /// Every stored version of a resource, newest first.
    ///
    /// # Errors
    /// If the resource type is unknown, a stored resource fails to parse, or
    /// on a database failure.
    pub async fn history(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Vec<crate::HistEntry>, StoreError> {
        let (s, hist) = self.hist_target(rtype)?;
        let pool = self.pool.clone();
        let id = id.to_string();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            let rows = conn
                .query(
                    &format!(
                        "SELECT \"version_id\",\"last_updated\",\"op\",\"resource\" FROM {s}.{hist} \
                         WHERE \"id\" = :1 ORDER BY \"version_id\" DESC"
                    ),
                    &[&id],
                )
                .map_err(db_err)?;
            let mut out = Vec::new();
            for row in rows {
                out.push(hist_entry(&row.map_err(db_err)?)?);
            }
            Ok(out)
        })
        .await
        .map_err(join_err)?
    }

    /// One specific version, as it was stored.
    ///
    /// # Errors
    /// If the resource type is unknown, the stored resource fails to parse,
    /// or on a database failure.
    pub async fn vread(
        &self,
        rtype: &str,
        id: &str,
        version_id: i64,
    ) -> Result<Option<crate::HistEntry>, StoreError> {
        let (s, hist) = self.hist_target(rtype)?;
        let pool = self.pool.clone();
        let id = id.to_string();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            let row = conn.query_row(
                &format!(
                    "SELECT \"version_id\",\"last_updated\",\"op\",\"resource\" FROM {s}.{hist} \
                     WHERE \"id\" = :1 AND \"version_id\" = :2"
                ),
                &[&id, &version_id],
            );
            match row {
                Ok(r) => Ok(Some(hist_entry(&r)?)),
                Err(_) => Ok(None),
            }
        })
        .await
        .map_err(join_err)?
    }

    /// Delete a resource, leaving a tombstone in history.
    ///
    /// # Errors
    /// If the resource type is unknown or on a database failure.
    pub async fn delete(
        &self,
        rtype: &str,
        id: &str,
        audit: &crate::Audit,
    ) -> Result<Option<i64>, StoreError> {
        let map = self.map.clone();
        let pool = self.pool.clone();
        let keys = self.keys.clone();
        let rtype = rtype.to_string();
        let id = id.to_string();
        let audit = audit.clone();
        tokio::task::spawn_blocking(move || {
            let rm = map
                .resources
                .get(&rtype)
                .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
            let conn = pool.get().map_err(db_err)?;
            let s = quote_ident(&map.schema);
            let base = quote_ident(&rm.base_table().name);
            let hist = qualified(
                &map.schema,
                &rm.find_table(fhir_oracle_map::model::TableKind::History)
                    .map(|(_, t)| t.name.clone())
                    .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?,
            );
            let outcome = delete_in_tx(&conn, &keys, &s, &base, &hist, &id, &audit);
            match &outcome {
                Ok(_) => conn.commit().map_err(db_err)?,
                Err(_) => {
                    let _ = conn.rollback();
                }
            }
            outcome
        })
        .await
        .map_err(join_err)?
    }
}

fn delete_in_tx(
    conn: &Connection,
    keys: &crate::chain::KeyRing,
    s: &str,
    base: &str,
    hist: &str,
    id: &str,
    audit: &crate::Audit,
) -> Result<Option<i64>, StoreError> {
    let _lock = conn.query_row(
        &format!("SELECT \"version_id\" FROM {s}.{base} WHERE \"id\" = :1 FOR UPDATE"),
        &[&id],
    );
    let existed = conn
        .query_row(
            &format!("SELECT 1 FROM {s}.{base} WHERE \"id\" = :1"),
            &[&id],
        )
        .is_ok();
    if !existed {
        return Ok(None);
    }
    conn.execute(&format!("DELETE FROM {s}.{base} WHERE \"id\" = :1"), &[&id])
        .map_err(db_err)?;

    let prev_row = conn.query_row(
        &format!(
            "SELECT \"version_id\",\"row_hash\",\"row_hash_sha3\" FROM {hist} \
             WHERE \"id\" = :1 AND \"version_id\" = \
             (SELECT MAX(\"version_id\") FROM {hist} WHERE \"id\" = :1)"
        ),
        &[&id],
    );
    let (version_id, prev_256, prev_3): ChainTip = match prev_row {
        Ok(r) => (
            r.get::<usize, Option<i64>>(0).unwrap_or(None).unwrap_or(0) + 1,
            r.get::<usize, Option<Vec<u8>>>(1).unwrap_or(None),
            r.get::<usize, Option<Vec<u8>>>(2).unwrap_or(None),
        ),
        Err(_) => (1, None, None),
    };

    let ts = utc_micros(std::time::SystemTime::now());
    let pre = crate::chain::preimage(id, version_id, &ts, "D", None, &audit.actor);
    let (row_hash, row_sha3) = crate::chain::link(prev_256.as_deref(), prev_3.as_deref(), &pre);
    let row_mac = keys
        .signing()
        .map(|k| crate::chain::mac(k, prev_256.as_deref(), &pre));

    let cols = [
        "\"id\"",
        "\"version_id\"",
        "\"last_updated\"",
        "\"op\"",
        "\"actor\"",
        "\"actor_source\"",
        "\"client\"",
        "\"request_id\"",
        "\"reason\"",
        "\"prev_hash\"",
        "\"row_hash\"",
        "\"prev_hash_sha3\"",
        "\"row_hash_sha3\"",
        "\"row_mac\"",
    ]
    .map(String::from)
    .to_vec();
    let vals = vec![
        Bound::Str(Some(id.to_string())),
        Bound::I64(Some(version_id)),
        Bound::Timestamp(parse_ts(&ts)),
        Bound::Str(Some("D".to_string())),
        Bound::Str(Some(audit.actor.clone())),
        Bound::Str(audit.actor_source.clone()),
        Bound::Str(audit.client.clone()),
        Bound::Str(audit.request_id.clone()),
        Bound::Str(audit.reason.clone()),
        Bound::Bytes(prev_256),
        Bound::Bytes(Some(row_hash)),
        Bound::Bytes(prev_3),
        Bound::Bytes(Some(row_sha3)),
        Bound::Str(row_mac),
    ];
    insert_row(conn, hist, &cols, &vals)?;
    Ok(Some(version_id))
}

// --------------------------------------------------------------- audit chain

impl OracleStore {
    /// Recompute every history chain and report anywhere the stored hashes
    /// or keyed tag disagree with what the rows themselves imply.
    ///
    /// # Errors
    /// On a database failure.
    pub async fn verify_audit(&self) -> Result<Vec<crate::ChainBreak>, StoreError> {
        let map = self.map.clone();
        let pool = self.pool.clone();
        let keys = self.keys.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            let s = quote_ident(&map.schema);
            let mut breaks = Vec::new();

            let cs_table = qualified(&map.schema, "fhir_oracle_countersign");
            let mut countersigns: std::collections::HashMap<(String, String, i64), String> =
                std::collections::HashMap::new();
            if let Ok(rows) = conn.query(&format!("SELECT \"rtype\",\"id\",\"version_id\",\"row_mac\" FROM {cs_table}"), &[]) {
                for row in rows.flatten() {
                    let rtype: String = row.get::<usize, Option<String>>(0).unwrap_or(None).unwrap_or_default();
                    let id: String = row.get::<usize, Option<String>>(1).unwrap_or(None).unwrap_or_default();
                    let version_id: i64 = row.get::<usize, Option<i64>>(2).unwrap_or(None).unwrap_or(0);
                    let row_mac: String = row.get::<usize, Option<String>>(3).unwrap_or(None).unwrap_or_default();
                    countersigns.insert((rtype, id, version_id), row_mac);
                }
            }

            for rm in map.resources.values() {
                let Some((_, t)) = rm.find_table(fhir_oracle_map::model::TableKind::History) else {
                    continue;
                };
                let hist = quote_ident(&t.name);
                let rows = conn
                    .query(
                        &format!(
                            "SELECT \"id\",\"version_id\",\"last_updated\",\"op\",\"resource\",\"actor\",\
                                    \"prev_hash\",\"row_hash\",\"prev_hash_sha3\",\"row_hash_sha3\",\"row_mac\" \
                             FROM {s}.{hist} ORDER BY \"id\", \"version_id\""
                        ),
                        &[],
                    )
                    .map_err(db_err)?;

                let mut cur = String::new();
                let mut prior_256: Option<Vec<u8>> = None;
                let mut prior_3: Option<Vec<u8>> = None;

                for row in rows {
                    let row = row.map_err(db_err)?;
                    let id: String = row.get::<usize, Option<String>>(0).unwrap_or(None).unwrap_or_default();
                    let version_id: i64 = row.get::<usize, Option<i64>>(1).unwrap_or(None).unwrap_or(0);
                    // TIMESTAMP(6) formatted to match `utc_micros`'s own
                    // output exactly, or every row fails to verify against
                    // itself — see `fhir-mssql`'s analogous comment on
                    // `hist_entry`, which this port could not confirm live.
                    let last_updated = row
                        .get::<usize, Option<chrono::NaiveDateTime>>(2)
                        .unwrap_or(None)
                        .map(|d| d.format("%Y-%m-%dT%H:%M:%S%.6f").to_string())
                        .unwrap_or_default();
                    let op: String = row.get::<usize, Option<String>>(3).unwrap_or(None).unwrap_or_default();
                    let resource: Option<String> = row.get(4).unwrap_or(None);
                    let actor: String = row.get::<usize, Option<String>>(5).unwrap_or(None).unwrap_or_default();
                    let prev_256: Option<Vec<u8>> = row.get(6).unwrap_or(None);
                    let row_256: Option<Vec<u8>> = row.get(7).unwrap_or(None);
                    let prev_3: Option<Vec<u8>> = row.get(8).unwrap_or(None);
                    let row_3: Option<Vec<u8>> = row.get(9).unwrap_or(None);
                    let row_mac: Option<String> = row.get(10).unwrap_or(None);

                    if id != cur {
                        cur.clone_from(&id);
                        prior_256 = None;
                        prior_3 = None;
                    }

                    let pre = crate::chain::preimage(&id, version_id, &last_updated, &op, resource.as_deref(), &actor);

                    check_mac(&keys, &countersigns, &rm.name, &id, version_id, row_mac.as_deref(), prev_256.as_deref(), &pre, &mut breaks);

                    if op == "P" {
                        prior_256 = row_256;
                        prior_3 = row_3;
                        continue;
                    }

                    let (Some(stored_256), Some(stored_3)) = (&row_256, &row_3) else {
                        prior_256 = row_256;
                        prior_3 = row_3;
                        continue;
                    };

                    let (want_256, want_3) = crate::chain::link(prior_256.as_deref(), prior_3.as_deref(), &pre);
                    for (algorithm, stored, want, stored_link, prior) in [
                        ("sha256", stored_256, &want_256, &prev_256, &prior_256),
                        ("sha3-256", stored_3, &want_3, &prev_3, &prior_3),
                    ] {
                        let bad = !crate::chain::digests_equal(stored, want);
                        let unlinked = stored_link.as_deref() != prior.as_deref();
                        if bad || unlinked {
                            breaks.push(crate::ChainBreak::new(
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

                    prior_256 = row_256;
                    prior_3 = row_3;
                }
            }
            Ok(breaks)
        })
        .await
        .map_err(join_err)?
    }
}

/// Verify one row's keyed tag — ported from `fhir-mssql-store`'s
/// `check_mac`, which is itself ported from `fhir-mysql-store`'s. Records a
/// finding only when the tag is present and wrong.
#[allow(clippy::too_many_arguments)]
fn check_mac(
    keys: &crate::chain::KeyRing,
    countersigns: &std::collections::HashMap<(String, String, i64), String>,
    rtype: &str,
    id: &str,
    version_id: i64,
    stored: Option<&str>,
    prev_sha256: Option<&[u8]>,
    pre: &[u8],
    breaks: &mut Vec<crate::ChainBreak>,
) {
    use crate::chain::MacCheck;

    let own = keys.check(stored, prev_sha256, pre);
    let verdict = match (
        &own,
        countersigns.get(&(rtype.to_string(), id.to_string(), version_id)),
    ) {
        (MacCheck::Absent | MacCheck::Unverifiable { .. }, Some(have)) => match keys.signing() {
            Some(k)
                if crate::chain::digests_equal(
                    crate::chain::mac(k, prev_sha256, pre).as_bytes(),
                    have.as_bytes(),
                ) =>
            {
                MacCheck::Ok
            }
            _ => own,
        },
        _ => own,
    };

    match verdict {
        MacCheck::Mismatch => breaks.push(crate::ChainBreak::new(
            rtype,
            id,
            version_id,
            "hmac-sha256",
            "keyed tag does not match",
        )),
        MacCheck::Ok | MacCheck::Absent => {}
        MacCheck::Unverifiable { key_id } => tracing::warn!(
            %rtype, %id, version_id, %key_id,
            "row is signed with a key this process does not hold; not checked"
        ),
        MacCheck::Malformed => tracing::warn!(
            %rtype, %id, version_id,
            "row_mac is not <key-id>:<hex>; not checked"
        ),
    }
}

// ------------------------------------------------------------------- purge

impl OracleStore {
    /// GDPR Art. 17: replace a resource's history with a tombstone.
    ///
    /// # Errors
    /// If the resource type is unknown or on a database failure.
    pub async fn purge(
        &self,
        rtype: &str,
        id: &str,
        audit: &crate::Audit,
    ) -> Result<crate::PurgeReport, StoreError> {
        let map = self.map.clone();
        let pool = self.pool.clone();
        let keys = self.keys.clone();
        let rtype = rtype.to_string();
        let id = id.to_string();
        let audit = audit.clone();
        tokio::task::spawn_blocking(move || {
            let rm = map
                .resources
                .get(&rtype)
                .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
            let conn = pool.get().map_err(db_err)?;
            let s = quote_ident(&map.schema);
            let base = quote_ident(&rm.base_table().name);
            let hist = qualified(
                &map.schema,
                &rm.find_table(fhir_oracle_map::model::TableKind::History)
                    .map(|(_, t)| t.name.clone())
                    .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?,
            );

            let result = (|| -> Result<crate::PurgeReport, StoreError> {
                let existed = conn
                    .query_row(
                        &format!("SELECT 1 FROM {s}.{base} WHERE \"id\" = :1"),
                        &[&id],
                    )
                    .is_ok();

                // Counted before the delete, the same fix `fhir-mssql` needed
                // live (its `ExecuteResult::total()` double-counted a nested
                // trigger delete) — untested here, but taken as the safer
                // default rather than trusting a post-delete row count this
                // port has never run either way.
                let versions_erased: i64 = conn
                    .query_row(
                        &format!("SELECT COUNT(*) FROM {hist} WHERE \"id\" = :1"),
                        &[&id],
                    )
                    .ok()
                    .and_then(|r| r.get::<usize, Option<i64>>(0).unwrap_or(None))
                    .unwrap_or(0);

                // The `M3.17` append-only trigger's escape hatch (`M14.29`):
                // `CLIENT_INFO` is connection-scoped, not transaction-scoped,
                // so on a *pooled* connection it MUST be cleared again before
                // this connection can be trusted for another caller's
                // ordinary `DELETE` — set immediately before the delete it
                // authorises, cleared in every exit path, never assumed to
                // reset itself.
                conn.execute(
                    "BEGIN DBMS_APPLICATION_INFO.SET_CLIENT_INFO(:1); END;",
                    &[&"fhir_oracle_erasure=on"],
                )
                .map_err(db_err)?;

                conn.execute(&format!("DELETE FROM {hist} WHERE \"id\" = :1"), &[&id])
                    .map_err(db_err)?;

                conn.execute(
                    "BEGIN DBMS_APPLICATION_INFO.SET_CLIENT_INFO(:1); END;",
                    &[&Option::<&str>::None],
                )
                .map_err(db_err)?;

                if existed {
                    conn.execute(&format!("DELETE FROM {s}.{base} WHERE \"id\" = :1"), &[&id])
                        .map_err(db_err)?;
                }

                let ts = utc_micros(std::time::SystemTime::now());
                let pre = crate::chain::preimage(&id, 0, &ts, "P", None, &audit.actor);
                let (row_hash, row_sha3) = crate::chain::link(None, None, &pre);
                let row_mac = keys.signing().map(|k| crate::chain::mac(k, None, &pre));

                let cols = [
                    "\"id\"",
                    "\"version_id\"",
                    "\"last_updated\"",
                    "\"op\"",
                    "\"actor\"",
                    "\"actor_source\"",
                    "\"client\"",
                    "\"request_id\"",
                    "\"reason\"",
                    "\"row_hash\"",
                    "\"row_hash_sha3\"",
                    "\"row_mac\"",
                ]
                .map(String::from)
                .to_vec();
                let vals = vec![
                    Bound::Str(Some(id.clone())),
                    Bound::I64(Some(0)),
                    Bound::Timestamp(parse_ts(&ts)),
                    Bound::Str(Some("P".to_string())),
                    Bound::Str(Some(audit.actor.clone())),
                    Bound::Str(audit.actor_source.clone()),
                    Bound::Str(audit.client.clone()),
                    Bound::Str(audit.request_id.clone()),
                    Bound::Str(audit.reason.clone()),
                    Bound::Bytes(Some(row_hash)),
                    Bound::Bytes(Some(row_sha3)),
                    Bound::Str(row_mac),
                ];
                insert_row(&conn, &hist, &cols, &vals)?;

                Ok(crate::PurgeReport {
                    versions_erased: versions_erased as u64,
                    existed,
                })
            })();

            match &result {
                Ok(_) => conn.commit().map_err(db_err)?,
                Err(_) => {
                    let _ = conn.rollback();
                }
            }
            // Belt and suspenders: if anything between setting the flag and
            // clearing it failed, the inline clear above was skipped.
            let _ = conn.execute(
                "BEGIN DBMS_APPLICATION_INFO.SET_CLIENT_INFO(:1); END;",
                &[&Option::<&str>::None],
            );
            result
        })
        .await
        .map_err(join_err)?
    }
}

// --------------------------------------------------------- access log & search

impl OracleStore {
    /// Record one disclosure: who saw what, and what they were told (`PR12.5`).
    ///
    /// # Errors
    /// On a database failure.
    pub async fn log_access(&self, rec: &crate::AccessRecord) -> Result<(), StoreError> {
        let map = self.map.clone();
        let pool = self.pool.clone();
        let rec = rec.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            let s = quote_ident(&map.schema);
            let cols = [
                "\"ts\"",
                "\"request_id\"",
                "\"actor\"",
                "\"actor_source\"",
                "\"client\"",
                "\"interaction\"",
                "\"rtype\"",
                "\"id\"",
                "\"version_id\"",
                "\"outcome\"",
                "\"result_count\"",
                "\"reason\"",
            ]
            .map(String::from)
            .to_vec();
            let vals = vec![
                Bound::Timestamp(parse_ts(&utc_micros(std::time::SystemTime::now()))),
                Bound::Str(rec.audit.request_id.clone()),
                Bound::Str(Some(rec.audit.actor.clone())),
                Bound::Str(rec.audit.actor_source.clone()),
                Bound::Str(rec.audit.client.clone()),
                Bound::Str(Some(rec.interaction.clone())),
                Bound::Str(rec.rtype.clone()),
                Bound::Str(rec.id.clone()),
                Bound::I64(rec.version_id),
                Bound::Str(Some(rec.outcome.clone())),
                Bound::I64(rec.result_count),
                Bound::Str(rec.audit.reason.clone()),
            ];
            insert_row(
                &conn,
                &format!("{s}.\"fhir_oracle_access_log\""),
                &cols,
                &vals,
            )
            .map_err(|e| StoreError::Other(format!("log_access: {e}")))?;
            conn.commit().map_err(db_err)?;
            Ok(())
        })
        .await
        .map_err(join_err)?
    }

    /// Log several disclosures at once. Test/dev convenience.
    ///
    /// # Errors
    /// On a database failure.
    pub async fn log_access_batch(&self, recs: &[crate::AccessRecord]) -> Result<(), StoreError> {
        for r in recs {
            self.log_access(r).await?;
        }
        Ok(())
    }

    /// Row count of the access log. Test/dev convenience.
    ///
    /// # Errors
    /// On a database failure.
    pub async fn access_log_len(&self) -> Result<i64, StoreError> {
        let map = self.map.clone();
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(db_err)?;
            let log = qualified(&map.schema, "fhir_oracle_access_log");
            let row = conn
                .query_row(&format!("SELECT COUNT(*) FROM {log}"), &[])
                .map_err(db_err)?;
            Ok(row
                .get::<usize, Option<i64>>(0)
                .unwrap_or(None)
                .unwrap_or(0))
        })
        .await
        .map_err(join_err)?
    }
}

// --------------------------------------------------------------------- search

impl OracleStore {
    /// Just the ids.
    ///
    /// # Errors
    /// If the resource type or a search parameter is unsupported, or on a
    /// database failure.
    pub async fn search(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
    ) -> Result<Vec<String>, StoreError> {
        Ok(self
            .search_page(rtype, params, count, offset, &[], false, None)
            .await?
            .ids)
    }

    /// A page plus, optionally, the total.
    ///
    /// # Errors
    /// If the resource type or a search parameter is unsupported, or on a
    /// database failure.
    pub async fn search_full(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        sort: &[crate::oracle_search::SortKey],
        want_total: bool,
    ) -> Result<crate::SearchOutcome, StoreError> {
        self.search_page(rtype, params, count, offset, sort, want_total, None)
            .await
    }

    /// A page, with an optional keyset cursor.
    ///
    /// # Errors
    /// If the resource type or a search parameter is unsupported, or on a
    /// database failure.
    #[allow(clippy::too_many_arguments)]
    pub async fn search_page(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        sort: &[crate::oracle_search::SortKey],
        want_total: bool,
        after_id: Option<&str>,
    ) -> Result<crate::SearchOutcome, StoreError> {
        let map = self.map.clone();
        let pool = self.pool.clone();
        let rtype = rtype.to_string();
        let params = params.to_vec();
        let sort = sort.to_vec();
        let after_id = after_id.map(str::to_string);
        tokio::task::spawn_blocking(move || {
            let rm = map
                .resources
                .get(&rtype)
                .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
            let q = crate::oracle_search::build_search_sql(
                &map,
                rm,
                &params,
                count,
                offset,
                &sort,
                after_id.as_deref(),
            )?;
            let conn = pool.get().map_err(db_err)?;

            let total = if want_total {
                let binds: Vec<crate::oracle_search::Bind> =
                    q.binds.iter().take(q.count_binds).cloned().collect();
                let params: Vec<Bound> = binds.into_iter().map(bind_to_bound).collect();
                let refs: Vec<&dyn ToSql> = params.iter().map(|b| b as &dyn ToSql).collect();
                let row = conn
                    .query_row(&q.count_sql, &refs)
                    .map_err(|e| StoreError::Other(format!("count: {e}\n{}", q.count_sql)))?;
                row.get::<usize, Option<i64>>(0).unwrap_or(None)
            } else {
                None
            };

            let params: Vec<Bound> = q.binds.iter().cloned().map(bind_to_bound).collect();
            let refs: Vec<&dyn ToSql> = params.iter().map(|b| b as &dyn ToSql).collect();
            let rows = conn
                .query(&q.sql, &refs)
                .map_err(|e| StoreError::Other(format!("search: {e}\n{}", q.sql)))?;
            let mut ids = Vec::new();
            for row in rows {
                let row = row.map_err(|e| StoreError::Other(format!("search: {e}\n{}", q.sql)))?;
                ids.push(
                    row.get::<usize, Option<String>>(0)
                        .unwrap_or(None)
                        .unwrap_or_default(),
                );
            }
            Ok(crate::SearchOutcome { ids, total })
        })
        .await
        .map_err(join_err)?
    }
}

fn bind_to_bound(b: crate::oracle_search::Bind) -> Bound {
    match b {
        crate::oracle_search::Bind::Str(s) => Bound::Str(Some(s)),
        crate::oracle_search::Bind::Bytes(b) => Bound::Bytes(Some(b)),
        crate::oracle_search::Bind::I64(i) => Bound::I64(Some(i)),
    }
}

// ------------------------------------------------------------------- binding

/// A value ready to bind, independent of borrowed lifetimes — needed because
/// rows are built up in an owned `Vec<Bound>` before the statement that
/// consumes them exists.
///
/// **`Timestamp`/`CalDate` exist because `Str` alone was wrong, found live.**
/// Binding a `TIMESTAMP(6)`/`DATE` column from a plain `NVARCHAR2` string and
/// letting Oracle convert it implicitly failed on the very first `put()`:
/// `ORA-01843: An invalid month was specified` — Oracle's implicit
/// string-to-timestamp conversion uses the session's `NLS_TIMESTAMP_FORMAT`,
/// not ISO 8601, unlike SQL Server's `DATETIME2` (`M14.10` there). Binding
/// real `chrono` values instead sidesteps session format settings entirely,
/// the same reasoning `fhir-mssql`'s `chrono`-typed reads already rely on.
///
/// Otherwise only three variants, unlike `fhir-mssql`'s four: this store
/// never needs a typed-NULL workaround the way tiberius did (`Bound::Str`
/// bound `NULL` as `ColumnData::String(None)` regardless of the target
/// column's real type, which SQL Server rejected against non-NVARCHAR
/// columns). The `oracle` crate's `impl<T: ToSql + ToSqlNull> ToSql for
/// Option<T>` reports the correct Oracle type for `None` per `T`, so
/// `Bound::Str(None)` already binds as a typed NVARCHAR2 null and
/// `Bound::I64(None)`/`Bound::Bytes(None)` likewise.
enum Bound {
    Str(Option<String>),
    I64(Option<i64>),
    Bytes(Option<Vec<u8>>),
    Timestamp(Option<chrono::NaiveDateTime>),
    CalDate(Option<chrono::NaiveDate>),
}

impl ToSql for Bound {
    fn oratype(&self, conn: &Connection) -> oracle::Result<OracleType> {
        match self {
            Bound::Str(v) => v.oratype(conn),
            Bound::I64(v) => v.oratype(conn),
            Bound::Bytes(v) => v.oratype(conn),
            Bound::Timestamp(v) => v.oratype(conn),
            Bound::CalDate(v) => v.oratype(conn),
        }
    }
    fn to_sql(&self, val: &mut SqlValue) -> oracle::Result<()> {
        match self {
            Bound::Str(v) => v.to_sql(val),
            Bound::I64(v) => v.to_sql(val),
            Bound::Bytes(v) => v.to_sql(val),
            Bound::Timestamp(v) => v.to_sql(val),
            Bound::CalDate(v) => v.to_sql(val),
        }
    }
}

/// Parse this store's own `utc_micros()` output, or the shared
/// `value::datetime_sort`/`date_sort` output — both ISO 8601, differing only
/// in a trailing `Z` and whether a fractional second is present. Both are
/// handled by stripping `Z` and parsing with an optional-fraction format.
///
/// Returns `None` (not a `Result`) on anything unparseable, which callers
/// turn into a null bind rather than a hard failure — an offset-bearing
/// instant (`+02:00`) is a shape this has not been run against and this
/// store would rather store nothing than store the wrong instant.
fn parse_ts(s: &str) -> Option<chrono::NaiveDateTime> {
    let s = s.strip_suffix('Z').unwrap_or(s);
    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f").ok()
}

fn parse_date(s: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
}

/// The correctly-typed NULL for an arbitrary data column, chosen the same
/// way `col_sql` chose its physical type.
fn null_for_ty(ty: ColTy) -> Bound {
    match ty {
        ColTy::Bool => Bound::I64(None),
        ColTy::Int | ColTy::BigInt => Bound::I64(None),
        ColTy::Digest => Bound::Bytes(None),
        ColTy::Timestamptz => Bound::Timestamp(None),
        ColTy::Date => Bound::CalDate(None),
        ColTy::Numeric | ColTy::Text | ColTy::TextC | ColTy::TextIdx | ColTy::Jsonb => {
            Bound::Str(None)
        }
    }
}

/// One `INSERT`, one row — not batched, for the same reason `fhir-mssql`
/// does not batch: a variable-width `INSERT ... VALUES (..),(..)` needs its
/// placeholder count rebuilt per batch size, and there is no live server
/// here to develop that against.
/// `target` is the **fully schema-qualified** table — `"schema"."table"` —
/// never a bare table name. Found live: three call sites (the history-table
/// insert in `put`, `delete`, and `purge`) used to pass an *already*
/// qualified `hist` alongside a separate `schema` argument this function
/// then qualified again, producing `"R5"."R5"."patient_history"` and
/// `ORA-00926: Missing VALUES or SET keyword`. Taking one pre-qualified
/// string removes the chance of qualifying twice — every caller now builds
/// its own target the same way, rather than this function guessing which of
/// two arguments already carries the schema.
fn insert_row(
    conn: &Connection,
    target: &str,
    cols: &[String],
    vals: &[Bound],
) -> Result<(), StoreError> {
    let placeholders: Vec<String> = (1..=vals.len()).map(|i| format!(":{i}")).collect();
    let sql = format!(
        "INSERT INTO {target} ({}) VALUES ({})",
        cols.join(","),
        placeholders.join(",")
    );
    let refs: Vec<&dyn ToSql> = vals.iter().map(|b| b as &dyn ToSql).collect();
    conn.execute(&sql, &refs)
        .map_err(|e| StoreError::Other(format!("insert: {e}\n{sql}")))?;
    Ok(())
}

fn sqlval(v: &fhir_oracle_map::shred::SqlVal) -> Bound {
    match v {
        fhir_oracle_map::shred::SqlVal::Bool(b) => Bound::I64(Some(i64::from(*b))),
        fhir_oracle_map::shred::SqlVal::Int(i) => Bound::I64(Some(*i)),
        fhir_oracle_map::shred::SqlVal::Num(s) => Bound::Str(Some(s.clone())),
        fhir_oracle_map::shred::SqlVal::Text(s) => Bound::Str(Some(s.clone())),
        fhir_oracle_map::shred::SqlVal::Ts(s) => Bound::Timestamp(parse_ts(s)),
        fhir_oracle_map::shred::SqlVal::Date(s) => Bound::CalDate(parse_date(s)),
        fhir_oracle_map::shred::SqlVal::Jsonb(s) => Bound::Str(Some(s.clone())),
        fhir_oracle_map::shred::SqlVal::Bytes(b) => Bound::Bytes(Some(b.clone())),
    }
}

fn fmt_ords(ords: &[i16]) -> String {
    let mut s = String::from("{");
    for (i, o) in ords.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&o.to_string());
    }
    s.push('}');
    s
}

fn parse_ords(s: &str) -> Result<Vec<i16>, StoreError> {
    let inner = s.trim_start_matches('{').trim_end_matches('}');
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    inner
        .split(',')
        .map(|p| {
            p.parse::<i16>()
                .map_err(|e| StoreError::Other(format!("bad ords {s:?}: {e}")))
        })
        .collect()
}

fn leaf_from_cols(
    kind: &str,
    text: Option<String>,
    num: Option<String>,
    b: Option<bool>,
) -> fhir_oracle_map::value::LeafVal {
    use fhir_oracle_map::value::LeafVal;
    match kind {
        "s" => LeafVal::Str(text.unwrap_or_default()),
        "n" => LeafVal::Num(num.unwrap_or_default()),
        "b" => LeafVal::Bool(b.unwrap_or(false)),
        _ => LeafVal::Null,
    }
}

/// `ords` is `RAW(255)` holding the same ASCII text image every other port
/// stores (`M14.13`), not `VARCHAR2` — mirrors `fhir-mssql`'s `VARBINARY`
/// choice and its own `ords_bytes_to_str`.
fn ords_bytes_to_string(row: &Row, idx: usize) -> Result<String, StoreError> {
    let bytes: Vec<u8> = row
        .get::<usize, Option<Vec<u8>>>(idx)
        .unwrap_or(None)
        .unwrap_or_default();
    String::from_utf8(bytes).map_err(|e| StoreError::Other(format!("ords not utf8: {e}")))
}

/// Read one column as text, dispatching on the map's own `ColTy` rather than
/// on whatever the driver decides to hand back — unlike `fhir-mssql`'s
/// `cell_text`, which has to reverse-engineer the type from tiberius's raw
/// `ColumnData` because `Row::get::<T,_>` panics on a mismatch. This store
/// already knows `ty` for every column from the map that generated it, so it
/// asks for exactly that Rust type — **assuming** `Row::get` returns a typed
/// `Err` rather than panicking on its own mismatch, which is unverified.
fn cell_text(row: &Row, idx: usize, ty: ColTy) -> Result<Option<String>, StoreError> {
    match ty {
        ColTy::Bool => Ok(row
            .get::<usize, Option<i64>>(idx)
            .map_err(db_err)?
            .map(|v| (v != 0).to_string())),
        ColTy::Int | ColTy::BigInt => Ok(row
            .get::<usize, Option<i64>>(idx)
            .map_err(db_err)?
            .map(|v| v.to_string())),
        ColTy::Digest => Ok(row
            .get::<usize, Option<Vec<u8>>>(idx)
            .map_err(db_err)?
            .map(|b| b.iter().map(|x| format!("{x:02x}")).collect())),
        ColTy::Date => Ok(row
            .get::<usize, Option<chrono::NaiveDate>>(idx)
            .map_err(db_err)?
            .map(|d| d.format("%Y-%m-%d").to_string())),
        ColTy::Timestamptz => Ok(row
            .get::<usize, Option<chrono::NaiveDateTime>>(idx)
            .map_err(db_err)?
            .map(|d| d.format("%Y-%m-%dT%H:%M:%S%.6f").to_string())),
        ColTy::Numeric | ColTy::Text | ColTy::TextC | ColTy::TextIdx | ColTy::Jsonb => {
            row.get::<usize, Option<String>>(idx).map_err(db_err)
        }
    }
}

/// The surrogate primary key for an `Ext` or `Deep` row (`M14.9`), identical
/// byte-for-byte to every other port's — pure Rust, no engine involved.
pub fn surrogate_key(parts: &[&str]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    for p in parts {
        h.update((p.len() as u64).to_le_bytes());
        h.update(p.as_bytes());
    }
    h.finalize().into()
}

fn utc_micros(t: std::time::SystemTime) -> String {
    let dur = t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = dur.as_secs() as i64;
    let micros = dur.subsec_micros();
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    let hh = rem / 3600;
    let mm = (rem % 3600) / 60;
    let ss = rem % 60;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}.{micros:06}")
}

/// Howard Hinnant's `civil_from_days`, identical across every port.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn hist_entry(r: &Row) -> Result<crate::HistEntry, StoreError> {
    let version_id: i64 = r.get::<usize, Option<i64>>(0).unwrap_or(None).unwrap_or(0);
    let last_updated = r
        .get::<usize, Option<chrono::NaiveDateTime>>(1)
        .unwrap_or(None)
        .map(|d| d.format("%Y-%m-%dT%H:%M:%S%.6f").to_string())
        .unwrap_or_default();
    let op: String = r
        .get::<usize, Option<String>>(2)
        .unwrap_or(None)
        .unwrap_or_default();
    let raw: Option<String> = r.get(3).unwrap_or(None);
    let resource = match raw {
        Some(t) => Some(
            serde_json::from_str(&t)
                .map_err(|e| StoreError::Other(format!("history {version_id}: {e}")))?,
        ),
        None => None,
    };
    Ok(crate::HistEntry {
        version_id,
        last_updated,
        op: op.chars().next().unwrap_or('?'),
        resource,
    })
}

/// Diff the installed map against the current one, by name, across all
/// resources. A column whose *type* changed is neither additive nor
/// destructive: it is an error, because a type change means the shred writes
/// a different value shape and rewriting stored data is a migration somebody
/// must design (`L12`). `DROP TABLE … CASCADE CONSTRAINTS` makes drop order
/// irrelevant on this engine — the clause drops referencing constraints with
/// the table, unlike SQL Server's error 3726 (`M14.36` there).
/// `M14.38` (**F-47** step 5): bring every installed `"path"` column to
/// the map's recorded `path_bound` (`U12a`). Oracle cannot `ALTER … MODIFY`
/// a `CLOB` into a `VARCHAR2`, so the conversion is add-column, copy,
/// drop, rename — each statement implicitly committing (`M14.35`: no
/// transactional DDL). It is therefore a catalog-driven state machine per
/// table: whatever prefix of the sequence already ran, a rerun reads
/// `user_tab_columns` fresh and finishes the rest — the resumability story
/// the annex stated before this code existed.
///
/// The new column is **nullable** (`M14.39`): `''` is NULL on this engine
/// and the empty attach path is a root-level extension's, so the `NOT
/// NULL` the legacy column carried is exactly what made **F-85** fail.
/// The copy pre-checks the data and refuses, naming rows, if any stored
/// path exceeds the bound; widening a bounded column is additive;
/// narrowing one refuses (`U12a`: a recorded bound never shrinks in
/// place).
fn convert_path_columns(
    conn: &oracle::Connection,
    schema: &str,
    bound: u32,
) -> Result<usize, StoreError> {
    if bound == 0 {
        return Ok(0);
    }
    let s = quote_ident(schema);
    #[derive(Default)]
    struct St {
        /// (data_type, nullable, char_length) of `"path"`, if present.
        path: Option<(String, String, u32)>,
        path_new: bool,
    }
    let mut tables: std::collections::BTreeMap<String, St> = std::collections::BTreeMap::new();
    let rows = conn
        .query(
            "SELECT table_name, column_name, data_type, nullable, char_length \
             FROM user_tab_columns WHERE column_name IN ('path', 'path_new')",
            &[],
        )
        .map_err(db_err)?;
    for row in rows.flatten() {
        let t: String = row.get(0).map_err(db_err)?;
        let c: String = row.get(1).map_err(db_err)?;
        let dt: String = row.get(2).map_err(db_err)?;
        let nn: String = row.get(3).map_err(db_err)?;
        let ch: Option<u32> = row.get(4).map_err(db_err)?;
        let e = tables.entry(t).or_default();
        if c == "path" {
            e.path = Some((dt, nn, ch.unwrap_or(0)));
        } else {
            e.path_new = true;
        }
    }
    let mut converted = 0usize;
    for (t, st) in tables {
        let tq = format!("{s}.{}", quote_ident(&t));
        match st.path {
            Some((dt, _, _)) if dt == "CLOB" => {
                // Rows longer than the bound predate it (`U12a` shreds
                // refuse anything longer) and rewriting them is a migration
                // somebody must design — refused before any DDL touches
                // this table.
                let row = conn
                    .query_row(
                        &format!(
                            "SELECT COUNT(*), NVL(MAX(LENGTH(\"path\")), 0) \
                             FROM {tq} WHERE LENGTH(\"path\") > :1"
                        ),
                        &[&i64::from(bound)],
                    )
                    .map_err(db_err)?;
                let n: i64 = row.get(0).map_err(db_err)?;
                if n > 0 {
                    let longest: i64 = row.get(1).map_err(db_err)?;
                    return Err(StoreError::Other(format!(
                        "{tq} holds {n} row(s) whose \"path\" exceeds \
                         path_bound {bound} (longest: {longest} characters); \
                         they predate the bound and need a manual migration \
                         (U12a)"
                    )));
                }
                if !st.path_new {
                    conn.execute(
                        &format!("ALTER TABLE {tq} ADD (\"path_new\" VARCHAR2({bound} CHAR))"),
                        &[],
                    )
                    .map_err(db_err)?;
                }
                // Resumable: only rows an interrupted attempt did not
                // reach. An empty path stays NULL — its stored form here
                // (`M14.39`).
                conn.execute(
                    &format!(
                        "UPDATE {tq} SET \"path_new\" = \"path\" \
                         WHERE \"path_new\" IS NULL AND \"path\" IS NOT NULL"
                    ),
                    &[],
                )
                .map_err(db_err)?;
                conn.commit().map_err(db_err)?;
                conn.execute(&format!("ALTER TABLE {tq} DROP COLUMN \"path\""), &[])
                    .map_err(db_err)?;
                conn.execute(
                    &format!("ALTER TABLE {tq} RENAME COLUMN \"path_new\" TO \"path\""),
                    &[],
                )
                .map_err(db_err)?;
                converted += 1;
            }
            Some((_, nn, ch)) => {
                if ch > bound {
                    return Err(StoreError::Other(format!(
                        "narrowing {tq}.\"path\" from VARCHAR2({ch} CHAR) to \
                         VARCHAR2({bound} CHAR) is a manual migration (U12a): \
                         a recorded bound never shrinks in place"
                    )));
                }
                if ch < bound {
                    conn.execute(
                        &format!("ALTER TABLE {tq} MODIFY (\"path\" VARCHAR2({bound} CHAR))"),
                        &[],
                    )
                    .map_err(db_err)?;
                    converted += 1;
                }
                if nn == "N" {
                    // F-85's fix for a bounded-but-NOT-NULL column: the
                    // empty path is NULL here, and NOT NULL forbids it.
                    conn.execute(&format!("ALTER TABLE {tq} MODIFY (\"path\" NULL)"), &[])
                        .map_err(db_err)?;
                    converted += 1;
                }
            }
            None if st.path_new => {
                // An interrupted run that stopped between DROP and RENAME.
                conn.execute(
                    &format!("ALTER TABLE {tq} RENAME COLUMN \"path_new\" TO \"path\""),
                    &[],
                )
                .map_err(db_err)?;
                converted += 1;
            }
            None => {}
        }
    }
    Ok(converted)
}

fn diff_maps(map: &RelMap, old_map: &RelMap) -> Result<(Vec<String>, Vec<String>), StoreError> {
    use std::collections::{HashMap, HashSet};
    let s = &map.schema;
    let esc = quote_ident(s);
    let (mut adds, mut destructive) = (Vec::new(), Vec::new());
    let mut old_tables: HashMap<&str, &fhir_oracle_map::model::Table> = HashMap::new();
    for rm in old_map.resources.values() {
        for t in &rm.tables {
            old_tables.insert(t.name.as_str(), t);
        }
    }
    let mut new_names: HashSet<&str> = HashSet::new();
    for rm in map.resources.values() {
        for t in &rm.tables {
            new_names.insert(t.name.as_str());
            let Some(old_t) = old_tables.get(t.name.as_str()) else {
                adds.push(fhir_oracle_map::ddl::create_table(s, rm, t));
                continue;
            };
            let old_cols: HashMap<&str, ColTy> =
                old_t.cols.iter().map(|c| (c.name.as_str(), c.ty)).collect();
            let new_cols: HashSet<&str> = t.cols.iter().map(|c| c.name.as_str()).collect();
            for c in &t.cols {
                match old_cols.get(c.name.as_str()) {
                    None => adds.push(format!(
                        "ALTER TABLE {esc}.{} ADD ({} {})",
                        quote_ident(&t.name),
                        quote_ident(&c.name),
                        fhir_oracle_map::ddl::col_sql(c.ty)
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
                if !new_cols.contains(name) {
                    destructive.push(format!(
                        "ALTER TABLE {esc}.{} DROP COLUMN {}",
                        quote_ident(&t.name),
                        quote_ident(name)
                    ));
                }
            }
        }
    }
    for name in old_tables.keys() {
        if !new_names.contains(name) {
            destructive.push(format!(
                "DROP TABLE {esc}.{} CASCADE CONSTRAINTS",
                quote_ident(name)
            ));
        }
    }
    Ok((adds, destructive))
}

/// Reconstruct one resource under an **explicitly supplied** map.
///
/// `get` resolves the map from this store's artifact, which is what every read
/// wants. The `O10.4c` re-shred cannot: it has to read rows laid out by the map
/// that is still installed (`G2.5`'s stored asset) while the store already
/// carries the new one. Factored out for that caller so the migration reads
/// through the same path the conformance suite exercises on every other read.
fn recon_with_map(
    conn: &Connection,
    s: &str,
    rm: &fhir_oracle_map::model::ResourceMap,
    id: &str,
) -> Result<Option<serde_json::Value>, StoreError> {
    let base = quote_ident(&rm.base_table().name);
    let present = conn
        .query_row(
            &format!("SELECT 1 FROM {s}.{base} WHERE \"id\" = :1"),
            &[&id],
        )
        .is_ok();
    if !present {
        let _ = conn.rollback();
        return Ok(None);
    }

    use fhir_oracle_map::model::TableKind;
    use fhir_oracle_map::reconstruct::{InRow, ReconIn};

    let mut input = ReconIn {
        tables: vec![Vec::new(); rm.tables.len()],
        ..Default::default()
    };

    for (ti, t) in rm.tables.iter().enumerate() {
        let table = quote_ident(&t.name);
        match t.kind {
            TableKind::Base | TableKind::Elem => {
                let names: Vec<(String, ColTy)> =
                    t.cols.iter().map(|c| (c.name.clone(), c.ty)).collect();
                let key = if t.kind == TableKind::Base {
                    "id"
                } else {
                    "rid"
                };
                let mut sel: Vec<String> = Vec::new();
                if t.kind == TableKind::Elem {
                    sel.push("\"ords\"".to_string());
                }
                sel.extend(names.iter().map(|(n, _)| quote_ident(n)));
                if sel.is_empty() {
                    sel.push("NULL".to_string());
                }
                let rows = conn
                    .query(
                        &format!(
                            "SELECT {} FROM {s}.{table} WHERE \"{key}\" = :1",
                            sel.join(", ")
                        ),
                        &[&id],
                    )
                    .map_err(db_err)?;
                for row in rows {
                    let row = row.map_err(db_err)?;
                    let mut ords = Vec::new();
                    let mut off = 0usize;
                    if t.kind == TableKind::Elem {
                        ords = parse_ords(&ords_bytes_to_string(&row, 0)?)?;
                        off = 1;
                    }
                    let mut cols = std::collections::HashMap::new();
                    for (i, (n, ty)) in names.iter().enumerate() {
                        if let Some(v) = cell_text(&row, i + off, *ty)? {
                            cols.insert(n.clone(), v);
                        }
                    }
                    input.tables[ti].push(InRow { ords, cols });
                }
            }
            TableKind::Ext => {
                let rows = conn
                    .query(
                        &format!(
                            "SELECT \"path\",\"ords\",\"modifier\",\"ext_ord\",\"url\",\"leaf\",\
                                    \"v_kind\",\"v_text\",\"v_num\",\"v_bool\" FROM {s}.{table} WHERE \"rid\" = :1"
                        ),
                        &[&id],
                    )
                    .map_err(db_err)?;
                for row in rows {
                    let row = row.map_err(db_err)?;
                    let path: String = row
                        .get::<usize, Option<String>>(0)
                        .unwrap_or(None)
                        .unwrap_or_default();
                    let ords = parse_ords(&ords_bytes_to_string(&row, 1)?)?;
                    let modifier = row
                        .get::<usize, Option<i64>>(2)
                        .unwrap_or(None)
                        .unwrap_or(0)
                        != 0;
                    let ext_ord = row
                        .get::<usize, Option<i64>>(3)
                        .unwrap_or(None)
                        .unwrap_or(0) as i16;
                    let url: Option<String> = row.get(4).unwrap_or(None);
                    let leaf: String = row
                        .get::<usize, Option<String>>(5)
                        .unwrap_or(None)
                        .unwrap_or_default();
                    let kind: String = row
                        .get::<usize, Option<String>>(6)
                        .unwrap_or(None)
                        .unwrap_or_default();
                    let text: Option<String> = row.get(7).unwrap_or(None);
                    let num: Option<String> = row.get(8).unwrap_or(None);
                    let b: Option<i64> = row.get(9).unwrap_or(None);
                    let val = leaf_from_cols(&kind, text, num, b.map(|v| v != 0));
                    input.ext.push(fhir_oracle_map::shred::ExtRow {
                        path,
                        ords,
                        modifier,
                        ext_ord,
                        url,
                        leaf,
                        val,
                    });
                }
            }
            TableKind::Deep => {
                let rows = conn
                    .query(
                        &format!(
                            "SELECT \"path\",\"ords\",\"leaf\",\"v_kind\",\"v_text\",\"v_num\",\"v_bool\" \
                             FROM {s}.{table} WHERE \"rid\" = :1"
                        ),
                        &[&id],
                    )
                    .map_err(db_err)?;
                for row in rows {
                    let row = row.map_err(db_err)?;
                    let path: String = row
                        .get::<usize, Option<String>>(0)
                        .unwrap_or(None)
                        .unwrap_or_default();
                    let ords = parse_ords(&ords_bytes_to_string(&row, 1)?)?;
                    let leaf: String = row
                        .get::<usize, Option<String>>(2)
                        .unwrap_or(None)
                        .unwrap_or_default();
                    let kind: String = row
                        .get::<usize, Option<String>>(3)
                        .unwrap_or(None)
                        .unwrap_or_default();
                    let text: Option<String> = row.get(4).unwrap_or(None);
                    let num: Option<String> = row.get(5).unwrap_or(None);
                    let b: Option<i64> = row.get(6).unwrap_or(None);
                    let val = leaf_from_cols(&kind, text, num, b.map(|v| v != 0));
                    input.deep.push(fhir_oracle_map::shred::DeepRow {
                        path,
                        ords,
                        leaf,
                        val,
                    });
                }
            }
            TableKind::Contained => {
                let rows = conn
                    .query(
                        &format!("SELECT \"ord\",\"resource\" FROM {s}.{table} WHERE \"rid\" = :1"),
                        &[&id],
                    )
                    .map_err(db_err)?;
                for row in rows {
                    let row = row.map_err(db_err)?;
                    let ord: i16 = row
                        .get::<usize, Option<i64>>(0)
                        .unwrap_or(None)
                        .unwrap_or(0) as i16;
                    let raw: String = row
                        .get::<usize, Option<String>>(1)
                        .unwrap_or(None)
                        .unwrap_or_default();
                    let v: serde_json::Value = serde_json::from_str(&raw)
                        .map_err(|e| StoreError::Other(format!("contained: {e}")))?;
                    input.contained.push((ord, v));
                }
            }
            TableKind::History => {}
        }
    }

    let _ = conn.rollback();
    let v = fhir_oracle_map::reconstruct::reconstruct(rm, &input, Some(id))?;
    Ok(Some(v))
}

/// Write one shredded resource's rows: the base row, the element tables, and
/// the ext / deep / contained spill.
///
/// Factored out of `put_in_tx` for the `O10.4c` re-shred, which must write
/// through a map that is **not** this store's — the resource is reconstructed
/// under the installed old map and written under the new one. Sharing the code
/// means the migration writes rows the same way every ordinary write does.
#[allow(clippy::too_many_arguments)]
fn write_shredded(
    conn: &Connection,
    s: &str,
    base: &str,
    rm: &fhir_oracle_map::model::ResourceMap,
    id: &str,
    version_id: i64,
    ts: &str,
    out: &fhir_oracle_map::shred::ShredOut,
) -> Result<(), StoreError> {
    // Base row first: every child has a foreign key to it.
    let mut cols = vec![
        "\"id\"".to_string(),
        "\"version_id\"".to_string(),
        "\"last_updated\"".to_string(),
    ];
    let mut vals: Vec<Bound> = vec![
        Bound::Str(Some(id.to_string())),
        Bound::I64(Some(version_id)),
        Bound::Timestamp(parse_ts(ts)),
    ];
    for r in out.rows.iter().filter(|r| r.table == 0) {
        for (name, v) in &r.cols {
            cols.push(quote_ident(name));
            vals.push(sqlval(v));
        }
    }
    insert_row(conn, &format!("{s}.{base}"), &cols, &vals)?;

    // Element tables, one insert per row — see `insert_row`'s note.
    let mut by_table: std::collections::BTreeMap<u32, Vec<&fhir_oracle_map::shred::Row>> =
        std::collections::BTreeMap::new();
    for r in out.rows.iter().filter(|r| r.table != 0) {
        by_table.entry(r.table).or_default().push(r);
    }
    for (ti, rows) in by_table {
        let t = &rm.tables[ti as usize];
        let mut names: Vec<String> = Vec::new();
        for r in &rows {
            for (n, _) in &r.cols {
                if !names.contains(n) {
                    names.push(n.clone());
                }
            }
        }
        let types: Vec<ColTy> = names
            .iter()
            .map(|n| {
                t.cols
                    .iter()
                    .find(|c| &c.name == n)
                    .map_or(ColTy::Text, |c| c.ty)
            })
            .collect();
        let mut cols = vec!["\"rid\"".to_string(), "\"ords\"".to_string()];
        cols.extend(names.iter().map(|n| quote_ident(n)));
        for r in &rows {
            let mut vals: Vec<Bound> = vec![
                Bound::Str(Some(id.to_string())),
                Bound::Bytes(Some(fmt_ords(&r.ords).into_bytes())),
            ];
            for (n, ty) in names.iter().zip(&types) {
                vals.push(
                    r.cols
                        .iter()
                        .find(|(c, _)| c == n)
                        .map_or_else(|| null_for_ty(*ty), |(_, v)| sqlval(v)),
                );
            }
            insert_row(conn, &format!("{s}.{}", quote_ident(&t.name)), &cols, &vals)?;
        }
    }

    // Extensions and spill carry the surrogate primary key (M14.9): their
    // natural keys hold a CLOB, which this engine can neither index nor
    // `=`-compare at all.
    if let Some((_, t)) = rm.find_table(fhir_oracle_map::model::TableKind::Ext) {
        for e in &out.ext {
            let (kind, text, num, b) = e.val.cols();
            let ords = fmt_ords(&e.ords);
            let ext_ord = e.ext_ord.to_string();
            let modifier = u8::from(e.modifier).to_string();
            let key = surrogate_key(&[id, &e.path, &ords, &modifier, &ext_ord, &e.leaf]);
            let cols = [
                "\"key_hash\"",
                "\"rid\"",
                "\"path\"",
                "\"ords\"",
                "\"modifier\"",
                "\"ext_ord\"",
                "\"url\"",
                "\"leaf\"",
                "\"v_kind\"",
                "\"v_text\"",
                "\"v_num\"",
                "\"v_bool\"",
            ]
            .map(String::from)
            .to_vec();
            let vals = vec![
                Bound::Bytes(Some(key.to_vec())),
                Bound::Str(Some(id.to_string())),
                Bound::Str(Some(e.path.clone())),
                Bound::Bytes(Some(ords.into_bytes())),
                Bound::I64(Some(i64::from(e.modifier))),
                Bound::I64(Some(i64::from(e.ext_ord))),
                Bound::Str(e.url.clone()),
                Bound::Str(Some(e.leaf.clone())),
                Bound::Str(Some(kind.to_string())),
                Bound::Str(text.map(str::to_string)),
                Bound::Str(num.map(str::to_string)),
                Bound::I64(b.map(i64::from)),
            ];
            insert_row(conn, &format!("{s}.{}", quote_ident(&t.name)), &cols, &vals)?;
        }
    }

    if let Some((_, t)) = rm.find_table(fhir_oracle_map::model::TableKind::Deep) {
        for d in &out.deep {
            let (kind, text, num, b) = d.val.cols();
            let ords = fmt_ords(&d.ords);
            let key = surrogate_key(&[id, &d.path, &ords, &d.leaf]);
            let cols = [
                "\"key_hash\"",
                "\"rid\"",
                "\"path\"",
                "\"ords\"",
                "\"leaf\"",
                "\"v_kind\"",
                "\"v_text\"",
                "\"v_num\"",
                "\"v_bool\"",
            ]
            .map(String::from)
            .to_vec();
            let vals = vec![
                Bound::Bytes(Some(key.to_vec())),
                Bound::Str(Some(id.to_string())),
                Bound::Str(Some(d.path.clone())),
                Bound::Bytes(Some(ords.into_bytes())),
                Bound::Str(Some(d.leaf.clone())),
                Bound::Str(Some(kind.to_string())),
                Bound::Str(text.map(str::to_string)),
                Bound::Str(num.map(str::to_string)),
                Bound::I64(b.map(i64::from)),
            ];
            insert_row(conn, &format!("{s}.{}", quote_ident(&t.name)), &cols, &vals)?;
        }
    }

    if let Some((_, t)) = rm.find_table(fhir_oracle_map::model::TableKind::Contained) {
        for (ord, v) in &out.contained {
            let cols = ["\"rid\"", "\"ord\"", "\"resource\""]
                .map(String::from)
                .to_vec();
            let vals = vec![
                Bound::Str(Some(id.to_string())),
                Bound::I64(Some(i64::from(*ord))),
                Bound::Str(Some(v.to_string())),
            ];
            insert_row(conn, &format!("{s}.{}", quote_ident(&t.name)), &cols, &vals)?;
        }
    }

    Ok(())
}
