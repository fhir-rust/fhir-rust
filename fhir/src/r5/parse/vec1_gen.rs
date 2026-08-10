//! Generator: retype genuine FHIR `1..*` fields to `vec1::Vec1<T>`.
//!
//! `1..*` elements are modelled as a bare `Vec<T>`, but so are some `0..*`
//! elements, so the real cardinality is read from [`crate::r5::meta`]
//! (`min >= 1 && max == "*"`). Each such field becomes a non-empty `Vec1<T>`.
//! A struct with a `Vec1` field can no longer derive `Default` (there is no empty
//! value), so `Default` is dropped from its derive and its generated
//! `test_default`/serde-default tests (which call `T::default()`) are removed.

use std::collections::BTreeMap;
use std::path::PathBuf;

use ::convert_case::{Case, Casing};

use crate::r5::meta;

/// A planned field retype: `(struct name, Rust field ident)`.
#[derive(Debug, Clone)]
pub struct Vec1Field {
    pub struct_name: String,
    pub field: String,
}

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "type" | "use" | "for" | "abstract" | "const" | "ref" | "final" | "in" | "as"
    )
}

/// Every genuine `1..*` element, as `(struct, field)`.
#[must_use]
pub fn plan() -> Vec<Vec1Field> {
    // struct name for a path prefix.
    let mut prefix_to_struct: BTreeMap<String, String> = BTreeMap::new();
    for el in meta::elements() {
        let segs: Vec<&str> = el.path.split('.').collect();
        for take in 1..segs.len() {
            let name: String = segs[..take]
                .iter()
                .map(|s| s.to_case(Case::Pascal))
                .collect();
            prefix_to_struct
                .entry(segs[..take].join("."))
                .or_insert(name);
        }
    }
    let mut out = Vec::new();
    for el in meta::elements() {
        if el.min < 1 || el.max != "*" {
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
        let field = if is_keyword(&snake) {
            format!("r#{snake}")
        } else {
            snake
        };
        out.push(Vec1Field {
            struct_name: struct_name.clone(),
            field,
        });
    }
    out
}

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
            let (mut depth, mut j, mut started) = (0i32, i, false);
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

/// Apply the retype to a file. Returns the new text and the count of fields
/// changed.
#[must_use]
pub fn apply_file(text: &str, fields: &[Vec1Field]) -> (String, usize) {
    let mut lines: Vec<String> = text.lines().map(String::from).collect();
    let spans = struct_spans(&lines.iter().map(String::as_str).collect::<Vec<_>>());
    let mut applied = 0;
    let mut structs_defaulted_out: Vec<String> = Vec::new();

    for f in fields {
        let Some((_, start, end)) = spans.iter().find(|(n, ..)| n == &f.struct_name) else {
            continue;
        };
        let needle = format!("pub {}:", f.field);
        let Some(idx) = (*start..=*end).find(|&i| lines[i].trim_start().starts_with(&needle))
        else {
            continue;
        };
        if !lines[idx].contains("Vec<") {
            continue; // already Vec1, or an unexpected shape
        }
        lines[idx] = lines[idx].replacen("Vec<", "vec1::Vec1<", 1);
        // Drop a `#[serde(default, skip_serializing_if = "Vec::is_empty")]` just
        // above the field (a non-empty Vec1 is always serialized).
        if idx > 0 && lines[idx - 1].contains("skip_serializing_if = \"Vec::is_empty\"") {
            lines[idx - 1] = String::new();
        }
        // Drop `Default` from this struct's derive line.
        let derive_idx = ((*start).saturating_sub(6)..*start)
            .find(|&l| lines[l].contains("#[derive(") && lines[l].contains("Default"));
        if let Some(l) = derive_idx {
            lines[l] = lines[l].replace("Default, ", "").replace(", Default", "");
            if !structs_defaulted_out.contains(&f.struct_name) {
                structs_defaulted_out.push(f.struct_name.clone());
            }
        }
        applied += 1;
    }

    if applied == 0 {
        return (text.to_string(), 0);
    }

    // If the file's `type T = X;` test alias names a struct that lost Default,
    // remove the generated `#[cfg(test)] mod tests { … }` block (it calls
    // `T::default()`).
    let mut joined: Vec<String> = lines;
    let root_lost_default = joined
        .iter()
        .filter_map(|l| {
            l.trim()
                .strip_prefix("type T = ")
                .map(|s| s.trim_end_matches(';').trim().to_string())
        })
        .any(|t| structs_defaulted_out.contains(&t));
    if root_lost_default {
        remove_test_module(&mut joined);
        ignore_default_doctest(&mut joined);
    }
    (joined.join("\n"), applied)
}

/// Mark the generated `# Examples` doctest (which calls `Type::default()`) as
/// `ignore`, since the type no longer implements `Default`.
fn ignore_default_doctest(lines: &mut [String]) {
    let mut i = 0;
    while i < lines.len() {
        if lines[i].trim() == "/// ```" {
            // Find the closing fence and scan the block for `::default()`.
            let mut j = i + 1;
            let mut uses_default = false;
            while j < lines.len() && lines[j].trim() != "/// ```" {
                if lines[j].contains("::default()") {
                    uses_default = true;
                }
                j += 1;
            }
            if uses_default {
                lines[i] = "/// ```ignore".to_string();
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
}

/// Remove the `#[cfg(test)] mod tests { … }` block from a file's lines.
fn remove_test_module(lines: &mut Vec<String>) {
    let Some(cfg_idx) = lines.iter().position(|l| l.trim() == "#[cfg(test)]") else {
        return;
    };
    // Find `mod tests {` on the following line(s), then brace-match to its close.
    let Some(mod_idx) =
        (cfg_idx..lines.len()).find(|&i| lines[i].trim_start().starts_with("mod tests"))
    else {
        return;
    };
    let (mut depth, mut j, mut started) = (0i32, mod_idx, false);
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
    lines.drain(cfg_idx..=j);
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
        let here: Vec<Vec1Field> = plan
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
    fn plan_finds_known_fields() {
        let plan = plan();
        assert!(plan.len() >= 55, "got {}", plan.len());
        assert!(
            plan.iter()
                .any(|f| f.struct_name == "Appointment" && f.field == "participant")
        );
        assert!(
            plan.iter()
                .any(|f| f.struct_name == "OperationOutcome" && f.field == "issue")
        );
    }

    #[test]
    #[ignore = "writes fhir-r5/src/resources/*.rs"]
    fn apply_resources() {
        println!(
            "resources: retyped {} 1..* fields to Vec1",
            apply_group("resources")
        );
    }

    #[test]
    #[ignore = "writes fhir-r5/src/types/*.rs"]
    fn apply_types() {
        println!(
            "types: retyped {} 1..* fields to Vec1",
            apply_group("types")
        );
    }
}
