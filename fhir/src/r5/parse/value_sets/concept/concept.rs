//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::value_sets::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Concept {
    /// Example: "chol-mmol"
    pub code: String,

    /// Example: "SChol (mmol/L)"
    pub display: String,

    /// Example: "Serum Cholesterol, in mmol/L"
    pub definition: String,

    /// Example: TODO
    pub designation: Vec<Designation>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Concept;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::value_sets::DIR
            .join("concept")
            .join("concept.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
