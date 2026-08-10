//! Generator (T10): type `required`-binding coded fields as `Coded<Enum>`.
//!
//! For each element with a `required` binding whose value set maps to an
//! existing [`codes`](crate::r5::codes) enum, swap the field's `types::Code`
//! storage for `crate::r5::coded::Coded<crate::r5::codes::Enum>`. Like the other
//! generators it splices the hand-documented files rather than regenerating
//! them, is struct-scoped, and is idempotent.

use std::collections::BTreeMap;
use std::path::PathBuf;

use ::convert_case::{Case, Casing};

use crate::r5::meta;

/// One planned field retype.
#[derive(Debug, Clone)]
pub struct CodedField {
    /// FHIR element path, e.g. `"Patient.gender"`.
    pub path: String,
    /// Owning Rust struct, e.g. `Patient`.
    pub struct_name: String,
    /// Rust field identifier as written (may be `r#type`).
    pub field: String,
    /// `codes` enum name, e.g. `AdministrativeGender`.
    pub enum_name: String,
}

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The set of enum names declared in `codes.rs`.
fn code_enum_names() -> std::collections::HashSet<String> {
    let codes = std::fs::read_to_string(crate_root().join("fhir-r5").join("src").join("codes.rs"))
        .expect("read codes.rs");
    codes
        .lines()
        .filter_map(|l| l.trim().strip_prefix("pub enum "))
        .map(|r| r.chars().take_while(char::is_ascii_alphanumeric).collect())
        .collect()
}

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

/// Plan every required-binding field whose value set maps to a known enum.
#[must_use]
pub fn plan() -> Vec<CodedField> {
    let enums = code_enum_names();
    let prefix_to_struct: BTreeMap<String, String> = struct_path_prefixes()
        .into_iter()
        .map(|(name, prefix)| (prefix, name))
        .collect();

    let mut out = Vec::new();
    for el in meta::elements() {
        let Some(binding) = &el.binding else { continue };
        if binding.strength != meta::BindingStrength::Required {
            continue;
        }
        let Some(value_set) = binding.value_set else {
            continue;
        };
        let vs_name = value_set
            .split('|')
            .next()
            .unwrap()
            .rsplit('/')
            .next()
            .unwrap();
        let enum_name = vs_name.to_case(Case::Pascal);
        if !enums.contains(&enum_name) {
            continue;
        }
        let (Some(parent), Some(leaf)) = (
            el.path.rsplit_once('.').map(|x| x.0),
            el.path.rsplit('.').next(),
        ) else {
            continue;
        };
        let Some(struct_name) = prefix_to_struct.get(parent) else {
            continue;
        };
        let snake = leaf.to_case(Case::Snake);
        let field = if is_rust_keyword(&snake) {
            format!("r#{snake}")
        } else {
            snake
        };
        out.push(CodedField {
            path: el.path.to_string(),
            struct_name: struct_name.clone(),
            field,
            enum_name,
        });
    }
    out
}

fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
        "type" | "use" | "for" | "abstract" | "const" | "ref" | "final" | "as" | "in" | "if"
    )
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
                i += 1;
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

/// Replace a `types::Code` field type with `Coded<codes::Enum>`. Returns the new
/// type string, or `None` if the field isn't a plain `Code` storage type.
fn retype(ty: &str, enum_name: &str) -> Option<String> {
    let coded = format!("crate::r5::coded::Coded<crate::r5::codes::{enum_name}>");
    match ty.trim() {
        "types::Code" => Some(coded),
        "Option<types::Code>" => Some(format!("Option<{coded}>")),
        "Vec<types::Code>" => Some(format!("Vec<{coded}>")),
        "Option<Vec<types::Code>>" => Some(format!("Option<Vec<{coded}>>")),
        _ => None,
    }
}

/// Apply the retype to a file's text; returns new text and count applied.
#[must_use]
pub fn apply_file(text: &str, fields: &[CodedField]) -> (String, usize) {
    let mut lines: Vec<String> = text.lines().map(String::from).collect();
    let spans = struct_spans(&lines.iter().map(String::as_str).collect::<Vec<_>>());
    let mut applied = 0;
    let mut retyped_fields: Vec<String> = Vec::new();

    for f in fields {
        let Some((_, start, end)) = spans.iter().find(|(n, ..)| n == &f.struct_name) else {
            continue;
        };
        let needle = format!("pub {}:", f.field);
        let Some(idx) = (*start..=*end).find(|&i| lines[i].trim_start().starts_with(&needle))
        else {
            continue;
        };
        // Split off a trailing `// comment`.
        let (code_part, comment) = match lines[idx].split_once("//") {
            Some((c, cm)) => (c, Some(cm)),
            None => (lines[idx].as_str(), None),
        };
        let ty = code_part
            .trim()
            .trim_end_matches(',')
            .split_once(':')
            .map(|x| x.1.trim());
        let Some(ty) = ty else { continue };
        let Some(new_ty) = retype(ty, &f.enum_name) else {
            continue;
        };
        let indent: String = lines[idx]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect();
        let mut new_line = format!("{indent}pub {}: {new_ty},", f.field);
        if let Some(cm) = comment {
            new_line.push_str(" //");
            new_line.push_str(cm);
        }
        lines[idx] = new_line;
        retyped_fields.push(f.field.trim_start_matches("r#").to_string());
        applied += 1;
    }

    if applied == 0 {
        return (text.to_string(), 0);
    }

    // Drop retyped non-Option fields from test_default literals (their old
    // `types::Code::default()` no longer type-checks); `..Default::default()`
    // covers them.
    let removed: std::collections::HashSet<&str> =
        retyped_fields.iter().map(String::as_str).collect();
    let mut in_expect = false;
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    for line in lines {
        let t = line.trim_start();
        if t.starts_with("let expect = T {") {
            in_expect = true;
            out.push(line);
            continue;
        }
        if in_expect {
            let field = t
                .split(':')
                .next()
                .unwrap_or("")
                .trim()
                .trim_start_matches("r#");
            if removed.contains(field) && !t.contains("None") {
                continue; // drop non-Option code fields; Option ones keep `None`
            }
            if t == "};" {
                in_expect = false;
            }
        }
        out.push(line);
    }

    let mut joined = out.join("\n");
    if text.ends_with('\n') && !joined.ends_with('\n') {
        joined.push('\n');
    }
    (joined, applied)
}

/// Apply to a group (`types` or `resources`), writing files. Returns count.
pub fn apply_group(group: &str) -> usize {
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
        let names: std::collections::HashSet<String> =
            struct_spans(&text.lines().collect::<Vec<_>>())
                .into_iter()
                .map(|(n, ..)| n)
                .collect();
        let here: Vec<CodedField> = plan
            .iter()
            .filter(|f| names.contains(&f.struct_name))
            .cloned()
            .collect();
        if here.is_empty() {
            continue;
        }
        let (new_text, applied) = apply_file(&text, &here);
        if applied > 0 {
            std::fs::write(&file, new_text).expect("write");
            total += applied;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_maps_known_fields() {
        let plan = plan();
        assert!(
            plan.len() > 100,
            "expected >100 coded fields, got {}",
            plan.len()
        );
        let gender = plan
            .iter()
            .find(|f| f.path == "Patient.gender")
            .expect("Patient.gender");
        assert_eq!(gender.enum_name, "AdministrativeGender");
        assert_eq!(gender.field, "gender");
    }

    #[test]
    #[ignore = "writes fhir-r5/src/types/*.rs"]
    fn apply_types() {
        println!("types: retyped {} coded fields", apply_group("types"));
    }

    #[test]
    #[ignore = "writes fhir-r5/src/resources/*.rs"]
    fn apply_resources() {
        println!(
            "resources: retyped {} coded fields",
            apply_group("resources")
        );
    }
}
