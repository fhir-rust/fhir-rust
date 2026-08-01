//! uuid
//!
//! URL: http://hl7.org/fhir/StructureDefinition/uuid
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for uuid type: A UUID, represented as a URI
///
/// # Examples
///
/// ```
/// use fhir::r4::types::uuid::Uuid;
///
/// let value = Uuid::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Uuid = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Uuid(
    /// The UUID as a `urn:uuid:` URI.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Uuid::default(), Uuid(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Uuid("urn:uuid:c757873d-ec9a-4326-a141-556f43239520".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("urn:uuid:c757873d-ec9a-4326-a141-556f43239520"));
        let back: Uuid = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
