//! FHIR specification-package parsing: profiles-resources.json and
//! profiles-types.json → element definitions the builder can walk.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use serde_json::Value;

use crate::GenError;

#[derive(Debug)]
pub struct Spec {
    pub fhir_version: String,
    /// Concrete resource definitions, by name.
    pub resources: BTreeMap<String, Def>,
    /// Concrete complex datatype definitions, by name.
    pub types: HashMap<String, Def>,
    /// Primitive type names (plus xhtml).
    pub primitives: HashSet<String>,
}

/// One StructureDefinition's snapshot, indexed for child lookup.
#[derive(Debug)]
pub struct Def {
    pub name: String,
    pub elems: Vec<SpecElem>,
    /// Parent path → indexes into `elems` of its direct children,
    /// in snapshot order.
    pub children: HashMap<String, Vec<usize>>,
}

#[derive(Debug)]
pub struct SpecElem {
    /// Full dotted definition path ("Patient.contact.name"); choice paths
    /// keep the "[x]" suffix.
    pub path: String,
    /// Last path segment, "[x]" stripped.
    pub name: String,
    pub choice: bool,
    pub repeats: bool,
    /// max = "0": element removed.
    pub omitted: bool,
    /// Type codes, deduplicated, in order.
    pub types: Vec<String>,
    /// "#Path" targets with the '#' stripped.
    pub content_ref: Option<String>,
}

impl Def {
    pub fn kids(&self, path: &str) -> &[usize] {
        self.children.get(path).map(Vec::as_slice).unwrap_or(&[])
    }
}

pub fn load_spec(dir: &Path) -> Result<Spec, GenError> {
    let types_file = dir.join("profiles-types.json");
    let resources_file = dir.join("profiles-resources.json");
    let tv = read_json(&types_file)?;
    let rv = read_json(&resources_file)?;

    let mut primitives: HashSet<String> = HashSet::new();
    primitives.insert("xhtml".to_string());
    let mut types = HashMap::new();
    let mut fhir_version = String::new();

    for res in bundle_resources(&tv) {
        if res.get("resourceType").and_then(Value::as_str) != Some("StructureDefinition") {
            continue;
        }
        let name = res.get("name").and_then(Value::as_str).unwrap_or_default();
        let kind = res.get("kind").and_then(Value::as_str).unwrap_or_default();
        let abstract_ = res
            .get("abstract")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let derivation = res.get("derivation").and_then(Value::as_str).unwrap_or("");
        if kind == "primitive-type" {
            primitives.insert(name.to_string());
            continue;
        }
        if kind != "complex-type" || abstract_ || derivation == "constraint" {
            continue;
        }
        types.insert(name.to_string(), parse_def(name, res)?);
    }

    let mut resources = BTreeMap::new();
    for res in bundle_resources(&rv) {
        if res.get("resourceType").and_then(Value::as_str) != Some("StructureDefinition") {
            continue;
        }
        let name = res.get("name").and_then(Value::as_str).unwrap_or_default();
        let kind = res.get("kind").and_then(Value::as_str).unwrap_or_default();
        let abstract_ = res
            .get("abstract")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let derivation = res.get("derivation").and_then(Value::as_str).unwrap_or("");
        if kind != "resource" || abstract_ || derivation == "constraint" {
            continue;
        }
        if fhir_version.is_empty() {
            fhir_version = res
                .get("fhirVersion")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
        }
        resources.insert(name.to_string(), parse_def(name, res)?);
    }

    if resources.is_empty() {
        return Err(GenError::Spec("no resource definitions found".into()));
    }
    Ok(Spec {
        fhir_version,
        resources,
        types,
        primitives,
    })
}

fn read_json(path: &Path) -> Result<Value, GenError> {
    let bytes =
        std::fs::read(path).map_err(|e| GenError::Spec(format!("{}: {e}", path.display())))?;
    serde_json::from_slice(&bytes).map_err(|e| GenError::Spec(format!("{}: {e}", path.display())))
}

fn bundle_resources(bundle: &Value) -> impl Iterator<Item = &Value> {
    bundle
        .get("entry")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|e| e.get("resource"))
}

fn parse_def(name: &str, sd: &Value) -> Result<Def, GenError> {
    let snapshot = sd
        .pointer("/snapshot/element")
        .and_then(Value::as_array)
        .ok_or_else(|| GenError::Spec(format!("{name}: no snapshot")))?;
    let mut elems = Vec::with_capacity(snapshot.len());
    for el in snapshot.iter().skip(1) {
        let path = el
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| GenError::Spec(format!("{name}: element without path")))?
            .to_string();
        let max = el.get("max").and_then(Value::as_str).unwrap_or("1");
        let seg = path.rsplit('.').next().unwrap_or(&path);
        let choice = seg.ends_with("[x]");
        let ename = seg.trim_end_matches("[x]").to_string();
        let mut types = Vec::new();
        if let Some(ts) = el.get("type").and_then(Value::as_array) {
            for t in ts {
                let code = type_code(t);
                if let Some(code) = code
                    && !types.contains(&code)
                {
                    types.push(code);
                }
            }
        }
        let content_ref = el
            .get("contentReference")
            .and_then(Value::as_str)
            .map(|s| s.trim_start_matches('#').to_string());
        elems.push(SpecElem {
            path,
            name: ename,
            choice,
            repeats: max != "0" && max != "1",
            omitted: max == "0",
            types,
            content_ref,
        });
    }
    let mut children: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, e) in elems.iter().enumerate() {
        if let Some(dot) = e.path.rfind('.') {
            children
                .entry(e.path[..dot].to_string())
                .or_default()
                .push(i);
        }
    }
    Ok(Def {
        name: name.to_string(),
        elems,
        children,
    })
}

/// Extract the effective type code of one `element.type` entry. FHIRPath
/// system types (R4+) carry the FHIR type in an extension; R3 uses plain
/// codes throughout.
fn type_code(t: &Value) -> Option<String> {
    let code = t.get("code").and_then(Value::as_str)?;
    if let Some(sys) = code.strip_prefix("http://hl7.org/fhirpath/System.") {
        if let Some(exts) = t.get("extension").and_then(Value::as_array) {
            for e in exts {
                if e.get("url").and_then(Value::as_str)
                    == Some("http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type")
                    && let Some(v) = e
                        .get("valueUrl")
                        .or_else(|| e.get("valueUri"))
                        .and_then(Value::as_str)
                {
                    return Some(v.to_string());
                }
            }
        }
        // Fall back to the System type's obvious FHIR analogue.
        return Some(
            match sys {
                "String" => "string",
                "Boolean" => "boolean",
                "Integer" => "integer",
                "Decimal" => "decimal",
                "Date" => "date",
                "DateTime" => "dateTime",
                "Time" => "time",
                other => other,
            }
            .to_string(),
        );
    }
    Some(code.to_string())
}
