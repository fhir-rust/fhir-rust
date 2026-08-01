//! Decimal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/decimal
//!
//! decimal Type: A rational number with implicit precision
//!
//! FHIR: <https://build.fhir.org/>
//!
//! `decimal` is defined identically in every modelled release, so the type
//! lives at the crate root ([`::fhir_core::decimal`]) and each release re-exports
//! it. Its precision is preserved lexically — see spec 02 (R2.2).

pub use ::fhir_core::decimal::{Decimal, DecimalError};
