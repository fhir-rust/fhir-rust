//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::search_parameters::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// Example: "fullUrl" : "http://hl7.org/fhir/StructureDefinition/Narrative"
    pub full_url: String,

    /// Example: "resource" : { "resourceType": "StructureDefinition", "id" : "Narrative",  }
    pub resource: Resource,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Entry;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::search_parameters::DIR
            .join("entry")
            .join("entry.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.full_url, "http://hl7.org/fhir/CodeSystem/example");
    }
}
