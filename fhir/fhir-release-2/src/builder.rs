//! Support for the generated R2 resource/datatype builders.
//!
//! `#[derive(fhir_derive_macros::Builder)]` generates a `<Type>Builder` with a
//! chainable setter per field and a `build()` that returns
//! [`Result<T, BuilderError>`], failing if a required (`1..1`) field was not set.
//! Optional (`0..1`) and repeating fields default to absent/empty.
//!
//! [`BuilderError`] itself is release-independent, so it is defined once in
//! [`::fhir_core::builder`]; this module re-exports it so R2 code can use
//! `fhir::r2::builder::BuilderError`.
//!
//! ```
//! use fhir::r2::resources::Observation;
//! use fhir::r2::coded::Coded;
//! use fhir::r2::codes::ObservationStatus;
//! use fhir::r2::types::CodeableConcept;
//!
//! // `status` and `code` are required (1..1); omitting them fails build().
//! assert!(Observation::builder().build().is_err());
//!
//! let obs = Observation::builder()
//!     .status(Coded::Known(ObservationStatus::Final))
//!     .code(CodeableConcept::default())
//!     .build()
//!     .expect("both required fields set");
//! assert!(matches!(obs.status, Coded::Known(ObservationStatus::Final)));
//! ```

pub use ::fhir_core::builder::BuilderError;
