//! DDL emission: the map, rendered as SQLite CREATE statements.
//! Deterministic — same map, same statements, same order.
//!
//! Ported from the PostgreSQL original per spec section 14. Three differences
//! are worth knowing before reading:
//!
//! - **A "schema" is an attached database.** `"{schema}"."{table}"` survives as
//!   a qualified name (M14.17), but there is no `CREATE SCHEMA` — the store
//!   attaches a file. Note that SQLite qualifies the *created object* rather
//!   than the target: it is `CREATE INDEX "s"."ix" ON "t"`, not
//!   `CREATE INDEX "ix" ON "s"."t"`.
//! - **Foreign keys cannot cross databases.** `REFERENCES` is therefore
//!   unqualified and resolves within the table's own database.
//! - **There are no stored functions.** The shared append-only guard becomes
//!   per-table triggers, and the `_norm` folding function is not emitted at
//!   all — it never had a caller (M14.4).

use std::fmt::Write as _;

use crate::model::{ColTy, RelMap, ResourceMap, Table, TableKind};

/// SQLite type affinities for the map's column types (M14.10).
///
/// These are affinities, not constraints: SQLite will store a string in an
/// `INTEGER` column without complaint, so nothing here should be read as the
/// database validating the shredder's output.
pub fn col_sql(ty: ColTy) -> &'static str {
    match ty {
        ColTy::Bool => "INTEGER",
        ColTy::Int => "INTEGER",
        ColTy::BigInt => "INTEGER",
        // Not REAL: M3.6 requires a decimal's original textual precision to
        // survive round-trip, and binary floating point cannot hold `1.50`
        // distinctly from `1.5`. Range search is served by a separate derived
        // sort column (M14.11), not by this one.
        ColTy::Numeric => "TEXT",
        ColTy::Text => "TEXT",
        // SQLite's default collation is already byte-exact; naming it keeps the
        // intent visible next to the PostgreSQL original's `COLLATE "C"`.
        ColTy::TextC => "TEXT COLLATE BINARY",
        // Fixed-width ISO-8601, normalized in Rust so that lexicographic order
        // equals chronological order (M14.12).
        ColTy::Date => "TEXT",
        ColTy::Timestamptz => "TEXT",
        ColTy::Jsonb => "TEXT",
    }
}

/// All statements to install one version's schema, in application order.
pub fn ddl(map: &RelMap) -> Vec<String> {
    ddl_in(map, &map.schema)
}

/// The same statements, targeting an explicit attached-database name.
pub fn ddl_in(map: &RelMap, schema: &str) -> Vec<String> {
    let mut out = Vec::new();
    let s = schema;
    out.push(format!(
        "CREATE TABLE \"{s}\".\"fhir_sqlite_meta\" (\"key\" TEXT PRIMARY KEY, \"value\" TEXT NOT NULL)"
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
/// idempotently so that `init` and `init --upgrade` can both apply them.
///
/// The per-resource table diff in the store cannot see these — they are not
/// in the relational map — so an upgrade applies them explicitly.
#[must_use]
pub fn schema_wide_objects(s: &str) -> Vec<String> {
    let mut out = vec![access_log_table(s), countersign_table(s), erasure_table(s)];
    out.extend(access_log_indexes(s));
    out
}

/// `ALTER TABLE … ADD COLUMN` for a history table's audit envelope, so an
/// installed schema gains M3.15/M3.16 without a rewrite. Purely additive:
/// existing rows get `actor = 'unauthenticated'` and a null chain, which
/// `verify-audit` reports as "chain starts here", not as a break.
///
/// Unlike PostgreSQL, SQLite has no `ADD COLUMN IF NOT EXISTS`, so these are
/// **not** idempotent. The caller MUST diff against `table_info` first and
/// apply only the columns that are missing.
#[must_use]
pub fn history_audit_columns(schema: &str, table: &str) -> Vec<String> {
    audit_envelope_columns()
        .iter()
        .map(|(name, ty)| {
            format!("ALTER TABLE \"{schema}\".\"{table}\" ADD COLUMN \"{name}\" {ty}")
        })
        .collect()
}

/// The audit envelope, defined once so `create_table` and the upgrade path
/// cannot disagree about it.
fn audit_envelope_columns() -> &'static [(&'static str, &'static str)] {
    &[
        ("actor", "TEXT NOT NULL DEFAULT 'unauthenticated'"),
        ("actor_source", "TEXT"),
        ("client", "TEXT"),
        ("request_id", "TEXT"),
        ("reason", "TEXT"),
        ("prev_hash", "BLOB"),
        ("row_hash", "BLOB"),
        // SHA3-256 alongside SHA-256 (spec M3.16a): a second chain in a
        // different design family, so one line of cryptanalysis cannot take
        // both. `prev_hash_sha3` is the SHA-3 chain's own predecessor link;
        // the two chains are independent and verified independently.
        ("prev_hash_sha3", "BLOB"),
        ("row_hash_sha3", "BLOB"),
        // The keyed tag, `<key-id>:<hex>`, or NULL when unkeyed. The key id
        // travels with the tag so a verifier can distinguish "signed with a
        // key I do not hold" from "tampered with" — different claims that
        // must never be conflated — and so rotating a key does not invalidate
        // every historical row at once.
        ("row_mac", "TEXT"),
    ]
}

/// The disclosure log (PR12.5): one row per read, not per change.
///
/// A store that records only mutations cannot answer "who looked at this
/// patient", which is the question an audit actually starts with.
///
/// `ts` has no `DEFAULT`, where the PostgreSQL original had `now()`. SQLite's
/// `strftime` cannot produce the six fractional digits M14.12 fixes on, and a
/// mix of three-digit and six-digit timestamps would not sort correctly as
/// text. The caller supplies the value, normalized in Rust.
fn access_log_table(s: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS \"{s}\".\"fhir_sqlite_access_log\" (\n\
         \x20 \"seq\" INTEGER PRIMARY KEY AUTOINCREMENT,\n\
         \x20 \"ts\" TEXT NOT NULL,\n\
         \x20 \"request_id\" TEXT,\n\
         \x20 \"actor\" TEXT NOT NULL,\n\
         \x20 \"actor_source\" TEXT,\n\
         \x20 \"client\" TEXT,\n\
         \x20 \"interaction\" TEXT NOT NULL,\n\
         \x20 \"rtype\" TEXT,\n\
         \x20 \"id\" TEXT,\n\
         \x20 \"version_id\" INTEGER,\n\
         \x20 \"outcome\" TEXT NOT NULL,\n\
         \x20 \"result_count\" INTEGER,\n\
         \x20 \"reason\" TEXT\n\
         )"
    )
}

/// Counter-signatures over history rows, appended when a key is retired
/// (spec M3.16d).
///
/// A separate table rather than an update to `row_mac`, for two reasons.
/// History is append-only, and re-signing in place would be the application
/// doing exactly what the append-only guard exists to prevent. And the
/// original tag is evidence: replacing it destroys the record of what the
/// retired key attested, leaving no way to tell a legitimate re-signing from
/// a forged one.
fn countersign_table(s: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS \"{s}\".\"fhir_sqlite_countersign\" (\n\
         \x20 \"seq\" INTEGER PRIMARY KEY AUTOINCREMENT,\n\
         \x20 \"rtype\" TEXT NOT NULL,\n\
         \x20 \"id\" TEXT NOT NULL,\n\
         \x20 \"version_id\" INTEGER NOT NULL,\n\
         \x20 \"row_mac\" TEXT NOT NULL,\n\
         \x20 \"signed_at\" TEXT NOT NULL,\n\
         \x20 \"actor\" TEXT NOT NULL,\n\
         \x20 \"reason\" TEXT NOT NULL,\n\
         \x20 UNIQUE (\"rtype\", \"id\", \"version_id\", \"row_mac\")\n\
         )"
    )
}

/// The erasure flag the append-only DELETE trigger consults (M14.22).
///
/// PostgreSQL used a session GUC, `SET LOCAL fhir_sqlite.erasure = 'on'`.
/// SQLite has no session variables, and — the constraint that decides the
/// design — a trigger body may not reference tables in another database, so a
/// `temp.` table cannot serve. This is an ordinary table in the same database
/// as the trigger, which turns out to be *better* than the GUC in one respect:
/// the flag is transactional, so an aborted erasure cannot leave it set.
///
/// `fhir-sqlite purge` inserts a row, performs the deletes, and removes the
/// row, all in one transaction.
fn erasure_table(s: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS \"{s}\".\"fhir_sqlite_erasure\" (\"token\" TEXT PRIMARY KEY)"
    )
}

fn access_log_indexes(s: &str) -> Vec<String> {
    // The three questions an auditor asks: what happened to this patient,
    // what did this person see, and what happened in this window.
    let t = "fhir_sqlite_access_log";
    vec![
        format!(
            "CREATE INDEX IF NOT EXISTS \"{s}\".\"fhir_sqlite_access_log_subject_ix\" ON \"{t}\" (\"rtype\", \"id\", \"ts\")"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS \"{s}\".\"fhir_sqlite_access_log_actor_ix\" ON \"{t}\" (\"actor\", \"ts\")"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS \"{s}\".\"fhir_sqlite_access_log_ts_ix\" ON \"{t}\" (\"ts\")"
        ),
    ]
}

/// History is append-only in the database, not merely by convention: an
/// application bug cannot rewrite it, and escaping this is a deliberate act
/// that leaves its own trace (M3.17).
///
/// Two triggers rather than PostgreSQL's one, because SQLite has no stored
/// functions to share and no combined `BEFORE UPDATE OR DELETE` event.
///
/// UPDATE is never permitted: there is no legitimate reason to rewrite a
/// history row in place. DELETE is permitted only while the erasure flag row
/// exists, which is how `fhir-sqlite purge` performs a GDPR Art. 17 erasure
/// (M3.18) — and which leaves a tombstone naming who did it. The guard is
/// therefore not a defence against the application itself, which can set the
/// flag; it is a defence against the far likelier accident of ordinary code, a
/// migration, or a stray `DELETE` touching history at all.
#[must_use]
pub fn append_only_triggers(s: &str, table: &str) -> Vec<String> {
    let upd = index_name(table, &["append_only_upd_trg"]);
    let del = index_name(table, &["append_only_del_trg"]);
    vec![
        format!(
            "CREATE TRIGGER IF NOT EXISTS \"{s}\".\"{upd}\" BEFORE UPDATE ON \"{table}\" \
             FOR EACH ROW BEGIN \
             SELECT RAISE(ABORT, 'fhir-sqlite: UPDATE on {table} is forbidden; history is append-only (spec M3.17)'); \
             END"
        ),
        format!(
            "CREATE TRIGGER IF NOT EXISTS \"{s}\".\"{del}\" BEFORE DELETE ON \"{table}\" \
             FOR EACH ROW WHEN NOT EXISTS (SELECT 1 FROM \"fhir_sqlite_erasure\") BEGIN \
             SELECT RAISE(ABORT, 'fhir-sqlite: DELETE on {table} is forbidden; history is append-only (spec M3.17)'); \
             END"
        ),
    ]
}

/// One index per distinct search-target column set (P6.4).
///
/// The 63-byte name budget is PostgreSQL's, not SQLite's — SQLite has no
/// meaningful identifier limit. It is kept so that a table's index names are
/// identical across dialects, which makes the two schemas comparable.
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
            let table = &rm.tables[t.table as usize].name;
            let key = format!("{table}:{}", cols.join(","));
            if !seen.insert(key) {
                continue;
            }
            let name = index_name(table, &cols);
            let collist: Vec<String> = cols.iter().map(|c| format!("\"{c}\"")).collect();
            out.push(format!(
                "CREATE INDEX \"{schema}\".\"{name}\" ON \"{table}\" ({})",
                collist.join(", ")
            ));
        }
    }
    out
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
    let mut sql = format!("CREATE TABLE \"{schema}\".\"{}\" (\n", t.name);
    match t.kind {
        TableKind::Base => {
            sql.push_str("  \"id\" TEXT PRIMARY KEY,\n");
            sql.push_str("  \"version_id\" INTEGER NOT NULL,\n");
            sql.push_str("  \"last_updated\" TEXT NOT NULL");
            push_data_cols(&mut sql, t);
        }
        TableKind::Elem => {
            let _ = write!(
                sql,
                "  \"rid\" TEXT NOT NULL REFERENCES \"{base}\" (\"id\") ON DELETE CASCADE,\n  \"ords\" TEXT NOT NULL"
            );
            push_data_cols(&mut sql, t);
            sql.push_str(",\n  PRIMARY KEY (\"rid\", \"ords\")");
        }
        TableKind::Ext => {
            let _ = write!(
                sql,
                "  \"rid\" TEXT NOT NULL REFERENCES \"{base}\" (\"id\") ON DELETE CASCADE,\n\
                 \x20 \"path\" TEXT NOT NULL,\n\
                 \x20 \"ords\" TEXT NOT NULL,\n\
                 \x20 \"modifier\" INTEGER NOT NULL,\n\
                 \x20 \"ext_ord\" INTEGER NOT NULL,\n\
                 \x20 \"url\" TEXT,\n\
                 \x20 \"leaf\" TEXT NOT NULL,\n\
                 \x20 \"v_kind\" TEXT NOT NULL,\n\
                 \x20 \"v_text\" TEXT,\n\
                 \x20 \"v_num\" TEXT,\n\
                 \x20 \"v_bool\" INTEGER,\n\
                 \x20 PRIMARY KEY (\"rid\", \"path\", \"ords\", \"modifier\", \"ext_ord\", \"leaf\")"
            );
        }
        TableKind::Deep => {
            let _ = write!(
                sql,
                "  \"rid\" TEXT NOT NULL REFERENCES \"{base}\" (\"id\") ON DELETE CASCADE,\n\
                 \x20 \"path\" TEXT NOT NULL,\n\
                 \x20 \"ords\" TEXT NOT NULL,\n\
                 \x20 \"leaf\" TEXT NOT NULL,\n\
                 \x20 \"v_kind\" TEXT NOT NULL,\n\
                 \x20 \"v_text\" TEXT,\n\
                 \x20 \"v_num\" TEXT,\n\
                 \x20 \"v_bool\" INTEGER,\n\
                 \x20 PRIMARY KEY (\"rid\", \"path\", \"ords\", \"leaf\")"
            );
        }
        TableKind::Contained => {
            let _ = write!(
                sql,
                "  \"rid\" TEXT NOT NULL REFERENCES \"{base}\" (\"id\") ON DELETE CASCADE,\n\
                 \x20 \"ord\" INTEGER NOT NULL,\n\
                 \x20 \"resource\" TEXT NOT NULL,\n\
                 \x20 PRIMARY KEY (\"rid\", \"ord\")"
            );
        }
        TableKind::History => {
            // The audit envelope (M3.15) and the hash chain (M3.16) live on
            // the same row as the change they describe, written by the same
            // statement inside the same transaction: an audit record that can
            // be lost independently of its change is not an audit record.
            let _ = write!(
                sql,
                "  \"id\" TEXT NOT NULL,\n\
                 \x20 \"version_id\" INTEGER NOT NULL,\n\
                 \x20 \"last_updated\" TEXT NOT NULL,\n\
                 \x20 \"op\" TEXT NOT NULL,\n\
                 \x20 \"resource\" TEXT"
            );
            for (name, ty) in audit_envelope_columns() {
                let _ = write!(sql, ",\n  \"{name}\" {ty}");
            }
            sql.push_str(",\n  PRIMARY KEY (\"id\", \"version_id\")");
        }
    }
    sql.push_str("\n)");
    sql
}

fn push_data_cols(sql: &mut String, t: &Table) {
    for c in &t.cols {
        let _ = write!(sql, ",\n  \"{}\" {}", c.name, col_sql(c.ty));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_postgres_only_types_are_emitted() {
        // Guards against a half-finished port: any of these in the output means
        // a branch of `create_table` was missed.
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
            for bad in ["jsonb", "bytea", "timestamptz", "smallint", "bigserial"] {
                assert!(!s.contains(bad), "{ty:?} maps to {s}, which contains {bad}");
            }
        }
    }

    #[test]
    fn decimal_does_not_become_a_float() {
        // M3.6: a decimal's original textual precision must survive round-trip,
        // which REAL cannot do. This is the assertion that keeps someone from
        // "optimizing" it later.
        assert_eq!(col_sql(ColTy::Numeric), "TEXT");
        assert!(!col_sql(ColTy::Numeric).contains("REAL"));
    }

    #[test]
    fn exact_text_keeps_a_binary_collation() {
        // SQLite's default is already byte-exact, but an implicit default is
        // one refactor away from becoming NOCASE.
        assert!(col_sql(ColTy::TextC).contains("BINARY"));
    }

    #[test]
    fn append_only_triggers_cover_both_events_and_the_erasure_escape() {
        let t = append_only_triggers("r5", "patient_history");
        assert_eq!(t.len(), 2, "SQLite needs one trigger per event");
        assert!(t[0].contains("BEFORE UPDATE ON \"patient_history\""));
        assert!(t[1].contains("BEFORE DELETE ON \"patient_history\""));
        // UPDATE has no escape hatch at all; DELETE has exactly one.
        assert!(!t[0].contains("fhir_sqlite_erasure"));
        assert!(t[1].contains("fhir_sqlite_erasure"));
        for stmt in &t {
            assert!(stmt.contains("RAISE(ABORT"));
            // The trigger is the qualified object; the table is not.
            assert!(stmt.contains("\"r5\".\""));
            assert!(!stmt.contains("ON \"r5\".\""));
        }
    }

    #[test]
    fn erasure_flag_is_a_real_table_in_the_schema() {
        // Not `temp.`: a trigger body may not reference another database, which
        // is the whole reason this is not a temporary table.
        let s = erasure_table("r5");
        assert!(s.contains("\"r5\".\"fhir_sqlite_erasure\""));
        assert!(!s.contains("temp"));
        assert!(!s.contains("TEMPORARY"));
    }

    #[test]
    fn schema_wide_objects_no_longer_emit_a_norm_function() {
        // M14.4: the PostgreSQL original emitted `fhir_*_norm` and never called
        // it. Folding is pure Rust, so the port does not carry the function or
        // its `unaccent` extension dependency.
        let all = schema_wide_objects("r5").join("\n");
        assert!(!all.contains("CREATE FUNCTION"));
        assert!(!all.contains("_norm"));
        assert!(!all.contains("unaccent"));
        assert!(!all.contains("plpgsql"));
    }

    #[test]
    fn indexes_qualify_the_index_not_the_table() {
        // The mistake this catches is writing PostgreSQL's shape,
        // `CREATE INDEX "ix" ON "s"."t"`, which SQLite rejects.
        let all = access_log_indexes("r5").join("\n");
        assert!(all.contains("\"r5\".\"fhir_sqlite_access_log_ts_ix\""));
        assert!(!all.contains("ON \"r5\""));
    }

    #[test]
    fn audit_envelope_is_defined_once() {
        // `create_table` and the upgrade path must not drift; they now read the
        // same list. Compare the column names each produces.
        let alters = history_audit_columns("r5", "patient_history");
        assert_eq!(alters.len(), audit_envelope_columns().len());
        for (name, _) in audit_envelope_columns() {
            assert!(
                alters.iter().any(|a| a.contains(&format!("\"{name}\""))),
                "{name} missing from the upgrade path"
            );
        }
        // And no `IF NOT EXISTS`, which SQLite does not support here.
        assert!(alters.iter().all(|a| !a.contains("IF NOT EXISTS")));
    }

    #[test]
    fn no_create_schema_statement() {
        // A "schema" is an attached database; the store attaches the file.
        let all = schema_wide_objects("r5").join("\n");
        assert!(!all.contains("CREATE SCHEMA"));
    }
}
