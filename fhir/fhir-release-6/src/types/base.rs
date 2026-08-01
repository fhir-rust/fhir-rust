//! Base
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Base
//!
//! Version: 6.0.0-ballot3
//!
//! Base for all types and resources
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base Type: Base definition for all types defined in FHIR type system.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::base::Base;
///
/// let value = Base::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Base = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
