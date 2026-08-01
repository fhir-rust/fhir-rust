//! code
//!
//! URL: http://hl7.org/fhir/StructureDefinition/code
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for code type: A string which has at least one
/// character and no leading or trailing whitespace and where there is no
/// whitespace other than single spaces in the contents
///
/// # Examples
///
/// ```
/// use fhir::r4::types::code::Code;
///
/// let value = Code::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Code = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Code(
    /// The code as it appears on the wire.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Code::default(), Code(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Code("final".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("final"));
        let back: Code = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
