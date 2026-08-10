//! boolean
//!
//! URL: http://hl7.org/fhir/StructureDefinition/boolean
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for boolean Type: Value of "true" or "false"
///
/// # Examples
///
/// ```
/// use fhir::r4::types::boolean::Boolean;
///
/// let value = Boolean::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Boolean = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Boolean(
    /// The underlying `true` or `false` value.
    pub bool,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Boolean::default(), Boolean(false));
    }

    #[test]
    fn test_serde() {
        let value = Boolean(true);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(true));
        let back: Boolean = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
