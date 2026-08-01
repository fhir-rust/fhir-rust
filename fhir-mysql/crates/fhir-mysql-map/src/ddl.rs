//! DDL emission: the map, rendered as MySQL CREATE statements.
//! Deterministic — same map, same statements, same order.
//!
//! Ported from the PostgreSQL original per spec section 14. Five differences
//! shape almost everything below:
//!
//! - **Identifiers are backquoted**, since `"` is a string delimiter unless the
//!   server happens to run in `ANSI_QUOTES` mode, which is not something a
//!   schema should depend on.
//! - **A "schema" is a database.** `CREATE SCHEMA` survives as a synonym for
//!   `CREATE DATABASE`, and qualified names work as before (M14.21).
//! - **Index keys are capped at 3072 bytes** and a `TEXT` column cannot be
//!   indexed at all without a prefix length, so `search_indexes` computes one.
//! - **`Ext` and `Deep` cannot keep their natural primary keys**, because those
//!   include unbounded text. They get a hash surrogate (M14.12).
//! - **There are no stored functions for the guard**, so the shared plpgsql
//!   trigger function becomes per-table triggers using `SIGNAL`. The `_norm`
//!   folding function is not emitted at all — it never had a caller (M14.5).

use std::fmt::Write as _;

use crate::model::{ColTy, RelMap, ResourceMap, Table, TableKind};

/// InnoDB's maximum index key, in bytes.
const MAX_KEY_BYTES: usize = 3072;

/// Worst-case bytes per character under `utf8mb4`.
const BYTES_PER_CHAR: usize = 4;

/// A prefix longer than this buys negligible selectivity for the cost.
const MAX_PREFIX_CHARS: usize = 255;

/// The FHIR `id` type is `[A-Za-z0-9\-\.]{1,64}`, so 64 characters is an exact
/// bound rather than a guess (M14.12). Bounding it is what lets `id` and `rid`
/// be keyed and foreign-keyed without a prefix.
const ID_COL: &str = "VARCHAR(64) COLLATE utf8mb4_0900_bin";

/// MySQL type mapping for the map's column types (M14.14).
pub fn col_sql(ty: ColTy) -> &'static str {
    match ty {
        ColTy::Bool => "TINYINT(1)",
        ColTy::Int => "INT",
        ColTy::BigInt => "BIGINT",
        // Not DECIMAL: M3.6 requires a decimal's original textual precision to
        // survive round-trip, and `DECIMAL(65,30)` returns `1.50` as
        // `1.500000000000000000000000000000` — a fixed declared scale cannot
        // preserve a per-value lexical form. Range search is served by a
        // separate derived sort column (M14.15), not by this one.
        ColTy::Numeric => "TEXT",
        ColTy::Text => "TEXT",
        // `utf8mb4_0900_bin`, not `utf8mb4_bin`: the latter is PAD SPACE, under
        // which 'Smith' = 'Smith ' is true. That would quietly widen `:exact`
        // matching and weaken key identity. PostgreSQL's `COLLATE "C"` does not
        // pad, and `utf8mb4_0900_bin` is MySQL 8's NO PAD binary collation.
        // (The sibling fhir-mariadb port uses `utf8mb4_nopad_bin` — MariaDB's
        // own spelling of the same property. The two schemas are not
        // interchangeable, and are not meant to be.)
        ColTy::TextC => "TEXT COLLATE utf8mb4_0900_bin",
        ColTy::Date => "DATE",
        // DATETIME rather than TIMESTAMP: TIMESTAMP silently converts on the
        // session time zone and its range ends in 2038 (M14.16).
        ColTy::Timestamptz => "DATETIME(6)",
        // LONGTEXT rather than JSON, deliberately. The hash chain now commits
        // to bytes canonicalized in Rust (M14.19/M14.20); a `JSON` column would
        // re-normalize what it is given, so the bytes read back would not be
        // the bytes signed and every chain would fail verification.
        ColTy::Jsonb => "LONGTEXT",
    }
}

/// Is this type stored as a text kind that MySQL cannot index whole?
fn needs_index_prefix(ty: ColTy) -> bool {
    matches!(
        ty,
        ColTy::Text | ColTy::TextC | ColTy::Numeric | ColTy::Jsonb
    )
}

/// All statements to install one version's schema, in application order.
pub fn ddl(map: &RelMap) -> Vec<String> {
    ddl_in(map, &map.schema)
}

/// The same statements, targeting an explicit schema name.
pub fn ddl_in(map: &RelMap, schema: &str) -> Vec<String> {
    let mut out = Vec::new();
    let s = schema;
    out.push(format!("CREATE SCHEMA IF NOT EXISTS `{s}`"));
    out.push(format!(
        "CREATE TABLE `{s}`.`fhir_mysql_meta` (`key` VARCHAR(191) NOT NULL PRIMARY KEY, `value` LONGTEXT NOT NULL) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
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

/// Objects that exist once per schema rather than once per resource.
///
/// The per-resource table diff in the store cannot see these — they are not
/// in the relational map — so an upgrade applies them explicitly.
///
/// **Only partly idempotent, and the caller must know which part.** The two
/// tables carry `IF NOT EXISTS`; the three access-log indexes do **not**,
/// because MySQL has no `CREATE INDEX IF NOT EXISTS`. Re-applying this list
/// wholesale therefore fails with `Duplicate key name` on the second run. An
/// upgrade MUST filter the index statements against
/// `information_schema.statistics` first (audit **F-28**, `M14.36`).
///
/// This doc comment claimed unqualified idempotence until F-28; nothing had
/// noticed because nothing re-applied the list — `init` runs once, and there
/// was no upgrade path until now.
#[must_use]
pub fn schema_wide_objects(s: &str) -> Vec<String> {
    let mut out = vec![access_log_table(s), countersign_table(s)];
    out.extend(access_log_indexes(s));
    out
}

/// `ALTER TABLE … ADD COLUMN` for a history table's audit envelope, so an
/// installed schema gains M3.15/M3.16 without a rewrite. Purely additive:
/// existing rows get `actor = 'unauthenticated'` and a null chain, which
/// `verify-audit` reports as "chain starts here", not as a break.
///
/// Unlike PostgreSQL — and unlike MariaDB, which the sibling port exploits —
/// MySQL has no `ADD COLUMN IF NOT EXISTS`, so these are **not** idempotent. The
/// caller MUST diff against `information_schema.columns` first and apply only
/// the columns that are missing.
#[must_use]
pub fn history_audit_columns(schema: &str, table: &str) -> Vec<String> {
    audit_envelope_columns()
        .iter()
        .map(|(name, ty)| format!("ALTER TABLE `{schema}`.`{table}` ADD COLUMN `{name}` {ty}"))
        .collect()
}

/// The audit envelope, defined once so `create_table` and the upgrade path
/// cannot disagree about it.
///
/// `actor` carries no `DEFAULT` because MySQL forbids defaults on `TEXT`
/// columns; the store supplies `'unauthenticated'` explicitly instead. That is
/// a real difference from the PostgreSQL original and the reason an upgraded
/// table's existing rows must be backfilled rather than defaulted.
fn audit_envelope_columns() -> &'static [(&'static str, &'static str)] {
    &[
        ("actor", "TEXT NOT NULL"),
        ("actor_source", "TEXT"),
        ("client", "TEXT"),
        ("request_id", "TEXT"),
        ("reason", "TEXT"),
        ("prev_hash", "VARBINARY(32)"),
        ("row_hash", "VARBINARY(32)"),
        // SHA3-256 alongside SHA-256 (spec M3.16a): a second chain in a
        // different design family, so one line of cryptanalysis cannot take
        // both. `prev_hash_sha3` is the SHA-3 chain's own predecessor link;
        // the two chains are independent and verified independently.
        ("prev_hash_sha3", "VARBINARY(32)"),
        ("row_hash_sha3", "VARBINARY(32)"),
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
/// `rtype` and `id` are bounded rather than `TEXT` so the auditor's indexes
/// below need no prefix on them.
fn access_log_table(s: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS `{s}`.`fhir_mysql_access_log` (\n\
         \x20 `seq` BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,\n\
         \x20 `ts` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),\n\
         \x20 `request_id` TEXT,\n\
         \x20 `actor` TEXT NOT NULL,\n\
         \x20 `actor_source` TEXT,\n\
         \x20 `client` TEXT,\n\
         \x20 `interaction` VARCHAR(64) NOT NULL,\n\
         \x20 `rtype` VARCHAR(64),\n\
         \x20 `id` {ID_COL},\n\
         \x20 `version_id` BIGINT,\n\
         \x20 `outcome` VARCHAR(64) NOT NULL,\n\
         \x20 `result_count` BIGINT,\n\
         \x20 `reason` TEXT\n\
         ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
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
///
/// The uniqueness constraint covers `row_mac`, which is unbounded `text` in the
/// PostgreSQL original; here it is bounded so the unique key fits in 3072
/// bytes. A MAC is `<key-id>:<hex>` and never approaches the limit.
fn countersign_table(s: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS `{s}`.`fhir_mysql_countersign` (\n\
         \x20 `seq` BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,\n\
         \x20 `rtype` VARCHAR(64) NOT NULL,\n\
         \x20 `id` {ID_COL} NOT NULL,\n\
         \x20 `version_id` BIGINT NOT NULL,\n\
         \x20 `row_mac` VARCHAR(191) COLLATE utf8mb4_0900_bin NOT NULL,\n\
         \x20 `signed_at` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),\n\
         \x20 `actor` TEXT NOT NULL,\n\
         \x20 `reason` TEXT NOT NULL,\n\
         \x20 UNIQUE KEY `fhir_mysql_countersign_uq` (`rtype`, `id`, `version_id`, `row_mac`)\n\
         ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
    )
}

fn access_log_indexes(s: &str) -> Vec<String> {
    // The three questions an auditor asks: what happened to this patient,
    // what did this person see, and what happened in this window.
    let t = "fhir_mysql_access_log";
    vec![
        format!(
            "CREATE INDEX `fhir_mysql_access_log_subject_ix` ON `{s}`.`{t}` (`rtype`, `id`, `ts`)"
        ),
        format!(
            "CREATE INDEX `fhir_mysql_access_log_actor_ix` ON `{s}`.`{t}` (`actor`(191), `ts`)"
        ),
        format!("CREATE INDEX `fhir_mysql_access_log_ts_ix` ON `{s}`.`{t}` (`ts`)"),
    ]
}

/// History is append-only in the database, not merely by convention: an
/// application bug cannot rewrite it, and escaping this is a deliberate act
/// that leaves its own trace (M3.17).
///
/// Two triggers rather than PostgreSQL's one, because MySQL allows a trigger to
/// name only a single event. Each is preceded by a `DROP … IF EXISTS` because
/// MySQL has neither `CREATE OR REPLACE TRIGGER` nor `CREATE TRIGGER IF NOT
/// EXISTS`, and the upgrade path has to be able to reapply these. MariaDB does
/// have `OR REPLACE`, and the sibling fhir-mariadb port uses it — the two ports
/// are free to diverge here.
///
/// UPDATE is never permitted: there is no legitimate reason to rewrite a
/// history row in place. DELETE is permitted only when the session variable
/// `@fhir_mysql_erasure` is set, which is how `fhir-mysql purge` performs a
/// GDPR Art. 17 erasure (M3.18) — and which leaves a tombstone naming who did
/// it. The guard is therefore not a defence against the application itself,
/// which can set the variable; it is a defence against the far likelier
/// accident of ordinary code, a migration, or a stray `DELETE` touching
/// history at all.
///
/// `SIGNAL` truncates `MESSAGE_TEXT` at 128 characters, so the messages omit
/// the table name to stay intact — the trigger name in the error already
/// identifies it.
#[must_use]
pub fn append_only_triggers(s: &str, table: &str) -> Vec<String> {
    let upd = index_name(table, &["append_only_upd_trg"]);
    let del = index_name(table, &["append_only_del_trg"]);
    vec![
        format!("DROP TRIGGER IF EXISTS `{s}`.`{upd}`"),
        format!(
            "CREATE TRIGGER `{s}`.`{upd}` BEFORE UPDATE ON `{s}`.`{table}` FOR EACH ROW \
             SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = \
             'fhir-mysql: UPDATE forbidden; history is append-only (M3.17)'"
        ),
        format!("DROP TRIGGER IF EXISTS `{s}`.`{del}`"),
        format!(
            "CREATE TRIGGER `{s}`.`{del}` BEFORE DELETE ON `{s}`.`{table}` FOR EACH ROW \
             BEGIN \
             IF COALESCE(@fhir_mysql_erasure, '') <> 'on' THEN \
             SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = \
             'fhir-mysql: DELETE forbidden; history is append-only (M3.17)'; \
             END IF; \
             END"
        ),
    ]
}

/// One index per distinct search-target column set (P6.4).
///
/// MySQL cannot index a `TEXT` column without a prefix length, and the whole
/// key must fit in 3072 bytes. The prefix is therefore computed from how many
/// text columns share the key rather than hard-coded: a single-column index
/// gets the full 255 characters, and a wider one divides the budget.
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
            let name = index_name(&table.name, &cols);
            out.push(format!(
                "CREATE INDEX `{name}` ON `{schema}`.`{}` ({})",
                table.name,
                collist.join(", ")
            ));
        }
    }
    out
}

/// Render an index's column list, adding a prefix length to each column MySQL
/// cannot index whole.
fn index_columns(table: &Table, cols: &[&str]) -> Vec<String> {
    let ty_of = |name: &str| table.cols.iter().find(|c| c.name == name).map(|c| c.ty);
    let n_text = cols
        .iter()
        .filter(|c| ty_of(c).is_some_and(needs_index_prefix))
        .count();
    let prefix = if n_text == 0 {
        0
    } else {
        // Leave headroom for the non-text members of the key.
        let budget = MAX_KEY_BYTES - 256;
        (budget / (BYTES_PER_CHAR * n_text)).min(MAX_PREFIX_CHARS)
    };
    cols.iter()
        .map(|c| match ty_of(c) {
            Some(ty) if needs_index_prefix(ty) => format!("`{c}`({prefix})"),
            // A column the map does not describe is a system column, which is
            // always bounded.
            _ => format!("`{c}`"),
        })
        .collect()
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
    let mut sql = format!("CREATE TABLE `{schema}`.`{}` (\n", t.name);
    match t.kind {
        TableKind::Base => {
            let _ = writeln!(sql, "  `id` {ID_COL} NOT NULL PRIMARY KEY,");
            sql.push_str("  `version_id` BIGINT NOT NULL,\n");
            sql.push_str("  `last_updated` DATETIME(6) NOT NULL");
            push_data_cols(&mut sql, t);
        }
        TableKind::Elem => {
            // `ords` is the array path, stored as the same text image the
            // PostgreSQL original wrote (M14.9). VARBINARY keeps it one byte
            // per character against the key budget and compares exactly.
            let _ = write!(
                sql,
                "  `rid` {ID_COL} NOT NULL,\n  `ords` VARBINARY(255) NOT NULL"
            );
            push_data_cols(&mut sql, t);
            let _ = write!(
                sql,
                ",\n  PRIMARY KEY (`rid`, `ords`),\n\
                 \x20 CONSTRAINT `{}` FOREIGN KEY (`rid`) REFERENCES `{schema}`.`{base}` (`id`) ON DELETE CASCADE",
                fk_name(&t.name)
            );
        }
        TableKind::Ext => {
            // The natural key — (rid, path, ords, modifier, ext_ord, leaf) —
            // includes two unbounded text columns and cannot be a MySQL primary
            // key. A prefix index would not do: it cannot enforce uniqueness
            // over the full value, so two rows differing only past the prefix
            // would collide and silently lose data. Hence a hash surrogate
            // (M14.12), computed in Rust over the canonically joined natural
            // key.
            let _ = write!(
                sql,
                "  `key_hash` BINARY(32) NOT NULL PRIMARY KEY,\n\
                 \x20 `rid` {ID_COL} NOT NULL,\n\
                 \x20 `path` TEXT NOT NULL,\n\
                 \x20 `ords` VARBINARY(255) NOT NULL,\n\
                 \x20 `modifier` TINYINT(1) NOT NULL,\n\
                 \x20 `ext_ord` SMALLINT NOT NULL,\n\
                 \x20 `url` TEXT,\n\
                 \x20 `leaf` TEXT NOT NULL,\n\
                 \x20 `v_kind` CHAR(1) NOT NULL,\n\
                 \x20 `v_text` TEXT,\n\
                 \x20 `v_num` TEXT,\n\
                 \x20 `v_bool` TINYINT(1),\n\
                 \x20 KEY `{0}` (`rid`),\n\
                 \x20 CONSTRAINT `{1}` FOREIGN KEY (`rid`) REFERENCES `{schema}`.`{base}` (`id`) ON DELETE CASCADE",
                rid_index_name(&t.name),
                fk_name(&t.name)
            );
        }
        TableKind::Deep => {
            let _ = write!(
                sql,
                "  `key_hash` BINARY(32) NOT NULL PRIMARY KEY,\n\
                 \x20 `rid` {ID_COL} NOT NULL,\n\
                 \x20 `path` TEXT NOT NULL,\n\
                 \x20 `ords` VARBINARY(255) NOT NULL,\n\
                 \x20 `leaf` TEXT NOT NULL,\n\
                 \x20 `v_kind` CHAR(1) NOT NULL,\n\
                 \x20 `v_text` TEXT,\n\
                 \x20 `v_num` TEXT,\n\
                 \x20 `v_bool` TINYINT(1),\n\
                 \x20 KEY `{0}` (`rid`),\n\
                 \x20 CONSTRAINT `{1}` FOREIGN KEY (`rid`) REFERENCES `{schema}`.`{base}` (`id`) ON DELETE CASCADE",
                rid_index_name(&t.name),
                fk_name(&t.name)
            );
        }
        TableKind::Contained => {
            let _ = write!(
                sql,
                "  `rid` {ID_COL} NOT NULL,\n\
                 \x20 `ord` SMALLINT NOT NULL,\n\
                 \x20 `resource` LONGTEXT NOT NULL,\n\
                 \x20 PRIMARY KEY (`rid`, `ord`),\n\
                 \x20 CONSTRAINT `{}` FOREIGN KEY (`rid`) REFERENCES `{schema}`.`{base}` (`id`) ON DELETE CASCADE",
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
                "  `id` {ID_COL} NOT NULL,\n\
                 \x20 `version_id` BIGINT NOT NULL,\n\
                 \x20 `last_updated` DATETIME(6) NOT NULL,\n\
                 \x20 `op` CHAR(1) NOT NULL,\n\
                 \x20 `resource` LONGTEXT"
            );
            for (name, ty) in audit_envelope_columns() {
                let _ = write!(sql, ",\n  `{name}` {ty}");
            }
            sql.push_str(",\n  PRIMARY KEY (`id`, `version_id`)");
        }
    }
    sql.push_str("\n) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4");
    sql
}

/// Foreign-key constraint names share a namespace across the whole schema in
/// MySQL, not just the table, so they get the same 63-byte fitting as indexes.
fn fk_name(table: &str) -> String {
    index_name(table, &["rid_fk"])
}

/// The `rid` lookup index on `Ext`/`Deep`, which the read path uses since those
/// tables no longer key on `rid` first.
fn rid_index_name(table: &str) -> String {
    index_name(table, &["rid"])
}

fn push_data_cols(sql: &mut String, t: &Table) {
    for c in &t.cols {
        let _ = write!(sql, ",\n  `{}` {}", c.name, col_sql(c.ty));
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
        assert_eq!(col_sql(ColTy::Numeric), "TEXT");
        assert!(!col_sql(ColTy::Numeric).contains("DECIMAL"));
        assert!(!col_sql(ColTy::Numeric).contains("DOUBLE"));
    }

    #[test]
    fn json_is_stored_as_text_to_preserve_signed_bytes() {
        // A native JSON column re-normalizes on write, so the bytes read back
        // would not be the bytes the hash chain committed to.
        assert_eq!(col_sql(ColTy::Jsonb), "LONGTEXT");
    }

    #[test]
    fn timestamps_do_not_use_the_2038_type() {
        let s = col_sql(ColTy::Timestamptz);
        assert!(s.starts_with("DATETIME"));
        assert!(!s.contains("TIMESTAMP("));
    }

    #[test]
    fn exact_text_keeps_a_binary_collation() {
        // MySQL's default collation is accent- and case-insensitive, which would
        // silently make `:exact` matching fuzzy.
        // NO PAD specifically: `utf8mb4_bin` is PAD SPACE, under which
        // 'Smith' = 'Smith ' is true — which would quietly break `:exact`
        // matching and key identity. PostgreSQL's `COLLATE "C"` never pads.
        assert!(col_sql(ColTy::TextC).contains("utf8mb4_0900_bin"));
    }

    #[test]
    fn identifiers_are_backquoted_not_double_quoted() {
        // Double quotes are string delimiters unless the server runs in
        // ANSI_QUOTES mode, which a schema must not depend on.
        let all = schema_wide_objects("r5").join("\n");
        assert!(all.contains('`'));
        assert!(!all.contains('"'));
    }

    #[test]
    fn append_only_triggers_drop_before_create_and_gate_delete() {
        let t = append_only_triggers("r5", "patient_history");
        // MySQL has neither OR REPLACE nor IF NOT EXISTS for triggers, so each
        // create is paired with a drop to keep the upgrade path reappliable.
        // fhir-mariadb emits two statements here instead of four.
        assert_eq!(t.len(), 4);
        assert!(t[0].starts_with("DROP TRIGGER IF EXISTS"));
        assert!(t[1].contains("BEFORE UPDATE ON"));
        assert!(t[2].starts_with("DROP TRIGGER IF EXISTS"));
        assert!(t[3].contains("BEFORE DELETE ON"));
        // UPDATE has no escape hatch; DELETE has exactly one.
        assert!(!t[1].contains("fhir_mysql_erasure"));
        assert!(t[3].contains("@fhir_mysql_erasure"));
        for stmt in [&t[1], &t[3]] {
            assert!(stmt.contains("SIGNAL SQLSTATE '45000'"));
            // MESSAGE_TEXT is truncated at 128 characters by MySQL.
            let msg = stmt.split("MESSAGE_TEXT = ").nth(1).unwrap();
            let quoted: String = msg.chars().skip(1).take_while(|c| *c != '\'').collect();
            assert!(quoted.len() < 128, "message would be truncated: {quoted}");
        }
    }

    #[test]
    fn index_prefixes_stay_within_the_key_budget() {
        // Three text columns sharing one key must divide the budget, not each
        // claim 255 characters.
        let t = table(
            "t",
            vec![
                col("a", ColTy::Text),
                col("b", ColTy::Text),
                col("c", ColTy::Text),
            ],
        );
        let rendered = index_columns(&t, &["a", "b", "c"]);
        let mut total = 0usize;
        for r in &rendered {
            let n: usize = r
                .split('(')
                .nth(1)
                .expect("text column has a prefix")
                .trim_end_matches(')')
                .parse()
                .unwrap();
            assert!(n > 0);
            total += n * BYTES_PER_CHAR;
        }
        assert!(
            total <= MAX_KEY_BYTES,
            "prefixes total {total} bytes, over the {MAX_KEY_BYTES} limit"
        );
    }

    #[test]
    fn single_text_column_gets_the_full_prefix() {
        let t = table("t", vec![col("a", ColTy::TextC)]);
        assert_eq!(index_columns(&t, &["a"]), vec!["`a`(255)"]);
    }

    #[test]
    fn non_text_columns_get_no_prefix() {
        let t = table("t", vec![col("d", ColTy::Date), col("n", ColTy::Int)]);
        assert_eq!(index_columns(&t, &["d", "n"]), vec!["`d`", "`n`"]);
    }

    #[test]
    fn system_columns_absent_from_the_map_get_no_prefix() {
        // `rid`/`ords` are not in `Table::cols`; they are bounded by
        // construction, so they must not acquire a prefix.
        let t = table("t", vec![col("a", ColTy::Text)]);
        let rendered = index_columns(&t, &["rid", "a"]);
        assert_eq!(rendered[0], "`rid`");
        assert!(rendered[1].starts_with("`a`("));
    }

    #[test]
    fn schema_wide_objects_no_longer_emit_a_norm_function() {
        // M14.5: the PostgreSQL original emitted `fhir_*_norm` and never called
        // it. Folding is pure Rust, so the port carries neither the function nor
        // its `unaccent` extension dependency.
        let all = schema_wide_objects("r5").join("\n");
        assert!(!all.contains("CREATE FUNCTION"));
        assert!(!all.contains("_norm"));
        assert!(!all.contains("unaccent"));
        assert!(!all.contains("plpgsql"));
    }
}
