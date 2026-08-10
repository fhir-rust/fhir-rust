//! MetadataResource
//!
//! ## UML
//!
//! <https://build.fhir.org/uml.html>
//!
//! MetadataResource «Interface»:
//!
//! - usage : markdown [0..1]
//! - approvalDate : date [0..1]
//! - lastReviewDate : date [0..1]
//! - effectivePeriod : Period [0..1]
//! - topic : CodeableConcept [0..*] « DefinitionTopic?? »
//! - author : ContactDetail [0..*]
//! - editor : ContactDetail [0..*]
//! - reviewer : ContactDetail [0..*]
//! - endorser : ContactDetail [0..*]
//! - relatedArtifact : RelatedArtifact [0..*]
//!

#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataResource {}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MetadataResource;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {};
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: MetadataResource = ::serde_json::from_value(json).expect("from_value");
            let expect: MetadataResource = MetadataResource {};
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
