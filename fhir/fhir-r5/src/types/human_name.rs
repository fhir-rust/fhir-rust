//! HumanName
//!
//! URL: http://hl7.org/fhir/StructureDefinition/HumanName
//!
//! Version: 5.0.0
//!
//! HumanName Type: A name, normally of a human, that can be used for other living entities (e.g. animals but not organizations) that have been assigned names by a human and may need the use of name parts or the need for usage information.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! This module represents the name of a person or other living entity, split into parts such as family, given, prefix, and suffix, together with usage information.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A name, normally of a human, that can be used for other living entities (e.g. animals but
/// not organizations) that have been assigned names by a human and may need the use of name
/// parts or the need for usage information. A `HumanName` breaks a name into its constituent
/// parts (family, given, prefix, suffix), records a purpose-of-use code, and can carry a
/// plain-text rendering plus a validity period.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::human_name::HumanName;
/// use fhir::r5::types;
///
/// let value = HumanName {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: HumanName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The purpose for which this name is used, such as official, nickname, or maiden.
    #[serde(rename = "use")]
    pub use1: Option<types::Code>, // « NameUse! »
    /// Primitive extension sibling for [`use1`](Self::use1) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// A full text representation of the name as it should be displayed or printed.
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// The family name, surname, or last name of the person.
    pub family: Option<types::String>,
    /// Primitive extension sibling for [`family`](Self::family) (FHIR `_family`).
    #[serde(rename = "_family")]
    pub family_ext: Option<types::Element>,

    /// The given names, including first and middle names, in the order they should be used.
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub given: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`given`](Self::given) (FHIR `_given`).
    #[serde(rename = "_given")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub given_ext: Vec<Option<types::Element>>,

    /// Parts that come before the name, such as titles (e.g. "Dr.", "Mr.").
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub prefix: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`).
    #[serde(rename = "_prefix")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prefix_ext: Vec<Option<types::Element>>,

    /// Parts that come after the name, such as generational or qualification suffixes (e.g. "Jr.", "MD").
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub suffix: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`suffix`](Self::suffix) (FHIR `_suffix`).
    #[serde(rename = "_suffix")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suffix_ext: Vec<Option<types::Element>>,

    /// The period during which this name was, or is expected to be, in use.
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = HumanName;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            use1: None,
            text: None,
            family: None,
            given: ::fhir_core::PrimVec::new(),
            prefix: ::fhir_core::PrimVec::new(),
            suffix: ::fhir_core::PrimVec::new(),
            period: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "given": [],
                    "prefix": [],
                    "suffix": [],
                }
            );
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
