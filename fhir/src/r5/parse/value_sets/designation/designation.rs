//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::Coding;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Designation {
    /// Example: {"system": …, "code": …}
    pub r#use: Option<Coding>,

    /// Example: "Obdurate Labs uses this with both kinds of units..."
    pub value: Option<String>,

    /// Example: "en"
    pub language: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Designation;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::value_sets::DIR
            .join("designation")
            .join("designation.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
