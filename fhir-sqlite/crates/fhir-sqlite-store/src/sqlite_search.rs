//! Search-query construction for SQLite (spec section 14, T64).
//!
//! Forked from the inherited PostgreSQL `search` module rather than shared with
//! it: the two ports are independent by design, and a shared builder would have
//! to be parameterised at every one of the ~18 points where the dialects differ,
//! which is more coupling than either engine benefits from.
//!
//! What actually differs, and why:
//!
//! - **Placeholders** are `?n`, not `$n`. SQLite numbers parameters too, so the
//!   builder's bind bookkeeping carries over unchanged.
//! - **No casts.** Everything is stored with TEXT affinity (M14.10), so
//!   `($n::text)::numeric` and friends have nothing to convert.
//! - **Numbers compare as REAL, explicitly.** A numeric column holds its exact
//!   lexical form (M3.6 requires that to survive round-trip), and text compares
//!   lexicographically, under which "9" > "10". `CAST(… AS REAL)` restores
//!   arithmetic ordering. It costs the index — the range scan becomes a table
//!   scan — which is the price of correctness until the derived sort columns of
//!   M14.11 exist. Correct and slow beats fast and wrong.
//! - **Dates need no cast.** They are stored as fixed-width ISO-8601 in UTC
//!   (M14.12), so lexicographic order already is chronological order — which is
//!   the whole reason for fixing the width.
//! - **`infinity` becomes a sentinel string**, `9999-12-31T23:59:59.999999Z`,
//!   for the same reason: it has to sort above every real timestamp *as text*.
//! - **`ILIKE` becomes `LIKE`.** SQLite's `LIKE` is case-insensitive for ASCII
//!   only, but that is not what carries correctness here: every non-exact string
//!   comparison runs against the folded column, whose value came from Rust's
//!   `fold::fold` (M14.4). The engine's collation is not load-bearing.

use fhir_sqlite_map::fold::{fold, prefix_upper};
use fhir_sqlite_map::model::{RelMap, ResourceMap, SearchDef, TargetKind};

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
    Ok(format!("p.\"{col}\""))
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
    let schema = map.schema.as_str();
    let base = &rm.base_table().name;
    let from = format!("FROM \"{schema}\".\"{base}\" p");
    let mut sql = format!("SELECT p.\"id\" {from}");
    let mut wheres: Vec<String> = Vec::new();
    let mut binds: Vec<String> = Vec::new();

    for (rawname, value) in params {
        let name = rawname.split([':', '.']).next().unwrap_or(rawname);
        if name == "_id" && !rawname.contains('.') {
            let ors: Vec<String> = value
                .split(',')
                .map(|v| {
                    binds.push(v.to_string());
                    format!("p.\"id\" = ?{}", binds.len())
                })
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
    // _total counts every match; the keyset cursor narrows only the page
    // query, so it is appended after the count SQL is fixed.
    let mut where_clause = String::new();
    if !wheres.is_empty() {
        where_clause = format!(" WHERE {}", wheres.join(" AND "));
    }
    let count_sql = format!("SELECT count(*) {from}{where_clause}");
    let count_binds = binds.len();
    if let Some(after) = after_id {
        binds.push(after.to_string());
        let pred = format!("p.\"id\" > ?{}", binds.len());
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
    order.push("p.\"id\" ASC".to_string());
    sql.push_str(&format!(" ORDER BY {}", order.join(", ")));
    binds.push(count.to_string());
    sql.push_str(&format!(" LIMIT ?{}", binds.len()));
    binds.push(offset.to_string());
    sql.push_str(&format!(" OFFSET ?{}", binds.len()));
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
    let schema = map.schema.as_str();
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
        let tbase = &target_rm.base_table().name;
        let mut ors = Vec::new();
        for t in &def.targets {
            let TargetKind::Reference { c_type, c_id, .. } = &t.kind else {
                continue;
            };
            let d = depth;
            let (ref_alias, wrap) = if t.table == 0 {
                (alias.to_string(), None)
            } else {
                let tname = &rm.tables[fhir_sqlite_map::model::index(t.table)].name;
                (
                    format!("c{d}"),
                    Some(format!(
                        "EXISTS (SELECT 1 FROM \"{schema}\".\"{tname}\" c{d}                          WHERE c{d}.\"rid\" = {alias}.\"id\" AND "
                    )),
                )
            };
            binds.push(target_type.to_string());
            let ty_bind = binds.len();
            let t_alias = format!("t{d}");
            let inner = param_predicate(map, target_rm, &t_alias, rest, value, binds, depth + 1)?;
            let core = format!(
                "({ref_alias}.\"{c_type}\" = ?{ty_bind} AND EXISTS (SELECT 1 FROM                  \"{schema}\".\"{tbase}\" {t_alias} WHERE {t_alias}.\"id\" =                  {ref_alias}.\"{c_id}\" AND {inner}))"
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
            let cols = &rm.tables[fhir_sqlite_map::model::index(t.table)].cols;
            let pred = target_pred(def, t, v, modifier, cols, binds)?;
            let tname = &rm.tables[fhir_sqlite_map::model::index(t.table)].name;
            if t.table == 0 {
                ors.push(pred.replace("«c»", alias));
            } else {
                let ca = format!("c{depth}x");
                ors.push(format!(
                    "EXISTS (SELECT 1 FROM \"{schema}\".\"{tname}\" {ca} WHERE {ca}.\"rid\" = {alias}.\"id\" AND {})",
                    pred.replace("«c»", &ca)
                ));
            }
        }
    }
    Ok(format!("({})", ors.join(" OR ")))
}

/// Is this column declared `ColTy::Bool`? A bare boolean token
/// (`active=true`) must bind `"1"`/`"0"`, not the literal words
/// `"true"`/`"false"` — SQLite's affinity rule only promotes a bound TEXT
/// value to NUMERIC when it *looks like* a number, so `active = ?` bound to
/// `"true"` compares the TEXT value `'true'` against the column's INTEGER
/// storage class and never matches, silently returning zero rows. Found
/// live: `active=true`, FHIR's own boolean-token spelling, matched nothing.
fn col_is_bool(cols: &[fhir_sqlite_map::model::Column], name: &str) -> bool {
    cols.iter()
        .any(|c| c.name == name && matches!(c.ty, fhir_sqlite_map::model::ColTy::Bool))
}

fn bool_token_as_bind(v: &str) -> &str {
    match v {
        "true" => "1",
        "false" => "0",
        other => other,
    }
}

/// One predicate for one target and one value. Column references use the
/// placeholder «c» for the table alias.
fn target_pred(
    def: &SearchDef,
    t: &fhir_sqlite_map::model::SearchTarget,
    value: &str,
    modifier: Option<&str>,
    cols: &[fhir_sqlite_map::model::Column],
    binds: &mut Vec<String>,
) -> Result<String, StoreError> {
    let b = |binds: &mut Vec<String>, v: &str| {
        binds.push(v.to_string());
        format!("?{}", binds.len())
    };
    // Only string and reference targets give a modifier a meaning here. On
    // every other kind the modifier used to be dropped silently, so
    // `status:not=x` ran as `status=x` and returned the opposite set
    // (clinical-safety audit 2026-11-25). Refuse it instead; `:not` can be
    // implemented later as a whole-resource complement.
    if let Some(m) = modifier
        && !matches!(t.kind, TargetKind::Str { .. } | TargetKind::Reference { .. })
    {
        return Err(StoreError::Unsupported(format!(
            "unsupported modifier :{m}"
        )));
    }
    match &t.kind {
        TargetKind::Str { col, norm } => {
            let c = format!("«c».\"{col}\"");
            // `:exact` is defined as the literal string, so it compares the
            // stored value. Everything else is case- and accent-insensitive
            // and compares the folded companion column, folding the search
            // term with the same function that produced it (P6.6). Maps
            // generated before folding existed have no companion column; fall
            // back to bare `LIKE`, which on SQLite is case-insensitive for ASCII
            // and nothing else — so "Muller" will not find "Müller". That is a
            // real narrowing, and it is why regenerating the map is the fix
            // rather than something to paper over here.
            let Some(n) = norm else {
                return match modifier {
                    Some("exact") => Ok(format!("{c} = {}", b(binds, value))),
                    Some("contains") => Ok(format!(
                        "{c} LIKE {}",
                        b(binds, &format!("%{}%", like_escape(value)))
                    )),
                    None | Some("text") => Ok(format!(
                        "{c} LIKE {}",
                        b(binds, &format!("{}%", like_escape(value)))
                    )),
                    Some(m) => Err(StoreError::Unsupported(format!(
                        "unsupported modifier :{m}"
                    ))),
                };
            };
            let nc = format!("«c».\"{n}\"");
            let folded = fold(value);
            match modifier {
                Some("exact") => Ok(format!("{c} = {}", b(binds, value))),
                Some("contains") => Ok(format!(
                    "{nc} LIKE {} ESCAPE '\\'",
                    b(binds, &format!("%{}%", like_escape(&folded)))
                )),
                None | Some("text") => {
                    // A range scan rather than `LIKE $1 || '%'`: the planner
                    // only extracts a prefix from a *constant* pattern, so a
                    // bound parameter under a generic plan would degrade to a
                    // sequential scan. `>=`/`<` on a COLLATE "C" column always
                    // uses the btree index.
                    let lo = b(binds, &folded);
                    match prefix_upper(&folded) {
                        Some(hi) => Ok(format!("({nc} >= {lo} AND {nc} < {})", b(binds, &hi))),
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
            let code_c = format!("«c».\"{code}\"");
            let code_is_bool = col_is_bool(cols, code);
            match value.split_once('|') {
                None => {
                    let v = if code_is_bool {
                        bool_token_as_bind(value)
                    } else {
                        value
                    };
                    Ok(format!("{code_c} = {}", b(binds, v)))
                }
                Some((sys, cv)) => {
                    let Some(sys_col) = system else {
                        // Bare-code element cannot match a system-qualified
                        // token unless the system part is empty.
                        return if sys.is_empty() {
                            Ok(format!("{code_c} = {}", b(binds, cv)))
                        } else {
                            Ok("FALSE".to_string())
                        };
                    };
                    let sys_c = format!("«c».\"{sys_col}\"");
                    if sys.is_empty() {
                        Ok(format!("({sys_c} IS NULL AND {code_c} = {})", b(binds, cv)))
                    } else if cv.is_empty() {
                        Ok(format!("{sys_c} = {}", b(binds, sys)))
                    } else {
                        Ok(format!(
                            "({sys_c} = {} AND {code_c} = {})",
                            b(binds, sys),
                            b(binds, cv)
                        ))
                    }
                }
            }
        }
        TargetKind::Date { lo, hi } => {
            let lo_c = format!("«c».\"{lo}\"");
            let hi_c = hi.as_ref().map(|h| format!("«c».\"{h}\""));
            date_pred(&lo_c, hi_c.as_deref(), value, binds)
        }
        TargetKind::Number { col } => {
            let (op, num) = num_prefix(value)?;
            Ok(numeric_pred(
                &format!("CAST(«c».\"{col}\" AS REAL)"),
                op,
                num,
                binds,
                |n| format!("CAST(?{n} AS REAL)"),
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
            let (op, num) = num_prefix(num)?;
            let mut pred = numeric_pred(
                &format!("CAST(«c».\"{vcol}\" AS REAL)"),
                op,
                num,
                binds,
                |n| format!("CAST(?{n} AS REAL)"),
            );
            if let (Some(sys), Some(sc)) = (sys.filter(|s| !s.is_empty()), system) {
                pred = format!("({pred} AND «c».\"{sc}\" = {})", b(binds, sys));
            }
            if let Some(u) = unit.filter(|s| !s.is_empty())
                && let Some(cc) = code
            {
                pred = format!("({pred} AND «c».\"{cc}\" = {})", b(binds, u));
            }
            Ok(pred)
        }
        TargetKind::Reference {
            c_type,
            c_id,
            c_url,
        } => {
            let ty_c = format!("«c».\"{c_type}\"");
            let id_c = format!("«c».\"{c_id}\"");
            let url_c = format!("«c».\"{c_url}\"");
            if value.contains("://") || value.starts_with("urn:") || value.starts_with('#') {
                Ok(format!("{url_c} = {}", b(binds, value)))
            } else if let Some((ty, id)) = value.split_once('/') {
                Ok(format!(
                    "({ty_c} = {} AND {id_c} = {})",
                    b(binds, ty),
                    b(binds, id)
                ))
            } else {
                let tymod = modifier.filter(|m| m.chars().next().is_some_and(char::is_uppercase));
                match tymod {
                    Some(ty) => Ok(format!(
                        "({ty_c} = {} AND {id_c} = {})",
                        b(binds, ty),
                        b(binds, value)
                    )),
                    None => Ok(format!("{id_c} = {}", b(binds, value))),
                }
            }
        }
        TargetKind::Uri { col } => {
            let _ = def;
            Ok(format!("«c».\"{col}\" = {}", b(binds, value)))
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
    // Bind lazily: a parameter PostgreSQL never sees referenced is an error,
    // so each comparison binds only the bound(s) it actually uses.
    let bind = |binds: &mut Vec<String>, v: &str| {
        binds.push(v.to_string());
        format!("?{}", binds.len())
    };
    let lo_val = lo;
    let hi_of = |binds: &mut Vec<String>| match &hi {
        // Full timestamps: exclusive upper bound one second later, computed
        // in SQL to sidestep calendar arithmetic.
        None => {
            let lo_b = bind(binds, &lo_val);
            format!("datetime({lo_b}, '+1 second')")
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
                        "({lo_col} < {hi_b} AND coalesce({e}, '9999-12-31T23:59:59.999999Z') >= {lo_b})"
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
                Some(e) => format!("coalesce({e}, '9999-12-31T23:59:59.999999Z') >= {hi_b}"),
                None => format!("{lo_col} >= {hi_b}"),
            }
        }
        "ge" => {
            let lo_b = bind(binds, &lo_val);
            match &end {
                Some(e) => format!("coalesce({e}, '9999-12-31T23:59:59.999999Z') >= {lo_b}"),
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

/// Split a number search value into its comparison operator and the number.
///
/// A number that is not a finite decimal is refused by name (RCA-IN-2): the
/// database would otherwise cast it to 0 (or fail obscurely) and answer a
/// different question than the one asked.
fn num_prefix(v: &str) -> Result<(&'static str, &str), StoreError> {
    let (op, num) = match v.get(..2) {
        Some("eq") => ("=", &v[2..]),
        Some("ne") => ("<>", &v[2..]),
        Some("lt") => ("<", &v[2..]),
        Some("gt") => (">", &v[2..]),
        Some("ge") => (">=", &v[2..]),
        Some("le") => ("<=", &v[2..]),
        _ => ("=", v),
    };
    match num.parse::<f64>() {
        Ok(n) if n.is_finite() => Ok((op, num)),
        _ => Err(StoreError::Unsupported(format!(
            "invalid number value {num:?}"
        ))),
    }
}

/// FHIR search precision: a number written `5.0` stands for the interval
/// [4.95, 5.05), so an `eq` query matches 5.04 and an `ne` query excludes it
/// (clinical-safety audit 2026-11-25). Returns the decimal bounds as strings,
/// computed with integer arithmetic so no float rounding is involved; `None`
/// for exponent notation or values too long to hold, which keep exact
/// comparison.
fn precision_bounds(num: &str) -> Option<(String, String)> {
    let (neg, body) = match num.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, num.strip_prefix('+').unwrap_or(num)),
    };
    let (int, frac) = body.split_once('.').unwrap_or((body, ""));
    let digits_only = |s: &str| s.bytes().all(|c| c.is_ascii_digit());
    if (int.is_empty() && frac.is_empty()) || !digits_only(int) || !digits_only(frac) {
        return None;
    }
    let digits = format!("{int}{frac}");
    if digits.len() > 30 {
        return None;
    }
    let n: i128 = digits.parse().ok()?;
    let n = if neg { -n } else { n };
    let scale = frac.len() + 1;
    let fmt = |m: i128| {
        let sign = if m < 0 { "-" } else { "" };
        let abs = format!("{:0>w$}", m.unsigned_abs(), w = scale + 1);
        let (i, f) = abs.split_at(abs.len() - scale);
        format!("{sign}{i}.{f}")
    };
    Some((fmt((2 * n - 1) * 5), fmt((2 * n + 1) * 5)))
}

/// `lhs op num`, with `eq`/`ne` widened to the precision interval. `rhs`
/// renders a bound placeholder in the dialect's numeric cast.
fn numeric_pred(
    lhs: &str,
    op: &str,
    num: &str,
    binds: &mut Vec<String>,
    rhs: impl Fn(usize) -> String,
) -> String {
    let mut bind = |v: &str| {
        binds.push(v.to_string());
        rhs(binds.len())
    };
    match (op, precision_bounds(num)) {
        ("=", Some((lo, hi))) => {
            let (l, h) = (bind(&lo), bind(&hi));
            format!("({lhs} >= {l} AND {lhs} < {h})")
        }
        ("<>", Some((lo, hi))) => {
            let (l, h) = (bind(&lo), bind(&hi));
            format!("({lhs} < {l} OR {lhs} >= {h})")
        }
        _ => format!("{lhs} {op} {}", bind(num)),
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
            if b.get(4) != Some(&b'-') {
                return None;
            }
            let m: i64 = v.get(5..7)?.parse().ok()?;
            if !(1..=12).contains(&m) {
                return None;
            }
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
            if b.get(4) != Some(&b'-') || b.get(7) != Some(&b'-') {
                return None;
            }
            let m: i64 = v.get(5..7)?.parse().ok()?;
            let d: i64 = v.get(8..10)?.parse().ok()?;
            let last = days_in(year, m);
            if last == 0 || !(1..=last).contains(&d) {
                return None;
            }
            let (ny, nm, nd) = if d < last {
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

#[cfg(test)]
mod tests {
    use fhir_sqlite_map::model::{ColTy, Column, SearchTarget, SearchTy};

    use super::*;

    fn def(code: &str, ty: SearchTy) -> SearchDef {
        SearchDef {
            code: code.to_string(),
            ty,
            targets: Vec::new(),
            note: None,
        }
    }

    fn col(name: &str, ty: ColTy) -> Column {
        Column {
            name: name.to_string(),
            ty,
            path: String::new(),
        }
    }

    fn pred(
        kind: TargetKind,
        value: &str,
        modifier: Option<&str>,
        cols: &[Column],
    ) -> Result<(String, Vec<String>), StoreError> {
        let target = SearchTarget { table: 0, kind };
        let mut binds = Vec::new();
        let sql = target_pred(
            &def("test", SearchTy::Special),
            &target,
            value,
            modifier,
            cols,
            &mut binds,
        )?;
        Ok((sql, binds))
    }

    #[test]
    fn date_bounds_cover_every_fhir_precision_and_calendar_edge() {
        assert_eq!(
            date_bounds("2024"),
            Some((
                "2024-01-01T00:00:00Z".into(),
                Some("2025-01-01T00:00:00Z".into())
            ))
        );
        assert_eq!(
            date_bounds("2024-12"),
            Some((
                "2024-12-01T00:00:00Z".into(),
                Some("2025-01-01T00:00:00Z".into())
            ))
        );
        assert_eq!(
            date_bounds("2024-02-28"),
            Some((
                "2024-02-28T00:00:00Z".into(),
                Some("2024-02-29T00:00:00Z".into())
            ))
        );
        assert_eq!(
            date_bounds("2023-02-28"),
            Some((
                "2023-02-28T00:00:00Z".into(),
                Some("2023-03-01T00:00:00Z".into())
            ))
        );
        assert_eq!(
            date_bounds("2024-12-31"),
            Some((
                "2024-12-31T00:00:00Z".into(),
                Some("2025-01-01T00:00:00Z".into())
            ))
        );
        assert_eq!(
            date_bounds("2024-01-02T03:04:05Z"),
            Some(("2024-01-02T03:04:05Z".into(), None))
        );
        assert_eq!(date_bounds("bad"), None);
        assert_eq!(date_bounds("2024-01-02 03:04:05Z"), None);

        assert_eq!(date_prefix("ge2024"), ("ge", "2024"));
        assert_eq!(date_prefix("2024"), ("eq", "2024"));
        assert_eq!(num_prefix("ne2.5").unwrap(), ("<>", "2.5"));
        // RCA-TEST-8: a malformed or non-finite number is refused by name, not
        // cast to 0 by the database.
        for bad in ["ap2.5", "abc", "", "eq", "inf", "NaN", "1e999", "2.5x"] {
            let err = num_prefix(bad).expect_err(bad);
            assert!(err.to_string().starts_with("invalid number value"), "{err}");
        }
    }

    #[test]
    fn date_predicates_bind_the_correct_interval_edges() {
        let cases = [
            (
                "2024",
                "(d >= ?1 AND d < ?2)",
                vec!["2024-01-01T00:00:00Z", "2025-01-01T00:00:00Z"],
            ),
            (
                "ne2024",
                "NOT (d >= ?1 AND d < ?2)",
                vec!["2024-01-01T00:00:00Z", "2025-01-01T00:00:00Z"],
            ),
            ("lt2024", "d < ?1", vec!["2024-01-01T00:00:00Z"]),
            ("eb2024", "d < ?1", vec!["2024-01-01T00:00:00Z"]),
            ("gt2024", "d >= ?1", vec!["2025-01-01T00:00:00Z"]),
            ("sa2024", "d >= ?1", vec!["2025-01-01T00:00:00Z"]),
            ("ge2024", "d >= ?1", vec!["2024-01-01T00:00:00Z"]),
            ("le2024", "d < ?1", vec!["2025-01-01T00:00:00Z"]),
        ];
        for (value, expected, expected_binds) in cases {
            let mut binds = Vec::new();
            assert_eq!(date_pred("d", None, value, &mut binds).unwrap(), expected);
            assert_eq!(binds, expected_binds);
        }

        let mut binds = Vec::new();
        assert_eq!(
            date_pred("start", Some("finish"), "2024", &mut binds).unwrap(),
            "(start < ?1 AND coalesce(finish, '9999-12-31T23:59:59.999999Z') >= ?2)"
        );
        assert_eq!(binds, ["2025-01-01T00:00:00Z", "2024-01-01T00:00:00Z"]);
        let mut binds = Vec::new();
        assert!(
            date_pred("d", None, "ap2024", &mut binds)
                .unwrap_err()
                .to_string()
                .contains("unsupported date prefix")
        );
        assert!(
            date_pred("d", None, "nonsense", &mut Vec::new())
                .unwrap_err()
                .to_string()
                .contains("invalid date value")
        );
    }

    #[test]
    fn eq_uses_the_precision_of_the_query() {
        assert_eq!(precision_bounds("5.0"), Some(("4.95".into(), "5.05".into())));
        assert_eq!(precision_bounds("5"), Some(("4.5".into(), "5.5".into())));
        assert_eq!(precision_bounds("5.00"), Some(("4.995".into(), "5.005".into())));
        assert_eq!(precision_bounds("-0.5"), Some(("-0.55".into(), "-0.45".into())));
        assert_eq!(precision_bounds("0"), Some(("-0.5".into(), "0.5".into())));
        assert_eq!(precision_bounds("1e3"), None);
    }

    #[test]
    fn modifiers_are_refused_where_they_would_be_ignored() {
        // `status:not=x` must not run as `status=x` (clinical-safety audit).
        let token = TargetKind::Token {
            system: None,
            code: "code".into(),
        };
        for m in ["not", "text", "missing"] {
            let err = pred(token.clone(), "final", Some(m), &[]).unwrap_err();
            assert!(err.to_string().contains("unsupported modifier"), "{m}");
        }
        assert!(pred(token, "final", None, &[]).is_ok());
    }

    #[test]
    fn string_token_number_and_uri_targets_preserve_fhir_semantics() {
        let string = TargetKind::Str {
            col: "value".into(),
            norm: Some("value_norm".into()),
        };
        assert_eq!(
            pred(string.clone(), "Müller", None, &[]).unwrap(),
            (
                "(«c».\"value_norm\" >= ?1 AND «c».\"value_norm\" < ?2)".into(),
                vec!["muller".into(), "mulles".into()]
            )
        );
        assert_eq!(
            pred(string.clone(), "50%_\\", Some("contains"), &[]).unwrap(),
            (
                "«c».\"value_norm\" LIKE ?1 ESCAPE '\\'".into(),
                vec!["%50\\%\\_\\\\%".into()]
            )
        );
        assert_eq!(
            pred(string.clone(), "Case", Some("exact"), &[]).unwrap(),
            ("«c».\"value\" = ?1".into(), vec!["Case".into()])
        );
        assert!(pred(string, "x", Some("missing"), &[]).is_err());

        let legacy = TargetKind::Str {
            col: "value".into(),
            norm: None,
        };
        assert_eq!(
            pred(legacy.clone(), "x", Some("text"), &[]).unwrap().1,
            ["x%"]
        );
        assert_eq!(
            pred(legacy.clone(), "x", Some("contains"), &[]).unwrap().1,
            ["%x%"]
        );
        assert_eq!(pred(legacy, "x", Some("exact"), &[]).unwrap().1, ["x"]);

        let token = TargetKind::Token {
            system: Some("system".into()),
            code: "code".into(),
        };
        assert_eq!(pred(token.clone(), "code", None, &[]).unwrap().1, ["code"]);
        assert_eq!(
            pred(token.clone(), "sys|code", None, &[]).unwrap().1,
            ["sys", "code"]
        );
        assert_eq!(pred(token.clone(), "|code", None, &[]).unwrap().1, ["code"]);
        assert_eq!(pred(token, "sys|", None, &[]).unwrap().1, ["sys"]);

        let bare = TargetKind::Token {
            system: None,
            code: "active".into(),
        };
        let cols = [col("active", ColTy::Bool)];
        assert_eq!(pred(bare.clone(), "true", None, &cols).unwrap().1, ["1"]);
        assert_eq!(pred(bare.clone(), "false", None, &cols).unwrap().1, ["0"]);
        assert_eq!(pred(bare.clone(), "|yes", None, &cols).unwrap().1, ["yes"]);
        assert_eq!(pred(bare, "sys|yes", None, &cols).unwrap().0, "FALSE");

        for (value, op, number) in [
            ("lt2", "<", "2"),
            ("gt2", ">", "2"),
            ("ge2", ">=", "2"),
            ("le2", "<=", "2"),
        ] {
            let (sql, binds) =
                pred(TargetKind::Number { col: "n".into() }, value, None, &[]).unwrap();
            assert!(sql.contains(&format!(" AS REAL) {op} CAST(")));
            assert_eq!(binds, [number]);
        }
        // `eq` widens to the precision interval: 2 means [1.5, 2.5).
        let (sql, binds) = pred(TargetKind::Number { col: "n".into() }, "2", None, &[]).unwrap();
        assert!(sql.contains(" >= CAST(?1 AS REAL)") && sql.contains(" < CAST(?2 AS REAL)"));
        assert_eq!(binds, ["1.5", "2.5"]);
        assert_eq!(
            pred(TargetKind::Uri { col: "url".into() }, "urn:test", None, &[]).unwrap(),
            ("«c».\"url\" = ?1".into(), vec!["urn:test".into()])
        );
        assert_eq!(
            pred(
                TargetKind::Date {
                    lo: "start".into(),
                    hi: Some("finish".into()),
                },
                "ge2024",
                None,
                &[],
            )
            .unwrap()
            .1,
            ["2024-01-01T00:00:00Z"]
        );
    }

    #[test]
    fn quantity_and_reference_targets_cover_qualified_and_absolute_forms() {
        let quantity = TargetKind::Quantity {
            value: "value".into(),
            system: Some("system".into()),
            code: Some("code".into()),
        };
        assert_eq!(
            pred(quantity.clone(), "ge5|UCUM|mg", None, &[]).unwrap().1,
            ["5", "UCUM", "mg"]
        );
        assert_eq!(pred(quantity, "5||mg", None, &[]).unwrap().1, ["4.5", "5.5", "mg"]);
        let (sql, binds) = pred(
            TargetKind::Quantity {
                value: "value".into(),
                system: None,
                code: Some("code".into()),
            },
            "5||mg",
            None,
            &[],
        )
        .unwrap();
        assert!(sql.contains("\"code\" = ?3"));
        assert_eq!(binds, ["4.5", "5.5", "mg"]);
        assert_eq!(
            pred(
                TargetKind::Quantity {
                    value: "value".into(),
                    system: None,
                    code: None,
                },
                "5||mg",
                None,
                &[],
            )
            .unwrap()
            .1,
            ["4.5", "5.5"]
        );
        assert_eq!(
            pred(
                TargetKind::Quantity {
                    value: "value".into(),
                    system: None,
                    code: None,
                },
                "5",
                None,
                &[],
            )
            .unwrap()
            .1,
            ["4.5", "5.5"]
        );

        let reference = TargetKind::Reference {
            c_type: "ref_type".into(),
            c_id: "ref_id".into(),
            c_url: "ref_url".into(),
        };
        assert_eq!(
            pred(reference.clone(), "Patient/7", None, &[]).unwrap().1,
            ["Patient", "7"]
        );
        assert_eq!(
            pred(reference.clone(), "7", Some("Patient"), &[])
                .unwrap()
                .1,
            ["Patient", "7"]
        );
        assert_eq!(pred(reference.clone(), "7", None, &[]).unwrap().1, ["7"]);
        assert_eq!(
            pred(reference.clone(), "https://example.test/p/7", None, &[])
                .unwrap()
                .1,
            ["https://example.test/p/7"]
        );
        assert_eq!(
            pred(reference.clone(), "urn:uuid:7", None, &[]).unwrap().1,
            ["urn:uuid:7"]
        );
        assert_eq!(
            pred(reference, "#contained", None, &[]).unwrap().1,
            ["#contained"]
        );
    }

    #[test]
    fn compiled_queries_cover_filters_sort_cursor_and_reference_chains() {
        let map = RelMap::bundled("r5").expect("bundled R5 map");
        let patient = &map.resources["Patient"];
        let query = build_search_sql(
            &map,
            patient,
            &[
                ("_id".into(), "a,b".into()),
                ("_lastUpdated".into(), "ge2024".into()),
                ("active".into(), "true".into()),
                ("family:contains".into(), "Müller".into()),
            ],
            25,
            10,
            &[SortKey {
                code: "_lastUpdated".into(),
                descending: true,
            }],
            Some("cursor"),
        )
        .unwrap();
        assert!(query.sql.contains("p.\"id\" = ?1 OR p.\"id\" = ?2"));
        assert!(query.sql.contains("p.\"id\" > ?"));
        assert!(
            query
                .sql
                .contains("ORDER BY p.\"last_updated\" DESC NULLS LAST, p.\"id\" ASC")
        );
        assert!(!query.count_sql.contains("cursor"));
        assert_eq!(query.count_binds + 3, query.binds.len());
        assert_eq!(
            &query.binds[query.binds.len() - 3..],
            ["cursor", "25", "10"]
        );

        let observation = &map.resources["Observation"];
        let chained = build_search_sql(
            &map,
            observation,
            &[("subject:Patient.family".into(), "Smith".into())],
            10,
            0,
            &[],
            None,
        )
        .unwrap();
        assert!(chained.sql.contains("subject_ref_type"));
        assert!(chained.sql.contains("patient"));
        assert_eq!(chained.binds.first().unwrap(), "Patient");
    }

    #[test]
    fn unsupported_queries_fail_instead_of_silently_broadening_results() {
        let map = RelMap::bundled("r5").expect("bundled R5 map");
        let patient = &map.resources["Patient"];
        for params in [
            vec![("unknown".into(), "x".into())],
            vec![("deceased".into(), "true".into())],
            vec![("general-practitioner.name".into(), "x".into())],
            vec![("general-practitioner:Unknown.name".into(), "x".into())],
            vec![("family:Patient.name".into(), "x".into())],
            vec![(
                "general-practitioner:Practitioner.organization:Organization.name".into(),
                "x".into(),
            )],
        ] {
            assert!(
                build_search_sql(&map, patient, &params, 10, 0, &[], None).is_err(),
                "{params:?}"
            );
        }

        assert!(
            build_search_sql(
                &map,
                patient,
                &[],
                10,
                0,
                &[SortKey {
                    code: "unknown".into(),
                    descending: false
                }],
                None,
            )
            .is_err()
        );
        assert!(
            build_search_sql(
                &map,
                patient,
                &[],
                10,
                0,
                &[SortKey {
                    code: "family".into(),
                    descending: false
                }],
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn remaining_sort_chain_and_predicate_edges_are_explicit() {
        let map = RelMap::bundled("r5").expect("bundled R5 map");
        let patient = &map.resources["Patient"];

        assert_eq!(
            sort_col(
                patient,
                &SortKey {
                    code: "_id".into(),
                    descending: false,
                },
            )
            .unwrap(),
            "p.\"id\""
        );

        let mut sortable = patient.clone();
        for (code, kind, expected) in [
            (
                "quantity-sort",
                TargetKind::Quantity {
                    value: "quantity_value".into(),
                    system: None,
                    code: None,
                },
                "p.\"quantity_value\"",
            ),
            (
                "reference-sort",
                TargetKind::Reference {
                    c_type: "reference_type".into(),
                    c_id: "reference_id".into(),
                    c_url: "reference_url".into(),
                },
                "p.\"reference_id\"",
            ),
        ] {
            sortable.search.push(SearchDef {
                code: code.into(),
                ty: SearchTy::Special,
                targets: vec![SearchTarget { table: 0, kind }],
                note: None,
            });
            assert_eq!(
                sort_col(
                    &sortable,
                    &SortKey {
                        code: code.into(),
                        descending: false,
                    },
                )
                .unwrap(),
                expected
            );
        }

        let cursor_only = build_search_sql(&map, patient, &[], 1, 0, &[], Some("p0")).unwrap();
        assert!(cursor_only.sql.contains(" WHERE p.\"id\" > ?1"));

        let practitioner_chain = build_search_sql(
            &map,
            patient,
            &[(
                "general-practitioner:Practitioner.name".into(),
                "Smith".into(),
            )],
            10,
            0,
            &[],
            None,
        )
        .unwrap();
        assert!(practitioner_chain.sql.contains("EXISTS (SELECT 1"));

        assert!(
            build_search_sql(
                &map,
                patient,
                &[("missing:Patient.name".into(), "x".into())],
                10,
                0,
                &[],
                None,
            )
            .is_err()
        );

        let legacy = TargetKind::Str {
            col: "value".into(),
            norm: None,
        };
        assert!(pred(legacy, "x", Some("unsupported"), &[]).is_err());
        let folded = TargetKind::Str {
            col: "value".into(),
            norm: Some("value_norm".into()),
        };
        assert_eq!(pred(folded, "", None, &[]).unwrap().1, [""]);
        assert_eq!(bool_token_as_bind("other"), "other");
        assert_eq!(num_prefix("eq2.5").unwrap(), ("=", "2.5"));
    }

    #[test]
    fn remaining_date_edges_cover_periods_seconds_and_month_shapes() {
        for (value, expected) in [
            (
                "gt2024",
                "coalesce(finish, '9999-12-31T23:59:59.999999Z') >= ?1",
            ),
            (
                "ge2024",
                "coalesce(finish, '9999-12-31T23:59:59.999999Z') >= ?1",
            ),
        ] {
            let mut binds = Vec::new();
            assert_eq!(
                date_pred("start", Some("finish"), value, &mut binds).unwrap(),
                expected
            );
        }

        let mut binds = Vec::new();
        assert_eq!(
            date_pred("instant", None, "2024-01-02T03:04:05Z", &mut binds).unwrap(),
            "(instant >= ?1 AND instant < datetime(?2, '+1 second'))"
        );
        assert_eq!(binds, ["2024-01-02T03:04:05Z", "2024-01-02T03:04:05Z"]);

        assert_eq!(
            date_bounds("2024-04"),
            Some((
                "2024-04-01T00:00:00Z".into(),
                Some("2024-05-01T00:00:00Z".into())
            ))
        );
        assert_eq!(
            date_bounds("2024-04-30"),
            Some((
                "2024-04-30T00:00:00Z".into(),
                Some("2024-05-01T00:00:00Z".into())
            ))
        );
        assert_eq!(date_bounds("2024-13-01"), None);
        assert_eq!(date_bounds("2024-04-31"), None);
        assert_eq!(date_bounds("2024/04"), None);
        assert_eq!(date_bounds("2024-13"), None);
        assert_eq!(date_bounds("2024/04/30"), None);
    }
}
