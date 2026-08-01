//! Element into Rust struct attribute
//!
//! Convert one FHIR `ElementDefinition` into the Rust struct field(s) that
//! represent it, including the field name, the Rust type, the FHIR cardinality
//! (mapped to `Option<T>` / `Vec<T>` / `Option<Vec<T>>` / `T`), and a doc
//! comment derived from the element's `short` description.

use crate::SourceCodeString;
use crate::r5::parse::all::ElementType;
use crate::r5::parse::profiles_types::*;
use ::convert_case::{Case, Casing};

/// Indent the start of each Rust attribute line of source code.
const RUST_ATTRIBUTE_INDENT: &str = "    ";

/// Rust reserved words that cannot be used as bare identifiers, so a field
/// with one of these names is emitted as a raw identifier (e.g. `r#type`).
/// Serde's `rename_all = "camelCase"` still sees the unescaped name, so no
/// explicit `#[serde(rename = ...)]` is required.
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "abstract", "become", "box", "do", "final", "macro", "override",
    "priv", "typeof", "unsized", "virtual", "yield", "async", "await", "try", "gen",
];

/// Given one element, generate Rust struct attribute source code.
///
/// Returns an empty string for elements that are not struct fields, such as the
/// root element of a resource or datatype (its path has no `.` separator).
///
/// A choice element (path ending in `[x]`) expands to one field per allowed
/// type, e.g. `Annotation.author[x]` with types `Reference | string` yields
/// both `author_reference` and `author_string` fields.
///
/// Example:
///
/// ```text
/// let element = …; // e.g. path "Annotation.time", type "dateTime", 0..1
/// let attribute = element_into_rust_struct_attribute(&element);
/// // => "    /// When the annotation was made\n    time: Option<types::DateTime>, // dateTime [0..1]\n"
/// ```
#[allow(dead_code)]
pub fn element_into_rust_struct_attribute(element: &Element) -> SourceCodeString {
    let path = if element.path.is_empty() {
        element.id.as_str()
    } else {
        element.path.as_str()
    };

    // The root element (e.g. "Annotation") is the type itself, not a field.
    let Some((_, tail)) = path.rsplit_once('.') else {
        return SourceCodeString::new();
    };

    let doc = element
        .short
        .as_deref()
        .unwrap_or("Short description goes here.")
        .replace('\n', " ");
    let (min, max) = cardinality(element);

    // A choice element expands to one field per allowed type.
    if let Some(base) = tail.strip_suffix("[x]") {
        return element
            .r#type
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|t| {
                let field = format!("{base}_{}", t.code.to_case(Case::Snake));
                // Each choice variant is independently optional (0..1).
                render_field(
                    &doc,
                    &field,
                    &wrap_cardinality(&rust_type(&t.code), 0, "1"),
                    &t.code,
                    min,
                    &max,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    let inner = match element.r#type.as_deref().and_then(<[ElementType]>::first) {
        Some(t) => rust_type(&t.code),
        // No declared type (e.g. a `contentReference` backbone) => opaque value.
        None => String::from("::serde_json::Value"),
    };
    let fhir_type = element
        .r#type
        .as_deref()
        .and_then(<[ElementType]>::first)
        .map_or("?", |t| t.code.as_str());
    render_field(
        &doc,
        &escape_field_name(tail),
        &wrap_cardinality(&inner, min, &max),
        fhir_type,
        min,
        max_display(&max),
    )
}

/// Render a single `/// doc` + `name: Type, // fhir [min..max]` field.
fn render_field(
    doc: &str,
    field: &str,
    rust_type: &str,
    fhir_type: &str,
    min: u32,
    max: &str,
) -> String {
    format!(
        "{RUST_ATTRIBUTE_INDENT}/// {doc}\n{RUST_ATTRIBUTE_INDENT}pub {field}: {rust_type}, // {fhir_type} [{min}..{max}]\n",
    )
}

/// The element's `(min, max)` cardinality, defaulting to `0..1`.
fn cardinality(element: &Element) -> (u32, String) {
    let min = element.min.unwrap_or(0);
    let max = element.max.clone().unwrap_or_else(|| String::from("1"));
    (min, max)
}

/// Normalize a raw `max` for display in the trailing comment.
fn max_display(max: &str) -> &str {
    max
}

/// Wrap a Rust type in `Option` / `Vec` per FHIR cardinality.
///
/// - `0..1` => `Option<T>`
/// - `1..1` => `T`
/// - `0..*` (or n>1) => `Option<Vec<T>>`
/// - `1..*` (or n>1) => `Vec<T>`
fn wrap_cardinality(inner: &str, min: u32, max: &str) -> String {
    let is_list = max == "*" || max.parse::<u32>().is_ok_and(|n| n > 1);
    match (is_list, min) {
        (true, 0) => format!("Option<Vec<{inner}>>"),
        (true, _) => format!("Vec<{inner}>"),
        (false, 0) => format!("Option<{inner}>"),
        (false, _) => inner.to_string(),
    }
}

/// Map a FHIR type code to its Rust type in this crate's `types` module.
///
/// FHIRPath system primitives (e.g. `http://hl7.org/fhirpath/System.String`)
/// map by their trailing type name. All other codes map to `types::{Pascal}`.
fn rust_type(code: &str) -> String {
    let name = code.rsplit_once("System.").map_or(code, |(_, name)| name);
    format!("types::{}", name.to_case(Case::Pascal))
}

/// Convert a FHIR element name to a snake_case Rust field name, escaping Rust
/// reserved words as raw identifiers.
fn escape_field_name(name: &str) -> String {
    let snake = name.to_case(Case::Snake);
    if RUST_KEYWORDS.contains(&snake.as_str()) {
        format!("r#{snake}")
    } else {
        snake
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn element(path: &str, code: Option<&str>, min: u32, max: &str) -> Element {
        Element {
            path: path.into(),
            short: Some(String::from("Short comment")),
            min: Some(min),
            max: Some(max.into()),
            r#type: code.map(|c| {
                vec![ElementType {
                    code: c.into(),
                    ..ElementType::default()
                }]
            }),
            ..Element::default()
        }
    }

    #[test]
    fn root_element_is_skipped() {
        let actual = element_into_rust_struct_attribute(&element("Annotation", None, 0, "1"));
        assert_eq!(actual, "");
    }

    #[test]
    fn optional_scalar() {
        let actual = element_into_rust_struct_attribute(&element(
            "Annotation.time",
            Some("dateTime"),
            0,
            "1",
        ));
        assert_eq!(
            actual,
            "    /// Short comment\n    pub time: Option<types::DateTime>, // dateTime [0..1]\n"
        );
    }

    #[test]
    fn required_scalar() {
        let actual = element_into_rust_struct_attribute(&element(
            "Annotation.text",
            Some("markdown"),
            1,
            "1",
        ));
        assert_eq!(
            actual,
            "    /// Short comment\n    pub text: types::Markdown, // markdown [1..1]\n"
        );
    }

    #[test]
    fn optional_list() {
        let actual =
            element_into_rust_struct_attribute(&element("HumanName.given", Some("string"), 0, "*"));
        assert_eq!(
            actual,
            "    /// Short comment\n    pub given: Option<Vec<types::String>>, // string [0..*]\n"
        );
    }

    #[test]
    fn keyword_field_is_raw_identifier() {
        let actual =
            element_into_rust_struct_attribute(&element("Reference.type", Some("uri"), 0, "1"));
        assert!(
            actual.contains("pub r#type: Option<types::Uri>,"),
            "{actual}"
        );
    }

    #[test]
    fn system_primitive_maps_by_trailing_name() {
        let actual = element_into_rust_struct_attribute(&element(
            "Element.id",
            Some("http://hl7.org/fhirpath/System.String"),
            0,
            "1",
        ));
        assert!(
            actual.contains("pub id: Option<types::String>,"),
            "{actual}"
        );
    }

    #[test]
    fn choice_expands_to_one_field_per_type() {
        let mut e = element("Annotation.author[x]", None, 0, "1");
        e.r#type = Some(vec![
            ElementType {
                code: "Reference".into(),
                ..ElementType::default()
            },
            ElementType {
                code: "string".into(),
                ..ElementType::default()
            },
        ]);
        let actual = element_into_rust_struct_attribute(&e);
        assert!(
            actual.contains("pub author_reference: Option<types::Reference>,"),
            "{actual}"
        );
        assert!(
            actual.contains("pub author_string: Option<types::String>,"),
            "{actual}"
        );
    }
}
