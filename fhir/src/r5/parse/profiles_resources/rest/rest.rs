//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::profiles_resources::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Rest {
    /// Example:"server"
    pub mode: String,

    /// Example: "An empty Capability Statement"
    pub documentation: String,

    /// Example: TODO
    pub security: Security,

    /// Example: TODO
    pub resource: Option<Vec<RestResource>>,

    /// TODO
    pub interaction: Option<Vec<Interaction>>,

    /// TODO
    pub operation: Option<Vec<Operation>>,

    /// TODO
    pub search_param: Option<Vec<RestSearchParam>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Rest;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_resources::DIR
            .join("rest")
            .join("rest.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
