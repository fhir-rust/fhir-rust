//! Generator: retype FHIR `0..*` fields from `Option<Vec<T>>` to bare `Vec<T>`.
//!
//! A `0..*` element is optional-repeating; modelling it as a bare `Vec<T>`
//! (empty when absent) is simpler than `Option<Vec<T>>`. The empty vector is
//! omitted from JSON with `#[serde(default, skip_serializing_if = "Vec::is_empty")]`,
//! matching the previous "absent when `None`" behaviour. Generated `test_default`
//! literals that list a converted field as `None` have that line removed (the
//! struct's `..Default::default()` covers the now-empty `Vec`).

use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

const SERDE_ATTR: &str = "    #[serde(default, skip_serializing_if = \"Vec::is_empty\")]";

/// Convert one file's `Option<Vec<…>>` fields. Returns the new text and count.
#[must_use]
pub fn apply_file(text: &str) -> (String, usize) {
    let lines: Vec<&str> = text.lines().collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len() + 64);
    let mut converted: Vec<String> = Vec::new();
    let mut count = 0;

    for line in &lines {
        if let Some(field) = parse_option_vec_field(line) {
            // Insert the skip-empty serde attribute, then the retyped field.
            out.push(SERDE_ATTR.to_string());
            out.push(retype_line(line));
            converted.push(field);
            count += 1;
        } else {
            out.push((*line).to_string());
        }
    }

    if count == 0 {
        return (text.to_string(), 0);
    }

    // Drop `<field>: None,` lines for converted fields inside a `let expect = T {`
    // test literal (their type is now `Vec`, so `None` no longer type-checks;
    // `..Default::default()` supplies the empty vec).
    let removed: std::collections::HashSet<&str> = converted.iter().map(String::as_str).collect();
    let mut in_expect = false;
    let mut filtered: Vec<String> = Vec::with_capacity(out.len());
    for line in out {
        let t = line.trim_start();
        if t.starts_with("let expect = T {") {
            in_expect = true;
        } else if in_expect {
            if t == "};" {
                in_expect = false;
            } else {
                let field = t.split(':').next().unwrap_or("").trim();
                if removed.contains(field) && t.ends_with("None,") {
                    continue;
                }
            }
        }
        filtered.push(line);
    }

    let mut joined = filtered.join("\n");
    if text.ends_with('\n') {
        joined.push('\n');
    }
    (joined, count)
}

/// If `line` declares `pub <name>: Option<Vec<…>>,`, return `<name>`.
fn parse_option_vec_field(line: &str) -> Option<String> {
    let t = line.trim_start();
    let rest = t.strip_prefix("pub ")?;
    let colon = rest.find(':')?;
    let name = rest[..colon].trim().to_string();
    let after = rest[colon + 1..].trim_start();
    if after.starts_with("Option<Vec<") {
        Some(name)
    } else {
        None
    }
}

/// Rewrite `pub x: Option<Vec<INNER>>,` to `pub x: Vec<INNER>,`, preserving any
/// trailing comment.
fn retype_line(line: &str) -> String {
    let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
    // Split a trailing `// comment`.
    let (code, comment) = match line.split_once("//") {
        Some((c, cm)) => (c.trim_end(), Some(cm)),
        None => (line.trim_end(), None),
    };
    let code = code.trim();
    // code is `pub <name>: Option<Vec<INNER>>,`
    let body = code.trim_end_matches(',');
    let (decl, ty) = body.split_once(':').expect("field has a colon");
    let ty = ty.trim();
    let new_ty = ty
        .strip_prefix("Option<")
        .and_then(|s| s.strip_suffix('>'))
        .unwrap_or(ty);
    let mut result = format!("{indent}{}: {new_ty},", decl.trim());
    if let Some(cm) = comment {
        result.push_str(" //");
        result.push_str(cm);
    }
    result
}

/// Apply to a group (`types` or `resources`), writing files. Returns count.
pub fn apply_group(group: &str) -> usize {
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
        let (new_text, count) = apply_file(&text);
        if count > 0 {
            std::fs::write(&file, new_text).expect("write");
            total += count;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retypes_a_field() {
        let (out, n) = apply_file("    pub name: Option<Vec<types::HumanName>>,\n");
        assert_eq!(n, 1);
        assert!(out.contains("pub name: Vec<types::HumanName>,"), "{out}");
        assert!(
            out.contains("skip_serializing_if = \"Vec::is_empty\""),
            "{out}"
        );
    }

    #[test]
    fn preserves_comment_and_nested_generics() {
        let (out, _) =
            apply_file("    pub contained: Option<Vec<::serde_json::Value>>, // « x »\n");
        assert!(
            out.contains("pub contained: Vec<::serde_json::Value>, // « x »"),
            "{out}"
        );
    }

    #[test]
    #[ignore = "writes fhir-r5/src/resources/*.rs"]
    fn apply_resources() {
        println!(
            "resources: converted {} Option<Vec> fields",
            apply_group("resources")
        );
    }

    #[test]
    #[ignore = "writes fhir-r5/src/types/*.rs"]
    fn apply_types() {
        println!(
            "types: converted {} Option<Vec> fields",
            apply_group("types")
        );
    }
}
