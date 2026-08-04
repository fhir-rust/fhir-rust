//! DDL emission: the map, rendered as SQL Server (T-SQL) CREATE statements.
//! Deterministic — same map, same statements, same order.
//!
//! Ported per spec section 14. What shapes this file:
//!
//! - **Identifiers are bracketed** (`[name]`). Double quotes only work with
//!   `QUOTED_IDENTIFIER ON`, which is the default but is session state, and a
//!   schema must not depend on session state.
//! - **A "schema" is a SQL Server schema**, inside one database — closer to
//!   PostgreSQL than to MySQL, where it would have been a whole database.
//! - **Index keys are capped at 900 bytes** (1700 for a nonclustered index),
//!   which is *tighter* than MySQL's 3072. [Ext] and [Deep] therefore keep the
//!   hash surrogate key the MySQL port introduced, and it matters more here.
//! - **There is no `IF NOT EXISTS`** on `CREATE TABLE`. Idempotence is spelled
//!   with an `IF NOT EXISTS (SELECT … FROM sys.objects)` guard around the
//!   statement, which is why the system tables look wordier than elsewhere.
//! - **[NVARCHAR] throughout, not [VARCHAR].** SQL Server's [VARCHAR] is a
//!   single-byte code page unless the column carries a UTF-8 collation; FHIR
//!   text is Unicode, and losing a patient name to a code page is not a trade
//!   worth making for storage.

use std::fmt::Write as _;

use crate::model::{ColTy, RelMap, ResourceMap, Table, TableKind};

/// The FHIR [id] type is `[A-Za-z0-9\-\.]{1,64}], so 64 characters is an exact
/// bound rather than a guess (M14.12). Bounding it is what lets [id] and [rid]
/// be keyed and foreign-keyed without a prefix.
const ID_COL: &str = "NVARCHAR(64) COLLATE Latin1_General_100_BIN2";

/// SQL Server type mapping for the map's column types (M14.6).
/// Does this dialect need the unbounded-string adjuncts (`U1`, `U9`)?
///
/// `NVARCHAR(MAX)` cannot be indexed, so U1 requires both adjuncts.
///
/// The generator reads this to decide whether to put `<col>_idx` and `<col>_h`
/// in the map at all. `gen` is byte-identical across all six ports (`X15.1`);
/// this constant is in `ddl.rs`, which is the one file a dialect may own.
pub const TEXT_ADJUNCTS: bool = true;

/// Can this dialect **not** index or compare a column of this type as bound?
///
/// The second half of `U1a`'s trigger. The first — that a search reaches the
/// column — is the generator's to know; this is the dialect's, and only the two
/// together justify an adjunct.
///
/// `NVARCHAR(MAX)` cannot be part of an index key on this engine.
///
/// Getting this wrong is not free in either direction. Return `true` too widely
/// and every boolean a token search touches grows a derived column nothing
/// reads; too narrowly and a search silently fails on the engine.
///
/// Every variant is listed rather than wildcarded, so adding a `ColTy` is a
/// compile error here and forces the decision instead of defaulting it.
pub fn needs_adjunct(ty: ColTy) -> bool {
    match ty {
        ColTy::Bool => false,
        ColTy::Int => false,
        ColTy::BigInt => false,
        ColTy::Numeric => true,
        ColTy::Text => true,
        ColTy::TextC => false,
        ColTy::TextIdx => false,
        ColTy::Digest => false,
        ColTy::Date => false,
        ColTy::Timestamptz => false,
        ColTy::Jsonb => true,
    }
}

pub fn col_sql(ty: ColTy) -> &'static str {
    match ty {
        // SQL Server has a real BIT type; it stores 0/1 and packs 8 to a byte.
        ColTy::Bool => "BIT",
        ColTy::Int => "INT",
        ColTy::BigInt => "BIGINT",
        // Not DECIMAL: M3.6 requires a decimal's original textual precision to
        // survive round-trip, and a fixed declared scale cannot preserve a
        // per-value lexical form — `DECIMAL(38,10)` returns 1.50 as
        // 1.5000000000. Range search is served by a derived sort column, not by
        // this one.
        //
        // `COLLATE Latin1_General_100_BIN2` on both `Numeric` and `Text`: an
        // `NVARCHAR(MAX)` column with no `COLLATE` clause inherits the
        // *database* default collation, and this port's reference columns
        // (`*_ref_id`, `*_ref_type`, `*_ref_url` — `ColTy::Text`) are compared
        // directly against `id`/`rid` (`ColTy::TextC`, always `BIN2`) in a
        // chained-reference search join. Two columns with different explicit
        // collations make SQL Server refuse the comparison outright —
        // "Cannot resolve the collation conflict" (error 468) — found by
        // running a chained reference search live; nothing before this
        // exercised an `id`-to-`id` column comparison rather than an
        // `id`-to-parameter one, so no earlier live run could have surfaced
        // it. Matching collation everywhere sidesteps the whole class rather
        // than special-casing the three reference columns.
        ColTy::Numeric => "NVARCHAR(MAX) COLLATE Latin1_General_100_BIN2",
        ColTy::Text => "NVARCHAR(MAX) COLLATE Latin1_General_100_BIN2",
        // A binary collation, and [BIN2] rather than the deprecated [BIN]:
        // SQL Server's default collation is case- and accent-insensitive, so a
        // column left at the default would silently acquire fuzzy equality —
        // the opposite of what `text COLLATE "C"` means, and enough to break
        // `:exact` matching and key identity. BIN2 also compares by code point
        // rather than by the old byte-wise rule, which is what the folded
        // column's Rust-side ordering assumes.
        ColTy::TextC => "NVARCHAR(450) COLLATE Latin1_General_100_BIN2",
        // U1/U10: this port materializes both adjuncts. The bound is 450 —
        // recorded in the annex as U10 requires. The digest is 64 lowercase hex
        // bytes (SHA-256) stored binary, per U4a — not hex text.
        ColTy::TextIdx => "NVARCHAR(450) COLLATE Latin1_General_100_BIN2",
        ColTy::Digest => "BINARY(32)",
        ColTy::Date => "DATE",
        // DATETIME2(6) rather than DATETIME: DATETIME rounds to 1/300th of a
        // second, which would silently alter a timestamp the hash chain commits
        // to. DATETIMEOFFSET is not used because every value is normalised to
        // UTC in Rust before binding, so an offset column would store a zero
        // offset and invite the belief that local times are preserved.
        ColTy::Timestamptz => "DATETIME2(6)",
        // NVARCHAR(MAX), not the JSON type: the hash chain commits to bytes
        // canonicalised in Rust, and any column that re-normalises what it is
        // given would make the bytes read back differ from the bytes signed.
        // BIN2 for the same reason as `Numeric`/`Text` above: consistent
        // collation across every `NVARCHAR(MAX)` column avoids a 468 the
        // instant one is ever compared against another.
        ColTy::Jsonb => "NVARCHAR(MAX) COLLATE Latin1_General_100_BIN2",
    }
}

/// All statements to install one version's schema, in application order.
pub fn ddl(map: &RelMap) -> Vec<String> {
    ddl_in(map, &map.schema)
}

/// The same statements, targeting an explicit schema name.
pub fn ddl_in(map: &RelMap, schema: &str) -> Vec<String> {
    let mut out = Vec::new();
    let s = schema;
    out.push(format!(
        "IF SCHEMA_ID('{s}') IS NULL EXEC('CREATE SCHEMA [{s}]')"
    ));
    out.push(format!(
        "CREATE TABLE [{s}].[fhir_mssql_meta] ([key] NVARCHAR(450) NOT NULL PRIMARY KEY, [value] NVARCHAR(MAX) NOT NULL)"
    ));
    out.extend(schema_wide_objects(s));
    for rm in map.resources.values() {
        for t in &rm.tables {
            out.push(create_table(s, rm, t));
            if t.kind == TableKind::History {
                out.extend(append_only_triggers(s, &t.name));
            }
        }
        out.extend(search_indexes(s, rm));
    }
    out
}

/// Objects that exist once per schema rather than once per resource, written
/// idempotently so that [init] and `init --upgrade` can both apply them.
///
/// The per-resource table diff in the store cannot see these — they are not
/// in the relational map — so an upgrade applies them explicitly.
#[must_use]
pub fn schema_wide_objects(s: &str) -> Vec<String> {
    let mut out = vec![access_log_table(s), countersign_table(s)];
    out.extend(access_log_indexes(s));
    out
}

/// `ALTER TABLE … ADD` for a history table's audit envelope, so an installed
/// schema gains M3.15/M3.16 without a rewrite. Purely additive: existing rows
/// get `actor = 'unauthenticated'` and a null chain, which `verify-audit`
/// reports as "chain starts here", not as a break.
///
/// **No `COLUMN` keyword.** T-SQL spells this `ALTER TABLE t ADD col type`;
/// `ADD COLUMN` is MySQL and PostgreSQL syntax and SQL Server's parser rejects
/// it outright. This emitted `ADD COLUMN` until audit F-25 — invisible because
/// the live DDL test installs a fresh schema and never takes the upgrade path.
///
/// Unlike PostgreSQL — and unlike MariaDB, which the sibling port exploits —
/// SQL Server has no `ADD … IF NOT EXISTS`, so these are **not** idempotent
/// (M14.18). The caller MUST diff against `sys.columns` first and apply only the
/// columns that are missing.
#[must_use]
pub fn history_audit_columns(schema: &str, table: &str) -> Vec<String> {
    audit_envelope_columns()
        .iter()
        .map(|(name, ty)| format!("ALTER TABLE [{schema}].[{table}] ADD [{name}] {ty}"))
        .collect()
}

/// The audit envelope, defined once so [create_table] and the upgrade path
/// cannot disagree about it.
///
/// [actor] carries a [DEFAULT], following the PostgreSQL original and
/// `fhir-sqlite` rather than the MySQL port. MySQL's omission is its engine's
/// doing — defaults are forbidden on `TEXT` there — and copying it here was a
/// defect, not a departure (audit F-26): SQL Server refuses outright to add a
/// `NOT NULL` column with no default to a table that has rows, which is every
/// history table an upgrade would touch.
///
/// With the default, existing rows are filled by the `ALTER` itself and read as
/// `unauthenticated` — the honest answer for a change recorded before the
/// envelope existed — instead of the upgrade failing.
fn audit_envelope_columns() -> &'static [(&'static str, &'static str)] {
    &[
        ("actor", "NVARCHAR(MAX) NOT NULL DEFAULT 'unauthenticated'"),
        ("actor_source", "NVARCHAR(MAX)"),
        ("client", "NVARCHAR(MAX)"),
        ("request_id", "NVARCHAR(MAX)"),
        ("reason", "NVARCHAR(MAX)"),
        ("prev_hash", "VARBINARY(32)"),
        ("row_hash", "VARBINARY(32)"),
        // SHA3-256 alongside SHA-256 (spec M3.16a): a second chain in a
        // different design family, so one line of cryptanalysis cannot take
        // both. [prev_hash_sha3] is the SHA-3 chain's own predecessor link;
        // the two chains are independent and verified independently.
        ("prev_hash_sha3", "VARBINARY(32)"),
        ("row_hash_sha3", "VARBINARY(32)"),
        // The keyed tag, `<key-id>:<hex>`, or NULL when unkeyed. The key id
        // travels with the tag so a verifier can distinguish "signed with a
        // key I do not hold" from "tampered with" — different claims that
        // must never be conflated — and so rotating a key does not invalidate
        // every historical row at once.
        ("row_mac", "NVARCHAR(MAX)"),
    ]
}

/// The disclosure log (PR12.5): one row per read, not per change.
///
/// A store that records only mutations cannot answer "who looked at this
/// patient", which is the question an audit actually starts with.
///
/// [rtype] and [id] are bounded rather than [NVARCHAR(MAX)] so the auditor's indexes
/// below need no prefix on them.
fn access_log_table(s: &str) -> String {
    format!(
        "CREATE TABLE [{s}].[fhir_mssql_access_log] (\n\
         \x20 [seq] BIGINT IDENTITY(1,1) NOT NULL PRIMARY KEY,\n\
         \x20 [ts] DATETIME2(6) NOT NULL DEFAULT SYSUTCDATETIME(),\n\
         \x20 [request_id] NVARCHAR(MAX),\n\
         \x20 [actor] NVARCHAR(450) NOT NULL,\n\
         \x20 [actor_source] NVARCHAR(MAX),\n\
         \x20 [client] NVARCHAR(MAX),\n\
         \x20 [interaction] NVARCHAR(64) NOT NULL,\n\
         \x20 [rtype] NVARCHAR(64),\n\
         \x20 [id] {ID_COL},\n\
         \x20 [version_id] BIGINT,\n\
         \x20 [outcome] NVARCHAR(64) NOT NULL,\n\
         \x20 [result_count] BIGINT,\n\
         \x20 [reason] NVARCHAR(MAX)\n\
         )"
    )
}

/// Counter-signatures over history rows, appended when a key is retired
/// (spec M3.16d).
///
/// A separate table rather than an update to [row_mac], for two reasons.
/// History is append-only, and re-signing in place would be the application
/// doing exactly what the append-only guard exists to prevent. And the
/// original tag is evidence: replacing it destroys the record of what the
/// retired key attested, leaving no way to tell a legitimate re-signing from
/// a forged one.
///
/// The uniqueness constraint covers [row_mac], which is unbounded [text] in the
/// PostgreSQL original; here it is bounded so the unique key fits in the
/// 900-byte limit (M14.15). A MAC is `<key-id>:<hex>` and never approaches it.
fn countersign_table(s: &str) -> String {
    format!(
        "CREATE TABLE [{s}].[fhir_mssql_countersign] (\n\
         \x20 [seq] BIGINT IDENTITY(1,1) NOT NULL PRIMARY KEY,\n\
         \x20 [rtype] NVARCHAR(64) NOT NULL,\n\
         \x20 [id] {ID_COL} NOT NULL,\n\
         \x20 [version_id] BIGINT NOT NULL,\n\
         \x20 [row_mac] NVARCHAR(450) COLLATE Latin1_General_100_BIN2 NOT NULL,\n\
         \x20 [signed_at] DATETIME2(6) NOT NULL DEFAULT SYSUTCDATETIME(),\n\
         \x20 [actor] NVARCHAR(MAX) NOT NULL,\n\
         \x20 [reason] NVARCHAR(MAX) NOT NULL,\n\
         \x20 CONSTRAINT [fhir_mssql_countersign_uq] UNIQUE ([rtype], [id], [version_id], [row_mac])\n\
         )"
    )
}

fn access_log_indexes(s: &str) -> Vec<String> {
    // The three questions an auditor asks: what happened to this patient,
    // what did this person see, and what happened in this window.
    let t = "fhir_mssql_access_log";
    vec![
        format!(
            "CREATE INDEX [fhir_mssql_access_log_subject_ix] ON [{s}].[{t}] ([rtype], [id], [ts])"
        ),
        format!("CREATE INDEX [fhir_mssql_access_log_actor_ix] ON [{s}].[{t}] ([actor], [ts])"),
        format!("CREATE INDEX [fhir_mssql_access_log_ts_ix] ON [{s}].[{t}] ([ts])"),
    ]
}

/// History is append-only in the database, not merely by convention: an
/// application bug cannot rewrite it, and escaping this is a deliberate act
/// that leaves its own trace (M3.17).
///
/// Two triggers rather than PostgreSQL's one, because an `INSTEAD OF` trigger
/// names a single event. Neither carries a companion `DROP`: T-SQL has
/// `CREATE OR ALTER`, so each guard is **one idempotent statement** (M14.19),
/// and a `DROP`-then-`CREATE` pair MUST NOT be substituted — it leaves a window
/// in which history is unguarded. A unit test below asserts no `DROP TRIGGER`
/// appears, because a reader "restoring" one would reopen that window.
///
/// UPDATE is never permitted: there is no legitimate reason to rewrite a
/// history row in place. DELETE is permitted only when the session variable
/// `@fhir_mssql_erasure` is set, which is how `fhir-mssql purge` performs a
/// GDPR Art. 17 erasure (M3.18) — and which leaves a tombstone naming who did
/// it. The guard is therefore not a defence against the application itself,
/// which can set the variable; it is a defence against the far likelier
/// accident of ordinary code, a migration, or a stray [DELETE] touching
/// history at all.
///
/// The messages omit the table name — the trigger name in the error already
/// identifies it — which also keeps them well inside what [THROW] carries.
#[must_use]
pub fn append_only_triggers(s: &str, table: &str) -> Vec<String> {
    let upd = index_name(table, &["append_only_upd_trg"]);
    let del = index_name(table, &["append_only_del_trg"]);
    vec![
        // `CREATE OR ALTER` is T-SQL's, so each guard is one idempotent
        // statement — closer to the PostgreSQL original than MySQL manages.
        format!(
            "CREATE OR ALTER TRIGGER [{s}].[{upd}] ON [{s}].[{table}] INSTEAD OF UPDATE AS \
             THROW 50000, 'fhir-mssql: UPDATE forbidden; history is append-only (M3.17)', 1"
        ),
        // The erasure escape reads SESSION_CONTEXT, which is per-session and
        // survives inside a transaction — T-SQL's nearest equivalent to
        // PostgreSQL's `SET LOCAL`. `sp_set_session_context` sets it.
        format!(
            "CREATE OR ALTER TRIGGER [{s}].[{del}] ON [{s}].[{table}] INSTEAD OF DELETE AS \
             BEGIN \
             IF ISNULL(CAST(SESSION_CONTEXT(N'fhir_mssql_erasure') AS NVARCHAR(8)), N'') <> N'on' \
               THROW 50000, 'fhir-mssql: DELETE forbidden; history is append-only (M3.17)', 1; \
             ELSE DELETE FROM [{s}].[{table}] WHERE EXISTS \
               (SELECT 1 FROM deleted d WHERE d.[id] = [{s}].[{table}].[id] \
                  AND d.[version_id] = [{s}].[{table}].[version_id]); \
             END"
        ),
    ]
}

/// One index per distinct search-target column set (P6.4).
///
/// A [NVARCHAR(MAX)] column cannot be part of an index key at all here, and the
/// whole key must fit in 900 bytes (M14.15). There is no prefix arithmetic to
/// do — see [index_columns], which drops such columns rather than truncating
/// them, and M14.16 for why that is a recorded departure and not a note.
pub fn search_indexes(schema: &str, rm: &ResourceMap) -> Vec<String> {
    use crate::model::TargetKind;
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for def in &rm.search {
        for t in &def.targets {
            let cols: Vec<&str> = match &t.kind {
                // Index the folded column when there is one: it is what every
                // modifier except `:exact` actually compares.
                TargetKind::Str { col, norm } => vec![norm.as_deref().unwrap_or(col)],
                TargetKind::Number { col } | TargetKind::Uri { col } => {
                    vec![col]
                }
                TargetKind::Token { system, code } => match system {
                    Some(s) => vec![s, code],
                    None => vec![code],
                },
                TargetKind::Date { lo, hi } => match hi {
                    Some(h) => vec![lo, h],
                    None => vec![lo],
                },
                TargetKind::Quantity { value, .. } => vec![value],
                TargetKind::Reference { c_type, c_id, .. } => vec![c_type, c_id],
            };
            let table = &rm.tables[t.table as usize];
            let key = format!("{}:{}", table.name, cols.join(","));
            if !seen.insert(key) {
                continue;
            }
            let collist = index_columns(table, &cols);
            if collist.is_empty() {
                // Every column of this index is NVARCHAR(MAX). Emitting an
                // index with no columns is a syntax error; emitting nothing is
                // an honest gap.
                continue;
            }
            let name = index_name(&table.name, &cols);
            out.push(format!(
                "CREATE INDEX [{name}] ON [{schema}].[{}] ({})",
                table.name,
                collist.join(", ")
            ));
        }
    }
    out
}

/// Render an index's column list, dropping any column SQL Server cannot index.
///
/// **This is the port's sharpest constraint, and it is not a prefix problem.**
/// MySQL indexes a long column by taking its first N bytes; SQL Server has no
/// such thing, and `NVARCHAR(MAX)` cannot participate in an index key at all.
/// The 900-byte key limit is also tighter than MySQL's 3072.
///
/// So the prefix arithmetic the MySQL port needed is gone, and the question
/// becomes which columns are indexable in the first place:
///
/// - `ColTy::TextC` is `NVARCHAR(450)` — 900 bytes, exactly the key limit — and
///   is what every non-`:exact` string search actually compares, because the
///   folded companion column carries that type. So the common case indexes.
/// - `ColTy::Text`, `Numeric`, and `Jsonb` are `NVARCHAR(MAX)` and are skipped.
///   A token's `system`/`code` therefore go unindexed today, which is a real
///   performance gap and is recorded as such rather than hidden: the search is
///   still correct, just a scan.
///
/// **Open decision (M14.16):** the fix is a persisted computed column holding the
/// leading 450 characters, indexed in place of the original — indexable without
/// truncating what is stored. That adds a column per indexed text column and
/// belongs in the generated map, so it is a map change and not a DDL one.
fn index_columns(table: &Table, cols: &[&str]) -> Vec<String> {
    let ty_of = |name: &str| table.cols.iter().find(|c| c.name == name).map(|c| c.ty);
    cols.iter()
        .filter(|c| !ty_of(c).is_some_and(unindexable))
        .map(|c| format!("[{c}]"))
        .collect()
}

/// Is this type stored as `NVARCHAR(MAX)`, which cannot be part of an index key?
fn unindexable(ty: ColTy) -> bool {
    matches!(ty, ColTy::Text | ColTy::Numeric | ColTy::Jsonb)
}

fn index_name(table: &str, cols: &[&str]) -> String {
    let full = format!("{table}_{}_ix", cols.join("_"));
    if full.len() <= 63 {
        return full;
    }
    // FNV-1a of the full name keeps truncated names unique and stable.
    let mut h: u64 = 0xcbf29ce484222325;
    for b in full.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    let hex = format!("{h:016x}");
    let keep: String = full.chars().take(63 - 17).collect();
    format!("{keep}_{hex}")
}

pub fn create_table(schema: &str, rm: &ResourceMap, t: &Table) -> String {
    let base = &rm.base_table().name;
    let mut sql = format!("CREATE TABLE [{schema}].[{}] (\n", t.name);
    match t.kind {
        TableKind::Base => {
            let _ = writeln!(sql, "  [id] {ID_COL} NOT NULL PRIMARY KEY,");
            sql.push_str("  [version_id] BIGINT NOT NULL,\n");
            sql.push_str("  [last_updated] DATETIME2(6) NOT NULL");
            push_data_cols(&mut sql, t);
        }
        TableKind::Elem => {
            // [ords] is the array path, stored as the same text image the
            // PostgreSQL original wrote (M14.9). VARBINARY keeps it one byte
            // per character against the key budget and compares exactly.
            let _ = write!(
                sql,
                "  [rid] {ID_COL} NOT NULL,\n  [ords] VARBINARY(255) NOT NULL"
            );
            push_data_cols(&mut sql, t);
            let _ = write!(
                sql,
                ",\n  PRIMARY KEY ([rid], [ords]),\n\
                 \x20 CONSTRAINT [{}] FOREIGN KEY ([rid]) REFERENCES [{schema}].[{base}] ([id]) ON DELETE CASCADE",
                fk_name(&t.name)
            );
        }
        TableKind::Ext => {
            // The natural key — (rid, path, ords, modifier, ext_ord, leaf) —
            // includes two NVARCHAR(MAX) columns and cannot be a primary key at
            // all: SQL Server has no prefix-index syntax to fall back on, and a
            // prefix would not do anyway — it cannot enforce uniqueness over the
            // full value, so two rows differing only past the prefix would
            // collide and silently lose data. Hence a hash surrogate (M14.15),
            // computed in Rust over the canonically joined natural key.
            let _ = write!(
                sql,
                "  [key_hash] BINARY(32) NOT NULL PRIMARY KEY,\n\
                 \x20 [rid] {ID_COL} NOT NULL,\n\
                 \x20 [path] NVARCHAR(MAX) NOT NULL,\n\
                 \x20 [ords] VARBINARY(255) NOT NULL,\n\
                 \x20 [modifier] BIT NOT NULL,\n\
                 \x20 [ext_ord] SMALLINT NOT NULL,\n\
                 \x20 [url] NVARCHAR(MAX),\n\
                 \x20 [leaf] NVARCHAR(MAX) NOT NULL,\n\
                 \x20 [v_kind] CHAR(1) NOT NULL,\n\
                 \x20 [v_text] NVARCHAR(MAX),\n\
                 \x20 [v_num] NVARCHAR(MAX),\n\
                 \x20 [v_bool] BIT",
            );
            push_adjunct_cols(&mut sql, t);
            let _ = write!(
                sql,
                ",\n  INDEX [{0}] ([rid]),\n\
                 \x20 CONSTRAINT [{1}] FOREIGN KEY ([rid]) REFERENCES [{schema}].[{base}] ([id]) ON DELETE CASCADE",
                rid_index_name(&t.name),
                fk_name(&t.name)
            );
        }
        TableKind::Deep => {
            let _ = write!(
                sql,
                "  [key_hash] BINARY(32) NOT NULL PRIMARY KEY,\n\
                 \x20 [rid] {ID_COL} NOT NULL,\n\
                 \x20 [path] NVARCHAR(MAX) NOT NULL,\n\
                 \x20 [ords] VARBINARY(255) NOT NULL,\n\
                 \x20 [leaf] NVARCHAR(MAX) NOT NULL,\n\
                 \x20 [v_kind] CHAR(1) NOT NULL,\n\
                 \x20 [v_text] NVARCHAR(MAX),\n\
                 \x20 [v_num] NVARCHAR(MAX),\n\
                 \x20 [v_bool] BIT",
            );
            push_adjunct_cols(&mut sql, t);
            let _ = write!(
                sql,
                ",\n  INDEX [{0}] ([rid]),\n\
                 \x20 CONSTRAINT [{1}] FOREIGN KEY ([rid]) REFERENCES [{schema}].[{base}] ([id]) ON DELETE CASCADE",
                rid_index_name(&t.name),
                fk_name(&t.name)
            );
        }
        TableKind::Contained => {
            let _ = write!(
                sql,
                "  [rid] {ID_COL} NOT NULL,\n\
                 \x20 [ord] SMALLINT NOT NULL,\n\
                 \x20 [resource] NVARCHAR(MAX) NOT NULL,\n\
                 \x20 PRIMARY KEY ([rid], [ord]),\n\
                 \x20 CONSTRAINT [{}] FOREIGN KEY ([rid]) REFERENCES [{schema}].[{base}] ([id]) ON DELETE CASCADE",
                fk_name(&t.name)
            );
        }
        TableKind::History => {
            // The audit envelope (M3.15) and the hash chain (M3.16) live on
            // the same row as the change they describe, written by the same
            // statement inside the same transaction: an audit record that can
            // be lost independently of its change is not an audit record.
            let _ = write!(
                sql,
                "  [id] {ID_COL} NOT NULL,\n\
                 \x20 [version_id] BIGINT NOT NULL,\n\
                 \x20 [last_updated] DATETIME2(6) NOT NULL,\n\
                 \x20 [op] CHAR(1) NOT NULL,\n\
                 \x20 [resource] NVARCHAR(MAX)"
            );
            for (name, ty) in audit_envelope_columns() {
                let _ = write!(sql, ",\n  [{name}] {ty}");
            }
            sql.push_str(",\n  PRIMARY KEY ([id], [version_id])");
        }
    }
    sql.push_str("\n)");
    sql
}

/// Foreign-key constraint names share a namespace across the whole schema, not
/// just the table, so they get the same 63-byte fitting as indexes (M14.2).
fn fk_name(table: &str) -> String {
    index_name(table, &["rid_fk"])
}

/// The [rid] lookup index on [Ext]/[Deep], which the read path uses since those
/// tables no longer key on [rid] first.
fn rid_index_name(table: &str) -> String {
    index_name(table, &["rid"])
}

/// Emit the `U1` adjuncts a fixed-shape table carries.
///
/// `Ext` and `Deep` hardcode their data columns, so `push_data_cols` never sees
/// them, and the adjunct columns the generator attached to `url`, `leaf` and
/// `v_text` would never reach the schema — leaving the map describing columns
/// the database does not have (**F-46**).
///
/// This writes nothing on a dialect that indexes its unbounded text type,
/// because `add_adjunct_columns` attached nothing there (`U9`). It must be
/// called *before* the trailing key and constraint clauses: SQLite requires
/// every column definition to precede them.
fn push_adjunct_cols(sql: &mut String, t: &Table) {
    for a in &t.adjunct_cols {
        if let Some(n) = &a.bounded {
            let _ = write!(sql, ",\n  [{n}] {}", col_sql(ColTy::TextIdx));
        }
        if let Some(n) = &a.digest {
            let _ = write!(sql, ",\n  [{n}] {}", col_sql(ColTy::Digest));
        }
    }
}

fn push_data_cols(sql: &mut String, t: &Table) {
    for c in &t.cols {
        let _ = write!(sql, ",\n  [{}] {}", c.name, col_sql(c.ty));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn col(name: &str, ty: ColTy) -> crate::model::Column {
        crate::model::Column {
            name: name.to_string(),
            ty,
            path: String::new(),
        }
    }

    fn table(name: &str, cols: Vec<crate::model::Column>) -> Table {
        Table {
            name: name.to_string(),
            kind: TableKind::Elem,
            path: String::new(),
            cols,
            norm_cols: Vec::new(),
            adjunct_cols: Vec::new(),
        }
    }

    #[test]
    fn no_postgres_only_types_are_emitted() {
        for ty in [
            ColTy::Bool,
            ColTy::Int,
            ColTy::BigInt,
            ColTy::Numeric,
            ColTy::Text,
            ColTy::TextC,
            ColTy::Date,
            ColTy::Timestamptz,
            ColTy::Jsonb,
        ] {
            let s = col_sql(ty);
            for bad in ["jsonb", "bytea", "timestamptz", "smallint[]", "bigserial"] {
                assert!(!s.contains(bad), "{ty:?} maps to {s}, which contains {bad}");
            }
        }
    }

    #[test]
    fn decimal_does_not_become_a_fixed_scale_number() {
        // M3.6: original textual precision must survive round-trip, which
        // DECIMAL(65,30) cannot do — it pads 1.50 to thirty decimal places.
        assert!(col_sql(ColTy::Numeric).starts_with("NVARCHAR(MAX)"));
        assert!(!col_sql(ColTy::Numeric).contains("DECIMAL"));
        assert!(!col_sql(ColTy::Numeric).contains("DOUBLE"));
    }

    #[test]
    fn json_is_stored_as_text_to_preserve_signed_bytes() {
        // A native JSON column re-normalizes on write, so the bytes read back
        // would not be the bytes the hash chain committed to.
        assert!(col_sql(ColTy::Jsonb).starts_with("NVARCHAR(MAX)"));
    }

    #[test]
    fn unbounded_text_columns_share_id_s_collation() {
        // A reference column (`ColTy::Text`) is compared directly against
        // `id`/`rid` (`ColTy::TextC`) in a chained-reference search join;
        // mismatched explicit collations make SQL Server refuse the compare
        // outright (error 468), found by running exactly that query live.
        for ty in [ColTy::Text, ColTy::Numeric, ColTy::Jsonb] {
            assert!(
                col_sql(ty).contains("Latin1_General_100_BIN2"),
                "{ty:?} maps to {}, missing id's collation",
                col_sql(ty)
            );
        }
    }

    #[test]
    fn timestamps_do_not_use_the_2038_type() {
        let s = col_sql(ColTy::Timestamptz);
        assert!(s.starts_with("DATETIME"));
        assert!(!s.contains("TIMESTAMP("));
    }

    #[test]
    fn exact_text_keeps_a_binary_collation() {
        // SQL Server's default collation is accent- and case-insensitive, which
        // would silently make `:exact` matching fuzzy (M14.9). BIN2 rather than
        // the deprecated BIN: BIN2 compares by code point, which is what the
        // folded column's Rust-side ordering assumes.
        assert!(col_sql(ColTy::TextC).contains("Latin1_General_100_BIN2"));
        assert!(!col_sql(ColTy::TextC).contains("_BIN "));
    }

    #[test]
    fn append_only_triggers_are_single_idempotent_statements() {
        let t = append_only_triggers("r5", "patient_history");
        // `CREATE OR ALTER` is T-SQL's, so no companion DROP is needed (M14.19)
        // — closer to the PostgreSQL original than MySQL manages. The absence of
        // the DROP is a correctness decision, not a style one: a DROP/CREATE
        // pair leaves a window in which history is unguarded.
        assert_eq!(t.len(), 2);
        assert!(t.iter().all(|s| s.starts_with("CREATE OR ALTER TRIGGER")));
        assert!(t.iter().all(|s| !s.contains("DROP TRIGGER")));
        assert!(t[0].contains("INSTEAD OF UPDATE"));
        assert!(t[1].contains("INSTEAD OF DELETE"));
        // UPDATE has no escape hatch; DELETE has exactly one.
        assert!(!t[0].contains("SESSION_CONTEXT"));
        assert!(t[1].contains("SESSION_CONTEXT(N'fhir_mssql_erasure')"));
        for stmt in &t {
            assert!(stmt.contains("THROW 50000"), "no THROW in: {stmt}");
        }
    }

    #[test]
    fn max_columns_are_dropped_from_indexes_not_prefixed() {
        // SQL Server has no prefix indexes and cannot key on NVARCHAR(MAX) at
        // all, so the MySQL port's byte arithmetic does not apply. A dropped
        // column is an honest performance gap; a prefix would be a syntax error.
        let t = table(
            "t",
            vec![
                col("folded", ColTy::TextC),
                col("narrative", ColTy::Text),
                col("d", ColTy::Date),
            ],
        );
        assert_eq!(
            index_columns(&t, &["folded", "narrative", "d"]),
            vec!["[folded]", "[d]"],
            "an NVARCHAR(MAX) column must be dropped from the key"
        );
    }

    #[test]
    fn the_folded_column_is_indexable() {
        // This is the one that matters: every non-`:exact` string search
        // compares the folded companion column, so if that were not indexable
        // the port would have no usable string search at all.
        assert!(col_sql(ColTy::TextC).starts_with("NVARCHAR(450)"));
        assert!(!unindexable(ColTy::TextC));
        assert!(unindexable(ColTy::Text));
    }

    #[test]
    fn identifiers_are_bracketed() {
        // Double quotes need QUOTED_IDENTIFIER ON, which is session state.
        let all = schema_wide_objects("r5").join("\n");
        assert!(all.contains('['));
        assert!(!all.contains('`'), "MySQL backquotes survived: {all}");
    }

    #[test]
    fn upgrade_ddl_is_t_sql_not_my_sql() {
        // Audit F-25 and F-26, both from copying the MySQL emitter unread.
        //
        // Nothing installed by the live DDL test reaches this path — it creates
        // a fresh schema, where the envelope arrives via `create_table` — so
        // these two would have survived a green run against a real server.
        let stmts = history_audit_columns("r5", "patient_history");
        assert_eq!(stmts.len(), audit_envelope_columns().len());
        for s in &stmts {
            // F-25: `ADD COLUMN` is MySQL and PostgreSQL; T-SQL spells it `ADD`,
            // and rejects the other outright.
            assert!(!s.contains("ADD COLUMN"), "MySQL ADD COLUMN survived: {s}");
            assert!(s.contains("] ADD ["), "not an ALTER … ADD: {s}");
            assert!(!s.contains('`'), "MySQL backquotes survived: {s}");
        }
        // F-26: SQL Server refuses to add a NOT NULL column with no default to a
        // populated table, and every history table an upgrade touches has rows.
        let actor = &stmts[0];
        assert!(actor.contains("[actor]"), "actor is not first: {actor}");
        assert!(
            actor.contains("NOT NULL DEFAULT 'unauthenticated'"),
            "a NOT NULL column with no default cannot be added to a populated \
             table: {actor}"
        );
    }

    #[test]
    fn schema_wide_objects_no_longer_emit_a_norm_function() {
        // L3: the PostgreSQL original defined `fhir_*_norm` and never called
        // it — never emitted it either, as audit F-18 turned out on fixing.
        // Folding is pure Rust, so this port carries neither the function nor
        // its [unaccent] extension dependency.
        let all = schema_wide_objects("r5").join("\n");
        assert!(!all.contains("CREATE FUNCTION"));
        assert!(!all.contains("_norm"));
        assert!(!all.contains("unaccent"));
        assert!(!all.contains("plpgsql"));
    }
}
