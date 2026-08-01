//! Generating the `meta` element table.
//!
//! `meta` is the model's memory of what the specification said: per-element
//! cardinality, bindings, choice types, reference targets, and summary
//! membership. The generated model cannot carry all of that in its types — a
//! bare `Vec<T>` does not say whether FHIR wrote `0..*` or `1..*`, and a
//! `Reference` does not say which resources it may point at — so the table is
//! what `#[derive(Validate)]` and the summary view consult at run time.
//!
//! One table is produced per release into `src/<release>/meta/generated.rs`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use super::spec::{self, ElementDefinition};
use super::version::Version;

/// Collect every element of every datatype and resource, keyed by FHIR path.
///
/// Resources are read before datatypes and the first occurrence of a path wins,
/// which matters only for the handful of paths both bundles define.
pub fn collect(version: Version) -> std::io::Result<BTreeMap<String, ElementDefinition>> {
    let mut table: BTreeMap<String, ElementDefinition> = BTreeMap::new();
    for bundle in [version.resources_bundle(), version.types_bundle()] {
        for sd in spec::read_structure_definitions(&bundle)? {
            let Some(snapshot) = sd.snapshot else {
                continue;
            };
            for element in snapshot.element {
                // The root element carries the type's own cardinality, which is
                // not a fact about any field.
                if !element.path.contains('.') {
                    continue;
                }
                table.entry(element.path.clone()).or_insert(element);
            }
        }
    }
    Ok(table)
}

/// Render `src/<release>/meta/generated.rs` from a collected table.
#[must_use]
pub fn render(table: &BTreeMap<String, ElementDefinition>, version: Version) -> String {
    let label = version.label();
    let module = version.module();
    let mut out = format!(
        "//! Generated element-metadata table — DO NOT EDIT.\n\
         //!\n\
         //! Produced from the FHIR {label} specification JSON by `crate::codegen::meta_gen`.\n\
         //! Regenerate with `cargo run -- {module}`.\n\
         \n\
         use crate::meta::{{BindingMeta, BindingStrength, ElementMeta, TypeRef}};\n\
         \n\
         pub(super) static ELEMENTS: &[ElementMeta] = &[\n"
    );

    for (path, element) in table {
        let _ = writeln!(
            out,
            "    ElementMeta {{ path: {path:?}, min: {}, max: {:?}, is_summary: {}, binding: {}, types: {}, content_reference: {} }},",
            element.min,
            element.max.as_deref().unwrap_or("1"),
            element.is_summary.unwrap_or(false),
            render_binding(element),
            render_types(element),
            element
                .content_reference_path()
                .map_or_else(|| "None".to_string(), |p| format!("Some({p:?})")),
        );
    }

    out.push_str("];\n");
    out
}

/// Render an element's binding as a Rust literal.
fn render_binding(element: &ElementDefinition) -> String {
    let Some(binding) = &element.binding else {
        return "None".to_string();
    };
    let strength = match binding.strength.as_str() {
        "required" => "Required",
        "extensible" => "Extensible",
        "preferred" => "Preferred",
        _ => "Example",
    };
    let value_set = binding
        .value_set()
        .map_or_else(|| "None".to_string(), |vs| format!("Some({vs:?})"));
    format!("Some(BindingMeta {{ strength: BindingStrength::{strength}, value_set: {value_set} }})")
}

/// Render an element's allowed types as a Rust literal.
fn render_types(element: &ElementDefinition) -> String {
    let entries: Vec<String> = element
        .types
        .iter()
        .map(|t| {
            let profiles: Vec<String> = t
                .target_profiles()
                .iter()
                .map(|p| format!("{p:?}"))
                .collect();
            format!(
                "TypeRef {{ code: {:?}, target_profiles: &[{}] }}",
                t.code,
                profiles.join(", ")
            )
        })
        .collect();
    format!("&[{}]", entries.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn element(json: ::serde_json::Value) -> ElementDefinition {
        ::serde_json::from_value(json).unwrap()
    }

    fn table(elements: Vec<ElementDefinition>) -> BTreeMap<String, ElementDefinition> {
        elements.into_iter().map(|e| (e.path.clone(), e)).collect()
    }

    #[test]
    fn renders_a_sorted_static_table() {
        let out = render(
            &table(vec![
                element(
                    ::serde_json::json!({ "path": "Patient.active", "min": 0, "max": "1",
                    "isSummary": true, "type": [{ "code": "boolean" }] }),
                ),
                element(
                    ::serde_json::json!({ "path": "Patient.name", "min": 0, "max": "*",
                    "type": [{ "code": "HumanName" }] }),
                ),
            ]),
            Version::R4,
        );
        assert!(out.contains("pub(super) static ELEMENTS: &[ElementMeta] = &["));
        let active = out.find("Patient.active").unwrap();
        let name = out.find("Patient.name").unwrap();
        assert!(active < name, "entries must be sorted for binary search");
        assert!(out.contains(
            r#"ElementMeta { path: "Patient.active", min: 0, max: "1", is_summary: true, binding: None, types: &[TypeRef { code: "boolean", target_profiles: &[] }], content_reference: None }"#
        ));
    }

    #[test]
    fn a_content_reference_is_carried_into_the_table() {
        // Recursive backbones are the reason the field exists: without it the
        // cross-release converter cannot tell where `item.item` re-enters, and
        // reports every element of every nested item as missing.
        let out = render(
            &table(vec![element(::serde_json::json!({
                "path": "Questionnaire.item.item", "min": 0, "max": "*",
                "contentReference": "#Questionnaire.item"
            }))]),
            Version::R4,
        );
        assert!(
            out.contains(r#"content_reference: Some("Questionnaire.item")"#),
            "the leading '#' is stripped, leaving a path that matches the table: {out}"
        );
    }

    #[test]
    fn bindings_and_targets_survive() {
        let out = render(
            &table(vec![element(::serde_json::json!({
                "path": "Observation.status", "min": 1, "max": "1",
                "type": [{ "code": "code" }],
                "binding": { "strength": "required",
                             "valueSet": "http://hl7.org/fhir/ValueSet/observation-status|4.0.1" }
            }))]),
            Version::R4,
        );
        assert!(out.contains("BindingStrength::Required"));
        assert!(out.contains("http://hl7.org/fhir/ValueSet/observation-status|4.0.1"));

        let out = render(
            &table(vec![element(::serde_json::json!({
                "path": "Observation.subject", "min": 0, "max": "1",
                "type": [{ "code": "Reference",
                           "targetProfile": ["http://hl7.org/fhir/StructureDefinition/Patient"] }]
            }))]),
            Version::R4,
        );
        assert!(
            out.contains(
                r#"target_profiles: &["http://hl7.org/fhir/StructureDefinition/Patient"]"#
            )
        );
    }

    #[test]
    fn an_unknown_strength_degrades_to_example() {
        let out = render(
            &table(vec![element(::serde_json::json!({
                "path": "X.y", "min": 0, "max": "1", "type": [{ "code": "code" }],
                "binding": { "strength": "whatever" }
            }))]),
            Version::R4,
        );
        assert!(out.contains("BindingStrength::Example"));
        assert!(out.contains("value_set: None"));
    }
}
