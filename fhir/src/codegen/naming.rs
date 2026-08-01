//! Turning FHIR names into Rust identifiers.
//!
//! The rules here are the ones the shipped R5 model already follows (see
//! `AGENTS/conventions.md`), factored out so every release names things the
//! same way: `Patient.contact` becomes the struct `PatientContact` in the
//! module `patient`, its `type` element becomes the field `r#type`, and the
//! code `entered-in-error` becomes the enum variant `EnteredInError`.

use ::convert_case::{Case, Casing};

/// Rust keywords that a FHIR element name can collide with, which are written
/// as raw identifiers (`r#type`).
///
/// `rename_all = "camelCase"` sees the unescaped name, so no per-field
/// `#[serde(rename)]` is needed.
const RUST_KEYWORDS: [&str; 36] = [
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "final", "fn", "for", "if", "impl", "in", "let", "loop",
    "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return", "static", "type",
    "typeof", "use",
];

/// Keywords that cannot be written as raw identifiers, so they take a trailing
/// underscore instead.
const NON_RAW_KEYWORDS: [&str; 3] = ["crate", "self", "super"];

/// PascalCase, e.g. `referenceRange` becomes `ReferenceRange`.
#[must_use]
pub fn pascal(s: &str) -> String {
    s.to_case(Case::Pascal)
}

/// snake_case, e.g. `referenceRange` becomes `reference_range`.
#[must_use]
pub fn snake(s: &str) -> String {
    s.to_case(Case::Snake)
}

/// A Rust field identifier for a FHIR element name, escaping keywords.
#[must_use]
pub fn field_ident(name: &str) -> String {
    let snake = snake(name);
    if NON_RAW_KEYWORDS.contains(&snake.as_str()) {
        format!("{snake}_")
    } else if RUST_KEYWORDS.contains(&snake.as_str()) {
        format!("r#{snake}")
    } else {
        snake
    }
}

/// The JSON key serde derives from a field identifier under
/// `#[serde(rename_all = "camelCase")]`.
///
/// serde only ever uppercases the letter after an underscore, so it cannot
/// reproduce a FHIR name with consecutive capitals: `truth_tp` becomes
/// `truthTp`, never `truthTP`. Comparing this against the real element name is
/// how [`needs_explicit_rename`] decides when a field must spell its key out.
#[must_use]
pub fn serde_camel(ident: &str) -> String {
    let bare = ident.strip_prefix("r#").unwrap_or(ident);
    let mut out = String::new();
    let mut capitalize = false;
    for c in bare.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            out.extend(c.to_uppercase());
            capitalize = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// Whether a field needs an explicit `#[serde(rename = …)]` because
/// `rename_all = "camelCase"` cannot produce its FHIR element name.
#[must_use]
pub fn needs_explicit_rename(ident: &str, fhir_name: &str) -> bool {
    serde_camel(ident) != fhir_name
}

/// The struct name for an element path: the PascalCase of every segment joined
/// together, e.g. `Observation.component` becomes `ObservationComponent`.
#[must_use]
pub fn struct_name(path: &str) -> String {
    path.split('.').map(pascal).collect()
}

/// The module file stem for a type, e.g. `CodeableConcept` becomes
/// `codeable_concept`.
#[must_use]
pub fn module_name(type_name: &str) -> String {
    snake(type_name)
}

/// The FHIR JSON key of one variant of a choice element: the base name plus the
/// type code with its first letter capitalized, e.g. `value` + `dateTime`
/// becomes `valueDateTime`.
#[must_use]
pub fn choice_key(base: &str, type_code: &str) -> String {
    let mut chars = type_code.chars();
    let first = chars.next().map(|c| c.to_ascii_uppercase());
    format!(
        "{base}{}{}",
        first.map(String::from).unwrap_or_default(),
        chars.as_str()
    )
}

/// A Rust type name for a FHIR identifier that is not necessarily an
/// identifier at all.
///
/// Code system URLs do not have to be `http` URLs with tidy path segments —
/// `urn:iso-astm:E1762-95:2013` is a real one — so the name is cleaned the same
/// way a variant is. Both the `codes` generator and the binding lookup that
/// resolves a value set to an enum use this, so they always agree.
#[must_use]
pub fn enum_name(s: &str) -> String {
    sanitize_variant(s)
}

/// A Rust enum variant name for a FHIR code, e.g. `entered-in-error` becomes
/// `EnteredInError`.
///
/// Codes are not always valid identifiers: they may start with a digit, be a
/// Rust keyword, contain punctuation, or (after PascalCasing) collide with
/// another code in the same system. `sanitize_variant` handles the first three;
/// callers resolve collisions with [`dedupe`].
#[must_use]
pub fn sanitize_variant(code: &str) -> String {
    // Punctuation that PascalCase does not already treat as a word boundary
    // would otherwise survive into the identifier.
    let cleaned: String = code
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect();
    let mut name = pascal(cleaned.trim());
    if name.is_empty() {
        name = "Unnamed".to_string();
    }
    // An identifier may not start with a digit.
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        name.insert(0, 'N');
    }
    // `Self` is reserved even in type position.
    if name == "Self" {
        name = "SelfCode".to_string();
    }
    name
}

/// Make every name in `names` unique by appending `2`, `3`, … to repeats,
/// preserving order.
///
/// Distinct FHIR codes can PascalCase to the same identifier (`in-progress` and
/// `inProgress`, for example), and a Rust enum cannot repeat a variant name.
#[must_use]
pub fn dedupe(names: &[String]) -> Vec<String> {
    let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    names
        .iter()
        .map(|name| {
            let count = seen.entry(name.clone()).or_insert(0);
            *count += 1;
            if *count == 1 {
                name.clone()
            } else {
                format!("{name}{count}")
            }
        })
        .collect()
}

/// Wrap prose as a `///` doc comment at the given indent, hard-wrapping to a
/// readable width and escaping anything rustdoc would misread.
#[must_use]
pub fn doc_comment(text: &str, indent: &str) -> String {
    let text = collapse_whitespace(text);
    if text.is_empty() {
        return String::new();
    }
    wrap(&text, 79 - indent.len() - 4)
        .into_iter()
        .fold(String::new(), |mut out, line| {
            out.push_str(indent);
            out.push_str("/// ");
            out.push_str(&line);
            out.push('\n');
            out
        })
}

/// Wrap prose as a `//!` module doc comment.
#[must_use]
pub fn module_doc_comment(text: &str) -> String {
    let text = collapse_whitespace(text);
    if text.is_empty() {
        return String::new();
    }
    wrap(&text, 75)
        .into_iter()
        .fold(String::new(), |mut out, line| {
            out.push_str("//! ");
            out.push_str(&line);
            out.push('\n');
            out
        })
}

/// Collapse all runs of whitespace (including the newlines the specification
/// prose is full of) into single spaces.
fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Greedily hard-wrap `text` to `width` columns, never splitting a word.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split(' ') {
        if current.is_empty() {
            current = word.to_string();
        } else if current.chars().count() + 1 + word.chars().count() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    for line in &mut lines {
        escape_block_marker(line);
    }
    lines
}

/// Escape a leading character that Markdown would read as a block marker.
///
/// The specification's prose is plain paragraphs, but hard-wrapping it can put
/// a word like "- types that are defined" at the start of a line, which rustdoc
/// then renders as a bullet list with a badly indented continuation. A leading
/// backslash makes the character literal again.
fn escape_block_marker(line: &mut String) {
    let starts_a_block = match line.chars().next() {
        Some('-' | '+' | '*' | '>' | '#' | '=') => true,
        Some(c) if c.is_ascii_digit() => {
            let rest = line.trim_start_matches(|c: char| c.is_ascii_digit());
            rest.starts_with(". ") || rest.starts_with(") ")
        }
        _ => false,
    };
    if starts_a_block {
        line.insert(0, '\\');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers() {
        assert_eq!(field_ident("referenceRange"), "reference_range");
        assert_eq!(field_ident("type"), "r#type");
        assert_eq!(field_ident("use"), "r#use");
        assert_eq!(field_ident("abstract"), "r#abstract");
        assert_eq!(field_ident("name"), "name");
    }

    #[test]
    fn serde_camel_matches_serdes_own_rule() {
        assert_eq!(serde_camel("reference_range"), "referenceRange");
        assert_eq!(serde_camel("r#type"), "type");
        assert_eq!(serde_camel("name"), "name");
        // serde cannot recover consecutive capitals.
        assert_eq!(serde_camel("truth_tp"), "truthTp");
    }

    #[test]
    fn only_unrecoverable_names_need_a_rename() {
        assert!(!needs_explicit_rename("reference_range", "referenceRange"));
        assert!(!needs_explicit_rename("r#type", "type"));
        // `truthTP` and `requestURL` keep runs of capitals that camelCase loses.
        assert!(needs_explicit_rename("truth_tp", "truthTP"));
        assert!(needs_explicit_rename("request_url", "requestURL"));
    }

    #[test]
    fn struct_names_join_segments() {
        assert_eq!(struct_name("Patient.contact"), "PatientContact");
        assert_eq!(struct_name("Claim.item.detail"), "ClaimItemDetail");
        assert_eq!(struct_name("Timing.repeat"), "TimingRepeat");
        assert_eq!(struct_name("Observation"), "Observation");
    }

    #[test]
    fn module_names() {
        assert_eq!(module_name("CodeableConcept"), "codeable_concept");
        assert_eq!(module_name("Base64Binary"), "base_64_binary");
    }

    #[test]
    fn choice_keys_capitalize_the_type_code() {
        assert_eq!(choice_key("value", "dateTime"), "valueDateTime");
        assert_eq!(
            choice_key("value", "CodeableConcept"),
            "valueCodeableConcept"
        );
        assert_eq!(choice_key("value", "base64Binary"), "valueBase64Binary");
        assert_eq!(choice_key("onset", "Age"), "onsetAge");
        assert_eq!(choice_key("reason", "Reference"), "reasonReference");
    }

    #[test]
    fn variants_are_valid_identifiers() {
        assert_eq!(sanitize_variant("entered-in-error"), "EnteredInError");
        assert_eq!(sanitize_variant("final"), "Final");
        assert_eq!(sanitize_variant("1.0"), "N10");
        assert_eq!(sanitize_variant("<"), "Unnamed");
        assert_eq!(sanitize_variant("self"), "SelfCode");
        assert_eq!(sanitize_variant("_"), "Unnamed");
    }

    #[test]
    fn collisions_get_numbered() {
        let names = vec![
            "A".to_string(),
            "B".to_string(),
            "A".to_string(),
            "A".to_string(),
        ];
        assert_eq!(dedupe(&names), ["A", "B", "A2", "A3"]);
    }

    #[test]
    fn docs_wrap_and_collapse() {
        let doc = doc_comment("one\n  two   three", "    ");
        assert_eq!(doc, "    /// one two three\n");
        let long = doc_comment(&"word ".repeat(40), "");
        assert!(long.lines().count() > 1);
        assert!(long.lines().all(|l| l.starts_with("/// ")));
    }

    #[test]
    fn wrapped_lines_never_start_a_markdown_block() {
        // Wrapping this prose puts "- types that are defined" at the start of a
        // line, which rustdoc would otherwise render as a bullet list.
        let doc = doc_comment(
            "A list of the base types defined by this version of the FHIR specification \
             - types that are defined, but for which only specializations actually are created.",
            "",
        );
        assert!(doc.contains("/// \\- types"), "{doc}");
        assert!(!doc.contains("/// - types"), "{doc}");
    }

    #[test]
    fn ordinary_prose_is_left_alone() {
        assert_eq!(doc_comment("plain text", ""), "/// plain text\n");
    }

    #[test]
    fn empty_docs_are_omitted() {
        assert_eq!(doc_comment("", "    "), "");
        assert_eq!(module_doc_comment("   "), "");
    }
}
