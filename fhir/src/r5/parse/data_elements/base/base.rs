//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Base {
    /// TODO
    pub path: String,

    /// TODO
    pub min: u32,

    /// TODO
    pub max: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Base;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::data_elements::DIR
            .join("base")
            .join("base.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
