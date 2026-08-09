//! Reference
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Reference
//!
//! Version: 4.0.1
//!
//! A reference from one resource to another
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;
use std::marker::PhantomData;

/// Base StructureDefinition for Reference Type: A reference from one resource
/// to another.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::reference::Reference;
/// use fhir::r4::types;
///
/// let value = Reference {
///     r#type: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `type` is the name this serializes to on the wire.
/// assert_eq!(json["type"], ::serde_json::json!("http://example.org"));
///
/// let back: Reference = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Reference<T = Any> {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<types::String>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Type the reference refers to (e.g. "Patient")
    pub r#type: Option<types::Uri>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Logical reference, when literal reference is not known
    pub identifier: Option<types::Identifier>,

    /// Text alternative for the resource
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Compile-time marker for the referenced resource type. Zero-sized and not
    /// serialized; `Reference<Patient>` and `Reference<Any>` share one wire form.
    ///
    /// Public only so that callers can use the struct-literal idiom the rest of
    /// the model documents — `Reference { reference: …, ..Default::default() }`
    /// needs access to every field. Treat it as an implementation detail.
    #[doc(hidden)]
    #[serde(skip)]
    pub _marker: PhantomData<fn() -> T>,
}

impl<T> Default for Reference<T> {
    fn default() -> Self {
        Reference {
            id: None,
            extension: Vec::new(),
            reference: None,
            reference_ext: None,
            r#type: None,
            type_ext: None,
            identifier: None,
            display: None,
            display_ext: None,
            _marker: PhantomData,
        }
    }
}

/// A marker type naming the resource a [`Reference`] points to.
///
/// Implemented by every resource type (in `resources.rs`, beside the
/// `Resource` enum) and by [`Any`].
pub trait ResourceType {
    /// The FHIR resource type name (e.g. `"Patient"`), or `None` for [`Any`].
    fn resource_type_name() -> Option<&'static str>;
}

/// The untyped reference target: any resource type. This is the default `T`, so
/// a bare `Reference` is `Reference<Any>` and existing code is unaffected.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Any;

impl ResourceType for Any {
    fn resource_type_name() -> Option<&'static str> {
        None
    }
}

impl<T> Reference<T> {
    /// Re-interpret the compile-time target type. The wire form is identical for
    /// every `T`, so this only changes the phantom marker.
    #[must_use]
    pub fn cast<U>(self) -> Reference<U> {
        Reference {
            id: self.id,
            extension: self.extension,
            reference: self.reference,
            reference_ext: self.reference_ext,
            r#type: self.r#type,
            type_ext: self.type_ext,
            identifier: self.identifier,
            display: self.display,
            display_ext: self.display_ext,
            _marker: PhantomData,
        }
    }

    /// Drop the compile-time target type, yielding an untyped `Reference<Any>`.
    #[must_use]
    pub fn into_any(self) -> Reference<Any> {
        self.cast()
    }
}

impl<T: ResourceType> Reference<T> {
    /// Resolve this reference within a `Bundle`, returning the matching entry's
    /// resource JSON.
    ///
    /// The reference string is matched against each entry's `fullUrl` or its
    /// `resourceType/id`. When `T` names a concrete resource type, a candidate
    /// whose `resourceType` differs is rejected.
    ///
    /// ```
    /// use fhir::r4::resources::{Bundle, Patient};
    /// use fhir::r4::types::reference::Reference;
    ///
    /// let bundle: Bundle = serde_json::from_value(serde_json::json!({
    ///     "resourceType": "Bundle",
    ///     "type": "collection",
    ///     "entry": [
    ///         { "fullUrl": "urn:uuid:pat-1",
    ///           "resource": { "resourceType": "Patient", "id": "pat-1" } }
    ///     ]
    /// })).unwrap();
    ///
    /// let subject: Reference<Patient> = serde_json::from_value(
    ///     serde_json::json!({ "reference": "Patient/pat-1" })
    /// ).unwrap();
    ///
    /// let resolved = subject.resolve(&bundle).unwrap();
    /// assert_eq!(resolved["resourceType"], "Patient");
    /// ```
    #[must_use]
    pub fn resolve<'b>(
        &self,
        bundle: &'b crate::r4::resources::Bundle,
    ) -> Option<&'b ::serde_json::Value> {
        let want = &self.reference.as_ref()?.0;
        let expected = T::resource_type_name();
        for entry in &bundle.entry {
            let matches_full_url = entry.full_url.as_ref().is_some_and(|u| &u.0 == want);
            let resource = entry.resource.as_ref();
            let rt = resource
                .and_then(|r| r.get("resourceType"))
                .and_then(|v| v.as_str());
            let id = resource.and_then(|r| r.get("id")).and_then(|v| v.as_str());
            let matches_rel = match (rt, id) {
                (Some(rt), Some(id)) => *want == format!("{rt}/{id}"),
                _ => false,
            };
            if (matches_full_url || matches_rel)
                && expected.is_none_or(|want_ty| rt == Some(want_ty))
            {
                return resource;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Reference;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
