//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct UseContext {
    /// Example: {"system" : "http://terminology.hl7.org/CodeSystem/usage-context-type", "code" : "venue" }
    pub code: ::serde_json::Value,

    /// Example: { "text" : "for CCDA Usage" }
    pub value_codeable_concept: Option<CodeableConcept>,

    /// TODO
    pub value_quantity: Option<Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = UseContext;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("use_context")
            .join("use_context.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
