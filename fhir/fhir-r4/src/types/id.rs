//! id
//!
//! URL: http://hl7.org/fhir/StructureDefinition/id
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for id type: Any combination of letters, numerals,
/// "-" and ".", with a length limit of 64 characters. (This might be an
/// integer, an unprefixed OID, UUID or any other identifier pattern that meets
/// these constraints.) Ids are case-insensitive.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::id::Id;
///
/// let value = Id::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Id = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Id(
    /// The logical id: up to 64 characters of `[A-Za-z0-9-.]`.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Id::default(), Id(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Id("pat-1".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("pat-1"));
        let back: Id = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
