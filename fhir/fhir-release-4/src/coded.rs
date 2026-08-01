//! A coded value that is usually a known enum variant but tolerates any code.
//!
//! FHIR elements with a `required` binding draw their value from a value set, so
//! this crate types them as the matching [`codes`](crate::r4::codes) enum. But
//! real data occasionally carries a code outside the set (a newer code, a local
//! extension, or simply invalid data), and a data-exchange library must not fail
//! to parse it. [`Coded<E>`] wraps the enum with an [`Unknown`](Coded::Unknown)
//! fallback so every wire value round-trips. See `spec/05-code-systems.md`.
//!
//! [`Coded<E>`] is generic over the enum and therefore release-independent, so
//! it is defined once in [`::fhir_core::coded`]; this module re-exports it so R4 code
//! can use `fhir::r4::coded::Coded`.
//!
//! ```
//! use fhir::r4::coded::Coded;
//! use fhir::r4::codes::AdministrativeGender;
//!
//! // A known code parses into the enum variant.
//! let known: Coded<AdministrativeGender> =
//!     serde_json::from_value(serde_json::json!("female")).unwrap();
//! assert_eq!(known, Coded::Known(AdministrativeGender::Female));
//!
//! // An unrecognized code is preserved as-is rather than rejected.
//! let unknown: Coded<AdministrativeGender> =
//!     serde_json::from_value(serde_json::json!("robot")).unwrap();
//! assert_eq!(unknown, Coded::Unknown("robot".to_string()));
//! ```

pub use ::fhir_core::coded::Coded;
