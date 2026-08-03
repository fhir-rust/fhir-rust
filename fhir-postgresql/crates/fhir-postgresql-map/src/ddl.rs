//! DDL emission: the map, rendered as PostgreSQL CREATE statements.
//! Deterministic — same map, same statements, same order.

use std::fmt::Write as _;

use crate::model::{ColTy, RelMap, ResourceMap, Table, TableKind};

/// Does this dialect need the unbounded-string adjuncts (`U1`, `U9`)?
///
/// `text` is indexable and comparable directly, so U9 forbids the adjuncts.
///
/// The generator reads this to decide whether to put `<col>_idx` and `<col>_h`
/// in the map at all. `gen` is byte-identical across all six ports (`X15.1`);
/// this constant is in `ddl.rs`, which is the one file a dialect may own.
pub const TEXT_ADJUNCTS: bool = false;

/// Can this dialect **not** index or compare a column of this type as bound?
///
/// The second half of `U1a`'s trigger. The first — that a search reaches the
/// column — is the generator's to know; this is the dialect's, and only the two
/// together justify an adjunct.
///
/// This engine indexes and compares every bound type it emits, so `U1a`'s trigger never fires here.
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
        ColTy::Numeric => false,
        ColTy::Text => false,
        ColTy::TextC => false,
        ColTy::TextIdx => false,
        ColTy::Digest => false,
        ColTy::Date => false,
        ColTy::Timestamptz => false,
        ColTy::Jsonb => false,
    }
}

pub fn col_sql(ty: ColTy) -> &'static str {
    match ty {
        ColTy::Bool => "boolean",
        ColTy::Int => "integer",
        ColTy::BigInt => "bigint",
        ColTy::Numeric => "numeric",
        ColTy::Text => "text",
        ColTy::TextC => "text COLLATE \"C\"",
        // U9: this port does not materialize adjuncts — `TEXT_ADJUNCTS` is
        // false, so a map generated here never carries these columns. The arms
        // exist because `col_sql` must be total; the types are what would be
        // correct if it ever did.
        ColTy::TextIdx => "text COLLATE \"C\"",
        ColTy::Digest => "bytea",
        ColTy::Date => "date",
        ColTy::Timestamptz => "timestamptz",
        ColTy::Jsonb => "jsonb",
    }
}

/// All statements to install one version's schema, in application order.
pub fn ddl(map: &RelMap) -> Vec<String> {
    ddl_in(map, &map.schema)
}

/// The same statements, targeting an explicit schema name (used to stage an
/// install under a temporary schema and rename it into place atomically).
pub fn ddl_in(map: &RelMap, schema: &str) -> Vec<String> {
    let mut out = Vec::new();
    let s = schema;
    out.push(format!("CREATE SCHEMA IF NOT EXISTS \"{s}\""));
    out.push(format!(
        "CREATE TABLE \"{s}\".\"fhir_postgresql_meta\" (\"key\" text PRIMARY KEY, \"value\" text NOT NULL)"
    ));
    out.extend(schema_wide_objects(s));
    for rm in map.resources.values() {
        for t in &rm.tables {
            out.push(create_table(s, rm, t));
            if t.kind == TableKind::History {
                out.push(append_only_trigger(s, &t.name));
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
    let mut out = vec![access_log_table(s), countersign_table(s)];
    out.extend(access_log_indexes(s));
    out.push(append_only_guard(s));
    out
}

/// `ADD COLUMN IF NOT EXISTS` for a history table's audit envelope, so an
/// installed schema gains M3.15/M3.16 without a rewrite. Purely additive:
/// existing rows get `actor = 'unauthenticated'` and a null chain, which
/// `verify-audit` reports as "chain starts here", not as a break.
#[must_use]
pub fn history_audit_columns(schema: &str, table: &str) -> Vec<String> {
    [
        ("actor", "text NOT NULL DEFAULT 'unauthenticated'"),
        ("actor_source", "text"),
        ("client", "text"),
        ("request_id", "text"),
        ("reason", "text"),
        ("prev_hash", "bytea"),
        ("row_hash", "bytea"),
        // SHA3-256 alongside SHA-256 (spec M3.16a): a second chain in a
        // different design family, so one line of cryptanalysis cannot take
        // both. `prev_hash_sha3` is the SHA-3 chain's own predecessor link;
        // the two chains are independent and verified independently.
        ("prev_hash_sha3", "bytea"),
        ("row_hash_sha3", "bytea"),
        // The keyed tag, `<key-id>:<hex>`, or NULL when unkeyed. The key id
        // travels with the tag so a verifier can distinguish "signed with a
        // key I do not hold" from "tampered with" — different claims that
        // must never be conflated — and so rotating a key does not invalidate
        // every historical row at once.
        ("row_mac", "text"),
    ]
    .iter()
    .map(|(name, ty)| {
        format!("ALTER TABLE \"{schema}\".\"{table}\" ADD COLUMN IF NOT EXISTS \"{name}\" {ty}")
    })
    .collect()
}

/// The disclosure log (PR12.5): one row per read, not per change.
///
/// A store that records only mutations cannot answer "who looked at this
/// patient", which is the question an audit actually starts with.
fn access_log_table(s: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS \"{s}\".\"fhir_postgresql_access_log\" (\n\
         \x20 \"seq\" bigserial PRIMARY KEY,\n\
         \x20 \"ts\" timestamptz NOT NULL DEFAULT now(),\n\
         \x20 \"request_id\" text,\n\
         \x20 \"actor\" text NOT NULL,\n\
         \x20 \"actor_source\" text,\n\
         \x20 \"client\" text,\n\
         \x20 \"interaction\" text NOT NULL,\n\
         \x20 \"rtype\" text,\n\
         \x20 \"id\" text,\n\
         \x20 \"version_id\" bigint,\n\
         \x20 \"outcome\" text NOT NULL,\n\
         \x20 \"result_count\" bigint,\n\
         \x20 \"reason\" text\n\
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
        "CREATE TABLE IF NOT EXISTS \"{s}\".\"fhir_postgresql_countersign\" (\n\
         \x20 \"seq\" bigserial PRIMARY KEY,\n\
         \x20 \"rtype\" text NOT NULL,\n\
         \x20 \"id\" text NOT NULL,\n\
         \x20 \"version_id\" bigint NOT NULL,\n\
         \x20 \"row_mac\" text NOT NULL,\n\
         \x20 \"signed_at\" timestamptz NOT NULL DEFAULT now(),\n\
         \x20 \"actor\" text NOT NULL,\n\
         \x20 \"reason\" text NOT NULL,\n\
         \x20 UNIQUE (\"rtype\", \"id\", \"version_id\", \"row_mac\")\n\
         )"
    )
}

fn access_log_indexes(s: &str) -> Vec<String> {
    // The three questions an auditor asks: what happened to this patient,
    // what did this person see, and what happened in this window.
    vec![
        format!(
            "CREATE INDEX IF NOT EXISTS \"fhir_postgresql_access_log_subject_ix\" ON \"{s}\".\"fhir_postgresql_access_log\" (\"rtype\", \"id\", \"ts\")"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS \"fhir_postgresql_access_log_actor_ix\" ON \"{s}\".\"fhir_postgresql_access_log\" (\"actor\", \"ts\")"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS \"fhir_postgresql_access_log_ts_ix\" ON \"{s}\".\"fhir_postgresql_access_log\" (\"ts\")"
        ),
    ]
}

/// The function every history table's trigger calls (M3.17).
///
/// History is append-only in the database, not merely by convention: an
/// application bug cannot rewrite it, and escaping this is a deliberate DBA
/// act (`ALTER TABLE … DISABLE TRIGGER`) that leaves its own trace.
fn append_only_guard(s: &str) -> String {
    // UPDATE is never permitted: there is no legitimate reason to rewrite a
    // history row in place.
    //
    // DELETE is permitted only inside a transaction that has set
    // `fhir_postgresql.erasure`, which is how `fhir-postgresql purge` performs a GDPR Art. 17
    // erasure (M3.18) — and which leaves a tombstone naming who did it. The
    // guard is therefore not a defence against the application itself, which
    // can set the flag; it is a defence against the far likelier accident of
    // ordinary code, a migration, or a stray `DELETE` touching history at all.
    format!(
        "CREATE OR REPLACE FUNCTION \"{s}\".\"fhir_postgresql_history_is_append_only\"() RETURNS trigger AS $$\n\
         BEGIN\n\
         \x20 IF TG_OP = 'DELETE' AND coalesce(current_setting('fhir_postgresql.erasure', true), '') = 'on' THEN\n\
         \x20   RETURN OLD;\n\
         \x20 END IF;\n\
         \x20 RAISE EXCEPTION 'fhir-postgresql: % on %.% is forbidden; history is append-only (spec M3.17)',\n\
         \x20   TG_OP, TG_TABLE_SCHEMA, TG_TABLE_NAME;\n\
         END;\n\
         $$ LANGUAGE plpgsql"
    )
}

pub fn append_only_trigger(s: &str, table: &str) -> String {
    let name = index_name(table, &["append_only_trg"]);
    format!(
        "CREATE OR REPLACE TRIGGER \"{name}\" BEFORE UPDATE OR DELETE ON \"{s}\".\"{table}\" \
         FOR EACH ROW EXECUTE FUNCTION \"{s}\".\"fhir_postgresql_history_is_append_only\"()"
    )
}

/// One index per distinct search-target column set (P6.4). Index names share
/// the relation namespace with tables, so they get a `_ix` suffix and the
/// same 63-byte discipline.
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
                "CREATE INDEX \"{name}\" ON \"{schema}\".\"{table}\" ({})",
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
            sql.push_str("  \"id\" text PRIMARY KEY,\n");
            sql.push_str("  \"version_id\" bigint NOT NULL,\n");
            sql.push_str("  \"last_updated\" timestamptz NOT NULL");
            push_data_cols(&mut sql, t);
        }
        TableKind::Elem => {
            let _ = write!(
                sql,
                "  \"rid\" text NOT NULL REFERENCES \"{schema}\".\"{base}\" (\"id\") ON DELETE CASCADE,\n  \"ords\" smallint[] NOT NULL"
            );
            push_data_cols(&mut sql, t);
            sql.push_str(",\n  PRIMARY KEY (\"rid\", \"ords\")");
        }
        TableKind::Ext => {
            let _ = write!(
                sql,
                "  \"rid\" text NOT NULL REFERENCES \"{schema}\".\"{base}\" (\"id\") ON DELETE CASCADE,\n\
                 \x20 \"path\" text NOT NULL,\n\
                 \x20 \"ords\" smallint[] NOT NULL,\n\
                 \x20 \"modifier\" boolean NOT NULL,\n\
                 \x20 \"ext_ord\" smallint NOT NULL,\n\
                 \x20 \"url\" text,\n\
                 \x20 \"leaf\" text NOT NULL,\n\
                 \x20 \"v_kind\" char(1) NOT NULL,\n\
                 \x20 \"v_text\" text,\n\
                 \x20 \"v_num\" numeric,\n\
                 \x20 \"v_bool\" boolean",
            );
            push_adjunct_cols(&mut sql, t);
            let _ = write!(
                sql,
                ",\n  PRIMARY KEY (\"rid\", \"path\", \"ords\", \"modifier\", \"ext_ord\", \"leaf\")"
            );
        }
        TableKind::Deep => {
            let _ = write!(
                sql,
                "  \"rid\" text NOT NULL REFERENCES \"{schema}\".\"{base}\" (\"id\") ON DELETE CASCADE,\n\
                 \x20 \"path\" text NOT NULL,\n\
                 \x20 \"ords\" smallint[] NOT NULL,\n\
                 \x20 \"leaf\" text NOT NULL,\n\
                 \x20 \"v_kind\" char(1) NOT NULL,\n\
                 \x20 \"v_text\" text,\n\
                 \x20 \"v_num\" numeric,\n\
                 \x20 \"v_bool\" boolean",
            );
            push_adjunct_cols(&mut sql, t);
            let _ = write!(
                sql,
                ",\n  PRIMARY KEY (\"rid\", \"path\", \"ords\", \"leaf\")"
            );
        }
        TableKind::Contained => {
            let _ = write!(
                sql,
                "  \"rid\" text NOT NULL REFERENCES \"{schema}\".\"{base}\" (\"id\") ON DELETE CASCADE,\n\
                 \x20 \"ord\" smallint NOT NULL,\n\
                 \x20 \"resource\" jsonb NOT NULL,\n\
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
                "  \"id\" text NOT NULL,\n\
                 \x20 \"version_id\" bigint NOT NULL,\n\
                 \x20 \"last_updated\" timestamptz NOT NULL,\n\
                 \x20 \"op\" char(1) NOT NULL,\n\
                 \x20 \"resource\" jsonb,\n\
                 \x20 \"actor\" text NOT NULL DEFAULT 'unauthenticated',\n\
                 \x20 \"actor_source\" text,\n\
                 \x20 \"client\" text,\n\
                 \x20 \"request_id\" text,\n\
                 \x20 \"reason\" text,\n\
                 \x20 \"prev_hash\" bytea,\n\
                 \x20 \"row_hash\" bytea,\n\
                 \x20 \"prev_hash_sha3\" bytea,\n\
                 \x20 \"row_hash_sha3\" bytea,\n\
                 \x20 \"row_mac\" text,\n\
                 \x20 PRIMARY KEY (\"id\", \"version_id\")"
            );
        }
    }
    sql.push_str("\n)");
    sql
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
            let _ = write!(sql, ",\n  \"{n}\" {}", col_sql(ColTy::TextIdx));
        }
        if let Some(n) = &a.digest {
            let _ = write!(sql, ",\n  \"{n}\" {}", col_sql(ColTy::Digest));
        }
    }
}

fn push_data_cols(sql: &mut String, t: &Table) {
    for c in &t.cols {
        let _ = write!(sql, ",\n  \"{}\" {}", c.name, col_sql(c.ty));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `L3`: the engine MUST NOT emit a SQL folding function.
    ///
    /// This port carried `fhir_postgresql_norm` — the pre-`P6.6` design — as a
    /// `pub fn` that nothing ever called, long after folding moved into Rust
    /// (`fold.rs`, `P6.6`, `L1`). The other five ports had already dropped it
    /// and grown this guard; audit **F-18** was that this one had not.
    ///
    /// The risk `L3` exists to prevent is a *second definition of string
    /// equality* — one in SQL and one in Rust, which must then agree for every
    /// codepoint in Unicode. A folding function reachable from the schema is an
    /// invitation to write a query against it.
    #[test]
    fn schema_wide_objects_do_not_emit_a_folding_function() {
        let all = schema_wide_objects("r5").join("\n");
        assert!(!all.contains("_norm"), "a folding function survived: {all}");
        assert!(!all.contains("unaccent"), "unaccent dependency survived");
        assert!(
            !all.contains("normalize("),
            "SQL-side normalization survived"
        );
    }

    /// The one function this port *does* emit, so the guard above cannot pass
    /// by accident if `schema_wide_objects` ever returns nothing.
    ///
    /// Unlike the sibling ports, PostgreSQL's append-only guard is a `plpgsql`
    /// trigger function, so "emits no function at all" would be the wrong
    /// assertion here — `L3` prohibits a *folding* function specifically.
    #[test]
    fn the_append_only_guard_is_still_emitted() {
        let all = schema_wide_objects("r5").join("\n");
        assert!(all.contains("fhir_postgresql_history_is_append_only"));
        assert!(all.contains("LANGUAGE plpgsql"));
    }
}
