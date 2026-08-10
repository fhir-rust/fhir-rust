//! decimal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/decimal
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

//! `decimal` is defined identically in every modelled release, so the type
//! lives in the `fhir-core` crate ([`::fhir_core::decimal`]) and each release
//! it. Its precision is preserved lexically — see spec 02 (R2.2).

pub use ::fhir_core::decimal::{Decimal, DecimalError};
