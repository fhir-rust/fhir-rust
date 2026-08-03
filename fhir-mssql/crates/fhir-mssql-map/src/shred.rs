//! Shredding: one FHIR resource (JSON) → relational rows, driven by the map.

use std::collections::{HashMap, HashSet};

use serde_json::{Map, Value};

use crate::error::ShredError;
use crate::model::{Elem, ElemKind, Prim, ResourceMap};
use crate::value::{LeafVal, ParsedRef, date_sort, datetime_sort, flatten, parse_reference};

/// A typed SQL value; every variant carries the exact image sent to (and read
/// back from) PostgreSQL as text with an explicit cast.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlVal {
    Bool(bool),
    Int(i64),
    /// numeric, lexical form.
    Num(String),
    Text(String),
    /// timestamptz sort image.
    Ts(String),
    /// date sort image.
    Date(String),
    Jsonb(String),
    /// Raw bytes. Today this is only the unbounded-string checksum adjunct
    /// (`U4a`), which is SHA-256's 32 bytes stored binary rather than as hex.
    ///
    /// Adding a value type obliges every store to bind it, and that is where
    /// **F-20** was found: booleans, integers and dates were dropped on one
    /// engine and panicked on two others because each store bound them
    /// separately and nothing tested the binding. `U4a` requires a driver
    /// round-trip test wherever this column is materialized, for that reason.
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Row {
    pub table: u32,
    pub ords: Vec<i16>,
    pub cols: Vec<(String, SqlVal)>,
}

#[derive(Debug, Clone)]
pub struct ExtRow {
    /// Dotted JSON-name attach path ("" = resource level, "name.given", …).
    pub path: String,
    /// 1-based ordinals at each repeating crossing along `path`.
    pub ords: Vec<i16>,
    pub modifier: bool,
    /// 1-based index in the extension array; 0 is the element-id row.
    pub ext_ord: i16,
    pub url: Option<String>,
    pub leaf: String,
    pub val: LeafVal,
}

#[derive(Debug, Clone)]
pub struct DeepRow {
    pub path: String,
    pub ords: Vec<i16>,
    pub leaf: String,
    pub val: LeafVal,
}

#[derive(Debug)]
pub struct ShredOut {
    pub id: Option<String>,
    pub rows: Vec<Row>,
    pub ext: Vec<ExtRow>,
    pub deep: Vec<DeepRow>,
    pub contained: Vec<(i16, Value)>,
}

pub fn shred(rm: &ResourceMap, v: &Value) -> Result<ShredOut, ShredError> {
    let obj = v
        .as_object()
        .ok_or_else(|| ShredError::at("", "resource must be a JSON object"))?;
    let rt = obj
        .get("resourceType")
        .and_then(Value::as_str)
        .ok_or_else(|| ShredError::at("resourceType", "missing or not a string"))?;
    if rt != rm.name {
        return Err(ShredError::at(
            "resourceType",
            format!("expected {:?}, found {rt:?}", rm.name),
        ));
    }
    let mut sh = Sh {
        rm,
        rows: Vec::new(),
        idx: HashMap::new(),
        ext: Vec::new(),
        deep: Vec::new(),
        contained: Vec::new(),
        id: None,
    };
    sh.walk_obj(rm.root, obj, 0, &[], "", true)?;
    fill_norm_cols(rm, &mut sh.rows);
    // U3: derived, written here, never read back by reconstruct.
    fill_adjunct_cols(rm, &mut sh.rows);
    Ok(ShredOut {
        id: sh.id,
        rows: sh.rows,
        ext: sh.ext,
        deep: sh.deep,
        contained: sh.contained,
    })
}

/// Fill each table's folded search columns (P6.6) from the values just
/// written. A row that never set the source column gets no folded value, so
/// the folded column stays NULL alongside it rather than becoming an empty
/// string that would match a prefix search for "".
fn fill_norm_cols(rm: &ResourceMap, rows: &mut [Row]) {
    for row in rows {
        let pairs = &rm.tables[row.table as usize].norm_cols;
        if pairs.is_empty() {
            continue;
        }
        let mut add: Vec<(String, SqlVal)> = Vec::new();
        for (src, dst) in pairs {
            if let Some((_, SqlVal::Text(v))) = row.cols.iter().find(|(c, _)| c == src) {
                add.push((dst.clone(), SqlVal::Text(crate::fold::fold(v))));
            }
        }
        row.cols.extend(add);
    }
}

/// Fill each table's unbounded-string adjuncts (`U1`, `U3`) from the values
/// just written.
///
/// Empty on the four ports whose `ddl::TEXT_ADJUNCTS` is false, because their
/// maps carry no `adjunct_cols` at all (`U9`) — the loop costs one `is_empty`
/// per row there.
///
/// Both columns are written together (`U2`), and only where the source column
/// was actually set: a row that never wrote the source gets neither adjunct,
/// exactly as `fill_norm_cols` leaves the folded column NULL rather than an
/// empty string that would match a prefix search for "".
///
/// `U3` makes these derived: they are not part of the resource, so they must
/// not affect `R4.2` round-trip fidelity and must not enter `M3.16`'s
/// hash-chain pre-image. Both hold by construction here — `reconstruct` walks
/// the map's elements and never sees a derived column, and the chain
/// pre-image is built from canonical JSON of the resource, not from rows.
fn fill_adjunct_cols(rm: &ResourceMap, rows: &mut [Row]) {
    for row in rows {
        let adjuncts = &rm.tables[row.table as usize].adjunct_cols;
        if adjuncts.is_empty() {
            continue;
        }
        let mut add: Vec<(String, SqlVal)> = Vec::new();
        for a in adjuncts {
            let Some((_, SqlVal::Text(v))) = row.cols.iter().find(|(c, _)| c == &a.source) else {
                continue;
            };
            // U2b: fill what the map says exists, never a fixed pair. A column
            // searched only for equality has no bounded adjunct to fill, and
            // writing one would create a column no query ever reads.
            if let Some(idx) = &a.bounded {
                // U5: the folded form, truncated by characters. ADJUNCT_BOUND
                // matches what `ddl::col_sql` declares.
                let folded = crate::fold::bounded(v, ADJUNCT_BOUND);
                add.push((idx.clone(), SqlVal::Text(folded)));
            }
            if let Some(dig) = &a.digest {
                // U4/U4a: SHA-256 over the whole value, computed in Rust,
                // carried as its 32 raw bytes rather than as hex text.
                let d = crate::fold::digest(v).to_vec();
                add.push((dig.clone(), SqlVal::Bytes(d)));
            }
        }
        row.cols.extend(add);
    }
}

/// Characters kept in the bounded adjunct (`U1`, `U10`).
///
/// Must equal the width `ddl::col_sql` gives `ColTy::TextIdx`. A shredder that
/// wrote more than the column holds would be truncated by the engine — silently
/// on some — and the two ports that materialize adjuncts both declare 450.
const ADJUNCT_BOUND: usize = 450;

struct Sh<'a> {
    rm: &'a ResourceMap,
    rows: Vec<Row>,
    idx: HashMap<(u32, Vec<i16>), usize>,
    ext: Vec<ExtRow>,
    deep: Vec<DeepRow>,
    contained: Vec<(i16, Value)>,
    id: Option<String>,
}

fn jp(prefix: &str, seg: &str) -> String {
    if prefix.is_empty() {
        seg.to_string()
    } else {
        format!("{prefix}.{seg}")
    }
}

fn push_ord(ords: &[i16], i: usize, neg: bool, path: &str) -> Result<Vec<i16>, ShredError> {
    let o: i16 = i16::try_from(i).map_err(|_| ShredError::at(path, "array too long"))?;
    let mut v = ords.to_vec();
    v.push(if neg { -o } else { o });
    Ok(v)
}

impl<'a> Sh<'a> {
    fn ensure_row(&mut self, table: u32, ords: &[i16]) -> usize {
        let key = (table, ords.to_vec());
        if let Some(&i) = self.idx.get(&key) {
            return i;
        }
        self.rows.push(Row {
            table,
            ords: ords.to_vec(),
            cols: Vec::new(),
        });
        let i = self.rows.len() - 1;
        self.idx.insert(key, i);
        i
    }

    fn put(&mut self, table: u32, ords: &[i16], col: &str, val: SqlVal) {
        let i = self.ensure_row(table, ords);
        self.rows[i].cols.push((col.to_string(), val));
    }

    fn walk_obj(
        &mut self,
        node: u32,
        obj: &Map<String, Value>,
        table: u32,
        ords: &[i16],
        jpath: &str,
        root: bool,
    ) -> Result<(), ShredError> {
        self.ensure_row(table, ords);
        let mut consumed: HashSet<&str> = HashSet::new();

        if root {
            consumed.insert("resourceType");
            if let Some(idv) = obj.get("id") {
                let s = idv
                    .as_str()
                    .ok_or_else(|| ShredError::at("id", "resource id must be a string"))?;
                self.id = Some(s.to_string());
                consumed.insert("id");
            }
        } else if let Some(idv) = obj.get("id") {
            let s = idv
                .as_str()
                .ok_or_else(|| ShredError::at(jp(jpath, "id"), "element id must be a string"))?;
            self.ext.push(ExtRow {
                path: jpath.to_string(),
                ords: ords.to_vec(),
                modifier: false,
                ext_ord: 0,
                url: None,
                leaf: "id".to_string(),
                val: LeafVal::Str(s.to_string()),
            });
            consumed.insert("id");
        }
        if let Some(v) = obj.get("extension") {
            self.ext_array(v, jpath, ords, false)?;
            consumed.insert("extension");
        }
        if let Some(v) = obj.get("modifierExtension") {
            self.ext_array(v, jpath, ords, true)?;
            consumed.insert("modifierExtension");
        }

        // `rm` is a shared reference with the walker's lifetime, so holding
        // element borrows across &mut self calls is fine.
        let rm = self.rm;
        for elem in &rm.node(node).elems {
            self.handle_elem(elem, obj, table, ords, jpath, &mut consumed)?;
        }

        for key in obj.keys() {
            if !consumed.contains(key.as_str()) {
                return Err(ShredError::at(
                    jp(jpath, key),
                    "unknown element for this FHIR version",
                ));
            }
        }
        Ok(())
    }

    fn handle_elem<'k>(
        &mut self,
        elem: &'k Elem,
        obj: &'k Map<String, Value>,
        table: u32,
        ords: &[i16],
        jpath: &str,
        consumed: &mut HashSet<&'k str>,
    ) -> Result<(), ShredError> {
        if let ElemKind::Choice(variants) = &elem.kind {
            let mut found: Option<&Elem> = None;
            for var in variants {
                let has =
                    obj.contains_key(&var.json) || obj.contains_key(&format!("_{}", var.json));
                if has {
                    if let Some(prev) = found {
                        return Err(ShredError::at(
                            jp(jpath, &var.json),
                            format!("choice element also present as {:?}", prev.json),
                        ));
                    }
                    found = Some(var);
                }
            }
            let Some(var) = found else { return Ok(()) };
            // A force-split choice owns a table; its variants' columns live
            // in that table's row.
            let (t, o): (u32, Vec<i16>) = match elem.table {
                Some(t) => {
                    let o = push_ord(ords, 1, false, jpath)?;
                    self.ensure_row(t, &o);
                    (t, o)
                }
                None => (table, ords.to_vec()),
            };
            return self.handle_elem(var, obj, t, &o, jpath, consumed);
        }

        let v = obj.get(&elem.json);
        let pext_key = format!("_{}", elem.json);
        let pext = obj.get(pext_key.as_str());
        if v.is_none() && pext.is_none() {
            return Ok(());
        }
        if v.is_some() {
            let (k, _) = obj
                .get_key_value(elem.json.as_str())
                .expect("checked present");
            consumed.insert(k.as_str());
        }
        if pext.is_some() {
            // Mark the `_name` key consumed; we must find the owned key in
            // the map to insert a borrowed &str with the right lifetime.
            let (k, _) = obj
                .get_key_value(pext_key.as_str())
                .expect("checked present");
            consumed.insert(k.as_str());
        }
        let epath = jp(jpath, &elem.json);

        match &elem.kind {
            ElemKind::Choice(_) => unreachable!("handled above"),
            ElemKind::Prim(pc) => {
                if elem.repeats {
                    let t = elem.table.expect("repeating prim has a table");
                    self.prim_array(pc, v, pext, t, ords, &epath)?;
                } else {
                    if let Some(v) = v {
                        let (val, sort) = prim_val(pc.prim, v, &epath)?;
                        self.put(table, ords, &pc.col, val);
                        if let (Some(sc), Some(sv)) = (&pc.sort, sort) {
                            self.put(table, ords, sc, sv);
                        }
                    }
                    if let Some(p) = pext {
                        self.prim_ext(p, &epath, ords)?;
                    }
                }
            }
            ElemKind::RefStr(rc) => {
                if let Some(v) = v {
                    let s = v
                        .as_str()
                        .ok_or_else(|| ShredError::at(&epath, "reference must be a string"))?;
                    match parse_reference(s) {
                        ParsedRef::Relative { rtype, rid } => {
                            self.put(table, ords, &rc.c_type, SqlVal::Text(rtype));
                            self.put(table, ords, &rc.c_id, SqlVal::Text(rid));
                        }
                        ParsedRef::Other(u) => {
                            self.put(table, ords, &rc.c_url, SqlVal::Text(u));
                        }
                    }
                }
                if let Some(p) = pext {
                    self.prim_ext(p, &epath, ords)?;
                }
            }
            ElemKind::Group(child) => {
                let child = *child;
                let Some(v) = v else {
                    return Err(ShredError::at(
                        &epath,
                        "primitive-extension form on a complex element",
                    ));
                };
                if elem.repeats {
                    let t = elem.table.expect("repeating group has a table");
                    let arr = v
                        .as_array()
                        .ok_or_else(|| ShredError::at(&epath, "expected an array"))?;
                    if arr.is_empty() {
                        return Err(ShredError::at(&epath, "empty array"));
                    }
                    for (i, item) in arr.iter().enumerate() {
                        let o = push_ord(ords, i + 1, elem.neg_lane, &epath)?;
                        let m = item
                            .as_object()
                            .ok_or_else(|| ShredError::at(&epath, "expected objects"))?;
                        self.walk_obj(child, m, t, &o, &epath, false)?;
                    }
                } else {
                    let m = v
                        .as_object()
                        .ok_or_else(|| ShredError::at(&epath, "expected an object"))?;
                    match elem.table {
                        Some(t) => {
                            let o = push_ord(ords, 1, elem.neg_lane, &epath)?;
                            self.walk_obj(child, m, t, &o, &epath, false)?;
                        }
                        None => self.walk_obj(child, m, table, ords, &epath, false)?,
                    }
                }
            }
            ElemKind::Spill => {
                let Some(v) = v else {
                    return Err(ShredError::at(
                        &epath,
                        "primitive-extension form on a complex element",
                    ));
                };
                if elem.repeats {
                    let arr = v
                        .as_array()
                        .ok_or_else(|| ShredError::at(&epath, "expected an array"))?;
                    for (i, item) in arr.iter().enumerate() {
                        let o = push_ord(ords, i + 1, false, &epath)?;
                        for (leaf, val) in flatten(item, &epath, None)? {
                            self.deep.push(DeepRow {
                                path: epath.clone(),
                                ords: o.clone(),
                                leaf,
                                val,
                            });
                        }
                    }
                } else {
                    for (leaf, val) in flatten(v, &epath, None)? {
                        self.deep.push(DeepRow {
                            path: epath.clone(),
                            ords: ords.to_vec(),
                            leaf,
                            val,
                        });
                    }
                }
            }
            ElemKind::ResourceValue(col) => {
                let Some(v) = v else {
                    return Err(ShredError::at(
                        &epath,
                        "primitive-extension form on a resource element",
                    ));
                };
                if !v.is_object() {
                    return Err(ShredError::at(&epath, "expected a resource object"));
                }
                let s =
                    serde_json::to_string(v).map_err(|e| ShredError::at(&epath, e.to_string()))?;
                self.put(table, ords, col, SqlVal::Jsonb(s));
            }
            ElemKind::Contained => {
                let Some(v) = v else {
                    return Err(ShredError::at(
                        &epath,
                        "primitive-extension form on contained",
                    ));
                };
                let arr = v
                    .as_array()
                    .ok_or_else(|| ShredError::at(&epath, "expected an array"))?;
                for (i, item) in arr.iter().enumerate() {
                    if !item.is_object() {
                        return Err(ShredError::at(&epath, "expected resource objects"));
                    }
                    let o = i16::try_from(i + 1)
                        .map_err(|_| ShredError::at(&epath, "array too long"))?;
                    self.contained.push((o, item.clone()));
                }
            }
        }
        Ok(())
    }

    /// A repeating primitive: parallel `name` / `_name` arrays.
    fn prim_array(
        &mut self,
        pc: &crate::model::PrimCol,
        v: Option<&Value>,
        pext: Option<&Value>,
        t: u32,
        ords: &[i16],
        epath: &str,
    ) -> Result<(), ShredError> {
        let empty: Vec<Value> = Vec::new();
        let arr = match v {
            Some(v) => v
                .as_array()
                .ok_or_else(|| ShredError::at(epath, "expected an array"))?,
            None => &empty,
        };
        let parr = match pext {
            Some(p) => p
                .as_array()
                .ok_or_else(|| ShredError::at(epath, "expected an array of extension objects"))?,
            None => &empty,
        };
        let len = arr.len().max(parr.len());
        if len == 0 {
            return Err(ShredError::at(epath, "empty array"));
        }
        for i in 0..len {
            let o = push_ord(ords, i + 1, false, epath)?;
            let item = arr.get(i).unwrap_or(&Value::Null);
            let pitem = parr.get(i).unwrap_or(&Value::Null);
            if item.is_null() && pitem.is_null() {
                return Err(ShredError::at(
                    epath,
                    format!("array entry {i} is null with no extension"),
                ));
            }
            if !item.is_null() {
                let (val, sort) = prim_val(pc.prim, item, epath)?;
                self.put(t, &o, &pc.col, val);
                if let (Some(sc), Some(sv)) = (&pc.sort, sort) {
                    self.put(t, &o, sc, sv);
                }
            }
            if !pitem.is_null() {
                self.prim_ext(pitem, epath, &o)?;
            }
        }
        Ok(())
    }

    /// A primitive-extension object: `{"id": …, "extension": […]}` attached
    /// at (path, ords).
    fn prim_ext(&mut self, p: &Value, path: &str, ords: &[i16]) -> Result<(), ShredError> {
        let m = p
            .as_object()
            .ok_or_else(|| ShredError::at(path, "primitive extension must be an object"))?;
        for key in m.keys() {
            if key != "id" && key != "extension" {
                return Err(ShredError::at(
                    path,
                    format!("unexpected key {key:?} in primitive extension"),
                ));
            }
        }
        if let Some(idv) = m.get("id") {
            let s = idv
                .as_str()
                .ok_or_else(|| ShredError::at(path, "element id must be a string"))?;
            self.ext.push(ExtRow {
                path: path.to_string(),
                ords: ords.to_vec(),
                modifier: false,
                ext_ord: 0,
                url: None,
                leaf: "id".to_string(),
                val: LeafVal::Str(s.to_string()),
            });
        }
        if let Some(v) = m.get("extension") {
            self.ext_array(v, path, ords, false)?;
        }
        Ok(())
    }

    fn ext_array(
        &mut self,
        v: &Value,
        path: &str,
        ords: &[i16],
        modifier: bool,
    ) -> Result<(), ShredError> {
        let arr = v
            .as_array()
            .ok_or_else(|| ShredError::at(path, "extension must be an array"))?;
        if arr.is_empty() {
            return Err(ShredError::at(path, "empty extension array"));
        }
        for (i, item) in arr.iter().enumerate() {
            let m = item
                .as_object()
                .ok_or_else(|| ShredError::at(path, "extension entries must be objects"))?;
            let url = m
                .get("url")
                .and_then(Value::as_str)
                .ok_or_else(|| ShredError::at(path, "extension without a url"))?;
            let ext_ord = i16::try_from(i + 1)
                .map_err(|_| ShredError::at(path, "extension array too long"))?;
            for (leaf, val) in flatten(item, path, Some("url"))? {
                self.ext.push(ExtRow {
                    path: path.to_string(),
                    ords: ords.to_vec(),
                    modifier,
                    ext_ord,
                    url: Some(url.to_string()),
                    leaf,
                    val,
                });
            }
        }
        Ok(())
    }
}

/// Validate one primitive JSON value and produce its column image plus an
/// optional derived sort image.
fn prim_val(prim: Prim, v: &Value, path: &str) -> Result<(SqlVal, Option<SqlVal>), ShredError> {
    match prim {
        Prim::Bool => v
            .as_bool()
            .map(|b| (SqlVal::Bool(b), None))
            .ok_or_else(|| ShredError::at(path, "expected a boolean")),
        Prim::Int => {
            let n = v
                .as_i64()
                .ok_or_else(|| ShredError::at(path, "expected an integer"))?;
            if i32::try_from(n).is_err() {
                return Err(ShredError::at(path, "integer out of 32-bit range"));
            }
            Ok((SqlVal::Int(n), None))
        }
        Prim::Int64 => {
            let s = v
                .as_str()
                .ok_or_else(|| ShredError::at(path, "integer64 must be a JSON string"))?;
            let n: i64 = s
                .parse()
                .map_err(|_| ShredError::at(path, "invalid integer64"))?;
            Ok((SqlVal::Int(n), None))
        }
        Prim::Decimal => match v {
            Value::Number(n) => Ok((SqlVal::Num(n.to_string()), None)),
            _ => Err(ShredError::at(path, "expected a decimal number")),
        },
        Prim::Date => {
            let s = as_str(v, path)?;
            let sort =
                date_sort(s).ok_or_else(|| ShredError::at(path, format!("invalid date {s:?}")))?;
            Ok((SqlVal::Text(s.to_string()), Some(SqlVal::Date(sort))))
        }
        Prim::DateTime | Prim::Instant => {
            let s = as_str(v, path)?;
            let sort = datetime_sort(s)
                .ok_or_else(|| ShredError::at(path, format!("invalid dateTime {s:?}")))?;
            Ok((SqlVal::Text(s.to_string()), Some(SqlVal::Ts(sort))))
        }
        Prim::Time | Prim::Str => {
            let s = as_str(v, path)?;
            Ok((SqlVal::Text(s.to_string()), None))
        }
    }
}

fn as_str<'v>(v: &'v Value, path: &str) -> Result<&'v str, ShredError> {
    v.as_str()
        .ok_or_else(|| ShredError::at(path, "expected a string"))
}
