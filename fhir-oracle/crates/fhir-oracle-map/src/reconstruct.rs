//! Reconstruction: relational rows → the original FHIR resource (JSON).
//!
//! The inverse of shredding. Consumption is audited: every stored row must be
//! used exactly once, so a map/schema mismatch or corrupted ordinal surfaces
//! as an integrity error instead of silent data loss.

use std::collections::{BTreeMap, HashMap};

use serde_json::{Map, Value};

use crate::error::ShredError;
use crate::model::{Elem, ElemKind, Prim, PrimCol, ResourceMap};
use crate::shred::{DeepRow, ExtRow};
use crate::value::{LeafVal, unflatten};

/// One stored row read back: only non-null data columns, as their text images.
#[derive(Debug, Clone)]
pub struct InRow {
    pub ords: Vec<i16>,
    pub cols: HashMap<String, String>,
}

/// Everything stored for one resource instance.
#[derive(Debug, Default)]
pub struct ReconIn {
    /// Indexed parallel to `ResourceMap.tables`; system tables' entries stay
    /// empty.
    pub tables: Vec<Vec<InRow>>,
    pub ext: Vec<ExtRow>,
    pub deep: Vec<DeepRow>,
    pub contained: Vec<(i16, Value)>,
}

pub fn reconstruct(
    rm: &ResourceMap,
    input: &ReconIn,
    id: Option<&str>,
) -> Result<Value, ShredError> {
    let mut rows: Vec<HashMap<&[i16], &InRow>> = Vec::with_capacity(rm.tables.len());
    let mut kids: Vec<HashMap<&[i16], Vec<i16>>> = Vec::with_capacity(rm.tables.len());
    for trs in &input.tables {
        let mut m: HashMap<&[i16], &InRow> = HashMap::new();
        let mut k: HashMap<&[i16], Vec<i16>> = HashMap::new();
        for r in trs {
            if m.insert(r.ords.as_slice(), r).is_some() {
                return Err(ShredError::integrity("duplicate ords in one table"));
            }
            if let Some((last, parent)) = r.ords.split_last() {
                k.entry(parent).or_default().push(*last);
            }
        }
        for v in k.values_mut() {
            v.sort_unstable();
        }
        rows.push(m);
        kids.push(k);
    }

    // Extension attaches, grouped by (path, ords).
    let mut ext: HashMap<(String, Vec<i16>), Attach> = HashMap::new();
    let mut ext_kids: HashMap<(String, Vec<i16>), Vec<i16>> = HashMap::new();
    for e in &input.ext {
        let a = ext.entry((e.path.clone(), e.ords.clone())).or_default();
        if e.ext_ord == 0 {
            match &e.val {
                LeafVal::Str(s) if e.leaf == "id" => a.id = Some(s.clone()),
                _ => return Err(ShredError::integrity("malformed element-id ext row")),
            }
        } else {
            let side = if e.modifier { &mut a.mods } else { &mut a.exts };
            let entry = side
                .entry(e.ext_ord)
                .or_insert_with(|| (e.url.clone().unwrap_or_default(), Vec::new()));
            entry.1.push((e.leaf.clone(), e.val.clone()));
        }
        if let Some((last, parent)) = e.ords.split_last() {
            let k = ext_kids
                .entry((e.path.clone(), parent.to_vec()))
                .or_default();
            if !k.contains(last) {
                k.push(*last);
            }
        }
    }

    // Spill leaves, grouped by (path, ords).
    let mut deep: HashMap<(String, Vec<i16>), Leaves> = HashMap::new();
    for d in &input.deep {
        deep.entry((d.path.clone(), d.ords.clone()))
            .or_default()
            .push((d.leaf.clone(), d.val.clone()));
    }

    let mut rc = Rc {
        rm,
        rows,
        kids,
        ext,
        ext_kids,
        deep,
        contained: &input.contained,
        rows_used: 0,
        rows_total: input.tables.iter().map(Vec::len).sum(),
    };

    if !rc.rows[0].contains_key(&[][..]) {
        return Err(ShredError::integrity("missing base row"));
    }
    rc.rows_used += 1; // the base row

    let body = rc.walk_node(rm.root, 0, &[], "")?;
    let mut out = Map::new();
    out.insert("resourceType".to_string(), Value::String(rm.name.clone()));
    if let Some(id) = id {
        out.insert("id".to_string(), Value::String(id.to_string()));
    }
    for (k, v) in body {
        out.insert(k, v);
    }

    if rc.rows_used != rc.rows_total {
        return Err(ShredError::integrity(format!(
            "{} of {} stored rows unconsumed — ordinal gap or map mismatch",
            rc.rows_total - rc.rows_used,
            rc.rows_total
        )));
    }
    if let Some((p, o)) = rc.ext.keys().next() {
        return Err(ShredError::integrity(format!(
            "unconsumed extension rows at {p:?} {o:?}"
        )));
    }
    if let Some((p, o)) = rc.deep.keys().next() {
        return Err(ShredError::integrity(format!(
            "unconsumed spill rows at {p:?} {o:?}"
        )));
    }
    Ok(Value::Object(out))
}

/// Leaves of one extension instance: (leaf path, value).
type Leaves = Vec<(String, LeafVal)>;
/// One attach point's extensions by array ordinal: (url, leaves).
type ExtGroup = BTreeMap<i16, (String, Leaves)>;

#[derive(Default)]
struct Attach {
    id: Option<String>,
    exts: ExtGroup,
    mods: ExtGroup,
}

struct Rc<'a> {
    rm: &'a ResourceMap,
    rows: Vec<HashMap<&'a [i16], &'a InRow>>,
    kids: Vec<HashMap<&'a [i16], Vec<i16>>>,
    ext: HashMap<(String, Vec<i16>), Attach>,
    ext_kids: HashMap<(String, Vec<i16>), Vec<i16>>,
    deep: HashMap<(String, Vec<i16>), Leaves>,
    contained: &'a [(i16, Value)],
    rows_used: usize,
    rows_total: usize,
}

fn jp(prefix: &str, seg: &str) -> String {
    if prefix.is_empty() {
        seg.to_string()
    } else {
        format!("{prefix}.{seg}")
    }
}

fn plus(ords: &[i16], i: i16) -> Vec<i16> {
    let mut v = ords.to_vec();
    v.push(i);
    v
}

impl<'a> Rc<'a> {
    /// Build the object content for one node at (table, ords). Returns an
    /// empty map when nothing is stored there (element absent).
    fn walk_node(
        &mut self,
        node: u32,
        table: u32,
        ords: &[i16],
        jpath: &str,
    ) -> Result<Map<String, Value>, ShredError> {
        let mut out = Map::new();

        // Element id / extensions attached to this object.
        if let Some(a) = self.ext.remove(&(jpath.to_string(), ords.to_vec())) {
            if let Some(id) = &a.id {
                out.insert("id".to_string(), Value::String(id.clone()));
            }
            if !a.exts.is_empty() {
                out.insert("extension".to_string(), build_ext_array(&a.exts)?);
            }
            if !a.mods.is_empty() {
                out.insert("modifierExtension".to_string(), build_ext_array(&a.mods)?);
            }
        }

        let row: Option<&InRow> = self.rows[table as usize].get(ords).copied();
        let rm = self.rm;
        for elem in &rm.node(node).elems {
            self.emit_elem(elem, row, table, ords, jpath, &mut out)?;
        }
        Ok(out)
    }

    fn emit_elem(
        &mut self,
        elem: &'a Elem,
        row: Option<&'a InRow>,
        table: u32,
        ords: &[i16],
        jpath: &str,
        out: &mut Map<String, Value>,
    ) -> Result<(), ShredError> {
        match &elem.kind {
            ElemKind::Choice(variants) => {
                match elem.table {
                    Some(t) => {
                        let o = plus(ords, 1);
                        let Some(srow) = self.take_row(t, &o) else {
                            return Ok(());
                        };
                        for var in variants {
                            self.emit_elem(var, Some(srow), t, &o, jpath, out)?;
                        }
                    }
                    None => {
                        for var in variants {
                            self.emit_elem(var, row, table, ords, jpath, out)?;
                        }
                    }
                }
                Ok(())
            }
            ElemKind::Prim(pc) => {
                if elem.repeats {
                    let t = elem.table.expect("repeating prim has a table");
                    self.emit_prim_array(elem, pc, t, ords, jpath, out)
                } else {
                    let epath = jp(jpath, &elem.json);
                    if let Some(text) = row.and_then(|r| r.cols.get(&pc.col)) {
                        out.insert(elem.json.clone(), prim_json(pc.prim, text)?);
                    }
                    self.emit_prim_ext(&epath, ords, &format!("_{}", elem.json), out)?;
                    Ok(())
                }
            }
            ElemKind::RefStr(rc) => {
                let epath = jp(jpath, &elem.json);
                if let Some(r) = row {
                    if let (Some(t), Some(i)) = (r.cols.get(&rc.c_type), r.cols.get(&rc.c_id)) {
                        out.insert(elem.json.clone(), Value::String(format!("{t}/{i}")));
                    } else if let Some(u) = r.cols.get(&rc.c_url) {
                        out.insert(elem.json.clone(), Value::String(u.clone()));
                    }
                }
                self.emit_prim_ext(&epath, ords, &format!("_{}", elem.json), out)?;
                Ok(())
            }
            ElemKind::Group(child) => {
                let child = *child;
                let epath = jp(jpath, &elem.json);
                if elem.repeats {
                    let t = elem.table.expect("repeating group has a table");
                    let indexes = self.contiguous_kids(t, ords, elem.neg_lane, &epath)?;
                    if indexes == 0 {
                        return Ok(());
                    }
                    let mut arr = Vec::with_capacity(indexes as usize);
                    for i in 1..=indexes {
                        let o = plus(ords, if elem.neg_lane { -i } else { i });
                        self.take_row(t, &o).ok_or_else(|| {
                            ShredError::integrity(format!("ordinal gap in {epath}"))
                        })?;
                        let m = self.walk_node(child, t, &o, &epath)?;
                        arr.push(Value::Object(m));
                    }
                    out.insert(elem.json.clone(), Value::Array(arr));
                } else {
                    match elem.table {
                        Some(t) => {
                            let o = plus(ords, if elem.neg_lane { -1 } else { 1 });
                            if self.take_row(t, &o).is_none() {
                                return Ok(());
                            }
                            let m = self.walk_node(child, t, &o, &epath)?;
                            if !m.is_empty() {
                                out.insert(elem.json.clone(), Value::Object(m));
                            }
                        }
                        None => {
                            let m = self.walk_node(child, table, ords, &epath)?;
                            if !m.is_empty() {
                                out.insert(elem.json.clone(), Value::Object(m));
                            }
                        }
                    }
                }
                Ok(())
            }
            ElemKind::Spill => {
                let epath = jp(jpath, &elem.json);
                if elem.repeats {
                    let mut arr = Vec::new();
                    let mut i: i16 = 1;
                    while let Some(leaves) = self.deep.remove(&(epath.clone(), plus(ords, i))) {
                        arr.push(unflatten(&leaves)?);
                        i += 1;
                    }
                    if !arr.is_empty() {
                        out.insert(elem.json.clone(), Value::Array(arr));
                    }
                } else if let Some(leaves) = self.deep.remove(&(epath.clone(), ords.to_vec())) {
                    out.insert(elem.json.clone(), unflatten(&leaves)?);
                }
                Ok(())
            }
            ElemKind::ResourceValue(col) => {
                if let Some(text) = row.and_then(|r| r.cols.get(col)) {
                    let v: Value = serde_json::from_str(text)
                        .map_err(|e| ShredError::integrity(e.to_string()))?;
                    out.insert(elem.json.clone(), v);
                }
                Ok(())
            }
            ElemKind::Contained => {
                if !self.contained.is_empty() {
                    let mut items: Vec<(i16, Value)> = self.contained.to_vec();
                    items.sort_by_key(|(o, _)| *o);
                    out.insert(
                        elem.json.clone(),
                        Value::Array(items.into_iter().map(|(_, v)| v).collect()),
                    );
                }
                Ok(())
            }
        }
    }

    /// Count and verify contiguous child indexes ±1..=±n under
    /// (table, ords), looking only at this element's lane sign — recursion
    /// lanes (see `Elem::neg_lane`) share tables but never signs.
    fn contiguous_kids(
        &self,
        t: u32,
        ords: &[i16],
        neg: bool,
        path: &str,
    ) -> Result<i16, ShredError> {
        let Some(ix) = self.kids[t as usize].get(ords) else {
            return Ok(0);
        };
        // The kids index only contains direct children (parent = ords);
        // deeper descendants of recursive tables have longer parents.
        let lane: Vec<i16> = ix
            .iter()
            .copied()
            .filter(|o| if neg { *o < 0 } else { *o > 0 })
            .map(i16::abs)
            .collect();
        let mut sorted = lane.clone();
        sorted.sort_unstable();
        let n = sorted.len() as i16;
        for (want, got) in (1..=n).zip(sorted.iter()) {
            if want != *got {
                return Err(ShredError::integrity(format!("ordinal gap in {path}")));
            }
        }
        Ok(n)
    }

    fn take_row(&mut self, t: u32, ords: &[i16]) -> Option<&'a InRow> {
        let r = self.rows[t as usize].get(ords).copied();
        if r.is_some() {
            self.rows_used += 1;
        }
        r
    }

    fn emit_prim_array(
        &mut self,
        elem: &Elem,
        pc: &PrimCol,
        t: u32,
        ords: &[i16],
        jpath: &str,
        out: &mut Map<String, Value>,
    ) -> Result<(), ShredError> {
        let epath = jp(jpath, &elem.json);
        let max_row = self.kids[t as usize]
            .get(ords)
            .and_then(|v| v.last().copied())
            .unwrap_or(0);
        let max_ext = self
            .ext_kids
            .get(&(epath.clone(), ords.to_vec()))
            .and_then(|v| v.iter().max().copied())
            .unwrap_or(0);
        let len = max_row.max(max_ext);
        if len == 0 {
            return Ok(());
        }
        let mut vals = Vec::with_capacity(len as usize);
        let mut pexts = Vec::with_capacity(len as usize);
        let mut any_val = false;
        let mut any_ext = false;
        for i in 1..=len {
            let o = plus(ords, i);
            match self.take_row(t, &o) {
                Some(r) => match r.cols.get(&pc.col) {
                    Some(text) => {
                        vals.push(prim_json(pc.prim, text)?);
                        any_val = true;
                    }
                    None => vals.push(Value::Null),
                },
                None => vals.push(Value::Null),
            }
            match self.build_prim_ext(&epath, &o)? {
                Some(v) => {
                    pexts.push(v);
                    any_ext = true;
                }
                None => pexts.push(Value::Null),
            }
        }
        if any_val {
            out.insert(elem.json.clone(), Value::Array(vals));
        }
        if any_ext {
            out.insert(format!("_{}", elem.json), Value::Array(pexts));
        }
        Ok(())
    }

    fn emit_prim_ext(
        &mut self,
        epath: &str,
        ords: &[i16],
        key: &str,
        out: &mut Map<String, Value>,
    ) -> Result<(), ShredError> {
        if let Some(v) = self.build_prim_ext(epath, ords)? {
            out.insert(key.to_string(), v);
        }
        Ok(())
    }

    fn build_prim_ext(&mut self, epath: &str, ords: &[i16]) -> Result<Option<Value>, ShredError> {
        let Some(a) = self.ext.remove(&(epath.to_string(), ords.to_vec())) else {
            return Ok(None);
        };
        let mut m = Map::new();
        if let Some(id) = &a.id {
            m.insert("id".to_string(), Value::String(id.clone()));
        }
        if !a.exts.is_empty() {
            m.insert("extension".to_string(), build_ext_array(&a.exts)?);
        }
        if !a.mods.is_empty() {
            return Err(ShredError::integrity(
                "modifier extension attached to a primitive",
            ));
        }
        Ok(Some(Value::Object(m)))
    }
}

fn build_ext_array(exts: &ExtGroup) -> Result<Value, ShredError> {
    let mut arr = Vec::with_capacity(exts.len());
    for (i, (ord, (url, leaves))) in exts.iter().enumerate() {
        if usize::from(ord.unsigned_abs()) != i + 1 {
            return Err(ShredError::integrity("extension ordinal gap"));
        }
        let inner = unflatten(leaves)?;
        let mut m = Map::new();
        m.insert("url".to_string(), Value::String(url.clone()));
        if let Value::Object(im) = inner {
            for (k, v) in im {
                m.insert(k, v);
            }
        } else {
            return Err(ShredError::integrity("extension content is not an object"));
        }
        arr.push(Value::Object(m));
    }
    Ok(Value::Array(arr))
}

/// Parse a column's text image back into its JSON form.
fn prim_json(prim: Prim, text: &str) -> Result<Value, ShredError> {
    Ok(match prim {
        Prim::Bool => match text {
            "true" | "t" => Value::Bool(true),
            "false" | "f" => Value::Bool(false),
            _ => return Err(ShredError::integrity(format!("bad boolean image {text:?}"))),
        },
        Prim::Int => {
            let n: i64 = text
                .parse()
                .map_err(|_| ShredError::integrity(format!("bad integer image {text:?}")))?;
            Value::Number(serde_json::Number::from(n))
        }
        Prim::Int64 => Value::String(text.to_string()),
        Prim::Decimal => Value::Number(serde_json::Number::from_string_unchecked(text.to_string())),
        Prim::Date | Prim::DateTime | Prim::Instant | Prim::Time | Prim::Str => {
            Value::String(text.to_string())
        }
    })
}
