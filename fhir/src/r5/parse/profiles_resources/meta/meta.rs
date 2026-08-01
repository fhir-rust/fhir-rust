//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// Example: "lastUpdated" : "2023-03-26T15:21:02.749+11:00"
    pub last_updated: Option<String>,

    /// Example: "profile" : ["http://hl7.org/fhir/StructureDefinition/shareablecodesystem"]
    pub profile: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Meta;

    #[test]
    fn serde_json_from_reader() {
        let path = crate::r5::parse::profiles_resources::DIR
            .join("meta")
            .join("meta.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).expect("from_reader");
        assert_eq!(
            actual.last_updated,
            Some(String::from("2023-03-26T15:21:02.749+11:00"))
        );
    }
}
