//! Search-query construction for SQL Server (spec section 14, T64).
//!
//! Forked from the MySQL builder (itself forked from the inherited PostgreSQL
//! one) rather than shared with either: the ports are independent by design
//! (M14.0a), and parameterising one builder at every point the dialects
//! differ would be more coupling than any engine gains.
//!
//! What differs, and why:
//!
//! - **Placeholders are numbered `@P1, @P2, …`**, not `?`. `mssql` binds
//!   positionally regardless of the name printed in the SQL text, but every
//!   value pushed to `binds` still needs a distinct token to reference at its
//!   position in that text, so the placeholder is generated from the bind
//!   vector's length at the moment each value is pushed (`bind()` below) —
//!   never a hand-maintained counter, which would drift the instant a branch
//!   pushes zero or two binds instead of one.
//! - **Identifiers are bracket-quoted** (`M14.5`): `"` is a string delimiter
//!   here unless the session has `QUOTED_IDENTIFIER ON`, which is session
//!   state a query must not depend on.
//! - **Paging is `OFFSET … ROWS FETCH NEXT … ROWS ONLY`** (`M14.22`), not
//!   `LIMIT`.
//! - **`NULLS LAST` does not exist** (`M14.23`). MySQL emulates it with
//!   1-argument `ISNULL(col)`, which returns 0/1; T-SQL's `ISNULL` is strictly
//!   2-argument (`ISNULL(col, replacement)`), so the same trick is spelled
//!   `CASE WHEN col IS NULL THEN 1 ELSE 0 END` instead.
//! - **Numbers compare as `FLOAT`, never `DECIMAL`** (`M14.8`). The column
//!   holds the exact lexical form (`M3.6`), and text compares lexicographically
//!   — under which "9" > "10" — so a search comparison still needs a numeric
//!   cast. `DECIMAL(38, n)` has a fixed declared scale and both loses the
//!   comparison's own precision at the boundary and is what `M14.8` names
//!   outright; `FLOAT` has no declared scale to lose, the same reasoning the
//!   SQLite fork gives for `REAL` over `DECIMAL`.
//! - **No boolean literal.** T-SQL has no bare `TRUE`/`FALSE` on this port's
//!   floor (`M14.3`: SQL Server 2016). An always-false predicate is spelled
//!   `1 = 0`.
//! - **Dates need no cast.** They are `DATETIME2(6)`, and the bound value is
//!   the same fixed-width UTC text the store writes, which SQL Server coerces
//!   correctly through its usual type-precedence rules.
//! - **`ILIKE` becomes `LIKE`.** Correctness does not rest on the engine's
//!   collation: every non-exact string comparison runs against the folded
//!   column whose value came from Rust's `fold::fold` (M14.5-shared / P6.6).
//! - **Token `system`/`code` columns are `NVARCHAR(MAX)` and scan** (`M14.16`).
//!   Bounded/checksum adjuncts (`U1`–`U10`) exist in the generated map for
//!   `string` search parameters but are not wired into `TargetKind` at all —
//!   no port queries them yet — so this builder does not either; a Token
//!   comparison against a `Text` column is correct and unindexed, not broken.

use fhir_mssql_map::fold::{fold, prefix_upper};
use fhir_mssql_map::model::{RelMap, ResourceMap, SearchDef, TargetKind};

use crate::StoreError;

pub struct CompiledQuery {
    pub sql: String,
    /// Same WHERE clause, counting instead of selecting (for _total).
    pub count_sql: String,
    /// How many leading binds `count_sql` uses (cursor/limit/offset binds
    /// belong only to the page query).
    pub count_binds: usize,
    pub binds: Vec<String>,
}

/// One _sort key: parameter code (or _id/_lastUpdated) and direction.
#[derive(Debug, Clone)]
pub struct SortKey {
    pub code: String,
    pub descending: bool,
}

fn quote_ident(s: &str) -> String {
    format!("[{}]", s.replace(']', "]]"))
}

/// Push a value and return the `@Pn` placeholder for its position.
fn bind(binds: &mut Vec<String>, v: &str) -> String {
    binds.push(v.to_string());
    format!("@P{}", binds.len())
}

/// Resolve a sort key to a base-table column. Sorting on child-table
/// parameters is not supported (kept honest with an error, not a wrong
/// order).
fn sort_col(rm: &ResourceMap, key: &SortKey) -> Result<String, StoreError> {
    if key.code == "_id" {
        return Ok("p.[id]".to_string());
    }
    if key.code == "_lastUpdated" {
        return Ok("p.[last_updated]".to_string());
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
    let mut sql = format!("SELECT p.[id] {from}");
    let mut wheres: Vec<String> = Vec::new();
    let mut binds: Vec<String> = Vec::new();

    for (rawname, value) in params {
        let name = rawname.split([':', '.']).next().unwrap_or(rawname);
        if name == "_id" && !rawname.contains('.') {
            let ors: Vec<String> = value
                .split(',')
                .map(|v| format!("p.[id] = {}", bind(&mut binds, v)))
                .collect();
            wheres.push(format!("({})", ors.join(" OR ")));
            continue;
        }
        if name == "_lastUpdated" {
            let mut ors = Vec::new();
            for v in value.split(',') {
                ors.push(date_pred("p.[last_updated]", None, v, &mut binds)?);
            }
            wheres.push(format!("({})", ors.join(" OR ")));
            continue;
        }
        wheres.push(param_predicate(
            map, rm, "p", rawname, value, &mut binds, 0,
        )?);
    }
    // _total counts every match; the keyset cursor narrows only the page
    // query, so it is appended after the count SQL is fixed.
    let mut where_clause = String::new();
    if !wheres.is_empty() {
        where_clause = format!(" WHERE {}", wheres.join(" AND "));
    }
    let count_sql = format!("SELECT COUNT(*) {from}{where_clause}");
    let count_binds = binds.len();
    if let Some(after) = after_id {
        let pred = format!("p.[id] > {}", bind(&mut binds, after));
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
        // T-SQL's ISNULL is 2-argument; the CASE spells the same "nulls sort
        // after values under either direction" MySQL gets from 1-arg ISNULL.
        order.push(format!(
            "CASE WHEN {col} IS NULL THEN 1 ELSE 0 END, {col} {}",
            if key.descending { "DESC" } else { "ASC" }
        ));
    }
    // The tiebreaker is only appended when no caller-supplied key already
    // sorts by `_id`: unlike MySQL, `ORDER BY` on this engine rejects a
    // column named twice ("A column has been specified more than once in
    // the order by list") rather than tolerating the redundancy — found by
    // running a `_id`-sorted search live, which MySQL's own test for the
    // shared builder could not have caught.
    if !sort.iter().any(|k| k.code == "_id") {
        order.push("p.[id] ASC".to_string());
    }
    sql.push_str(&format!(" ORDER BY {}", order.join(", ")));
    // Inlined, not bound: both are Rust `i64` supplied by this store's own
    // caller, never SQL text, so `Display` on an integer cannot inject
    // anything — and it sidesteps needing `OFFSET`/`FETCH` to resolve an
    // `NVARCHAR`-typed parameter to an integer, which T-SQL does not
    // guarantee the way it does for a `CAST` or a typed column comparison.
    sql.push_str(&format!(
        " OFFSET {offset} ROWS FETCH NEXT {count} ROWS ONLY"
    ));
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
    binds: &mut Vec<String>,
    depth: usize,
) -> Result<String, StoreError> {
    let schema = quote_ident(&map.schema);
    // Chain: "refparam[:Type].rest…" — resolve the reference, then apply
    // the rest of the chain to the referenced type.
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
                         WHERE c{d}.[rid] = {alias}.[id] AND "
                    )),
                )
            };
            let ty_p = bind(binds, target_type);
            let t_alias = format!("t{d}");
            let inner = param_predicate(map, target_rm, &t_alias, rest, value, binds, depth + 1)?;
            let core = format!(
                "({ref_alias}.{} = {ty_p} AND EXISTS (SELECT 1 FROM \
                 {schema}.{tbase} {t_alias} WHERE {t_alias}.[id] = \
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
            let pred = target_pred(def, t, v, modifier, binds)?;
            let tname = quote_ident(&rm.tables[t.table as usize].name);
            if t.table == 0 {
                ors.push(pred.replace("«c»", alias));
            } else {
                let ca = format!("c{depth}x");
                ors.push(format!(
                    "EXISTS (SELECT 1 FROM {schema}.{tname} {ca} WHERE {ca}.[rid] = {alias}.[id] AND {})",
                    pred.replace("«c»", &ca)
                ));
            }
        }
    }
    Ok(format!("({})", ors.join(" OR ")))
}

/// One predicate for one target and one value. Column references use the
/// placeholder «c» for the table alias.
fn target_pred(
    def: &SearchDef,
    t: &fhir_mssql_map::model::SearchTarget,
    value: &str,
    modifier: Option<&str>,
    binds: &mut Vec<String>,
) -> Result<String, StoreError> {
    match &t.kind {
        TargetKind::Str { col, norm } => {
            let c = format!("«c».{}", quote_ident(col));
            // `:exact` is defined as the literal string, so it compares the
            // stored value. Everything else is case- and accent-insensitive
            // and compares the folded companion column, folding the search
            // term with the same function that produced it (P6.6). Maps
            // generated before folding existed have no companion column;
            // fall back to bare `LIKE`, which under this column's default
            // collation is case- and accent-insensitive but not the same
            // fold Rust applies. Regenerating the map is the fix, not
            // papering over it here.
            let Some(n) = norm else {
                return match modifier {
                    Some("exact") => Ok(format!("{c} = {}", bind(binds, value))),
                    Some("contains") => Ok(format!(
                        "{c} LIKE {}",
                        bind(binds, &format!("%{}%", like_escape(value)))
                    )),
                    None | Some("text") => Ok(format!(
                        "{c} LIKE {}",
                        bind(binds, &format!("{}%", like_escape(value)))
                    )),
                    Some(m) => Err(StoreError::Unsupported(format!(
                        "unsupported modifier :{m}"
                    ))),
                };
            };
            let nc = format!("«c».{}", quote_ident(n));
            let folded = fold(value);
            match modifier {
                Some("exact") => Ok(format!("{c} = {}", bind(binds, value))),
                Some("contains") => Ok(format!(
                    "{nc} LIKE {} ESCAPE '\\'",
                    bind(binds, &format!("%{}%", like_escape(&folded)))
                )),
                None | Some("text") => {
                    // A range scan rather than `LIKE $1 + '%'`: a bound
                    // parameter defeats prefix extraction from `LIKE` under a
                    // generic plan, so `>=`/`<` against the binary-collation
                    // column is what actually seeks the index.
                    let lo = bind(binds, &folded);
                    match prefix_upper(&folded) {
                        Some(hi) => Ok(format!("({nc} >= {lo} AND {nc} < {})", bind(binds, &hi))),
                        // Empty term, or a term at the top of the codepoint
                        // range: only the lower bound constrains anything.
                        None => Ok(format!("{nc} >= {lo}")),
                    }
                }
                Some(m) => Err(StoreError::Unsupported(format!(
                    "unsupported modifier :{m}"
                ))),
            }
        }
        TargetKind::Token { system, code } => {
            let code_c = format!("«c».{}", quote_ident(code));
            match value.split_once('|') {
                None => Ok(format!("{code_c} = {}", bind(binds, value))),
                Some((sys, cv)) => {
                    let Some(sys_col) = system else {
                        // Bare-code element cannot match a system-qualified
                        // token unless the system part is empty.
                        return if sys.is_empty() {
                            Ok(format!("{code_c} = {}", bind(binds, cv)))
                        } else {
                            Ok("1 = 0".to_string())
                        };
                    };
                    let sys_c = format!("«c».{}", quote_ident(sys_col));
                    if sys.is_empty() {
                        Ok(format!(
                            "({sys_c} IS NULL AND {code_c} = {})",
                            bind(binds, cv)
                        ))
                    } else if cv.is_empty() {
                        Ok(format!("{sys_c} = {}", bind(binds, sys)))
                    } else {
                        Ok(format!(
                            "({sys_c} = {} AND {code_c} = {})",
                            bind(binds, sys),
                            bind(binds, cv)
                        ))
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
                "CAST(«c».{} AS FLOAT) {op} CAST({} AS FLOAT)",
                quote_ident(col),
                bind(binds, num)
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
                "CAST(«c».{} AS FLOAT) {op} CAST({} AS FLOAT)",
                quote_ident(vcol),
                bind(binds, num)
            );
            if let (Some(sys), Some(sc)) = (sys.filter(|s| !s.is_empty()), system) {
                pred = format!(
                    "({pred} AND «c».{} = {})",
                    quote_ident(sc),
                    bind(binds, sys)
                );
            }
            if let (Some(u), Some(cc)) = (unit.filter(|s| !s.is_empty()), code) {
                pred = format!("({pred} AND «c».{} = {})", quote_ident(cc), bind(binds, u));
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
            let url_c = format!("«c».{}", quote_ident(c_url));
            if value.contains("://") || value.starts_with("urn:") || value.starts_with('#') {
                Ok(format!("{url_c} = {}", bind(binds, value)))
            } else if let Some((ty, id)) = value.split_once('/') {
                Ok(format!(
                    "({ty_c} = {} AND {id_c} = {})",
                    bind(binds, ty),
                    bind(binds, id)
                ))
            } else {
                let tymod = modifier.filter(|m| m.chars().next().is_some_and(char::is_uppercase));
                match tymod {
                    Some(ty) => Ok(format!(
                        "({ty_c} = {} AND {id_c} = {})",
                        bind(binds, ty),
                        bind(binds, value)
                    )),
                    None => Ok(format!("{id_c} = {}", bind(binds, value))),
                }
            }
        }
        TargetKind::Uri { col } => {
            let _ = def;
            Ok(format!("«c».{} = {}", quote_ident(col), bind(binds, value)))
        }
    }
}

/// FHIR date comparison against a derived sort column (plus an end column
/// for Periods, where equality means overlap).
fn date_pred(
    lo_col: &str,
    hi_col: Option<&str>,
    value: &str,
    binds: &mut Vec<String>,
) -> Result<String, StoreError> {
    let (prefix, v) = date_prefix(value);
    let Some((lo, hi)) = date_bounds(v) else {
        return Err(StoreError::Unsupported(format!("invalid date value {v:?}")));
    };
    let lo_val = lo;
    let hi_of = |binds: &mut Vec<String>| match &hi {
        // Full timestamps: exclusive upper bound one second later, computed
        // in SQL to sidestep calendar arithmetic.
        None => {
            let lo_b = bind(binds, &lo_val);
            format!("DATEADD(SECOND, 1, {lo_b})")
        }
        Some(h) => bind(binds, h),
    };
    let end = hi_col.map(|h| h.to_string());
    Ok(match prefix {
        "eq" | "ne" => {
            let inner = match &end {
                // Period overlap: starts before the range ends, ends after
                // it begins (open-ended periods overlap everything later).
                Some(e) => {
                    let hi_b = hi_of(binds);
                    let lo_b = bind(binds, &lo_val);
                    format!(
                        "({lo_col} < {hi_b} AND COALESCE({e}, '9999-12-31T23:59:59.999999') >= {lo_b})"
                    )
                }
                None => {
                    let lo_b = bind(binds, &lo_val);
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
            let lo_b = bind(binds, &lo_val);
            format!("{lo_col} < {lo_b}")
        }
        "gt" | "sa" => {
            let hi_b = hi_of(binds);
            match &end {
                Some(e) => format!("COALESCE({e}, '9999-12-31T23:59:59.999999') >= {hi_b}"),
                None => format!("{lo_col} >= {hi_b}"),
            }
        }
        "ge" => {
            let lo_b = bind(binds, &lo_val);
            match &end {
                Some(e) => format!("COALESCE({e}, '9999-12-31T23:59:59.999999') >= {lo_b}"),
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
            format!("{year:04}-01-01T00:00:00Z"),
            Some(format!("{:04}-01-01T00:00:00Z", year + 1)),
        )),
        7 => {
            let m: i64 = v.get(5..7)?.parse().ok()?;
            let (ny, nm) = if m == 12 {
                (year + 1, 1)
            } else {
                (year, m + 1)
            };
            Some((
                format!("{year:04}-{m:02}-01T00:00:00Z"),
                Some(format!("{ny:04}-{nm:02}-01T00:00:00Z")),
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
                format!("{year:04}-{m:02}-{d:02}T00:00:00Z"),
                Some(format!("{ny:04}-{nm:02}-{nd:02}T00:00:00Z")),
            ))
        }
        _ => {
            // Full dateTime: [t, t + 1 second), upper bound left to SQL.
            if !v.contains('T') {
                return None;
            }
            Some((v.to_string(), None))
        }
    }
}

fn like_escape(v: &str) -> String {
    v.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}
