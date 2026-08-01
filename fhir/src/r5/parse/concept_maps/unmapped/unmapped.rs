//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

// use crate::r5::parse::concept_maps::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Unmapped {
    /// Example: "other-map",
    pub mode: String,

    /// Example: "http://example.org/fhir/ConceptMap/map2"
    other_map: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Unmapped;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("unmapped")
            .join("unmapped.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
