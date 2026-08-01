//! Leaf encoding: lossless decomposition of arbitrary JSON subtrees into
//! (path, typed value) rows and back. Used for extension content and for
//! type-recursion spill (`_ext` and `_deep` tables).
//!
//! A leaf path is dotted; object keys appear verbatim and array positions
//! appear as 0-based decimal segments: `valueCodeableConcept.coding.0.code`.
//! FHIR JSON keys are element names and never all-digits, which keeps the
//! encoding unambiguous (enforced at shred time).

use serde_json::{Map, Value};

use crate::error::ShredError;

/// A typed leaf value. `Num` keeps the lexical form.
#[derive(Debug, Clone, PartialEq)]
pub enum LeafVal {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
}

impl LeafVal {
    /// (v_kind, v_text, v_num, v_bool) column images.
    pub fn cols(&self) -> (char, Option<&str>, Option<&str>, Option<bool>) {
        match self {
            LeafVal::Null => ('z', None, None, None),
            LeafVal::Bool(b) => ('b', None, None, Some(*b)),
            LeafVal::Num(s) => ('n', Some(s), Some(s), None),
            LeafVal::Str(s) => ('s', Some(s), None, None),
        }
    }

    pub fn from_cols(kind: &str, text: Option<&str>) -> Result<Self, ShredError> {
        match kind {
            "z" => Ok(LeafVal::Null),
            "b" => match text {
                Some("true") => Ok(LeafVal::Bool(true)),
                Some("false") => Ok(LeafVal::Bool(false)),
                _ => Err(ShredError::integrity("bad boolean leaf")),
            },
            "n" => text
                .map(|t| LeafVal::Num(t.to_string()))
                .ok_or_else(|| ShredError::integrity("numeric leaf without text")),
            "s" => text
                .map(|t| LeafVal::Str(t.to_string()))
                .ok_or_else(|| ShredError::integrity("string leaf without text")),
            other => Err(ShredError::integrity(format!(
                "unknown leaf kind {other:?}"
            ))),
        }
    }

    fn to_json(&self) -> Result<Value, ShredError> {
        Ok(match self {
            LeafVal::Null => Value::Null,
            LeafVal::Bool(b) => Value::Bool(*b),
            LeafVal::Num(s) => Value::Number(serde_json::Number::from_string_unchecked(s.clone())),
            LeafVal::Str(s) => Value::String(s.clone()),
        })
    }

    /// For boolean leaves the stored image lives in v_bool; reconstructing
    /// callers read it back as text "true"/"false" via SQL casts.
    fn from_json(v: &Value) -> Option<Self> {
        match v {
            Value::Null => Some(LeafVal::Null),
            Value::Bool(b) => Some(LeafVal::Bool(*b)),
            Value::Number(n) => Some(LeafVal::Num(n.to_string())),
            Value::String(s) => Some(LeafVal::Str(s.clone())),
            _ => None,
        }
    }
}

/// Decompose a JSON value into leaf rows. `skip_top_key` drops one top-level
/// object key (used to omit an extension's `url`, which is stored in its own
/// column).
pub fn flatten(
    v: &Value,
    at: &str,
    skip_top_key: Option<&str>,
) -> Result<Vec<(String, LeafVal)>, ShredError> {
    let mut out = Vec::new();
    flatten_into(v, at, String::new(), skip_top_key, &mut out)?;
    if out.is_empty() {
        return Err(ShredError::at(at, "empty value"));
    }
    Ok(out)
}

fn flatten_into(
    v: &Value,
    at: &str,
    prefix: String,
    skip_top_key: Option<&str>,
    out: &mut Vec<(String, LeafVal)>,
) -> Result<(), ShredError> {
    match v {
        Value::Object(m) => {
            if m.is_empty() {
                return Err(ShredError::at(at, "empty object"));
            }
            for (k, val) in m {
                if prefix.is_empty() && skip_top_key == Some(k.as_str()) {
                    continue;
                }
                if k.chars().all(|c| c.is_ascii_digit()) {
                    return Err(ShredError::at(at, format!("all-digit object key {k:?}")));
                }
                if k.contains('.') {
                    return Err(ShredError::at(at, format!("dotted object key {k:?}")));
                }
                let p = join(&prefix, k);
                flatten_into(val, at, p, None, out)?;
            }
            Ok(())
        }
        Value::Array(a) => {
            if a.is_empty() {
                return Err(ShredError::at(at, "empty array"));
            }
            for (i, item) in a.iter().enumerate() {
                let p = join(&prefix, &i.to_string());
                flatten_into(item, at, p, None, out)?;
            }
            Ok(())
        }
        scalar => {
            let leaf = LeafVal::from_json(scalar).expect("scalar");
            out.push((prefix, leaf));
            Ok(())
        }
    }
}

fn join(prefix: &str, seg: &str) -> String {
    if prefix.is_empty() {
        seg.to_string()
    } else {
        format!("{prefix}.{seg}")
    }
}

/// Rebuild a JSON value from leaf rows produced by [`flatten`].
pub fn unflatten(leaves: &[(String, LeafVal)]) -> Result<Value, ShredError> {
    if leaves.is_empty() {
        return Err(ShredError::integrity("no leaves to unflatten"));
    }
    if leaves.len() == 1 && leaves[0].0.is_empty() {
        return leaves[0].1.to_json();
    }
    let mut root = Value::Object(Map::new());
    for (path, leaf) in leaves {
        if path.is_empty() {
            return Err(ShredError::integrity("mixed root and nested leaves"));
        }
        insert_at(&mut root, path, leaf.to_json()?)?;
    }
    Ok(root)
}

fn insert_at(root: &mut Value, path: &str, val: Value) -> Result<(), ShredError> {
    let segs: Vec<&str> = path.split('.').collect();
    let mut cur = root;
    for (i, seg) in segs.iter().enumerate() {
        let last = i + 1 == segs.len();
        let is_index = seg.chars().all(|c| c.is_ascii_digit());
        if is_index {
            let idx: usize = seg
                .parse()
                .map_err(|_| ShredError::integrity("bad leaf index"))?;
            if !cur.is_array() {
                match cur {
                    Value::Object(m) if m.is_empty() => *cur = Value::Array(Vec::new()),
                    _ => {
                        return Err(ShredError::integrity(format!(
                            "leaf path {path:?} mixes object and array"
                        )));
                    }
                }
            }
            let arr = cur.as_array_mut().expect("array");
            while arr.len() <= idx {
                arr.push(Value::Object(Map::new()));
            }
            if last {
                arr[idx] = val;
                return Ok(());
            }
            cur = &mut arr[idx];
        } else {
            if !cur.is_object() {
                return Err(ShredError::integrity(format!(
                    "leaf path {path:?} mixes object and array"
                )));
            }
            let map = cur.as_object_mut().expect("object");
            if last {
                map.insert(seg.to_string(), val);
                return Ok(());
            }
            cur = map
                .entry(seg.to_string())
                .or_insert_with(|| Value::Object(Map::new()));
        }
    }
    Ok(())
}

/// Derived sort value for a FHIR date: the first day of the stated period.
/// Returns None (and the caller stores no sort value) if the lexical form is
/// not a valid FHIR date.
pub fn date_sort(s: &str) -> Option<String> {
    let b = s.as_bytes();
    match b.len() {
        4 if all_digits(&b[0..4]) => Some(format!("{s}-01-01")),
        7 if all_digits(&b[0..4]) && b[4] == b'-' && all_digits(&b[5..7]) => {
            Some(format!("{s}-01"))
        }
        10 if all_digits(&b[0..4])
            && b[4] == b'-'
            && all_digits(&b[5..7])
            && b[7] == b'-'
            && all_digits(&b[8..10]) =>
        {
            Some(s.to_string())
        }
        _ => None,
    }
}

/// Derived sort value for a FHIR dateTime/instant: the starting instant,
/// UTC-assumed when no offset is present.
pub fn datetime_sort(s: &str) -> Option<String> {
    if s.len() <= 10 {
        return date_sort(s).map(|d| format!("{d}T00:00:00Z"));
    }
    // Full dateTime: YYYY-MM-DDThh:mm:ss(.sss)?(Z|±hh:mm)?
    let (date, time) = s.split_once('T')?;
    date_sort(date)?;
    let has_offset = time.ends_with('Z') || time.rfind(['+', '-']).is_some_and(|i| i >= 5);
    if has_offset {
        Some(s.to_string())
    } else {
        Some(format!("{s}Z"))
    }
}

fn all_digits(b: &[u8]) -> bool {
    b.iter().all(|c| c.is_ascii_digit())
}

/// A parsed FHIR reference string.
pub enum ParsedRef {
    /// "Type/id" with a known-looking type segment.
    Relative { rtype: String, rid: String },
    /// Anything else (absolute URL, urn:, "#fragment", version-specific).
    Other(String),
}

pub fn parse_reference(s: &str) -> ParsedRef {
    if !s.contains("://") && !s.starts_with('#') && !s.starts_with("urn:") {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2
            && parts[0]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_uppercase())
            && !parts[0].is_empty()
            && !parts[1].is_empty()
        {
            return ParsedRef::Relative {
                rtype: parts[0].to_string(),
                rid: parts[1].to_string(),
            };
        }
    }
    ParsedRef::Other(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn flatten_roundtrip() {
        let v = json!({
            "coding": [
                {"system": "http://loinc.org", "code": "1234-5"},
                {"system": "http://snomed.info/sct", "code": "271649006", "userSelected": true}
            ],
            "text": "Systolic"
        });
        let leaves = flatten(&v, "x", None).unwrap();
        let back = unflatten(&leaves).unwrap();
        assert_eq!(v, back);
    }

    #[test]
    fn flatten_scalar_and_nulls() {
        let v: Value = serde_json::from_str(r#"{"a": [null, {"b": 1.50}]}"#).unwrap();
        let leaves = flatten(&v, "x", None).unwrap();
        let back = unflatten(&leaves).unwrap();
        assert_eq!(back["a"][0], Value::Null);
        assert_eq!(back["a"][1]["b"].to_string(), "1.50");
    }

    #[test]
    fn date_sorts() {
        assert_eq!(date_sort("2026").as_deref(), Some("2026-01-01"));
        assert_eq!(date_sort("2026-07").as_deref(), Some("2026-07-01"));
        assert_eq!(date_sort("2026-07-23").as_deref(), Some("2026-07-23"));
        assert_eq!(date_sort("julio"), None);
        assert_eq!(
            datetime_sort("2026-07").as_deref(),
            Some("2026-07-01T00:00:00Z")
        );
        assert_eq!(
            datetime_sort("2026-07-23T10:00:00").as_deref(),
            Some("2026-07-23T10:00:00Z")
        );
        assert_eq!(
            datetime_sort("2026-07-23T10:00:00+02:00").as_deref(),
            Some("2026-07-23T10:00:00+02:00")
        );
    }

    #[test]
    fn reference_parse() {
        match parse_reference("Patient/123") {
            ParsedRef::Relative { rtype, rid } => {
                assert_eq!(rtype, "Patient");
                assert_eq!(rid, "123");
            }
            _ => panic!(),
        }
        assert!(matches!(
            parse_reference("http://x/Patient/1"),
            ParsedRef::Other(_)
        ));
        assert!(matches!(parse_reference("#p1"), ParsedRef::Other(_)));
        assert!(matches!(
            parse_reference("Patient/123/_history/2"),
            ParsedRef::Other(_)
        ));
    }
}
