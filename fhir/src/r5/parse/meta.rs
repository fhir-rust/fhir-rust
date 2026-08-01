//! Generator: extract per-element metadata from the FHIR R5 specification.
//!
//! Reads the `profiles-resources.json` and `profiles-types.json` definition
//! bundles, walks every `StructureDefinition`'s snapshot elements, and records
//! per-element facts (cardinality, coded-value binding, allowed `value[x]`
//! types, reference target profiles, summary membership). It emits two
//! artifacts:
//!
//! - `tmp/out/meta.json` — the raw table, for inspection and downstream tools.
//! - `src/r5/meta/generated.rs` — the compiled lookup consumed by
//!   [`crate::r5::meta`].
//!
//! Run it via `cargo run`, or the ignored `regenerate` test below:
//!
//! ```sh
//! cargo test -p fhir r5::parse::meta::tests::regenerate -- --ignored
//! ```
//!
//! The extraction uses its own permissive structs (no `deny_unknown_fields`) so
//! it tolerates the whole bundle, which mixes `StructureDefinition` with
//! `OperationDefinition`, `SearchParameter`, and other resource types.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use ::serde::{Deserialize, Serialize};

// ---- Input: permissive views of the specification JSON ----------------------

#[derive(Deserialize)]
struct Bundle {
    entry: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    resource: ::serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructureDefinition {
    #[serde(default, rename = "type")]
    type_name: String,
    snapshot: Option<Snapshot>,
}

#[derive(Deserialize)]
struct Snapshot {
    #[serde(default)]
    element: Vec<ElementDef>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementDef {
    path: String,
    min: Option<u32>,
    max: Option<String>,
    is_summary: Option<bool>,
    binding: Option<BindingDef>,
    #[serde(default, rename = "type")]
    types: Vec<ElementTypeDef>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BindingDef {
    strength: String,
    value_set: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementTypeDef {
    code: String,
    #[serde(default)]
    target_profile: Vec<String>,
}

// ---- Output: the metadata table --------------------------------------------

/// Serializable per-element metadata (mirrors [`crate::r5::meta::ElementMeta`]).
#[derive(Serialize)]
pub struct ElementMetaOut {
    pub path: String,
    pub min: u32,
    pub max: String,
    pub is_summary: bool,
    pub binding: Option<BindingOut>,
    pub types: Vec<TypeRefOut>,
}

/// Serializable binding (mirrors [`crate::r5::meta::BindingMeta`]).
#[derive(Serialize)]
pub struct BindingOut {
    pub strength: String,
    pub value_set: Option<String>,
}

/// Serializable type reference (mirrors [`crate::r5::meta::TypeRef`]).
#[derive(Serialize)]
pub struct TypeRefOut {
    pub code: String,
    pub target_profiles: Vec<String>,
}

fn definitions_dir() -> PathBuf {
    crate::DEFINITIONS_DIR.clone()
}

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Extract every element's metadata, keyed and sorted by full FHIR path.
///
/// The first occurrence of a path wins (resources are processed before
/// datatypes), and the root element (whose path equals the type name) is
/// skipped because it is the type itself, not a field.
#[must_use]
pub fn extract() -> BTreeMap<String, ElementMetaOut> {
    let mut out: BTreeMap<String, ElementMetaOut> = BTreeMap::new();
    for file in ["profiles-resources.json", "profiles-types.json"] {
        let path = definitions_dir().join(file);
        let reader = BufReader::new(File::open(&path).expect("open definitions file"));
        let bundle: Bundle = ::serde_json::from_reader(reader).expect("parse definitions bundle");
        for entry in bundle.entry {
            if entry
                .resource
                .get("resourceType")
                .and_then(::serde_json::Value::as_str)
                != Some("StructureDefinition")
            {
                continue;
            }
            let sd: StructureDefinition =
                ::serde_json::from_value(entry.resource).expect("parse StructureDefinition");
            let Some(snapshot) = sd.snapshot else {
                continue;
            };
            for el in snapshot.element {
                if el.path == sd.type_name {
                    continue;
                }
                out.entry(el.path.clone())
                    .or_insert_with(|| ElementMetaOut {
                        path: el.path,
                        min: el.min.unwrap_or(0),
                        max: el.max.unwrap_or_else(|| "*".to_string()),
                        is_summary: el.is_summary.unwrap_or(false),
                        binding: el.binding.map(|b| BindingOut {
                            strength: b.strength,
                            value_set: b.value_set,
                        }),
                        types: el
                            .types
                            .into_iter()
                            .map(|t| TypeRefOut {
                                code: t.code,
                                target_profiles: t.target_profile,
                            })
                            .collect(),
                    });
            }
        }
    }
    out
}

fn strength_variant(strength: &str) -> &'static str {
    match strength {
        "required" => "Required",
        "extensible" => "Extensible",
        "preferred" => "Preferred",
        _ => "Example",
    }
}

/// Render the extracted table as the body of `src/r5/meta/generated.rs`.
#[must_use]
pub fn render_generated_rs(table: &BTreeMap<String, ElementMetaOut>) -> String {
    use std::fmt::Write as _;

    let mut s = String::new();
    s.push_str(
        "//! Generated element-metadata table — DO NOT EDIT.\n\
         //!\n\
         //! Produced from the FHIR R5 specification JSON by `crate::r5::parse::meta`.\n\
         //! Regenerate with `cargo run` (or the `regenerate` test in that module).\n\n\
         use super::{BindingMeta, BindingStrength, ElementMeta, TypeRef};\n\n\
         pub(super) static ELEMENTS: &[ElementMeta] = &[\n",
    );
    for e in table.values() {
        write!(
            s,
            "    ElementMeta {{ path: {:?}, min: {}, max: {:?}, is_summary: {}, binding: ",
            e.path, e.min, e.max, e.is_summary
        )
        .unwrap();
        match &e.binding {
            None => s.push_str("None"),
            Some(b) => {
                write!(
                    s,
                    "Some(BindingMeta {{ strength: BindingStrength::{}, value_set: ",
                    strength_variant(&b.strength)
                )
                .unwrap();
                match &b.value_set {
                    None => s.push_str("None"),
                    Some(vs) => write!(s, "Some({vs:?})").unwrap(),
                }
                s.push_str(" })");
            }
        }
        s.push_str(", types: &[");
        for (i, t) in e.types.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            write!(s, "TypeRef {{ code: {:?}, target_profiles: &[", t.code).unwrap();
            for (j, tp) in t.target_profiles.iter().enumerate() {
                if j > 0 {
                    s.push_str(", ");
                }
                write!(s, "{tp:?}").unwrap();
            }
            s.push_str("] }");
        }
        s.push_str("] },\n");
    }
    s.push_str("];\n");
    s
}

/// Extract the metadata and write both artifacts:
/// `tmp/out/meta.json` and `src/r5/meta/generated.rs`.
pub fn generate() {
    let table = extract();

    // Raw JSON table.
    let out_dir = crate_root().join("tmp").join("out");
    std::fs::create_dir_all(&out_dir).expect("create tmp/out");
    let values: Vec<&ElementMetaOut> = table.values().collect();
    let json = ::serde_json::to_string_pretty(&values).expect("serialize meta");
    let mut json_file = File::create(out_dir.join("meta.json")).expect("create meta.json");
    json_file
        .write_all(json.as_bytes())
        .expect("write meta.json");

    // Compiled lookup.
    let rs = render_generated_rs(&table);
    let rs_path = crate_root()
        .join("fhir-release-5")
        .join("src")
        .join("meta")
        .join("generated.rs");
    let mut rs_file = File::create(&rs_path).expect("create generated.rs");
    rs_file
        .write_all(rs.as_bytes())
        .expect("write generated.rs");

    eprintln!(
        "meta: wrote {} elements to {}",
        table.len(),
        rs_path.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_finds_known_elements() {
        let table = extract();
        assert!(table.len() > 5000, "got {}", table.len());
        let gender = table.get("Patient.gender").expect("Patient.gender");
        assert_eq!(gender.binding.as_ref().unwrap().strength, "required");
    }

    /// Regenerate the committed artifacts. Ignored so it does not run in the
    /// normal test gate (it writes into the source tree).
    #[test]
    #[ignore = "writes fhir-release-5/src/meta/generated.rs and tmp/out/meta.json"]
    fn regenerate() {
        generate();
    }
}
