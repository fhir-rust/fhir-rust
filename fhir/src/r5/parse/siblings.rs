//! Generator (T7): splice `_field` primitive-extension siblings into the
//! existing, hand-documented `src/r5/{types,resources}` source files.
//!
//! The committed model files are generated skeletons that have since been
//! heavily hand-enriched (prose docs, `Validate` derive, …), so this tool does
//! **not** regenerate them. Instead it computes, from the [`crate::r5::meta`]
//! table, exactly which primitive fields need a sibling and splices the new
//! field in immediately after each value field — leaving everything else byte
//! for byte intact.
//!
//! Representation follows `spec/09-primitive-extensions.md` (Approach A):
//! a scalar primitive `x` gets `x_ext: Option<Element>`; a repeating primitive
//! gets `x_ext: Option<Vec<Option<Element>>>`; both renamed to the FHIR `_x`
//! key. The splice is idempotent (a struct that already has the `_x` rename is
//! left alone) and choice `value[x]` primitives are intentionally skipped for
//! now (their Rust fields, e.g. `deceased_boolean`, have no direct `meta` path;
//! they are reported so a later pass can handle them).
//!
//! Drive it with the ignored tests below:
//!
//! ```sh
//! cargo test r5::parse::siblings::tests::report_types  -- --ignored --nocapture
//! cargo test r5::parse::siblings::tests::apply_types    -- --ignored --nocapture
//! ```

use std::collections::HashMap;
use std::path::PathBuf;

use ::convert_case::{Case, Casing};

use crate::r5::meta;

/// FHIR primitive type codes. An element whose single type is one of these is a
/// primitive and so may carry a `_field` extension sibling.
const PRIMITIVES: &[&str] = &[
    "base64Binary",
    "boolean",
    "canonical",
    "code",
    "date",
    "dateTime",
    "decimal",
    "id",
    "instant",
    "integer",
    "integer64",
    "markdown",
    "oid",
    "positiveInt",
    "string",
    "time",
    "unsignedInt",
    "uri",
    "url",
    "uuid",
    "xhtml",
];

/// One planned sibling insertion.
#[derive(Debug, Clone)]
pub struct Insertion {
    /// Rust struct the field lives in.
    pub struct_name: String,
    /// FHIR element path, e.g. `"HumanName.family"`.
    pub path: String,
    /// Existing Rust value field name, e.g. `"family"`.
    pub field: String,
    /// New sibling Rust field name, e.g. `"family_ext"`.
    pub sibling: String,
    /// FHIR `_` key, e.g. `"_family"`.
    pub json_key: String,
    /// Whether the primitive repeats (`Vec<Option<Element>>`) or is scalar.
    pub repeating: bool,
    /// 0-based index of the value field's declaration line to insert after.
    pub after_line: usize,
}

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Map every backbone/root struct name to its FHIR path prefix, derived from the
/// metadata table. For a path `A.b.c`, both `A` (prefix `A`) and its parent
/// struct `A.b` (struct `AB`, prefix `A.b`) are registered.
fn struct_path_prefixes() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for el in meta::elements() {
        let segments: Vec<&str> = el.path.split('.').collect();
        // Every proper prefix of length >= 1 is a struct that owns the next
        // segment as a field.
        for take in 1..segments.len() {
            let prefix_segments = &segments[..take];
            let struct_name: String = prefix_segments
                .iter()
                .map(|s| s.to_case(Case::Pascal))
                .collect();
            let path_prefix = prefix_segments.join(".");
            map.entry(struct_name).or_insert(path_prefix);
        }
    }
    map
}

fn is_primitive(el: &meta::ElementMeta) -> bool {
    el.types.len() == 1 && PRIMITIVES.contains(&el.types[0].code)
}

/// A field's value line, parsed from generated source: `    pub <name>: <ty>,`.
/// Returns the identifier as written, which may be a raw identifier such as
/// `r#type` (FHIR fields named after Rust keywords).
fn parse_field_name(line: &str) -> Option<&str> {
    let t = line.trim_start();
    let rest = t.strip_prefix("pub ")?;
    let colon = rest.find(':')?;
    let name = rest[..colon].trim();
    // Validate the bare identifier (after any `r#` raw-identifier prefix).
    let bare = name.strip_prefix("r#").unwrap_or(name);
    if bare.is_empty() || !bare.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some(name)
}

/// Find `pub struct NAME {` lines and the brace-matched end line for each.
fn struct_spans(lines: &[&str]) -> Vec<(String, usize, usize)> {
    let mut spans = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim_start();
        if let Some(rest) = t.strip_prefix("pub struct ") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            // Track braces from this line to the matching close.
            let mut depth = 0i32;
            let mut j = i;
            let mut started = false;
            while j < lines.len() {
                for c in lines[j].chars() {
                    if c == '{' {
                        depth += 1;
                        started = true;
                    } else if c == '}' {
                        depth -= 1;
                    }
                }
                if started && depth == 0 {
                    break;
                }
                j += 1;
            }
            spans.push((name, i, j));
            i = j + 1;
        } else {
            i += 1;
        }
    }
    spans
}

/// Compute the sibling insertions for one already-loaded source file.
///
/// `skipped` collects `(struct, field)` pairs that look like fields but were not
/// primitives or had no metadata path (e.g. choice `value[x]` variants), for
/// reporting.
#[must_use]
pub fn plan_file(text: &str, skipped: &mut Vec<(String, String)>) -> Vec<Insertion> {
    let lines: Vec<&str> = text.lines().collect();
    let prefixes = struct_path_prefixes();
    let mut insertions = Vec::new();

    for (struct_name, start, end) in struct_spans(&lines) {
        let Some(prefix) = prefixes.get(&struct_name) else {
            continue; // struct we can't map to a FHIR path; leave it alone
        };
        // Collect existing `_x` renames in this struct to stay idempotent.
        let mut existing_siblings = std::collections::HashSet::new();
        for line in &lines[start..=end] {
            if let Some(idx) = line.find("rename = \"_") {
                let rest = &line[idx + "rename = \"".len()..];
                if let Some(q) = rest.find('"') {
                    existing_siblings.insert(rest[..q].to_string());
                }
            }
        }

        for (offset, line) in lines[start..=end].iter().enumerate() {
            let line_no = start + offset;
            let Some(field) = parse_field_name(line) else {
                continue;
            };
            // Bare identifier without any `r#` raw prefix (e.g. `type`).
            let bare = field.strip_prefix("r#").unwrap_or(field);
            if bare == "id" {
                continue; // Element.id / Resource.id do not take `_id`
            }
            let json_name = bare.to_case(Case::Camel);
            let path = format!("{prefix}.{json_name}");
            let Some(el) = meta::element(&path) else {
                // No metadata path: choice variant or non-spec field.
                skipped.push((struct_name.clone(), field.to_string()));
                continue;
            };
            if !is_primitive(el) {
                continue;
            }
            let json_key = format!("_{json_name}");
            if existing_siblings.contains(&json_key) {
                continue; // already spliced
            }
            insertions.push(Insertion {
                struct_name: struct_name.clone(),
                path,
                field: field.to_string(), // raw ident, for the Self:: doc link
                sibling: format!("{bare}_ext"),
                json_key,
                repeating: el.is_multiple(),
                after_line: line_no,
            });
        }
    }
    insertions
}

/// Render the Rust source for one sibling field (with doc comment + rename).
fn render_sibling(ins: &Insertion) -> String {
    let ty = if ins.repeating {
        "Option<Vec<Option<types::Element>>>"
    } else {
        "Option<types::Element>"
    };
    let display = ins.field.strip_prefix("r#").unwrap_or(&ins.field);
    format!(
        "\n    /// Primitive extension sibling for [`{display}`](Self::{field}) \
         (FHIR `{json_key}`).\n    #[serde(rename = \"{json_key}\")]\n    pub {sibling}: {ty},",
        field = ins.field,
        json_key = ins.json_key,
        sibling = ins.sibling,
    )
}

/// Apply insertions to file text and return the new text. Insertions are
/// applied bottom-up so earlier line indices stay valid.
#[must_use]
pub fn apply_insertions(text: &str, mut insertions: Vec<Insertion>) -> String {
    let mut lines: Vec<String> = text.lines().map(String::from).collect();
    insertions.sort_by_key(|i| std::cmp::Reverse(i.after_line));
    for ins in insertions {
        lines[ins.after_line].push_str(&render_sibling(&ins));
    }
    let mut out = lines.join("\n");
    if text.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// Make the generated `test_default` blocks robust to added fields by inserting
/// `..Default::default()` into their exhaustive `let expect = T { … };` literal.
///
/// These blocks list every field as `None`; adding a sibling field would break
/// the exhaustive literal (E0063). The struct-rest keeps the existing per-field
/// assertions while tolerating new fields. Idempotent.
#[must_use]
pub fn patch_default_tests<S: std::hash::BuildHasher>(
    text: &str,
    changed_structs: &std::collections::HashSet<String, S>,
) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    // Track the current `type T = X;` alias so a literal is only patched when
    // its struct actually gained a field (otherwise `..Default::default()` is a
    // needless struct update).
    let mut current_t: Option<String> = None;
    let mut i = 0;
    while i < lines.len() {
        out.push(lines[i].to_string());
        if let Some(rest) = lines[i].trim_start().strip_prefix("type T = ") {
            current_t = Some(rest.trim_end_matches(';').trim().to_string());
        }
        let t_changed = current_t
            .as_ref()
            .is_some_and(|t| changed_structs.contains(t));
        if t_changed && lines[i].trim_start().starts_with("let expect = T {") {
            // Find the closing `};` of this literal.
            let mut j = i + 1;
            let mut already = false;
            while j < lines.len() && lines[j].trim() != "};" {
                if lines[j].contains("..Default::default()") {
                    already = true;
                }
                j += 1;
            }
            if j < lines.len() && !already {
                for line in &lines[i + 1..j] {
                    out.push((*line).to_string());
                }
                out.push("            ..Default::default()".to_string());
                out.push(lines[j].to_string());
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    let mut joined = out.join("\n");
    if text.ends_with('\n') {
        joined.push('\n');
    }
    joined
}

/// Add the FHIR `Element` base (`id` + `extension`) to the root complex-datatype
/// struct in every `types` file that lacks it. Complex datatypes derive from
/// `Element` and so may carry an `id` and `extension`s, but the generated
/// structs omitted them (round-trip category B). Primitives (tuple structs),
/// the empty `Base`, and any struct that already models `extension` are skipped.
/// Returns the number of structs changed.
pub fn apply_element_base_group() -> usize {
    let element_base = "\n    /// Unique id for inter-element referencing\n    \
        pub id: Option<types::String>,\n\n    \
        /// Additional content defined by implementations\n    \
        pub extension: Option<Vec<types::Extension>>,";
    let mut total = 0;
    for file in group_files("types") {
        let text = std::fs::read_to_string(&file).expect("read file");
        let lines: Vec<&str> = text.lines().collect();
        // Only the first (root) braced struct of the datatype file.
        let Some((name, start, end)) = struct_spans(&lines).into_iter().next() else {
            continue;
        };
        let span = &lines[start..=end];
        // Skip tuple/unit structs (primitives are `pub struct X(pub …);`), empty
        // structs (`pub struct Base {}`), and any that already model `extension`.
        let is_braced = lines[start].contains('{');
        let has_extension = span
            .iter()
            .any(|l| l.trim_start().starts_with("pub extension:"));
        let is_empty = lines[start].contains("{}");
        if !is_braced || has_extension || is_empty {
            continue;
        }
        let mut new_lines: Vec<String> = lines.iter().map(|s| (*s).to_string()).collect();
        new_lines[start].push_str(element_base);
        let spliced = new_lines.join("\n");
        let with_ln = if text.ends_with('\n') {
            format!("{spliced}\n")
        } else {
            spliced
        };
        let changed: std::collections::HashSet<String> = std::iter::once(name).collect();
        let patched = patch_default_tests(&with_ln, &changed);
        if patched != text {
            std::fs::write(&file, patched).expect("write file");
            total += 1;
        }
    }
    total
}

/// List the source files for a group: `types` or `resources`.
fn group_files(group: &str) -> Vec<PathBuf> {
    let dir = crate_root().join("fhir-r5").join("src").join(group);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("read group dir")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    files.sort();
    files
}

/// Dry-run: return the full insertion plan and the skipped fields for a group.
#[must_use]
pub fn plan_group(group: &str) -> (Vec<Insertion>, Vec<(String, String)>) {
    let mut all = Vec::new();
    let mut skipped = Vec::new();
    for file in group_files(group) {
        let text = std::fs::read_to_string(&file).expect("read file");
        all.extend(plan_file(&text, &mut skipped));
    }
    (all, skipped)
}

/// Apply the sibling splice to every file in a group, writing changes in place.
/// Also patches the generated `test_default` literals. Idempotent: only files
/// whose contents actually change are rewritten.
pub fn apply_group(group: &str) -> usize {
    let mut total = 0;
    for file in group_files(group) {
        let text = std::fs::read_to_string(&file).expect("read file");
        let mut skipped = Vec::new();
        let insertions = plan_file(&text, &mut skipped);
        total += insertions.len();
        let changed_structs: std::collections::HashSet<String> =
            insertions.iter().map(|i| i.struct_name.clone()).collect();
        let spliced = apply_insertions(&text, insertions);
        let new_text = patch_default_tests(&spliced, &changed_structs);
        if new_text != text {
            std::fs::write(&file, new_text).expect("write file");
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_field_names() {
        assert_eq!(
            parse_field_name("    pub family: Option<types::String>,"),
            Some("family")
        );
        assert_eq!(
            parse_field_name("    pub given: Vec<types::String>, // « C »"),
            Some("given")
        );
        assert_eq!(
            parse_field_name("    pub r#type: types::Code,"),
            Some("r#type")
        );
        assert_eq!(parse_field_name("pub struct HumanName {"), None);
        assert_eq!(parse_field_name("    /// a doc comment"), None);
    }

    #[test]
    fn struct_prefix_map_has_known_entries() {
        let map = struct_path_prefixes();
        assert_eq!(map.get("HumanName").map(String::as_str), Some("HumanName"));
        assert_eq!(
            map.get("PatientContact").map(String::as_str),
            Some("Patient.contact")
        );
    }

    /// Dry run for datatypes; prints the plan for review. Ignored.
    #[test]
    #[ignore = "dry-run report only"]
    fn report_types() {
        let (plan, skipped) = plan_group("types");
        println!("types: {} sibling insertions planned", plan.len());
        for ins in &plan {
            println!(
                "  {} :: {} -> {} ({})",
                ins.struct_name,
                ins.field,
                ins.json_key,
                if ins.repeating { "repeating" } else { "scalar" }
            );
        }
        let mut choice_like: Vec<_> = skipped
            .iter()
            .filter(|(_, f)| f.contains('_') && f.rsplit('_').next().is_some())
            .collect();
        choice_like.sort();
        println!(
            "skipped (no meta path; incl. choice value[x]): {}",
            skipped.len()
        );
    }

    /// Apply the datatype splice. Ignored (writes source files).
    #[test]
    #[ignore = "writes fhir-r5/src/types/*.rs"]
    fn apply_types() {
        let n = apply_group("types");
        println!("types: applied {n} sibling insertions");
    }

    /// Apply the resource splice. Ignored (writes source files).
    #[test]
    #[ignore = "writes fhir-r5/src/resources/*.rs"]
    fn apply_resources() {
        let n = apply_group("resources");
        println!("resources: applied {n} sibling insertions");
    }

    /// Add id + extension to complex datatypes. Ignored (writes source files).
    #[test]
    #[ignore = "writes fhir-r5/src/types/*.rs"]
    fn apply_element_base() {
        let n = apply_element_base_group();
        println!("types: added Element base (id + extension) to {n} datatypes");
    }
}
