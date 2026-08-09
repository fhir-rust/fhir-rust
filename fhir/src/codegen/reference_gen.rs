//! The typed `Reference<T>` machinery (T11).
//!
//! `Reference` is the one datatype whose rendered struct is post-processed: it
//! gains a zero-sized compile-time target marker (`Reference<Patient>` and
//! `Reference<Any>` share one wire form) plus the `ResourceType` trait and the
//! `cast`/`into_any`/`resolve` machinery. The hand-written R5 file prototyped
//! this shape; the generator now emits it for every release, and
//! `render_resources_module` emits the per-resource `ResourceType` impls
//! beside the `Resource` enum.

use std::fmt::Write;

use super::plan::TypePlan;
use super::version::Version;

/// Post-process the rendered `Reference` module.
pub fn augment(mut source: String, plan: &TypePlan, version: Version) -> String {
    let module = version.module();

    // Builder does not survive the generic parameter, and a reference never
    // needed one — R5 dropped the derive when the marker landed. Default is
    // implemented by hand below instead of derived: the derive would demand
    // `T: Default`, and a typed target like `Reference<Appointment>` must
    // default fine even when `Appointment` (required fields) cannot.
    source = source.replace(
        "use fhir_derive_macros::{Builder, Validate};",
        "use fhir_derive_macros::Validate;\nuse std::marker::PhantomData;",
    );
    source = source.replace(", Validate, Builder)]", ", Validate)]");
    source = source.replace("#[derive(Debug, Default, Clone,", "#[derive(Debug, Clone,");

    // The generic parameter, defaulting to the untyped target so that a bare
    // `types::Reference` keeps meaning what it always did.
    source = source.replace("pub struct Reference {", "pub struct Reference<T = Any> {");

    // The marker field, before the struct's closing brace — the first
    // column-zero `}` after the opening.
    let open = source
        .find("pub struct Reference<T = Any> {")
        .expect("the Reference struct");
    let close = source[open..].find("\n}\n").expect("the struct's end") + open;
    source.insert_str(
        close,
        "\n\n    /// Compile-time marker for the referenced resource type. Zero-sized and not\n\
         \x20   /// serialized; `Reference<Patient>` and `Reference<Any>` share one wire form.\n\
         \x20   ///\n\
         \x20   /// Public only so that callers can use the struct-literal idiom the rest of\n\
         \x20   /// the model documents — `Reference { reference: …, ..Default::default() }`\n\
         \x20   /// needs access to every field. Treat it as an implementation detail.\n\
         \x20   #[doc(hidden)]\n\
         \x20   #[serde(skip)]\n\
         \x20   pub _marker: PhantomData<fn() -> T>,",
    );

    // The machinery, between the struct and the tests module.
    // Every field the renderer emitted, in order — including the `_ext`
    // primitive-extension siblings, which ride on their field's plan rather
    // than being plans of their own. The bool is "defaults to an empty Vec"
    // (everything else on Reference is an Option defaulting to None).
    let mut fields: Vec<(String, bool)> = Vec::new();
    for f in &plan
        .structs
        .iter()
        .find(|s| s.is_root)
        .expect("Reference has a root struct")
        .fields
    {
        let is_vec = matches!(
            f.wrapper,
            super::plan::Wrapper::Vec | super::plan::Wrapper::Vec1
        );
        fields.push((f.ident.clone(), is_vec));
        if f.sibling.is_some() {
            fields.push((format!("{}_ext", f.ident.trim_start_matches("r#")), false));
        }
    }

    let machinery = machinery(&fields, module);

    match source.find("#[cfg(test)]") {
        Some(at) => source.insert_str(at, &format!("{machinery}\n")),
        None => source.push_str(&machinery),
    }
    source
}

/// The bound-free `Default`: the derive would demand `T: Default`, and a
/// typed reference must default fine even when its target resource (required
/// fields) cannot.
fn default_impl(fields: &[(String, bool)]) -> String {
    let mut out = String::from(
        "\nimpl<T> Default for Reference<T> {\n    fn default() -> Self {\n        Reference {\n",
    );
    for (field, is_vec) in fields {
        let value = if *is_vec { "Vec::new()" } else { "None" };
        let _ = writeln!(out, "            {field}: {value},");
    }
    out.push_str("            _marker: PhantomData,\n        }\n    }\n}\n");
    out
}

/// The trait, the untyped target, and the `Default`/`cast`/`into_any`/
/// `resolve` impls.
fn machinery(fields: &[(String, bool)], module: &str) -> String {
    let mut machinery = default_impl(fields);

    let _ = write!(
        machinery,
        "\n/// A marker type naming the resource a [`Reference`] points to.\n\
         ///\n\
         /// Implemented by every resource type (in `resources.rs`, beside the\n\
         /// `Resource` enum) and by [`Any`].\n\
         pub trait ResourceType {{\n\
         \x20   /// The FHIR resource type name (e.g. `\"Patient\"`), or `None` for [`Any`].\n\
         \x20   fn resource_type_name() -> Option<&'static str>;\n\
         }}\n\
         \n\
         /// The untyped reference target: any resource type. This is the default `T`, so\n\
         /// a bare `Reference` is `Reference<Any>` and existing code is unaffected.\n\
         #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]\n\
         pub struct Any;\n\
         \n\
         impl ResourceType for Any {{\n\
         \x20   fn resource_type_name() -> Option<&'static str> {{\n\
         \x20       None\n\
         \x20   }}\n\
         }}\n\
         \n\
         impl<T> Reference<T> {{\n\
         \x20   /// Re-interpret the compile-time target type. The wire form is identical for\n\
         \x20   /// every `T`, so this only changes the phantom marker.\n\
         \x20   #[must_use]\n\
         \x20   pub fn cast<U>(self) -> Reference<U> {{\n\
         \x20       Reference {{\n"
    );
    for (field, _) in fields {
        let _ = writeln!(machinery, "            {field}: self.{field},");
    }
    let _ = write!(
        machinery,
        "            _marker: PhantomData,\n\
         \x20       }}\n\
         \x20   }}\n\
         \n\
         \x20   /// Drop the compile-time target type, yielding an untyped `Reference<Any>`.\n\
         \x20   #[must_use]\n\
         \x20   pub fn into_any(self) -> Reference<Any> {{\n\
         \x20       self.cast()\n\
         \x20   }}\n\
         }}\n\
         \n\
         impl<T: ResourceType> Reference<T> {{\n\
         \x20   /// Resolve this reference within a `Bundle`, returning the matching entry's\n\
         \x20   /// resource JSON.\n\
         \x20   ///\n\
         \x20   /// The reference string is matched against each entry's `fullUrl` or its\n\
         \x20   /// `resourceType/id`. When `T` names a concrete resource type, a candidate\n\
         \x20   /// whose `resourceType` differs is rejected.\n\
         \x20   ///\n\
         \x20   /// ```\n\
         \x20   /// use fhir::{module}::resources::{{Bundle, Patient}};\n\
         \x20   /// use fhir::{module}::types::reference::Reference;\n\
         \x20   ///\n\
         \x20   /// let bundle: Bundle = serde_json::from_value(serde_json::json!({{\n\
         \x20   ///     \"resourceType\": \"Bundle\",\n\
         \x20   ///     \"type\": \"collection\",\n\
         \x20   ///     \"entry\": [\n\
         \x20   ///         {{ \"fullUrl\": \"urn:uuid:pat-1\",\n\
         \x20   ///           \"resource\": {{ \"resourceType\": \"Patient\", \"id\": \"pat-1\" }} }}\n\
         \x20   ///     ]\n\
         \x20   /// }})).unwrap();\n\
         \x20   ///\n\
         \x20   /// let subject: Reference<Patient> = serde_json::from_value(\n\
         \x20   ///     serde_json::json!({{ \"reference\": \"Patient/pat-1\" }})\n\
         \x20   /// ).unwrap();\n\
         \x20   ///\n\
         \x20   /// let resolved = subject.resolve(&bundle).unwrap();\n\
         \x20   /// assert_eq!(resolved[\"resourceType\"], \"Patient\");\n\
         \x20   /// ```\n\
         \x20   #[must_use]\n\
         \x20   pub fn resolve<'b>(\n\
         \x20       &self,\n\
         \x20       bundle: &'b crate::{module}::resources::Bundle,\n\
         \x20   ) -> Option<&'b ::serde_json::Value> {{\n\
         \x20       let want = &self.reference.as_ref()?.0;\n\
         \x20       let expected = T::resource_type_name();\n\
         \x20       for entry in &bundle.entry {{\n\
         \x20           let matches_full_url = entry.full_url.as_ref().is_some_and(|u| &u.0 == want);\n\
         \x20           let resource = entry.resource.as_ref();\n\
         \x20           let rt = resource\n\
         \x20               .and_then(|r| r.get(\"resourceType\"))\n\
         \x20               .and_then(|v| v.as_str());\n\
         \x20           let id = resource.and_then(|r| r.get(\"id\")).and_then(|v| v.as_str());\n\
         \x20           let matches_rel = match (rt, id) {{\n\
         \x20               (Some(rt), Some(id)) => *want == format!(\"{{rt}}/{{id}}\"),\n\
         \x20               _ => false,\n\
         \x20           }};\n\
         \x20           if (matches_full_url || matches_rel)\n\
         \x20               && expected.is_none_or(|want_ty| rt == Some(want_ty))\n\
         \x20           {{\n\
         \x20               return resource;\n\
         \x20           }}\n\
         \x20       }}\n\
         \x20       None\n\
         \x20   }}\n\
         }}\n"
    );
    machinery
}
