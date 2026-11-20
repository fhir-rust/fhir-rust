//! The SQLite store (spec section 14, T64).
//!
//! Built alongside the inherited PostgreSQL store rather than replacing it in
//! one edit. A crate-wide driver swap does not compile until every one of the
//! ~57 call sites is converted, which means no test can run in between; this way
//! each capability is verifiable against a real database as it lands, and the
//! PostgreSQL module is deleted when this one reaches parity.
//!
//! # Differences from the PostgreSQL store that shape this module
//!
//! - **A schema is an attached database** (M14.15). `"{schema}"."{table}"`
//!   survives as a qualified name; `ATTACH` replaces `CREATE SCHEMA`.
//! - **Installation is one transaction** (M14.16). SQLite's DDL is
//!   transactional, so the staged-schema-then-rename dance PostgreSQL needs —
//!   it exists only because single-transaction DDL exhausts PostgreSQL's lock
//!   budget — is not carried over. Install either completes or leaves nothing.
//! - **One writer at a time** (M14.18/M14.19). `BEGIN IMMEDIATE` takes the
//!   write lock up front, which is what `pg_advisory_xact_lock` and
//!   `SELECT … FOR UPDATE` were buying. WAL lets readers continue meanwhile.
//! - **The driver is synchronous.** `rusqlite` blocks, so every call is wrapped
//!   in `spawn_blocking` to keep the async contract the server layer expects.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use fhir_sqlite_map::model::{ColTy, RelMap, TableKind};
use rusqlite::Connection;

use crate::{StoreError, UpgradeReport};

/// Render one stored cell as the **text image** the shared reconstruction
/// engine expects, given the column's declared type.
///
/// This exists because SQLite is dynamically typed and the shared engine is
/// not. `reconstruct::prim_json` parses a column out of a `String`, and for a
/// boolean it accepts exactly `"true"`, `"t"`, `"false"`, or `"f"` — the text
/// image PostgreSQL yields. SQLite binds `Bool`, `Int`, and `BigInt` all to
/// `INTEGER` (M14.10), so those columns come back as `ValueRef::Integer` and a
/// bare `r.get::<_, Option<String>>(i)` fails to convert.
///
/// It previously failed *into an `if let Ok(Some(v))`*, which discarded the
/// error and simply left the column out of the row — so every boolean and
/// every integer element vanished from every reconstructed resource, silently.
/// `Patient.active` did not survive a round trip. That is an R4.2 violation,
/// and the silence is an R4.3 one: data the engine cannot render must be an
/// error naming the column, never a quiet omission (audit F-20).
///
/// `Ok(None)` means SQL NULL, which is a legitimately absent element. Every
/// other unrenderable cell is an error.
fn cell_text(
    v: rusqlite::types::ValueRef<'_>,
    ty: ColTy,
    idx: usize,
    name: &str,
) -> rusqlite::Result<Option<String>> {
    use rusqlite::types::ValueRef as V;
    let bad = |v: V<'_>| rusqlite::Error::InvalidColumnType(idx, name.to_string(), v.data_type());
    Ok(match (ty, v) {
        (_, V::Null) => None,

        // The one type whose SQL image differs from its JSON image.
        (ColTy::Bool, V::Integer(i)) => Some(if i == 0 { "false" } else { "true" }.to_string()),
        // Tolerated so a database written by another port's text image, or
        // repaired by hand, still reads. Anything else is a real error.
        (ColTy::Bool, V::Text(t)) => match std::str::from_utf8(t).map_err(|_| bad(v))? {
            s @ ("true" | "t" | "false" | "f") => Some(s.to_string()),
            "1" => Some("true".to_string()),
            "0" => Some("false".to_string()),
            _ => return Err(bad(v)),
        },

        (_, V::Integer(i)) => Some(i.to_string()),
        (_, V::Real(f)) => Some(f.to_string()),
        (_, V::Text(t)) => Some(std::str::from_utf8(t).map_err(|_| bad(v))?.to_string()),
        (_, V::Blob(_)) => return Err(bad(v)),
    })
}

/// The stored image of an `ords` path: `{1,2}`, `{}`, `{-1,3}`.
///
/// Deliberately the same text PostgreSQL wrote for its `smallint[]` column, so
/// a database can still be compared value-for-value against one. The database
/// never orders, subscripts, or unnests this — it only stores it and enforces
/// uniqueness — which is what makes a plain TEXT column sufficient (spec M14.7).
pub(crate) fn fmt_ords(ords: &[i16]) -> String {
    let inner: Vec<String> = ords.iter().map(|o| o.to_string()).collect();
    format!("{{{}}}", inner.join(","))
}

/// Inverse of [`fmt_ords`]. Lenient about surrounding braces and whitespace so
/// a value written by either engine round-trips.
pub(crate) fn parse_ords(s: &str) -> Result<Vec<i16>, StoreError> {
    let trimmed = s.trim();
    let t = trimmed.trim_start_matches('{').trim_end_matches('}');
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

/// A history row's chain tip: `(version_id, sha256 link, sha3 link)`.
type ChainTip = (i64, Option<Vec<u8>>, Option<Vec<u8>>);

/// A handle to one SQLite database file and the map describing its schema.
///
/// Not a pool. SQLite admits a single writer, so a pool of writers would only
/// convert lock contention into `SQLITE_BUSY`; a connection per store, guarded
/// by the engine's own locking, is the honest shape. Read concurrency comes from
/// WAL, not from more connections.
pub struct SqliteStore {
    path: PathBuf,
    map: Arc<RelMap>,
    conn: Arc<tokio::sync::Mutex<Connection>>,
    /// Serialises read-then-write sequences within this process.
    ///
    /// SQLite already admits one writer at a time, but a conditional create is
    /// a *search* followed by a write, and the engine's lock does not span the
    /// gap between them. This does. It is the local equivalent of the advisory
    /// lock the PostgreSQL store takes on the criteria — coarser, since it
    /// serialises all conditional operations rather than just colliding ones,
    /// which is an acceptable trade when the engine permits one writer anyway.
    write_gate: Arc<tokio::sync::Mutex<()>>,

    /// Keys for signing and verifying history rows.
    ///
    /// Held on the store rather than passed per call, matching the PostgreSQL
    /// original. The reason is not symmetry: a signing key that travels as an
    /// argument is one a caller can forget, and a write that silently goes
    /// unsigned is indistinguishable later from one that was never keyed.
    /// Empty by default, which is unkeyed — a weaker mode, and one the store
    /// says out loud rather than implying by omission.
    keys: crate::chain::KeyRing,
}

impl SqliteStore {
    /// Open (creating if absent) the database at `path`.
    ///
    /// The pragmas are not tuning; each one is load-bearing:
    /// - `foreign_keys=ON` — SQLite ignores foreign keys unless asked, and the
    ///   child tables' `ON DELETE CASCADE` is how a resource's rows are removed
    ///   when it is rewritten. Without this, a rewrite silently orphans rows and
    ///   reconstruction later reports them as an ordinal gap.
    /// - `journal_mode=WAL` — readers do not block the writer, which is what
    ///   makes snapshot reads possible at all (M14.19).
    /// - `busy_timeout` — with one writer, a concurrent writer must wait rather
    ///   than fail immediately.
    /// - `synchronous=FULL` — this stores health records with a hash chain; a
    ///   torn write is not an acceptable trade for speed.
    pub async fn open(path: impl AsRef<Path>, map: Arc<RelMap>) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let p = path.clone();
        let conn = tokio::task::spawn_blocking(move || -> Result<Connection, StoreError> {
            if let Some(dir) = p.parent()
                && !dir.as_os_str().is_empty()
            {
                std::fs::create_dir_all(dir).map_err(|e| StoreError::Other(e.to_string()))?;
            }
            let c = Connection::open(&p).store()?;
            configure_connection(c)
        })
        .await
        .map_err(join_err)??;

        Ok(Self {
            path,
            map,
            conn: Arc::new(tokio::sync::Mutex::new(conn)),
            write_gate: Arc::new(tokio::sync::Mutex::new(())),
            keys: crate::chain::KeyRing::default(),
        })
    }

    /// Attach a key ring for signing and verifying history.
    #[must_use]
    pub fn with_chain_keys(mut self, keys: crate::chain::KeyRing) -> Self {
        self.keys = keys;
        self
    }

    /// The id of the signing key, if this store is keyed.
    #[must_use]
    pub fn chain_key_id(&self) -> Option<String> {
        self.keys.signing().map(|k| k.id().to_string())
    }

    /// Where this store's main database lives.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The schema (FHIR version) name this store's map describes.
    #[must_use]
    pub fn schema(&self) -> &str {
        &self.map.schema
    }

    /// The file backing the attached database for `schema`.
    ///
    /// One file per FHIR version, attached under the version name, so `r3`,
    /// `r4`, and `r5` can be served from one process without their thousands of
    /// identically-named tables colliding (M14.15).
    fn attached_path(&self, schema: &str) -> PathBuf {
        let stem = self
            .path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "fhir".to_string());
        self.path.with_file_name(format!("{stem}-{schema}.sqlite"))
    }

    /// Attach this store's schema database, creating the file if needed.
    ///
    /// Idempotent: attaching an already-attached name is not an error worth
    /// surfacing, because `init` and every subsequent call want the same state.
    async fn attach(&self) -> Result<(), StoreError> {
        let schema = self.map.schema.clone();
        let file = self.attached_path(&schema);
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<(), StoreError> {
            let c = conn.blocking_lock();
            let already: i64 = c
                .query_row(
                    "SELECT count(*) FROM pragma_database_list WHERE name = ?1",
                    [&schema],
                    |r| r.get(0),
                )
                .store()?;
            if already > 0 {
                return Ok(());
            }
            // The name is a schema identifier from the map, not user input, but
            // it is quoted anyway: an unquoted identifier here would be one
            // rename away from a syntax error, and quoting costs nothing.
            c.execute(
                &format!("ATTACH DATABASE ?1 AS \"{}\"", escape_ident(&schema)),
                [file.to_string_lossy().as_ref()],
            )
            .store()?;
            Ok(())
        })
        .await
        .map_err(join_err)?
    }

    /// Install the schema, in a single transaction.
    ///
    /// `checksum` is recorded in the meta table so a later run can tell whether
    /// the installed schema was generated from the same map.
    ///
    /// The **map asset itself** is recorded alongside it, gzipped and hex-coded
    /// (`M14.30`). That is what makes [`upgrade`](Self::upgrade) possible: an
    /// upgrade diffs the installed map against the current one, and without a
    /// copy of what was installed there is nothing to diff against — only the
    /// checksum, which says *that* something changed and never *what*.
    ///
    /// Fails if the schema is already installed — this is `init`, not `upgrade`.
    /// Because SQLite's DDL is transactional, a failure part-way leaves the
    /// database exactly as it was, which is the guarantee PostgreSQL needed a
    /// staging schema to fake (M14.16).
    pub async fn init(&self, checksum: &str) -> Result<usize, StoreError> {
        self.attach().await?;
        let statements = fhir_sqlite_map::ddl::ddl(&self.map);
        let schema = self.map.schema.clone();
        let checksum = checksum.to_string();
        let asset_hex = map_asset_hex(&self.map)?;
        let version = self.map.fhir_version.as_str().to_string();
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<usize, StoreError> {
            let mut c = conn.blocking_lock();
            let tx = c
                .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                .store()?;
            let n = statements.len();
            for s in &statements {
                tx.execute_batch(s).map_err(|e| {
                    // Naming the statement matters: with several thousand of
                    // them, "syntax error" alone is not actionable.
                    StoreError::Other(format!("installing schema: {e}\nstatement was:\n{s}"))
                })?;
            }
            let meta = format!(
                "INSERT INTO \"{}\".\"fhir_sqlite_meta\" (\"key\", \"value\") VALUES (?1, ?2)",
                escape_ident(&schema)
            );
            for (k, v) in [
                ("map_checksum", &checksum),
                ("fhir_version", &version),
                ("map_asset", &asset_hex),
            ] {
                tx.execute(&meta, rusqlite::params![k, v]).store()?;
            }
            tx.commit().store()?;
            Ok(n)
        })
        .await
        .map_err(join_err)?
    }

    /// Is this schema installed, and does it match `checksum`?
    ///
    /// `Ok(None)` means "not installed", which callers must distinguish from
    /// "installed but stale" — the two need different remedies.
    pub async fn installed_checksum(&self) -> Result<Option<String>, StoreError> {
        self.attach().await?;
        let schema = self.map.schema.clone();
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<Option<String>, StoreError> {
            let c = conn.blocking_lock();
            let meta = format!("{schema}.fhir_sqlite_meta");
            let exists: i64 = c
                .query_row(
                    &format!(
                        "SELECT count(*) FROM \"{}\".sqlite_master WHERE type='table' AND name='fhir_sqlite_meta'",
                        escape_ident(&schema)
                    ),
                    [],
                    |r| r.get(0),
                )
                .map_err(|e| StoreError::Other(format!("checking {meta}: {e}")))?;
            if exists == 0 {
                return Ok(None);
            }
            let v = c
                .query_row(
                    &format!(
                        "SELECT \"value\" FROM \"{}\".\"fhir_sqlite_meta\" WHERE \"key\" = 'map_checksum'",
                        escape_ident(&schema)
                    ),
                    [],
                    |r| r.get::<_, String>(0),
                )
                .ok();
            Ok(v)
        })
        .await
        .map_err(join_err)?
    }

    /// Upgrade an installed schema to this store's map: additive changes (new
    /// tables, new columns, new indexes) apply automatically; destructive ones
    /// (dropped tables and columns) require `allow_destructive`. Column type
    /// changes always refuse — those need a manual migration (`O10.4a`, `L12`).
    ///
    /// Mirrors `fhir-postgresql`'s `upgrade`, which was the only one of the six
    /// (audit **F-15**), and differs from it in three places that are SQLite's
    /// doing rather than choices:
    ///
    /// 1. **The whole upgrade is one transaction.** SQLite's DDL is
    ///    transactional and its lock budget is a single writer, so there is no
    ///    reason to chunk as PostgreSQL does — and chunking would be worse: an
    ///    upgrade that fails at chunk 7 of 20 leaves a schema that is neither
    ///    the old one nor the new one. Here it either lands or it does not
    ///    (`M14.31`).
    /// 2. **The audit envelope is diffed, not reconciled.** SQLite has no
    ///    `ADD COLUMN IF NOT EXISTS`, so applying those statements
    ///    unconditionally — which is what PostgreSQL does — fails on the second
    ///    run. They are filtered against `pragma_table_info` first, which is
    ///    what `ddl::history_audit_columns` tells its caller to do.
    /// 3. **`DROP COLUMN` has preconditions.** SQLite refuses to drop a column
    ///    that is indexed, part of the primary key, or named in a trigger. That
    ///    surfaces as a plain `SQLITE_ERROR`, so the message is rewritten to say
    ///    which column and why (`M14.32`).
    ///
    /// The backfill of folded search columns runs last and is part of the
    /// upgrade, not a step afterwards — see [`backfill_norm`](Self::backfill_norm)
    /// for why that is not optional.
    pub async fn upgrade(
        &self,
        checksum: &str,
        allow_destructive: bool,
    ) -> Result<UpgradeReport, StoreError> {
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
    /// under the new one, version and timestamp preserved, no history entry
    /// (a representation change is not a new version), verified
    /// byte-identical before anything is dropped. All inside the same single
    /// transaction as the rest of the upgrade: on SQLite the migration either
    /// lands whole or not at all (`M14.31`).
    pub async fn upgrade_with(
        &self,
        checksum: &str,
        opts: crate::UpgradeOpts,
    ) -> Result<UpgradeReport, StoreError> {
        self.attach().await?;
        let s = self.map.schema.clone();
        let esc = escape_ident(&s);

        // The installed map, or a refusal that says which of the two reasons
        // applies: never installed, or installed before this port recorded the
        // asset. They need different remedies — `init` versus a reload — and a
        // single "cannot upgrade" would hide that.
        let old_hex = {
            let (conn, esc2) = (self.conn.clone(), esc.clone());
            tokio::task::spawn_blocking(move || -> Result<String, StoreError> {
                let c = conn.blocking_lock();
                let exists: i64 = c
                    .query_row(
                        &format!(
                            "SELECT count(*) FROM \"{esc2}\".sqlite_master \
                             WHERE type='table' AND name='fhir_sqlite_meta'"
                        ),
                        [],
                        |r| r.get(0),
                    )
                    .store()?;
                if exists == 0 {
                    return Err(StoreError::Other(format!("schema {esc2} is not installed")));
                }
                c.query_row(
                    &format!(
                        "SELECT \"value\" FROM \"{esc2}\".\"fhir_sqlite_meta\" \
                         WHERE \"key\" = 'map_asset'"
                    ),
                    [],
                    |r| r.get::<_, String>(0),
                )
                .map_err(|_| {
                    StoreError::Other(
                        "installed schema predates upgrade support (no stored map asset); \
                         reinstall with `init` to make later upgrades possible"
                            .into(),
                    )
                })
            })
            .await
            .map_err(join_err)??
        };
        let old_map = RelMap::from_gz_bytes(&hex_decode(&old_hex)?)
            .map_err(|e| StoreError::Other(format!("stored map asset unreadable: {e}")))?;

        let (adds, destructive) = self.diff_maps(&old_map, &esc)?;
        // O10.4b: a moved column is not a drop. A map change that relocates
        // an element between tables (G2.6a's force-split, F-90) reaches the
        // diff as an ADD plus a DROP, and `allow_destructive` was defined
        // for abandoning data, not relocating it. Refuse a data-bearing
        // move by name, independent of the flag; an empty source proceeds.
        // Checked before the destructive gate: "rerun with
        // --allow-destructive" is the wrong advice for a relocation.
        let moved = moved_columns(&old_map, &self.map);
        if !moved.is_empty() && !opts.reshred_moved {
            let (conn, esc2, moved2) = (self.conn.clone(), esc.clone(), moved.clone());
            let data_bearing =
                tokio::task::spawn_blocking(move || -> Result<Vec<String>, StoreError> {
                    let c = conn.blocking_lock();
                    let mut out = Vec::new();
                    for (t, col, nt) in &moved2 {
                        let has: i64 = c
                            .query_row(
                                &format!(
                                    "SELECT EXISTS(SELECT 1 FROM \"{esc2}\".\"{t}\" \
                                     WHERE \"{col}\" IS NOT NULL)"
                                ),
                                [],
                                |r| r.get(0),
                            )
                            .store()?;
                        if has != 0 {
                            out.push(format!("{t}.{col} → {nt}"));
                        }
                    }
                    Ok(out)
                })
                .await
                .map_err(join_err)??;
            if !data_bearing.is_empty() {
                return Err(StoreError::Other(format!(
                    "upgrade refuses {} moved column(s) holding data (O10.4b, F-90): {}. \
                     --allow-destructive does not cover relocation; rerun with \
                     reshred_moved (O10.4c), re-put the affected resource types \
                     through this artifact, or reload",
                    data_bearing.len(),
                    data_bearing.join(", ")
                )));
            }
        }
        if !destructive.is_empty() && !opts.allow_destructive {
            return Err(StoreError::Other(format!(
                "upgrade requires {} destructive change(s); rerun with --allow-destructive \
                 (first: {})",
                destructive.len(),
                destructive.first().expect("non-empty")
            )));
        }

        // Objects the per-resource diff cannot see, because they are not in the
        // relational map: the access log, the erasure flag, and the append-only
        // guards. Every one of these statements carries `IF NOT EXISTS`, so they
        // are *reconciled* — applied every time, counted never, which is what
        // keeps "a re-upgrade reports zero changes" true.
        let mut reconcile = fhir_sqlite_map::ddl::schema_wide_objects(&esc);
        let mut history_tables: Vec<String> = Vec::new();
        for rm in self.map.resources.values() {
            if let Some((_, hist)) = rm.find_table(TableKind::History) {
                reconcile.extend(fhir_sqlite_map::ddl::append_only_triggers(&esc, &hist.name));
                history_tables.push(hist.name.clone());
            }
        }

        let new_hex = map_asset_hex(&self.map)?;
        let (checksum, version) = (
            checksum.to_string(),
            self.map.fhir_version.as_str().to_string(),
        );
        let (n_add, n_drop) = (adds.len(), destructive.len());
        let conn = self.conn.clone();
        let esc2 = esc.clone();
        let new_map = std::sync::Arc::clone(&self.map);
        let reshred = opts.reshred_moved;

        let reshredded = tokio::task::spawn_blocking(move || -> Result<usize, StoreError> {
            let mut c = conn.blocking_lock();
            let tx = c
                .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                .store()?;

            // Adds first: a resource type new in this artifact has no tables
            // until `adds` creates them, and reconciliation would then try to
            // put a trigger on a table that does not exist.
            for stmt in &adds {
                tx.execute_batch(stmt).map_err(|e| {
                    StoreError::Other(format!("upgrade: {e}\nstatement was:\n{stmt}"))
                })?;
            }
            // The audit envelope, filtered to the columns actually missing.
            for table in &history_tables {
                let have = installed_columns(&tx, &esc2, table)?;
                for stmt in fhir_sqlite_map::ddl::history_audit_columns(&esc2, table) {
                    if added_column_name(&stmt).is_some_and(|name| have.contains(&name)) {
                        continue;
                    }
                    tx.execute_batch(&stmt).map_err(|e| {
                        StoreError::Other(format!("upgrade: {e}\nstatement was:\n{stmt}"))
                    })?;
                }
            }
            for stmt in &reconcile {
                tx.execute_batch(stmt)
                    .map_err(|e| StoreError::Other(drop_column_hint(stmt, &e.to_string())))?;
            }

            // O10.4c: carry data across relocated columns by re-shredding
            // each affected resource — reconstruct under the *stored* old
            // map, shred under the new, same version_id and last_updated
            // (a representation change is not a new version, so no history
            // entry and no chain link), verified byte-identical before the
            // drops below can run. Placed after `adds` (the new tables must
            // exist) and before `destructive` (the old columns must still
            // exist to read).
            let mut n_reshred = 0usize;
            if reshred && !moved.is_empty() {
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
                    let Some(new_rm) = new_map.resources.get(rtype) else {
                        return Err(StoreError::Other(format!(
                            "{rtype} has moved columns but the new map does not \
                             carry the resource; re-shred cannot target it"
                        )));
                    };
                    let base = &old_rm.base_table().name;
                    if base != &new_rm.base_table().name {
                        return Err(StoreError::Other(format!(
                            "{rtype}: base table renamed {base} → {}; re-shred \
                             does not support that",
                            new_rm.base_table().name
                        )));
                    }
                    let ids: Vec<String> = {
                        let mut st = tx
                            .prepare(&format!(
                                "SELECT \"id\" FROM \"{esc2}\".\"{base}\" ORDER BY \"id\""
                            ))
                            .store()?;
                        let rows = st.query_map([], |r| r.get::<_, String>(0)).store()?;
                        rows.collect::<Result<_, _>>().store()?
                    };
                    for id in ids {
                        let input = fetch_recon_input(&tx, &esc2, old_rm, &id)?;
                        let value =
                            fhir_sqlite_map::reconstruct::reconstruct(old_rm, &input, Some(&id))?;
                        let (version_id, ts): (i64, String) = tx
                            .query_row(
                                &format!(
                                    "SELECT \"version_id\", \"last_updated\" FROM \
                                     \"{esc2}\".\"{base}\" WHERE \"id\" = ?1"
                                ),
                                [&id],
                                |r| Ok((r.get(0)?, r.get(1)?)),
                            )
                            .store()?;
                        tx.execute(
                            &format!("DELETE FROM \"{esc2}\".\"{base}\" WHERE \"id\" = ?1"),
                            [&id],
                        )
                        .store()?;
                        let out = fhir_sqlite_map::shred::shred(new_rm, &value)?;
                        let plan = InsertPlan::build(new_rm, &out, &id)?;
                        plan.apply(&tx, &esc2, version_id, &ts)?;
                        let back = fetch_recon_input(&tx, &esc2, new_rm, &id)?;
                        let v2 =
                            fhir_sqlite_map::reconstruct::reconstruct(new_rm, &back, Some(&id))?;
                        verify_reshred_identity(rtype, &id, &value, &v2)?;
                        n_reshred += 1;
                    }
                }
                // The moved sources must now be empty — checked inside the
                // same transaction, before the drops run, so a miss rolls
                // everything back rather than dropping data.
                for (t, col, nt) in &moved {
                    let has: i64 = tx
                        .query_row(
                            &format!(
                                "SELECT EXISTS(SELECT 1 FROM \"{esc2}\".\"{t}\" \
                                 WHERE \"{col}\" IS NOT NULL)"
                            ),
                            [],
                            |r| r.get(0),
                        )
                        .store()?;
                    ensure_moved_source_empty(has, t, col, nt)?;
                }
            }

            for stmt in &destructive {
                tx.execute_batch(stmt)
                    .map_err(|e| StoreError::Other(drop_column_hint(stmt, &e.to_string())))?;
            }

            let meta = format!(
                "UPDATE \"{esc2}\".\"fhir_sqlite_meta\" SET \"value\" = ?2 WHERE \"key\" = ?1"
            );
            for (k, v) in [
                ("map_checksum", &checksum),
                ("fhir_version", &version),
                ("map_asset", &new_hex),
            ] {
                tx.execute(&meta, rusqlite::params![k, v]).store()?;
            }
            tx.commit().store()?;
            Ok(n_reshred)
        })
        .await
        .map_err(join_err)?;
        let reshredded = reshredded?;

        let folded = self.backfill_norm().await?;
        Ok(UpgradeReport {
            additive: n_add,
            destructive: n_drop,
            folded,
            reshredded,
        })
    }

    /// Diff the installed map against this store's, by name, across all
    /// resources. Returns the additive statements and the destructive ones.
    ///
    /// A column whose *type* changed is neither: it is an error. Widening
    /// `TEXT` to `TEXT` is not what these are — a type change means the shred
    /// writes a different value shape, and rewriting stored data is a migration
    /// somebody must design, not one a diff can infer (`L12`).
    fn diff_maps(
        &self,
        old_map: &RelMap,
        esc: &str,
    ) -> Result<(Vec<String>, Vec<String>), StoreError> {
        use std::collections::{HashMap, HashSet};
        let (mut adds, mut destructive) = (Vec::new(), Vec::new());
        let mut old_tables: HashMap<&str, &fhir_sqlite_map::model::Table> = HashMap::new();
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
                    adds.push(fhir_sqlite_map::ddl::create_table(esc, rm, t));
                    continue;
                };
                let old_cols: HashMap<&str, ColTy> =
                    old_t.cols.iter().map(|c| (c.name.as_str(), c.ty)).collect();
                let new_cols: HashSet<&str> = t.cols.iter().map(|c| c.name.as_str()).collect();
                for c in &t.cols {
                    match old_cols.get(c.name.as_str()) {
                        None => adds.push(format!(
                            "ALTER TABLE \"{esc}\".\"{}\" ADD COLUMN \"{}\" {}",
                            t.name,
                            c.name,
                            fhir_sqlite_map::ddl::col_sql(c.ty)
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
                            "ALTER TABLE \"{esc}\".\"{}\" DROP COLUMN \"{name}\"",
                            t.name
                        ));
                    }
                }
            }
        }
        for name in old_tables.keys() {
            if !new_names.contains(name) {
                destructive.push(format!("DROP TABLE \"{esc}\".\"{name}\""));
            }
        }
        // Indexes diff by full statement text, which catches a changed column
        // list as well as a new index.
        let old_ix: HashSet<String> = old_map
            .resources
            .values()
            .flat_map(|rm| fhir_sqlite_map::ddl::search_indexes(esc, rm))
            .collect();
        for rm in self.map.resources.values() {
            for stmt in fhir_sqlite_map::ddl::search_indexes(esc, rm) {
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
    /// existing row, and a string search compares the folded column — so those
    /// resources would silently stop matching. **Silent under-return is the one
    /// failure mode a clinical search must not have**, which is why this runs as
    /// part of the upgrade rather than as a step an operator can forget
    /// (`L13`, `L14`).
    ///
    /// Folds distinct *values* rather than rows — a surname repeats across
    /// patients — in bounded batches, and is **resumable**: each pass looks only
    /// at rows still NULL, so an interrupted run resumes where it stopped rather
    /// than restarting. That matters here more than on PostgreSQL, because a
    /// SQLite writer holds the single write lock for the length of its
    /// transaction.
    pub async fn backfill_norm(&self) -> Result<usize, StoreError> {
        const BATCH: usize = 1000;
        self.attach().await?;
        let esc = escape_ident(&self.map.schema);
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
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<usize, StoreError> {
            let mut c = conn.blocking_lock();
            let mut total = 0usize;
            for (tn, src, dst) in &work {
                loop {
                    let vals: Vec<String> = {
                        let mut stmt = c
                            .prepare(&format!(
                                "SELECT DISTINCT \"{src}\" FROM \"{esc}\".\"{tn}\" \
                                 WHERE \"{dst}\" IS NULL AND \"{src}\" IS NOT NULL \
                                 LIMIT {BATCH}"
                            ))
                            .store()?;
                        let rows = stmt.query_map([], |r| r.get::<_, String>(0)).store()?;
                        rows.collect::<Result<_, _>>().store()?
                    };
                    if vals.is_empty() {
                        break;
                    }
                    let n = vals.len();
                    // One transaction per batch, not one for the whole column:
                    // that is what makes an interrupted run resumable instead of
                    // rolled back to nothing.
                    let tx = c
                        .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                        .store()?;
                    {
                        let mut up = tx
                            .prepare(&format!(
                                "UPDATE \"{esc}\".\"{tn}\" SET \"{dst}\" = ?2 \
                                 WHERE \"{src}\" = ?1 AND \"{dst}\" IS NULL"
                            ))
                            .store()?;
                        for v in &vals {
                            up.execute(rusqlite::params![v, fhir_sqlite_map::fold::fold(v)])
                                .store()?;
                        }
                    }
                    tx.commit().store()?;
                    total += n;
                    if n < BATCH {
                        break;
                    }
                }
            }
            Ok(total)
        })
        .await
        .map_err(join_err)?
    }

    /// How many tables the installed schema has, for tests and diagnostics.
    pub async fn table_count(&self) -> Result<usize, StoreError> {
        self.attach().await?;
        let schema = self.map.schema.clone();
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<usize, StoreError> {
            let c = conn.blocking_lock();
            let n: i64 = c
                .query_row(
                    &format!(
                        "SELECT count(*) FROM \"{}\".sqlite_master WHERE type='table'",
                        escape_ident(&schema)
                    ),
                    [],
                    |r| r.get(0),
                )
                .store()?;
            Ok(usize::try_from(n).unwrap_or(0))
        })
        .await
        .map_err(join_err)?
    }

    /// Drop the schema by detaching and deleting its file.
    ///
    /// PostgreSQL needs `DROP SCHEMA … CASCADE` and pays for it in lock budget;
    /// here the whole schema is one file, so removing it is a single unlink and
    /// cannot half-succeed.
    pub async fn drop_schema(&self) -> Result<(), StoreError> {
        let schema = self.map.schema.clone();
        let file = self.attached_path(&schema);
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<(), StoreError> {
            let c = conn.blocking_lock();
            let attached: i64 = c
                .query_row(
                    "SELECT count(*) FROM pragma_database_list WHERE name = ?1",
                    [&schema],
                    |r| r.get(0),
                )
                .store()?;
            if attached > 0 {
                c.execute_batch(&format!("DETACH DATABASE \"{}\"", escape_ident(&schema)))
                    .store()?;
            }
            for suffix in ["", "-wal", "-shm"] {
                let p = if suffix.is_empty() {
                    file.clone()
                } else {
                    PathBuf::from(format!("{}{suffix}", file.display()))
                };
                if p.exists() {
                    std::fs::remove_file(&p)
                        .map_err(|e| StoreError::Other(format!("removing {}: {e}", p.display())))?;
                }
            }
            Ok(())
        })
        .await
        .map_err(join_err)?
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use fhir_sqlite_map::model::{Column, ResourceMap, Table};
    use rusqlite::types::{Value, ValueRef};

    use super::*;

    fn deny_reads_from(connection: &mut Connection, denied: &'static str) {
        use rusqlite::hooks::{AuthAction, AuthContext, Authorization};

        connection
            .authorizer(Some(move |ctx: AuthContext<'_>| match ctx.action {
                AuthAction::Read { table_name, .. } if table_name == denied => Authorization::Deny,
                AuthAction::Pragma { pragma_name, .. } if pragma_name == denied => {
                    Authorization::Deny
                }
                _ => Authorization::Allow,
            }))
            .unwrap();
    }

    fn allow_all(connection: &mut Connection) {
        use rusqlite::hooks::{AuthContext, Authorization};

        connection
            .authorizer(None::<fn(AuthContext<'_>) -> Authorization>)
            .unwrap();
    }

    fn scratch(name: &str) -> PathBuf {
        let path =
            std::env::temp_dir().join(format!("fhir-sqlite-unit-{name}-{}", std::process::id()));
        if path.exists() {
            std::fs::remove_dir_all(&path).unwrap();
        }
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn cell_rendering_and_ordinal_images_reject_corruption() {
        assert_eq!(
            cell_text(ValueRef::Null, ColTy::Text, 0, "x").unwrap(),
            None
        );
        assert_eq!(
            cell_text(ValueRef::Integer(0), ColTy::Bool, 0, "x").unwrap(),
            Some("false".into())
        );
        assert_eq!(
            cell_text(ValueRef::Integer(7), ColTy::Bool, 0, "x").unwrap(),
            Some("true".into())
        );
        for (raw, expected) in [
            ("true", "true"),
            ("t", "t"),
            ("false", "false"),
            ("f", "f"),
            ("1", "true"),
            ("0", "false"),
        ] {
            assert_eq!(
                cell_text(ValueRef::Text(raw.as_bytes()), ColTy::Bool, 2, "flag").unwrap(),
                Some(expected.into())
            );
        }
        assert!(cell_text(ValueRef::Text(b"yes"), ColTy::Bool, 2, "flag").is_err());
        assert!(cell_text(ValueRef::Text(&[0xff]), ColTy::Bool, 2, "flag").is_err());
        assert_eq!(
            cell_text(ValueRef::Integer(-4), ColTy::Int, 0, "x").unwrap(),
            Some("-4".into())
        );
        assert_eq!(
            cell_text(ValueRef::Real(1.25), ColTy::Numeric, 0, "x").unwrap(),
            Some("1.25".into())
        );
        assert_eq!(
            cell_text(ValueRef::Text(b"text"), ColTy::Text, 0, "x").unwrap(),
            Some("text".into())
        );
        assert!(cell_text(ValueRef::Text(&[0xff]), ColTy::Text, 3, "bad").is_err());
        assert!(cell_text(ValueRef::Blob(b"bytes"), ColTy::Text, 3, "bad").is_err());

        assert_eq!(fmt_ords(&[]), "{}");
        assert!(parse_ords("{}").unwrap().is_empty());
        assert_eq!(fmt_ords(&[1, -2, 3]), "{1,-2,3}");
        assert_eq!(parse_ords(" {1, -2,3} ").unwrap(), [1, -2, 3]);
        assert!(parse_ords("{1,nope}").is_err());
    }

    #[test]
    fn upgrade_parsers_and_encodings_are_lossless_and_diagnostic() {
        assert_eq!(escape_ident("a\"b"), "a\"\"b");
        assert_eq!(
            added_column_name("ALTER TABLE x ADD COLUMN \"audit_actor\" TEXT"),
            Some("audit_actor".into())
        );
        assert_eq!(added_column_name("CREATE TABLE x (a TEXT)"), None);
        let hinted = drop_column_hint("ALTER TABLE x DROP COLUMN y", "cannot drop");
        assert!(hinted.contains("table rebuilt"));
        assert!(hinted.contains("cannot drop"));
        assert_eq!(
            drop_column_hint("ALTER TABLE x ADD COLUMN y", "duplicate"),
            "upgrade: duplicate\nstatement was:\nALTER TABLE x ADD COLUMN y"
        );

        let bytes = [0, 1, 15, 16, 254, 255];
        let encoded = hex_encode(&bytes);
        assert_eq!(encoded, "00010f10feff");
        assert_eq!(hex_decode(&encoded).unwrap(), bytes);
        assert!(hex_decode("abc").is_err());
        assert!(hex_decode("zz").is_err());
        assert_eq!(map_asset_hex_with(|| Ok(vec![0, 255])).unwrap(), "00ff");
        let json_error = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        assert!(map_asset_hex_with(|| Err(json_error)).is_err());

        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            "ATTACH DATABASE ':memory:' AS r5; CREATE TABLE r5.sample (a TEXT, b INTEGER)",
        )
        .unwrap();
        let tx = db.unchecked_transaction().unwrap();
        let columns = installed_columns(&tx, "r5", "sample").unwrap();
        assert_eq!(
            columns,
            ["a".to_string(), "b".to_string()].into_iter().collect()
        );
    }

    #[test]
    fn time_leaf_and_sql_value_conversions_cover_every_storage_kind() {
        use fhir_sqlite_map::shred::SqlVal;
        use fhir_sqlite_map::value::LeafVal;

        assert_eq!(
            utc_micros(std::time::UNIX_EPOCH),
            "1970-01-01 00:00:00.000000"
        );
        assert_eq!(
            utc_micros(std::time::UNIX_EPOCH - std::time::Duration::from_secs(1)),
            "1970-01-01 00:00:00.000000"
        );
        assert_eq!(
            utc_micros(std::time::UNIX_EPOCH + std::time::Duration::new(951_868_799, 123_456_000)),
            "2000-02-29 23:59:59.123456"
        );
        assert_eq!(civil_from_days(-1), (1969, 12, 31));

        assert!(
            matches!(leaf_from_cols("s", Some("x".into()), None, None), LeafVal::Str(v) if v == "x")
        );
        assert!(matches!(
            leaf_from_cols("s", None, None, None),
            LeafVal::Str(v) if v.is_empty()
        ));
        assert!(
            matches!(leaf_from_cols("n", None, Some("1.20".into()), None), LeafVal::Num(v) if v == "1.20")
        );
        assert!(matches!(leaf_from_cols("n", None, None, None), LeafVal::Num(v) if v.is_empty()));
        assert!(matches!(
            leaf_from_cols("b", None, None, Some(1)),
            LeafVal::Bool(true)
        ));
        assert!(matches!(
            leaf_from_cols("b", None, None, None),
            LeafVal::Bool(false)
        ));
        assert!(matches!(
            leaf_from_cols("?", None, None, None),
            LeafVal::Null
        ));

        assert_eq!(sqlval(&SqlVal::Bool(true)), Value::Integer(1));
        assert_eq!(sqlval(&SqlVal::Int(-2)), Value::Integer(-2));
        for value in [
            SqlVal::Num("1.20".into()),
            SqlVal::Text("text".into()),
            SqlVal::Ts("2024-01-01T00:00:00Z".into()),
            SqlVal::Date("2024".into()),
            SqlVal::Jsonb("{}".into()),
        ] {
            assert!(matches!(sqlval(&value), Value::Text(_)));
        }
        assert_eq!(sqlval(&SqlVal::Bytes(vec![1, 2])), Value::Blob(vec![1, 2]));
        assert_eq!(opt_text(None), Value::Null);
        assert_eq!(opt_text(Some("x")), Value::Text("x".into()));
    }

    fn map_with_column(table: &str, column: &str, path: &str) -> RelMap {
        let resource = ResourceMap {
            name: "Patient".into(),
            tables: vec![Table {
                name: table.into(),
                kind: TableKind::Base,
                path: String::new(),
                cols: vec![Column {
                    name: column.into(),
                    ty: ColTy::Text,
                    path: path.into(),
                }],
                norm_cols: Vec::new(),
                adjunct_cols: Vec::new(),
            }],
            nodes: Vec::new(),
            root: 0,
            search: Vec::new(),
            path_bound: 0,
        };
        RelMap {
            fhir_version: "test".into(),
            schema: "r5".into(),
            resources: BTreeMap::from([("Patient".into(), resource)]),
        }
    }

    fn bundled_map(version: &str) -> Arc<RelMap> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../fhir-sqlite-map/assets")
            .join(format!("fhir-sqlite-relmap-{version}.json.gz"));
        let bytes = std::fs::read(path).expect("bundled relational map");
        Arc::new(RelMap::from_gz_bytes(&bytes).expect("valid bundled relational map"))
    }

    #[test]
    fn moved_columns_distinguish_relocation_from_removal_and_retention() {
        let old = map_with_column("patient", "family", "Patient.name.family");
        let moved = map_with_column("patient_name", "family", "Patient.name.family");
        assert_eq!(
            moved_columns(&old, &moved),
            [("patient".into(), "family".into(), "patient_name".into())]
        );
        assert!(moved_columns(&old, &old).is_empty());
        assert!(moved_columns(&old, &map_with_column("other", "x", "Patient.x")).is_empty());
        assert!(
            moved_columns(
                &map_with_column("patient", "family", ""),
                &map_with_column("patient_name", "family", "")
            )
            .is_empty()
        );

        let before = serde_json::json!({"resourceType": "Patient", "id": "p1", "active": true});
        let reordered = serde_json::json!({"active": true, "id": "p1", "resourceType": "Patient"});
        verify_reshred_identity("Patient", "p1", &before, &reordered).unwrap();
        let changed = serde_json::json!({"resourceType": "Patient", "id": "p1", "active": false});
        assert!(
            verify_reshred_identity("Patient", "p1", &before, &changed)
                .expect_err("changed content must roll the migration back")
                .to_string()
                .contains("Patient/p1")
        );
        ensure_moved_source_empty(0, "old", "value", "new").unwrap();
        assert!(
            ensure_moved_source_empty(1, "old", "value", "new")
                .expect_err("left-behind source data must roll the migration back")
                .to_string()
                .contains("old.value")
        );
    }

    #[tokio::test]
    async fn reshred_refuses_cross_resource_moves_and_base_table_renames() {
        let old = map_with_column("patient", "active", "Patient.active");

        let dir = scratch("reshred-missing-resource");
        let db = dir.join("fhir.sqlite");
        let installed = SqliteStore::open(&db, Arc::new(old.clone())).await.unwrap();
        installed.init("old").await.unwrap();
        drop(installed);

        let mut cross_resource = map_with_column("other", "active", "Patient.active");
        let mut other = cross_resource.resources.remove("Patient").unwrap();
        other.name = "Other".into();
        cross_resource.resources.insert("Other".into(), other);
        let upgraded = SqliteStore::open(&db, Arc::new(cross_resource))
            .await
            .unwrap();
        let err = upgraded
            .upgrade_with(
                "new",
                crate::UpgradeOpts {
                    allow_destructive: true,
                    reshred_moved: true,
                },
            )
            .await
            .expect_err("a moved path cannot silently change resource type");
        assert!(err.to_string().contains("does not carry the resource"));
        drop(upgraded);

        let dir = scratch("reshred-base-rename");
        let db = dir.join("fhir.sqlite");
        let installed = SqliteStore::open(&db, Arc::new(old)).await.unwrap();
        installed.init("old").await.unwrap();
        drop(installed);

        let renamed = map_with_column("patient_renamed", "active", "Patient.active");
        let upgraded = SqliteStore::open(&db, Arc::new(renamed)).await.unwrap();
        let err = upgraded
            .upgrade_with(
                "new",
                crate::UpgradeOpts {
                    allow_destructive: true,
                    reshred_moved: true,
                },
            )
            .await
            .expect_err("re-shred cannot infer a base-table rename");
        assert!(err.to_string().contains("base table renamed"));
    }

    #[test]
    fn batched_insert_checks_shape_quotes_identifiers_and_chunks() {
        let mut db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            "ATTACH DATABASE ':memory:' AS r5; CREATE TABLE r5.\"odd\"\"table\" (\"a\" TEXT, \"b\" INTEGER)",
        )
        .unwrap();
        let tx = db.transaction().unwrap();
        let cols = ["a".into(), "b".into()];
        let rows: Vec<Vec<Value>> = (0..451)
            .map(|i| vec![Value::Text(format!("row-{i}")), Value::Integer(i)])
            .collect();
        insert_rows(&tx, "r5", "odd\"table", &cols, &rows).unwrap();
        assert_eq!(
            tx.query_row("SELECT count(*) FROM r5.\"odd\"\"table\"", [], |r| r
                .get::<_, i64>(0))
                .unwrap(),
            451
        );
        insert_rows(&tx, "r5", "odd\"table", &cols, &[]).unwrap();
        assert!(insert_rows(&tx, "r5", "odd\"table", &cols, &[vec![Value::Null]]).is_err());
        assert!(
            insert_rows(
                &tx,
                "r5",
                "missing",
                &cols,
                &[vec![Value::Null, Value::Null]]
            )
            .is_err()
        );
    }

    #[test]
    fn lazy_query_helpers_report_binding_and_column_type_corruption() {
        let connection = Connection::open_in_memory().unwrap();

        let mut statement = connection.prepare("SELECT ?1").unwrap();
        assert!(query_search_ids(&mut statement, &[], "SELECT ?1").is_err());
        assert_eq!(
            query_search_ids(&mut statement, &["p1".into()], "SELECT ?1").unwrap(),
            ["p1"]
        );
        let mut statement = connection.prepare("SELECT X'00'").unwrap();
        assert!(query_search_ids(&mut statement, &[], "SELECT X'00'").is_err());

        let mut statement = connection.prepare("SELECT ?1, ?2").unwrap();
        assert!(query_reference_rows(&mut statement, &["Patient".into()]).is_err());
        let mut statement = connection.prepare("SELECT 'Patient', 'p1'").unwrap();
        assert_eq!(
            query_reference_rows(&mut statement, &[]).unwrap(),
            [("Patient".into(), "p1".into())]
        );
        let mut statement = connection.prepare("SELECT X'00', 'p1'").unwrap();
        assert!(query_reference_rows(&mut statement, &[]).is_err());

        assert_eq!(active_version(crate::ResourceStatus::Active(7)), Some(7));
        assert_eq!(active_version(crate::ResourceStatus::Deleted(8)), None);
        assert_eq!(active_version(crate::ResourceStatus::Unknown), None);

        let mut statement = connection
            .prepare(
                "SELECT 1, 'ts', '', '{\"resourceType\":\"Patient\"}' \
                 WHERE ?1 = 'p1'",
            )
            .unwrap();
        let history = query_history_entries(&mut statement, "p1").unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].op, '?');
        let mut statement = connection
            .prepare("SELECT 1, 'ts', 'C', NULL WHERE ?1 = ?2")
            .unwrap();
        assert!(query_history_entries(&mut statement, "p1").is_err());
        let mut statement = connection
            .prepare("SELECT X'00', 'ts', 'C', NULL WHERE ?1 = 'p1'")
            .unwrap();
        assert!(query_history_entries(&mut statement, "p1").is_err());
        let mut statement = connection
            .prepare("SELECT 1, 'ts', 'C', '{' WHERE ?1 = 'p1'")
            .unwrap();
        assert!(query_history_entries(&mut statement, "p1").is_err());

        let valid_access = "SELECT NULL, 'actor', NULL, NULL, 'read', 'Patient', 'p1', \
                            1, 'ok', 1, NULL WHERE ?1 = 'Patient' AND ?2 = 'p1'";
        let mut statement = connection.prepare(valid_access).unwrap();
        let access = query_access_records(&mut statement, "Patient", "p1").unwrap();
        assert_eq!(access.len(), 1);
        assert_eq!(access[0].audit.actor, "actor");
        let mut statement = connection
            .prepare(&format!("{valid_access} AND ?3 = 'extra'"))
            .unwrap();
        assert!(query_access_records(&mut statement, "Patient", "p1").is_err());
        let invalid_access = "SELECT NULL, X'00', NULL, NULL, 'read', 'Patient', 'p1', \
                              1, 'ok', 1, NULL WHERE ?1 = 'Patient' AND ?2 = 'p1'";
        let mut statement = connection.prepare(invalid_access).unwrap();
        assert!(query_access_records(&mut statement, "Patient", "p1").is_err());

        let mut statement = connection
            .prepare("SELECT 'Patient', 'p1', 1, 'key:00'")
            .unwrap();
        assert_eq!(query_countersigns(&mut statement).unwrap().len(), 1);
        let mut statement = connection
            .prepare("SELECT 'Patient', 'p1', 1, 'key:00' WHERE ?1")
            .unwrap();
        assert!(query_countersigns(&mut statement).is_err());
        let mut statement = connection
            .prepare("SELECT 'Patient', 'p1', X'00', 'key:00'")
            .unwrap();
        assert!(query_countersigns(&mut statement).is_err());

        let valid_audit = "SELECT 'p1', 1, 'ts', 'C', NULL, 'actor', NULL, NULL, \
                           NULL, NULL, NULL";
        let mut statement = connection.prepare(valid_audit).unwrap();
        assert_eq!(query_audit_rows(&mut statement).unwrap().len(), 1);
        let mut statement = connection
            .prepare(&format!("{valid_audit} WHERE ?1"))
            .unwrap();
        assert!(query_audit_rows(&mut statement).is_err());
        let mut statement = connection
            .prepare("SELECT 'p1', X'00', 'ts', 'C', NULL, 'actor', NULL, NULL, NULL, NULL, NULL")
            .unwrap();
        assert!(query_audit_rows(&mut statement).is_err());

        let columns = [("value".to_string(), ColTy::Text)];
        let mut statement = connection.prepare("SELECT 'text' WHERE ?1 = 'p1'").unwrap();
        let rows = query_recon_rows(&mut statement, "p1", &columns, false).unwrap();
        assert_eq!(rows[0].cols["value"], "text");
        let mut statement = connection
            .prepare("SELECT '{1}', 'text' WHERE ?1 = 'p1'")
            .unwrap();
        let rows = query_recon_rows(&mut statement, "p1", &columns, true).unwrap();
        assert_eq!(rows[0].ords, [1]);
        let mut statement = connection.prepare("SELECT 'text' WHERE ?1 = ?2").unwrap();
        assert!(query_recon_rows(&mut statement, "p1", &columns, false).is_err());
        let mut statement = connection.prepare("SELECT X'00' WHERE ?1 = 'p1'").unwrap();
        assert!(query_recon_rows(&mut statement, "p1", &columns, false).is_err());

        let ext_sql = "SELECT 'path', '{1}', 1, 2, 'url', 'leaf', 's', 'value', NULL, NULL \
                       WHERE ?1 = 'p1'";
        let mut statement = connection.prepare(ext_sql).unwrap();
        assert_eq!(query_ext_rows(&mut statement, "p1").unwrap().len(), 1);
        let mut statement = connection
            .prepare(&format!("{ext_sql} AND ?2 = 'extra'"))
            .unwrap();
        assert!(query_ext_rows(&mut statement, "p1").is_err());
        let mut statement = connection
            .prepare("SELECT X'00', '{1}', 1, 2, 'url', 'leaf', 's', 'value', NULL, NULL WHERE ?1 = 'p1'")
            .unwrap();
        assert!(query_ext_rows(&mut statement, "p1").is_err());

        let deep_sql = "SELECT 'path', '{1}', 'leaf', 's', 'value', NULL, NULL \
                        WHERE ?1 = 'p1'";
        let mut statement = connection.prepare(deep_sql).unwrap();
        assert_eq!(query_deep_rows(&mut statement, "p1").unwrap().len(), 1);
        let mut statement = connection
            .prepare(&format!("{deep_sql} AND ?2 = 'extra'"))
            .unwrap();
        assert!(query_deep_rows(&mut statement, "p1").is_err());
        let mut statement = connection
            .prepare("SELECT X'00', '{1}', 'leaf', 's', 'value', NULL, NULL WHERE ?1 = 'p1'")
            .unwrap();
        assert!(query_deep_rows(&mut statement, "p1").is_err());

        let mut statement = connection
            .prepare("SELECT 1, '{\"resourceType\":\"Patient\"}' WHERE ?1 = 'p1'")
            .unwrap();
        assert_eq!(query_contained_rows(&mut statement, "p1").unwrap().len(), 1);
        let mut statement = connection.prepare("SELECT 1, '{}' WHERE ?1 = ?2").unwrap();
        assert!(query_contained_rows(&mut statement, "p1").is_err());
        let mut statement = connection
            .prepare("SELECT X'00', '{}' WHERE ?1 = 'p1'")
            .unwrap();
        assert!(query_contained_rows(&mut statement, "p1").is_err());
        let mut statement = connection.prepare("SELECT 1, '{' WHERE ?1 = 'p1'").unwrap();
        assert!(query_contained_rows(&mut statement, "p1").is_err());
    }

    #[test]
    fn insert_plan_carries_every_auxiliary_row_kind() {
        use fhir_sqlite_map::shred::{DeepRow, ExtRow, Row, ShredOut, SqlVal};
        use fhir_sqlite_map::value::LeafVal;

        let table = |name: &str, kind: TableKind| Table {
            name: name.into(),
            kind,
            path: String::new(),
            cols: Vec::new(),
            norm_cols: Vec::new(),
            adjunct_cols: Vec::new(),
        };
        let rm = ResourceMap {
            name: "Patient".into(),
            tables: vec![
                table("patient", TableKind::Base),
                table("patient_name", TableKind::Elem),
                table("patient_ext", TableKind::Ext),
                table("patient_deep", TableKind::Deep),
                table("patient_contained", TableKind::Contained),
                table("patient_history", TableKind::History),
            ],
            nodes: Vec::new(),
            root: 0,
            search: Vec::new(),
            path_bound: 0,
        };
        let out = ShredOut {
            id: Some("p1".into()),
            rows: vec![
                Row {
                    table: 0,
                    ords: Vec::new(),
                    cols: vec![("active".into(), SqlVal::Bool(true))],
                },
                Row {
                    table: 1,
                    ords: vec![1],
                    cols: vec![("family".into(), SqlVal::Text("One".into()))],
                },
                Row {
                    table: 1,
                    ords: vec![2],
                    cols: vec![("given".into(), SqlVal::Text("Two".into()))],
                },
                Row {
                    table: 5,
                    ords: Vec::new(),
                    cols: Vec::new(),
                },
            ],
            ext: vec![ExtRow {
                path: "name".into(),
                ords: vec![1],
                modifier: true,
                ext_ord: 2,
                url: Some("urn:test".into()),
                leaf: "valueString".into(),
                val: LeafVal::Str("extension".into()),
            }],
            deep: vec![DeepRow {
                path: "recursive".into(),
                ords: vec![3],
                leaf: "valueBoolean".into(),
                val: LeafVal::Bool(true),
            }],
            contained: vec![(
                1,
                serde_json::json!({"resourceType": "Patient", "id": "c1"}),
            )],
        };
        let plan = InsertPlan::build(&rm, &out, "p1").unwrap();
        assert_eq!(plan.base_cols, ["active"]);
        assert_eq!(plan.batches.len(), 4);

        let mut db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            "ATTACH DATABASE ':memory:' AS r5;
             CREATE TABLE r5.patient (id TEXT PRIMARY KEY, version_id INTEGER, last_updated TEXT, active INTEGER);
             CREATE TABLE r5.patient_name (rid TEXT, ords TEXT, family TEXT, given TEXT);
             CREATE TABLE r5.patient_ext (rid TEXT, path TEXT, ords TEXT, modifier INTEGER, ext_ord INTEGER, url TEXT, leaf TEXT, v_kind TEXT, v_text TEXT, v_num TEXT, v_bool INTEGER);
             CREATE TABLE r5.patient_deep (rid TEXT, path TEXT, ords TEXT, leaf TEXT, v_kind TEXT, v_text TEXT, v_num TEXT, v_bool INTEGER);
             CREATE TABLE r5.patient_contained (rid TEXT, ord INTEGER, resource TEXT);",
        )
        .unwrap();
        let tx = db.transaction().unwrap();
        plan.apply(&tx, "r5", 7, "2026-11-19 12:00:00.000000")
            .unwrap();
        assert_eq!(
            tx.query_row("SELECT active FROM r5.patient WHERE id='p1'", [], |r| r
                .get::<_, i64>(0))
                .unwrap(),
            1
        );
        assert_eq!(
            tx.query_row("SELECT count(*) FROM r5.patient_name", [], |r| r
                .get::<_, i64>(0))
                .unwrap(),
            2
        );
        assert_eq!(
            tx.query_row("SELECT v_text FROM r5.patient_ext", [], |r| r
                .get::<_, String>(0))
                .unwrap(),
            "extension"
        );
        assert_eq!(
            tx.query_row("SELECT v_bool FROM r5.patient_deep", [], |r| r
                .get::<_, i64>(0))
                .unwrap(),
            1
        );
        assert!(
            tx.query_row("SELECT resource FROM r5.patient_contained", [], |r| r
                .get::<_, String>(0))
                .unwrap()
                .contains("\"c1\"")
        );
        tx.commit().unwrap();

        let input = fetch_recon_input(&db, "r5", &rm, "p1").unwrap();
        assert_eq!(input.tables[0].len(), 1);
        assert_eq!(input.tables[1].len(), 2);
        assert_eq!(input.ext.len(), 1);
        assert_eq!(input.deep.len(), 1);
        assert_eq!(input.contained.len(), 1);

        db.execute(
            "UPDATE r5.patient_contained SET resource = '{' WHERE rid = 'p1'",
            [],
        )
        .unwrap();
        assert!(
            fetch_recon_input(&db, "r5", &rm, "p1")
                .expect_err("invalid contained JSON is stored corruption")
                .to_string()
                .contains("contained")
        );
    }

    #[test]
    fn keyed_tag_verdicts_only_report_proven_mismatches() {
        let key = crate::chain::ChainKey::from_hex(
            "current",
            "1111111111111111111111111111111111111111111111111111111111111111",
        )
        .unwrap();
        let keys = crate::chain::KeyRing::new(vec![key.clone()]);
        let pre = b"canonical history row";
        let valid = crate::chain::mac(&key, None, pre);
        let mut breaks = Vec::new();
        let mut countersigns = HashMap::new();

        for stored in [
            Some(valid.as_str()),
            None,
            Some("malformed"),
            Some("retired:00"),
        ] {
            check_mac(
                &keys,
                &countersigns,
                "Patient",
                "p1",
                1,
                stored,
                None,
                pre,
                &mut breaks,
            );
        }
        assert!(breaks.is_empty());

        check_mac(
            &keys,
            &countersigns,
            "Patient",
            "p1",
            1,
            Some("current:00"),
            None,
            pre,
            &mut breaks,
        );
        assert_eq!(breaks.len(), 1);
        assert_eq!(breaks[0].algorithm, "hmac-sha256");

        countersigns.insert(("Patient".into(), "p2".into(), 2), valid);
        check_mac(
            &keys,
            &countersigns,
            "Patient",
            "p2",
            2,
            None,
            None,
            pre,
            &mut breaks,
        );
        assert_eq!(breaks.len(), 1);

        countersigns.insert(("Patient".into(), "p3".into(), 3), "active:00".into());
        check_mac(
            &keys,
            &countersigns,
            "Patient",
            "p3",
            3,
            None,
            None,
            pre,
            &mut breaks,
        );
        assert_eq!(breaks.len(), 1, "a bad countersign cannot invent a break");
    }

    #[tokio::test]
    async fn history_target_requires_both_a_known_resource_and_history_table() {
        let map = Arc::new(map_with_column("patient", "active", "Patient.active"));
        let dir = scratch("history-target");
        let store = SqliteStore {
            path: dir.join("fhir.sqlite"),
            map,
            conn: Arc::new(tokio::sync::Mutex::new(
                Connection::open_in_memory().unwrap(),
            )),
            write_gate: Arc::new(tokio::sync::Mutex::new(())),
            keys: crate::chain::KeyRing::default(),
        };
        assert!(matches!(
            store.hist_target("Missing"),
            Err(StoreError::Unsupported(_))
        ));
        assert!(
            store
                .hist_target("Patient")
                .expect_err("minimal test map has no history table")
                .to_string()
                .contains("no history table")
        );
        assert!(store.history_canon("Patient", "p1", 1).await.is_err());
        {
            let c = store.conn.lock().await;
            c.execute_batch(
                "CREATE TABLE r5.fhir_sqlite_countersign (\
                 rtype TEXT, id TEXT, version_id INTEGER, row_mac TEXT)",
            )
            .unwrap();
        }
        assert!(store.verify_audit().await.unwrap().is_empty());

        let mut no_history = (*bundled_map("r5")).clone();
        no_history.resources.retain(|name, _| name == "Patient");
        no_history
            .resources
            .get_mut("Patient")
            .unwrap()
            .tables
            .retain(|table| table.kind != TableKind::History);
        let no_history = SqliteStore::open(dir.join("no-history.sqlite"), Arc::new(no_history))
            .await
            .unwrap();
        no_history.init("no-history").await.unwrap();
        no_history.upgrade("no-history-again", false).await.unwrap();
        let patient = serde_json::json!({
            "resourceType": "Patient",
            "id": "no-history",
            "active": true
        });
        assert!(
            no_history
                .put(&patient, &crate::Audit::default())
                .await
                .expect_err("put requires an audit-history table")
                .to_string()
                .contains("has no history table")
        );
    }

    #[tokio::test]
    async fn blocking_task_panics_become_store_errors() {
        let err = tokio::task::spawn_blocking(|| panic!("intentional test panic"))
            .await
            .expect_err("task must panic");
        assert!(join_err(err).to_string().contains("blocking task failed"));
        assert!(matches!(
            sqlite_err(rusqlite::Error::InvalidQuery),
            StoreError::Other(message) if !message.is_empty()
        ));
        assert_eq!(Ok::<_, rusqlite::Error>(7).store().unwrap(), 7);
        assert!(Err::<(), _>(rusqlite::Error::InvalidQuery).store().is_err());
    }

    #[tokio::test]
    async fn store_lifecycle_accessors_and_empty_work_paths_are_observable() {
        let dir = scratch("lifecycle");
        let path = dir.join("nested").join("clinical.sqlite");
        let mut map = map_with_column("patient", "active", "Patient.active");
        map.resources.clear();
        let map = Arc::new(map);
        let store = SqliteStore::open(&path, Arc::clone(&map)).await.unwrap();
        let memory = SqliteStore::open(":memory:", Arc::clone(&map))
            .await
            .unwrap();
        drop(memory);

        assert_eq!(store.path(), path);
        assert_eq!(store.schema(), "r5");
        assert!(Arc::ptr_eq(store.map(), &map));
        assert_eq!(store.chain_key_id(), None);
        assert_eq!(
            store.attached_path("r5"),
            dir.join("nested/clinical-r5.sqlite")
        );
        assert_eq!(store.backfill_norm().await.unwrap(), 0);
        assert_eq!(store.installed_checksum().await.unwrap(), None);
        assert_eq!(store.table_count().await.unwrap(), 0);
        store.ping().await.unwrap();

        let keyed = store.with_chain_keys(crate::chain::KeyRing::new(vec![
            crate::chain::ChainKey::from_hex(
                "active",
                "1111111111111111111111111111111111111111111111111111111111111111",
            )
            .unwrap(),
        ]));
        assert_eq!(keyed.chain_key_id().as_deref(), Some("active"));
        keyed.drop_schema().await.unwrap();
        keyed.drop_schema().await.unwrap();

        let fallback = SqliteStore {
            path: PathBuf::from("/"),
            map,
            conn: Arc::new(tokio::sync::Mutex::new(
                Connection::open_in_memory().unwrap(),
            )),
            write_gate: Arc::new(tokio::sync::Mutex::new(())),
            keys: crate::chain::KeyRing::default(),
        };
        assert_eq!(
            fallback.attached_path("r5"),
            PathBuf::from("/fhir-r5.sqlite")
        );
    }

    #[tokio::test]
    async fn open_and_schema_helpers_surface_real_storage_errors() {
        let dir = scratch("errors");
        let blocker = dir.join("not-a-directory");
        std::fs::write(&blocker, b"file").unwrap();
        let map = Arc::new(map_with_column("patient", "active", "Patient.active"));
        let err = SqliteStore::open(blocker.join("store.sqlite"), Arc::clone(&map))
            .await
            .err()
            .expect("a file cannot be used as a parent directory");
        assert!(!err.to_string().is_empty());
        assert!(SqliteStore::open(&dir, map).await.is_err());

        let mut db = Connection::open_in_memory().unwrap();
        db.execute_batch("ATTACH DATABASE ':memory:' AS r5")
            .unwrap();
        let tx = db.transaction().unwrap();
        assert!(installed_columns(&tx, "bad\"schema", "table").is_err());

        let mut denied = Connection::open_in_memory().unwrap();
        deny_reads_from(&mut denied, "journal_mode");
        assert!(configure_connection(denied).is_err());

        let reused = scratch("reused");
        std::fs::write(reused.join("old"), b"old").unwrap();
        let reused = scratch("reused");
        assert!(!reused.join("old").exists());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn corrupted_schema_objects_surface_specific_read_and_audit_errors() {
        let dir = scratch("fault-injection");
        let store = SqliteStore::open(dir.join("fhir.sqlite"), bundled_map("r5"))
            .await
            .unwrap();
        store.init("fault-injection").await.unwrap();

        {
            let mut c = store.conn.lock().await;
            deny_reads_from(&mut c, "sqlite_master");
        }
        assert!(store.installed_checksum().await.is_err());
        assert!(store.table_count().await.is_err());
        assert!(store.upgrade("denied-metadata", false).await.is_err());
        {
            let mut c = store.conn.lock().await;
            allow_all(&mut c);
        }

        let mut probe_map = (*bundled_map("r5")).clone();
        probe_map.resources.clear();
        let probe = SqliteStore::open(dir.join("probe.sqlite"), Arc::new(probe_map))
            .await
            .unwrap();
        probe.attach().await.unwrap();
        {
            let mut c = probe.conn.lock().await;
            deny_reads_from(&mut c, "pragma_database_list");
        }
        assert!(probe.attach().await.is_err());
        assert!(probe.drop_schema().await.is_err());
        {
            let mut c = probe.conn.lock().await;
            allow_all(&mut c);
        }
        probe.drop_schema().await.unwrap();

        // An old audit envelope with one missing column must attempt a real
        // additive repair. Denying ALTER proves the repair failure is
        // contextual and transactional; removing the denial then heals it.
        {
            let c = store.conn.lock().await;
            c.execute_batch("ALTER TABLE r5.patient_history DROP COLUMN reason")
                .unwrap();
        }
        {
            use rusqlite::hooks::{AuthAction, AuthContext, Authorization};
            let mut c = store.conn.lock().await;
            c.authorizer(Some(|ctx: AuthContext<'_>| match ctx.action {
                AuthAction::AlterTable {
                    table_name: "patient_history",
                    ..
                } => Authorization::Deny,
                _ => Authorization::Allow,
            }))
            .unwrap();
        }
        assert!(store.upgrade("denied-audit-column", false).await.is_err());
        {
            let mut c = store.conn.lock().await;
            allow_all(&mut c);
        }
        store.upgrade("restored-audit-column", false).await.unwrap();

        // Reconciliation is also inside the upgrade transaction. Remove one
        // schema-wide object, deny its recreation, then prove a retry heals it.
        {
            let c = store.conn.lock().await;
            c.execute_batch("DROP TABLE r5.fhir_sqlite_access_log")
                .unwrap();
        }
        {
            use rusqlite::hooks::{AuthAction, AuthContext, Authorization};
            let mut c = store.conn.lock().await;
            c.authorizer(Some(|ctx: AuthContext<'_>| match ctx.action {
                AuthAction::CreateTable {
                    table_name: "fhir_sqlite_access_log",
                } => Authorization::Deny,
                _ => Authorization::Allow,
            }))
            .unwrap();
        }
        assert!(store.upgrade("denied-reconcile", false).await.is_err());
        {
            let mut c = store.conn.lock().await;
            allow_all(&mut c);
        }
        store.upgrade("restored-reconcile", false).await.unwrap();

        let mut reduced_map = (*bundled_map("r5")).clone();
        reduced_map.resources.remove("Basic").unwrap();
        let reduced = SqliteStore::open(dir.join("fhir.sqlite"), Arc::new(reduced_map))
            .await
            .unwrap();
        {
            use rusqlite::hooks::{AuthAction, AuthContext, Authorization};
            let mut c = reduced.conn.lock().await;
            c.authorizer(Some(|ctx: AuthContext<'_>| match ctx.action {
                AuthAction::DropTable { .. } => Authorization::Deny,
                _ => Authorization::Allow,
            }))
            .unwrap();
        }
        assert!(reduced.upgrade("denied-drop", true).await.is_err());
        {
            let mut c = reduced.conn.lock().await;
            allow_all(&mut c);
        }
        drop(reduced);

        let mut moved_map = (*bundled_map("r5")).clone();
        let patient_map = moved_map.resources.get_mut("Patient").unwrap();
        let position = patient_map.tables[0]
            .cols
            .iter()
            .position(|column| column.name == "multiple_birth_boolean")
            .unwrap();
        let column = patient_map.tables[0].cols.remove(position);
        patient_map.tables.push(Table {
            name: "patient_multiple_birth_moved".into(),
            kind: TableKind::Elem,
            path: "Patient.multipleBirth".into(),
            cols: vec![column],
            norm_cols: Vec::new(),
            adjunct_cols: Vec::new(),
        });
        let moved = SqliteStore::open(dir.join("fhir.sqlite"), Arc::new(moved_map))
            .await
            .unwrap();
        {
            let mut c = moved.conn.lock().await;
            deny_reads_from(&mut c, "patient");
        }
        assert!(moved.upgrade("denied-moved-read", true).await.is_err());
        {
            let mut c = moved.conn.lock().await;
            allow_all(&mut c);
        }
        drop(moved);

        let patient = serde_json::json!({
            "resourceType": "Patient",
            "id": "fault-patient",
            "active": true,
            "managingOrganization": {"reference": "Organization/fault-org"}
        });
        store.put(&patient, &crate::Audit::default()).await.unwrap();
        let orphan = serde_json::json!({
            "resourceType": "Patient",
            "id": "missing-history",
            "active": false
        });
        store.put(&orphan, &crate::Audit::default()).await.unwrap();

        assert!(matches!(
            store.history_canon("Missing", "x", 1).await,
            Err(StoreError::Unsupported(_))
        ));

        {
            let c = store.conn.lock().await;
            c.execute(
                "INSERT INTO r5.fhir_sqlite_countersign \
                 (rtype,id,version_id,row_mac,signed_at,actor,reason) \
                 VALUES ('Patient','fault-patient',1,'retired:00','2024-01-01T00:00:00.000000Z','tester','coverage')",
                [],
            )
            .unwrap();
            c.execute(
                "INSERT INTO r5.fhir_sqlite_erasure (token) VALUES ('fault-injection')",
                [],
            )
            .unwrap();
            c.execute(
                "DELETE FROM r5.patient_history WHERE id='missing-history'",
                [],
            )
            .unwrap();
            c.execute(
                "DELETE FROM r5.fhir_sqlite_erasure WHERE token='fault-injection'",
                [],
            )
            .unwrap();
        }
        assert_eq!(
            store
                .delete("Patient", "missing-history", &crate::Audit::default())
                .await
                .unwrap(),
            Some(1),
            "a damaged store with a live row but no history starts a fresh tombstone chain"
        );
        let subscriber = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::TRACE)
            .finish();
        let _guard = tracing::subscriber::set_default(subscriber);
        let mut direct_breaks = Vec::new();
        let preimage = b"coverage-preimage";
        check_mac(
            &crate::chain::KeyRing::default(),
            &HashMap::new(),
            "Patient",
            "unknown-key",
            1,
            Some("retired:00"),
            None,
            preimage,
            &mut direct_breaks,
        );
        check_mac(
            &crate::chain::KeyRing::default(),
            &HashMap::new(),
            "Patient",
            "malformed-tag",
            1,
            Some("not-a-tag"),
            None,
            preimage,
            &mut direct_breaks,
        );
        assert!(direct_breaks.is_empty());
        assert!(store.verify_audit().await.unwrap().is_empty());
        store.emit_checkpoint("clean").await;

        {
            let c = store.conn.lock().await;
            c.execute(
                "INSERT INTO r5.fhir_sqlite_countersign \
                 (rtype,id,version_id,row_mac,signed_at,actor,reason) \
                 VALUES ('Patient','bad-version','not-an-integer','retired:00','2024-01-01T00:00:00.000000Z','tester','coverage')",
                [],
            )
            .unwrap();
        }
        assert!(store.verify_audit().await.is_err());
        {
            let c = store.conn.lock().await;
            c.execute(
                "DELETE FROM r5.fhir_sqlite_countersign WHERE id='bad-version'",
                [],
            )
            .unwrap();
        }

        {
            let c = store.conn.lock().await;
            let trigger: String = c
                .query_row(
                    "SELECT name FROM r5.sqlite_master WHERE type='trigger' \
                     AND tbl_name='patient_history' AND name LIKE '%upd%'",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            c.execute_batch(&format!("DROP TRIGGER r5.\"{}\"", escape_ident(&trigger)))
                .unwrap();
            c.execute(
                "UPDATE r5.patient_history SET resource = replace(resource, '\"active\":true', '\"active\":false') \
                 WHERE id='fault-patient' AND version_id=1",
                [],
            )
            .unwrap();
        }
        assert!(!store.verify_audit().await.unwrap().is_empty());
        store.emit_checkpoint("tampered").await;

        {
            let c = store.conn.lock().await;
            c.execute_batch("DROP TABLE r5.fhir_sqlite_countersign")
                .unwrap();
        }
        assert!(store.verify_audit().await.is_err());
        store.emit_checkpoint("missing-countersigns").await;

        {
            let c = store.conn.lock().await;
            c.execute_batch("DROP TABLE r5.patient").unwrap();
        }
        assert!(store.get("Patient", "fault-patient").await.is_err());
        assert!(store.put(&patient, &crate::Audit::default()).await.is_err());
        assert!(
            store
                .delete("Patient", "fault-patient", &crate::Audit::default())
                .await
                .is_err()
        );
        let params = vec![("active".to_string(), "true".to_string())];
        assert!(
            store
                .search_full("Patient", &params, 10, 0, &[], true)
                .await
                .expect_err("count must fail after the base table is removed")
                .to_string()
                .contains("count:")
        );
        assert!(
            store
                .search_full("Patient", &params, 10, 0, &[], false)
                .await
                .expect_err("prepare must fail after the base table is removed")
                .to_string()
                .contains("preparing search:")
        );

        {
            let c = store.conn.lock().await;
            c.execute_batch("DROP TABLE r5.fhir_sqlite_access_log")
                .unwrap();
        }
        assert!(
            store
                .access_log_for("Patient", "fault-patient")
                .await
                .is_err()
        );
    }
}

/// Double any embedded quote, so an identifier cannot terminate its own quoting.
///
/// Schema names come from the generator, not from a request, so this is
/// belt-and-braces — but an identifier that is interpolated rather than bound is
/// exactly the shape that later becomes an injection when someone makes it
/// configurable.
fn escape_ident(s: &str) -> String {
    s.replace('"', "\"\"")
}

fn sqlite_err(e: rusqlite::Error) -> StoreError {
    StoreError::Other(e.to_string())
}

/// Apply the durability and concurrency contract to a newly opened handle.
///
/// Kept separate from filesystem opening so each pragma failure is testable
/// with SQLite's authorizer hook; otherwise `open` can only observe failures
/// that happen before a connection exists.
fn configure_connection(c: Connection) -> Result<Connection, StoreError> {
    c.pragma_update(None, "foreign_keys", "ON").store()?;
    // Returns a row, so it is a query rather than an update.
    let _: String = c
        .query_row("PRAGMA journal_mode=WAL", [], |r| r.get(0))
        .store()?;
    c.pragma_update(None, "busy_timeout", 30_000).store()?;
    c.pragma_update(None, "synchronous", "FULL").store()?;
    Ok(c)
}

/// Convert a driver result without repeating an error-only closure at every
/// call site. Besides keeping the adapter in one testable place, this avoids
/// LLVM attributing otherwise-successful statements solely to the closure's
/// never-taken error edge.
trait SqliteResultExt<T> {
    fn store(self) -> Result<T, StoreError>;
}

impl<T> SqliteResultExt<T> for rusqlite::Result<T> {
    fn store(self) -> Result<T, StoreError> {
        self.map_err(sqlite_err)
    }
}

fn join_err(e: tokio::task::JoinError) -> StoreError {
    StoreError::Other(format!("blocking task failed: {e}"))
}

/// The columns a table actually has, for the upgrade path's envelope diff.
///
/// `pragma_table_info` is the table-valued form, so it can be queried with the
/// schema qualified — the `PRAGMA` statement form cannot, and would silently
/// report on `main` instead, which on a multi-version database is a different
/// FHIR release's tables.
fn installed_columns(
    tx: &rusqlite::Transaction<'_>,
    schema: &str,
    table: &str,
) -> Result<std::collections::HashSet<String>, StoreError> {
    let mut stmt = tx
        .prepare(&format!(
            "SELECT name FROM \"{schema}\".pragma_table_info(?1)"
        ))
        .store()?;
    let rows = stmt.query_map([table], |r| r.get::<_, String>(0)).store()?;
    rows.collect::<Result<_, _>>().store()
}

/// The column name in an `ALTER TABLE … ADD COLUMN "name" …` statement.
///
/// Parsing back what was just formatted is not elegant, but the alternative is
/// for `ddl::history_audit_columns` to return structure the other five ports do
/// not need. `None` means the statement was not the shape expected, and the
/// caller then applies it rather than skipping it — a redundant `ADD COLUMN`
/// fails loudly, whereas a skipped one loses an audit column silently.
fn added_column_name(stmt: &str) -> Option<String> {
    let rest = stmt.split_once(" ADD COLUMN \"")?.1;
    rest.split_once('"').map(|(name, _)| name.to_string())
}

/// SQLite reports every `DROP COLUMN` precondition as a bare `SQLITE_ERROR`.
///
/// It refuses to drop a column that is a primary key, is indexed, is named in a
/// trigger, or carries a `UNIQUE` constraint — all of which the generated
/// schema uses. An operator who ran with `--allow-destructive` and got
/// "SQL logic error" has no way to tell that from a bug in this code.
fn drop_column_hint(stmt: &str, err: &str) -> String {
    if stmt.contains(" DROP COLUMN ") {
        format!(
            "upgrade: {err}\nstatement was:\n{stmt}\n\
             SQLite cannot drop a column that is indexed, part of the primary key, \
             or referenced by a trigger. Dropping it needs the table rebuilt, which \
             is a migration to design rather than one to infer."
        )
    } else {
        format!("upgrade: {err}\nstatement was:\n{stmt}")
    }
}

/// The stored map asset is gzip, and the meta table's `value` is `TEXT`. Hex
/// rather than base64 so the encoding matches `fhir-postgresql` byte for byte
/// and a map asset can be lifted from one port's meta table into another's.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn map_asset_hex(map: &RelMap) -> Result<String, StoreError> {
    map_asset_hex_with(|| map.to_gz_bytes())
}

fn map_asset_hex_with(
    serialize: impl FnOnce() -> Result<Vec<u8>, serde_json::Error>,
) -> Result<String, StoreError> {
    serialize()
        .map(|bytes| hex_encode(&bytes))
        .map_err(|error| StoreError::Other(error.to_string()))
}

fn verify_reshred_identity(
    rtype: &str,
    id: &str,
    before: &serde_json::Value,
    after: &serde_json::Value,
) -> Result<(), StoreError> {
    if fhir_sqlite_map::canon::canonicalize(before) != fhir_sqlite_map::canon::canonicalize(after) {
        return Err(StoreError::Other(format!(
            "re-shred verification failed for {rtype}/{id}: the new-shape reconstruction is not \
             byte-identical; the transaction is rolled back"
        )));
    }
    Ok(())
}

fn ensure_moved_source_empty(
    has_data: i64,
    table: &str,
    column: &str,
    destination: &str,
) -> Result<(), StoreError> {
    if has_data != 0 {
        return Err(StoreError::Other(format!(
            "re-shred left data behind in {table}.{column} (destined for {destination}); the \
             transaction is rolled back"
        )));
    }
    Ok(())
}

/// Everything `reconstruct` needs for one resource, read under `rm` — the
/// caller's choice of map, not necessarily the store's own. `get` reads
/// under the bundled map; `O10.4c`'s re-shred migration reads under the
/// *stored* old map, which is the whole point of the extraction.
fn fetch_recon_input(
    c: &rusqlite::Connection,
    s: &str,
    rm: &fhir_sqlite_map::model::ResourceMap,
    id: &str,
) -> Result<fhir_sqlite_map::reconstruct::ReconIn, StoreError> {
    use fhir_sqlite_map::model::TableKind;
    use fhir_sqlite_map::reconstruct::ReconIn;
    let mut input = ReconIn {
        tables: vec![Vec::new(); rm.tables.len()],
        ..Default::default()
    };

    for (ti, t) in rm.tables.iter().enumerate() {
        match t.kind {
            TableKind::Base | TableKind::Elem => {
                // Carry each column's declared type, not just its name:
                // rendering a cell back to its text image needs it
                // (see `cell_text`).
                let cols: Vec<(String, ColTy)> =
                    t.cols.iter().map(|c| (c.name.clone(), c.ty)).collect();
                let (key_col, want_ords) = match t.kind {
                    TableKind::Base => ("id", false),
                    _ => ("rid", true),
                };
                let mut select: Vec<String> = Vec::new();
                if want_ords {
                    select.push("\"ords\"".to_string());
                }
                select.extend(cols.iter().map(|(c, _)| format!("\"{c}\"")));
                // A table with no data columns still has rows worth
                // reading: their existence is what says the element was
                // present at all.
                if select.is_empty() {
                    select.push("NULL".to_string());
                }
                let sql = format!(
                    "SELECT {} FROM \"{s}\".\"{}\" WHERE \"{key_col}\" = ?1",
                    select.join(", "),
                    t.name
                );
                let mut st = c.prepare(&sql).store()?;
                input.tables[ti] = query_recon_rows(&mut st, id, &cols, want_ords)?;
            }
            TableKind::Ext => {
                let mut st = c
                    .prepare(&format!(
                        "SELECT \"path\", \"ords\", \"modifier\", \"ext_ord\", \"url\", \
                                \"leaf\", \"v_kind\", \"v_text\", \"v_num\", \"v_bool\" \
                         FROM \"{s}\".\"{}\" WHERE \"rid\" = ?1",
                        t.name
                    ))
                    .store()?;
                input.ext.extend(query_ext_rows(&mut st, id)?);
            }
            TableKind::Deep => {
                let mut st = c
                    .prepare(&format!(
                        "SELECT \"path\", \"ords\", \"leaf\", \"v_kind\", \"v_text\", \
                                \"v_num\", \"v_bool\" \
                         FROM \"{s}\".\"{}\" WHERE \"rid\" = ?1",
                        t.name
                    ))
                    .store()?;
                input.deep.extend(query_deep_rows(&mut st, id)?);
            }
            TableKind::Contained => {
                let mut st = c
                    .prepare(&format!(
                        "SELECT \"ord\", \"resource\" FROM \"{s}\".\"{}\" WHERE \"rid\" = ?1",
                        t.name
                    ))
                    .store()?;
                input.contained.extend(query_contained_rows(&mut st, id)?);
            }
            TableKind::History => {}
        }
    }

    Ok(input)
}

fn query_recon_rows(
    statement: &mut rusqlite::Statement<'_>,
    id: &str,
    columns: &[(String, ColTy)],
    want_ords: bool,
) -> Result<Vec<fhir_sqlite_map::reconstruct::InRow>, StoreError> {
    let rows = statement
        .query_map([id], |row| {
            let mut ords = Vec::new();
            let mut offset = 0usize;
            if want_ords {
                let image: String = row.get(0)?;
                ords = parse_ords(&image).unwrap_or_default();
                offset = 1;
            }
            let mut values = std::collections::HashMap::new();
            for (index, (name, ty)) in columns.iter().enumerate() {
                let index = index + offset;
                if let Some(value) = cell_text(row.get_ref(index)?, *ty, index, name)? {
                    values.insert(name.clone(), value);
                }
            }
            Ok(fhir_sqlite_map::reconstruct::InRow { ords, cols: values })
        })
        .store()?;
    rows.collect::<Result<_, _>>().store()
}

fn query_ext_rows(
    statement: &mut rusqlite::Statement<'_>,
    id: &str,
) -> Result<Vec<fhir_sqlite_map::shred::ExtRow>, StoreError> {
    let rows = statement
        .query_map([id], |row| {
            Ok(fhir_sqlite_map::shred::ExtRow {
                path: row.get(0)?,
                ords: parse_ords(&row.get::<_, String>(1)?).unwrap_or_default(),
                modifier: row.get::<_, i64>(2)? != 0,
                ext_ord: row.get(3)?,
                url: row.get(4)?,
                leaf: row.get(5)?,
                val: leaf_from_cols(
                    &row.get::<_, String>(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get::<_, Option<i64>>(9)?,
                ),
            })
        })
        .store()?;
    rows.collect::<Result<_, _>>().store()
}

fn query_deep_rows(
    statement: &mut rusqlite::Statement<'_>,
    id: &str,
) -> Result<Vec<fhir_sqlite_map::shred::DeepRow>, StoreError> {
    let rows = statement
        .query_map([id], |row| {
            Ok(fhir_sqlite_map::shred::DeepRow {
                path: row.get(0)?,
                ords: parse_ords(&row.get::<_, String>(1)?).unwrap_or_default(),
                leaf: row.get(2)?,
                val: leaf_from_cols(
                    &row.get::<_, String>(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get::<_, Option<i64>>(6)?,
                ),
            })
        })
        .store()?;
    rows.collect::<Result<_, _>>().store()
}

fn query_contained_rows(
    statement: &mut rusqlite::Statement<'_>,
    id: &str,
) -> Result<Vec<(i16, serde_json::Value)>, StoreError> {
    let rows = statement
        .query_map([id], |row| {
            Ok((row.get::<_, i16>(0)?, row.get::<_, String>(1)?))
        })
        .store()?;
    let mut values = Vec::new();
    for row in rows {
        let (ordinal, raw) = row.store()?;
        let value = serde_json::from_str(&raw)
            .map_err(|error| StoreError::Other(format!("contained: {error}")))?;
        values.push((ordinal, value));
    }
    Ok(values)
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

// ---------------------------------------------------------------- write & read

/// What a `put` did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutKind {
    Created,
    Updated,
}

/// Outcome of a `put`: which resource, at which version, and whether it existed.
#[derive(Debug, Clone)]
pub struct Put {
    pub id: String,
    pub version_id: i64,
    pub kind: PutKind,
}

impl SqliteStore {
    /// Store a resource, appending a history row with its hash chain.
    ///
    /// One `BEGIN IMMEDIATE` transaction covers the whole operation. That is
    /// both the isolation and the serialization: SQLite admits a single writer,
    /// so taking the write lock up front is what `pg_advisory_xact_lock` and
    /// `SELECT … FOR UPDATE` were buying in the PostgreSQL store (M14.18).
    ///
    /// A rewrite deletes the base row first and lets `ON DELETE CASCADE` clear
    /// the children, which is why `foreign_keys=ON` in `open` is load-bearing
    /// rather than hygiene. History is untouched by that delete — it has no
    /// foreign key to the base table, precisely so that it survives.
    pub async fn put(
        &self,
        resource: &serde_json::Value,
        audit: &crate::Audit,
    ) -> Result<Put, StoreError> {
        self.attach().await?;

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
        // it as a string erases that, and every bad submission then looks like
        // an internal fault — a 500 where the truth is a 400 (audit F-23).
        let out = fhir_sqlite_map::shred::shred(rm, resource)?;
        let id = out
            .id
            .clone()
            .ok_or_else(|| StoreError::Other("resource has no id".into()))?;

        // The bytes the chain commits to are computed *here*, in Rust, not by
        // the database (M14.15/spec 14). The PostgreSQL store asked the server
        // for `(($1::text)::jsonb)::text`; no other engine reproduces those
        // bytes, so a chain written by one dialect could never be verified by
        // another. `canon::canonicalize` is dialect-independent by construction.
        let canon = fhir_sqlite_map::canon::canonicalize(resource);

        // Rendered here too, and in UTC: the PostgreSQL store took pains to
        // avoid the session time zone leaking into the hashed image, and a
        // verifier in another zone recomputing different bytes would report
        // every row as broken.
        let now = std::time::SystemTime::now();
        let ts = crate::sqlite::utc_micros(now);

        let plan = InsertPlan::build(rm, &out, &id)?;
        let schema = self.map.schema.clone();
        let hist = rm
            .find_table(fhir_sqlite_map::model::TableKind::History)
            .map(|(_, t)| t.name.clone())
            .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?;
        let base = rm.base_table().name.clone();
        let audit = audit.clone();
        let signing = self.keys.signing().cloned();
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<Put, StoreError> {
            let mut c = conn.blocking_lock();
            let tx = c
                .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                .store()?;
            let s = escape_ident(&schema);

            // Previous version and chain tips, from history rather than the base
            // row: history is the record that cannot be deleted, so it is the
            // authority on what version number comes next even for a resource
            // that was deleted and is being recreated.
            let prev: Option<ChainTip> = tx
                .query_row(
                    &format!(
                        "SELECT \"version_id\", \"row_hash\", \"row_hash_sha3\" FROM \"{s}\".\"{hist}\" \
                         WHERE \"id\" = ?1 ORDER BY \"version_id\" DESC LIMIT 1"
                    ),
                    [&id],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
                )
                .ok();
            let (version_id, prev_sha256, prev_sha3) = match prev {
                Some((v, a, b)) => (v + 1, a, b),
                None => (1, None, None),
            };

            let existed: i64 = tx
                .query_row(
                    &format!("SELECT count(*) FROM \"{s}\".\"{base}\" WHERE \"id\" = ?1"),
                    [&id],
                    |r| r.get(0),
                )
                .store()?;
            if existed > 0 {
                tx.execute(
                    &format!("DELETE FROM \"{s}\".\"{base}\" WHERE \"id\" = ?1"),
                    [&id],
                )
                .store()?;
            }

            plan.apply(&tx, &s, version_id, &ts)?;

            // 'C' and 'U' are distinct in the history op column; recording every
            // version as an update would lose the distinction, and the op is part
            // of the hashed preimage, so it cannot be corrected later.
            let op = if existed > 0 { "U" } else { "C" };
            let pre = crate::chain::preimage(&id, version_id, &ts, op, Some(&canon), &audit.actor);
            let (row_hash, row_hash_sha3) =
                crate::chain::link(prev_sha256.as_deref(), prev_sha3.as_deref(), &pre);
            let row_mac = signing
                .as_ref()
                .map(|k| crate::chain::mac(k, prev_sha256.as_deref(), &pre));

            tx.execute(
                &format!(
                    "INSERT INTO \"{s}\".\"{hist}\" \
                       (\"id\", \"version_id\", \"last_updated\", \"op\", \"resource\", \
                        \"actor\", \"actor_source\", \"client\", \"request_id\", \"reason\", \
                        \"prev_hash\", \"row_hash\", \"prev_hash_sha3\", \"row_hash_sha3\", \
                        \"row_mac\") \
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)"
                ),
                rusqlite::params![
                    &id,
                    version_id,
                    &ts,
                    op,
                    // The canonical bytes are what is stored, so what is read
                    // back is exactly what was signed. Storing the submitted
                    // text instead would make verification depend on the
                    // client's key order and whitespace.
                    &canon,
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
            .store()?;

            tx.commit().store()?;
            Ok(Put {
                id,
                version_id,
                kind: if existed > 0 {
                    PutKind::Updated
                } else {
                    PutKind::Created
                },
            })
        })
        .await
        .map_err(join_err)?
    }

    /// Read a resource back, reconstructed from its rows.
    ///
    /// `Ok(None)` for absent. Every child table is read with a single
    /// `WHERE rid = ?` and reconstruction is order-insensitive, so no ordering
    /// clause is needed — see the `ords` rationale in spec M14.7.
    pub async fn get(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Option<serde_json::Value>, StoreError> {
        self.attach().await?;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?
            .clone();
        let schema = self.map.schema.clone();
        let id = id.to_string();
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<Option<serde_json::Value>, StoreError> {
            let guard = conn.blocking_lock();

            // R4.5: one snapshot for the whole reconstruction.
            //
            // A read touches the base table and every child table as separate
            // statements. Outside a transaction each statement gets its own WAL
            // snapshot, so a writer committing between them yields a resource
            // that never existed — base columns from one version, child rows
            // from the next. Holding this store's connection mutex hides that
            // from a single handle and does nothing about a second connection,
            // which is an ordinary deployment shape.
            //
            // It was not hypothetical: a reader on a second handle observed
            // `patient_name` from version 8 beside `patient_telecom` from
            // version 12 (audit F-21). WAL makes a consistent read *possible*,
            // which M14.19 already said; it does not take one for you.
            //
            // Deferred rather than immediate: this is read-only and must not
            // take a write lock, and a deferred transaction pins its snapshot
            // at the first statement — which is the one below.
            let tx = guard.unchecked_transaction().store()?;
            let c = &*tx;

            let s = escape_ident(&schema);
            let base = &rm.base_table().name;

            let present: i64 = c
                .query_row(
                    &format!("SELECT count(*) FROM \"{s}\".\"{base}\" WHERE \"id\" = ?1"),
                    [&id],
                    |r| r.get(0),
                )
                .store()?;
            if present == 0 {
                return Ok(None);
            }

            let input = fetch_recon_input(c, &s, &rm, &id)?;
            // `?` rather than flattening to `Other`, and here it matters more
            // than on the write side: reconstruction audits row consumption and
            // reports a residue as an integrity error (R4.7). That is the signal
            // that says stored data went unread — exactly what F-20 produced —
            // and a generic string makes it indistinguishable from an I/O
            // hiccup (audit F-23).
            let v = fhir_sqlite_map::reconstruct::reconstruct(&rm, &input, Some(&id))?;
            Ok(Some(v))
        })
        .await
        .map_err(join_err)?
    }

    /// The canonical bytes recorded for a version, as stored.
    ///
    /// Exposed so a verifier can rehash exactly what was signed rather than
    /// re-deriving it from a reconstructed resource, which would beg the
    /// question the audit chain exists to answer.
    pub async fn history_canon(
        &self,
        rtype: &str,
        id: &str,
        version_id: i64,
    ) -> Result<Option<String>, StoreError> {
        self.attach().await?;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let hist = rm
            .find_table(fhir_sqlite_map::model::TableKind::History)
            .map(|(_, t)| t.name.clone())
            .ok_or_else(|| StoreError::Other("no history table".into()))?;
        let schema = self.map.schema.clone();
        let id = id.to_string();
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<Option<String>, StoreError> {
            let c = conn.blocking_lock();
            Ok(c.query_row(
                &format!(
                    "SELECT \"resource\" FROM \"{}\".\"{hist}\" WHERE \"id\" = ?1 AND \"version_id\" = ?2",
                    escape_ident(&schema)
                ),
                rusqlite::params![&id, version_id],
                |r| r.get::<_, Option<String>>(0),
            )
            .ok()
            .flatten())
        })
        .await
        .map_err(join_err)?
    }
}

/// Rebuild a `LeafVal` from its four stored column images.
fn leaf_from_cols(
    kind: &str,
    text: Option<String>,
    num: Option<String>,
    b: Option<i64>,
) -> fhir_sqlite_map::value::LeafVal {
    use fhir_sqlite_map::value::LeafVal;
    match kind {
        "s" => LeafVal::Str(text.unwrap_or_default()),
        "n" => LeafVal::Num(num.unwrap_or_default()),
        "b" => LeafVal::Bool(b.unwrap_or(0) != 0),
        _ => LeafVal::Null,
    }
}

/// `YYYY-MM-DD HH:MM:SS.ffffff`, UTC, fixed width.
///
/// Fixed width is the point: these are compared as text (M14.12), so a variable
/// number of fractional digits would make lexicographic and chronological order
/// disagree. Rendered without a chrono dependency, which the crate does not have.
fn utc_micros(t: std::time::SystemTime) -> String {
    let d = t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = d.as_secs() as i64;
    let micros = d.subsec_micros();
    let days = secs.div_euclid(86_400);
    let sod = secs.rem_euclid(86_400);
    let (y, m, dd) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{dd:02} {:02}:{:02}:{:02}.{micros:06}",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    )
}

/// Howard Hinnant's `civil_from_days`: days since the Unix epoch to y/m/d.
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

/// The rows one resource turns into, rendered as bound statements.
///
/// Built outside the transaction so that time spent formatting SQL is not time
/// holding the write lock — with one writer, that lock is the whole system's
/// throughput.
struct InsertPlan {
    id: String,
    /// `(table name, column names, per-row values)`
    batches: Vec<(String, Vec<String>, Vec<Vec<rusqlite::types::Value>>)>,
    base_table: String,
    base_cols: Vec<String>,
    base_vals: Vec<rusqlite::types::Value>,
}

impl InsertPlan {
    fn build(
        rm: &fhir_sqlite_map::model::ResourceMap,
        out: &fhir_sqlite_map::shred::ShredOut,
        id: &str,
    ) -> Result<Self, StoreError> {
        use fhir_sqlite_map::model::TableKind;
        use rusqlite::types::Value as V;

        let mut plan = Self {
            id: id.to_string(),
            batches: Vec::new(),
            base_table: rm.base_table().name.clone(),
            base_cols: Vec::new(),
            base_vals: Vec::new(),
        };

        // Group element rows by table, so each table is one multi-row insert
        // rather than one statement per row.
        let mut by_table: std::collections::BTreeMap<u32, Vec<&fhir_sqlite_map::shred::Row>> =
            std::collections::BTreeMap::new();
        for r in &out.rows {
            by_table.entry(r.table).or_default().push(r);
        }

        for (ti, rows) in by_table {
            let t = &rm.tables[ti as usize];
            match t.kind {
                TableKind::Base => {
                    let r = rows[0];
                    for (name, v) in &r.cols {
                        plan.base_cols.push(name.clone());
                        plan.base_vals.push(sqlval(v));
                    }
                }
                TableKind::Elem => {
                    // The column union across the batch: a row that did not set
                    // a column binds NULL for it.
                    let mut names: Vec<String> = Vec::new();
                    for r in &rows {
                        for (n, _) in &r.cols {
                            if !names.contains(n) {
                                names.push(n.clone());
                            }
                        }
                    }
                    let mut cols = vec!["rid".to_string(), "ords".to_string()];
                    cols.extend(names.clone());
                    let mut vals = Vec::new();
                    for r in &rows {
                        let mut row = vec![V::Text(id.to_string()), V::Text(fmt_ords(&r.ords))];
                        for n in &names {
                            row.push(
                                r.cols
                                    .iter()
                                    .find(|(c, _)| c == n)
                                    .map(|(_, v)| sqlval(v))
                                    .unwrap_or(V::Null),
                            );
                        }
                        vals.push(row);
                    }
                    plan.batches.push((t.name.clone(), cols, vals));
                }
                _ => {}
            }
        }

        // Extensions, spill, and contained come from their own lists rather than
        // from `rows`.
        if let Some((_, t)) = rm.find_table(TableKind::Ext)
            && !out.ext.is_empty()
        {
            let cols = [
                "rid", "path", "ords", "modifier", "ext_ord", "url", "leaf", "v_kind", "v_text",
                "v_num", "v_bool",
            ]
            .map(String::from)
            .to_vec();
            let vals = out
                .ext
                .iter()
                .map(|e| {
                    let (kind, text, num, b) = e.val.cols();
                    vec![
                        V::Text(id.to_string()),
                        V::Text(e.path.clone()),
                        V::Text(fmt_ords(&e.ords)),
                        V::Integer(i64::from(e.modifier)),
                        V::Integer(i64::from(e.ext_ord)),
                        opt_text(e.url.as_deref()),
                        V::Text(e.leaf.clone()),
                        V::Text(kind.to_string()),
                        opt_text(text),
                        opt_text(num),
                        b.map_or(V::Null, |x| V::Integer(i64::from(x))),
                    ]
                })
                .collect();
            plan.batches.push((t.name.clone(), cols, vals));
        }

        if let Some((_, t)) = rm.find_table(TableKind::Deep)
            && !out.deep.is_empty()
        {
            let cols = [
                "rid", "path", "ords", "leaf", "v_kind", "v_text", "v_num", "v_bool",
            ]
            .map(String::from)
            .to_vec();
            let vals = out
                .deep
                .iter()
                .map(|d| {
                    let (kind, text, num, b) = d.val.cols();
                    vec![
                        V::Text(id.to_string()),
                        V::Text(d.path.clone()),
                        V::Text(fmt_ords(&d.ords)),
                        V::Text(d.leaf.clone()),
                        V::Text(kind.to_string()),
                        opt_text(text),
                        opt_text(num),
                        b.map_or(V::Null, |x| V::Integer(i64::from(x))),
                    ]
                })
                .collect();
            plan.batches.push((t.name.clone(), cols, vals));
        }

        if let Some((_, t)) = rm.find_table(TableKind::Contained)
            && !out.contained.is_empty()
        {
            let cols = ["rid", "ord", "resource"].map(String::from).to_vec();
            let vals = out
                .contained
                .iter()
                .map(|(ord, v)| {
                    vec![
                        V::Text(id.to_string()),
                        V::Integer(i64::from(*ord)),
                        V::Text(v.to_string()),
                    ]
                })
                .collect();
            plan.batches.push((t.name.clone(), cols, vals));
        }

        Ok(plan)
    }

    fn apply(
        &self,
        tx: &rusqlite::Transaction<'_>,
        schema: &str,
        version_id: i64,
        ts: &str,
    ) -> Result<(), StoreError> {
        use rusqlite::types::Value as V;

        // Base row first: every child row has a foreign key to it, and with
        // `foreign_keys=ON` an out-of-order insert is refused rather than
        // silently accepted.
        let mut cols = vec![
            "id".to_string(),
            "version_id".to_string(),
            "last_updated".to_string(),
        ];
        cols.extend(self.base_cols.clone());
        let mut vals: Vec<V> = vec![
            V::Text(self.id.clone()),
            V::Integer(version_id),
            V::Text(ts.to_string()),
        ];
        vals.extend(self.base_vals.clone());
        insert_rows(
            tx,
            schema,
            &self.base_table,
            &cols,
            std::slice::from_ref(&vals),
        )?;

        for (table, cols, rows) in &self.batches {
            insert_rows(tx, schema, table, cols, rows)?;
        }
        Ok(())
    }
}

/// One multi-row `INSERT`, chunked to stay under SQLite's bound-parameter limit.
///
/// `SQLITE_MAX_VARIABLE_NUMBER` is 32,766 on current builds and 999 on older
/// ones; the conservative bound is used because exceeding it is a runtime error
/// on exactly the resources that are large enough to matter.
fn insert_rows(
    tx: &rusqlite::Transaction<'_>,
    schema: &str,
    table: &str,
    cols: &[String],
    rows: &[Vec<rusqlite::types::Value>],
) -> Result<(), StoreError> {
    if rows.is_empty() {
        return Ok(());
    }
    const MAX_PARAMS: usize = 900;
    let per_row = cols.len().max(1);
    let chunk = (MAX_PARAMS / per_row).max(1);

    let collist = cols
        .iter()
        .map(|c| format!("\"{}\"", escape_ident(c)))
        .collect::<Vec<_>>()
        .join(", ");

    for group in rows.chunks(chunk) {
        let mut placeholders = Vec::with_capacity(group.len());
        let mut flat: Vec<&rusqlite::types::Value> = Vec::new();
        let mut n = 1usize;
        for row in group {
            if row.len() != cols.len() {
                return Err(StoreError::Other(format!(
                    "{table}: {} values for {} columns",
                    row.len(),
                    cols.len()
                )));
            }
            let marks: Vec<String> = (0..row.len())
                .map(|_| {
                    let m = format!("?{n}");
                    n += 1;
                    m
                })
                .collect();
            placeholders.push(format!("({})", marks.join(", ")));
            flat.extend(row.iter());
        }
        let sql = format!(
            "INSERT INTO \"{}\".\"{}\" ({collist}) VALUES {}",
            escape_ident(schema),
            escape_ident(table),
            placeholders.join(", ")
        );
        tx.execute(&sql, rusqlite::params_from_iter(flat))
            .map_err(|e| StoreError::Other(format!("inserting into {table}: {e}")))?;
    }
    Ok(())
}

fn sqlval(v: &fhir_sqlite_map::shred::SqlVal) -> rusqlite::types::Value {
    use fhir_sqlite_map::shred::SqlVal as S;
    use rusqlite::types::Value as V;
    match v {
        S::Bool(b) => V::Integer(i64::from(*b)),
        S::Int(i) => V::Integer(*i),
        // Numeric, timestamp, date, and JSON all cross as text: the column
        // affinities are TEXT (M14.10) and the lexical form is what M3.6
        // requires to survive round-trip.
        S::Num(s) | S::Text(s) | S::Ts(s) | S::Date(s) | S::Jsonb(s) => V::Text(s.clone()),
        // U4a: the checksum adjunct crosses as a BLOB, not as text. SQLite
        // compares BLOBs bytewise, which is what an equality probe needs.
        S::Bytes(b) => V::Blob(b.clone()),
    }
}

fn opt_text(s: Option<&str>) -> rusqlite::types::Value {
    s.map_or(rusqlite::types::Value::Null, |x| {
        rusqlite::types::Value::Text(x.to_string())
    })
}

// -------------------------------------------------------- history and versions

impl SqliteStore {
    /// Every stored version of a resource, newest first.
    ///
    /// A deletion appears as an entry with `op == 'D'` and no resource, which is
    /// how a reader tells "deleted" from "never existed" — the base row is gone
    /// in both cases, so history is the only witness.
    pub async fn history(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Vec<crate::HistEntry>, StoreError> {
        let (schema, hist) = self.hist_target(rtype)?;
        let id = id.to_string();
        let conn = self.conn.clone();
        self.attach().await?;
        tokio::task::spawn_blocking(move || -> Result<Vec<crate::HistEntry>, StoreError> {
            let c = conn.blocking_lock();
            let mut st = c
                .prepare(&format!(
                    "SELECT \"version_id\", \"last_updated\", \"op\", \"resource\" \
                     FROM \"{}\".\"{hist}\" WHERE \"id\" = ?1 ORDER BY \"version_id\" DESC",
                    escape_ident(&schema)
                ))
                .store()?;
            query_history_entries(&mut st, &id)
        })
        .await
        .map_err(join_err)?
    }

    /// History across one type, or every mapped type (`rtype` `None`),
    /// newest first — the store half of type-/system-level `_history`
    /// (`fhir-loco`'s `SV2.17` is the HTTP slice over it).
    ///
    /// Returns at most `count` `(rtype, id, entry)` rows, ordered by
    /// `last_updated` then `version_id`, both descending. `since` keeps
    /// versions written **at or after** that instant (FHIR `_since`),
    /// compared textually — every stored `last_updated` is RFC 3339 UTC
    /// from one writer, so lexical and chronological order agree. There is
    /// no cursor: the result is the newest `count` entries, an honest
    /// slice rather than an approximate page.
    pub async fn history_page(
        &self,
        rtype: Option<&str>,
        count: i64,
        since: Option<&str>,
    ) -> Result<Vec<(String, String, crate::HistEntry)>, StoreError> {
        self.attach().await?;
        let targets: Vec<(String, String)> = match rtype {
            Some(t) => vec![(t.to_string(), self.hist_target(t)?.1)],
            None => {
                let mut v = Vec::new();
                for t in self.map.resources.keys() {
                    v.push((t.clone(), self.hist_target(t)?.1));
                }
                v
            }
        };
        let count = count.clamp(1, 1000);
        let schema = self.map.schema.clone();
        let since = since.map(str::to_string);
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(
            move || -> Result<Vec<(String, String, crate::HistEntry)>, StoreError> {
                let c = conn.blocking_lock();
                let s = escape_ident(&schema);
                type Raw = (String, i64, String, String, Option<String>);
                let mut out: Vec<(String, String, crate::HistEntry)> = Vec::new();
                for (t, hist) in &targets {
                    let filter = if since.is_some() {
                        " WHERE \"last_updated\" >= ?1"
                    } else {
                        ""
                    };
                    let sql = format!(
                        "SELECT \"id\", \"version_id\", \"last_updated\", \"op\", \
                         \"resource\" FROM \"{s}\".\"{hist}\"{filter} \
                         ORDER BY \"last_updated\" DESC, \"version_id\" DESC LIMIT {count}"
                    );
                    let mut st = c.prepare(&sql).store()?;
                    let map_row = |r: &rusqlite::Row<'_>| -> rusqlite::Result<Raw> {
                        Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
                    };
                    let rows: Vec<rusqlite::Result<Raw>> = match &since {
                        Some(v) => st.query_map([v], map_row).store()?.collect(),
                        None => st.query_map([], map_row).store()?.collect(),
                    };
                    for row in rows {
                        let (id, version_id, last_updated, op, raw) = row.store()?;
                        let resource = match raw {
                            Some(txt) => Some(serde_json::from_str(&txt).map_err(|e| {
                                StoreError::Other(format!("history {t}/{id}/{version_id}: {e}"))
                            })?),
                            None => None,
                        };
                        out.push((
                            t.clone(),
                            id,
                            crate::HistEntry {
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
            },
        )
        .await
        .map_err(join_err)?
    }

    /// One specific version, as it was stored.
    ///
    /// Read from history rather than reassembled from the live tables, which is
    /// the only way an old version can be returned at all — and means the bytes
    /// returned are the bytes the hash chain covers.
    ///
    /// Returns a `HistEntry`, not a bare resource: a deleted version has no
    /// content, and a caller must be able to tell "version 3 was a deletion"
    /// from "version 3 does not exist". The first is 410 Gone, the second 404.
    pub async fn vread(
        &self,
        rtype: &str,
        id: &str,
        version_id: i64,
    ) -> Result<Option<crate::HistEntry>, StoreError> {
        let (schema, hist) = self.hist_target(rtype)?;
        self.attach().await?;
        let id = id.to_string();
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<Option<crate::HistEntry>, StoreError> {
            let c = conn.blocking_lock();
            let row = c
                .query_row(
                    &format!(
                        "SELECT \"version_id\", \"last_updated\", \"op\", \"resource\" \
                         FROM \"{}\".\"{hist}\" WHERE \"id\" = ?1 AND \"version_id\" = ?2",
                        escape_ident(&schema)
                    ),
                    rusqlite::params![&id, version_id],
                    |r| {
                        Ok((
                            r.get::<_, i64>(0)?,
                            r.get::<_, String>(1)?,
                            r.get::<_, String>(2)?,
                            r.get::<_, Option<String>>(3)?,
                        ))
                    },
                )
                .ok();
            let Some((version_id, last_updated, op, raw)) = row else {
                return Ok(None);
            };
            let resource = match raw {
                Some(t) => Some(
                    serde_json::from_str(&t)
                        .map_err(|e| StoreError::Other(format!("version {version_id}: {e}")))?,
                ),
                None => None,
            };
            Ok(Some(crate::HistEntry {
                version_id,
                last_updated,
                op: op.chars().next().unwrap_or('?'),
                resource,
            }))
        })
        .await
        .map_err(join_err)?
    }

    /// Delete a resource, leaving a tombstone in history.
    ///
    /// Returns the tombstone's version, or `None` if there was nothing to
    /// delete. The base row goes (cascading to its children) but history does
    /// not: a deletion that erased its own evidence would defeat the audit
    /// trail, which is why the history table has no foreign key to the base
    /// table.
    pub async fn delete(
        &self,
        rtype: &str,
        id: &str,
        audit: &crate::Audit,
    ) -> Result<Option<i64>, StoreError> {
        self.attach().await?;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let base = rm.base_table().name.clone();
        let (schema, hist) = self.hist_target(rtype)?;
        let id = id.to_string();
        let audit = audit.clone();
        let signing = self.keys.signing().cloned();
        let ts = utc_micros(std::time::SystemTime::now());
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<Option<i64>, StoreError> {
            let mut c = conn.blocking_lock();
            let tx = c
                .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                .store()?;
            let s = escape_ident(&schema);

            let present: i64 = tx
                .query_row(
                    &format!("SELECT count(*) FROM \"{s}\".\"{base}\" WHERE \"id\" = ?1"),
                    [&id],
                    |r| r.get(0),
                )
                .store()?;
            if present == 0 {
                return Ok(None);
            }

            let prev: Option<ChainTip> = tx
                .query_row(
                    &format!(
                        "SELECT \"version_id\", \"row_hash\", \"row_hash_sha3\" FROM \"{s}\".\"{hist}\" \
                         WHERE \"id\" = ?1 ORDER BY \"version_id\" DESC LIMIT 1"
                    ),
                    [&id],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
                )
                .ok();
            let (version_id, prev_256, prev_3) = match prev {
                Some((v, a, b)) => (v + 1, a, b),
                None => (1, None, None),
            };

            tx.execute(
                &format!("DELETE FROM \"{s}\".\"{base}\" WHERE \"id\" = ?1"),
                [&id],
            )
            .store()?;

            // No resource in the preimage: there is no content to commit to, and
            // an empty string is what `preimage` substitutes for `None`, so a
            // tombstone still extends the chain.
            let pre = crate::chain::preimage(&id, version_id, &ts, "D", None, &audit.actor);
            let (row_hash, row_sha3) =
                crate::chain::link(prev_256.as_deref(), prev_3.as_deref(), &pre);
            let row_mac = signing
                .as_ref()
                .map(|k| crate::chain::mac(k, prev_256.as_deref(), &pre));

            tx.execute(
                &format!(
                    "INSERT INTO \"{s}\".\"{hist}\" \
                       (\"id\", \"version_id\", \"last_updated\", \"op\", \"resource\", \
                        \"actor\", \"actor_source\", \"client\", \"request_id\", \"reason\", \
                        \"prev_hash\", \"row_hash\", \"prev_hash_sha3\", \"row_hash_sha3\", \
                        \"row_mac\") \
                     VALUES (?1,?2,?3,'D',NULL,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)"
                ),
                rusqlite::params![
                    &id,
                    version_id,
                    &ts,
                    &audit.actor,
                    &audit.actor_source,
                    &audit.client,
                    &audit.request_id,
                    &audit.reason,
                    &prev_256,
                    &row_hash,
                    &prev_3,
                    &row_sha3,
                    &row_mac,
                ],
            )
            .store()?;

            tx.commit().store()?;
            Ok(Some(version_id))
        })
        .await
        .map_err(join_err)?
    }

    /// Recompute every history chain and report what does not match.
    ///
    /// An empty result is the claim "nothing in history has been altered since
    /// it was written". Rows predating the chain columns have no stored hash;
    /// those are skipped rather than reported, because calling them breaks would
    /// train an operator to ignore the report — the same reasoning as the
    /// PostgreSQL store.
    ///
    /// Both chains are checked independently. They are SHA-256 and SHA-3, which
    /// are different constructions, so one line of cryptanalysis cannot take
    /// both (spec M3.16a).
    ///
    /// `keys` also verifies the keyed tag where one was written. Passing an
    /// empty ring is not an error — rows then report as unverifiable rather than
    /// broken, because "I do not hold that key" and "this row was altered" are
    /// different claims and conflating them makes the report useless (M3.16b).
    pub async fn verify_audit(&self) -> Result<Vec<crate::ChainBreak>, StoreError> {
        self.attach().await?;
        let map = self.map.clone();
        let schema = self.map.schema.clone();
        let keys = self.keys.clone();
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<Vec<crate::ChainBreak>, StoreError> {
            use fhir_sqlite_map::model::TableKind;
            let c = conn.blocking_lock();
            let s = escape_ident(&schema);
            let mut breaks = Vec::new();

            // Counter-signatures, keyed by the row they vouch for. Read up front:
            // there is one small table for the whole schema, and querying it per
            // history row would turn a linear walk into a quadratic one.
            let mut st = c
                .prepare(&format!(
                    "SELECT \"rtype\", \"id\", \"version_id\", \"row_mac\" \
                     FROM \"{s}\".\"fhir_sqlite_countersign\""
                ))
                .store()?;
            let countersigns = query_countersigns(&mut st)?;

            for rm in map.resources.values() {
                let Some((_, hist)) = rm.find_table(TableKind::History) else {
                    continue;
                };
                let mut st = c
                    .prepare(&format!(
                        "SELECT \"id\", \"version_id\", \"last_updated\", \"op\", \"resource\", \
                                \"actor\", \"prev_hash\", \"row_hash\", \"prev_hash_sha3\", \
                                \"row_hash_sha3\", \"row_mac\" \
                         FROM \"{s}\".\"{}\" ORDER BY \"id\", \"version_id\"",
                        hist.name
                    ))
                    .store()?;

                // Ordered by (id, version_id), so each resource's chain is walked
                // in order and the tip resets when the id changes.
                let mut cur = String::new();
                let mut prior_256: Option<Vec<u8>> = None;
                let mut prior_3: Option<Vec<u8>> = None;

                let rows = query_audit_rows(&mut st)?;

                for row in rows {
                    let (
                        id,
                        version_id,
                        ts,
                        op,
                        resource,
                        actor,
                        prev_256,
                        row_256,
                        prev_3,
                        row_3,
                        row_mac,
                    ) = row;
                    if id != cur {
                        cur = id.clone();
                        prior_256 = None;
                        prior_3 = None;
                    }

                    let pre = crate::chain::preimage(
                        &id,
                        version_id,
                        &ts,
                        &op,
                        resource.as_deref(),
                        &actor,
                    );

                    // The keyed tag is checked even for rows that predate the
                    // hash columns: those two gaps are independent, and skipping
                    // the tag because the hashes are absent would silently narrow
                    // what a keyed deployment actually verifies.
                    // Verified against the row's *stored* `prev_hash`, not the
                    // tip the walk arrived with. Those agree for an ordinary
                    // row — and where they do not, the link check below is what
                    // says so. Separating them is what lets an erasure tombstone
                    // keep a meaningful tag: its predecessors were deleted on
                    // purpose, so the walk reaches it with no prior, but the
                    // hash it was signed over is still recorded on the row.
                    // (The PostgreSQL original skips tombstones entirely here
                    // and so cannot verify them at all; the ports are free to
                    // differ, and this is strictly more checking.)
                    check_mac(
                        &keys,
                        &countersigns,
                        &rm.name,
                        &id,
                        version_id,
                        row_mac.as_deref(),
                        prev_256.as_deref(),
                        &pre,
                        &mut breaks,
                    );

                    // An erasure tombstone (M3.18) is a deliberate hole: the
                    // versions it replaced were removed on purpose, so its
                    // `prev_hash` points at a chain tip that no longer exists.
                    // Checking its link would report every lawful erasure as
                    // tampering — the loudest possible false positive, on the
                    // one operation an operator most needs to trust. Its own
                    // hash still becomes the tip for anything written after.
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

    /// `(schema, history table name)` for a resource type.
    fn hist_target(&self, rtype: &str) -> Result<(String, String), StoreError> {
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let hist = rm
            .find_table(fhir_sqlite_map::model::TableKind::History)
            .map(|(_, t)| t.name.clone())
            .ok_or_else(|| StoreError::Other(format!("{rtype} has no history table")))?;
        Ok((self.map.schema.clone(), hist))
    }
}

/// Verify one row's keyed tag, recording a finding only when the tag is present
/// and wrong.
///
/// The distinctions matter more than the code length suggests:
/// - **Absent** — written unkeyed. Not a finding.
/// - **Unverifiable** — signed under a key this process does not hold. A warning,
///   never a break: reporting it as tampering would train an operator to ignore
///   the report, and the row may be perfectly sound.
/// - **Mismatch** — a finding, and it stays one even if a counter-signature
///   vouches for the row. Otherwise re-signing would be a way to bless forged
///   history (M3.16d).
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
    // A counter-signature stands in only where the original tag cannot be
    // checked — never where it actively disagrees.
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

// --------------------------------------------------------------------- search

/// One page of search results.
///
/// An alias rather than its own type: the HTTP layer was written against
/// `SearchOutcome`, and two structurally identical types would mean a
/// conversion at every boundary for no benefit.
pub type Page = crate::SearchOutcome;

impl SqliteStore {
    /// Run a search, returning a page of ids.
    ///
    /// Values are bound, never interpolated: the fuzz target in the PostgreSQL
    /// original asserts that every attacker-controlled value lands in `binds`
    /// and never in `sql`, and that invariant is the reason this returns a
    /// `CompiledQuery` from the builder rather than assembling SQL here.
    /// Just the ids, for callers that do not need a total or a cursor.
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

    /// A page plus, optionally, the total — but no keyset cursor.
    pub async fn search_full(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        sort: &[crate::sqlite_search::SortKey],
        want_total: bool,
    ) -> Result<Page, StoreError> {
        self.search_page(rtype, params, count, offset, sort, want_total, None)
            .await
    }

    /// A page, with an optional keyset cursor.
    ///
    /// `after_id` narrows only the page query, never the count: a `_total` that
    /// shrank as the caller paged would make paging impossible to drive.
    #[allow(clippy::too_many_arguments)]
    pub async fn search_page(
        &self,
        rtype: &str,
        params: &[(String, String)],
        count: i64,
        offset: i64,
        sort: &[crate::sqlite_search::SortKey],
        want_total: bool,
        after_id: Option<&str>,
    ) -> Result<Page, StoreError> {
        self.attach().await?;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let q = crate::sqlite_search::build_search_sql(
            &self.map, rm, params, count, offset, sort, after_id,
        )?;
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<Page, StoreError> {
            let c = conn.blocking_lock();

            let total = if want_total {
                // The count query uses only the leading binds; the paging binds
                // belong to the page query alone.
                let cb: Vec<&String> = q.binds.iter().take(q.count_binds).collect();
                let n: i64 = c
                    .query_row(&q.count_sql, rusqlite::params_from_iter(cb), |r| r.get(0))
                    .map_err(|e| StoreError::Other(format!("count: {e}\n{}", q.count_sql)))?;
                Some(n)
            } else {
                None
            };

            let mut st = c
                .prepare(&q.sql)
                .map_err(|e| StoreError::Other(format!("preparing search: {e}\n{}", q.sql)))?;
            let ids = query_search_ids(&mut st, &q.binds, &q.sql)?;
            Ok(crate::SearchOutcome { ids, total })
        })
        .await
        .map_err(join_err)?
    }
}

// ------------------------------------------------------- access log & erasure

impl SqliteStore {
    /// Record one disclosure (PR12.5).
    ///
    /// A store that logs only mutations cannot answer "who looked at this
    /// patient", which is the question an audit usually opens with. This is
    /// therefore a read-path obligation, not a write-path one.
    ///
    /// The timestamp is supplied here rather than defaulted by the database:
    /// SQLite's `strftime` cannot produce the six fractional digits the rest of
    /// the schema fixes on, and mixed widths would not sort as text (M14.12).
    pub async fn log_access(&self, rec: &crate::AccessRecord) -> Result<(), StoreError> {
        self.attach().await?;
        let schema = self.map.schema.clone();
        let ts = utc_micros(std::time::SystemTime::now());
        let rec = rec.clone();
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<(), StoreError> {
            let c = conn.blocking_lock();
            c.execute(
                &format!(
                    "INSERT INTO \"{}\".\"fhir_sqlite_access_log\" \
                       (\"ts\", \"request_id\", \"actor\", \"actor_source\", \"client\", \
                        \"interaction\", \"rtype\", \"id\", \"version_id\", \"outcome\", \
                        \"result_count\", \"reason\") \
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
                    escape_ident(&schema)
                ),
                rusqlite::params![
                    &ts,
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
            .store()?;
            Ok(())
        })
        .await
        .map_err(join_err)?
    }

    /// How many disclosures have been recorded, for tests and diagnostics.
    pub async fn access_log_len(&self) -> Result<i64, StoreError> {
        self.attach().await?;
        let schema = self.map.schema.clone();
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<i64, StoreError> {
            let c = conn.blocking_lock();
            c.query_row(
                &format!(
                    "SELECT count(*) FROM \"{}\".\"fhir_sqlite_access_log\"",
                    escape_ident(&schema)
                ),
                [],
                |r| r.get(0),
            )
            .store()
        })
        .await
        .map_err(join_err)?
    }

    /// Erase one resource and its history (GDPR Art. 17, spec M3.18).
    ///
    /// The one sanctioned exception to append-only history, and deliberately
    /// noisy rather than quiet: the history rows go, and a single tombstone
    /// takes their place recording who erased it, when, why, and the hash the
    /// chain ended on. What is left is a *verifiable hole* — `verify_audit` can
    /// still see that a chain existed and was deliberately terminated — rather
    /// than something indistinguishable from a chain that never happened.
    ///
    /// The append-only trigger permits the delete only while the erasure flag
    /// row exists (M14.22). Inserting and removing it inside the transaction is
    /// what makes that permission scoped: unlike PostgreSQL's session GUC, an
    /// aborted erasure cannot leave the escape hatch open, because the flag
    /// rolls back with everything else.
    ///
    /// What this cannot do is un-say the data: backups, replicas, and WAL
    /// archives still hold it until they age out. A deployment promising erasure
    /// has to mean the whole estate.
    pub async fn purge(
        &self,
        rtype: &str,
        id: &str,
        audit: &crate::Audit,
    ) -> Result<crate::PurgeReport, StoreError> {
        self.attach().await?;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let base = rm.base_table().name.clone();
        let (schema, hist) = self.hist_target(rtype)?;
        let id = id.to_string();
        let audit = audit.clone();
        let signing = self.keys.signing().cloned();
        let ts = utc_micros(std::time::SystemTime::now());
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<crate::PurgeReport, StoreError> {
            let mut c = conn.blocking_lock();
            let tx = c
                .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                .store()?;
            let s = escape_ident(&schema);

            let tip: Option<ChainTip> = tx
                .query_row(
                    &format!(
                        "SELECT \"version_id\", \"row_hash\", \"row_hash_sha3\" FROM \"{s}\".\"{hist}\" \
                         WHERE \"id\" = ?1 ORDER BY \"version_id\" DESC LIMIT 1"
                    ),
                    [&id],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
                )
                .ok();
            let Some((last_version, tip_256, tip_3)) = tip else {
                return Ok(crate::PurgeReport {
                    versions_erased: 0,
                    existed: false,
                });
            };

            // Open the escape hatch, and close it before committing. Scoped to
            // this transaction by construction.
            tx.execute(
                &format!("INSERT INTO \"{s}\".\"fhir_sqlite_erasure\" (\"token\") VALUES (?1)"),
                [&id],
            )
            .store()?;

            tx.execute(
                &format!("DELETE FROM \"{s}\".\"{base}\" WHERE \"id\" = ?1"),
                [&id],
            )
            .store()?;
            let erased = tx
                .execute(
                    &format!("DELETE FROM \"{s}\".\"{hist}\" WHERE \"id\" = ?1"),
                    [&id],
                )
                .store()? as u64;

            let tomb_version = last_version + 1;
            let pre = crate::chain::preimage(&id, tomb_version, &ts, "X", None, &audit.actor);
            let (row_hash, row_sha3) =
                crate::chain::link(tip_256.as_deref(), tip_3.as_deref(), &pre);
            let row_mac = signing
                .as_ref()
                .map(|k| crate::chain::mac(k, tip_256.as_deref(), &pre));

            // `prev_hash` deliberately records the tip of the chain that was
            // erased. Those rows are gone, so the link cannot be re-derived —
            // that is the point. It is evidence that something was there.
            tx.execute(
                &format!(
                    "INSERT INTO \"{s}\".\"{hist}\" \
                       (\"id\", \"version_id\", \"last_updated\", \"op\", \"resource\", \
                        \"actor\", \"actor_source\", \"client\", \"request_id\", \"reason\", \
                        \"prev_hash\", \"row_hash\", \"prev_hash_sha3\", \"row_hash_sha3\", \
                        \"row_mac\") \
                     VALUES (?1,?2,?3,'X',NULL,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)"
                ),
                rusqlite::params![
                    &id,
                    tomb_version,
                    &ts,
                    &audit.actor,
                    &audit.actor_source,
                    &audit.client,
                    &audit.request_id,
                    &audit.reason,
                    &tip_256,
                    &row_hash,
                    &tip_3,
                    &row_sha3,
                    &row_mac,
                ],
            )
            .store()?;

            tx.execute(
                &format!("DELETE FROM \"{s}\".\"fhir_sqlite_erasure\" WHERE \"token\" = ?1"),
                [&id],
            )
            .store()?;

            tx.commit().store()?;
            Ok(crate::PurgeReport {
                versions_erased: erased,
                existed: true,
            })
        })
        .await
        .map_err(join_err)?
    }
}

// ------------------------------------------------------- the server's surface
//
// The HTTP layer was written against the inherited PostgreSQL `Store` and calls
// fourteen of its methods. These give `SqliteStore` the same names and shapes so
// the server can be pointed at it by changing a type rather than by being
// rewritten — and so the two implementations stay comparable while both exist.

impl SqliteStore {
    /// The map this store serves. The server reads it to build its
    /// CapabilityStatement and to resolve search parameters.
    #[must_use]
    pub fn map(&self) -> &Arc<RelMap> {
        &self.map
    }

    /// Is the database reachable? Used by the readiness probe.
    ///
    /// For a file this is nearly always true, which is itself worth saying: a
    /// green readiness check here is a much weaker signal than it is against a
    /// networked server, and should not be read as one.
    pub async fn ping(&self) -> Result<(), StoreError> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<(), StoreError> {
            let c = conn.blocking_lock();
            let _: i64 = c.query_row("SELECT 1", [], |r| r.get(0)).store()?;
            Ok(())
        })
        .await
        .map_err(join_err)?
    }

    /// A resource with its current version, or `None`.
    pub async fn get_versioned(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Option<crate::Got>, StoreError> {
        let Some(resource) = self.get(rtype, id).await? else {
            return Ok(None);
        };
        // The row was read a moment ago; if it is gone now the read is stale
        // rather than wrong, and reporting absence is the honest answer.
        let Some(version_id) = active_version(self.status(rtype, id).await?) else {
            return Ok(None);
        };
        Ok(Some(crate::Got {
            resource,
            version_id,
        }))
    }

    /// Several resources at once, preserving order and absence.
    ///
    /// Absence is `None` in place rather than a shorter list: the caller is
    /// resolving `_include` targets and needs to know *which* ones were missing.
    pub async fn get_all(
        &self,
        items: &[(String, String)],
    ) -> Result<Vec<Option<crate::Got>>, StoreError> {
        let mut out = Vec::with_capacity(items.len());
        for (rtype, id) in items {
            out.push(self.get_versioned(rtype, id).await?);
        }
        Ok(out)
    }

    /// Whether a resource is live, deleted, or unknown — and at which version.
    ///
    /// The three are distinct on the wire: live is 200, deleted is 410 Gone, and
    /// unknown is 404. Collapsing the last two would tell a caller that a record
    /// it once held never existed.
    pub async fn status(&self, rtype: &str, id: &str) -> Result<crate::ResourceStatus, StoreError> {
        self.attach().await?;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?;
        let base = rm.base_table().name.clone();
        let (schema, hist) = self.hist_target(rtype)?;
        let id = id.to_string();
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<crate::ResourceStatus, StoreError> {
            let c = conn.blocking_lock();
            let s = escape_ident(&schema);
            // One read transaction over both tables: a delete landing between
            // two independent statements would report Unknown for a resource
            // that is merely gone, which is a 404 where a 410 is owed.
            let live: Option<i64> = c
                .query_row(
                    &format!("SELECT \"version_id\" FROM \"{s}\".\"{base}\" WHERE \"id\" = ?1"),
                    [&id],
                    |r| r.get(0),
                )
                .ok();
            if let Some(v) = live {
                return Ok(crate::ResourceStatus::Active(v));
            }
            let tomb: Option<i64> = c
                .query_row(
                    &format!(
                        "SELECT \"version_id\" FROM \"{s}\".\"{hist}\" \
                         WHERE \"id\" = ?1 ORDER BY \"version_id\" DESC LIMIT 1"
                    ),
                    [&id],
                    |r| r.get(0),
                )
                .ok();
            Ok(match tomb {
                Some(v) => crate::ResourceStatus::Deleted(v),
                None => crate::ResourceStatus::Unknown,
            })
        })
        .await
        .map_err(join_err)?
    }

    /// Store a resource, honouring an `If-Match` version if one was given.
    ///
    /// `expected_version` is optimistic concurrency: the write is refused if the
    /// resource has moved on since the caller read it. Checked inside the same
    /// `BEGIN IMMEDIATE` as the write, because a check in a separate transaction
    /// is a race dressed up as a guarantee.
    /// Write a resource, optionally guarded by the version the caller expects.
    ///
    /// The guard is a **read-then-write**, and the two halves must not be
    /// separable: `status` runs in one transaction and `put` in another, so
    /// without a lock spanning both, N racing writers all read the same version,
    /// all find it matches, and all write. Every one of them is told it won.
    ///
    /// That is a lost update, and it is silent — which is why it survived until
    /// a concurrency test looked for it: eight racers presenting the same
    /// expected version produced eight successes (audit F-22). Optimistic
    /// concurrency that never refuses anything is not optimistic concurrency.
    ///
    /// `write_gate` is the same lock `conditional_create_audited` takes, and for
    /// the same reason — SQLite serialises writers, but its lock does not span
    /// the gap between a look and a write.
    pub async fn put_audited(
        &self,
        resource: &serde_json::Value,
        expected_version: Option<i64>,
        audit: &crate::Audit,
    ) -> Result<crate::PutOutcome, StoreError> {
        // Unguarded writes have nothing to race: `put` is one `BEGIN IMMEDIATE`
        // and assigns its own version. Taking the gate for them would serialise
        // every write in the process for no benefit.
        if expected_version.is_none() {
            return self.put_audited_locked(resource, None, audit).await;
        }
        self.attach().await?;
        let _lock = self.write_gate.lock().await;
        self.put_audited_locked(resource, expected_version, audit)
            .await
    }

    /// The body of [`Self::put_audited`], assuming `write_gate` is already held
    /// when `expected_version` is `Some`.
    ///
    /// Separate because `tokio::sync::Mutex` is not reentrant and
    /// `conditional_create_audited` calls this while holding the gate itself.
    async fn put_audited_locked(
        &self,
        resource: &serde_json::Value,
        expected_version: Option<i64>,
        audit: &crate::Audit,
    ) -> Result<crate::PutOutcome, StoreError> {
        if let Some(expected) = expected_version {
            let rtype = resource
                .get("resourceType")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| StoreError::Other("resource has no resourceType".into()))?;
            let id = resource
                .get("id")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| StoreError::Other("resource has no id".into()))?;
            match self.status(rtype, id).await? {
                crate::ResourceStatus::Active(v) if v == expected => {}
                crate::ResourceStatus::Active(v) => {
                    return Err(StoreError::Conflict { expected, found: v });
                }
                crate::ResourceStatus::Deleted(v) => {
                    return Err(StoreError::Conflict { expected, found: v });
                }
                // No version to disagree with. `found: 0` is the same shape the
                // PostgreSQL store uses for "not there", and 0 is not a version
                // any resource can have, since numbering starts at 1.
                crate::ResourceStatus::Unknown => {
                    return Err(StoreError::Conflict { expected, found: 0 });
                }
            }
        }
        let put = self.put(resource, audit).await?;
        Ok(crate::PutOutcome {
            id: put.id,
            version_id: put.version_id,
            created: put.kind == PutKind::Created,
        })
    }

    /// Delete, reporting whether there was anything to delete.
    pub async fn delete_audited(
        &self,
        rtype: &str,
        id: &str,
        audit: &crate::Audit,
    ) -> Result<bool, StoreError> {
        Ok(self.delete(rtype, id, audit).await?.is_some())
    }

    /// Record several disclosures.
    ///
    /// One statement per record rather than the PostgreSQL original's single
    /// multi-row insert: with one writer there is no round trip to amortise, and
    /// the loop is honest about what it does.
    pub async fn log_access_batch(&self, recs: &[crate::AccessRecord]) -> Result<(), StoreError> {
        for r in recs {
            self.log_access(r).await?;
        }
        Ok(())
    }
}

impl SqliteStore {
    /// Reference targets of `param` across `ids`, for `_include` resolution.
    ///
    /// Returns `(target type, target id)` pairs. `= ANY($1)` has no SQLite
    /// equivalent, so the ids become a generated `IN (?,?,…)` list, chunked to
    /// stay under the bound-parameter limit.
    pub async fn refs_of(
        &self,
        rtype: &str,
        ids: &[String],
        param: &str,
    ) -> Result<Vec<(String, String)>, StoreError> {
        use fhir_sqlite_map::model::TargetKind;
        self.attach().await?;
        let rm = self
            .map
            .resources
            .get(rtype)
            .ok_or_else(|| StoreError::Unsupported(format!("unknown resource type {rtype}")))?
            .clone();
        let def = rm
            .search
            .iter()
            .find(|d| d.code == param)
            .ok_or_else(|| StoreError::Other(format!("unknown _include parameter {param:?}")))?
            .clone();
        if def.targets.is_empty() {
            return Err(StoreError::Other(format!(
                "search parameter {param:?} has no reference targets"
            )));
        }
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let schema = self.map.schema.clone();
        let ids = ids.to_vec();
        let conn = self.conn.clone();

        tokio::task::spawn_blocking(move || -> Result<Vec<(String, String)>, StoreError> {
            let c = conn.blocking_lock();
            let s = escape_ident(&schema);
            let mut out = Vec::new();
            for t in &def.targets {
                let TargetKind::Reference { c_type, c_id, .. } = &t.kind else {
                    continue;
                };
                let table = &rm.tables[t.table as usize].name;
                let id_col = if t.table == 0 { "id" } else { "rid" };
                for chunk in ids.chunks(900) {
                    let marks = (1..=chunk.len())
                        .map(|i| format!("?{i}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let sql = format!(
                        "SELECT DISTINCT \"{c_type}\", \"{c_id}\" FROM \"{s}\".\"{table}\" \
                         WHERE \"{id_col}\" IN ({marks}) \
                           AND \"{c_type}\" IS NOT NULL AND \"{c_id}\" IS NOT NULL"
                    );
                    let mut st = c.prepare(&sql).store()?;
                    out.extend(query_reference_rows(&mut st, chunk)?);
                }
            }
            Ok(out)
        })
        .await
        .map_err(join_err)?
    }

    /// Apply a FHIR transaction Bundle.
    ///
    /// **Not implemented, deliberately.** A FHIR `transaction` Bundle is atomic
    /// by definition — a caller submitting one is saying these writes only make
    /// sense together — and this store cannot yet honour that.
    ///
    /// The tempting shortcut is to apply each op through the ordinary write path
    /// and undo the earlier ones if a later one fails. That is a compensating
    /// unwind, not atomicity, and it is weaker in two ways that matter here:
    /// readers between ops observe a half-applied bundle, and a process that
    /// dies mid-unwind leaves the partial state permanently. Shipping it under
    /// the name `transact` would be claiming a guarantee this code does not
    /// provide, in the one subsystem whose whole purpose is being trustworthy.
    ///
    /// Doing it properly means holding a single `BEGIN IMMEDIATE` across every
    /// op, which needs `put` and `delete` split so their bodies can run inside a
    /// caller-supplied transaction. That is the work; it is tracked, not
    /// forgotten.
    pub async fn transact_audited(
        &self,
        _ops: &[crate::TxOp],
        _audit: &crate::Audit,
    ) -> Result<Vec<crate::TxOutcome>, StoreError> {
        Err(StoreError::Unsupported(
            "transaction Bundles are not yet supported by the SQLite store: \
             atomicity needs one transaction across all operations (tasks.md, T64)"
                .to_string(),
        ))
    }
}

fn query_search_ids(
    statement: &mut rusqlite::Statement<'_>,
    binds: &[String],
    sql: &str,
) -> Result<Vec<String>, StoreError> {
    let rows = statement
        .query_map(rusqlite::params_from_iter(binds.iter()), |row| {
            row.get::<_, String>(0)
        })
        .map_err(|error| StoreError::Other(format!("search: {error}\n{sql}")))?;
    rows.collect::<Result<_, _>>().store()
}

fn query_reference_rows(
    statement: &mut rusqlite::Statement<'_>,
    ids: &[String],
) -> Result<Vec<(String, String)>, StoreError> {
    let rows = statement
        .query_map(rusqlite::params_from_iter(ids.iter()), |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .store()?;
    rows.collect::<Result<_, _>>().store()
}

type AuditRow = (
    String,
    i64,
    String,
    String,
    Option<String>,
    String,
    Option<Vec<u8>>,
    Option<Vec<u8>>,
    Option<Vec<u8>>,
    Option<Vec<u8>>,
    Option<String>,
);

fn query_countersigns(
    statement: &mut rusqlite::Statement<'_>,
) -> Result<std::collections::HashMap<(String, String, i64), String>, StoreError> {
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .store()?;
    let mut countersigns = std::collections::HashMap::new();
    for row in rows {
        let (rtype, id, version, mac) = row.store()?;
        countersigns.insert((rtype, id, version), mac);
    }
    Ok(countersigns)
}

fn query_audit_rows(statement: &mut rusqlite::Statement<'_>) -> Result<Vec<AuditRow>, StoreError> {
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<Vec<u8>>>(6)?,
                row.get::<_, Option<Vec<u8>>>(7)?,
                row.get::<_, Option<Vec<u8>>>(8)?,
                row.get::<_, Option<Vec<u8>>>(9)?,
                row.get::<_, Option<String>>(10)?,
            ))
        })
        .store()?;
    rows.collect::<Result<_, _>>().store()
}

fn active_version(status: crate::ResourceStatus) -> Option<i64> {
    match status {
        crate::ResourceStatus::Active(version) => Some(version),
        crate::ResourceStatus::Deleted(_) | crate::ResourceStatus::Unknown => None,
    }
}

// ----------------------------------------------------------- conditional ops

impl SqliteStore {
    /// Ids matching conditional criteria.
    ///
    /// Capped at two results, because every caller only needs to distinguish
    /// none / exactly one / more than one — and "more than one" is a 412
    /// regardless of whether it is two matches or two thousand.
    async fn matching(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
    ) -> Result<Vec<String>, StoreError> {
        self.search(rtype, criteria, 2, 0).await
    }

    /// Create only if nothing matches the criteria (`If-None-Exist`).
    ///
    /// The race this has to survive is two callers submitting the same
    /// `If-None-Exist` at once: both search, both find nothing, and both create.
    /// PostgreSQL needs an advisory lock keyed on the criteria to prevent that.
    /// SQLite does not, but only because of how the write lock behaves —
    /// `BEGIN IMMEDIATE` admits one writer at a time, so taking it *before* the
    /// search makes the search-then-create sequence indivisible with respect to
    /// other writers.
    ///
    /// That is why this opens a write transaction and holds it across the
    /// search: doing the search outside it would be the same race with extra
    /// steps.
    pub async fn conditional_create_audited(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
        resource: &serde_json::Value,
        audit: &crate::Audit,
    ) -> Result<crate::CondCreate, StoreError> {
        self.attach().await?;
        // Take the write lock first, then look. `_lock` is held for the whole
        // function: SQLite serialises writers, so no other writer can slip a
        // matching resource in between the search and the create.
        let _lock = self.write_gate.lock().await;
        let ids = self.matching(rtype, criteria).await?;
        match ids.len() {
            0 => {
                // `_locked`, because the gate is held here and
                // `tokio::sync::Mutex` is not reentrant.
                let out = self.put_audited_locked(resource, None, audit).await?;
                Ok(crate::CondCreate::Created(out))
            }
            1 => Ok(crate::CondCreate::Existing(ids[0].clone())),
            _ => Ok(crate::CondCreate::Multiple),
        }
    }

    /// Delete the single resource matching the criteria, if there is exactly one.
    ///
    /// More than one match is refused rather than resolved: deleting several
    /// records because a query was less selective than its author believed is
    /// not something to do on a guess.
    pub async fn conditional_delete_audited(
        &self,
        rtype: &str,
        criteria: &[(String, String)],
        audit: &crate::Audit,
    ) -> Result<crate::CondDelete, StoreError> {
        self.attach().await?;
        let _lock = self.write_gate.lock().await;
        let ids = self.matching(rtype, criteria).await?;
        match ids.len() {
            0 => Ok(crate::CondDelete::NoMatch),
            1 => {
                self.delete_audited(rtype, &ids[0], audit).await?;
                Ok(crate::CondDelete::Deleted)
            }
            _ => Ok(crate::CondDelete::Multiple),
        }
    }
}

impl SqliteStore {
    /// Log a chain checkpoint: proof, at a moment, that history verified.
    ///
    /// Deliberately a log line and not a table row. A checkpoint's value comes
    /// from living somewhere the database cannot rewrite — an attacker with
    /// write access could forge a checkpoint stored beside the data it vouches
    /// for, but not one already shipped to a log collector.
    ///
    /// Never fatal: a checkpoint that cannot be taken must not stop the server
    /// serving, but must be loud enough to notice.
    pub async fn emit_checkpoint(&self, reason: &str) {
        match self.verify_audit().await {
            Ok(breaks) if breaks.is_empty() => tracing::info!(
                target: "audit_checkpoint",
                schema = %self.map.schema,
                fhir_version = %self.map.fhir_version,
                keyed = self.keys.signing().is_some(),
                %reason,
                "chain checkpoint: verified"
            ),
            Ok(breaks) => tracing::error!(
                target: "audit_checkpoint",
                schema = %self.map.schema,
                breaks = breaks.len(),
                %reason,
                "chain checkpoint: BREAKS FOUND"
            ),
            Err(e) => tracing::error!(
                target: "audit_checkpoint",
                schema = %self.map.schema,
                error = %e,
                %reason,
                "chain checkpoint failed"
            ),
        }
    }
}

impl SqliteStore {
    /// Disclosure-log rows recorded for one resource, newest first.
    ///
    /// Answers "who looked at this patient", which is the question an audit
    /// usually opens with — and the reason the log exists at all (PR12.5).
    pub async fn access_log_for(
        &self,
        rtype: &str,
        id: &str,
    ) -> Result<Vec<crate::AccessRecord>, StoreError> {
        self.attach().await?;
        let schema = self.map.schema.clone();
        let (rtype, id) = (rtype.to_string(), id.to_string());
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || -> Result<Vec<crate::AccessRecord>, StoreError> {
            let c = conn.blocking_lock();
            let mut st = c
                .prepare(&format!(
                    "SELECT \"request_id\", \"actor\", \"actor_source\", \"client\", \
                            \"interaction\", \"rtype\", \"id\", \"version_id\", \"outcome\", \
                            \"result_count\", \"reason\" \
                     FROM \"{}\".\"fhir_sqlite_access_log\" \
                     WHERE \"rtype\" = ?1 AND \"id\" = ?2 ORDER BY \"seq\" DESC",
                    escape_ident(&schema)
                ))
                .store()?;
            query_access_records(&mut st, &rtype, &id)
        })
        .await
        .map_err(join_err)?
    }
}

fn query_history_entries(
    statement: &mut rusqlite::Statement<'_>,
    id: &str,
) -> Result<Vec<crate::HistEntry>, StoreError> {
    let rows = statement
        .query_map([id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        })
        .store()?;
    let mut out = Vec::new();
    for row in rows {
        let (version_id, last_updated, op, raw) = row.store()?;
        let resource = match raw {
            Some(text) => Some(
                serde_json::from_str(&text)
                    .map_err(|error| StoreError::Other(format!("history {version_id}: {error}")))?,
            ),
            None => None,
        };
        out.push(crate::HistEntry {
            version_id,
            last_updated,
            op: op.chars().next().unwrap_or('?'),
            resource,
        });
    }
    Ok(out)
}

fn query_access_records(
    statement: &mut rusqlite::Statement<'_>,
    rtype: &str,
    id: &str,
) -> Result<Vec<crate::AccessRecord>, StoreError> {
    let rows = statement
        .query_map(rusqlite::params![rtype, id], |row| {
            Ok(crate::AccessRecord {
                audit: crate::Audit {
                    request_id: row.get(0)?,
                    actor: row.get(1)?,
                    actor_source: row.get(2)?,
                    client: row.get(3)?,
                    reason: row.get(10)?,
                },
                interaction: row.get(4)?,
                rtype: row.get(5)?,
                id: row.get(6)?,
                version_id: row.get(7)?,
                outcome: row.get(8)?,
                result_count: row.get(9)?,
            })
        })
        .store()?;
    rows.collect::<Result<_, _>>().store()
}
