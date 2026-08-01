//! Base
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Base
//!
//! Version: 5.0.0
//!
//! Base Type: Base definition for all types defined in FHIR type system.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base is the abstract root definition for every type in the FHIR type system.
/// All FHIR datatypes and resources ultimately derive from `Base`, which itself
/// declares no elements. It exists purely to anchor the type hierarchy and to
/// provide a common ancestor for the FHIR object model in R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::base::Base;
///
/// let value = Base::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Base = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Base {}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Base;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
