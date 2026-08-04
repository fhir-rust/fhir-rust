//! The SQL Server store.
//!
//! # What is implemented, and what is not
//!
//! Implemented and verified against a live `azure-sql-edge` container:
//! `connect`, `init`, `put`, `get`, `delete`, `history`, `vread`,
//! `verify_audit`, `purge`, `log_access`, `search`/`search_full`/`search_page`
//! (query construction lives in [`crate::mssql_search`]), plus the test/dev
//! utilities `with_chain_keys`, `log_access_batch`, `table_count`,
//! `trigger_count`, `exec_raw`.
//!
//! **Not implemented:** `conditional_create_audited`, `put_audited`,
//! `upgrade` (schema migration from a pre-adjunct install), `backfill_norm`.
//! A method absent here is a method this store does not have — there is no
//! stub that panics or returns `Unsupported` standing in for it, because a
//! stub is exactly the kind of thing that gets mistaken for coverage
//! (`C0.9`, and see `tasks.md` **F-27**).
//!
//! # Differences from the MySQL/MariaDB stores this was built alongside
//!
//! - **No connection pool or typed transaction API in the driver.** `tiberius`
//!   has neither; [`crate::pool`] supplies a pool, and transactions here are
//!   plain `BEGIN TRANSACTION` / `COMMIT TRANSACTION` / `ROLLBACK TRANSACTION`
//!   statements rather than a typed `Transaction` handle.
//! - **Row locking is `WITH (UPDLOCK, ROWLOCK)`**, not `FOR UPDATE`. Same
//!   purpose as the MySQL store's lock (`H5.4`): serialise writers on one
//!   resource id so a chain tip read and the row it is based on cannot
//!   interleave with another writer's commit.
//! - **`Ext`/`Deep` carry a hash surrogate key** (`M14.12`), for the same
//!   reason as MySQL: their natural key includes `NVARCHAR(MAX)`, which
//!   cannot be part of an index key on this engine.
//! - **Adjunct columns** (`U1`–`U10`) exist in the schema (`M3.6a`/`M3.6b`) but
//!   [`crate::mssql_search`] does not query them — they are not wired into
//!   `TargetKind` at all, on any port, so a `Token` comparison against a
//!   `Text` column is correct and unindexed rather than broken (`M14.16`).

use std::sync::Arc;

use fhir_mssql_map::model::{ColTy, RelMap};
use tiberius::{ColumnData, Row};

use crate::StoreError;
use crate::pool::{self, Pool};

/// A history row's chain tip: `(version_id, sha256 link, sha3 link)`.
type ChainTip = (i64, Option<Vec<u8>>, Option<Vec<u8>>);

fn db_err(e: tiberius::error::Error) -> StoreError {
    StoreError::Db(e.to_string())
}

fn pool_err(e: bb8::RunError<tiberius::error::Error>) -> StoreError {
    StoreError::Pool(e.to_string())
}

/// `[schema].[table]`, both bracket-quoted.
///
/// `[` and `]` rather than `"`, because unlike PostgreSQL/MySQL-ANSI-mode a
/// double quote is a string delimiter here unless the session sets
/// `QUOTED_IDENTIFIER ON` — which this driver does not promise, so brackets
/// are the identifier quote that needs no session state to be correct.
fn quote_ident(s: &str) -> String {
    format!("[{}]", s.replace(']', "]]"))
}

fn qualified(schema: &str, name: &str) -> String {
    format!("{}.{}", quote_ident(schema), quote_ident(name))
}

/// The stored map asset is gzip and the meta table's `[value]` is text, so it
/// travels as hex — the same encoding every other port uses, so an asset can
/// be lifted from one port's meta table into another's.
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

/// The table name in a `CREATE TABLE [schema].[name] (…` statement, so
/// [`MsSqlStore::upgrade_in_tx`] can skip a schema-wide table that already
/// exists rather than fail (T-SQL's `CREATE TABLE` has no `IF NOT EXISTS`,
/// `M14.17`).
///
/// `None` means the statement was not that shape, and the caller then applies
/// it rather than skipping — a redundant statement fails loudly, a wrongly
/// skipped one loses a table silently.
fn table_name_of(stmt: &str) -> Option<String> {
    let rest = stmt.strip_prefix("CREATE TABLE [")?;
    let (_, rest) = rest.split_once("].[")?;
    rest.split_once(']').map(|(name, _)| name.to_string())
}

/// The index name in a `CREATE INDEX [name] ON …` statement, for the same
/// reason as [`table_name_of`] — T-SQL has no `IF NOT EXISTS` for `CREATE
/// INDEX` either.
fn index_name_of(stmt: &str) -> Option<String> {
    let rest = stmt.strip_prefix("CREATE INDEX [")?;
    rest.split_once(']').map(|(name, _)| name.to_string())
}

/// The column name in an `ALTER TABLE … ADD [name] …` statement (no `COLUMN`
/// keyword — `M14.32`).
fn added_column_name(stmt: &str) -> Option<String> {
    let rest = stmt.split_once(" ADD [")?.1;
    rest.split_once(']').map(|(name, _)| name.to_string())
}

/// Apply a batch of DDL statements with `simple_query`, naming which one
/// failed. Used only inside [`MsSqlStore::upgrade_in_tx`], where the whole
/// batch runs in one transaction the caller rolls back on `Err` — unlike
/// MySQL, a failure here does not leave a partially-applied schema behind.
async fn apply_stmts(conn: &mut pool::Connection, stmts: &[String]) -> Result<(), StoreError> {
    for stmt in stmts {
        conn.simple_query(stmt.as_str()).await.map_err(|e| {
            StoreError::Other(format!(
                "upgrade: {e} (transaction rolled back; schema unchanged)\nstatement was:\n{stmt}"
            ))
        })?;
    }
    Ok(())
}

/// A pool bound to one relational map.
#[derive(Debug)]
pub struct MsSqlStore {
    pool: Pool,
    map: Arc<RelMap>,
    keys: crate::chain::KeyRing,
}

impl MsSqlStore {
    /// Connect using an ADO connection string, e.g.
    /// `server=tcp:127.0.0.1,1433;user=sa;password=…;TrustServerCertificate=true`.
    ///
    /// The schema (FHIR version) named by the map is *not* selected as the
    /// default database context: every statement qualifies its tables, so a
    /// mistyped qualification fails loudly instead of silently hitting
    /// whatever schema happened to be current.
    ///
    /// # Errors
    /// If the DSN is malformed or the server cannot be reached.
    pub async fn connect(dsn: &str, map: Arc<RelMap>) -> Result<Self, StoreError> {
        let pool = pool::connect_pool(dsn).await?;
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
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        conn.simple_query("SELECT 1")
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;
        Ok(())
    }

    /// Apply the generated DDL for this store's map, idempotently.
    ///
    /// T-SQL has no transactional DDL: a `CREATE TABLE` commits immediately,
    /// so a failed install can leave a partial schema behind (`M14.x`, same
    /// regression as the MySQL/MariaDB ports' non-atomic install). This
    /// reports how far it got rather than pretending atomicity it does not
    /// have.
    ///
    /// `checksum` is recorded so a later run can tell "already installed this
    /// exact map" from "installed a different one".
    ///
    /// # Errors
    /// On the first statement that fails to apply.
    pub async fn init(&self, checksum: &str) -> Result<usize, StoreError> {
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let meta = qualified(&self.map.schema, "fhir_mssql_meta");

        // `ddl::ddl` already opens with `IF SCHEMA_ID(..) IS NULL CREATE
        // SCHEMA` and an unconditional `CREATE TABLE fhir_mssql_meta` — this
        // function's first version pre-created both by hand and collided with
        // the DDL's own creation on statement 2. `init` is documented (and
        // used, by every other port) as a fresh install; a second call is
        // `init --upgrade`'s job, not this one's, so there is nothing to
        // guard here beyond what the generated statements already do.
        let statements = fhir_mssql_map::ddl::ddl(&self.map);
        let mut applied = 0usize;
        for stmt in &statements {
            conn.simple_query(stmt.as_str()).await.map_err(|e| {
                StoreError::Other(format!(
                    "init: statement {} of {} failed: {e}\n{stmt}",
                    applied + 1,
                    statements.len()
                ))
            })?;
            applied += 1;
        }

        // The map asset itself, gzipped and hex-coded, is what makes `upgrade`
        // possible: an upgrade diffs the installed map against the current
        // one, and a checksum alone says only *that* something changed, never
        // *what* — mirrors `fhir-mysql`/`fhir-sqlite` exactly, except the
        // checksum's own key stays `checksum` (this port's established name,
        // predating this change) rather than their `map_checksum`.
        let asset_hex = hex_encode(
            &self
                .map
                .to_gz_bytes()
                .map_err(|e| StoreError::Other(e.to_string()))?,
        );
        for (k, v) in [
            ("checksum", checksum),
            ("fhir_version", self.map.fhir_version.as_str()),
            ("map_asset", asset_hex.as_str()),
        ] {
            conn.execute(
                format!(
                    "MERGE {meta} AS tgt USING (SELECT @P1 AS k, @P2 AS v) AS src ON tgt.[key] = src.k \
                     WHEN MATCHED THEN UPDATE SET [value] = src.v \
                     WHEN NOT MATCHED THEN INSERT ([key], [value]) VALUES (src.k, src.v);"
                ),
                &[&k, &v],
            )
            .await
            .map_err(db_err)?;
        }

        Ok(applied)
    }

    /// The checksum recorded by the last successful `init`, if any.
    ///
    /// # Errors
    /// On a connection or query failure. A missing meta table is **not** an
    /// error — it means this schema has never been installed — and is folded
    /// into `Ok(None)`.
    pub async fn installed_checksum(&self) -> Result<Option<String>, StoreError> {
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let meta = qualified(&self.map.schema, "fhir_mssql_meta");
        let row = conn
            .query(
                format!("SELECT [value] FROM {meta} WHERE [key] = @P1"),
                &[&"checksum"],
            )
            .await;
        let row = match row {
            Ok(stream) => stream.into_row().await,
            Err(_) => return Ok(None),
        };
        match row {
            Ok(Some(r)) => Ok(r.get::<&str, _>(0).map(str::to_string)),
            Ok(None) | Err(_) => Ok(None),
        }
    }

    /// Drop every object in this store's schema. Test/dev convenience, not
    /// part of any FHIR operation.
    ///
    /// # Errors
    /// On a connection failure.
    pub async fn drop_schema(&self) -> Result<(), StoreError> {
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        conn.simple_query(format!(
            "DECLARE @sql NVARCHAR(MAX) = N'';
             SELECT @sql = @sql + N'ALTER TABLE {s}.[' + OBJECT_NAME(parent_object_id) \
                 + N'] DROP CONSTRAINT [' + name + N'];'
               FROM sys.foreign_keys WHERE schema_id = SCHEMA_ID('{schema}');
             EXEC sp_executesql @sql;
             SET @sql = N'';
             SELECT @sql = @sql + N'DROP TABLE {s}.[' + name + N'];'
               FROM sys.tables WHERE schema_id = SCHEMA_ID('{schema}');
             EXEC sp_executesql @sql;
             IF SCHEMA_ID('{schema}') IS NOT NULL EXEC('DROP SCHEMA {s}');",
            s = quote_ident(&self.map.schema),
            schema = self.map.schema.replace('\'', "''"),
        ))
        .await
        .map_err(db_err)?;
        Ok(())
    }

    /// Upgrade an installed schema to this store's map: additive changes (new
    /// tables, columns, indexes) apply automatically; destructive ones
    /// require `allow_destructive`. Column type changes always refuse
    /// (`O10.4a`, `L12`).
    ///
    /// Closes this port's share of audit **F-15**.
    ///
    /// **Unlike `fhir-mysql`/`fhir-mariadb`, this is genuinely atomic.**
    /// T-SQL DDL participates in a transaction like any other statement —
    /// unlike MySQL, which commits it implicitly — so the additive and
    /// destructive changes both run inside one `BEGIN TRANSACTION` /
    /// `COMMIT TRANSACTION`, with `ROLLBACK TRANSACTION` on the first
    /// failure. A failed upgrade here cannot leave a schema that is neither
    /// the old one nor the new one — `mysql.rs`'s doc comment describes that
    /// as unpreventable on its engine; on this one it is prevented, not
    /// merely reported. [`Self::backfill_norm`] runs afterward, **outside**
    /// this transaction and in its own bounded batches: it is a bulk write
    /// over data, not schema DDL, and there is no reason to hold schema locks
    /// for however long it takes.
    ///
    /// # Errors
    /// If the schema is not installed; if it predates `map_asset` being
    /// recorded (installed before this method existed — reinstall with
    /// [`Self::init`] to make later upgrades possible); if a column's type
    /// changed (always refused, no flag overrides it); if destructive changes
    /// are needed and `allow_destructive` is `false`; or on a database
    /// failure, in which case the transaction was rolled back and the schema
    /// is unchanged.
    pub async fn upgrade(
        &self,
        checksum: &str,
        allow_destructive: bool,
    ) -> Result<crate::UpgradeReport, StoreError> {
        let schema_name = self.map.schema.clone();
        let esc = quote_ident(&schema_name);
        let meta = qualified(&schema_name, "fhir_mssql_meta");
        let mut conn = self.pool.get().await.map_err(pool_err)?;

        // Distinguish "never installed" from "installed before the asset was
        // recorded": the remedies differ, `init` versus a reload.
        let meta_exists = conn
            .query(
                "SELECT COUNT(*) FROM sys.tables t JOIN sys.schemas s \
                 ON s.schema_id = t.schema_id WHERE s.name = @P1 AND t.name = @P2",
                &[&schema_name.as_str(), &"fhir_mssql_meta"],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?
            .and_then(|r| r.get::<i32, _>(0))
            .unwrap_or(0);
        if meta_exists == 0 {
            return Err(StoreError::Other(format!(
                "schema {schema_name} is not installed"
            )));
        }

        let old_hex: Option<String> = conn
            .query(
                format!("SELECT [value] FROM {meta} WHERE [key] = @P1"),
                &[&"map_asset"],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?
            .and_then(|r| r.get::<&str, _>(0).map(str::to_string));
        let old_hex = old_hex.ok_or_else(|| {
            StoreError::Other(
                "installed schema predates upgrade support (no stored map asset); \
                 reinstall with `init` to make later upgrades possible"
                    .into(),
            )
        })?;
        let old_map = RelMap::from_gz_bytes(&hex_decode(&old_hex)?)
            .map_err(|e| StoreError::Other(format!("stored map asset unreadable: {e}")))?;

        let (adds, destructive) = self.diff_maps(&old_map, &schema_name, &esc)?;
        if !destructive.is_empty() && !allow_destructive {
            return Err(StoreError::Other(format!(
                "upgrade requires {} destructive change(s); rerun with allow_destructive \
                 (first: {})",
                destructive.len(),
                destructive.first().expect("non-empty")
            )));
        }
        let (n_add, n_drop) = (adds.len(), destructive.len());

        conn.simple_query("BEGIN TRANSACTION")
            .await
            .map_err(db_err)?;
        let result = self
            .upgrade_in_tx(&mut conn, &schema_name, &meta, checksum, &adds, &destructive)
            .await;
        match result {
            Ok(()) => {
                conn.simple_query("COMMIT TRANSACTION")
                    .await
                    .map_err(db_err)?;
            }
            Err(e) => {
                // Transactional DDL: a rollback here genuinely undoes
                // everything this call applied, not merely a best effort.
                let _ = conn.simple_query("ROLLBACK TRANSACTION").await;
                return Err(e);
            }
        }
        drop(conn);

        let folded = self.backfill_norm().await?;
        Ok(crate::UpgradeReport {
            additive: n_add,
            destructive: n_drop,
            folded,
        })
    }

    /// The body of [`Self::upgrade`] that runs inside its transaction:
    /// applies the additive diff, reconciles the schema-wide objects and
    /// audit-envelope columns the per-resource diff cannot see, applies the
    /// destructive diff, and records the new checksum/version/asset.
    #[allow(clippy::too_many_arguments)]
    async fn upgrade_in_tx(
        &self,
        conn: &mut pool::Connection,
        schema_name: &str,
        meta: &str,
        checksum: &str,
        adds: &[String],
        destructive: &[String],
    ) -> Result<(), StoreError> {
        apply_stmts(conn, adds).await?;

        // Reconciliation: objects the per-resource diff cannot see. Computed
        // against the schema *as the adds left it*, not as it was before: a
        // history table `create_table` just created a moment ago already
        // carries the audit envelope, and building this filter first would
        // emit an `ADD` for every envelope column it was about to gain and
        // fail with "Column names in each table must be unique" — the same
        // ordering mistake `fhir-mysql` made first. SQL Server's catalog
        // views reflect uncommitted DDL from earlier in the same transaction,
        // so querying them now sees what `apply_stmts` just built.
        let have_tables = self.installed_tables(conn, schema_name).await?;
        let have_ix = self.installed_indexes(conn, schema_name).await?;
        // The two schema-wide tables carry no `IF NOT EXISTS` — T-SQL's
        // `CREATE TABLE` has none (`M14.17`) — and the indexes have no
        // `IF NOT EXISTS` either, so both are filtered against what already
        // exists rather than applied wholesale.
        let mut reconcile: Vec<String> = fhir_mssql_map::ddl::schema_wide_objects(schema_name)
            .into_iter()
            .filter(|stmt| match table_name_of(stmt) {
                Some(name) => !have_tables.contains(&name),
                None => match index_name_of(stmt) {
                    Some(name) => !have_ix.contains(&name),
                    None => true,
                },
            })
            .collect();
        for rm in self.map.resources.values() {
            if let Some((_, hist)) = rm.find_table(fhir_mssql_map::model::TableKind::History) {
                let have_cols = self.installed_columns(conn, schema_name, &hist.name).await?;
                reconcile.extend(
                    fhir_mssql_map::ddl::history_audit_columns(schema_name, &hist.name)
                        .into_iter()
                        .filter(|stmt| match added_column_name(stmt) {
                            Some(c) => !have_cols.contains(&c),
                            None => true,
                        }),
                );
                // `CREATE OR ALTER`, so genuinely idempotent (`M14.19`) — no
                // filter needed, unlike the two above.
                reconcile.extend(fhir_mssql_map::ddl::append_only_triggers(
                    schema_name,
                    &hist.name,
                ));
            }
        }
        apply_stmts(conn, &reconcile).await?;
        apply_stmts(conn, destructive).await?;

        let new_hex = hex_encode(
            &self
                .map
                .to_gz_bytes()
                .map_err(|e| StoreError::Other(e.to_string()))?,
        );
        for (k, v) in [
            ("checksum", checksum),
            ("fhir_version", self.map.fhir_version.as_str()),
            ("map_asset", new_hex.as_str()),
        ] {
            conn.execute(
                format!("UPDATE {meta} SET [value] = @P1 WHERE [key] = @P2"),
                &[&v, &k],
            )
            .await
            .map_err(db_err)?;
        }
        Ok(())
    }

    /// Table names already present in this schema, so the schema-wide
    /// `CREATE TABLE` statements (`fhir_mssql_access_log`,
    /// `fhir_mssql_countersign`) — which carry no `IF NOT EXISTS`, because
    /// T-SQL's `CREATE TABLE` has none — can be skipped rather than fail with
    /// "There is already an object named …".
    async fn installed_tables(
        &self,
        conn: &mut pool::Connection,
        schema: &str,
    ) -> Result<std::collections::HashSet<String>, StoreError> {
        let rows = conn
            .query(
                "SELECT t.name FROM sys.tables t JOIN sys.schemas s \
                 ON s.schema_id = t.schema_id WHERE s.name = @P1",
                &[&schema],
            )
            .await
            .map_err(db_err)?
            .into_first_result()
            .await
            .map_err(db_err)?;
        Ok(rows
            .iter()
            .filter_map(|r| r.get::<&str, _>(0).map(str::to_string))
            .collect())
    }

    /// Index names already present in this schema, so the non-idempotent
    /// `CREATE INDEX` statements (T-SQL has no `IF NOT EXISTS` for it either)
    /// can be skipped rather than fail.
    async fn installed_indexes(
        &self,
        conn: &mut pool::Connection,
        schema: &str,
    ) -> Result<std::collections::HashSet<String>, StoreError> {
        let rows = conn
            .query(
                "SELECT i.name FROM sys.indexes i \
                 JOIN sys.tables t ON t.object_id = i.object_id \
                 JOIN sys.schemas s ON s.schema_id = t.schema_id \
                 WHERE s.name = @P1 AND i.name IS NOT NULL",
                &[&schema],
            )
            .await
            .map_err(db_err)?
            .into_first_result()
            .await
            .map_err(db_err)?;
        Ok(rows
            .iter()
            .filter_map(|r| r.get::<&str, _>(0).map(str::to_string))
            .collect())
    }

    /// The columns a table actually has, for the audit-envelope diff.
    async fn installed_columns(
        &self,
        conn: &mut pool::Connection,
        schema: &str,
        table: &str,
    ) -> Result<std::collections::HashSet<String>, StoreError> {
        let rows = conn
            .query(
                "SELECT c.name FROM sys.columns c \
                 JOIN sys.tables t ON t.object_id = c.object_id \
                 JOIN sys.schemas s ON s.schema_id = t.schema_id \
                 WHERE s.name = @P1 AND t.name = @P2",
                &[&schema, &table],
            )
            .await
            .map_err(db_err)?
            .into_first_result()
            .await
            .map_err(db_err)?;
        Ok(rows
            .iter()
            .filter_map(|r| r.get::<&str, _>(0).map(str::to_string))
            .collect())
    }

    /// Diff the installed map against this store's, by name, across all
    /// resources. A column whose *type* changed is neither additive nor
    /// destructive: it is an error, because a type change means the shred
    /// writes a different value shape and rewriting stored data is a
    /// migration somebody must design (`L12`).
    fn diff_maps(
        &self,
        old_map: &RelMap,
        raw: &str,
        esc: &str,
    ) -> Result<(Vec<String>, Vec<String>), StoreError> {
        use std::collections::{HashMap, HashSet};
        let (mut adds, mut destructive) = (Vec::new(), Vec::new());
        let mut old_tables: HashMap<&str, &fhir_mssql_map::model::Table> = HashMap::new();
        for rm in old_map.resources.values() {
            for t in &rm.tables {
                old_tables.insert(t.name.as_str(), t);
            }
        }
        let mut new_names: HashSet<&str> = HashSet::new();
        for rm in self.map.resources.values() {
            for t in &rm.tables {
                new_names.insert(t.name.as_str());
                let Some(old_t) = old_tables.get(t.name.as_str()) else {
                    adds.push(fhir_mssql_map::ddl::create_table(raw, rm, t));
                    continue;
                };
                let old_cols: HashMap<&str, ColTy> =
                    old_t.cols.iter().map(|c| (c.name.as_str(), c.ty)).collect();
                let new_cols: HashSet<&str> = t.cols.iter().map(|c| c.name.as_str()).collect();
                for c in &t.cols {
                    match old_cols.get(c.name.as_str()) {
                        // No `COLUMN` keyword: T-SQL's `ADD` syntax rejects
                        // it outright (`M14.32`).
                        None => adds.push(format!(
                            "ALTER TABLE {esc}.[{}] ADD [{}] {}",
                            t.name,
                            c.name,
                            fhir_mssql_map::ddl::col_sql(c.ty)
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
                            "ALTER TABLE {esc}.[{}] DROP COLUMN [{name}]",
                            t.name
                        ));
                    }
                }
            }
        }
        // Every non-`Base` table carries `FOREIGN KEY (rid) REFERENCES
        // base(id)` (`create_table`); SQL Server refuses `DROP TABLE` on a
        // table something else still references, `ON DELETE CASCADE`
        // notwithstanding — that clause governs `DELETE`, not `DROP TABLE`.
        // Dropped children must therefore come out before their base table.
        // Found live: `destructive_changes_succeed_with_the_flag` failed with
        // "Could not drop object 'basic' because it is referenced by a
        // FOREIGN KEY constraint" (error 3726) until this ordering was added.
        let mut drop_children = Vec::new();
        let mut drop_bases = Vec::new();
        for (name, t) in &old_tables {
            if new_names.contains(name) {
                continue;
            }
            let stmt = format!("DROP TABLE {esc}.[{name}]");
            if t.kind == fhir_mssql_map::model::TableKind::Base {
                drop_bases.push(stmt);
            } else {
                drop_children.push(stmt);
            }
        }
        destructive.extend(drop_children);
        destructive.extend(drop_bases);
        let old_ix: HashSet<String> = old_map
            .resources
            .values()
            .flat_map(|rm| fhir_mssql_map::ddl::search_indexes(raw, rm))
            .collect();
        for rm in self.map.resources.values() {
            for stmt in fhir_mssql_map::ddl::search_indexes(raw, rm) {
                if !old_ix.contains(&stmt) {
                    adds.push(stmt);
                }
            }
        }
        Ok((adds, destructive))
    }

    /// Populate folded search columns (`P6.6`) for rows written before the
    /// column existed, returning how many distinct values were folded.
    ///
    /// An upgrade that added the column would otherwise leave it NULL on
    /// every existing row, and every non-`:exact` string search compares that
    /// column — so those resources would silently stop matching their own
    /// values. **Silent under-return is the one failure mode a clinical
    /// search must not have**, which is why this runs as part of
    /// [`Self::upgrade`] rather than as a step an operator can forget (`L13`,
    /// `L14`).
    ///
    /// Folds distinct *values* rather than rows — a surname repeats across
    /// patients — in bounded batches, and is **resumable**: each pass looks
    /// only at rows still NULL, so an interrupted run resumes where it
    /// stopped. `TOP`, not `LIMIT` (`M14.22`'s paging rule applies here too).
    ///
    /// Runs outside any transaction `upgrade` may have opened: this is a bulk
    /// data write, not schema DDL, and each batch commits independently so an
    /// interruption loses at most one batch's progress rather than all of it.
    ///
    /// # Errors
    /// On a database failure.
    pub async fn backfill_norm(&self) -> Result<usize, StoreError> {
        const BATCH: i64 = 1000;
        let s = quote_ident(&self.map.schema);
        let mut work: Vec<(String, String, String)> = Vec::new();
        for rm in self.map.resources.values() {
            for t in &rm.tables {
                for (src, dst) in &t.norm_cols {
                    work.push((t.name.clone(), src.clone(), dst.clone()));
                }
            }
        }
        if work.is_empty() {
            return Ok(0);
        }
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let mut total = 0usize;
        for (tn, src, dst) in &work {
            let table = quote_ident(tn);
            let src_c = quote_ident(src);
            let dst_c = quote_ident(dst);
            loop {
                let rows = conn
                    .query(
                        format!(
                            "SELECT DISTINCT TOP ({BATCH}) {src_c} FROM {s}.{table} \
                             WHERE {dst_c} IS NULL AND {src_c} IS NOT NULL"
                        ),
                        &[],
                    )
                    .await
                    .map_err(db_err)?
                    .into_first_result()
                    .await
                    .map_err(db_err)?;
                if rows.is_empty() {
                    break;
                }
                let n = rows.len();
                let vals: Vec<String> = rows
                    .iter()
                    .filter_map(|r| r.get::<&str, _>(0).map(str::to_string))
                    .collect();
                let sql = format!(
                    "UPDATE {s}.{table} SET {dst_c} = @P1 WHERE {src_c} = @P2 AND {dst_c} IS NULL"
                );
                for v in &vals {
                    let folded = fhir_mssql_map::fold::fold(v);
                    conn.execute(sql.as_str(), &[&folded.as_str(), &v.as_str()])
                        .await
                        .map_err(db_err)?;
                }
                total += n;
                if (n as i64) < BATCH {
                    break;
                }
            }
        }
        Ok(total)
    }

    /// Write a resource, creating it or replacing its current version.
    ///
    /// # Errors
    /// If the resource has no `resourceType`/`id`, if shredding rejects it, or
    /// on a database failure.
    pub async fn put(
        &self,
        resource: &serde_json::Value,
        audit: &crate::Audit,
    ) -> Result<crate::PutOutcome, StoreError> {
        let rtype = resource
            .get("resourceType")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| StoreError::Other("resource has no resourceType".into()))?
            .to_string();
        let rm = self
            .map
            .resources
            .get(&rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let out = fhir_mssql_map::shred::shred(rm, resource)?;
        let id = out
            .id
            .clone()
            .ok_or_else(|| StoreError::Other("resource has no id".into()))?;

        // Computed here, not by the database (`M14.19`/`M14.20`-equivalent): no
        // two engines render JSON alike, so a chain written by one could never
        // be verified by another unless the pre-image is engine-independent.
        let canon = fhir_mssql_map::canon::canonicalize(resource);
        let ts = utc_micros(std::time::SystemTime::now());
        let s = quote_ident(&self.map.schema);
        let base = quote_ident(&rm.base_table().name);
        let hist = quote_ident(
            &rm.find_table(fhir_mssql_map::model::TableKind::History)
                .map(|(_, t)| t.name.clone())
                .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?,
        );

        let mut conn = self.pool.get().await.map_err(pool_err)?;
        conn.simple_query("BEGIN TRANSACTION")
            .await
            .map_err(db_err)?;

        let result = self
            .put_in_tx(
                &mut conn, &s, &base, &hist, rm, &out, &id, &canon, &ts, audit,
            )
            .await;

        match result {
            Ok(outcome) => {
                conn.simple_query("COMMIT TRANSACTION")
                    .await
                    .map_err(db_err)?;
                Ok(outcome)
            }
            Err(e) => {
                // Best-effort: if the connection is already broken the
                // rollback fails too, and the original error is the one worth
                // returning.
                let _ = conn.simple_query("ROLLBACK TRANSACTION").await;
                Err(e)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn put_in_tx(
        &self,
        conn: &mut pool::Connection,
        s: &str,
        base: &str,
        hist: &str,
        rm: &fhir_mssql_map::model::ResourceMap,
        out: &fhir_mssql_map::shred::ShredOut,
        id: &str,
        canon: &str,
        ts: &str,
        audit: &crate::Audit,
    ) -> Result<crate::PutOutcome, StoreError> {
        // H5.4: serialise writers for this resource id before reading the
        // chain tip. `WITH (UPDLOCK, ROWLOCK)` holds the lock until commit, so
        // a second writer for the same id blocks here rather than racing the
        // tip read below. A create has no base row to lock; racing creates of
        // the same id still resolve on the history table's primary key.
        let _lock: Option<Row> = conn
            .query(
                format!(
                    "SELECT [version_id] FROM {s}.{base} WITH (UPDLOCK, ROWLOCK) WHERE [id] = @P1"
                ),
                &[&id],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;

        let prev_row = conn
            .query(
                format!(
                    "SELECT TOP (1) [version_id], [row_hash], [row_hash_sha3] FROM {s}.{hist} \
                     WHERE [id] = @P1 ORDER BY [version_id] DESC"
                ),
                &[&id],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;
        let (version_id, prev_256, prev_3): ChainTip = match prev_row {
            Some(r) => (
                r.get::<i64, _>(0).unwrap_or(0) + 1,
                r.get::<&[u8], _>(1).map(<[u8]>::to_vec),
                r.get::<&[u8], _>(2).map(<[u8]>::to_vec),
            ),
            None => (1, None, None),
        };

        let existed = conn
            .query(format!("SELECT 1 FROM {s}.{base} WHERE [id] = @P1"), &[&id])
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?
            .is_some();
        if existed {
            conn.execute(format!("DELETE FROM {s}.{base} WHERE [id] = @P1"), &[&id])
                .await
                .map_err(db_err)?;
        }

        // Base row first: every child has a foreign key to it.
        let mut cols = vec![
            "[id]".to_string(),
            "[version_id]".to_string(),
            "[last_updated]".to_string(),
        ];
        let mut vals: Vec<Bound> = vec![
            Bound::Str(Some(id.to_string())),
            Bound::I64(Some(version_id)),
            Bound::Str(Some(ts.to_string())),
        ];
        for r in out.rows.iter().filter(|r| r.table == 0) {
            for (name, v) in &r.cols {
                cols.push(quote_ident(name));
                vals.push(sqlval(v));
            }
        }
        insert_row(conn, s, base, &cols, &vals).await?;

        // Element tables, grouped so each is one insert per row (see the note
        // on `insert_row` about why this is not batched).
        let mut by_table: std::collections::BTreeMap<u32, Vec<&fhir_mssql_map::shred::Row>> =
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
            // The type each name binds as, so a column absent from a given
            // row (a legitimately null leaf) can still get a NULL of the
            // right SQL type rather than a generic one SQL Server refuses
            // against a non-NVARCHAR column.
            let types: Vec<ColTy> = names
                .iter()
                .map(|n| {
                    t.cols
                        .iter()
                        .find(|c| &c.name == n)
                        .map_or(ColTy::Text, |c| c.ty)
                })
                .collect();
            let mut cols = vec!["[rid]".to_string(), "[ords]".to_string()];
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
                insert_row(conn, s, &quote_ident(&t.name), &cols, &vals).await?;
            }
        }

        // Extensions and spill carry the surrogate primary key (M14.12): their
        // natural keys hold unbounded text, which cannot be part of a key on
        // this engine.
        if let Some((_, t)) = rm.find_table(fhir_mssql_map::model::TableKind::Ext) {
            for e in &out.ext {
                let (kind, text, num, b) = e.val.cols();
                let ords = fmt_ords(&e.ords);
                let ext_ord = e.ext_ord.to_string();
                let modifier = u8::from(e.modifier).to_string();
                let key = surrogate_key(&[id, &e.path, &ords, &modifier, &ext_ord, &e.leaf]);
                let cols = [
                    "[key_hash]",
                    "[rid]",
                    "[path]",
                    "[ords]",
                    "[modifier]",
                    "[ext_ord]",
                    "[url]",
                    "[leaf]",
                    "[v_kind]",
                    "[v_text]",
                    "[v_num]",
                    "[v_bool]",
                ]
                .map(String::from)
                .to_vec();
                let vals = vec![
                    Bound::Bytes(Some(key.to_vec())),
                    Bound::Str(Some(id.to_string())),
                    Bound::Str(Some(e.path.clone())),
                    Bound::Bytes(Some(ords.into_bytes())),
                    Bound::Bool(Some(e.modifier)),
                    Bound::I64(Some(i64::from(e.ext_ord))),
                    Bound::Str(e.url.clone()),
                    Bound::Str(Some(e.leaf.clone())),
                    Bound::Str(Some(kind.to_string())),
                    Bound::Str(text.map(str::to_string)),
                    Bound::Str(num.map(str::to_string)),
                    Bound::Bool(b),
                ];
                insert_row(conn, s, &quote_ident(&t.name), &cols, &vals).await?;
            }
        }

        if let Some((_, t)) = rm.find_table(fhir_mssql_map::model::TableKind::Deep) {
            for d in &out.deep {
                let (kind, text, num, b) = d.val.cols();
                let ords = fmt_ords(&d.ords);
                let key = surrogate_key(&[id, &d.path, &ords, &d.leaf]);
                let cols = [
                    "[key_hash]",
                    "[rid]",
                    "[path]",
                    "[ords]",
                    "[leaf]",
                    "[v_kind]",
                    "[v_text]",
                    "[v_num]",
                    "[v_bool]",
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
                    Bound::Bool(b),
                ];
                insert_row(conn, s, &quote_ident(&t.name), &cols, &vals).await?;
            }
        }

        if let Some((_, t)) = rm.find_table(fhir_mssql_map::model::TableKind::Contained) {
            for (ord, v) in &out.contained {
                let cols = ["[rid]", "[ord]", "[resource]"].map(String::from).to_vec();
                let vals = vec![
                    Bound::Str(Some(id.to_string())),
                    Bound::I64(Some(i64::from(*ord))),
                    Bound::Str(Some(v.to_string())),
                ];
                insert_row(conn, s, &quote_ident(&t.name), &cols, &vals).await?;
            }
        }

        let op = if existed { "U" } else { "C" };
        let pre = crate::chain::preimage(id, version_id, ts, op, Some(canon), &audit.actor);
        let (row_hash, row_sha3) = crate::chain::link(prev_256.as_deref(), prev_3.as_deref(), &pre);
        let row_mac = self
            .keys
            .signing()
            .map(|k| crate::chain::mac(k, prev_256.as_deref(), &pre));

        let cols = [
            "[id]",
            "[version_id]",
            "[last_updated]",
            "[op]",
            "[resource]",
            "[actor]",
            "[actor_source]",
            "[client]",
            "[request_id]",
            "[reason]",
            "[prev_hash]",
            "[row_hash]",
            "[prev_hash_sha3]",
            "[row_hash_sha3]",
            "[row_mac]",
        ]
        .map(String::from)
        .to_vec();
        let vals = vec![
            Bound::Str(Some(id.to_string())),
            Bound::I64(Some(version_id)),
            Bound::Str(Some(ts.to_string())),
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
        insert_row(conn, s, hist, &cols, &vals).await?;

        Ok(crate::PutOutcome {
            id: id.to_string(),
            version_id,
            created: !existed,
        })
    }

    /// Read a resource back, reconstructed from its rows.
    ///
    /// # Errors
    /// If the map has no such resource type, if the shredder's residue check
    /// fails (a stored row went unread — an integrity error, `R4.7`), or on a
    /// database failure.
    pub async fn get(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Option<serde_json::Value>, StoreError> {
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let s = quote_ident(&self.map.schema);
        let base = quote_ident(&rm.base_table().name);
        let mut conn = self.pool.get().await.map_err(pool_err)?;

        // R4.5. A read touches the base table and every child table as
        // separate statements, and this engine's *default* isolation is
        // `READ COMMITTED`: unlike PostgreSQL/MySQL's `REPEATABLE READ`
        // default, a bare `BEGIN TRANSACTION` does **not** give those
        // statements one snapshot, even with `READ_COMMITTED_SNAPSHOT` on —
        // that setting still only gives each *statement* its own snapshot,
        // not the whole transaction one. Both were tried live and only the
        // second actually stopped the torn read `tests/concurrency.rs`'s
        // `reads_never_tear_under_concurrent_writes` reproduces:
        // `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` before `BEGIN
        // TRANSACTION`, which requires `ALLOW_SNAPSHOT_ISOLATION` at the
        // database level (`scripts/db.sh`'s `post_ready`) — and requires a
        // database this port can alter at all, which is why the DSN now
        // names one (`database=fhir_mssql`) rather than landing in `master`,
        // where the option cannot be set.
        conn.simple_query("SET TRANSACTION ISOLATION LEVEL SNAPSHOT")
            .await
            .map_err(db_err)?;
        conn.simple_query("BEGIN TRANSACTION")
            .await
            .map_err(db_err)?;

        let present = conn
            .query(format!("SELECT 1 FROM {s}.{base} WHERE [id] = @P1"), &[&id])
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;
        if present.is_none() {
            let _ = conn.simple_query("ROLLBACK TRANSACTION").await;
            // `SET TRANSACTION ISOLATION LEVEL` is session-scoped, not
            // transaction-scoped — on this *pooled* connection it must not
            // leak `SNAPSHOT` isolation to whichever caller borrows the
            // connection next, the same leak risk `purge`'s `SESSION_CONTEXT`
            // erasure flag has and is guarded against for the same reason.
            let _ = conn
                .simple_query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
                .await;
            return Ok(None);
        }

        // The read loop below is wrapped so that a failure partway through —
        // a query error, a malformed `[ords]` image — still reaches the
        // rollback and isolation-level reset after it, rather than
        // returning early with an open `SNAPSHOT` transaction still on this
        // *pooled* connection. Before this wrap, an error here would have
        // done exactly that: `?` inside the loop returned straight out of
        // `get`, past both cleanup statements, leaving the next borrower of
        // this connection to inherit an open transaction and `SNAPSHOT`
        // isolation it never asked for.
        let result = read_resource_rows(&mut conn, &s, rm, id).await;
        let _ = conn.simple_query("ROLLBACK TRANSACTION").await;
        let _ = conn
            .simple_query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
            .await;
        let input = result?;

        let v = fhir_mssql_map::reconstruct::reconstruct(rm, &input, Some(id))?;
        Ok(Some(v))
    }
}

async fn read_resource_rows(
    conn: &mut pool::Connection,
    s: &str,
    rm: &fhir_mssql_map::model::ResourceMap,
    id: &str,
) -> Result<fhir_mssql_map::reconstruct::ReconIn, StoreError> {
    use fhir_mssql_map::model::TableKind;
    use fhir_mssql_map::reconstruct::{InRow, ReconIn};

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
                    sel.push("[ords]".to_string());
                }
                sel.extend(names.iter().map(|(n, _)| quote_ident(n)));
                if sel.is_empty() {
                    sel.push("NULL".to_string());
                }
                let rows = conn
                    .query(
                        format!(
                            "SELECT {} FROM {s}.{table} WHERE [{key}] = @P1",
                            sel.join(", ")
                        ),
                        &[&id],
                    )
                    .await
                    .map_err(db_err)?
                    .into_first_result()
                    .await
                    .map_err(db_err)?;
                for row in rows {
                    let mut ords = Vec::new();
                    let mut off = 0usize;
                    if t.kind == TableKind::Elem {
                        let img = ords_bytes_to_str(&row, 0)?;
                        ords = parse_ords(img)?;
                        off = 1;
                    }
                    let mut cols = std::collections::HashMap::new();
                    for (i, (n, ty)) in names.iter().enumerate() {
                        if let Some(v) = cell_text(&row, i + off, *ty, n)? {
                            cols.insert(n.clone(), v);
                        }
                    }
                    input.tables[ti].push(InRow { ords, cols });
                }
            }
            TableKind::Ext => {
                let rows = conn
                    .query(
                        format!(
                            "SELECT [path],[ords],[modifier],[ext_ord],[url],[leaf],\
                                        [v_kind],[v_text],[v_num],[v_bool] \
                                 FROM {s}.{table} WHERE [rid] = @P1"
                        ),
                        &[&id],
                    )
                    .await
                    .map_err(db_err)?
                    .into_first_result()
                    .await
                    .map_err(db_err)?;
                for r in rows {
                    let ords_img = ords_bytes_to_str(&r, 1)?;
                    input.ext.push(fhir_mssql_map::shred::ExtRow {
                        path: r.get::<&str, _>(0).unwrap_or_default().to_string(),
                        ords: parse_ords(ords_img)?,
                        modifier: r.get::<bool, _>(2).unwrap_or(false),
                        ext_ord: r.get::<i16, _>(3).unwrap_or(0),
                        url: r.get::<&str, _>(4).map(str::to_string),
                        leaf: r.get::<&str, _>(5).unwrap_or_default().to_string(),
                        val: leaf_from_cols(
                            r.get::<&str, _>(6).unwrap_or_default(),
                            r.get::<&str, _>(7).map(str::to_string),
                            r.get::<&str, _>(8).map(str::to_string),
                            r.get::<bool, _>(9),
                        ),
                    });
                }
            }
            TableKind::Deep => {
                let rows = conn
                    .query(
                        format!(
                            "SELECT [path],[ords],[leaf],[v_kind],[v_text],[v_num],[v_bool] \
                                 FROM {s}.{table} WHERE [rid] = @P1"
                        ),
                        &[&id],
                    )
                    .await
                    .map_err(db_err)?
                    .into_first_result()
                    .await
                    .map_err(db_err)?;
                for r in rows {
                    let ords_img = ords_bytes_to_str(&r, 1)?;
                    input.deep.push(fhir_mssql_map::shred::DeepRow {
                        path: r.get::<&str, _>(0).unwrap_or_default().to_string(),
                        ords: parse_ords(ords_img)?,
                        leaf: r.get::<&str, _>(2).unwrap_or_default().to_string(),
                        val: leaf_from_cols(
                            r.get::<&str, _>(3).unwrap_or_default(),
                            r.get::<&str, _>(4).map(str::to_string),
                            r.get::<&str, _>(5).map(str::to_string),
                            r.get::<bool, _>(6),
                        ),
                    });
                }
            }
            TableKind::Contained => {
                let rows = conn
                    .query(
                        format!("SELECT [ord],[resource] FROM {s}.{table} WHERE [rid] = @P1"),
                        &[&id],
                    )
                    .await
                    .map_err(db_err)?
                    .into_first_result()
                    .await
                    .map_err(db_err)?;
                for r in rows {
                    let raw: &str = r.get(1).unwrap_or_default();
                    let v = serde_json::from_str(raw)
                        .map_err(|e| StoreError::Other(format!("contained: {e}")))?;
                    input.contained.push((r.get::<i16, _>(0).unwrap_or(0), v));
                }
            }
            TableKind::History => {}
        }
    }

    Ok(input)
}

impl MsSqlStore {
    /// `(schema, history table)` for a resource type, both already quoted.
    fn hist_target(&self, rtype: &str) -> Result<(String, String), StoreError> {
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let hist = rm
            .find_table(fhir_mssql_map::model::TableKind::History)
            .map(|(_, t)| quote_ident(&t.name))
            .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?;
        Ok((quote_ident(&self.map.schema), hist))
    }

    /// Every stored version of a resource, newest first.
    ///
    /// A deletion appears with `op == 'D'` and no resource, which is how a
    /// reader tells "deleted" from "never existed": the base row is gone in
    /// both cases, so history is the only witness.
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
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let rows = conn
            .query(
                format!(
                    "SELECT [version_id],[last_updated],[op],[resource] FROM {s}.{hist} \
                     WHERE [id] = @P1 ORDER BY [version_id] DESC"
                ),
                &[&id],
            )
            .await
            .map_err(db_err)?
            .into_first_result()
            .await
            .map_err(db_err)?;
        let mut out = Vec::with_capacity(rows.len());
        for r in &rows {
            out.push(hist_entry(r)?);
        }
        Ok(out)
    }

    /// One specific version, as it was stored.
    ///
    /// Returns a `HistEntry`, not a bare resource: a deleted version has no
    /// content, and a caller must be able to tell "version 3 was a deletion"
    /// (410) from "version 3 does not exist" (404).
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
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let row = conn
            .query(
                format!(
                    "SELECT [version_id],[last_updated],[op],[resource] FROM {s}.{hist} \
                     WHERE [id] = @P1 AND [version_id] = @P2"
                ),
                &[&id, &version_id],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;
        row.as_ref().map(hist_entry).transpose()
    }

    /// Delete a resource, leaving a tombstone in history.
    ///
    /// Returns the tombstone's version, or `None` if there was nothing to
    /// delete. The base row goes, cascading to its children; history does
    /// not, because it has no foreign key to the base table — a deletion
    /// that erased its own evidence would defeat the audit trail.
    ///
    /// # Errors
    /// If the resource type is unknown or on a database failure.
    pub async fn delete(
        &self,
        rtype: &str,
        id: &str,
        audit: &crate::Audit,
    ) -> Result<Option<i64>, StoreError> {
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let base = quote_ident(&rm.base_table().name);
        let (s, hist) = self.hist_target(rtype)?;
        let ts = utc_micros(std::time::SystemTime::now());

        let mut conn = self.pool.get().await.map_err(pool_err)?;
        conn.simple_query("BEGIN TRANSACTION")
            .await
            .map_err(db_err)?;

        let result = self
            .delete_in_tx(&mut conn, &s, &base, &hist, id, &ts, audit)
            .await;
        match result {
            Ok(v) => {
                conn.simple_query("COMMIT TRANSACTION")
                    .await
                    .map_err(db_err)?;
                Ok(v)
            }
            Err(e) => {
                let _ = conn.simple_query("ROLLBACK TRANSACTION").await;
                Err(e)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn delete_in_tx(
        &self,
        conn: &mut pool::Connection,
        s: &str,
        base: &str,
        hist: &str,
        id: &str,
        ts: &str,
        audit: &crate::Audit,
    ) -> Result<Option<i64>, StoreError> {
        let present = conn
            .query(
                format!("SELECT 1 FROM {s}.{base} WITH (UPDLOCK, ROWLOCK) WHERE [id] = @P1"),
                &[&id],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?
            .is_some();
        if !present {
            return Ok(None);
        }

        let prev_row = conn
            .query(
                format!(
                    "SELECT TOP (1) [version_id],[row_hash],[row_hash_sha3] FROM {s}.{hist} \
                     WHERE [id] = @P1 ORDER BY [version_id] DESC"
                ),
                &[&id],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;
        let (version_id, prev_256, prev_3): ChainTip = match prev_row {
            Some(r) => (
                r.get::<i64, _>(0).unwrap_or(0) + 1,
                r.get::<&[u8], _>(1).map(<[u8]>::to_vec),
                r.get::<&[u8], _>(2).map(<[u8]>::to_vec),
            ),
            None => (1, None, None),
        };

        conn.execute(format!("DELETE FROM {s}.{base} WHERE [id] = @P1"), &[&id])
            .await
            .map_err(db_err)?;

        // No resource in the pre-image: there is nothing to commit to, but a
        // tombstone still extends the chain.
        let pre = crate::chain::preimage(id, version_id, ts, "D", None, &audit.actor);
        let (row_hash, row_sha3) = crate::chain::link(prev_256.as_deref(), prev_3.as_deref(), &pre);
        let row_mac = self
            .keys
            .signing()
            .map(|k| crate::chain::mac(k, prev_256.as_deref(), &pre));

        let cols = [
            "[id]",
            "[version_id]",
            "[last_updated]",
            "[op]",
            "[actor]",
            "[actor_source]",
            "[client]",
            "[request_id]",
            "[reason]",
            "[prev_hash]",
            "[row_hash]",
            "[prev_hash_sha3]",
            "[row_hash_sha3]",
            "[row_mac]",
        ]
        .map(String::from)
        .to_vec();
        let vals = vec![
            Bound::Str(Some(id.to_string())),
            Bound::I64(Some(version_id)),
            Bound::Str(Some(ts.to_string())),
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
        insert_row(conn, s, hist, &cols, &vals).await?;

        Ok(Some(version_id))
    }

    /// Recompute every history chain and report what does not match.
    ///
    /// An empty result is the claim "nothing in history has been altered
    /// since it was written". Rows predating the chain columns have no
    /// stored hash and are skipped rather than reported: calling them breaks
    /// would train an operator to ignore the report.
    ///
    /// # Errors
    /// On a database failure.
    pub async fn verify_audit(&self) -> Result<Vec<crate::ChainBreak>, StoreError> {
        use fhir_mssql_map::model::TableKind;
        let s = quote_ident(&self.map.schema);
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let mut breaks = Vec::new();

        // Counter-signatures up front: one small table for the whole schema,
        // and querying it per history row would make a linear walk
        // quadratic.
        let mut countersigns: std::collections::HashMap<(String, String, i64), String> =
            std::collections::HashMap::new();
        let cs_table = qualified(&self.map.schema, "fhir_mssql_countersign");
        let cs = conn
            .query(
                format!("SELECT [rtype],[id],[version_id],[row_mac] FROM {cs_table}"),
                &[],
            )
            .await
            .map_err(db_err)?
            .into_first_result()
            .await
            .map_err(db_err)?;
        for r in &cs {
            let rtype: &str = r.get(0).unwrap_or_default();
            let id: &str = r.get(1).unwrap_or_default();
            let version_id: i64 = r.get(2).unwrap_or(0);
            let row_mac: &str = r.get(3).unwrap_or_default();
            countersigns.insert(
                (rtype.to_string(), id.to_string(), version_id),
                row_mac.to_string(),
            );
        }

        for rm in self.map.resources.values() {
            let Some((_, t)) = rm.find_table(TableKind::History) else {
                continue;
            };
            let hist = quote_ident(&t.name);
            let rows = conn
                .query(
                    format!(
                        "SELECT [id],[version_id],[last_updated],[op],[resource],[actor],\
                                [prev_hash],[row_hash],[prev_hash_sha3],[row_hash_sha3],[row_mac] \
                         FROM {s}.{hist} ORDER BY [id], [version_id]"
                    ),
                    &[],
                )
                .await
                .map_err(db_err)?
                .into_first_result()
                .await
                .map_err(db_err)?;

            let mut tip: std::collections::HashMap<String, ChainTip> =
                std::collections::HashMap::new();
            for r in &rows {
                let id: &str = r.get(0).unwrap_or_default();
                let version_id: i64 = r.get(1).unwrap_or(0);
                // `DATETIME2(6)`, not `NVARCHAR` — see `hist_entry`. The
                // reformatted string MUST reproduce exactly what `put`/
                // `delete` wrote into the chain pre-image (`utc_micros`'s
                // `%Y-%m-%dT%H:%M:%S.ffffff`), or every row's chain fails to
                // verify against itself. Proven by running this against a
                // live server: `verify_audit` returning zero breaks after a
                // create, an update and a delete is what confirms the two
                // formatters agree, not just that each compiles.
                let last_updated = r
                    .get::<chrono::NaiveDateTime, _>(2)
                    .map(|d| d.format("%Y-%m-%dT%H:%M:%S%.6f").to_string())
                    .unwrap_or_default();
                let op: &str = r.get(3).unwrap_or_default();
                let resource: Option<&str> = r.get(4);
                let actor: &str = r.get(5).unwrap_or_default();
                let stored_prev_256: Option<&[u8]> = r.get(6);
                let stored_row_256: Option<&[u8]> = r.get(7);
                let stored_prev_3: Option<&[u8]> = r.get(8);
                let stored_row_3: Option<&[u8]> = r.get(9);
                let row_mac: Option<&str> = r.get(10);

                let (expect_prev_256, expect_prev_3) = tip
                    .get(id)
                    .map(|(_, a, b)| (a.clone(), b.clone()))
                    .unwrap_or((None, None));

                // Verified against the row's *stored* `prev_hash`, not the
                // tip the walk arrived with. They agree for an ordinary row,
                // and where they do not the link check below is what says
                // so. Separating them lets an erasure tombstone keep a
                // meaningful tag: its predecessors were deleted on purpose.
                let pre =
                    crate::chain::preimage(id, version_id, &last_updated, op, resource, actor);
                check_mac(
                    &self.keys,
                    &countersigns,
                    &rm.name,
                    id,
                    version_id,
                    row_mac,
                    expect_prev_256.as_deref(),
                    &pre,
                    &mut breaks,
                );

                // Rows written before the chain columns existed have no
                // stored hash at all — not a break, just unwitnessed.
                let (Some(stored_row_256), Some(stored_row_3)) = (stored_row_256, stored_row_3)
                else {
                    tip.insert(id.to_string(), (version_id, None, None));
                    continue;
                };

                if expect_prev_256.as_deref() != stored_prev_256 {
                    breaks.push(crate::ChainBreak::new(
                        rm.name.clone(),
                        id.to_string(),
                        version_id,
                        "sha256",
                        "prev_hash does not match the previous row's row_hash".to_string(),
                    ));
                }
                if expect_prev_3.as_deref() != stored_prev_3 {
                    breaks.push(crate::ChainBreak::new(
                        rm.name.clone(),
                        id.to_string(),
                        version_id,
                        "sha3-256",
                        "prev_hash_sha3 does not match the previous row's row_hash_sha3"
                            .to_string(),
                    ));
                }
                let (want_256, want_3) =
                    crate::chain::link(expect_prev_256.as_deref(), expect_prev_3.as_deref(), &pre);
                if want_256 != stored_row_256 {
                    breaks.push(crate::ChainBreak::new(
                        rm.name.clone(),
                        id.to_string(),
                        version_id,
                        "sha256",
                        "row_hash does not match the recomputed chain".to_string(),
                    ));
                }
                if want_3 != stored_row_3 {
                    breaks.push(crate::ChainBreak::new(
                        rm.name.clone(),
                        id.to_string(),
                        version_id,
                        "sha3-256",
                        "row_hash_sha3 does not match the recomputed chain".to_string(),
                    ));
                }
                tip.insert(
                    id.to_string(),
                    (
                        version_id,
                        Some(stored_row_256.to_vec()),
                        Some(stored_row_3.to_vec()),
                    ),
                );
            }
        }
        Ok(breaks)
    }

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
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let base = quote_ident(&rm.base_table().name);
        let (s, hist) = self.hist_target(rtype)?;
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        conn.simple_query("BEGIN TRANSACTION")
            .await
            .map_err(db_err)?;

        let result = async {
            let existed = conn
                .query(format!("SELECT 1 FROM {s}.{base} WHERE [id] = @P1"), &[&id])
                .await
                .map_err(db_err)?
                .into_row()
                .await
                .map_err(db_err)?
                .is_some();

            // Counted before the delete, from `COUNT(*)`, rather than trusted
            // from `execute().total()` after it. The `M3.17` trigger below is
            // `INSTEAD OF DELETE` and issues its own nested `DELETE` inside
            // itself; running this live showed `total()` reporting **6** for
            // 3 actual history rows — SQL Server counts the outer statement's
            // rowcount and the trigger's own nested statement separately, and
            // `ExecuteResult::total()` sums every `DONE` token in the batch.
            // A row count is exactly the kind of number `W16.10` requires be
            // trustworthy, so it is not taken from a value that this store's
            // own trigger design makes ambiguous.
            let versions_erased: u64 = conn
                .query(
                    format!("SELECT COUNT(*) FROM {s}.{hist} WHERE [id] = @P1"),
                    &[&id],
                )
                .await
                .map_err(db_err)?
                .into_row()
                .await
                .map_err(db_err)?
                .and_then(|r| r.get::<i32, _>(0))
                .unwrap_or(0) as u64;

            // The `M3.17` append-only trigger's escape hatch (`ddl.rs`,
            // `append_only_triggers`): `SESSION_CONTEXT` is connection-scoped,
            // not transaction-scoped, so on a *pooled* connection it MUST be
            // cleared again before this connection can be trusted for another
            // caller's ordinary `DELETE` — set immediately before the delete
            // it authorises, cleared in every exit path below, never assumed
            // to reset itself.
            conn.simple_query(
                "EXEC sp_set_session_context @key = N'fhir_mssql_erasure', @value = N'on'",
            )
            .await
            .map_err(db_err)?;

            conn.execute(format!("DELETE FROM {s}.{hist} WHERE [id] = @P1"), &[&id])
                .await
                .map_err(db_err)?;

            conn.simple_query(
                "EXEC sp_set_session_context @key = N'fhir_mssql_erasure', @value = NULL",
            )
            .await
            .map_err(db_err)?;

            if existed {
                conn.execute(format!("DELETE FROM {s}.{base} WHERE [id] = @P1"), &[&id])
                    .await
                    .map_err(db_err)?;
            }

            let ts = utc_micros(std::time::SystemTime::now());
            let pre = crate::chain::preimage(id, 0, &ts, "P", None, &audit.actor);
            let (row_hash, row_sha3) = crate::chain::link(None, None, &pre);
            let row_mac = self
                .keys
                .signing()
                .map(|k| crate::chain::mac(k, None, &pre));

            let cols = [
                "[id]",
                "[version_id]",
                "[last_updated]",
                "[op]",
                "[actor]",
                "[actor_source]",
                "[client]",
                "[request_id]",
                "[reason]",
                "[row_hash]",
                "[row_hash_sha3]",
                "[row_mac]",
            ]
            .map(String::from)
            .to_vec();
            let vals = vec![
                Bound::Str(Some(id.to_string())),
                Bound::I64(Some(0)),
                Bound::Str(Some(ts)),
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
            insert_row(&mut conn, &s, &hist, &cols, &vals).await?;

            Ok::<_, StoreError>(crate::PurgeReport {
                versions_erased,
                existed,
            })
        }
        .await;

        let outcome = match result {
            Ok(report) => {
                conn.simple_query("COMMIT TRANSACTION")
                    .await
                    .map_err(db_err)?;
                Ok(report)
            }
            Err(e) => {
                let _ = conn.simple_query("ROLLBACK TRANSACTION").await;
                Err(e)
            }
        };

        // Belt and suspenders on the inline clear above: if any statement
        // between setting the flag and clearing it failed, the inline clear
        // was skipped and this connection would go back to the pool armed to
        // let an unrelated caller's ordinary `DELETE` through. Best-effort —
        // if the connection is unusable there is nothing left to protect.
        let _ = conn
            .simple_query("EXEC sp_set_session_context @key = N'fhir_mssql_erasure', @value = NULL")
            .await;

        outcome
    }

    /// Record one disclosure: who saw what, and what they were told (`PR12.5`).
    ///
    /// # Errors
    /// On a database failure.
    pub async fn log_access(&self, rec: &crate::AccessRecord) -> Result<(), StoreError> {
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let s = quote_ident(&self.map.schema);
        let cols = [
            "[ts]",
            "[request_id]",
            "[actor]",
            "[actor_source]",
            "[client]",
            "[interaction]",
            "[rtype]",
            "[id]",
            "[version_id]",
            "[outcome]",
            "[result_count]",
            "[reason]",
        ]
        .map(String::from)
        .to_vec();
        let vals = vec![
            Bound::Str(Some(utc_micros(std::time::SystemTime::now()))),
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
        insert_row(&mut conn, &s, "[fhir_mssql_access_log]", &cols, &vals)
            .await
            .map_err(|e| StoreError::Other(format!("log_access: {e}")))?;
        Ok(())
    }

    /// Log several disclosures at once. Test/dev convenience — there is no
    /// batched `INSERT` underneath, just a loop.
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
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let log = qualified(&self.map.schema, "fhir_mssql_access_log");
        let row = conn
            .query(format!("SELECT COUNT(*) FROM {log}"), &[])
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;
        Ok(row.and_then(|r| r.get::<i32, _>(0)).unwrap_or(0).into())
    }

    /// How many tables this store's schema currently has. Test/dev
    /// convenience, for asserting `init` actually built something rather
    /// than merely returning a statement count.
    ///
    /// # Errors
    /// On a database failure.
    pub async fn table_count(&self) -> Result<usize, StoreError> {
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let row = conn
            .query(
                "SELECT COUNT(*) FROM sys.tables t JOIN sys.schemas s \
                 ON s.schema_id = t.schema_id WHERE s.name = @P1",
                &[&self.map.schema.as_str()],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;
        Ok(row.and_then(|r| r.get::<i32, _>(0)).unwrap_or(0) as usize)
    }

    /// How many triggers this store's schema currently has — the enforcement
    /// behind `M3.17`, counted separately from `table_count` because a schema
    /// with its tables but not its triggers looks healthy while guaranteeing
    /// nothing.
    ///
    /// # Errors
    /// On a database failure.
    pub async fn trigger_count(&self) -> Result<usize, StoreError> {
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        let row = conn
            .query(
                "SELECT COUNT(*) FROM sys.triggers tr \
                 JOIN sys.tables t ON t.object_id = tr.parent_id \
                 JOIN sys.schemas s ON s.schema_id = t.schema_id \
                 WHERE s.name = @P1",
                &[&self.map.schema.as_str()],
            )
            .await
            .map_err(db_err)?
            .into_row()
            .await
            .map_err(db_err)?;
        Ok(row.and_then(|r| r.get::<i32, _>(0)).unwrap_or(0) as usize)
    }

    /// Run arbitrary SQL against this store's pool. Test/dev convenience —
    /// tampering with history to prove `verify_audit` catches it, or probing
    /// the append-only trigger directly, needs a way in that is not one of
    /// this store's own guarded methods.
    ///
    /// # Errors
    /// On a database failure.
    pub async fn exec_raw(&self, sql: &str) -> Result<(), StoreError> {
        let mut conn = self.pool.get().await.map_err(pool_err)?;
        conn.simple_query(sql).await.map_err(db_err)?;
        Ok(())
    }
}

// --------------------------------------------------------------------- search

impl MsSqlStore {
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
        sort: &[crate::mssql_search::SortKey],
        want_total: bool,
    ) -> Result<crate::SearchOutcome, StoreError> {
        self.search_page(rtype, params, count, offset, sort, want_total, None)
            .await
    }

    /// A page, with an optional keyset cursor.
    ///
    /// `after_id` narrows only the page query, never the count: a `_total`
    /// that shrank as a caller paged would make paging impossible to drive.
    ///
    /// Values are bound, never interpolated — the invariant this store's
    /// binding tests protect. Every `@Pn` in the compiled text corresponds
    /// positionally to `q.binds`, so the two are always converted and passed
    /// together.
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
        sort: &[crate::mssql_search::SortKey],
        want_total: bool,
        after_id: Option<&str>,
    ) -> Result<crate::SearchOutcome, StoreError> {
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let q = crate::mssql_search::build_search_sql(
            &self.map, rm, params, count, offset, sort, after_id,
        )?;
        let mut conn = self.pool.get().await.map_err(pool_err)?;

        let total = if want_total {
            let binds: Vec<Bound> = q
                .binds
                .iter()
                .take(q.count_binds)
                .map(|b| Bound::Str(Some(b.clone())))
                .collect();
            let params: Vec<&dyn tiberius::ToSql> =
                binds.iter().map(|b| b as &dyn tiberius::ToSql).collect();
            let row = conn
                .query(&q.count_sql, &params)
                .await
                .map_err(|e| StoreError::Other(format!("count: {e}\n{}", q.count_sql)))?
                .into_row()
                .await
                .map_err(|e| StoreError::Other(format!("count: {e}\n{}", q.count_sql)))?;
            row.and_then(|r| r.get::<i32, _>(0)).map(i64::from)
        } else {
            None
        };

        let binds: Vec<Bound> = q
            .binds
            .iter()
            .map(|b| Bound::Str(Some(b.clone())))
            .collect();
        let params: Vec<&dyn tiberius::ToSql> =
            binds.iter().map(|b| b as &dyn tiberius::ToSql).collect();
        let rows = conn
            .query(&q.sql, &params)
            .await
            .map_err(|e| StoreError::Other(format!("search: {e}\n{}", q.sql)))?
            .into_first_result()
            .await
            .map_err(|e| StoreError::Other(format!("search: {e}\n{}", q.sql)))?;
        let ids: Vec<String> = rows
            .iter()
            .map(|r| r.get::<&str, _>(0).unwrap_or_default().to_string())
            .collect();
        Ok(crate::SearchOutcome { ids, total })
    }
}

/// Verify one row's keyed tag, recording a finding only when the tag is
/// present and wrong.
///
/// - **Absent** — written unkeyed. Not a finding.
/// - **Unverifiable** — signed under a key this process does not hold. A
///   warning, never a break: "I cannot check this" and "this was altered"
///   are different claims, and conflating them makes the report useless.
/// - **Mismatch** — a finding, and it stays one even if a counter-signature
///   vouches for the row, or re-signing would be a way to bless forged
///   history.
///
/// Found missing entirely by running the concurrency-suite tamper test live:
/// `verify_audit` reported the `sha256`/`sha3-256` breaks from a tampered
/// `actor` column but never the `hmac-sha256` one, because nothing here ever
/// read `[row_mac]` back — the column was written on every insert and never
/// checked.
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

fn hist_entry(r: &Row) -> Result<crate::HistEntry, StoreError> {
    let version_id: i64 = r.get(0).unwrap_or(0);
    // `[last_updated]` is `DATETIME2(6)`, not `NVARCHAR` — the same class of
    // bug `cell_text` exists to avoid, caught here by running this against a
    // live server: the very first `history()` call panicked with `Row::get`
    // trying to interpret a `DateTime2` as a `String`.
    let last_updated = r
        .get::<chrono::NaiveDateTime, _>(1)
        .map(|d| d.format("%Y-%m-%dT%H:%M:%S%.6f").to_string())
        .unwrap_or_default();
    let op: &str = r.get(2).unwrap_or_default();
    let raw: Option<&str> = r.get(3);
    let resource = match raw {
        Some(t) => Some(
            serde_json::from_str(t)
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

/// A value ready to bind, independent of `tiberius`'s borrowed `ToSql`
/// lifetime — needed because rows are built up in owned `Vec<Bound>` before
/// the statement that consumes them exists.
enum Bound {
    Str(Option<String>),
    I64(Option<i64>),
    Bool(Option<bool>),
    Bytes(Option<Vec<u8>>),
}

impl Bound {
    fn as_column_data(&self) -> ColumnData<'_> {
        match self {
            // A NULL still carries its target SQL type: SQL Server rejects an
            // NVARCHAR-typed NULL parameter bound against a VARBINARY column
            // with "Implicit conversion ... is not allowed" even though the
            // value itself is NULL. Found by running `put()` against a live
            // server on the very first insert — every history row's first
            // version has no previous hash, so `prev_hash` is always NULL on
            // a create, and a single untyped `Bound::Null` failed every one.
            Bound::Str(s) => ColumnData::String(s.as_deref().map(Into::into)),
            Bound::I64(i) => ColumnData::I64(*i),
            Bound::Bool(b) => ColumnData::Bit(*b),
            Bound::Bytes(b) => ColumnData::Binary(b.as_deref().map(Into::into)),
        }
    }
}

/// The correctly-typed NULL for an arbitrary data column, chosen the same way
/// `col_sql` chose its physical type. Needed because an element table's
/// columns are heterogeneous and this function does not know which one it is
/// filling in until the caller tells it.
fn null_for_ty(ty: ColTy) -> Bound {
    match ty {
        ColTy::Bool => Bound::Bool(None),
        ColTy::Int | ColTy::BigInt => Bound::I64(None),
        ColTy::Digest => Bound::Bytes(None),
        ColTy::Numeric
        | ColTy::Text
        | ColTy::TextC
        | ColTy::TextIdx
        | ColTy::Date
        | ColTy::Timestamptz
        | ColTy::Jsonb => Bound::Str(None),
    }
}

impl tiberius::ToSql for Bound {
    fn to_sql(&self) -> ColumnData<'_> {
        self.as_column_data()
    }
}

/// One `INSERT`, one row.
///
/// Not batched into a multi-row `INSERT … VALUES (..), (..), ..` the way the
/// MySQL store batches: `tiberius` builds each parameter list against a fixed
/// `@P1..@PN` count per call rather than a query builder, so a variable-width
/// batch needs the SQL string rebuilt per batch size anyway, and one row at a
/// time is the version proven against a live server in the time available.
/// Batching is a performance improvement, not a correctness one — tracked as
/// a gap, not silently assumed.
async fn insert_row(
    conn: &mut pool::Connection,
    schema: &str,
    table: &str,
    cols: &[String],
    vals: &[Bound],
) -> Result<(), StoreError> {
    let marks: Vec<String> = (1..=vals.len()).map(|i| format!("@P{i}")).collect();
    let sql = format!(
        "INSERT INTO {schema}.{table} ({}) VALUES ({})",
        cols.join(", "),
        marks.join(", ")
    );
    let params: Vec<&dyn tiberius::ToSql> =
        vals.iter().map(|v| v as &dyn tiberius::ToSql).collect();
    conn.execute(sql, &params)
        .await
        .map_err(|e| StoreError::Other(format!("inserting into {table}: {e}")))?;
    Ok(())
}

/// Bind a shredded value against the physical type `col_sql` declared for it.
///
/// `Bool` and `Bytes` are the two variants that are not simply text: `Bool`
/// binds `BIT` natively rather than the `"true"`/`"false"` text image
/// `cell_text` reads back, and `Bytes` is `U4a`'s binary digest, never hex.
/// Everything else — including `Date`/`Ts`, whose columns are physically
/// `DATE`/`DATETIME2(6)` — binds as `NVARCHAR` text and relies on SQL
/// Server's implicit conversion, which is unambiguous for the ISO-8601
/// `T`-separated images `value.rs` produces.
fn sqlval(v: &fhir_mssql_map::shred::SqlVal) -> Bound {
    use fhir_mssql_map::shred::SqlVal;
    match v {
        SqlVal::Bool(b) => Bound::Bool(Some(*b)),
        SqlVal::Int(i) => Bound::I64(Some(*i)),
        SqlVal::Num(s) | SqlVal::Text(s) | SqlVal::Ts(s) | SqlVal::Date(s) | SqlVal::Jsonb(s) => {
            Bound::Str(Some(s.clone()))
        }
        SqlVal::Bytes(b) => Bound::Bytes(Some(b.clone())),
    }
}

/// `[a,b,c]`-joined ordinal path, the same text image every port writes
/// (`X15.5`).
fn fmt_ords(ords: &[i16]) -> String {
    ords.iter()
        .map(i16::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_ords(s: &str) -> Result<Vec<i16>, StoreError> {
    if s.is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|p| {
            p.parse()
                .map_err(|_| StoreError::Other(format!("malformed ords image: {s:?}")))
        })
        .collect()
}

fn leaf_from_cols(
    kind: &str,
    text: Option<String>,
    num: Option<String>,
    b: Option<bool>,
) -> fhir_mssql_map::value::LeafVal {
    use fhir_mssql_map::value::LeafVal;
    match kind {
        "s" => LeafVal::Str(text.unwrap_or_default()),
        "n" => LeafVal::Num(num.unwrap_or_default()),
        "b" => LeafVal::Bool(b.unwrap_or(false)),
        _ => LeafVal::Null,
    }
}

/// `[ords]` on `Ext`/`Deep` is `VARBINARY(255)` (`M14.9`): the same ASCII
/// text image every port writes (`"1,2,3"`), stored as raw bytes rather than
/// as `NVARCHAR` because it sits in the surrogate key alongside genuinely
/// unbounded columns and this engine cannot key an `NVARCHAR(MAX)`.
///
/// Reading it back as `&str` directly panics (`Row::get` on a `Binary`
/// column asking for a `String`); this decodes the raw bytes instead. Not
/// `cell_text`'s hex-render path, which is for an opaque digest — this is
/// text that merely happens to be stored as bytes, and hex-encoding it would
/// silently corrupt every `ords` path on read.
fn ords_bytes_to_str(row: &Row, idx: usize) -> Result<&str, StoreError> {
    let bytes: &[u8] = row.get(idx).unwrap_or_default();
    std::str::from_utf8(bytes)
        .map_err(|_| StoreError::Other(format!("column {idx}: ords image is not valid UTF-8")))
}

/// Render one stored column as the **text image** the shared reconstruction
/// engine expects, given its declared type.
///
/// The shared engine parses every column out of a string; the driver is
/// typed. `NUMBER`/`VARCHAR2`-analogues bind as `NVARCHAR`, so almost
/// everything here is already a string — this exists mainly for `BIT`
/// (`Bool`), where the JSON image (`"true"`/`"false"`) differs from the SQL
/// one.
fn cell_text(row: &Row, idx: usize, ty: ColTy, name: &str) -> Result<Option<String>, StoreError> {
    // `Row::get::<T,_>` panics on a type mismatch — its `try_get` returns
    // `Err`, and `get` is `try_get(..).unwrap()`. This store selects `[ords]`
    // (a `String` column) alongside genuine `DATE`/`DATETIME2` sort columns
    // in the same row, so blindly asking every cell for `&str` panicked on
    // the first date-bearing resource (`DateTime2(None) as a String`). Raw
    // access via `cells()` sidesteps `FromSql` entirely and cannot panic on a
    // type it was not told to expect — the same reasoning as the MySQL
    // store's `as_ref` instead of a typed `get`.
    let bad = || {
        StoreError::Other(format!(
            "column {name}: cannot render as a {ty:?} text image"
        ))
    };
    let data = row
        .cells()
        .nth(idx)
        .map(|(_, d)| d)
        .ok_or_else(|| StoreError::Other(format!("column {name}: index {idx} out of range")))?;
    Ok(match data {
        ColumnData::String(None)
        | ColumnData::I32(None)
        | ColumnData::I64(None)
        | ColumnData::Bit(None)
        | ColumnData::Binary(None)
        | ColumnData::Date(None)
        | ColumnData::DateTime2(None) => None,
        ColumnData::String(Some(s)) => Some(s.to_string()),
        ColumnData::Bit(Some(b)) => Some(if *b { "true" } else { "false" }.to_string()),
        ColumnData::I32(Some(i)) => Some(i.to_string()),
        ColumnData::I64(Some(i)) => Some(i.to_string()),
        ColumnData::Binary(Some(b)) => {
            Some(b.iter().map(|x| format!("{x:02x}")).collect::<String>())
        }
        // Derived sort columns (`_sort`): reconstruction ignores them, but
        // they are selected alongside everything else and still have to
        // render rather than panic. `chrono`'s `Display` for `NaiveDate` and
        // `NaiveDateTime` is already ISO-8601, matching what `value.rs`
        // wrote.
        ColumnData::Date(Some(_)) => row.get::<chrono::NaiveDate, _>(idx).map(|d| d.to_string()),
        ColumnData::DateTime2(Some(_)) => row
            .get::<chrono::NaiveDateTime, _>(idx)
            .map(|d| d.format("%Y-%m-%dT%H:%M:%S%.6f").to_string()),
        _ => return Err(bad()),
    })
}

/// Deterministic surrogate primary key for `Ext`/`Deep` rows (`M14.12`),
/// matching the MySQL/MariaDB ports' scheme byte-for-byte so the same fixture
/// data produces the same key on every engine that needs one.
#[must_use]
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
    let (y, mo, d) = civil_from_days(secs.div_euclid(86400));
    let s = secs.rem_euclid(86400);
    let (h, mi, se) = (s / 3600, (s % 3600) / 60, s % 60);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{se:02}.{micros:06}")
}

/// Howard Hinnant's `civil_from_days`, the same algorithm every port's
/// timestamp formatting uses.
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
