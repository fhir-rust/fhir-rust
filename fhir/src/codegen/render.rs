//! Rendering a [`TypePlan`] as Rust source.
//!
//! Everything here is text assembly: the decisions were already made in
//! [`super::plan`]. The output must match `AGENTS/conventions.md` exactly — the
//! same derives in the same order, `skip_serializing_none`, camelCase renaming,
//! `Vec` defaults, primitive-extension siblings, and a round-trip doctest — so
//! that a generated release module is indistinguishable in style from the
//! hand-tended one.

use std::fmt::Write as _;

use super::naming;
use super::plan::{ChoicePlan, FieldPlan, StructPlan, TypePlan, Wrapper};
use super::primitives;
use super::version::Version;

/// The derives every model struct carries, in the mandated order.
const STRUCT_DERIVES: &str =
    "Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate";

/// The same, for a struct that cannot have a default (it has a `1..*` field).
const STRUCT_DERIVES_NO_DEFAULT: &str =
    "Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate";

/// Render the complete Rust module for one datatype or resource.
#[must_use]
/// The `Version:` doc line, or nothing at all when the specification does
/// not state one.
///
/// The obvious `"//! Version: {version}"` leaves a trailing space when the
/// version is empty; rustfmt strips it, so every regeneration would rewrite
/// ~280 generated files with a whitespace-only diff.
pub(crate) fn doc_version(version: &str) -> String {
    if version.is_empty() {
        String::new()
    } else {
        format!(" Version: {version}")
    }
}

pub fn render_type(plan: &TypePlan, version: Version) -> String {
    let mut out = String::new();
    out.push_str(&module_header(plan, version));
    out.push_str(&imports(plan, version));

    for structure in &plan.structs {
        out.push('\n');
        out.push_str(&render_struct(structure, plan, version));
    }

    for choice in &plan.choices {
        out.push('\n');
        out.push_str(&render_choice(choice, version));
    }

    if let Some(root) = plan.structs.first()
        && root.has_default
    {
        out.push('\n');
        out.push_str(&render_tests(&root.name));
    }
    out
}

/// The `//!` module header: the FHIR name, canonical URL, version, summary, and
/// a link to the published specification.
fn module_header(plan: &TypePlan, version: Version) -> String {
    let mut out = format!("//! {}\n//!\n", plan.type_name);
    let _ = write!(out, "//! URL: {}\n//!\n", plan.url);
    // No trailing space when the version is empty: rustfmt strips it, so
    // emitting it would make every regeneration churn ~280 files.
    let _ = write!(out, "//!{}\n//!\n", doc_version(&plan.version));
    if !plan.short.is_empty() {
        out.push_str(&naming::module_doc_comment(&plan.short));
        out.push_str("//!\n");
    }
    let _ = writeln!(
        out,
        "//! FHIR {}: <{}>",
        version.label(),
        version.spec_url()
    );
    out
}

/// The `use` block. Generated modules import their release's datatypes as
/// `types`, so every rendered type name is release-agnostic.
fn imports(plan: &TypePlan, version: Version) -> String {
    let module = version.module();
    let derives = if plan.structs.iter().any(|s| s.is_root) {
        "use fhir_derive_macros::{Builder, Validate};\n"
    } else {
        "use fhir_derive_macros::Validate;\n"
    };
    format!(
        "\n// The `types` import is unused by a handful of types that have only \
         primitive fields.\n#![allow(unused_imports)]\n\n\
         use crate::{module}::types;\n\
         use ::serde::{{Deserialize, Serialize}};\n{derives}"
    )
}

/// Render one struct: the type's root, or one of its backbone elements.
fn render_struct(structure: &StructPlan, plan: &TypePlan, version: Version) -> String {
    let mut out = String::new();

    let doc = if structure.is_root && !plan.description.is_empty() {
        &plan.description
    } else {
        &structure.doc
    };
    out.push_str(&naming::doc_comment(doc, ""));
    out.push_str(&example_doctest(structure, plan, version));

    out.push_str("#[serde_with::skip_serializing_none]\n");
    let derives = if structure.has_default {
        STRUCT_DERIVES
    } else {
        STRUCT_DERIVES_NO_DEFAULT
    };
    let builder = if structure.is_root { ", Builder" } else { "" };
    let _ = writeln!(out, "#[derive({derives}{builder})]");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str(&version_attribute(version));
    let _ = writeln!(out, "pub struct {} {{", structure.name);

    for (index, field) in structure.fields.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        out.push_str(&render_field(field));
    }

    out.push_str("}\n");
    out
}

/// The `# Examples` doctest for a generated type.
///
/// # Why it sets a field
///
/// This used to round-trip `Type::default()` and assert the result equalled
/// the input. For most types that serialized to `{}` — two bytes — so the
/// assertion held no matter what the struct contained. Roughly two thousand
/// doctests across the generated releases were checking that an empty object
/// survives a round trip: no field name, no serde rename, no type, and no
/// cardinality was exercised by any of them.
///
/// Where the type has an optional primitive field, the example now sets it
/// and asserts the **wire name** it serializes to. That is the part a server
/// sees and the part a wrong `rename_all` or a mistyped field would break,
/// and it doubles as the thing a reader actually wants from an example:
/// how to put a value in.
///
/// A type with a `1..*` field has no default, so its example is `ignore`d
/// rather than dropped: the import line still documents where the type lives.
fn example_doctest(structure: &StructPlan, plan: &TypePlan, version: Version) -> String {
    let name = &structure.name;
    let has_default = structure.has_default;
    let module = version.module();
    let group = if plan.is_resource {
        "resources"
    } else {
        "types"
    };
    let fence = if has_default { "```" } else { "```ignore" };

    // Prefer an optional primitive: it can be set without constructing a
    // nested type, and it has a known sample value and JSON rendering.
    let populated = has_default
        .then(|| {
            // Every resource carries `id`, `implicitRules` and `language`,
            // so choosing one of those makes every example identical and
            // shows the reader nothing about *this* type. Prefer a field
            // whose wire name differs from its Rust identifier: that is the
            // case a broken `rename_all` would break, so the assertion can
            // actually fail. Then any type-specific field, then whatever
            // is left.
            const HEADER: [&str; 3] = ["id", "implicitRules", "language"];
            let candidates: Vec<(&FieldPlan, &'static primitives::Primitive)> = structure
                .fields
                .iter()
                .filter(|f| f.wrapper == Wrapper::Option && !f.boxed)
                .filter_map(|f| {
                    let code = f.inner_type.strip_prefix("types::")?;
                    let prim = primitives::by_rust_name(code)?;
                    // `decimal` has a private field, so an example cannot
                    // construct one; such a type falls back to the default
                    // round trip rather than emitting code that will not
                    // compile.
                    prim.is_directly_constructible().then_some((f, prim))
                })
                .collect();
            candidates
                .iter()
                .find(|(f, _)| !HEADER.contains(&f.wire.as_str()) && f.wire != f.ident)
                .or_else(|| {
                    candidates
                        .iter()
                        .find(|(f, _)| !HEADER.contains(&f.wire.as_str()))
                })
                .or_else(|| candidates.first())
                .copied()
        })
        .flatten();

    let body = match populated {
        Some((field, prim)) => format!(
            "/// use fhir::{module}::{group}::{}::{name};\n             /// use fhir::{module}::types;\n             ///\n             /// let value = {name} {{\n             ///     {}: Some(types::{}({})),\n             ///     ..Default::default()\n             /// }};\n             /// let json = ::serde_json::to_value(&value).unwrap();\n             /// // `{}` is the name this serializes to on the wire.\n             /// assert_eq!(json[\"{}\"], ::serde_json::json!({}));\n             ///\n             /// let back: {name} = ::serde_json::from_value(json).unwrap();\n             /// assert_eq!(value, back);\n",
            plan.module,
            field.ident,
            naming::pascal(prim.code),
            prim.sample,
            field.wire,
            field.wire,
            prim.sample_json,
        ),
        // No optional primitive to set — a type whose fields are all complex,
        // required, or repeating. The round trip is all there is to show.
        None => format!(
            "/// use fhir::{module}::{group}::{}::{name};\n             ///\n             /// let value = {name}::default();\n             /// let json = ::serde_json::to_value(&value).unwrap();\n             /// let back: {name} = ::serde_json::from_value(json).unwrap();\n             /// assert_eq!(value, back);\n",
            plan.module,
        ),
    };

    format!(
        "///\n\
         /// # Examples\n\
         ///\n\
         /// {fence}\n\
         {body}\
         /// ```\n"
    )
}

/// The `#[fhir_version(…)]` selector the derive macros read.
///
/// R5 is the macros' default, so only other releases need to say so.
#[must_use]
pub fn version_attribute(version: Version) -> String {
    match version {
        Version::R5 => String::new(),
        other => format!("#[fhir_version(\"{}\")]\n", other.module()),
    }
}

/// Render one field, plus its primitive-extension sibling when it has one.
fn render_field(field: &FieldPlan) -> String {
    let mut out = String::new();
    out.push_str(&naming::doc_comment(&field.doc, "    "));

    if let Some(choice) = &field.choice {
        // A choice enum's variants carry the FHIR keys, so it is flattened onto
        // the parent object rather than nested under the field name.
        let cardinality = if field.min >= 1 {
            format!(
                "    /// The `{}` choice element ({}..{}); see [`{choice}`]. It is \
                 `Option` even though the specification makes it mandatory, because \
                 a choice enum has no default.\n",
                field.path, field.min, field.max
            )
        } else {
            format!(
                "    /// The `{}` choice element ({}..{}); see [`{choice}`].\n",
                field.path, field.min, field.max
            )
        };
        out.push_str(&cardinality);
        out.push_str("    #[serde(flatten)]\n");
        let _ = writeln!(out, "    pub {}: Option<{choice}>,", field.ident);
        return out;
    }

    let inner = if field.boxed {
        format!("Box<{}>", field.inner_type)
    } else {
        field.inner_type.clone()
    };

    // `rename_all = "camelCase"` covers almost every field, but it cannot
    // reproduce a name with consecutive capitals such as `truthTP`, so those
    // spell their key out.
    if naming::needs_explicit_rename(&field.ident, &field.wire) {
        let _ = writeln!(out, "    #[serde(rename = {:?})]", field.wire);
    }

    match field.wrapper {
        Wrapper::Option => {
            let _ = writeln!(out, "    pub {}: Option<{inner}>,", field.ident);
        }
        Wrapper::Required => {
            let _ = writeln!(out, "    pub {}: {inner},", field.ident);
        }
        Wrapper::Vec => {
            out.push_str("    #[serde(default, skip_serializing_if = \"Vec::is_empty\")]\n");
            let _ = writeln!(out, "    pub {}: Vec<{inner}>,", field.ident);
        }
        Wrapper::Vec1 => {
            let _ = writeln!(out, "    pub {}: ::vec1::Vec1<{inner}>,", field.ident);
        }
    }

    if let Some(sibling) = &field.sibling {
        let _ = write!(
            out,
            "    /// Primitive extension sibling for [`{}`](Self::{}) (FHIR `{}`):\n\
             \x20   /// carries `id` and/or `extension` for the primitive value.\n\
             \x20   #[serde(rename = \"{}\")]\n",
            field.ident, field.ident, sibling.wire, sibling.wire,
        );
        if sibling.is_multiple {
            out.push_str("    #[serde(default, skip_serializing_if = \"Vec::is_empty\")]\n");
            let _ = writeln!(
                out,
                "    pub {}: Vec<Option<types::Element>>,",
                sibling.ident
            );
        } else {
            let _ = writeln!(out, "    pub {}: Option<types::Element>,", sibling.ident);
        }
    }

    out
}

/// Render one `value[x]` choice enum.
fn render_choice(choice: &ChoicePlan, version: Version) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "/// The `{}` choice element (see `spec/11-choice-types.md`).",
        choice.path
    );
    out.push_str(
        "#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]\n",
    );
    out.push_str(&version_attribute(version));
    // The variants hold datatypes of very different sizes; boxing every one of
    // them would cost an allocation on the common small variants.
    out.push_str("#[allow(clippy::large_enum_variant)]\n");
    let _ = writeln!(out, "pub enum {} {{", choice.name);
    for variant in &choice.variants {
        let _ = writeln!(out, "    /// `{}` variant.", variant.key);
        let _ = writeln!(out, "    #[fhir(\"{}\")]", variant.key);
        let _ = writeln!(out, "    {}({}),", variant.name, variant.payload);
    }
    out.push_str("}\n");
    out
}

/// The per-module unit tests: a default value, and a JSON round trip.
fn render_tests(name: &str) -> String {
    format!(
        "#[cfg(test)]\n\
         mod tests {{\n\
         \x20   use super::*;\n\
         \x20   type T = {name};\n\
         \n\
         \x20   #[test]\n\
         \x20   fn test_default() {{\n\
         \x20       let _ = T::default();\n\
         \x20   }}\n\
         \n\
         \x20   #[test]\n\
         \x20   fn test_serde_round_trip() {{\n\
         \x20       let value = T::default();\n\
         \x20       let json = ::serde_json::to_value(&value).expect(\"to_value\");\n\
         \x20       let back: T = ::serde_json::from_value(json).expect(\"from_value\");\n\
         \x20       assert_eq!(value, back);\n\
         \x20   }}\n\
         }}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::plan::{Context, plan_type};
    use crate::codegen::spec::StructureDefinition;

    fn ctx() -> Context {
        Context {
            primitives: ["string", "boolean", "dateTime"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            code_enums: std::collections::BTreeSet::new(),
            module: "r4".to_string(),
        }
    }

    fn sample_plan() -> TypePlan {
        let sd: StructureDefinition = ::serde_json::from_value(::serde_json::json!({
            "name": "Sample", "type": "Sample", "kind": "resource",
            "url": "http://example.org/Sample", "version": "4.0.1",
            "description": "A sample resource.",
            "snapshot": { "element": [
                { "path": "Sample", "short": "A sample" },
                { "path": "Sample.note", "min": 0, "max": "1", "short": "A note",
                  "type": [{ "code": "string" }] },
                { "path": "Sample.tag", "min": 0, "max": "*", "short": "Tags",
                  "type": [{ "code": "CodeableConcept" }] },
                { "path": "Sample.value[x]", "min": 0, "max": "1", "short": "The value",
                  "type": [{ "code": "Quantity" }, { "code": "string" }] },
            ]}
        }))
        .unwrap();
        plan_type(&sd, &ctx()).unwrap()
    }

    #[test]
    fn header_names_the_release() {
        let out = render_type(&sample_plan(), Version::R4);
        assert!(out.starts_with("//! Sample\n"));
        assert!(out.contains("//! URL: http://example.org/Sample"));
        assert!(out.contains("//! Version: 4.0.1"));
        assert!(out.contains("//! FHIR R4: <https://hl7.org/fhir/R4/>"));
        assert!(out.contains("use crate::r4::types;"));
    }

    #[test]
    fn struct_follows_the_conventions() {
        let out = render_type(&sample_plan(), Version::R4);
        assert!(out.contains("#[serde_with::skip_serializing_none]\n"));
        assert!(out.contains(
            "#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]"
        ));
        assert!(out.contains("#[serde(rename_all = \"camelCase\")]"));
        assert!(out.contains("#[fhir_version(\"r4\")]"));
        assert!(out.contains("pub struct Sample {"));
    }

    #[test]
    fn cardinality_and_siblings_render() {
        let out = render_type(&sample_plan(), Version::R4);
        assert!(out.contains("pub note: Option<types::String>,"));
        // A repeating field defaults to empty and is skipped when empty.
        assert!(out.contains("#[serde(default, skip_serializing_if = \"Vec::is_empty\")]"));
        assert!(out.contains("pub tag: Vec<types::CodeableConcept>,"));
        // A primitive field carries its `_note` extension sibling.
        assert!(out.contains("#[serde(rename = \"_note\")]"));
        assert!(out.contains("pub note_ext: Option<types::Element>,"));
    }

    #[test]
    fn names_camel_case_cannot_reproduce_are_renamed() {
        let sd: StructureDefinition = ::serde_json::from_value(::serde_json::json!({
            "name": "Sample", "type": "Sample", "kind": "resource",
            "url": "u", "version": "4.0.1",
            "snapshot": { "element": [
                { "path": "Sample" },
                { "path": "Sample.truthTP", "min": 0, "max": "1",
                  "type": [{ "code": "string" }] },
                { "path": "Sample.plainName", "min": 0, "max": "1",
                  "type": [{ "code": "string" }] },
            ]}
        }))
        .unwrap();
        let out = render_type(&plan_type(&sd, &ctx()).unwrap(), Version::R4);
        assert!(out.contains("#[serde(rename = \"truthTP\")]"));
        assert!(out.contains("pub truth_tp: Option<types::String>,"));
        // Its extension sibling keeps the original spelling too.
        assert!(out.contains("#[serde(rename = \"_truthTP\")]"));
        // An ordinary name is left to `rename_all`.
        assert!(!out.contains("#[serde(rename = \"plainName\")]"));
    }

    #[test]
    fn choice_field_is_flattened_and_enum_is_emitted() {
        let out = render_type(&sample_plan(), Version::R4);
        assert!(out.contains("#[serde(flatten)]"));
        assert!(out.contains("pub value: Option<SampleValue>,"));
        assert!(out.contains("pub enum SampleValue {"));
        assert!(out.contains("#[fhir(\"valueQuantity\")]"));
        assert!(out.contains("Quantity(Box<types::Quantity>),"));
        assert!(out.contains("String(crate::r4::choice::Primitive<types::String>),"));
    }

    #[test]
    fn r5_needs_no_version_attribute() {
        let out = render_type(&sample_plan(), Version::R5);
        assert!(!out.contains("#[fhir_version"));
        assert!(out.contains("use crate::r5::types;"));
    }

    #[test]
    fn tests_and_doctest_are_emitted_for_defaultable_types() {
        let out = render_type(&sample_plan(), Version::R4);
        assert!(out.contains("fn test_serde_round_trip()"));
        assert!(out.contains("/// use fhir::r4::resources::sample::Sample;"));
        assert!(!out.contains("```ignore"));
    }

    #[test]
    fn types_without_a_default_skip_the_tests() {
        let sd: StructureDefinition = ::serde_json::from_value(::serde_json::json!({
            "name": "Sample", "type": "Sample", "kind": "resource",
            "url": "u", "version": "4.0.1",
            "snapshot": { "element": [
                { "path": "Sample" },
                { "path": "Sample.item", "min": 1, "max": "*",
                  "type": [{ "code": "CodeableConcept" }] },
            ]}
        }))
        .unwrap();
        let plan = plan_type(&sd, &ctx()).unwrap();
        let out = render_type(&plan, Version::R4);
        assert!(out.contains("pub item: ::vec1::Vec1<types::CodeableConcept>,"));
        assert!(out.contains(
            "#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]"
        ));
        assert!(out.contains("```ignore"));
        assert!(!out.contains("mod tests"));
    }
}
