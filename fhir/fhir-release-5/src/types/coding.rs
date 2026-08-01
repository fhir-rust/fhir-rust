//! Coding
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coding
//!
//! Version: 5.0.0
//!
//! Coding Type: A reference to a code defined by a terminology system.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A Coding is a single, atomic value representing a code from a specific terminology system.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A reference to a code defined by a terminology system, capturing the code system,
/// version, code value, human-readable display text, and whether the code was picked
/// directly by a user. Coding is used within CodeableConcept and directly on elements
/// wherever a single coded value from a known system needs to be represented.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::coding::Coding;
/// use fhir::r5::types;
///
/// let value = Coding {
///     user_selected: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `userSelected` is the name this serializes to on the wire.
/// assert_eq!(json["userSelected"], ::serde_json::json!(true));
///
/// let back: Coding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Coding {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The identification of the code system that defines the meaning of the symbol in the code.
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,
    /// The version of the code system which was used when choosing this code.
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,
    /// A symbol in the syntax defined by the code system, representing the specific code.
    pub code: Option<types::Code>, // « C »
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,
    /// A human-readable representation of the meaning of the code, following the code system's rules.
    pub display: Option<types::String>, // « C »
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,
    /// Indicates that this coding was chosen by a user directly, rather than by an algorithm.
    pub user_selected: Option<types::Boolean>,
    /// Primitive extension sibling for [`user_selected`](Self::user_selected) (FHIR `_userSelected`).
    #[serde(rename = "_userSelected")]
    pub user_selected_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coding;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            system: None,
            version: None,
            code: None,
            display: None,
            user_selected: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
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
