//! Generator (T9c/d): convert flattened `value[x]` fields into `FhirChoice`
//! enums across the model.
//!
//! For each choice element `Struct.base[x]`, the model currently has one
//! `Option<T>` field per allowed type, named `<base>_<type>` (e.g.
//! `value_quantity`, `value_string`). This generator plans and applies the
//! replacement of those fields with a single `#[serde(flatten)]` field holding a
//! generated enum (see `spec/11-choice-types.md`). It does not regenerate the
//! hand-documented files; it splices, like `siblings`.
//!
//! This module currently produces the conversion **plan** and cross-checks it
//! against the source (a dry run); application is applied per group once the
//! plan is verified.

use std::collections::BTreeMap;
use std::path::PathBuf;

use ::convert_case::{Case, Casing};

use crate::r5::meta;

/// FHIR primitive type codes (a choice variant of one of these carries a paired
/// `_value<Type>` extension, so it uses `Primitive<T>`).
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

/// One variant of a choice enum.
#[derive(Debug, Clone)]
pub struct Variant {
    /// FHIR type code, e.g. `"Quantity"`, `"string"`.
    pub code: String,
    /// Enum variant identifier, e.g. `Quantity`, `String`, `DateTime`.
    pub variant: String,
    /// FHIR JSON key, e.g. `"valueQuantity"`.
    pub key: String,
    /// Existing flattened Rust field, e.g. `value_quantity`.
    pub old_field: String,
    /// Rust datatype path, e.g. `types::Quantity`.
    pub rust_type: String,
    /// Whether this is a primitive variant (uses `Primitive<T>`).
    pub primitive: bool,
}

/// A planned choice-element conversion.
#[derive(Debug, Clone)]
pub struct ChoiceConversion {
    /// FHIR element path, e.g. `"Observation.value[x]"`.
    pub path: String,
    /// Owning Rust struct, e.g. `Observation`.
    pub struct_name: String,
    /// New field name (snake of the base), e.g. `value`.
    pub field: String,
    /// Generated enum name, e.g. `ObservationValue`.
    pub enum_name: String,
    /// Variants, in the spec's type order.
    pub variants: Vec<Variant>,
}

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Snake-case a FHIR type code to match the model's field-naming convention.
/// `convert_case` splits digit runs (`base64Binary` -> `base_64_binary`), but
/// the generated fields keep digits attached (`base64_binary`, `integer64`).
fn type_snake(code: &str) -> String {
    match code {
        "base64Binary" => "base64_binary".to_string(),
        "integer64" => "integer64".to_string(),
        other => other.to_case(Case::Snake),
    }
}

/// Capitalize the first character (`string` -> `String`, `dateTime` -> `DateTime`).
fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Map struct name -> FHIR path prefix (root types + backbones), from meta.
fn struct_path_prefixes() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for el in meta::elements() {
        let segments: Vec<&str> = el.path.split('.').collect();
        for take in 1..segments.len() {
            let prefix = &segments[..take];
            let name: String = prefix.iter().map(|s| s.to_case(Case::Pascal)).collect();
            map.entry(name).or_insert_with(|| prefix.join("."));
        }
    }
    map
}

/// Build the full conversion plan from the metadata table.
#[must_use]
pub fn plan() -> Vec<ChoiceConversion> {
    // struct name for a given path prefix.
    let prefix_to_struct: BTreeMap<String, String> = struct_path_prefixes()
        .into_iter()
        .map(|(name, prefix)| (prefix, name))
        .collect();

    let mut plan = Vec::new();
    for el in meta::elements() {
        if !el.path.ends_with("[x]") {
            continue;
        }
        let base_camel = el.path.rsplit('.').next().unwrap().trim_end_matches("[x]");
        let parent_prefix = &el.path[..el.path.len() - base_camel.len() - "[x]".len() - 1];
        let Some(struct_name) = prefix_to_struct.get(parent_prefix) else {
            continue;
        };
        let field = base_camel.to_case(Case::Snake);
        let enum_name = format!("{struct_name}{}", base_camel.to_case(Case::Pascal));

        let variants = el
            .types
            .iter()
            .map(|t| {
                let variant = t.code.to_case(Case::Pascal);
                Variant {
                    key: format!("{base_camel}{}", cap_first(t.code)),
                    old_field: format!("{field}_{}", type_snake(t.code)),
                    rust_type: format!("types::{variant}"),
                    primitive: PRIMITIVES.contains(&t.code),
                    variant,
                    code: t.code.to_string(),
                }
            })
            .collect();

        plan.push(ChoiceConversion {
            path: el.path.to_string(),
            struct_name: struct_name.clone(),
            field,
            enum_name,
            variants,
        });
    }
    plan
}

/// Find `pub struct NAME {` spans (name, first line, brace-matched last line).
fn struct_spans(lines: &[&str]) -> Vec<(String, usize, usize)> {
    let mut spans = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if let Some(rest) = lines[i].trim_start().strip_prefix("pub struct ") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if !lines[i].contains('{') {
                i += 1; // tuple/unit struct
                continue;
            }
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

/// Render an enum definition for a conversion.
fn render_enum(c: &ChoiceConversion) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    let _ = writeln!(
        s,
        "\n/// The `{}` choice element (see spec/11-choice-types.md).",
        c.path
    );
    s.push_str(
        "#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]\n",
    );
    s.push_str("#[allow(clippy::large_enum_variant)]\n");
    let _ = writeln!(s, "pub enum {} {{", c.enum_name);
    for v in &c.variants {
        let _ = writeln!(s, "    /// `{}` variant.", v.key);
        let _ = writeln!(s, "    #[fhir(\"{}\")]", v.key);
        let inner = if v.primitive {
            format!("crate::r5::choice::Primitive<{}>", v.rust_type)
        } else {
            format!("Box<{}>", v.rust_type)
        };
        let _ = writeln!(s, "    {}({inner}),", v.variant);
    }
    s.push_str("}\n");
    s
}

/// Apply every conversion whose struct lives in `text`; return the new text and
/// the count of conversions applied. Conversions that cannot be applied safely
/// (fields not found in the struct, not contiguous, or already converted) are
/// skipped and counted in `skipped`.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn apply_file(
    text: &str,
    conversions: &[ChoiceConversion],
    skipped: &mut Vec<String>,
) -> (String, usize) {
    let lines: Vec<&str> = text.lines().collect();
    let spans = struct_spans(&lines);
    // (start_line, end_line_inclusive, replacement) edits within the struct.
    let mut edits: Vec<(usize, usize, String)> = Vec::new();
    let mut enums = String::new();
    let mut removed_fields: Vec<String> = Vec::new();
    let mut applied = 0;

    for c in conversions {
        // Already converted?
        if text.contains(&format!("pub enum {}", c.enum_name)) {
            continue;
        }
        let Some((_, s_start, s_end)) = spans.iter().find(|(n, ..)| n == &c.struct_name) else {
            skipped.push(format!("{}: struct not in file", c.path));
            continue;
        };
        // Locate each old field line within the struct.
        let mut field_lines = Vec::new();
        let mut all_found = true;
        for v in &c.variants {
            let needle = format!("pub {}:", v.old_field);
            if let Some(i) =
                (*s_start..=*s_end).find(|&i| lines[i].trim_start().starts_with(&needle))
            {
                field_lines.push(i);
            } else {
                all_found = false;
                break;
            }
        }
        if !all_found {
            skipped.push(format!("{}: missing flattened fields", c.path));
            continue;
        }
        field_lines.sort_unstable();
        let first = *field_lines.first().unwrap();
        let last = *field_lines.last().unwrap();
        // Contiguity: every `pub X:` between first..=last must be one of ours.
        let ours: std::collections::HashSet<usize> = field_lines.iter().copied().collect();
        let interloper =
            (first..=last).any(|i| lines[i].trim_start().starts_with("pub ") && !ours.contains(&i));
        if interloper {
            skipped.push(format!("{}: flattened fields not contiguous", c.path));
            continue;
        }
        // Extend the start up over the first field's leading doc comments.
        let mut start = first;
        while start > *s_start {
            let prev = lines[start - 1].trim_start();
            if prev.starts_with("///") || prev.starts_with("#[") {
                start -= 1;
            } else {
                break;
            }
        }
        let replacement = format!(
            "    /// The `{}` choice element (0..1); see [`{}`].\n    #[serde(flatten)]\n    pub {}: Option<{}>,",
            c.path, c.enum_name, c.field, c.enum_name
        );
        edits.push((start, last, replacement));
        enums.push_str(&render_enum(c));
        for v in &c.variants {
            removed_fields.push(v.old_field.clone());
        }
        applied += 1;
    }

    if applied == 0 {
        return (text.to_string(), 0);
    }

    // Apply span edits bottom-up so indices stay valid.
    let mut out: Vec<String> = lines.iter().map(|s| (*s).to_string()).collect();
    edits.sort_by_key(|e| std::cmp::Reverse(e.0));
    for (start, end, replacement) in edits {
        out.splice(start..=end, std::iter::once(replacement));
    }

    // Remove the old fields from any `let expect = T { … };` test literal, and
    // ensure `..Default::default()` is present so the new field is covered.
    let removed: std::collections::HashSet<&str> =
        removed_fields.iter().map(String::as_str).collect();
    let mut in_expect = false;
    let mut has_rest = false;
    let mut filtered: Vec<String> = Vec::with_capacity(out.len());
    for line in out {
        let t = line.trim_start();
        if t.starts_with("let expect = T {") {
            in_expect = true;
            has_rest = false;
            filtered.push(line);
            continue;
        }
        if in_expect {
            // A test-literal field line like `value_quantity: None,`.
            let field = t.split(':').next().unwrap_or("").trim();
            if removed.contains(field) {
                continue; // drop it
            }
            if t.contains("..Default::default()") {
                has_rest = true;
            }
            if t == "};" {
                if !has_rest {
                    filtered.push("            ..Default::default()".to_string());
                }
                in_expect = false;
            }
        }
        filtered.push(line);
    }

    let mut joined = filtered.join("\n");
    joined.push_str(&enums);
    if text.ends_with('\n') && !joined.ends_with('\n') {
        joined.push('\n');
    }
    (joined, applied)
}

/// Apply choice conversions to a group (`types` or `resources`), writing files.
pub fn apply_group(group: &str, skipped: &mut Vec<String>) -> usize {
    let plan = plan();
    let dir = crate_root().join("fhir-r5").join("src").join(group);
    let mut total = 0;
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("read dir")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    files.sort();
    for file in files {
        let text = std::fs::read_to_string(&file).expect("read");
        // Which conversions target a struct defined in this file?
        let names: std::collections::HashSet<String> =
            struct_spans(&text.lines().collect::<Vec<_>>())
                .into_iter()
                .map(|(n, ..)| n)
                .collect();
        let here: Vec<ChoiceConversion> = plan
            .iter()
            .filter(|c| names.contains(&c.struct_name))
            .cloned()
            .collect();
        if here.is_empty() {
            continue;
        }
        let (new_text, applied) = apply_file(&text, &here, skipped);
        if applied > 0 {
            std::fs::write(&file, new_text).expect("write");
            total += applied;
        }
    }
    total
}

/// Read all model source once, into one big string per group, for cross-checks.
#[cfg(test)]
fn all_source() -> String {
    let mut s = String::new();
    for group in ["types", "resources"] {
        let dir = crate_root().join("fhir-r5").join("src").join(group);
        for entry in std::fs::read_dir(&dir).expect("read dir").flatten() {
            let p = entry.path();
            if p.extension().is_some_and(|x| x == "rs") {
                s.push_str(&std::fs::read_to_string(&p).expect("read"));
                s.push('\n');
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_covers_known_choices() {
        let plan = plan();
        // Observation.value[x] -> ObservationValue with 13 variants.
        let obs = plan
            .iter()
            .find(|c| c.path == "Observation.value[x]")
            .expect("value[x]");
        assert_eq!(obs.enum_name, "ObservationValue");
        assert_eq!(obs.field, "value");
        assert_eq!(obs.variants.len(), 13);
        let q = obs.variants.iter().find(|v| v.code == "Quantity").unwrap();
        assert_eq!(q.key, "valueQuantity");
        assert_eq!(q.old_field, "value_quantity");
        assert!(!q.primitive);
        let s = obs.variants.iter().find(|v| v.code == "string").unwrap();
        assert_eq!(s.key, "valueString");
        assert_eq!(s.variant, "String");
        assert!(s.primitive);
    }

    /// Dry run: report the plan and flag any conversion whose expected flattened
    /// fields are not all present in the source (a mapping mismatch to fix
    /// before applying). Ignored.
    #[test]
    #[ignore = "dry-run report only"]
    fn report() {
        let plan = plan();
        let src = all_source();
        println!("choice conversions planned: {}", plan.len());
        let mut missing = 0;
        for c in &plan {
            let absent: Vec<&str> = c
                .variants
                .iter()
                .filter(|v| !src.contains(&format!("pub {}:", v.old_field)))
                .map(|v| v.old_field.as_str())
                .collect();
            if !absent.is_empty() {
                missing += 1;
                println!(
                    "  {} ({}): missing fields {:?}",
                    c.path, c.enum_name, absent
                );
            }
        }
        println!(
            "conversions with missing/renamed fields: {missing} of {}",
            plan.len()
        );
    }

    #[test]
    #[ignore = "writes fhir-r5/src/types/*.rs"]
    fn apply_types() {
        let mut skipped = Vec::new();
        let n = apply_group("types", &mut skipped);
        println!(
            "types: applied {n} choice conversions; skipped {}",
            skipped.len()
        );
        for s in &skipped {
            println!("  SKIP {s}");
        }
    }

    #[test]
    #[ignore = "writes fhir-r5/src/resources/*.rs"]
    fn apply_resources() {
        let mut skipped = Vec::new();
        let n = apply_group("resources", &mut skipped);
        println!(
            "resources: applied {n} choice conversions; skipped {}",
            skipped.len()
        );
        for s in &skipped {
            println!("  SKIP {s}");
        }
    }
}
