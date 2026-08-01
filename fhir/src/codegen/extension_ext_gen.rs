//! Generating the `extension_ext` module.
//!
//! The [`ExtensionExt`] accessors are implemented for every type that carries an
//! `extension` (or `modifierExtension`) list, which is most of the model. That
//! list changes with every release, and a hand-maintained one silently rots as
//! resources are added or removed, so it is derived from the same plans that
//! produced the types.
//!
//! The traits themselves are repeated per release rather than shared, because
//! `Extension` is a different Rust type in each one.
//!
//! [`ExtensionExt`]: crate::r5::extension_ext::ExtensionExt

use std::fmt::Write as _;

use super::plan::TypePlan;
use super::version::Version;

/// Render `src/<release>/extension_ext.rs`.
#[must_use]
pub fn render(types: &[TypePlan], resources: &[TypePlan], version: Version) -> String {
    let label = version.label();
    let module = version.module();

    // `Extension.url` is a `uri` in R3 but a `string` in R4/R5, so the doc
    // example and the test helper have to name whichever type this release
    // actually generated.
    let url_type = types
        .iter()
        .find(|p| p.type_name == "Extension")
        .and_then(|p| p.structs.iter().find(|s| s.is_root))
        .and_then(|s| s.fields.iter().find(|f| f.ident == "url"))
        .map_or("types::String", |f| f.inner_type.as_str())
        .to_string();
    let url_ctor = url_type
        .strip_prefix("types::")
        .unwrap_or(&url_type)
        .to_string();

    let mut out = format!(
        "//! Ergonomic extension accessors: the [`ExtensionExt`] and\n\
         //! [`ModifierExtensionExt`] traits.\n\
         //!\n\
         //! FHIR extensions are a `Vec<Extension>` keyed by `url`. These traits add the\n\
         //! everyday operations — find an extension by url, iterate all with a url, and\n\
         //! set/add — to every {label} resource and datatype that carries extensions.\n\
         //!\n\
         //! ```\n\
         //! use fhir::{module}::resources::Patient;\n\
         //! use fhir::{module}::types::{{Extension, {url_ctor}}};\n\
         //! use fhir::{module}::extension_ext::ExtensionExt;\n\
         //!\n\
         //! let mut patient = Patient::default();\n\
         //! patient.set_extension(Extension {{\n\
         //!     url: {url_ctor}(\"http://example.org/eye-color\".to_string()),\n\
         //!     ..Default::default()\n\
         //! }});\n\
         //! assert!(patient.extension(\"http://example.org/eye-color\").is_some());\n\
         //! assert!(patient.extension(\"http://other\").is_none());\n\
         //! ```\n\
         \n\
         use crate::{module}::types::Extension;\n\
         \n"
    );

    out.push_str(TRAITS);

    let with_extension = names_with_field(types, resources, "extension", version);
    let with_modifier = names_with_field(types, resources, "modifier_extension", version);

    out.push_str("\nimpl_has_extension!(\n");
    for name in &with_extension {
        let _ = writeln!(out, "    {name},");
    }
    out.push_str(");\n");

    out.push_str("\nimpl_has_modifier_extension!(\n");
    for name in &with_modifier {
        let _ = writeln!(out, "    {name},");
    }
    out.push_str(");\n");

    out.push_str(&tests(module, &url_ctor));
    out
}

/// The trait definitions and the two implementing macros, which are identical
/// in every release apart from the `Extension` type they are written against.
const TRAITS: &str = r"/// Types that carry a FHIR `extension` list.
pub trait HasExtension {
    /// The extensions as a slice (empty if none).
    fn extension_slice(&self) -> &[Extension];
    /// Mutable access to the underlying `Vec<Extension>`.
    fn extension_mut(&mut self) -> &mut Vec<Extension>;
}

/// Ergonomic accessors over [`HasExtension`]. Blanket-implemented.
pub trait ExtensionExt: HasExtension {
    /// The first extension with the given `url`, if any.
    fn extension(&self, url: &str) -> Option<&Extension> {
        self.extension_slice().iter().find(|e| e.url.0 == url)
    }
    /// Every extension with the given `url`.
    fn extensions(&self, url: &str) -> Vec<&Extension> {
        self.extension_slice().iter().filter(|e| e.url.0 == url).collect()
    }
    /// Set an extension, replacing any existing ones with the same `url`.
    fn set_extension(&mut self, ext: Extension) {
        let url = ext.url.0.clone();
        let list = self.extension_mut();
        list.retain(|e| e.url.0 != url);
        list.push(ext);
    }
    /// Append an extension without removing existing ones of the same `url`.
    fn add_extension(&mut self, ext: Extension) {
        self.extension_mut().push(ext);
    }
}

impl<T: HasExtension + ?Sized> ExtensionExt for T {}

/// Types that carry a FHIR `modifierExtension` list (resources and backbones).
pub trait HasModifierExtension {
    /// The modifier extensions as a slice (empty if none).
    fn modifier_extension_slice(&self) -> &[Extension];
    /// Mutable access to the underlying `Vec<Extension>`.
    fn modifier_extension_mut(&mut self) -> &mut Vec<Extension>;
}

/// Ergonomic accessors over [`HasModifierExtension`]. Blanket-implemented.
pub trait ModifierExtensionExt: HasModifierExtension {
    /// The first modifier extension with the given `url`, if any.
    fn modifier_extension(&self, url: &str) -> Option<&Extension> {
        self.modifier_extension_slice().iter().find(|e| e.url.0 == url)
    }
    /// Set a modifier extension, replacing any with the same `url`.
    fn set_modifier_extension(&mut self, ext: Extension) {
        let url = ext.url.0.clone();
        let list = self.modifier_extension_mut();
        list.retain(|e| e.url.0 != url);
        list.push(ext);
    }
}

impl<T: HasModifierExtension + ?Sized> ModifierExtensionExt for T {}

macro_rules! impl_has_extension {
    ($($t:ty),* $(,)?) => { $(
        impl HasExtension for $t {
            fn extension_slice(&self) -> &[Extension] {
                &self.extension
            }
            fn extension_mut(&mut self) -> &mut Vec<Extension> {
                &mut self.extension
            }
        }
    )* };
}

macro_rules! impl_has_modifier_extension {
    ($($t:ty),* $(,)?) => { $(
        impl HasModifierExtension for $t {
            fn modifier_extension_slice(&self) -> &[Extension] {
                &self.modifier_extension
            }
            fn modifier_extension_mut(&mut self) -> &mut Vec<Extension> {
                &mut self.modifier_extension
            }
        }
    )* };
}
";

/// Every type whose root struct has a repeating field with the given name, as a
/// fully qualified path.
///
/// Only root structs are listed: a backbone element's extensions are reached
/// through its parent, and naming every backbone would multiply the list
/// several-fold for little gain.
fn names_with_field(
    types: &[TypePlan],
    resources: &[TypePlan],
    field: &str,
    version: Version,
) -> Vec<String> {
    let module = version.module();
    let mut out = Vec::new();
    for (group, plans) in [("types", types), ("resources", resources)] {
        for plan in plans {
            let Some(root) = plan.structs.iter().find(|s| s.is_root) else {
                continue;
            };
            if root.fields.iter().any(|f| f.ident == field) {
                out.push(format!("crate::{module}::{group}::{}", plan.type_name));
            }
        }
    }
    out
}

/// The module's own tests, which check the accessors against a real resource.
///
/// `url_ctor` is the bare name of the type this release gives `Extension.url`
/// (`String`, `Uri`, …).
fn tests(module: &str, url_ctor: &str) -> String {
    format!(
        "\n#[cfg(test)]\n\
         mod tests {{\n\
         \x20   use super::*;\n\
         \x20   use crate::{module}::resources::Patient;\n\
         \n\
         \x20   fn extension(url: &str) -> Extension {{\n\
         \x20       Extension {{\n\
         \x20           url: crate::{module}::types::{url_ctor}(url.to_string()),\n\
         \x20           ..Default::default()\n\
         \x20       }}\n\
         \x20   }}\n\
         \n\
         \x20   #[test]\n\
         \x20   fn set_replaces_and_add_appends() {{\n\
         \x20       let mut patient = Patient::default();\n\
         \x20       patient.set_extension(extension(\"http://example.org/a\"));\n\
         \x20       patient.set_extension(extension(\"http://example.org/a\"));\n\
         \x20       assert_eq!(patient.extensions(\"http://example.org/a\").len(), 1);\n\
         \n\
         \x20       patient.add_extension(extension(\"http://example.org/a\"));\n\
         \x20       assert_eq!(patient.extensions(\"http://example.org/a\").len(), 2);\n\
         \x20       assert!(patient.extension(\"http://example.org/missing\").is_none());\n\
         \x20   }}\n\
         \n\
         \x20   #[test]\n\
         \x20   fn modifier_extensions_are_separate() {{\n\
         \x20       let mut patient = Patient::default();\n\
         \x20       patient.set_modifier_extension(extension(\"http://example.org/m\"));\n\
         \x20       assert!(patient.modifier_extension(\"http://example.org/m\").is_some());\n\
         \x20       assert!(patient.extension(\"http://example.org/m\").is_none());\n\
         \x20   }}\n\
         }}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::plan::{Context, plan_type};
    use crate::codegen::spec::StructureDefinition;

    fn plan(json: ::serde_json::Value) -> TypePlan {
        let sd: StructureDefinition = ::serde_json::from_value(json).unwrap();
        plan_type(&sd, &Context::default()).unwrap()
    }

    #[test]
    fn lists_only_the_types_that_carry_extensions() {
        let types = vec![plan(::serde_json::json!({
            "name": "Period", "type": "Period", "kind": "complex-type", "url": "u",
            "snapshot": { "element": [
                { "path": "Period" },
                { "path": "Period.extension", "min": 0, "max": "*",
                  "type": [{ "code": "Extension" }] },
            ]}
        }))];
        let resources = vec![
            plan(::serde_json::json!({
                "name": "Patient", "type": "Patient", "kind": "resource", "url": "u",
                "snapshot": { "element": [
                    { "path": "Patient" },
                    { "path": "Patient.extension", "min": 0, "max": "*",
                      "type": [{ "code": "Extension" }] },
                    { "path": "Patient.modifierExtension", "min": 0, "max": "*",
                      "type": [{ "code": "Extension" }] },
                ]}
            })),
            plan(::serde_json::json!({
                "name": "Binary", "type": "Binary", "kind": "resource", "url": "u",
                "snapshot": { "element": [{ "path": "Binary" }] }
            })),
        ];

        let out = render(&types, &resources, Version::R4);
        assert!(out.contains("crate::r4::types::Period,"));
        assert!(out.contains("crate::r4::resources::Patient,"));
        // A type with no extension list is not implemented for.
        assert!(!out.contains("crate::r4::resources::Binary,"));
        // Only Patient has modifierExtension.
        let modifier = out.split("impl_has_modifier_extension!").nth(1).unwrap();
        assert!(modifier.contains("crate::r4::resources::Patient,"));
        assert!(!modifier.contains("crate::r4::types::Period,"));
    }

    #[test]
    fn the_test_helper_names_this_releases_url_type() {
        // R3 types `Extension.url` as a `uri`; R4 and R5 as a `string`.
        let extension = plan(::serde_json::json!({
            "name": "Extension", "type": "Extension", "kind": "complex-type", "url": "u",
            "snapshot": { "element": [
                { "path": "Extension" },
                { "path": "Extension.url", "min": 1, "max": "1",
                  "type": [{ "code": "uri" }] },
            ]}
        }));
        let out = render(&[extension], &[], Version::R3);
        assert!(
            out.contains("url: crate::r3::types::Uri(url.to_string())"),
            "{out}"
        );
        // The module-doc example must agree with it.
        assert!(
            out.contains("//! use fhir::r3::types::{Extension, Uri};"),
            "{out}"
        );
        assert!(out.contains("//!     url: Uri(\"http://example.org/eye-color\".to_string()),"));

        // With no Extension in the plan set, the common case is assumed.
        let out = render(&[], &[], Version::R4);
        assert!(out.contains("url: crate::r4::types::String(url.to_string())"));
    }

    #[test]
    fn emits_the_traits_and_a_release_scoped_import() {
        let out = render(&[], &[], Version::R4);
        assert!(out.contains("use crate::r4::types::Extension;"));
        assert!(out.contains("use fhir::r4::types::{Extension, String};"));
        assert!(out.contains("pub trait ExtensionExt: HasExtension {"));
        assert!(out.contains("impl<T: HasExtension + ?Sized> ExtensionExt for T {}"));
        assert!(out.contains("use crate::r4::resources::Patient;"));
    }
}
