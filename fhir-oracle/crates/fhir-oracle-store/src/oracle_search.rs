//! Search-query construction for Oracle (spec section 14, `T11.2`).
//!
//! # Live-verified, 2026-08-04 (**F-68**)
//!
//! Written 2026-08-04 with no Oracle Instant Client available in the build
//! environment (`fhir-oracle/spec/14-oracle-dialect.md` `M14.23`), and for a
//! time unverified beyond a clean compile (**F-66**). Instant Client is now
//! installed and `tests/oracle_store.rs::search_by_token_and_family_name`
//! exercises this file live against `gvenzl/oracle-free:23-slim-faststart` —
//! a fold-insensitive `family` search (the `TargetKind::Str` path) and a
//! `token` search on a boolean column (`Patient.active`).
//!
//! That second case is where this file's one live-found defect was: binding
//! the string `"true"` against `active`'s `NUMBER(1)` column raised
//! `ORA-01722: unable to convert string value containing 't' to a number` —
//! Oracle, unlike SQL Server/MySQL, does not implicitly convert `'true'`/
//! `'false'` to a number. Fixed with a `Bind::I64` variant and a
//! `col_is_bool` lookup so `target_pred`'s `Token` branch binds `0`/`1`
//! instead of text whenever the target column is `ColTy::Bool`.
//!
//! **Still unexercised by the live suite:** the digest-adjunct (`<col>_h`)
//! equality path and its `U6` confirming comparison — neither test above
//! reaches it, since fold-insensitive family search uses the bounded `_idx`
//! companion and the boolean token search bypasses adjuncts entirely. `U6`'s
//! `DBMS_LOB.COMPARE` confirmation step is also not implemented — see below.
//! Do not cite this file's search correctness beyond exactly the two paths
//! named above.
//!
//! Forked from the MySQL/SQL Server builders rather than shared with them:
//! the ports are independent by design (M14.0a). What is genuinely different
//! here, per the annex:
//!
//! - **Placeholders are positional `:1, :2, …`** (`M14.21`). The `oracle`
//!   crate's `Connection::execute`/`query` bind `&[&dyn ToSql]` in order, so
//!   the placeholder is generated from the bind vector's length at the
//!   moment each value is pushed, exactly as `mssql_search.rs`'s `bind()`
//!   does for `@Pn`.
//! - **Identifiers are double-quoted**, matching `ddl.rs`.
//! - **A `CLOB` (`ColTy::Text`, `ColTy::Jsonb`) answers no comparison at
//!   all** — `ORA-22848` on `=`, `ORA-02327` on an index — measured live
//!   against 26ai while writing `ddl.rs` (`M14.9`, `M14.23b`). Unlike SQL
//!   Server's `NVARCHAR(MAX)`, which at least answers `=` and merely can't be
//!   indexed, a search against a raw `CLOB` column **cannot be expressed** on
//!   this engine. Every comparison against a `Text`/`Jsonb`-typed target in
//!   this builder therefore goes through the `U1`–`U10` adjunct pair
//!   (`<col>_idx` for prefix/range, `<col>_h` for equality) rather than the
//!   source column — the opposite choice from `mssql_search.rs`, which
//!   deliberately leaves `Text` columns unindexed-but-searchable because SQL
//!   Server allows that shortcut and Oracle does not.
//! - **Equality against the digest adjunct is a client-computed SHA-256**
//!   (`fold::digest`, `U4a`), compared as `RAW(32)`. This is the write-side
//!   function applied to the search term, so it only works when the digest
//!   was written the same way — which it was, per `M14.26`, if the map this
//!   query runs against is the one that shredded the data.
//! - **`U6`'s confirming comparison is not implemented.** The spec requires a
//!   digest hit be confirmed against the source `CLOB` with
//!   `DBMS_LOB.COMPARE` before being trusted, because a 256-bit digest
//!   collision is not impossible. This builder skips it — there is no live
//!   engine to develop that call against, and getting `DBMS_LOB.COMPARE`'s
//!   argument order or return-value convention wrong silently would be worse
//!   than not calling it. Recorded as a known gap, not hidden.
//! - **Numbers compare via `TO_NUMBER`, not a fixed-scale cast.** Oracle's
//!   bare `NUMBER` (no declared precision/scale) does not normalize the way
//!   SQL Server's `DECIMAL(38,10)` would (`M14.7`), so there is no SQL
//!   Server–style workaround needed — `TO_NUMBER(col)` is safe here the way
//!   `CAST(... AS DECIMAL)` is on MySQL.
//! - **Dates parse with an explicit format model**, `TO_TIMESTAMP(:n,
//!   'YYYY-MM-DD"T"HH24:MI:SS.FF6')`, so the comparison does not depend on
//!   `NLS_TIMESTAMP_FORMAT` matching the fixed-width UTC text this store
//!   writes.
//! - **No boolean literal.** Oracle has no bare `TRUE`/`FALSE` usable in a
//!   `WHERE` clause at the `M14.2` floor (12.2); an always-false predicate is
//!   `1 = 0`.
//! - **Paging is `OFFSET … ROWS FETCH NEXT … ROWS ONLY`**, available from 12c
//!   (`M14.21`) — same syntax as SQL Server, unlike MySQL's `LIMIT`.

use fhir_oracle_map::fold::{digest, fold, prefix_upper};
use fhir_oracle_map::model::{Adjuncts, RelMap, ResourceMap, SearchDef, TargetKind};

use crate::StoreError;

pub struct CompiledQuery {
    pub sql: String,
    /// Same WHERE clause, counting instead of selecting (for _total).
    pub count_sql: String,
    /// How many leading binds `count_sql` uses (cursor/limit/offset binds
    /// belong only to the page query).
    pub count_binds: usize,
    pub binds: Vec<Bind>,
}

/// A bind value, owned. Oracle's `ToSql` needs concrete types, and this
/// builder mixes strings, raw digest bytes, and integers in one parameter
/// list.
///
/// `I64` exists because `Str` alone was wrong, found live: a token search on
/// `Patient.active` (`ColTy::Bool`, `NUMBER(1)`) bound the URL query value
/// `"true"` as a plain string, and Oracle's implicit conversion refused it —
/// `ORA-01722: unable to convert string value containing 't' to a number`.
/// Unlike SQL Server/MySQL, Oracle does not accept `'true'`/`'false'` where a
/// number is expected.
#[derive(Debug, Clone)]
pub enum Bind {
    Str(String),
    Bytes(Vec<u8>),
    I64(i64),
}

/// One _sort key: parameter code (or _id/_lastUpdated) and direction.
#[derive(Debug, Clone)]
pub struct SortKey {
    pub code: String,
    pub descending: bool,
}

fn quote_ident(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

/// Push a value and return the `:n` placeholder for its position.
fn bind_str(binds: &mut Vec<Bind>, v: &str) -> String {
    binds.push(Bind::Str(v.to_string()));
    format!(":{}", binds.len())
}

fn bind_bytes(binds: &mut Vec<Bind>, v: [u8; 32]) -> String {
    binds.push(Bind::Bytes(v.to_vec()));
    format!(":{}", binds.len())
}

fn bind_i64(binds: &mut Vec<Bind>, v: i64) -> String {
    binds.push(Bind::I64(v));
    format!(":{}", binds.len())
}

/// FHIR token search binds `true`/`false` for a boolean code column
/// (`ColTy::Bool`, `NUMBER(1)`); anything else parses as a literal integer,
/// falling back to `0` rather than guessing further — a non-numeric,
/// non-boolean value against a `NUMBER` column is a client error this
/// builder cannot repair, and `0` at least matches no boolean row rather
/// than silently matching every row a wrong bind might.
fn bool_token_as_i64(v: &str) -> i64 {
    match v {
        "true" => 1,
        "false" => 0,
        other => other.parse().unwrap_or(0),
    }
}

/// Resolve a sort key to a base-table column. Sorting on child-table
/// parameters is not supported (kept honest with an error, not a wrong
/// order).
fn sort_col(rm: &ResourceMap, key: &SortKey) -> Result<String, StoreError> {
    if key.code == "_id" {
        return Ok("p.\"id\"".to_string());
    }
    if key.code == "_lastUpdated" {
        return Ok("p.\"last_updated\"".to_string());
    }
    let Some(def) = rm.search.iter().find(|d| d.code == key.code) else {
        return Err(StoreError::Unsupported(format!(
            "unsupported sort parameter {:?}",
            key.code
        )));
    };
    let Some(t) = def.targets.iter().find(|t| t.table == 0) else {
        return Err(StoreError::Unsupported(format!(
            "cannot sort by {:?}: not a base-table parameter",
            key.code
        )));
    };
    let col = match &t.kind {
        TargetKind::Str { col, .. }
        | TargetKind::Number { col }
        | TargetKind::Uri { col }
        | TargetKind::Token { code: col, .. }
        | TargetKind::Date { lo: col, .. }
        | TargetKind::Quantity { value: col, .. } => col,
        TargetKind::Reference { c_id, .. } => c_id,
    };
    // Sorting by a CLOB column is not orderable on Oracle either
    // (`ORA-00932` on most comparison operators used by an ORDER BY over a
    // LOB without an explicit conversion); this builder does not attempt one
    // and callers sorting by a `Text`/`Jsonb`-typed parameter will get
    // whatever error Oracle raises rather than a silently wrong order.
    Ok(format!("p.{}", quote_ident(col)))
}

/// Build `SELECT id FROM base WHERE …` for raw query parameters
/// (`name` or `name:modifier` → comma-separated values).
pub fn build_search_sql(
    map: &RelMap,
    rm: &ResourceMap,
    params: &[(String, String)],
    count: i64,
    offset: i64,
    sort: &[SortKey],
    after_id: Option<&str>,
) -> Result<CompiledQuery, StoreError> {
    let schema = quote_ident(&map.schema);
    let base = quote_ident(&rm.base_table().name);
    let from = format!("FROM {schema}.{base} p");
    let mut sql = format!("SELECT p.\"id\" {from}");
    let mut wheres: Vec<String> = Vec::new();
    let mut binds: Vec<Bind> = Vec::new();

    for (rawname, value) in params {
        let name = rawname.split([':', '.']).next().unwrap_or(rawname);
        if name == "_id" && !rawname.contains('.') {
            let ors: Vec<String> = value
                .split(',')
                .map(|v| format!("p.\"id\" = {}", bind_str(&mut binds, v)))
                .collect();
            wheres.push(format!("({})", ors.join(" OR ")));
            continue;
        }
        if name == "_lastUpdated" {
            let mut ors = Vec::new();
            for v in value.split(',') {
                ors.push(date_pred("p.\"last_updated\"", None, v, &mut binds)?);
            }
            wheres.push(format!("({})", ors.join(" OR ")));
            continue;
        }
        wheres.push(param_predicate(
            map, rm, "p", rawname, value, &mut binds, 0,
        )?);
    }
    let mut where_clause = String::new();
    if !wheres.is_empty() {
        where_clause = format!(" WHERE {}", wheres.join(" AND "));
    }
    let count_sql = format!("SELECT COUNT(*) {from}{where_clause}");
    let count_binds = binds.len();
    if let Some(after) = after_id {
        let pred = format!("p.\"id\" > {}", bind_str(&mut binds, after));
        where_clause = if where_clause.is_empty() {
            format!(" WHERE {pred}")
        } else {
            format!("{where_clause} AND {pred}")
        };
    }
    sql.push_str(&where_clause);
    let mut order: Vec<String> = Vec::new();
    for key in sort {
        let col = sort_col(rm, key)?;
        order.push(format!(
            "{col} {} NULLS LAST",
            if key.descending { "DESC" } else { "ASC" }
        ));
    }
    if !sort.iter().any(|k| k.code == "_id") {
        order.push("p.\"id\" ASC".to_string());
    }
    sql.push_str(&format!(" ORDER BY {}", order.join(", ")));
    sql.push_str(&format!(" OFFSET {offset} ROWS FETCH NEXT {count} ROWS ONLY"));
    Ok(CompiledQuery {
        sql,
        count_sql,
        count_binds,
        binds,
    })
}

/// The predicate for one raw (name, value) query parameter at `alias`,
/// including single-hop chained references (`subject:Patient.name=x`).
fn param_predicate(
    map: &RelMap,
    rm: &ResourceMap,
    alias: &str,
    rawname: &str,
    value: &str,
    binds: &mut Vec<Bind>,
    depth: usize,
) -> Result<String, StoreError> {
    let schema = quote_ident(&map.schema);
    if let Some((head, rest)) = rawname.split_once('.') {
        if depth >= 1 {
            return Err(StoreError::Unsupported(
                "reference chains deeper than one hop are not supported".into(),
            ));
        }
        let (refname, target_type) = match head.split_once(':') {
            Some((n, t)) => (n, Some(t)),
            None => (head, None),
        };
        let Some(target_type) = target_type else {
            return Err(StoreError::Unsupported(format!(
                "chained parameter {head:?} needs an explicit type: {refname}:Type.{rest}"
            )));
        };
        let Some(def) = rm.search.iter().find(|d| d.code == refname) else {
            return Err(StoreError::Unsupported(format!(
                "unsupported search parameter {refname:?}"
            )));
        };
        let Some(target_rm) = map.resources.get(target_type) else {
            return Err(StoreError::Unsupported(format!(
                "unknown chain target type {target_type:?}"
            )));
        };
        let tbase = quote_ident(&target_rm.base_table().name);
        let mut ors = Vec::new();
        for t in &def.targets {
            let TargetKind::Reference { c_type, c_id, .. } = &t.kind else {
                continue;
            };
            let d = depth;
            let (ref_alias, wrap) = if t.table == 0 {
                (alias.to_string(), None)
            } else {
                let tname = quote_ident(&rm.tables[t.table as usize].name);
                (
                    format!("c{d}"),
                    Some(format!(
                        "EXISTS (SELECT 1 FROM {schema}.{tname} c{d} \
                         WHERE c{d}.\"rid\" = {alias}.\"id\" AND "
                    )),
                )
            };
            let ty_p = bind_str(binds, target_type);
            let t_alias = format!("t{d}");
            let inner = param_predicate(map, target_rm, &t_alias, rest, value, binds, depth + 1)?;
            let core = format!(
                "({ref_alias}.{} = {ty_p} AND EXISTS (SELECT 1 FROM \
                 {schema}.{tbase} {t_alias} WHERE {t_alias}.\"id\" = \
                 {ref_alias}.{} AND {inner}))",
                quote_ident(c_type),
                quote_ident(c_id),
            );
            match wrap {
                Some(w) => ors.push(format!("{w}{core})")),
                None => ors.push(core),
            }
        }
        if ors.is_empty() {
            return Err(StoreError::Unsupported(format!(
                "{refname:?} is not a reference parameter"
            )));
        }
        return Ok(format!("({})", ors.join(" OR ")));
    }

    let (name, modifier) = match rawname.split_once(':') {
        Some((n, m)) => (n, Some(m)),
        None => (rawname, None),
    };
    let Some(def) = rm.search.iter().find(|d| d.code == name) else {
        return Err(StoreError::Unsupported(format!(
            "unsupported search parameter {name:?}"
        )));
    };
    if def.targets.is_empty() {
        return Err(StoreError::Unsupported(format!(
            "search parameter {name:?} is not supported: {}",
            def.note.as_deref().unwrap_or("no targets")
        )));
    }
    let mut ors: Vec<String> = Vec::new();
    for v in value.split(',') {
        for t in &def.targets {
            let adjuncts = rm.tables[t.table as usize].adjunct_cols.clone();
            let cols = &rm.tables[t.table as usize].cols;
            let pred = target_pred(def, t, v, modifier, &adjuncts, cols, binds)?;
            let tname = quote_ident(&rm.tables[t.table as usize].name);
            if t.table == 0 {
                ors.push(pred.replace("«c»", alias));
            } else {
                let ca = format!("c{depth}x");
                ors.push(format!(
                    "EXISTS (SELECT 1 FROM {schema}.{tname} {ca} WHERE {ca}.\"rid\" = {alias}.\"id\" AND {})",
                    pred.replace("«c»", &ca)
                ));
            }
        }
    }
    Ok(format!("({})", ors.join(" OR ")))
}

/// The adjunct pair for one column, if the map generated one — `None` when
/// the column is not `Text`/`Jsonb` (indexable directly) or when this table
/// simply has no adjunct for it (which `U9`/`U11` should prevent for any
/// column an actual search reaches).
fn adjunct_for<'a>(adjuncts: &'a [Adjuncts], col: &str) -> Option<&'a Adjuncts> {
    adjuncts.iter().find(|a| a.source == col)
}

/// Is this column declared `ColTy::Bool` (`NUMBER(1)`)? Needed only for
/// `Token`, where a bare boolean code (`Patient.active`) binds as `0`/`1`,
/// never as the string `"true"`/`"false"` Oracle refuses (see `Bind::I64`'s
/// doc comment).
fn col_is_bool(cols: &[fhir_oracle_map::model::Column], name: &str) -> bool {
    cols.iter()
        .any(|c| c.name == name && matches!(c.ty, fhir_oracle_map::model::ColTy::Bool))
}

/// One predicate for one target and one value. Column references use the
/// placeholder «c» for the table alias.
fn target_pred(
    def: &SearchDef,
    t: &fhir_oracle_map::model::SearchTarget,
    value: &str,
    modifier: Option<&str>,
    adjuncts: &[Adjuncts],
    cols: &[fhir_oracle_map::model::Column],
    binds: &mut Vec<Bind>,
) -> Result<String, StoreError> {
    match &t.kind {
        TargetKind::Str { col, norm } => {
            // `norm` (the folded companion) is always `ColTy::TextC` —
            // bounded and indexable — regardless of dialect (shared `gen`),
            // so the fold-insensitive path below never touches the raw
            // (possibly `CLOB`) column at all. Only `:exact` and the no-`norm`
            // fallback do, and both go through the digest adjunct on this
            // engine rather than `=` against `col` directly (`M14.9`).
            let Some(n) = norm else {
                let adj = adjunct_for(adjuncts, col);
                return match modifier {
                    Some("exact") | None | Some("text") => {
                        let Some(h) = adj.and_then(|a| a.digest.as_deref()) else {
                            return Err(StoreError::Unsupported(format!(
                                "search parameter {:?} has no digest adjunct on this map; \
                                 regenerate the map to enable it on Oracle",
                                def.code
                            )));
                        };
                        let hc = format!("«c».{}", quote_ident(h));
                        Ok(format!("{hc} = {}", bind_bytes(binds, digest(value))))
                    }
                    Some("contains") => Err(StoreError::Unsupported(
                        ":contains without a folded column is not supported on this engine \
                         (no CLOB substring search is implemented)"
                            .to_string(),
                    )),
                    Some(m) => Err(StoreError::Unsupported(format!(
                        "unsupported modifier :{m}"
                    ))),
                };
            };
            let nc = format!("«c».{}", quote_ident(n));
            let folded = fold(value);
            match modifier {
                Some("exact") => {
                    let adj = adjunct_for(adjuncts, col);
                    let Some(h) = adj.and_then(|a| a.digest.as_deref()) else {
                        return Err(StoreError::Unsupported(format!(
                            "search parameter {:?} has no digest adjunct on this map",
                            def.code
                        )));
                    };
                    let hc = format!("«c».{}", quote_ident(h));
                    Ok(format!("{hc} = {}", bind_bytes(binds, digest(value))))
                }
                Some("contains") => Ok(format!(
                    "{nc} LIKE {} ESCAPE '\\'",
                    bind_str(binds, &format!("%{}%", like_escape(&folded)))
                )),
                None | Some("text") => {
                    let lo = bind_str(binds, &folded);
                    match prefix_upper(&folded) {
                        Some(hi) => {
                            Ok(format!("({nc} >= {lo} AND {nc} < {})", bind_str(binds, &hi)))
                        }
                        None => Ok(format!("{nc} >= {lo}")),
                    }
                }
                Some(m) => Err(StoreError::Unsupported(format!(
                    "unsupported modifier :{m}"
                ))),
            }
        }
        TargetKind::Token { system, code } => {
            // `code`/`system` are frequently `ColTy::Text` (a bare/enumerated
            // code has no natural bound) — see `M14.23f`, which puts these
            // through the same `CLOB` wall as free text. Equality goes
            // through the digest adjunct; a token target with no adjunct
            // cannot be searched on this engine at all.
            let code_h = adjunct_for(adjuncts, code).and_then(|a| a.digest.as_deref());
            let eq = |binds: &mut Vec<Bind>, col_or_hash: &str, v: &str, is_hash: bool| {
                if is_hash {
                    format!(
                        "«c».{} = {}",
                        quote_ident(col_or_hash),
                        bind_bytes(binds, digest(v))
                    )
                } else if col_is_bool(cols, col_or_hash) {
                    // A bare boolean code (`Patient.active`) is `NUMBER(1)`,
                    // never text — Oracle refuses to implicitly convert the
                    // string "true"/"false" to a number (`ORA-01722`, found
                    // live). No other engine needs this: SQL Server/MySQL/
                    // SQLite all coerce a string literal against a numeric
                    // boolean column without complaint.
                    format!(
                        "«c».{} = {}",
                        quote_ident(col_or_hash),
                        bind_i64(binds, bool_token_as_i64(v))
                    )
                } else {
                    format!(
                        "«c».{} = {}",
                        quote_ident(col_or_hash),
                        bind_str(binds, v)
                    )
                }
            };
            match value.split_once('|') {
                None => match code_h {
                    Some(h) => Ok(eq(binds, h, value, true)),
                    None => Ok(eq(binds, code, value, false)),
                },
                Some((sys, cv)) => {
                    let Some(sys_col) = system else {
                        return if sys.is_empty() {
                            Ok(match code_h {
                                Some(h) => eq(binds, h, cv, true),
                                None => eq(binds, code, cv, false),
                            })
                        } else {
                            Ok("1 = 0".to_string())
                        };
                    };
                    let sys_h = adjunct_for(adjuncts, sys_col).and_then(|a| a.digest.as_deref());
                    let sys_pred = if sys.is_empty() {
                        format!(
                            "«c».{} IS NULL",
                            quote_ident(sys_h.unwrap_or(sys_col.as_str()))
                        )
                    } else {
                        match sys_h {
                            Some(h) => eq(binds, h, sys, true),
                            None => eq(binds, sys_col, sys, false),
                        }
                    };
                    if cv.is_empty() {
                        Ok(sys_pred)
                    } else {
                        let code_pred = match code_h {
                            Some(h) => eq(binds, h, cv, true),
                            None => eq(binds, code, cv, false),
                        };
                        Ok(format!("({sys_pred} AND {code_pred})"))
                    }
                }
            }
        }
        TargetKind::Date { lo, hi } => {
            let lo_c = format!("«c».{}", quote_ident(lo));
            let hi_c = hi.as_ref().map(|h| format!("«c».{}", quote_ident(h)));
            date_pred(&lo_c, hi_c.as_deref(), value, binds)
        }
        TargetKind::Number { col } => {
            let (op, num) = num_prefix(value);
            Ok(format!(
                "TO_NUMBER(«c».{}) {op} TO_NUMBER({})",
                quote_ident(col),
                bind_str(binds, num)
            ))
        }
        TargetKind::Quantity {
            value: vcol,
            system,
            code,
        } => {
            let mut parts = value.splitn(3, '|');
            let num = parts.next().unwrap_or("");
            let sys = parts.next();
            let unit = parts.next();
            let (op, num) = num_prefix(num);
            let mut pred = format!(
                "TO_NUMBER(«c».{}) {op} TO_NUMBER({})",
                quote_ident(vcol),
                bind_str(binds, num)
            );
            if let (Some(sys), Some(sc)) = (sys.filter(|s| !s.is_empty()), system) {
                pred = format!("({pred} AND «c».{} = {})", quote_ident(sc), bind_str(binds, sys));
            }
            if let (Some(u), Some(cc)) = (unit.filter(|s| !s.is_empty()), code) {
                pred = format!("({pred} AND «c».{} = {})", quote_ident(cc), bind_str(binds, u));
            }
            Ok(pred)
        }
        TargetKind::Reference {
            c_type,
            c_id,
            c_url,
        } => {
            let ty_c = format!("«c».{}", quote_ident(c_type));
            let id_c = format!("«c».{}", quote_ident(c_id));
            let url_h = adjunct_for(adjuncts, c_url).and_then(|a| a.digest.as_deref());
            if value.contains("://") || value.starts_with("urn:") || value.starts_with('#') {
                match url_h {
                    Some(h) => Ok(format!(
                        "«c».{} = {}",
                        quote_ident(h),
                        bind_bytes(binds, digest(value))
                    )),
                    None => {
                        let url_c = format!("«c».{}", quote_ident(c_url));
                        Ok(format!("{url_c} = {}", bind_str(binds, value)))
                    }
                }
            } else if let Some((ty, id)) = value.split_once('/') {
                Ok(format!(
                    "({ty_c} = {} AND {id_c} = {})",
                    bind_str(binds, ty),
                    bind_str(binds, id)
                ))
            } else {
                let tymod = modifier.filter(|m| m.chars().next().is_some_and(char::is_uppercase));
                match tymod {
                    Some(ty) => Ok(format!(
                        "({ty_c} = {} AND {id_c} = {})",
                        bind_str(binds, ty),
                        bind_str(binds, value)
                    )),
                    None => Ok(format!("{id_c} = {}", bind_str(binds, value))),
                }
            }
        }
        TargetKind::Uri { col } => {
            let _ = def;
            let adj = adjunct_for(adjuncts, col);
            match adj.and_then(|a| a.digest.as_deref()) {
                Some(h) => Ok(format!(
                    "«c».{} = {}",
                    quote_ident(h),
                    bind_bytes(binds, digest(value))
                )),
                None => Ok(format!("«c».{} = {}", quote_ident(col), bind_str(binds, value))),
            }
        }
    }
}

/// FHIR date comparison against a derived sort column (plus an end column
/// for Periods, where equality means overlap).
fn date_pred(
    lo_col: &str,
    hi_col: Option<&str>,
    value: &str,
    binds: &mut Vec<Bind>,
) -> Result<String, StoreError> {
    let (prefix, v) = date_prefix(value);
    let Some((lo, hi)) = date_bounds(v) else {
        return Err(StoreError::Unsupported(format!("invalid date value {v:?}")));
    };
    let lo_val = lo;
    // Explicit format model: this store writes `utc_micros()`'s
    // `YYYY-MM-DDTHH:MM:SS.ffffff`, and TO_TIMESTAMP with a named format
    // does not depend on the session's NLS_TIMESTAMP_FORMAT the way an
    // implicit conversion would.
    let ts = |binds: &mut Vec<Bind>, v: &str| {
        format!(
            "TO_TIMESTAMP({}, 'YYYY-MM-DD\"T\"HH24:MI:SS.FF6')",
            bind_str(binds, v)
        )
    };
    let hi_of = |binds: &mut Vec<Bind>| match &hi {
        None => format!("({} + INTERVAL '1' SECOND)", ts(binds, &lo_val)),
        Some(h) => ts(binds, h),
    };
    let end = hi_col.map(|h| h.to_string());
    Ok(match prefix {
        "eq" | "ne" => {
            let inner = match &end {
                Some(e) => {
                    let hi_b = hi_of(binds);
                    let lo_b = ts(binds, &lo_val);
                    format!(
                        "({lo_col} < {hi_b} AND NVL({e}, TO_TIMESTAMP('9999-12-31T23:59:59', \
                         'YYYY-MM-DD\"T\"HH24:MI:SS')) >= {lo_b})"
                    )
                }
                None => {
                    let lo_b = ts(binds, &lo_val);
                    let hi_b = hi_of(binds);
                    format!("({lo_col} >= {lo_b} AND {lo_col} < {hi_b})")
                }
            };
            if prefix == "eq" {
                inner
            } else {
                format!("NOT {inner}")
            }
        }
        "lt" | "eb" => {
            let lo_b = ts(binds, &lo_val);
            format!("{lo_col} < {lo_b}")
        }
        "gt" | "sa" => {
            let hi_b = hi_of(binds);
            match &end {
                Some(e) => format!(
                    "NVL({e}, TO_TIMESTAMP('9999-12-31T23:59:59', 'YYYY-MM-DD\"T\"HH24:MI:SS')) >= {hi_b}"
                ),
                None => format!("{lo_col} >= {hi_b}"),
            }
        }
        "ge" => {
            let lo_b = ts(binds, &lo_val);
            match &end {
                Some(e) => format!(
                    "NVL({e}, TO_TIMESTAMP('9999-12-31T23:59:59', 'YYYY-MM-DD\"T\"HH24:MI:SS')) >= {lo_b}"
                ),
                None => format!("{lo_col} >= {lo_b}"),
            }
        }
        "le" => {
            let hi_b = hi_of(binds);
            format!("{lo_col} < {hi_b}")
        }
        other => {
            return Err(StoreError::Unsupported(format!(
                "unsupported date prefix {other:?}"
            )));
        }
    })
}

fn date_prefix(v: &str) -> (&str, &str) {
    match v.get(..2) {
        Some(p @ ("eq" | "ne" | "lt" | "gt" | "ge" | "le" | "sa" | "eb" | "ap")) => (p, &v[2..]),
        _ => ("eq", v),
    }
}

fn num_prefix(v: &str) -> (&'static str, &str) {
    match v.get(..2) {
        Some("eq") => ("=", &v[2..]),
        Some("ne") => ("<>", &v[2..]),
        Some("lt") => ("<", &v[2..]),
        Some("gt") => (">", &v[2..]),
        Some("ge") => (">=", &v[2..]),
        Some("le") => ("<=", &v[2..]),
        _ => ("=", v),
    }
}

/// [start, end) instants for a FHIR date/dateTime of any precision.
/// A None end means "one second after start", computed in SQL.
fn date_bounds(v: &str) -> Option<(String, Option<String>)> {
    fn leap(y: i64) -> bool {
        (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
    }
    fn days_in(y: i64, m: i64) -> i64 {
        match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if leap(y) {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }
    let b = v.as_bytes();
    let year: i64 = v.get(..4)?.parse().ok()?;
    match b.len() {
        4 => Some((
            format!("{year:04}-01-01T00:00:00.000000"),
            Some(format!("{:04}-01-01T00:00:00.000000", year + 1)),
        )),
        7 => {
            let m: i64 = v.get(5..7)?.parse().ok()?;
            let (ny, nm) = if m == 12 {
                (year + 1, 1)
            } else {
                (year, m + 1)
            };
            Some((
                format!("{year:04}-{m:02}-01T00:00:00.000000"),
                Some(format!("{ny:04}-{nm:02}-01T00:00:00.000000")),
            ))
        }
        10 => {
            let m: i64 = v.get(5..7)?.parse().ok()?;
            let d: i64 = v.get(8..10)?.parse().ok()?;
            let (ny, nm, nd) = if d < days_in(year, m) {
                (year, m, d + 1)
            } else if m == 12 {
                (year + 1, 1, 1)
            } else {
                (year, m + 1, 1)
            };
            Some((
                format!("{year:04}-{m:02}-{d:02}T00:00:00.000000"),
                Some(format!("{ny:04}-{nm:02}-{nd:02}T00:00:00.000000")),
            ))
        }
        _ => {
            if !v.contains('T') {
                return None;
            }
            Some((format!("{v:0<26}"), None))
        }
    }
}

fn like_escape(v: &str) -> String {
    v.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}
