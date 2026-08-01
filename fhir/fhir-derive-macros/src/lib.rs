//! Procedural macros for the `fhir` crate.
//!
//! Provides `#[derive(Validate)]`, which generates a recursive
//! `crate::validate::Validate` implementation that validates every field
//! (for structs) or the active variant's data (for enums), prefixing each
//! nested issue's `path` with the field name; `#[derive(FhirChoice)]` for
//! `value[x]` choice enums; and `#[derive(Builder)]`.
//!
//! # Choosing a FHIR release
//!
//! Most of the generated code targets release-independent items in the crate
//! root (`crate::validate`, `crate::builder`). A few pieces are per-release —
//! the `meta` element table, `types::Element`, and `choice::Primitive` — so a
//! type from a release other than the default declares it:
//!
//! ```ignore
//! #[derive(Validate)]
//! #[fhir_version("r4")]
//! pub struct Patient { /* … */ }
//! ```
//!
//! R5 is the default, since it is the release this crate shipped first.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, GenericArgument, Index, LitStr,
    PathArguments, Type,
};

/// The FHIR releases whose model modules the generated code can name.
const KNOWN_VERSIONS: [&str; 5] = ["r2", "r3", "r4", "r5", "r6"];

/// Resolve `#[fhir_version("r4")]` into the release module path `crate::r4`.
///
/// Defaults to `crate::r5` when the attribute is absent.
fn version_path(attrs: &[Attribute]) -> Result<proc_macro2::TokenStream, syn::Error> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("fhir_version")) else {
        return Ok(quote! { crate::r5 });
    };
    let literal = attr.parse_args::<LitStr>()?;
    let version = literal.value();
    if !KNOWN_VERSIONS.contains(&version.as_str()) {
        return Err(syn::Error::new_spanned(
            &literal,
            format!(
                "unknown FHIR version {version:?}; expected one of {}",
                KNOWN_VERSIONS.join(", ")
            ),
        ));
    }
    let ident = syn::Ident::new(&version, literal.span());
    Ok(quote! { crate::#ident })
}

/// Derive a recursive `Validate` implementation.
#[proc_macro_derive(Validate, attributes(fhir_version))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_g, ty_g, where_g) = input.generics.split_for_impl();
    let version = match version_path(&input.attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let body = match &input.data {
        Data::Struct(s) => {
            let stmts = struct_field_stmts(&s.fields, &name.to_string(), &version);
            let invariants = invariant_stmts(&name.to_string(), &s.fields);
            quote! {
                let mut issues = ::std::vec::Vec::new();
                #(#stmts)*
                #invariants
                issues
            }
        }
        Data::Enum(e) => {
            let arms = e.variants.iter().map(|v| {
                let vname = &v.ident;
                match &v.fields {
                    Fields::Unit => quote! { #name::#vname => {} },
                    Fields::Unnamed(f) => {
                        let binds: Vec<syn::Ident> = (0..f.unnamed.len())
                            .map(|i| {
                                syn::Ident::new(&format!("__f{i}"), proc_macro2::Span::call_site())
                            })
                            .collect();
                        let calls = binds.iter().map(|b| {
                            quote! {
                                issues.extend(crate::validate::Validate::validate(#b));
                            }
                        });
                        quote! { #name::#vname( #(#binds),* ) => { #(#calls)* } }
                    }
                    Fields::Named(f) => {
                        let names: Vec<syn::Ident> =
                            f.named.iter().map(|x| x.ident.clone().unwrap()).collect();
                        let calls = names.iter().map(|b| {
                            quote! {
                                issues.extend(crate::validate::Validate::validate(#b));
                            }
                        });
                        quote! { #name::#vname { #(#names),* } => { #(#calls)* } }
                    }
                }
            });
            quote! {
                let mut issues = ::std::vec::Vec::new();
                match self { #(#arms),* }
                issues
            }
        }
        Data::Union(_) => quote! { ::std::vec::Vec::new() },
    };

    quote! {
        impl #impl_g crate::validate::Validate for #name #ty_g #where_g {
            fn validate(&self) -> ::std::vec::Vec<crate::validate::ValidationIssue> {
                #body
            }
        }
    }
    .into()
}

/// The types `qty-3` applies to: `Quantity` and every specialization of it.
///
/// FHIR states the rule on each separately rather than by inheritance, and this
/// crate generates each as its own struct, so the list is spelled out. DSTU2
/// models the specializations as profiles on `Quantity`, which the generator
/// emits as an empty `Age` beside an `AgeQuantity` holding the fields — hence
/// the `Quantity` suffix check as well as the list.
///
/// Every check below is additionally gated on the fields actually being there,
/// so a release that shapes a type differently is skipped rather than failing
/// to compile. `Coding` also has `code` and `system` and must not match, which
/// is why this is not simply "any struct with those two fields".
const QUANTITY_TYPES: &[&str] = &[
    "Quantity",
    "SimpleQuantity",
    "MoneyQuantity",
    "Age",
    "Count",
    "Distance",
    "Duration",
];

/// The named field, if the struct has one.
fn field_named<'a>(fields: &'a Fields, name: &str) -> Option<&'a syn::Field> {
    match fields {
        Fields::Named(f) => f
            .named
            .iter()
            .find(|x| x.ident.as_ref().is_some_and(|i| i == name)),
        _ => None,
    }
}

/// Whether the struct has a field of this name whose outer type is `outer`
/// (`Option` or `Vec`).
///
/// Cardinality differs between releases — `DataRequirement.codeFilter.path` is
/// `0..1` in R4 and `1..1` in R3 — so a check written against one release's
/// shape must confirm the shape before emitting, or it breaks the build of
/// another release rather than simply not applying to it.
fn has_field_of(fields: &Fields, name: &str, outer: &str) -> bool {
    field_named(fields, name).is_some_and(|f| {
        matches!(&f.ty, syn::Type::Path(p)
            if p.path.segments.last().is_some_and(|s| s.ident == outer))
    })
}

/// The `ele-1` check for a struct, if it is a shape that can be empty.
///
/// `ele-1` is `hasValue() or (children().count() > id.count())`: an element
/// passes if it carries a primitive value, or if it has at least one child that
/// is not its `id`. It is the single most restated constraint in FHIR — 8,363
/// of R5's 10,992 constraint occurrences — and it does not need a FHIRPath
/// evaluator *in this representation*, which is the point. `children()` and
/// `id` are the only functions in it, and against a statically typed model
/// "children other than `id`" is just "fields other than `id`".
///
/// The primitives carry their value in a newtype and so always satisfy the
/// first clause; only the complex types can be empty. A struct with a required
/// field other than `id` can never be empty either, and gets no check.
fn ele_1_check(fields: &Fields) -> Option<proc_macro2::TokenStream> {
    let Fields::Named(named) = fields else {
        return None;
    };

    // `ele-1` is stated on `Element` and inherited by the datatypes and by
    // backbone elements. It is *not* stated on any resource root: in FHIR,
    // `Resource` descends from `Base` rather than from `Element`, and checking
    // the R5 definitions bears that out — 71 root elements carry `ele-1` and
    // every one of them is a datatype. An empty `Patient` is therefore not an
    // `ele-1` violation, however unhelpful it is.
    //
    // A resource is exactly a struct with `implicitRules`, which comes from the
    // `Resource` base: it appears once per resource and in no datatype or
    // backbone.
    if has_field_of(fields, "implicit_rules", "Option") {
        return None;
    }

    let mut emptiness = Vec::new();
    for field in &named.named {
        let ident = field.ident.as_ref()?;
        if ident == "id" {
            continue;
        }
        let syn::Type::Path(path) = &field.ty else {
            return None;
        };
        match path.path.segments.last()?.ident.to_string().as_str() {
            "Option" => emptiness.push(quote! { self.#ident.is_none() }),
            "Vec" => emptiness.push(quote! { self.#ident.is_empty() }),
            // A `Vec1` is never empty and anything else is a required field:
            // either way this struct always has a child, so there is nothing
            // to check.
            _ => return None,
        }
    }

    // A struct with no fields but `id` — DSTU2's `Age`, which is a profile on
    // `Quantity` with its content in `AgeQuantity` — would otherwise report a
    // violation unconditionally, which says nothing.
    if emptiness.is_empty() {
        return None;
    }

    Some(quote! {
        if #(#emptiness)&&* {
            issues.push(crate::validate::ValidationIssue::new(
                "",
                "ele-1: all FHIR elements must have a @value or children",
            ));
        }
    })
}

/// Emit the recognized FHIR invariant checks for a struct (see
/// `spec/10-invariants-coverage.md`).
///
/// Each is a constraint whose FHIRPath expression is checkable structurally —
/// presence, absence, and exclusive choice — with no evaluator. Anything
/// needing to *traverse* (`resolve()`, `descendants()`, `isDistinct()` over a
/// projection) is out of reach until there is one, and is listed as unenforced
/// in spec 10 rather than approximated here.
///
/// These run for every release at once, which is the point of living in the
/// derive macro rather than in generated code.
fn invariant_stmts(struct_name: &str, fields: &Fields) -> proc_macro2::TokenStream {
    let mut checks = proc_macro2::TokenStream::new();

    if let Some(ele_1) = ele_1_check(fields) {
        checks.extend(ele_1);
    }

    // att-1: `data.empty() or contentType.exists()` — bytes with no media type
    // cannot be interpreted by a receiver.
    if struct_name == "Attachment"
        && has_field_of(fields, "data", "Option")
        && has_field_of(fields, "content_type", "Option")
    {
        checks.extend(quote! {
            if self.data.is_some() && self.content_type.is_none() {
                issues.push(crate::validate::ValidationIssue::new(
                    "data",
                    "att-1: if the Attachment has data, it SHALL have a contentType",
                ));
            }
        });
    }

    // qty-3: `code.empty() or system.exists()` — a unit code means nothing
    // without the system that defines it.
    if (QUANTITY_TYPES.contains(&struct_name) || struct_name.ends_with("Quantity"))
        && has_field_of(fields, "code", "Option")
        && has_field_of(fields, "system", "Option")
    {
        checks.extend(quote! {
            if self.code.is_some() && self.system.is_none() {
                issues.push(crate::validate::ValidationIssue::new(
                    "code",
                    "qty-3: if a code for the unit is present, the system SHALL also be present",
                ));
            }
        });
    }

    // drq-1: `path.exists() xor searchParam.exists()` — exactly one, so both
    // and neither are equally wrong.
    // R3 types `path` as `1..1` and does not state drq-1 at all, so the shape
    // gate and the specification agree that it is skipped there.
    if matches!(
        struct_name,
        "DataRequirementCodeFilter" | "DataRequirementDateFilter"
    ) && has_field_of(fields, "path", "Option")
        && has_field_of(fields, "search_param", "Option")
    {
        checks.extend(quote! {
            if self.path.is_some() == self.search_param.is_some() {
                issues.push(crate::validate::ValidationIssue::new(
                    "path",
                    "drq-1: either a path or a searchParam must be provided, but not both",
                ));
            }
        });
    }

    // inv-1: a parameter carries parts, or exactly one of a value and a
    // resource — never a mixture, and never nothing.
    if struct_name == "ParametersParameter"
        && has_field_of(fields, "part", "Vec")
        && has_field_of(fields, "value", "Option")
        && has_field_of(fields, "resource", "Option")
    {
        checks.extend(quote! {
            let __has_part = !self.part.is_empty();
            let __has_value = self.value.is_some();
            let __has_resource = self.resource.is_some();
            let __ok = if __has_part {
                !__has_value && !__has_resource
            } else {
                __has_value != __has_resource
            };
            if !__ok {
                issues.push(crate::validate::ValidationIssue::new(
                    "",
                    "inv-1: a parameter must have one and only one of (value, resource, part)",
                ));
            }
        });
    }

    // ext-1: an Extension SHALL have either a value or nested extensions, not both.
    if struct_name == "Extension" {
        checks.extend(quote! {
            let __has_value = self.value.is_some();
            let __has_ext = !self.extension.is_empty();
            if __has_value == __has_ext {
                issues.push(crate::validate::ValidationIssue::new(
                    "",
                    "ext-1: an extension SHALL have either a value or nested extensions, not both",
                ));
            }
        });
    }

    // dom-2 / dom-4: rules on a domain resource's contained resources.
    let has_contained = matches!(fields, Fields::Named(f)
        if f.named.iter().any(|x| x.ident.as_ref().is_some_and(|i| i == "contained")));
    if has_contained {
        checks.extend(quote! {
            for (__i, __c) in self.contained.iter().enumerate() {
                if __c.get("contained").is_some() {
                    issues.push(crate::validate::ValidationIssue::new(
                        &format!("contained[{__i}]"),
                        "dom-2: a contained resource SHALL NOT itself contain resources",
                    ));
                }
                let __meta = __c.get("meta");
                if __meta.and_then(|m| m.get("versionId")).is_some()
                    || __meta.and_then(|m| m.get("lastUpdated")).is_some()
                {
                    issues.push(crate::validate::ValidationIssue::new(
                        &format!("contained[{__i}]"),
                        "dom-4: a contained resource SHALL NOT have meta.versionId or meta.lastUpdated",
                    ));
                }
            }
        });
    }

    checks
}

/// Snake-case identifier (after stripping a raw `r#`) to camelCase, matching the
/// FHIR element name.
fn to_camel(s: &str) -> String {
    let bare = s.strip_prefix("r#").unwrap_or(s);
    let mut out = String::new();
    let mut upper = false;
    for c in bare.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.extend(c.to_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// Whether a field type is a bare `Vec<…>` (not `Option<Vec<…>>`).
fn is_bare_vec(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.segments.last().is_some_and(|s| s.ident == "Vec"))
}

fn struct_field_stmts(
    fields: &Fields,
    struct_name: &str,
    version: &proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    match fields {
        Fields::Named(f) => f
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().unwrap();
                let fname = ident.to_string();
                // Cardinality: a bare `Vec` that FHIR marks `1..*` must be
                // non-empty. Which fields are `1..*` is not encoded in the type
                // (bare `Vec` is also used for some `0..*`), so consult `meta` at
                // runtime keyed by the struct's FHIR path prefix.
                let cardinality = if is_bare_vec(&field.ty) {
                    let fhir_field = to_camel(&fname);
                    quote! {
                        if self.#ident.is_empty() {
                            if let ::core::option::Option::Some(__prefix) =
                                #version::meta::struct_prefix(#struct_name)
                            {
                                let __path = format!("{}.{}", __prefix, #fhir_field);
                                if #version::meta::element(&__path)
                                    .is_some_and(|__e| __e.min >= 1 && __e.is_multiple())
                                {
                                    issues.push(crate::validate::ValidationIssue::new(
                                        #fname,
                                        "cardinality: a 1..* element must have at least one entry",
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    quote! {}
                };
                quote! {
                    for mut __issue in crate::validate::Validate::validate(&self.#ident) {
                        __issue.path = if __issue.path.is_empty() {
                            #fname.to_string()
                        } else {
                            format!("{}.{}", #fname, __issue.path)
                        };
                        issues.push(__issue);
                    }
                    #cardinality
                }
            })
            .collect(),
        Fields::Unnamed(f) => f
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let idx = Index::from(i);
                quote! {
                    issues.extend(crate::validate::Validate::validate(&self.#idx));
                }
            })
            .collect(),
        Fields::Unit => vec![],
    }
}

/// If `ty` is `Primitive<T>`, return `T` (the inner primitive type).
fn primitive_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(tp) = ty else { return None };
    let seg = tp.path.segments.last()?;
    if seg.ident != "Primitive" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    match args.args.first()? {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

/// Derive flatten-compatible serde for a FHIR `value[x]` choice enum.
///
/// Each variant carries a single unnamed field and a `#[fhir("valueQuantity")]`
/// attribute giving its FHIR key. A variant whose field type is `Primitive<T>`
/// is a *primitive* choice: it serializes both the value key and, when present,
/// the paired `_value<Type>` extension key. Other variants serialize the value
/// key directly (their field is typically `Box<ComplexType>`).
///
/// `Serialize` emits a one- or two-entry map so `#[serde(flatten)]` merges the
/// keys onto the parent object; `Deserialize` scans a (flattened) map for those
/// keys, ignoring all others. Deserialization is lenient: a map with no value
/// key errors, which under `#[serde(flatten)]` on `Option<_>` becomes `None`.
#[proc_macro_derive(FhirChoice, attributes(fhir, fhir_version))]
pub fn derive_fhir_choice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let version = match version_path(&input.attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input.ident, "FhirChoice can only derive on enums")
            .to_compile_error()
            .into();
    };

    let mut ser_arms = Vec::new();
    let mut val_decls = Vec::new();
    let mut key_arms = Vec::new();
    let mut build_arms = Vec::new();

    for variant in &data.variants {
        let vident = &variant.ident;
        // The FHIR key from #[fhir("...")].
        let key: LitStr = match variant
            .attrs
            .iter()
            .find(|a| a.path().is_ident("fhir"))
            .map(|a| a.parse_args::<LitStr>())
        {
            Some(Ok(k)) => k,
            _ => {
                return syn::Error::new_spanned(
                    vident,
                    "each FhirChoice variant needs #[fhir(\"valueXxx\")]",
                )
                .to_compile_error()
                .into();
            }
        };
        let key_str = key.value();
        let ext_key = LitStr::new(&format!("_{key_str}"), key.span());

        let Fields::Unnamed(f) = &variant.fields else {
            return syn::Error::new_spanned(vident, "FhirChoice variants must be tuple variants")
                .to_compile_error()
                .into();
        };
        let field_ty = &f.unnamed.first().expect("one field").ty;
        let val_var = format_ident!("__val_{}", vident);

        if let Some(inner) = primitive_inner(field_ty) {
            // Primitive variant: value + optional `_value<Type>` extension.
            let ext_var = format_ident!("__ext_{}", vident);
            ser_arms.push(quote! {
                #name::#vident(p) => {
                    ::serde::ser::SerializeMap::serialize_entry(&mut map, #key, &p.value)?;
                    if let ::core::option::Option::Some(e) = &p.extension {
                        ::serde::ser::SerializeMap::serialize_entry(&mut map, #ext_key, e)?;
                    }
                }
            });
            val_decls.push(quote! {
                let mut #val_var: ::core::option::Option<#inner> = ::core::option::Option::None;
                let mut #ext_var: ::core::option::Option<#version::types::Element> =
                    ::core::option::Option::None;
            });
            key_arms.push(quote! {
                #key => { #val_var = ::core::option::Option::Some(map.next_value()?); }
                #ext_key => { #ext_var = ::core::option::Option::Some(map.next_value()?); }
            });
            build_arms.push(quote! {
                if let ::core::option::Option::Some(value) = #val_var {
                    return ::core::result::Result::Ok(#name::#vident(
                        #version::choice::Primitive { value, extension: #ext_var }
                    ));
                }
            });
        } else {
            // Complex variant: a single value key.
            ser_arms.push(quote! {
                #name::#vident(v) => {
                    ::serde::ser::SerializeMap::serialize_entry(&mut map, #key, v)?;
                }
            });
            val_decls.push(quote! {
                let mut #val_var: ::core::option::Option<#field_ty> = ::core::option::Option::None;
            });
            key_arms.push(quote! {
                #key => { #val_var = ::core::option::Option::Some(map.next_value()?); }
            });
            build_arms.push(quote! {
                if let ::core::option::Option::Some(v) = #val_var {
                    return ::core::result::Result::Ok(#name::#vident(v));
                }
            });
        }
    }

    let expecting = format!("a FHIR {name} choice element");
    let visitor = format_ident!("__{}Visitor", name);

    quote! {
        impl ::serde::Serialize for #name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S)
                -> ::core::result::Result<S::Ok, S::Error>
            {
                let mut map = serializer.serialize_map(::core::option::Option::None)?;
                match self { #(#ser_arms)* }
                ::serde::ser::SerializeMap::end(map)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D)
                -> ::core::result::Result<Self, D::Error>
            {
                struct #visitor;
                impl<'de> ::serde::de::Visitor<'de> for #visitor {
                    type Value = #name;
                    fn expecting(&self, f: &mut ::core::fmt::Formatter)
                        -> ::core::fmt::Result
                    {
                        f.write_str(#expecting)
                    }
                    #[allow(non_snake_case)]
                    fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A)
                        -> ::core::result::Result<Self::Value, A::Error>
                    {
                        #(#val_decls)*
                        while let ::core::option::Option::Some(__k) =
                            map.next_key::<::std::string::String>()?
                        {
                            match __k.as_str() {
                                #(#key_arms)*
                                _ => { map.next_value::<::serde::de::IgnoredAny>()?; }
                            }
                        }
                        #(#build_arms)*
                        ::core::result::Result::Err(::serde::de::Error::custom(
                            "no value[x] variant present"
                        ))
                    }
                }
                deserializer.deserialize_map(#visitor)
            }
        }
    }
    .into()
}

/// If `ty` is `Option<Inner>`, return `Inner`.
fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(p) = ty else { return None };
    let seg = p.path.segments.last()?;
    if seg.ident != "Option" {
        return None;
    }
    match &seg.arguments {
        PathArguments::AngleBracketed(a) => match a.args.first()? {
            GenericArgument::Type(t) => Some(t),
            _ => None,
        },
        _ => None,
    }
}

/// Derive a chainable builder: `<Type>::builder()` returns a `<Type>Builder`
/// with a setter per field and a `build() -> Result<Type, BuilderError>` that
/// fails if a required (non-`Option`, non-`Vec`, i.e. FHIR `1..1`) field is
/// unset. Optional and repeating fields default to absent/empty.
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder = format_ident!("{name}Builder");

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(name, "Builder can only derive on structs")
            .to_compile_error()
            .into();
    };
    let Fields::Named(fields) = &data.fields else {
        return syn::Error::new_spanned(name, "Builder needs named fields")
            .to_compile_error()
            .into();
    };

    let mut builder_fields = Vec::new();
    let mut setters = Vec::new();
    let mut build_fields = Vec::new();

    for f in &fields.named {
        let ident = f.ident.as_ref().unwrap();
        let fname = ident.to_string();
        let ty = &f.ty;

        if let Some(inner) = option_inner(ty) {
            // Optional field: builder holds `Option<Inner>`, setter takes `Inner`.
            builder_fields.push(quote! { #ident: ::core::option::Option<#inner> });
            setters.push(quote! {
                #[must_use]
                pub fn #ident(mut self, value: #inner) -> Self {
                    self.#ident = ::core::option::Option::Some(value);
                    self
                }
            });
            build_fields.push(quote! { #ident: self.#ident });
        } else if is_bare_vec(ty) {
            // Repeating field: builder holds the `Vec` directly (default empty).
            builder_fields.push(quote! { #ident: #ty });
            setters.push(quote! {
                #[must_use]
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = value;
                    self
                }
            });
            build_fields.push(quote! { #ident: self.#ident });
        } else {
            // Required 1..1 field: builder holds `Option<T>`; build() errors if unset.
            builder_fields.push(quote! { #ident: ::core::option::Option<#ty> });
            setters.push(quote! {
                #[must_use]
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = ::core::option::Option::Some(value);
                    self
                }
            });
            build_fields.push(quote! {
                #ident: self.#ident.ok_or_else(|| crate::builder::BuilderError::missing(#fname))?
            });
        }
    }

    quote! {
        #[doc = concat!("Builder for [`", stringify!(#name), "`].")]
        #[derive(Debug, Default, Clone)]
        pub struct #builder {
            #(#builder_fields,)*
        }

        impl #name {
            #[doc = "Start building a value with the builder."]
            #[must_use]
            pub fn builder() -> #builder {
                #builder::default()
            }
        }

        impl #builder {
            #(#setters)*

            #[doc = "Finish building, erroring if a required field is unset."]
            pub fn build(self) -> ::core::result::Result<#name, crate::builder::BuilderError> {
                ::core::result::Result::Ok(#name {
                    #(#build_fields,)*
                })
            }
        }
    }
    .into()
}
