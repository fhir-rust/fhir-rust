//! Search-parameter compilation: resolve each SearchParameter's FHIRPath
//! expression against the built map, producing concrete (table, column)
//! targets. Parameters that use FHIRPath features beyond the supported
//! subset compile to an empty target list with a note — the server reports
//! them as unsupported rather than guessing.

use std::collections::HashMap;
use std::path::Path;

use fhir_postgresql_map::model::{
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
        if seg.starts_with("where(") {
            continue; // typed-reference restriction — lenient
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
    use fhir_postgresql_map::model::{ColTy, Column};

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
