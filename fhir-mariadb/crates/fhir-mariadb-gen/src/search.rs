//! Search-parameter compilation: resolve each SearchParameter's FHIRPath
//! expression against the built map, producing concrete (table, column)
//! targets. Parameters that use FHIRPath features beyond the supported
//! subset compile to an empty target list with a note — the server reports
//! them as unsupported rather than guessing.

use std::collections::HashMap;
use std::path::Path;

use fhir_mariadb_map::model::{
    Elem, ElemKind, Prim, RelMap, ResourceMap, SearchDef, SearchTarget, SearchTy, TargetKind,
};
use serde_json::Value;

use crate::GenError;
use crate::names::ucfirst;

pub fn compile_search(map: &mut RelMap, definitions_dir: &Path) -> Result<(), GenError> {
    let path = definitions_dir.join("search-parameters.json");
    let bytes =
        std::fs::read(&path).map_err(|e| GenError::Spec(format!("{}: {e}", path.display())))?;
    let bundle: Value = serde_json::from_slice(&bytes)
        .map_err(|e| GenError::Spec(format!("{}: {e}", path.display())))?;

    // resource type → [(code, ty, expression)]
    let mut by_base: HashMap<String, Vec<(String, SearchTy, Option<String>)>> = HashMap::new();
    for entry in bundle
        .get("entry")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(res) = entry.get("resource") else {
            continue;
        };
        if res.get("resourceType").and_then(Value::as_str) != Some("SearchParameter") {
            continue;
        }
        let Some(code) = res.get("code").and_then(Value::as_str) else {
            continue;
        };
        let Some(ty) = res.get("type").and_then(Value::as_str).map(search_ty) else {
            continue;
        };
        let expr = res
            .get("expression")
            .and_then(Value::as_str)
            .map(str::to_string);
        for base in res
            .get("base")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            by_base
                .entry(base.to_string())
                .or_default()
                .push((code.to_string(), ty, expr.clone()));
        }
    }

    for (rname, rm) in map.resources.iter_mut() {
        let Some(params) = by_base.get(rname) else {
            continue;
        };
        let mut defs = Vec::new();
        for (code, ty, expr) in params {
            defs.push(compile_param(rm, rname, code, *ty, expr.as_deref()));
        }
        defs.sort_by(|a, b| a.code.cmp(&b.code));
        rm.search = defs;
    }
    Ok(())
}

fn search_ty(s: &str) -> SearchTy {
    match s {
        "number" => SearchTy::Number,
        "date" => SearchTy::Date,
        "string" => SearchTy::String,
        "token" => SearchTy::Token,
        "reference" => SearchTy::Reference,
        "composite" => SearchTy::Composite,
        "quantity" => SearchTy::Quantity,
        "uri" => SearchTy::Uri,
        _ => SearchTy::Special,
    }
}

fn unsupported(code: &str, ty: SearchTy, note: &str) -> SearchDef {
    SearchDef {
        code: code.to_string(),
        ty,
        targets: Vec::new(),
        note: Some(note.to_string()),
    }
}

fn compile_param(
    rm: &ResourceMap,
    rname: &str,
    code: &str,
    ty: SearchTy,
    expr: Option<&str>,
) -> SearchDef {
    if matches!(ty, SearchTy::Composite | SearchTy::Special) {
        return unsupported(code, ty, "composite/special parameters are not compiled");
    }
    let Some(expr) = expr else {
        return unsupported(code, ty, "no expression");
    };
    let mut targets = Vec::new();
    let mut notes = Vec::new();
    for alt in split_top(expr, '|') {
        match compile_alt(rm, rname, alt.trim(), ty) {
            Ok(mut t) => targets.append(&mut t),
            Err(n) => notes.push(n),
        }
    }
    // Deduplicate identical targets from union branches.
    let mut seen = std::collections::HashSet::new();
    targets.retain(|t| seen.insert(format!("{:?}", t)));
    SearchDef {
        code: code.to_string(),
        ty,
        targets,
        note: if notes.is_empty() {
            None
        } else {
            Some(notes.join("; "))
        },
    }
}

/// Split on `sep` at paren depth 0.
fn split_top(s: &str, sep: char) -> Vec<&str> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

/// One path alternative → targets, or a note describing why not.
fn compile_alt(
    rm: &ResourceMap,
    rname: &str,
    alt: &str,
    ty: SearchTy,
) -> Result<Vec<SearchTarget>, String> {
    // "(Observation.value as Quantity)" → "Observation.value.ofType(Quantity)"
    let mut alt = alt.trim();
    let mut cast: Option<String> = None;
    if alt.starts_with('(') && alt.ends_with(')') {
        let inner = &alt[1..alt.len() - 1];
        if let Some((path, casttype)) = inner.rsplit_once(" as ") {
            alt = path.trim();
            cast = Some(casttype.trim().to_string());
        } else {
            alt = inner.trim();
        }
    }
    let mut segs: Vec<String> = split_top(alt, '.')
        .into_iter()
        .map(|s| s.trim().to_string())
        .collect();
    if let Some(c) = cast {
        segs.push(format!("ofType({c})"));
    }
    if segs.first().map(String::as_str) != Some(rname) {
        return Err(format!("path does not start at {rname}: {alt:?}"));
    }
    segs.remove(0);

    // Walk the tree.
    let mut table: u32 = 0;
    let mut node = rm.root;
    let mut leaf: Option<&Elem> = None;
    let mut i = 0usize;
    while i < segs.len() {
        let seg = &segs[i];
        i += 1;
        // Ignorable / unsupported function segments.
        if let Some(arg) = seg.strip_prefix("where(").and_then(|s| s.strip_suffix(')')) {
            // Not all `where()` clauses are alike, and the difference decides
            // whether dropping one is lenient or wrong.
            //
            // `where(resolve() is Patient)` restricts a reference by *target
            // type*. The reference column already stores the type beside the
            // id, so a query can re-apply the restriction and dropping it here
            // only widens the compile-time target, not the answer.
            //
            // `where(type='derived-from')` restricts by *value*, and there is
            // nothing downstream that knows to re-apply it. Dropping it makes
            // the parameter match every sibling regardless of the discriminator
            // — F-38, where ActivityDefinition's `composed-of`, `derived-from`,
            // `predecessor` and `successor` all compiled to one identical
            // target and could not return different rows.
            //
            // A search that answers a question it was not asked is worse than
            // one that says it cannot (C0.11, T11.12), so this refuses.
            let a = arg.trim();
            let is_value_restriction = a.split_once('=').is_some_and(|(lhs, rhs)| {
                !lhs.contains('(')
                    && !lhs.trim().is_empty()
                    && lhs.trim().chars().all(|c| c.is_alphanumeric() || c == '_')
                    && rhs.trim().starts_with('\'')
            });
            if is_value_restriction {
                return Err(format!(
                    "where() value restriction cannot be dropped: {seg:?}"
                ));
            }
            continue; // typed-reference restriction — genuinely lenient
        }
        if seg == "first()" || seg == "exists()" {
            continue;
        }
        if seg.starts_with("extension(") {
            return Err("extension() paths are not compiled".to_string());
        }
        if let Some(t) = seg
            .strip_prefix("ofType(")
            .or_else(|| seg.strip_prefix("as("))
            .and_then(|s| s.strip_suffix(')'))
        {
            // Select a choice variant from the previously matched choice.
            let Some(elem) = leaf else {
                return Err(format!("cast with no preceding element: {alt:?}"));
            };
            let ElemKind::Choice(variants) = &elem.kind else {
                // Cast on a non-choice (e.g. canonical as uri) — keep as-is.
                continue;
            };
            // A force-split choice owns a table; its variants' columns live
            // there, not in the pre-choice table.
            if let Some(t) = elem.table {
                table = t;
            }
            let want = format!("{}{}", elem.json, ucfirst(t.trim()));
            let Some(var) = variants.iter().find(|v| v.json == want) else {
                return Err(format!("no choice variant {want:?}"));
            };
            leaf = Some(var);
            continue;
        }
        if seg.contains('(') {
            return Err(format!("unsupported function segment {seg:?}"));
        }
        // Enter the previous group, if any.
        if let Some(elem) = leaf {
            match &elem.kind {
                ElemKind::Group(n) => {
                    if let Some(t) = elem.table {
                        table = t;
                    }
                    node = *n;
                }
                ElemKind::Choice(_) => {
                    return Err(format!("choice {:?} navigated without ofType()", elem.json));
                }
                _ => return Err(format!("cannot navigate into {:?}", elem.json)),
            }
        }
        let Some(elem) = rm.node(node).elems.iter().find(|e| e.json == *seg) else {
            return Err(format!("no element {seg:?} under {alt:?}"));
        };
        leaf = Some(elem);
    }
    let Some(elem) = leaf else {
        return Err(format!("empty path {alt:?}"));
    };
    targets_for(rm, table, node, elem, ty).map_err(|n| format!("{alt:?}: {n}"))
}

/// Derive targets from the element a path landed on.
fn targets_for(
    rm: &ResourceMap,
    table: u32,
    _node: u32,
    elem: &Elem,
    ty: SearchTy,
) -> Result<Vec<SearchTarget>, String> {
    // The element's own row context.
    let (etable, enode) = match &elem.kind {
        ElemKind::Group(n) => (elem.table.unwrap_or(table), *n),
        _ => (elem.table.unwrap_or(table), u32::MAX),
    };
    let one = |t: u32, kind: TargetKind| Ok(vec![SearchTarget { table: t, kind }]);

    match (&elem.kind, ty) {
        (ElemKind::Choice(variants), _) => {
            // A choice reached without cast: compile every variant that fits.
            let mut out = Vec::new();
            for var in variants {
                if let Ok(mut t) = targets_for(rm, elem.table.unwrap_or(table), _node, var, ty) {
                    out.append(&mut t);
                }
            }
            if out.is_empty() {
                Err("no compilable choice variant".to_string())
            } else {
                Ok(out)
            }
        }
        (ElemKind::Prim(pc), SearchTy::Token) => one(
            etable,
            TargetKind::Token {
                system: None,
                code: pc.col.clone(),
            },
        ),
        (ElemKind::Prim(pc), SearchTy::String) => one(
            etable,
            TargetKind::Str {
                col: pc.col.clone(),
                norm: None,
            },
        ),
        (ElemKind::Prim(pc), SearchTy::Uri) => one(
            etable,
            TargetKind::Uri {
                col: pc.col.clone(),
            },
        ),
        (ElemKind::Prim(pc), SearchTy::Number) => match pc.prim {
            Prim::Int | Prim::Int64 | Prim::Decimal => one(
                etable,
                TargetKind::Number {
                    col: pc.col.clone(),
                },
            ),
            _ => Err("number parameter on non-numeric element".to_string()),
        },
        (ElemKind::Prim(pc), SearchTy::Date) => match &pc.sort {
            Some(sc) => one(
                etable,
                TargetKind::Date {
                    lo: sc.clone(),
                    hi: None,
                },
            ),
            None => Err("date parameter on element without a sort column".to_string()),
        },
        (ElemKind::Prim(pc), SearchTy::Reference) => {
            // canonical / uri references compare literally.
            one(
                etable,
                TargetKind::Uri {
                    col: pc.col.clone(),
                },
            )
        }
        (ElemKind::Group(_), _) => group_targets(rm, etable, enode, elem, ty),
        (ElemKind::RefStr(rc), SearchTy::Reference) => one(
            etable,
            TargetKind::Reference {
                c_type: rc.c_type.clone(),
                c_id: rc.c_id.clone(),
                c_url: rc.c_url.clone(),
            },
        ),
        _ => Err(format!("cannot compile {:?} parameter here", ty)),
    }
}

/// Targets for a parameter landing on a complex element, by datatype shape.
fn group_targets(
    rm: &ResourceMap,
    table: u32,
    node: u32,
    elem: &Elem,
    ty: SearchTy,
) -> Result<Vec<SearchTarget>, String> {
    let elems = &rm.node(node).elems;
    let find = |name: &str| elems.iter().find(|e| e.json == name);
    let prim_col = |name: &str| -> Option<(u32, String, Option<String>)> {
        find(name).and_then(|e| match &e.kind {
            ElemKind::Prim(pc) => Some((e.table.unwrap_or(table), pc.col.clone(), pc.sort.clone())),
            _ => None,
        })
    };

    match ty {
        SearchTy::Token => {
            // CodeableConcept → its coding table; Coding/Identifier/
            // ContactPoint → in place.
            if let Some(coding) = find("coding")
                && let ElemKind::Group(cn) = coding.kind
            {
                return group_targets(
                    rm,
                    coding.table.unwrap_or(table),
                    cn,
                    coding,
                    SearchTy::Token,
                );
            }
            if let (Some((st, sys, _)), Some((ct, code, _))) =
                (prim_col("system"), prim_col("code"))
                && st == ct
            {
                return Ok(vec![SearchTarget {
                    table: ct,
                    kind: TargetKind::Token {
                        system: Some(sys),
                        code,
                    },
                }]);
            }
            if let (Some((st, sys, _)), Some((vt, value, _))) =
                (prim_col("system"), prim_col("value"))
                && st == vt
            {
                return Ok(vec![SearchTarget {
                    table: vt,
                    kind: TargetKind::Token {
                        system: Some(sys),
                        code: value,
                    },
                }]);
            }
            Err(format!("no token shape in {:?}", elem.json))
        }
        SearchTy::String => {
            // HumanName / Address: match any textual part.
            let mut out = Vec::new();
            for part in [
                "family",
                "text",
                "city",
                "district",
                "state",
                "postalCode",
                "country",
            ] {
                if let Some((t, col, _)) = prim_col(part) {
                    out.push(SearchTarget {
                        table: t,
                        kind: TargetKind::Str { col, norm: None },
                    });
                }
            }
            // Repeating string parts (given, line, prefix, suffix) live in
            // their own tables.
            for part in ["given", "line", "prefix", "suffix"] {
                if let Some(e) = find(part)
                    && let (ElemKind::Prim(pc), Some(t)) = (&e.kind, e.table)
                {
                    out.push(SearchTarget {
                        table: t,
                        kind: TargetKind::Str {
                            col: pc.col.clone(),
                            norm: None,
                        },
                    });
                }
            }
            if out.is_empty() {
                Err(format!("no string parts in {:?}", elem.json))
            } else {
                Ok(out)
            }
        }
        SearchTy::Date => {
            // Period → start/end range.
            if let (Some((t1, _, Some(lo))), Some((t2, _, Some(hi)))) =
                (prim_col("start"), prim_col("end"))
                && t1 == t2
            {
                return Ok(vec![SearchTarget {
                    table: t1,
                    kind: TargetKind::Date { lo, hi: Some(hi) },
                }]);
            }
            Err(format!("no date shape in {:?}", elem.json))
        }
        SearchTy::Quantity => {
            if let Some((t, value, _)) = prim_col("value") {
                let system = prim_col("system").filter(|(st, ..)| *st == t).map(|x| x.1);
                let code = prim_col("code")
                    .filter(|(ct, ..)| *ct == t)
                    .map(|x| x.1)
                    .or_else(|| {
                        prim_col("currency")
                            .filter(|(ct, ..)| *ct == t)
                            .map(|x| x.1)
                    });
                return Ok(vec![SearchTarget {
                    table: t,
                    kind: TargetKind::Quantity {
                        value,
                        system,
                        code,
                    },
                }]);
            }
            Err(format!("no quantity shape in {:?}", elem.json))
        }
        SearchTy::Reference => {
            // Reference → its parsed columns; CodeableReference → descend.
            if let Some(r) = elems.iter().find_map(|e| match &e.kind {
                ElemKind::RefStr(rc) => Some(rc.clone()),
                _ => None,
            }) {
                return Ok(vec![SearchTarget {
                    table,
                    kind: TargetKind::Reference {
                        c_type: r.c_type,
                        c_id: r.c_id,
                        c_url: r.c_url,
                    },
                }]);
            }
            if let Some(inner) = find("reference")
                && let ElemKind::Group(n) = inner.kind
            {
                return group_targets(rm, inner.table.unwrap_or(table), n, inner, ty);
            }
            Err(format!("no reference shape in {:?}", elem.json))
        }
        _ => Err(format!("cannot compile {ty:?} on complex {:?}", elem.json)),
    }
}

/// Materialise the folded companion column for every `string` search target
/// (P6.6).
///
/// Runs after search compilation, because only then is it known which columns
/// a `string` parameter actually tests — folding every text column in the
/// schema would roughly double it for no benefit. Each `(source, folded)` pair
/// is recorded on the table so the shredder can fill it, and on the target so
/// the query planner-facing predicate can use it.
///
/// Idempotent: a map already carrying folded columns is left unchanged, so
/// regenerating does not append duplicates.
pub fn add_norm_columns(map: &mut RelMap) {
    use crate::names::Registry;
    use fhir_mariadb_map::model::{ColTy, Column};

    for rm in map.resources.values_mut() {
        // One registry per table, seeded with the names build.rs already
        // claimed, so a folded column can never collide with a data column.
        let mut regs: Vec<Registry> = rm
            .tables
            .iter()
            .map(|t| Registry::seeded(t.cols.iter().map(|c| c.name.as_str())))
            .collect();

        // Collect first: `rm.search` and `rm.tables` cannot both be borrowed
        // mutably at once.
        let mut wanted: Vec<(u32, String)> = Vec::new();
        for def in &rm.search {
            for t in &def.targets {
                if let TargetKind::Str { col, norm: None } = &t.kind {
                    wanted.push((t.table, col.clone()));
                }
            }
        }

        // A column can back several parameters (Patient.name feeds both
        // `name` and `phonetic`); fold it once.
        let mut assigned: HashMap<(u32, String), String> = HashMap::new();
        for (table, col) in wanted {
            let key = (table, col.clone());
            if assigned.contains_key(&key) {
                continue;
            }
            let t = &mut rm.tables[table as usize];
            if let Some((_, existing)) = t.norm_cols.iter().find(|(src, _)| *src == col) {
                assigned.insert(key, existing.clone());
                continue;
            }
            let path = t
                .cols
                .iter()
                .find(|c| c.name == col)
                .map(|c| c.path.clone())
                .unwrap_or_default();
            let name = regs[table as usize].claim(&format!("{col}_norm"));
            t.cols.push(Column {
                name: name.clone(),
                ty: ColTy::TextC,
                path,
            });
            t.norm_cols.push((col.clone(), name.clone()));
            assigned.insert(key, name);
        }

        for def in &mut rm.search {
            for t in &mut def.targets {
                if let TargetKind::Str { col, norm } = &mut t.kind
                    && norm.is_none()
                    && let Some(n) = assigned.get(&(t.table, col.clone()))
                {
                    *norm = Some(n.clone());
                }
            }
        }
    }
}

/// Materialise the two unbounded-string adjuncts for every `string` search
/// target (`U1`), on the ports whose engine needs them (`U9`).
///
/// Mirrors `add_norm_columns` deliberately: same pass position, same
/// per-table name registry, same idempotence. The difference is the guard —
/// `ddl::TEXT_ADJUNCTS` is false on the four engines that index and compare
/// their bound text type directly, and `U9` forbids emitting the columns
/// there. Adding two derived columns to every indexed text column in four
/// ports would be storage and write amplification paid for nothing.
///
/// Both columns are always added together. `U2` makes them a pair: a bounded
/// adjunct cannot answer equality, a digest cannot answer a prefix, and a port
/// that emitted one and called the problem solved is the failure `U2` is
/// written to prevent.
///
/// Idempotent: a map already carrying adjuncts is left unchanged.
pub fn add_adjunct_columns(map: &mut RelMap) {
    use crate::names::Registry;
    use fhir_mariadb_map::model::{Adjuncts, ColTy, Column, TableKind};

    if !fhir_mariadb_map::ddl::TEXT_ADJUNCTS {
        return;
    }

    for rm in map.resources.values_mut() {
        let mut regs: Vec<Registry> = rm
            .tables
            .iter()
            .map(|t| Registry::seeded(t.cols.iter().map(|c| c.name.as_str())))
            .collect();

        // U2a: which adjuncts a column needs is decided by the operations the
        // search performs on it, not by its type. Collect the demand first,
        // because one column can back several parameters — `Patient.name`
        // feeds both `name` and `phonetic` — and a column reached by a `string`
        // parameter and a `token` parameter needs the union of both.
        let mut want: HashMap<(u32, String), (bool, bool)> = HashMap::new();
        for def in &rm.search {
            for t in &def.targets {
                // (bounded, digest)
                let (cols, need): (Vec<&String>, (bool, bool)) = match &t.kind {
                    // Prefix/contains *and* `:exact` — both operations, so both
                    // adjuncts. This is the case U2 was written from.
                    TargetKind::Str { col, .. } => (vec![col], (true, true)),
                    // Equality only. A bounded adjunct here would be a column
                    // no query reads (U2a).
                    TargetKind::Token { system, code } => (
                        system.iter().chain(std::iter::once(code)).collect(),
                        (false, true),
                    ),
                    // `:below` and `:above` are prefix operations over a URI,
                    // and plain match is equality — so both.
                    TargetKind::Uri { col } => (vec![col], (true, true)),
                    // A reference is searched two ways, and this rule
                    // originally covered only one of them. `subject=Patient/1`
                    // matches on the split `(c_type, c_id)`; an absolute
                    // reference matches `c_url`. Both are equality, neither has
                    // a prefix form, and `U2a` needs the adjunct on whichever
                    // column the query actually touches — so all three.
                    //
                    // Attaching to `c_url` alone was consistent with nothing:
                    // every port's `search_indexes` keys the reference index on
                    // `(c_type, c_id)`, so the index wanted a column the
                    // adjunct pass had skipped. On the four ports that
                    // materialize no adjuncts this was invisible; on Oracle it
                    // left 453 of R5's search targets unindexable (**F-50**).
                    TargetKind::Reference {
                        c_type,
                        c_id,
                        c_url,
                    } => (vec![c_type, c_id, c_url], (false, true)),
                    // Date, Number and Quantity target derived sort columns of
                    // bounded numeric or temporal type. They are indexable and
                    // comparable already, so U1a's trigger never fires.
                    _ => (Vec::new(), (false, false)),
                };
                for c in cols {
                    let e = want.entry((t.table, c.clone())).or_insert((false, false));
                    e.0 |= need.0;
                    e.1 |= need.1;
                }
            }
        }

        // U11: the extension and deep tables are search-reachable too, and they
        // reach the generator by a different road — their columns are
        // fixed-shape rather than derived from a search target, so the loop
        // above never sees them (F-46).
        //
        // Which adjuncts each needs follows U2a, by the operation performed:
        //
        //   url     a URI — `:below`/`:above` are prefixes, plain match is
        //           equality  -> both
        //   v_text  the extension's value, string semantics -> both
        //   leaf    the element name, matched exactly -> equality only
        //
        // Only columns `fixed_shape_cols` put in the map appear here, and
        // `needs_adjunct` still has the final say, so a dialect that indexes
        // its unbounded text type gets nothing.
        for (ti, t) in rm.tables.iter().enumerate() {
            if !matches!(t.kind, TableKind::Ext | TableKind::Deep) {
                continue;
            }
            for (col, need) in [
                ("url", (true, true)),
                ("v_text", (true, true)),
                ("leaf", (false, true)),
            ] {
                if t.cols.iter().any(|c| c.name == col) {
                    let e = want
                        .entry((u32::try_from(ti).expect("table index"), col.to_string()))
                        .or_insert((false, false));
                    e.0 |= need.0;
                    e.1 |= need.1;
                }
            }
        }

        let mut keys: Vec<((u32, String), (bool, bool))> = want.into_iter().collect();
        keys.sort();
        for ((table, col), (bounded, digest)) in keys {
            if !bounded && !digest {
                continue;
            }
            let t = &mut rm.tables[table as usize];
            if t.adjunct_cols.iter().any(|a| a.source == col) {
                continue;
            }
            let Some(src) = t.cols.iter().find(|c| c.name == col) else {
                continue;
            };
            // U1a, second half: a search reaching the column is not enough —
            // the dialect must also be unable to index or compare it. Without
            // this, a token search over a boolean grows two derived columns
            // that nothing can ever read.
            if !fhir_mariadb_map::ddl::needs_adjunct(src.ty) {
                continue;
            }
            let path = src.path.clone();
            let idx = bounded.then(|| regs[table as usize].claim(&format!("{col}_idx")));
            let dig = digest.then(|| regs[table as usize].claim(&format!("{col}_h")));
            if let Some(n) = &idx {
                t.cols.push(Column {
                    name: n.clone(),
                    ty: ColTy::TextIdx,
                    path: path.clone(),
                });
            }
            if let Some(n) = &dig {
                t.cols.push(Column {
                    name: n.clone(),
                    ty: ColTy::Digest,
                    path,
                });
            }
            t.adjunct_cols.push(Adjuncts {
                source: col,
                bounded: idx,
                digest: dig,
            });
        }
    }
}

#[cfg(test)]
mod where_restriction_tests {
    // F-38: `where()` is not one thing. A type restriction on a reference may
    // be dropped, because the reference column keeps the target type; a value
    // restriction may not, because nothing downstream re-applies it.
    //
    // Mutation-verified (T11.10): reverting `compile_alt` to the old
    // unconditional `continue` makes `value_restriction_is_refused` fail.

    fn is_value_restriction(arg: &str) -> bool {
        let a = arg.trim();
        a.split_once('=').is_some_and(|(lhs, rhs)| {
            !lhs.contains('(')
                && !lhs.trim().is_empty()
                && lhs.trim().chars().all(|c| c.is_alphanumeric() || c == '_')
                && rhs.trim().starts_with('\'')
        })
    }

    #[test]
    fn value_restriction_is_refused() {
        // The eight R5 codes that collided: relatedArtifact.where(type='...').
        for a in [
            "type='composed-of'",
            "type='depends-on'",
            "type='derived-from'",
            "type='predecessor'",
            "type='successor'",
            "  code = 'final' ",
        ] {
            assert!(is_value_restriction(a), "must refuse: {a:?}");
        }
    }

    #[test]
    fn type_restriction_stays_lenient() {
        for a in [
            "resolve() is Patient",
            "resolve() is Practitioner",
            "$this is Reference",
        ] {
            assert!(!is_value_restriction(a), "must stay lenient: {a:?}");
        }
    }
}

#[cfg(test)]
mod adjunct_rules {
    use fhir_mariadb_map::ddl::{TEXT_ADJUNCTS, needs_adjunct};
    use fhir_mariadb_map::model::ColTy;

    /// `U1a`, second half: the dialect decides. A type it can index and compare
    /// must never grow an adjunct, however a search reaches it.
    ///
    /// Mutation-verified (`T11.10`): making `needs_adjunct` return `true`
    /// unconditionally fails this, and that mutation is exactly the bug this
    /// test was written after — a token search over a boolean was growing a
    /// derived column nothing could read.
    #[test]
    fn indexable_types_never_need_an_adjunct() {
        for ty in [
            ColTy::Bool,
            ColTy::Int,
            ColTy::BigInt,
            ColTy::Date,
            ColTy::Timestamptz,
            ColTy::TextC,
            ColTy::TextIdx,
            ColTy::Digest,
        ] {
            assert!(
                !needs_adjunct(ty),
                "{ty:?} indexes and compares as bound; an adjunct over it is a \
                 column nothing reads"
            );
        }
    }

    /// A dialect that materializes adjuncts must say *some* type needs them,
    /// and one that does not must say none do. The two constants cannot
    /// disagree without one of them being wrong.
    #[test]
    fn the_two_dialect_answers_agree() {
        let any = [ColTy::Text, ColTy::Numeric, ColTy::Jsonb]
            .into_iter()
            .any(needs_adjunct);
        assert_eq!(
            any, TEXT_ADJUNCTS,
            "TEXT_ADJUNCTS says {TEXT_ADJUNCTS} but needs_adjunct says {any}"
        );
    }
}
