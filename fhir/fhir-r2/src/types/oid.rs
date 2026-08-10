//! oid
//!
//! URL: http://hl7.org/fhir/StructureDefinition/oid
//!
//!
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for oid type: An oid represented as a URI
///
/// # Examples
///
/// ```
/// use fhir::r2::types::oid::Oid;
///
/// let value = Oid::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Oid = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Oid(
    /// The OID as a `urn:oid:` URI.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Oid::default(), Oid(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Oid("urn:oid:1.2.3".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("urn:oid:1.2.3"));
        let back: Oid = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
