//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::profiles_resources::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RestResource {
    /// Example: "Account"
    pub r#type: String,

    /// Example : "http://hl7.org/fhir/StructureDefinition/Account"
    pub profile: String,

    /// TODO
    pub interaction: Vec<Interaction>,

    /// TODO
    pub conditional_create: bool,

    /// TODO
    pub conditional_update: bool,

    /// Example: "multiple"
    pub conditional_delete: String,

    /// Example: ["literal", "logical"]
    pub reference_policy: Vec<String>,

    /// Example: ["Account.owner", "Account.patient", "Account.subject"]]
    pub search_include: Option<Vec<String>>,

    /// Example: ["Account.relatedaccount", "ChargeItem.account", "Encounter.account"]
    pub search_rev_include: Option<Vec<String>>,

    /// TODO
    pub search_param: Option<Vec<RestSearchParam>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RestResource;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_resources::DIR
            .join("rest_resource")
            .join("rest_resource.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
