//! The MySQL store (spec section 14, T64).
//!
//! Built alongside the inherited PostgreSQL store rather than replacing it in
//! one edit, for the same reason the SQLite port did: converting every call site
//! at once means nothing compiles, and so nothing is testable, until the whole
//! file is done. The PostgreSQL module is deleted when this one reaches parity.
//!
//! # Differences from the PostgreSQL store that shape this module
//!
//! - **A schema is a database** (M14.21). `CREATE SCHEMA` survives as a synonym
//!   for `CREATE DATABASE`, and qualified names work as before — but identifiers
//!   are backquoted, since `"` is a string delimiter unless the server happens
//!   to run in `ANSI_QUOTES` mode.
//! - **Installation is not atomic** (M14.22). MySQL's DDL implicitly commits, so
//!   the staged-schema-then-rename dance has no equivalent and a failed install
//!   leaves a partial schema behind. This is a real regression from PostgreSQL
//!   and is surfaced, not hidden: `init` reports how far it got.
//! - **`Ext` and `Deep` carry a hash surrogate key** (M14.12). Their natural keys
//!   include unbounded text, which MySQL cannot key on, and a prefix index
//!   cannot enforce uniqueness over the full value. The store computes that hash
//!   here, in Rust — the same reasoning as the canonical form: a rule the
//!   database applies is a rule that can differ between engines.
//! - **The driver is async**, so unlike the SQLite store there is no
//!   `spawn_blocking` around every call.

use std::sync::Arc;

use fhir_mariadb_map::model::{ColTy, RelMap};
use mysql_async::prelude::*;
use mysql_async::{Opts, Pool};

use crate::{StoreError, UpgradeReport};

/// Render one stored cell as the **text image** the shared reconstruction
/// engine expects, given the column's declared type.
///
/// This exists because the driver is typed and the shared engine is not.
/// `reconstruct::prim_json` parses each column out of a `String`, and for a
/// boolean it accepts exactly `"true"`, `"t"`, `"false"`, or `"f"` — the image
/// PostgreSQL yields. MariaDB binds `Bool` to `TINYINT(1)`, `Int` to `INT`, and
/// the derived sort columns to `DATE`/`DATETIME(6)`, and reads them back over
/// the binary protocol as `Value::Int` and `Value::Date`.
///
/// The previous code asked for a `String` regardless:
///
/// ```ignore
/// if let Some(Some(v)) = row.get::<Option<String>, _>(i + off) {
/// ```
///
/// `mysql_common`'s `FromValue for String` accepts `Bytes` only, and
/// `Row::get` **panics** on a conversion failure rather than returning `None`.
/// So reading any resource carrying a boolean, an integer, or a date did not
/// lose the field — it panicked inside the store:
///
/// ```text
/// Could not retrieve `Option<String>`: Couldn't convert the value `Int(1)`
/// ```
///
/// Since almost every real `Patient` carries `active` or `birthDate`, that made
/// the port unusable for real data, and a panic in a library is a denial of
/// service for whatever hosts it (T11.9). Audit F-20.
///
/// `Ok(None)` means SQL NULL, a legitimately absent element. Anything this
/// cannot render is an error naming the column, never a silent omission (R4.3).
fn cell_text(v: &mysql_async::Value, ty: ColTy, name: &str) -> Result<Option<String>, StoreError> {
    use mysql_async::Value as V;
    let bad = || {
        StoreError::Other(format!(
            "column {name}: cannot render {v:?} as a {ty:?} text image"
        ))
    };
    Ok(match (ty, v) {
        (_, V::NULL) => None,

        // The one type whose SQL image differs from its JSON image.
        (ColTy::Bool, V::Int(i)) => Some(if *i == 0 { "false" } else { "true" }.to_string()),
        (ColTy::Bool, V::UInt(u)) => Some(if *u == 0 { "false" } else { "true" }.to_string()),
        (ColTy::Bool, V::Bytes(b)) => match std::str::from_utf8(b).map_err(|_| bad())? {
            s @ ("true" | "t" | "false" | "f") => Some(s.to_string()),
            "1" => Some("true".to_string()),
            "0" => Some("false".to_string()),
            _ => return Err(bad()),
        },

        (_, V::Int(i)) => Some(i.to_string()),
        (_, V::UInt(u)) => Some(u.to_string()),
        (_, V::Float(f)) => Some(f.to_string()),
        (_, V::Double(d)) => Some(d.to_string()),
        (_, V::Bytes(b)) => Some(std::str::from_utf8(b).map_err(|_| bad())?.to_string()),

        // Derived sort columns (`_sort`), which reconstruction ignores — but
        // they are selected with everything else, so they still have to render
        // rather than blow up. ISO-8601, matching what the shredder wrote.
        (_, V::Date(y, mo, d, h, mi, sec, us)) => Some(if *us == 0 {
            format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{sec:02}")
        } else {
            format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{sec:02}.{us:06}")
        }),
        (_, V::Time(neg, days, h, mi, sec, us)) => {
            let sign = if *neg { "-" } else { "" };
            Some(format!(
                "{sign}{}:{mi:02}:{sec:02}.{us:06}",
                days * 24 + u32::from(*h)
            ))
        }
    })
}

/// A history row's chain tip: `(version_id, sha256 link, sha3 link)`.
type ChainTip = (i64, Option<Vec<u8>>, Option<Vec<u8>>);

/// A pool bound to one relational map.
///
/// `Debug` is derived rather than hand-written and deliberately shallow: the
/// pool's own `Debug` would print connection options, and a DSN can carry a
/// password. A store that leaks credentials into a log line on the way to
/// reporting some unrelated error is a poor trade.
///
/// Unlike SQLite, MySQL is a real multi-writer server, so pooling is worth
/// having: concurrent writers make progress rather than serialising.
#[derive(Debug)]
pub struct MariaDbStore {
    pool: Pool,
    map: Arc<RelMap>,
    keys: crate::chain::KeyRing,
}

impl MariaDbStore {
    /// Connect using a URL such as `mysql://root@127.0.0.1:3306`.
    ///
    /// The schema (FHIR version) named by the map is *not* selected as the
    /// default database: every statement qualifies its tables, so leaving the
    /// connection unbound means a mistyped qualification fails loudly instead of
    /// silently hitting whatever database happened to be current.
    pub async fn connect(url: &str, map: Arc<RelMap>) -> Result<Self, StoreError> {
        let opts = Opts::from_url(url).map_err(|e| StoreError::Db(format!("bad DSN: {e}")))?;
        let pool = Pool::new(opts);
        // Fail here rather than at first use: a store that constructs
        // successfully and then cannot talk to anything is a worse diagnostic
        // than a connection error at startup.
        let mut conn = pool.get_conn().await.map_err(db_err)?;
        "SELECT 1".ignore(&mut conn).await.map_err(db_err)?;
        drop(conn);
        Ok(Self {
            pool,
            map,
            keys: crate::chain::KeyRing::default(),
        })
    }

    /// Attach a key ring for signing and verifying history.
    #[must_use]
    pub fn with_chain_keys(mut self, keys: crate::chain::KeyRing) -> Self {
        self.keys = keys;
        self
    }

    /// The map this store serves.
    #[must_use]
    pub fn map(&self) -> &Arc<RelMap> {
        &self.map
    }

    /// The schema (FHIR version) name this store's map describes.
    #[must_use]
    pub fn schema(&self) -> &str {
        &self.map.schema
    }

    /// The id of the signing key, if this store is keyed.
    #[must_use]
    pub fn chain_key_id(&self) -> Option<String> {
        self.keys.signing().map(|k| k.id().to_string())
    }

    /// Is the server reachable?
    pub async fn ping(&self) -> Result<(), StoreError> {
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        "SELECT 1".ignore(&mut conn).await.map_err(db_err)
    }

    /// Install the schema.
    ///
    /// Returns the number of statements applied. **Not atomic** — MySQL commits
    /// DDL implicitly, so a failure part-way leaves the statements that already
    /// ran (M14.22). The error names how many succeeded, because an operator
    /// cleaning up needs to know whether the database is empty or half-built;
    /// "install failed" alone leaves them guessing.
    pub async fn init(&self, checksum: &str) -> Result<usize, StoreError> {
        let statements = fhir_mariadb_map::ddl::ddl(&self.map);
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let mut applied = 0usize;
        for s in &statements {
            if let Err(e) = s.as_str().ignore(&mut conn).await {
                return Err(StoreError::Other(format!(
                    "installing schema: {e}\n\
                     {applied} of {} statement(s) had already been applied and remain \
                     — MySQL commits DDL implicitly, so this schema is partial (spec M14.22)\n\
                     statement was:\n{s}",
                    statements.len()
                )));
            }
            applied += 1;
        }
        let schema = quote_ident(&self.map.schema);
        // The map asset itself, gzipped and hex-coded, is what makes `upgrade`
        // possible: an upgrade diffs the installed map against the current one,
        // and a checksum says only *that* something changed, never *what*
        // (`M14.33`). Hex rather than base64 so the encoding matches every other
        // port's byte for byte.
        let asset_hex = hex_encode(
            &self
                .map
                .to_gz_bytes()
                .map_err(|e| StoreError::Other(e.to_string()))?,
        );
        conn.exec_drop(
            format!(
                "INSERT INTO {schema}.`fhir_mariadb_meta` (`key`, `value`) \
                 VALUES ('map_checksum', ?), ('fhir_version', ?), ('map_asset', ?)"
            ),
            (checksum, self.map.fhir_version.as_str(), &asset_hex),
        )
        .await
        .map_err(db_err)?;
        Ok(applied)
    }

    /// The checksum of the installed schema, or `None` if none is installed.
    pub async fn installed_checksum(&self) -> Result<Option<String>, StoreError> {
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let present: Option<i64> = conn
            .exec_first(
                "SELECT 1 FROM information_schema.tables \
                 WHERE table_schema = ? AND table_name = 'fhir_mariadb_meta'",
                (&self.map.schema,),
            )
            .await
            .map_err(db_err)?;
        if present.is_none() {
            return Ok(None);
        }
        let schema = quote_ident(&self.map.schema);
        conn.query_first(format!(
            "SELECT `value` FROM {schema}.`fhir_mariadb_meta` WHERE `key` = 'map_checksum'"
        ))
        .await
        .map_err(db_err)
    }

    /// Upgrade an installed schema to this store's map: additive changes (new
    /// tables, columns, indexes) apply automatically; destructive ones require
    /// `allow_destructive`. Column type changes always refuse (`O10.4a`, `L12`).
    ///
    /// Closes this port's share of audit **F-15**. Two differences from
    /// `fhir-sqlite`'s, both mariadb's doing:
    ///
    /// 1. **There is no transaction.** MariaDB commits DDL implicitly, so an
    ///    upgrade that fails partway leaves a schema that is neither the old one
    ///    nor the new one. That cannot be prevented here, so it is *reported*:
    ///    the error names how many statements had already been applied and that
    ///    they remain (`M14.22`, `M14.35`). SQLite's "it either lands or it does
    ///    not" is not available.
    /// 2. **The access-log indexes are not idempotent.** MariaDB has no
    ///    `CREATE INDEX IF NOT EXISTS`, so reconciling `schema_wide_objects`
    ///    wholesale fails on the second run with `Duplicate key name`. They are
    ///    filtered against `information_schema.statistics` (**F-28**).
    ///
    /// The audit envelope is likewise filtered, against
    /// `information_schema.columns`, exactly as `ddl::history_audit_columns`
    /// tells its caller to do.
    pub async fn upgrade(
        &self,
        checksum: &str,
        allow_destructive: bool,
    ) -> Result<UpgradeReport, StoreError> {
        let s = self.map.schema.clone();
        // Two forms, deliberately. `ddl::` functions add their own backticks, so
        // they take the raw name; hand-written SQL below takes the quoted one.
        // Passing the quoted name to `ddl::` yields ``` ``schema`` ``` and a
        // syntax error, which is how this was caught against a live server.
        let esc = quote_ident(&s);
        let raw = s.as_str();
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;

        // Distinguish "never installed" from "installed before the asset was
        // recorded": the remedies differ, `init` versus a reload.
        let meta_exists: Option<i64> = conn
            .exec_first(
                "SELECT COUNT(*) FROM information_schema.tables \
                 WHERE table_schema = ? AND table_name = ?",
                (&s, "fhir_mariadb_meta"),
            )
            .await
            .map_err(db_err)?;
        if meta_exists.unwrap_or(0) == 0 {
            return Err(StoreError::Other(format!("schema {s} is not installed")));
        }
        let old_hex: String = conn
            .exec_first(
                format!("SELECT `value` FROM {esc}.`fhir_mariadb_meta` WHERE `key` = 'map_asset'"),
                (),
            )
            .await
            .map_err(db_err)?
            .ok_or_else(|| {
                StoreError::Other(
                    "installed schema predates upgrade support (no stored map asset); \
                     reinstall with `init` to make later upgrades possible"
                        .into(),
                )
            })?;
        let old_map = RelMap::from_gz_bytes(&hex_decode(&old_hex)?)
            .map_err(|e| StoreError::Other(format!("stored map asset unreadable: {e}")))?;

        let (adds, destructive) = self.diff_maps(&old_map, raw, &esc)?;
        if !destructive.is_empty() && !allow_destructive {
            return Err(StoreError::Other(format!(
                "upgrade requires {} destructive change(s); rerun with --allow-destructive \
                 (first: {})",
                destructive.len(),
                destructive.first().expect("non-empty")
            )));
        }

        let (n_add, n_drop) = (adds.len(), destructive.len());
        let mut applied = 0usize;

        // Adds first: a resource type new in this artifact has no tables until
        // `adds` creates them, and reconciliation would otherwise alter a table
        // that does not exist.
        //
        // The reconcile set is then computed **against the database as the adds
        // left it**, not as it was before. A history table created a moment ago
        // by `create_table` already carries the audit envelope, so building the
        // filter beforehand would emit an `ADD COLUMN` for every envelope column
        // it was about to gain and fail with `Duplicate column name 'actor'` —
        // which is exactly what a live server said when this was written the
        // other way round.
        let apply = async |stmts: &[String], applied: &mut usize, conn: &mut mysql_async::Conn| {
            for stmt in stmts {
                if let Err(e) = stmt.as_str().ignore(&mut *conn).await {
                    return Err(StoreError::Other(format!(
                        "upgrade: {e}\n\
                         {applied} statement(s) had already been applied and remain \
                         — MariaDB commits DDL implicitly, so this schema is partial \
                         (spec M14.22, M14.35)\n\
                         statement was:\n{stmt}"
                    )));
                }
                *applied += 1;
            }
            Ok(())
        };
        apply(&adds, &mut applied, &mut conn).await?;

        // Reconciliation: objects the per-resource diff cannot see. The two
        // tables carry IF NOT EXISTS; the indexes do not, so they are filtered.
        let have_ix = self.installed_indexes(&mut conn, &s).await?;
        let mut reconcile: Vec<String> = fhir_mariadb_map::ddl::schema_wide_objects(raw)
            .into_iter()
            .filter(|stmt| match index_name_of(stmt) {
                Some(name) => !have_ix.contains(&name),
                None => true,
            })
            .collect();
        for rm in self.map.resources.values() {
            if let Some((_, hist)) = rm.find_table(fhir_mariadb_map::model::TableKind::History) {
                let have = self.installed_columns(&mut conn, &s, &hist.name).await?;
                reconcile.extend(
                    fhir_mariadb_map::ddl::history_audit_columns(raw, &hist.name)
                        .into_iter()
                        .filter(|stmt| match added_column_name(stmt) {
                            Some(c) => !have.contains(&c),
                            None => true,
                        }),
                );
                // DROP … IF EXISTS then CREATE, so genuinely idempotent.
                reconcile.extend(fhir_mariadb_map::ddl::append_only_triggers(raw, &hist.name));
            }
        }
        apply(&reconcile, &mut applied, &mut conn).await?;
        apply(&destructive, &mut applied, &mut conn).await?;

        let new_hex = hex_encode(
            &self
                .map
                .to_gz_bytes()
                .map_err(|e| StoreError::Other(e.to_string()))?,
        );
        for (k, v) in [
            ("map_checksum", checksum),
            ("fhir_version", self.map.fhir_version.as_str()),
            ("map_asset", new_hex.as_str()),
        ] {
            conn.exec_drop(
                format!("UPDATE {esc}.`fhir_mariadb_meta` SET `value` = ? WHERE `key` = ?"),
                (v, k),
            )
            .await
            .map_err(db_err)?;
        }
        drop(conn);

        let folded = self.backfill_norm().await?;
        Ok(UpgradeReport {
            additive: n_add,
            destructive: n_drop,
            folded,
        })
    }

    /// Index names already present in this schema, so the non-idempotent
    /// `CREATE INDEX` statements can be skipped rather than fail (**F-28**).
    async fn installed_indexes(
        &self,
        conn: &mut mysql_async::Conn,
        schema: &str,
    ) -> Result<std::collections::HashSet<String>, StoreError> {
        let rows: Vec<String> = conn
            .exec(
                "SELECT DISTINCT index_name FROM information_schema.statistics \
                 WHERE table_schema = ?",
                (schema,),
            )
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().collect())
    }

    /// The columns a table actually has, for the audit-envelope diff.
    async fn installed_columns(
        &self,
        conn: &mut mysql_async::Conn,
        schema: &str,
        table: &str,
    ) -> Result<std::collections::HashSet<String>, StoreError> {
        let rows: Vec<String> = conn
            .exec(
                "SELECT column_name FROM information_schema.columns \
                 WHERE table_schema = ? AND table_name = ?",
                (schema, table),
            )
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().collect())
    }

    /// Diff the installed map against this store's, by name, across all
    /// resources. A column whose *type* changed is neither additive nor
    /// destructive: it is an error, because a type change means the shred writes
    /// a different value shape and rewriting stored data is a migration somebody
    /// must design (`L12`).
    fn diff_maps(
        &self,
        old_map: &RelMap,
        raw: &str,
        esc: &str,
    ) -> Result<(Vec<String>, Vec<String>), StoreError> {
        use std::collections::{HashMap, HashSet};
        let (mut adds, mut destructive) = (Vec::new(), Vec::new());
        let mut old_tables: HashMap<&str, &fhir_mariadb_map::model::Table> = HashMap::new();
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
                    adds.push(fhir_mariadb_map::ddl::create_table(raw, rm, t));
                    continue;
                };
                let old_cols: HashMap<&str, ColTy> =
                    old_t.cols.iter().map(|c| (c.name.as_str(), c.ty)).collect();
                let new_cols: HashSet<&str> = t.cols.iter().map(|c| c.name.as_str()).collect();
                for c in &t.cols {
                    match old_cols.get(c.name.as_str()) {
                        None => adds.push(format!(
                            "ALTER TABLE {esc}.`{}` ADD COLUMN `{}` {}",
                            t.name,
                            c.name,
                            fhir_mariadb_map::ddl::col_sql(c.ty)
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
                            "ALTER TABLE {esc}.`{}` DROP COLUMN `{name}`",
                            t.name
                        ));
                    }
                }
            }
        }
        for name in old_tables.keys() {
            if !new_names.contains(name) {
                destructive.push(format!("DROP TABLE {esc}.`{name}`"));
            }
        }
        let old_ix: HashSet<String> = old_map
            .resources
            .values()
            .flat_map(|rm| fhir_mariadb_map::ddl::search_indexes(raw, rm))
            .collect();
        for rm in self.map.resources.values() {
            for stmt in fhir_mariadb_map::ddl::search_indexes(raw, rm) {
                if !old_ix.contains(&stmt) {
                    adds.push(stmt);
                }
            }
        }
        Ok((adds, destructive))
    }

    /// Populate folded search columns (P6.6) for rows written before the column
    /// existed, returning how many distinct values were folded.
    ///
    /// An upgrade that added the column would otherwise leave it NULL on every
    /// existing row, and every non-`:exact` string search compares that column —
    /// so those resources would silently stop matching their own values. **Silent
    /// under-return is the one failure mode a clinical search must not have**,
    /// which is why this runs as part of the upgrade rather than as a step an
    /// operator can forget (`L13`, `L14`).
    ///
    /// Folds distinct *values* rather than rows — a surname repeats across
    /// patients — in bounded batches, and is **resumable**: each pass looks only
    /// at rows still NULL, so an interrupted run resumes where it stopped.
    pub async fn backfill_norm(&self) -> Result<usize, StoreError> {
        const BATCH: usize = 1000;
        let esc = quote_ident(&self.map.schema);
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
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let mut total = 0usize;
        for (tn, src, dst) in &work {
            loop {
                let vals: Vec<String> = conn
                    .exec(
                        format!(
                            "SELECT DISTINCT `{src}` FROM {esc}.`{tn}` \
                             WHERE `{dst}` IS NULL AND `{src}` IS NOT NULL LIMIT {BATCH}"
                        ),
                        (),
                    )
                    .await
                    .map_err(db_err)?;
                if vals.is_empty() {
                    break;
                }
                let n = vals.len();
                let sql = format!(
                    "UPDATE {esc}.`{tn}` SET `{dst}` = ? WHERE `{src}` = ? AND `{dst}` IS NULL"
                );
                for v in &vals {
                    conn.exec_drop(&sql, (fhir_mariadb_map::fold::fold(v), v))
                        .await
                        .map_err(db_err)?;
                }
                total += n;
                if n < BATCH {
                    break;
                }
            }
        }
        Ok(total)
    }

    /// How many tables the installed schema has, for tests and diagnostics.
    pub async fn table_count(&self) -> Result<usize, StoreError> {
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let n: Option<i64> = conn
            .exec_first(
                "SELECT count(*) FROM information_schema.tables WHERE table_schema = ?",
                (&self.map.schema,),
            )
            .await
            .map_err(db_err)?;
        Ok(usize::try_from(n.unwrap_or(0)).unwrap_or(0))
    }

    /// How many append-only triggers the installed schema has.
    ///
    /// Worth being able to count separately from tables: the triggers are the
    /// enforcement behind M3.17, and a schema that installed its tables but not
    /// its triggers would look healthy while guaranteeing nothing.
    pub async fn trigger_count(&self) -> Result<usize, StoreError> {
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let n: Option<i64> = conn
            .exec_first(
                "SELECT count(*) FROM information_schema.triggers WHERE trigger_schema = ?",
                (&self.map.schema,),
            )
            .await
            .map_err(db_err)?;
        Ok(usize::try_from(n.unwrap_or(0)).unwrap_or(0))
    }

    /// Drop the schema and everything in it.
    pub async fn drop_schema(&self) -> Result<(), StoreError> {
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        format!("DROP SCHEMA IF EXISTS {}", quote_ident(&self.map.schema))
            .ignore(&mut conn)
            .await
            .map_err(db_err)
    }

    /// Run a statement directly.
    ///
    /// For tests and operator tooling only: nothing in the store's own paths
    /// interpolates SQL like this, and the fuzz invariant that attacker values
    /// reach the database as parameters applies to those paths, not to a
    /// deliberate escape hatch.
    pub async fn exec_raw(&self, sql: &str) -> Result<(), StoreError> {
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        sql.ignore(&mut conn).await.map_err(db_err)
    }

    /// Close the pool. Not required — dropping the store is enough — but useful
    /// in tests, which otherwise race the pool's background cleanup.
    pub async fn close(self) -> Result<(), StoreError> {
        self.pool.disconnect().await.map_err(db_err)
    }
}

/// The surrogate primary key for an `Ext` or `Deep` row (M14.12).
///
/// Those tables' natural keys include unbounded text, which MySQL cannot key on,
/// and a prefix index is not a substitute: it cannot enforce uniqueness over the
/// full value, so two rows differing only past the prefix would collide and
/// silently lose data.
///
/// The components are joined with a delimiter that cannot occur in any of them,
/// rather than concatenated. Concatenation would make `("ab", "c")` and
/// `("a", "bc")` hash identically — a collision that would look exactly like
/// data loss and be very hard to trace back to here.
#[must_use]
pub fn surrogate_key(parts: &[&str]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    for p in parts {
        h.update(p.as_bytes());
        h.update([0x00]);
    }
    h.finalize().into()
}

/// Backquote an identifier, doubling any embedded backquote.
///
/// Schema and table names come from the generator rather than from a request,
/// so this is belt-and-braces — but an interpolated identifier is exactly the
/// shape that becomes an injection the day someone makes it configurable.
fn quote_ident(s: &str) -> String {
    format!("`{}`", s.replace('`', "``"))
}

fn db_err(e: mysql_async::Error) -> StoreError {
    StoreError::Db(e.to_string())
}

/// The stored map asset is gzip and the meta table's `value` is text, so it
/// travels as hex — the same encoding every other port uses, so an asset can be
/// lifted from one port's meta table into another's.
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

/// The index name in a `CREATE INDEX \`name\` ON …` statement, so an upgrade can
/// skip one that already exists (**F-28**).
///
/// `None` means the statement was not that shape, and the caller then applies it
/// rather than skipping — a redundant statement fails loudly, a wrongly skipped
/// one loses an index silently.
fn index_name_of(stmt: &str) -> Option<String> {
    let rest = stmt.strip_prefix("CREATE INDEX `")?;
    rest.split_once('`').map(|(name, _)| name.to_string())
}

/// The column name in an `ALTER TABLE … ADD COLUMN \`name\` …` statement.
fn added_column_name(stmt: &str) -> Option<String> {
    let rest = stmt.split_once(" ADD COLUMN `")?.1;
    rest.split_once('`').map(|(name, _)| name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_quoted_and_escapes_are_doubled() {
        assert_eq!(quote_ident("r5"), "`r5`");
        assert_eq!(quote_ident("we`ird"), "`we``ird`");
    }

    #[test]
    fn surrogate_key_is_delimited_not_concatenated() {
        // Without a delimiter these two would hash identically, and the
        // collision would present as silent data loss.
        assert_ne!(surrogate_key(&["ab", "c"]), surrogate_key(&["a", "bc"]));
        // Same inputs, same key — it is a primary key, so it must be stable.
        assert_eq!(surrogate_key(&["a", "b"]), surrogate_key(&["a", "b"]));
        // An empty component is distinct from an absent one.
        assert_ne!(surrogate_key(&["a", ""]), surrogate_key(&["a"]));
    }
}

// ---------------------------------------------------------------- write & read

/// What a `put` did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutKind {
    Created,
    Updated,
}

/// Outcome of a `put`.
#[derive(Debug, Clone)]
pub struct Put {
    pub id: String,
    pub version_id: i64,
    pub kind: PutKind,
}

/// The `ords` image: `{1,2}`, `{}`, `{-1,3}`.
///
/// The same text PostgreSQL wrote for its `smallint[]` column. The database
/// never orders, subscripts, or unnests this — it stores it and enforces
/// uniqueness — which is why a bounded `VARBINARY` is sufficient (M14.9).
pub(crate) fn fmt_ords(ords: &[i16]) -> String {
    let inner: Vec<String> = ords.iter().map(ToString::to_string).collect();
    format!("{{{}}}", inner.join(","))
}

pub(crate) fn parse_ords(s: &str) -> Result<Vec<i16>, StoreError> {
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

/// `YYYY-MM-DD HH:MM:SS.ffffff`, UTC — MySQL's `DATETIME(6)` literal form.
///
/// Rendered here rather than by the server: `NOW()` follows the session time
/// zone, so a verifier connecting from elsewhere would recompute different bytes
/// and report every history row as broken.
fn utc_micros(t: std::time::SystemTime) -> String {
    let d = t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let (secs, micros) = (d.as_secs() as i64, d.subsec_micros());
    let (days, sod) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    let (y, m, dd) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{dd:02} {:02}:{:02}:{:02}.{micros:06}",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    )
}

/// Howard Hinnant's `civil_from_days`.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn sqlval(v: &fhir_mariadb_map::shred::SqlVal) -> mysql_async::Value {
    use fhir_mariadb_map::shred::SqlVal as S;
    use mysql_async::Value as V;
    match v {
        S::Bool(b) => V::Int(i64::from(*b)),
        S::Int(i) => V::Int(*i),
        // Numeric, date, timestamp, and JSON all cross as text: the columns are
        // TEXT (M14.14) and the lexical form is what M3.6 requires to survive.
        S::Num(s) | S::Text(s) | S::Ts(s) | S::Date(s) | S::Jsonb(s) => V::Bytes(s.clone().into()),
    }
}

fn opt_text(s: Option<&str>) -> mysql_async::Value {
    s.map_or(mysql_async::Value::NULL, |x| {
        mysql_async::Value::Bytes(x.to_string().into())
    })
}

impl MariaDbStore {
    /// Store a resource, appending a history row with its hash chain.
    ///
    /// One transaction covers the whole operation. A rewrite deletes the base
    /// row and lets `ON DELETE CASCADE` clear the children; history has no
    /// foreign key to the base table, precisely so a deletion cannot erase its
    /// own evidence.
    pub async fn put(
        &self,
        resource: &serde_json::Value,
        audit: &crate::Audit,
    ) -> Result<Put, StoreError> {
        use mysql_async::Value as V;

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
        // `?` rather than flattening to `Other`: `StoreError::Shred` exists so a
        // caller can tell a rejected *resource* from a failed *store*. Rendering
        // it as a string erases that, and every bad submission then looks like an
        // internal fault — a 500 where the truth is a 400 (audit F-23).
        let out = fhir_mariadb_map::shred::shred(rm, resource)?;
        let id = out
            .id
            .clone()
            .ok_or_else(|| StoreError::Other("resource has no id".into()))?;

        // The bytes the chain commits to, computed here rather than by the
        // database (M14.19/M14.20): no two engines render JSON alike, so a chain
        // written by one could never be verified by another.
        let canon = fhir_mariadb_map::canon::canonicalize(resource);
        let ts = utc_micros(std::time::SystemTime::now());
        let s = quote_ident(&self.map.schema);
        let base = quote_ident(&rm.base_table().name);
        let hist = quote_ident(
            &rm.find_table(fhir_mariadb_map::model::TableKind::History)
                .map(|(_, t)| t.name.clone())
                .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?,
        );

        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let mut tx = conn
            .start_transaction(mysql_async::TxOpts::default())
            .await
            .map_err(db_err)?;

        // H5.4: serialise writers for this resource id before reading the chain
        // tip.
        //
        // Under REPEATABLE READ the tip read below is a *consistent* read: it
        // takes no lock, so N concurrent writers all see the same tip, all
        // compute the same next version, and all try to insert it. Correctness
        // survives — the history primary key rejects the duplicates — but only
        // one writer makes progress and the other N-1 get a raw
        // `Duplicate entry … for key 'patient_history.PRIMARY'`, which a caller
        // cannot tell from a genuine conflict or a bug. Measured before this
        // lock existed: **1 of 8 concurrent writers succeeded** (audit F-24).
        //
        // `FOR UPDATE` on the base row makes them queue instead, which is what
        // the PostgreSQL store does and what H5.4 requires. A create has no base
        // row to lock, so racing creates of the same id still resolve on the
        // primary key — the same backstop, and the same behaviour as PostgreSQL.
        let _lock: Option<i64> = tx
            .exec_first(
                format!("SELECT `version_id` FROM {s}.{base} WHERE `id` = ? FOR UPDATE"),
                (&id,),
            )
            .await
            .map_err(db_err)?;

        let prev: Option<ChainTip> = tx
            .exec_first(
                format!(
                    "SELECT `version_id`, `row_hash`, `row_hash_sha3` FROM {s}.{hist} \
                     WHERE `id` = ? ORDER BY `version_id` DESC LIMIT 1"
                ),
                (&id,),
            )
            .await
            .map_err(db_err)?;
        let (version_id, prev_256, prev_3) = match prev {
            Some((v, a, b)) => (v + 1, a, b),
            None => (1, None, None),
        };

        let existed: Option<i64> = tx
            .exec_first(format!("SELECT 1 FROM {s}.{base} WHERE `id` = ?"), (&id,))
            .await
            .map_err(db_err)?;
        let existed = existed.is_some();
        if existed {
            tx.exec_drop(format!("DELETE FROM {s}.{base} WHERE `id` = ?"), (&id,))
                .await
                .map_err(db_err)?;
        }

        // Base row first: every child has a foreign key to it.
        let mut cols = vec![
            "`id`".to_string(),
            "`version_id`".to_string(),
            "`last_updated`".to_string(),
        ];
        let mut vals: Vec<V> = vec![
            V::Bytes(id.clone().into()),
            V::Int(version_id),
            V::Bytes(ts.clone().into()),
        ];
        for r in out.rows.iter().filter(|r| r.table == 0) {
            for (name, v) in &r.cols {
                cols.push(format!("`{name}`"));
                vals.push(sqlval(v));
            }
        }
        insert_rows(&mut tx, &s, &base, &cols, &[vals]).await?;

        // Element tables, grouped so each is one multi-row insert.
        let mut by_table: std::collections::BTreeMap<u32, Vec<&fhir_mariadb_map::shred::Row>> =
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
            let mut cols = vec!["`rid`".to_string(), "`ords`".to_string()];
            cols.extend(names.iter().map(|n| format!("`{n}`")));
            let batch: Vec<Vec<V>> = rows
                .iter()
                .map(|r| {
                    let mut row = vec![
                        V::Bytes(id.clone().into()),
                        V::Bytes(fmt_ords(&r.ords).into()),
                    ];
                    for n in &names {
                        row.push(
                            r.cols
                                .iter()
                                .find(|(c, _)| c == n)
                                .map_or(V::NULL, |(_, v)| sqlval(v)),
                        );
                    }
                    row
                })
                .collect();
            insert_rows(&mut tx, &s, &quote_ident(&t.name), &cols, &batch).await?;
        }

        // Extensions and spill carry the surrogate primary key (M14.12): their
        // natural keys hold unbounded text, which MySQL cannot key on.
        if let Some((_, t)) = rm.find_table(fhir_mariadb_map::model::TableKind::Ext)
            && !out.ext.is_empty()
        {
            let cols: Vec<String> = [
                "key_hash", "rid", "path", "ords", "modifier", "ext_ord", "url", "leaf", "v_kind",
                "v_text", "v_num", "v_bool",
            ]
            .iter()
            .map(|c| format!("`{c}`"))
            .collect();
            let batch: Vec<Vec<V>> = out
                .ext
                .iter()
                .map(|e| {
                    let (kind, text, num, b) = e.val.cols();
                    let ords = fmt_ords(&e.ords);
                    let ext_ord = e.ext_ord.to_string();
                    let modifier = u8::from(e.modifier).to_string();
                    let key = surrogate_key(&[&id, &e.path, &ords, &modifier, &ext_ord, &e.leaf]);
                    vec![
                        V::Bytes(key.to_vec()),
                        V::Bytes(id.clone().into()),
                        V::Bytes(e.path.clone().into()),
                        V::Bytes(ords.into()),
                        V::Int(i64::from(e.modifier)),
                        V::Int(i64::from(e.ext_ord)),
                        opt_text(e.url.as_deref()),
                        V::Bytes(e.leaf.clone().into()),
                        V::Bytes(kind.to_string().into()),
                        opt_text(text),
                        opt_text(num),
                        b.map_or(V::NULL, |x| V::Int(i64::from(x))),
                    ]
                })
                .collect();
            insert_rows(&mut tx, &s, &quote_ident(&t.name), &cols, &batch).await?;
        }

        if let Some((_, t)) = rm.find_table(fhir_mariadb_map::model::TableKind::Deep)
            && !out.deep.is_empty()
        {
            let cols: Vec<String> = [
                "key_hash", "rid", "path", "ords", "leaf", "v_kind", "v_text", "v_num", "v_bool",
            ]
            .iter()
            .map(|c| format!("`{c}`"))
            .collect();
            let batch: Vec<Vec<V>> = out
                .deep
                .iter()
                .map(|d| {
                    let (kind, text, num, b) = d.val.cols();
                    let ords = fmt_ords(&d.ords);
                    let key = surrogate_key(&[&id, &d.path, &ords, &d.leaf]);
                    vec![
                        V::Bytes(key.to_vec()),
                        V::Bytes(id.clone().into()),
                        V::Bytes(d.path.clone().into()),
                        V::Bytes(ords.into()),
                        V::Bytes(d.leaf.clone().into()),
                        V::Bytes(kind.to_string().into()),
                        opt_text(text),
                        opt_text(num),
                        b.map_or(V::NULL, |x| V::Int(i64::from(x))),
                    ]
                })
                .collect();
            insert_rows(&mut tx, &s, &quote_ident(&t.name), &cols, &batch).await?;
        }

        if let Some((_, t)) = rm.find_table(fhir_mariadb_map::model::TableKind::Contained)
            && !out.contained.is_empty()
        {
            let cols = ["`rid`", "`ord`", "`resource`"].map(String::from).to_vec();
            let batch: Vec<Vec<V>> = out
                .contained
                .iter()
                .map(|(ord, v)| {
                    vec![
                        V::Bytes(id.clone().into()),
                        V::Int(i64::from(*ord)),
                        V::Bytes(v.to_string().into()),
                    ]
                })
                .collect();
            insert_rows(&mut tx, &s, &quote_ident(&t.name), &cols, &batch).await?;
        }

        // 'C' and 'U' are distinct in the op column, and the op is part of the
        // hashed preimage, so it cannot be corrected later.
        let op = if existed { "U" } else { "C" };
        let pre = crate::chain::preimage(&id, version_id, &ts, op, Some(&canon), &audit.actor);
        let (row_hash, row_sha3) = crate::chain::link(prev_256.as_deref(), prev_3.as_deref(), &pre);
        let row_mac = self
            .keys
            .signing()
            .map(|k| crate::chain::mac(k, prev_256.as_deref(), &pre));

        tx.exec_drop(
            format!(
                "INSERT INTO {s}.{hist} \
                   (`id`, `version_id`, `last_updated`, `op`, `resource`, `actor`, \
                    `actor_source`, `client`, `request_id`, `reason`, `prev_hash`, \
                    `row_hash`, `prev_hash_sha3`, `row_hash_sha3`, `row_mac`) \
                 VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"
            ),
            // Positional rather than a tuple: `Params: From<(..)>` stops at
            // twelve elements and this row has fifteen.
            mysql_async::Params::Positional(vec![
                V::Bytes(id.clone().into()),
                V::Int(version_id),
                V::Bytes(ts.clone().into()),
                V::Bytes(op.into()),
                // The canonical bytes are stored, so what is read back is
                // exactly what was signed.
                V::Bytes(canon.clone().into()),
                V::Bytes(audit.actor.clone().into()),
                opt_text(audit.actor_source.as_deref()),
                opt_text(audit.client.as_deref()),
                opt_text(audit.request_id.as_deref()),
                opt_text(audit.reason.as_deref()),
                prev_256.clone().map_or(V::NULL, V::Bytes),
                V::Bytes(row_hash.clone()),
                prev_3.clone().map_or(V::NULL, V::Bytes),
                V::Bytes(row_sha3.clone()),
                row_mac.map_or(V::NULL, |m| V::Bytes(m.into())),
            ]),
        )
        .await
        .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(Put {
            id,
            version_id,
            kind: if existed {
                PutKind::Updated
            } else {
                PutKind::Created
            },
        })
    }

    /// Read a resource back, reconstructed from its rows.
    pub async fn get(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Option<serde_json::Value>, StoreError> {
        use fhir_mariadb_map::model::TableKind;
        use fhir_mariadb_map::reconstruct::{InRow, ReconIn};

        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let s = quote_ident(&self.map.schema);
        let base = quote_ident(&rm.base_table().name);
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;

        // R4.5: one snapshot for the whole reconstruction.
        //
        // A read touches the base table and every child table as separate
        // statements. Outside a transaction each gets its own snapshot, so a
        // writer committing between them yields a resource that never existed.
        // REPEATABLE READ is the server default, but that is a property of a
        // *transaction*, and this code had none — a reader observed `name` from
        // one version beside `telecom` from the next (audit F-21).
        //
        // Read-only, so it is rolled back rather than committed at each exit.
        let mut tx = conn
            .start_transaction(mysql_async::TxOpts::default())
            .await
            .map_err(db_err)?;

        let present: Option<i64> = tx
            .exec_first(format!("SELECT 1 FROM {s}.{base} WHERE `id` = ?"), (id,))
            .await
            .map_err(db_err)?;
        if present.is_none() {
            tx.rollback().await.map_err(db_err)?;
            return Ok(None);
        }

        let mut input = ReconIn {
            tables: vec![Vec::new(); rm.tables.len()],
            ..Default::default()
        };

        for (ti, t) in rm.tables.iter().enumerate() {
            let table = quote_ident(&t.name);
            match t.kind {
                TableKind::Base | TableKind::Elem => {
                    // Carry each column's declared type, not just its name:
                    // rendering a cell back to its text image needs it (see
                    // `cell_text`).
                    let names: Vec<(String, ColTy)> =
                        t.cols.iter().map(|c| (c.name.clone(), c.ty)).collect();
                    let key = if t.kind == TableKind::Base {
                        "id"
                    } else {
                        "rid"
                    };
                    let mut sel: Vec<String> = Vec::new();
                    if t.kind == TableKind::Elem {
                        sel.push("`ords`".to_string());
                    }
                    sel.extend(names.iter().map(|(n, _)| format!("`{n}`")));
                    if sel.is_empty() {
                        sel.push("NULL".to_string());
                    }
                    let rows: Vec<mysql_async::Row> = tx
                        .exec(
                            format!(
                                "SELECT {} FROM {s}.{table} WHERE `{key}` = ?",
                                sel.join(", ")
                            ),
                            (id,),
                        )
                        .await
                        .map_err(db_err)?;
                    for row in rows {
                        let mut ords = Vec::new();
                        let mut off = 0usize;
                        if t.kind == TableKind::Elem {
                            let img: String = row.get(0).unwrap_or_default();
                            ords = parse_ords(&img)?;
                            off = 1;
                        }
                        let mut cols = std::collections::HashMap::new();
                        for (i, (n, ty)) in names.iter().enumerate() {
                            // `as_ref` on the raw Value, never `get::<String>`:
                            // that panics on a typed column (F-20).
                            let Some(raw) = row.as_ref(i + off) else {
                                continue;
                            };
                            if let Some(v) = cell_text(raw, *ty, n)? {
                                cols.insert(n.clone(), v);
                            }
                        }
                        input.tables[ti].push(InRow { ords, cols });
                    }
                }
                TableKind::Ext => {
                    let rows: Vec<mysql_async::Row> = tx
                        .exec(
                            format!(
                                "SELECT `path`,`ords`,`modifier`,`ext_ord`,`url`,`leaf`,\
                                        `v_kind`,`v_text`,`v_num`,`v_bool` \
                                 FROM {s}.{table} WHERE `rid` = ?"
                            ),
                            (id,),
                        )
                        .await
                        .map_err(db_err)?;
                    for r in rows {
                        input.ext.push(fhir_mariadb_map::shred::ExtRow {
                            path: r.get(0).unwrap_or_default(),
                            ords: parse_ords(&r.get::<String, _>(1).unwrap_or_default())?,
                            modifier: r.get::<i64, _>(2).unwrap_or(0) != 0,
                            ext_ord: r.get(3).unwrap_or(0),
                            url: r.get(4).unwrap_or(None),
                            leaf: r.get(5).unwrap_or_default(),
                            val: leaf_from_cols(
                                &r.get::<String, _>(6).unwrap_or_default(),
                                r.get(7).unwrap_or(None),
                                r.get(8).unwrap_or(None),
                                r.get(9).unwrap_or(None),
                            ),
                        });
                    }
                }
                TableKind::Deep => {
                    let rows: Vec<mysql_async::Row> = tx
                        .exec(
                            format!(
                                "SELECT `path`,`ords`,`leaf`,`v_kind`,`v_text`,`v_num`,`v_bool` \
                                 FROM {s}.{table} WHERE `rid` = ?"
                            ),
                            (id,),
                        )
                        .await
                        .map_err(db_err)?;
                    for r in rows {
                        input.deep.push(fhir_mariadb_map::shred::DeepRow {
                            path: r.get(0).unwrap_or_default(),
                            ords: parse_ords(&r.get::<String, _>(1).unwrap_or_default())?,
                            leaf: r.get(2).unwrap_or_default(),
                            val: leaf_from_cols(
                                &r.get::<String, _>(3).unwrap_or_default(),
                                r.get(4).unwrap_or(None),
                                r.get(5).unwrap_or(None),
                                r.get(6).unwrap_or(None),
                            ),
                        });
                    }
                }
                TableKind::Contained => {
                    let rows: Vec<mysql_async::Row> = tx
                        .exec(
                            format!("SELECT `ord`,`resource` FROM {s}.{table} WHERE `rid` = ?"),
                            (id,),
                        )
                        .await
                        .map_err(db_err)?;
                    for r in rows {
                        let raw: String = r.get(1).unwrap_or_default();
                        let v = serde_json::from_str(&raw)
                            .map_err(|e| StoreError::Other(format!("contained: {e}")))?;
                        input.contained.push((r.get(0).unwrap_or(0), v));
                    }
                }
                TableKind::History => {}
            }
        }

        tx.rollback().await.map_err(db_err)?;

        // `?` rather than flattening to `Other`: reconstruction audits row
        // consumption and reports a residue as an integrity error (R4.7). That
        // is the signal saying stored data went unread — exactly what F-20 was
        // doing — and an untyped string makes it look like an I/O hiccup
        // (audit F-23).
        let v = fhir_mariadb_map::reconstruct::reconstruct(rm, &input, Some(id))?;
        Ok(Some(v))
    }
}

fn leaf_from_cols(
    kind: &str,
    text: Option<String>,
    num: Option<String>,
    b: Option<i64>,
) -> fhir_mariadb_map::value::LeafVal {
    use fhir_mariadb_map::value::LeafVal;
    match kind {
        "s" => LeafVal::Str(text.unwrap_or_default()),
        "n" => LeafVal::Num(num.unwrap_or_default()),
        "b" => LeafVal::Bool(b.unwrap_or(0) != 0),
        _ => LeafVal::Null,
    }
}

/// One multi-row `INSERT`, chunked under MySQL's 65,535-placeholder limit.
async fn insert_rows(
    tx: &mut mysql_async::Transaction<'_>,
    schema: &str,
    table: &str,
    cols: &[String],
    rows: &[Vec<mysql_async::Value>],
) -> Result<(), StoreError> {
    if rows.is_empty() {
        return Ok(());
    }
    let per_row = cols.len().max(1);
    let chunk = (60_000 / per_row).max(1);
    let collist = cols.join(", ");
    for group in rows.chunks(chunk) {
        let marks = std::iter::repeat_n(
            format!("({})", vec!["?"; cols.len()].join(", ")),
            group.len(),
        )
        .collect::<Vec<_>>()
        .join(", ");
        let flat: Vec<mysql_async::Value> = group.iter().flatten().cloned().collect();
        tx.exec_drop(
            format!("INSERT INTO {schema}.{table} ({collist}) VALUES {marks}"),
            mysql_async::Params::Positional(flat),
        )
        .await
        .map_err(|e| StoreError::Other(format!("inserting into {table}: {e}")))?;
    }
    Ok(())
}

// -------------------------------------------------------- history and versions

impl MariaDbStore {
    /// `(schema, history table)` for a resource type, both already quoted.
    fn hist_target(&self, rtype: &str) -> Result<(String, String), StoreError> {
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let hist = rm
            .find_table(fhir_mariadb_map::model::TableKind::History)
            .map(|(_, t)| quote_ident(&t.name))
            .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?;
        Ok((quote_ident(&self.map.schema), hist))
    }

    /// Every stored version of a resource, newest first.
    ///
    /// A deletion appears with `op == 'D'` and no resource, which is how a
    /// reader tells "deleted" from "never existed": the base row is gone in both
    /// cases, so history is the only witness.
    pub async fn history(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Vec<crate::HistEntry>, StoreError> {
        let (s, hist) = self.hist_target(rtype)?;
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let rows: Vec<mysql_async::Row> = conn
            .exec(
                format!(
                    "SELECT `version_id`, DATE_FORMAT(`last_updated`, '%Y-%m-%d %H:%i:%s.%f'), `op`, `resource` FROM {s}.{hist} \
                     WHERE `id` = ? ORDER BY `version_id` DESC"
                ),
                (id,),
            )
            .await
            .map_err(db_err)?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            let version_id: i64 = r.get(0).unwrap_or(0);
            let raw: Option<String> = r.get(3).unwrap_or(None);
            let resource = match raw {
                Some(t) => Some(
                    serde_json::from_str(&t)
                        .map_err(|e| StoreError::Other(format!("history {version_id}: {e}")))?,
                ),
                None => None,
            };
            out.push(crate::HistEntry {
                version_id,
                last_updated: r.get::<String, _>(1).unwrap_or_default(),
                op: r
                    .get::<String, _>(2)
                    .unwrap_or_default()
                    .chars()
                    .next()
                    .unwrap_or('?'),
                resource,
            });
        }
        Ok(out)
    }

    /// One specific version, as it was stored.
    ///
    /// Returns a `HistEntry`, not a bare resource: a deleted version has no
    /// content, and a caller must be able to tell "version 3 was a deletion"
    /// (410) from "version 3 does not exist" (404).
    pub async fn vread(
        &self,
        rtype: &str,
        id: &str,
        version_id: i64,
    ) -> Result<Option<crate::HistEntry>, StoreError> {
        let (s, hist) = self.hist_target(rtype)?;
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let row: Option<mysql_async::Row> = conn
            .exec_first(
                format!(
                    "SELECT `version_id`, DATE_FORMAT(`last_updated`, '%Y-%m-%d %H:%i:%s.%f'), `op`, `resource` FROM {s}.{hist} \
                     WHERE `id` = ? AND `version_id` = ?"
                ),
                (id, version_id),
            )
            .await
            .map_err(db_err)?;
        let Some(r) = row else { return Ok(None) };
        let raw: Option<String> = r.get(3).unwrap_or(None);
        let resource = match raw {
            Some(t) => Some(
                serde_json::from_str(&t)
                    .map_err(|e| StoreError::Other(format!("version {version_id}: {e}")))?,
            ),
            None => None,
        };
        Ok(Some(crate::HistEntry {
            version_id: r.get(0).unwrap_or(version_id),
            last_updated: r.get::<String, _>(1).unwrap_or_default(),
            op: r
                .get::<String, _>(2)
                .unwrap_or_default()
                .chars()
                .next()
                .unwrap_or('?'),
            resource,
        }))
    }

    /// Delete a resource, leaving a tombstone in history.
    ///
    /// Returns the tombstone's version, or `None` if there was nothing to
    /// delete. The base row goes, cascading to its children; history does not,
    /// because it has no foreign key to the base table — a deletion that erased
    /// its own evidence would defeat the audit trail.
    pub async fn delete(
        &self,
        rtype: &str,
        id: &str,
        audit: &crate::Audit,
    ) -> Result<Option<i64>, StoreError> {
        use mysql_async::Value as V;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let base = quote_ident(&rm.base_table().name);
        let (s, hist) = self.hist_target(rtype)?;
        let ts = utc_micros(std::time::SystemTime::now());

        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let mut tx = conn
            .start_transaction(mysql_async::TxOpts::default())
            .await
            .map_err(db_err)?;

        let present: Option<i64> = tx
            .exec_first(format!("SELECT 1 FROM {s}.{base} WHERE `id` = ?"), (id,))
            .await
            .map_err(db_err)?;
        if present.is_none() {
            return Ok(None);
        }

        let prev: Option<ChainTip> = tx
            .exec_first(
                format!(
                    "SELECT `version_id`, `row_hash`, `row_hash_sha3` FROM {s}.{hist} \
                     WHERE `id` = ? ORDER BY `version_id` DESC LIMIT 1"
                ),
                (id,),
            )
            .await
            .map_err(db_err)?;
        let (version_id, prev_256, prev_3) = match prev {
            Some((v, a, b)) => (v + 1, a, b),
            None => (1, None, None),
        };

        tx.exec_drop(format!("DELETE FROM {s}.{base} WHERE `id` = ?"), (id,))
            .await
            .map_err(db_err)?;

        // No resource in the preimage: there is no content to commit to, but a
        // tombstone still extends the chain.
        let pre = crate::chain::preimage(id, version_id, &ts, "D", None, &audit.actor);
        let (row_hash, row_sha3) = crate::chain::link(prev_256.as_deref(), prev_3.as_deref(), &pre);
        let row_mac = self
            .keys
            .signing()
            .map(|k| crate::chain::mac(k, prev_256.as_deref(), &pre));

        tx.exec_drop(
            format!(
                "INSERT INTO {s}.{hist} \
                   (`id`, `version_id`, `last_updated`, `op`, `resource`, `actor`, \
                    `actor_source`, `client`, `request_id`, `reason`, `prev_hash`, \
                    `row_hash`, `prev_hash_sha3`, `row_hash_sha3`, `row_mac`) \
                 VALUES (?,?,?,'D',NULL,?,?,?,?,?,?,?,?,?,?)"
            ),
            mysql_async::Params::Positional(vec![
                V::Bytes(id.to_string().into()),
                V::Int(version_id),
                V::Bytes(ts.into()),
                V::Bytes(audit.actor.clone().into()),
                opt_text(audit.actor_source.as_deref()),
                opt_text(audit.client.as_deref()),
                opt_text(audit.request_id.as_deref()),
                opt_text(audit.reason.as_deref()),
                prev_256.map_or(V::NULL, V::Bytes),
                V::Bytes(row_hash),
                prev_3.map_or(V::NULL, V::Bytes),
                V::Bytes(row_sha3),
                row_mac.map_or(V::NULL, |m| V::Bytes(m.into())),
            ]),
        )
        .await
        .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(Some(version_id))
    }

    /// Recompute every history chain and report what does not match.
    ///
    /// An empty result is the claim "nothing in history has been altered since
    /// it was written". Rows predating the chain columns have no stored hash and
    /// are skipped rather than reported: calling them breaks would train an
    /// operator to ignore the report.
    pub async fn verify_audit(&self) -> Result<Vec<crate::ChainBreak>, StoreError> {
        use fhir_mariadb_map::model::TableKind;
        let s = quote_ident(&self.map.schema);
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let mut breaks = Vec::new();

        // Counter-signatures up front: one small table for the whole schema, and
        // querying it per history row would make a linear walk quadratic.
        let mut countersigns: std::collections::HashMap<(String, String, i64), String> =
            std::collections::HashMap::new();
        let cs: Vec<mysql_async::Row> = conn
            .query(format!(
                "SELECT `rtype`, `id`, `version_id`, `row_mac` FROM {s}.`fhir_mariadb_countersign`"
            ))
            .await
            .map_err(db_err)?;
        for r in cs {
            countersigns.insert(
                (
                    r.get(0).unwrap_or_default(),
                    r.get(1).unwrap_or_default(),
                    r.get(2).unwrap_or(0),
                ),
                r.get(3).unwrap_or_default(),
            );
        }

        for rm in self.map.resources.values() {
            let Some((_, hist)) = rm.find_table(TableKind::History) else {
                continue;
            };
            let table = quote_ident(&hist.name);
            let rows: Vec<mysql_async::Row> = conn
                .query(format!(
                    "SELECT `id`,`version_id`,DATE_FORMAT(`last_updated`, '%Y-%m-%d %H:%i:%s.%f'),`op`,`resource`,`actor`,\
                            `prev_hash`,`row_hash`,`prev_hash_sha3`,`row_hash_sha3`,`row_mac` \
                     FROM {s}.{table} ORDER BY `id`, `version_id`"
                ))
                .await
                .map_err(db_err)?;

            // Ordered by (id, version_id), so each chain is walked in order and
            // the tip resets when the id changes.
            let mut cur = String::new();
            let mut prior_256: Option<Vec<u8>> = None;
            let mut prior_3: Option<Vec<u8>> = None;

            for r in rows {
                let id: String = r.get(0).unwrap_or_default();
                let version_id: i64 = r.get(1).unwrap_or(0);
                let ts: String = r.get(2).unwrap_or_default();
                let op: String = r.get(3).unwrap_or_default();
                let resource: Option<String> = r.get(4).unwrap_or(None);
                let actor: String = r.get(5).unwrap_or_default();
                let prev_256: Option<Vec<u8>> = r.get(6).unwrap_or(None);
                let row_256: Option<Vec<u8>> = r.get(7).unwrap_or(None);
                let prev_3: Option<Vec<u8>> = r.get(8).unwrap_or(None);
                let row_3: Option<Vec<u8>> = r.get(9).unwrap_or(None);
                let row_mac: Option<String> = r.get(10).unwrap_or(None);

                if id != cur {
                    cur.clone_from(&id);
                    prior_256 = None;
                    prior_3 = None;
                }

                let pre =
                    crate::chain::preimage(&id, version_id, &ts, &op, resource.as_deref(), &actor);

                // Verified against the row's *stored* `prev_hash`, not the tip
                // the walk arrived with. They agree for an ordinary row, and
                // where they do not the link check below is what says so.
                // Separating them lets an erasure tombstone keep a meaningful
                // tag: its predecessors were deleted on purpose.
                check_mac(
                    &self.keys,
                    &countersigns,
                    &rm.name,
                    &id,
                    version_id,
                    row_mac.as_deref(),
                    prev_256.as_deref(),
                    &pre,
                    &mut breaks,
                );

                // An erasure tombstone is a deliberate hole: its `prev_hash`
                // points at rows removed on purpose, so checking its link would
                // report every lawful erasure as tampering.
                if op == "X" {
                    prior_256 = row_256;
                    prior_3 = row_3;
                    continue;
                }

                let (Some(stored_256), Some(stored_3)) = (&row_256, &row_3) else {
                    prior_256 = row_256;
                    prior_3 = row_3;
                    continue;
                };

                let (want_256, want_3) =
                    crate::chain::link(prior_256.as_deref(), prior_3.as_deref(), &pre);
                for (algorithm, stored, want, stored_link, prior) in [
                    ("sha256", stored_256, &want_256, &prev_256, &prior_256),
                    ("sha3-256", stored_3, &want_3, &prev_3, &prior_3),
                ] {
                    let bad = !crate::chain::digests_equal(stored, want);
                    let unlinked = stored_link.as_deref() != prior.as_deref();
                    if bad || unlinked {
                        breaks.push(crate::ChainBreak {
                            rtype: rm.name.clone(),
                            id: id.clone(),
                            version_id,
                            algorithm,
                            detail: match (bad, unlinked) {
                                (true, true) => "row hash and link both differ".into(),
                                (true, false) => "row contents differ from their hash".into(),
                                _ => "link to the previous version differs".into(),
                            },
                        });
                    }
                }

                prior_256 = row_256;
                prior_3 = row_3;
            }
        }
        Ok(breaks)
    }
}

/// Verify one row's keyed tag, recording a finding only when the tag is present
/// and wrong.
///
/// - **Absent** — written unkeyed. Not a finding.
/// - **Unverifiable** — signed under a key this process does not hold. A
///   warning, never a break: "I cannot check this" and "this was altered" are
///   different claims, and conflating them makes the report useless.
/// - **Mismatch** — a finding, and it stays one even if a counter-signature
///   vouches for the row, or re-signing would be a way to bless forged history.
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
    use crate::chain::{self, MacCheck};

    let own = keys.check(stored, prev_sha256, pre);
    let verdict = match (
        &own,
        countersigns.get(&(rtype.to_string(), id.to_string(), version_id)),
    ) {
        (MacCheck::Absent | MacCheck::Unverifiable { .. }, Some(have)) => match keys.signing() {
            Some(k)
                if chain::digests_equal(
                    chain::mac(k, prev_sha256, pre).as_bytes(),
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
        MacCheck::Mismatch => breaks.push(crate::ChainBreak {
            rtype: rtype.to_string(),
            id: id.to_string(),
            version_id,
            algorithm: "hmac-sha256",
            detail: "keyed tag does not match".into(),
        }),
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

// --------------------------------------------------------------------- search

/// One page of search results.
pub type Page = crate::SearchOutcome;

impl MariaDbStore {
    /// Just the ids.
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
    pub async fn search_full(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        sort: &[crate::mariadb_search::SortKey],
        want_total: bool,
    ) -> Result<Page, StoreError> {
        self.search_page(rtype, params, count, offset, sort, want_total, None)
            .await
    }

    /// A page, with an optional keyset cursor.
    ///
    /// `after_id` narrows only the page query, never the count: a `_total` that
    /// shrank as a caller paged would make paging impossible to drive.
    ///
    /// Values are bound, never interpolated — the invariant the fuzz target
    /// protects. MySQL binds strictly in order, so the builder's binds are
    /// handed over as a positional list exactly as emitted.
    #[allow(clippy::too_many_arguments)]
    pub async fn search_page(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        sort: &[crate::mariadb_search::SortKey],
        want_total: bool,
        after_id: Option<&str>,
    ) -> Result<Page, StoreError> {
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let q = crate::mariadb_search::build_search_sql(
            &self.map, rm, params, count, offset, sort, after_id,
        )?;
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;

        let total = if want_total {
            let binds: Vec<mysql_async::Value> = q
                .binds
                .iter()
                .take(q.count_binds)
                .map(|b| mysql_async::Value::Bytes(b.clone().into()))
                .collect();
            let n: Option<i64> = conn
                .exec_first(&q.count_sql, mysql_async::Params::Positional(binds))
                .await
                .map_err(|e| StoreError::Other(format!("count: {e}\n{}", q.count_sql)))?;
            n
        } else {
            None
        };

        let binds: Vec<mysql_async::Value> = q
            .binds
            .iter()
            .map(|b| mysql_async::Value::Bytes(b.clone().into()))
            .collect();
        let ids: Vec<String> = conn
            .exec(&q.sql, mysql_async::Params::Positional(binds))
            .await
            .map_err(|e| StoreError::Other(format!("search: {e}\n{}", q.sql)))?;
        Ok(crate::SearchOutcome { ids, total })
    }
}

// ------------------------------------------------------- access log & erasure

impl MariaDbStore {
    /// Record one disclosure (PR12.5).
    ///
    /// A store that logs only mutations cannot answer "who looked at this
    /// patient", which is the question an audit usually opens with. This is a
    /// read-path obligation, and the read path is where it is easiest to forget.
    pub async fn log_access(&self, rec: &crate::AccessRecord) -> Result<(), StoreError> {
        use mysql_async::Value as V;
        let s = quote_ident(&self.map.schema);
        let ts = utc_micros(std::time::SystemTime::now());
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        conn.exec_drop(
            format!(
                "INSERT INTO {s}.`fhir_mariadb_access_log` \
                   (`ts`, `request_id`, `actor`, `actor_source`, `client`, `interaction`, \
                    `rtype`, `id`, `version_id`, `outcome`, `result_count`, `reason`) \
                 VALUES (?,?,?,?,?,?,?,?,?,?,?,?)"
            ),
            mysql_async::Params::Positional(vec![
                V::Bytes(ts.into()),
                opt_text(rec.audit.request_id.as_deref()),
                V::Bytes(rec.audit.actor.clone().into()),
                opt_text(rec.audit.actor_source.as_deref()),
                opt_text(rec.audit.client.as_deref()),
                V::Bytes(rec.interaction.clone().into()),
                opt_text(rec.rtype.as_deref()),
                opt_text(rec.id.as_deref()),
                rec.version_id.map_or(V::NULL, V::Int),
                V::Bytes(rec.outcome.clone().into()),
                rec.result_count.map_or(V::NULL, V::Int),
                opt_text(rec.audit.reason.as_deref()),
            ]),
        )
        .await
        .map_err(db_err)
    }

    /// Record several disclosures.
    pub async fn log_access_batch(&self, recs: &[crate::AccessRecord]) -> Result<(), StoreError> {
        for r in recs {
            self.log_access(r).await?;
        }
        Ok(())
    }

    /// How many disclosures have been recorded.
    pub async fn access_log_len(&self) -> Result<i64, StoreError> {
        let s = quote_ident(&self.map.schema);
        let mut conn = self.pool.get_conn().await.map_err(db_err)?;
        let n: Option<i64> = conn
            .query_first(format!(
                "SELECT count(*) FROM {s}.`fhir_mariadb_access_log`"
            ))
            .await
            .map_err(db_err)?;
        Ok(n.unwrap_or(0))
    }

    /// Erase one resource and its history (GDPR Art. 17, spec M3.18).
    ///
    /// The one sanctioned exception to append-only history, and deliberately
    /// noisy: the history rows go and a tombstone takes their place recording
    /// who erased it, when, why, and the hash the chain ended on. What is left is
    /// a *verifiable hole* — `verify_audit` can still see that a chain existed
    /// and was deliberately terminated — rather than something indistinguishable
    /// from a chain that never happened.
    ///
    /// The append-only trigger permits the delete only while
    /// `@fhir_mariadb_erasure` is set. That variable is per-connection, so every
    /// statement here runs on **one** connection held for the whole operation:
    /// setting it on a pooled connection and then deleting on another would
    /// leave the trigger firing and the erasure failing, in a way that would
    /// look like the trigger was broken.
    ///
    /// What this cannot do is un-say the data: backups, replicas, and binlogs
    /// still hold it until they age out.
    pub async fn purge(
        &self,
        rtype: &str,
        id: &str,
        audit: &crate::Audit,
    ) -> Result<crate::PurgeReport, StoreError> {
        use mysql_async::Value as V;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let base = quote_ident(&rm.base_table().name);
        let (s, hist) = self.hist_target(rtype)?;
        let ts = utc_micros(std::time::SystemTime::now());

        let mut conn = self.pool.get_conn().await.map_err(db_err)?;

        let tip: Option<ChainTip> = conn
            .exec_first(
                format!(
                    "SELECT `version_id`, `row_hash`, `row_hash_sha3` FROM {s}.{hist} \
                     WHERE `id` = ? ORDER BY `version_id` DESC LIMIT 1"
                ),
                (id,),
            )
            .await
            .map_err(db_err)?;
        let Some((last_version, tip_256, tip_3)) = tip else {
            return Ok(crate::PurgeReport {
                versions_erased: 0,
                existed: false,
            });
        };

        // Open the escape hatch on this connection, and close it before the
        // connection returns to the pool — a pooled connection carrying a live
        // erasure flag would let an unrelated later request delete history.
        "SET @fhir_mariadb_erasure = 'on'"
            .ignore(&mut conn)
            .await
            .map_err(db_err)?;

        let result = async {
            conn.exec_drop(format!("DELETE FROM {s}.{base} WHERE `id` = ?"), (id,))
                .await
                .map_err(db_err)?;
            conn.exec_drop(format!("DELETE FROM {s}.{hist} WHERE `id` = ?"), (id,))
                .await
                .map_err(db_err)?;
            let erased = conn.affected_rows();

            let tomb_version = last_version + 1;
            let pre = crate::chain::preimage(id, tomb_version, &ts, "X", None, &audit.actor);
            let (row_hash, row_sha3) =
                crate::chain::link(tip_256.as_deref(), tip_3.as_deref(), &pre);
            let row_mac = self
                .keys
                .signing()
                .map(|k| crate::chain::mac(k, tip_256.as_deref(), &pre));

            // `prev_hash` records the tip of the chain that was erased. Those
            // rows are gone, so the link cannot be re-derived — that is the
            // point. It is evidence that something was there.
            conn.exec_drop(
                format!(
                    "INSERT INTO {s}.{hist} \
                       (`id`, `version_id`, `last_updated`, `op`, `resource`, `actor`, \
                        `actor_source`, `client`, `request_id`, `reason`, `prev_hash`, \
                        `row_hash`, `prev_hash_sha3`, `row_hash_sha3`, `row_mac`) \
                     VALUES (?,?,?,'X',NULL,?,?,?,?,?,?,?,?,?,?)"
                ),
                mysql_async::Params::Positional(vec![
                    V::Bytes(id.to_string().into()),
                    V::Int(tomb_version),
                    V::Bytes(ts.clone().into()),
                    V::Bytes(audit.actor.clone().into()),
                    opt_text(audit.actor_source.as_deref()),
                    opt_text(audit.client.as_deref()),
                    opt_text(audit.request_id.as_deref()),
                    opt_text(audit.reason.as_deref()),
                    tip_256.clone().map_or(V::NULL, V::Bytes),
                    V::Bytes(row_hash),
                    tip_3.clone().map_or(V::NULL, V::Bytes),
                    V::Bytes(row_sha3),
                    row_mac.map_or(V::NULL, |m| V::Bytes(m.into())),
                ]),
            )
            .await
            .map_err(db_err)?;
            Ok::<u64, StoreError>(erased)
        }
        .await;

        // Close the hatch whatever happened, then report.
        "SET @fhir_mariadb_erasure = NULL"
            .ignore(&mut conn)
            .await
            .map_err(db_err)?;

        Ok(crate::PurgeReport {
            versions_erased: result?,
            existed: true,
        })
    }
}
