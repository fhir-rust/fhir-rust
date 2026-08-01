//! decimal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/decimal
//!
//! Version: 6.0.0-ballot3
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

//! `decimal` is defined identically in every modelled release, so the type
//! lives in the `fhir-core` crate ([`::fhir_core::decimal`]) and each release
//! it. Its precision is preserved lexically — see spec 02 (R2.2).

pub use ::fhir_core::decimal::{Decimal, DecimalError};
