//! DomainResource
//!
//! ## UML
//!
//! <https://build.fhir.org/uml.html>
//!
//! DomainResource:
//!
//!

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DomainResource {
    text: Option<types::Narrative>, // « C »

    #[serde(skip_serializing_if = "Vec::is_empty")]
    contained: Vec<types::String>, // « C »

    #[serde(skip_serializing_if = "Vec::is_empty")]
    extension: Vec<types::Extension>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    modifier_extension: Vec<types::Extension>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DomainResource;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            text: None,
            contained: vec![],
            extension: vec![],
            modifier_extension: vec![],
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
                    "text": null,
                    "contained": [],
                    "extension": [],
                    "modifierExtension": [],
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
