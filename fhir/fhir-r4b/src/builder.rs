//! Support for the generated R4B resource/datatype builders.
//!
//! `#[derive(fhir_derive_macros::Builder)]` generates a `<Type>Builder` with a
//! chainable setter per field and a `build()` that returns
//! [`Result<T, BuilderError>`], failing if a required (`1..1`) field was not set.
//! Optional (`0..1`) and repeating fields default to absent/empty.
//!
//! [`BuilderError`] itself is release-independent, so it is defined once in
//! [`::fhir_core::builder`]; this module re-exports it so R4B code can use
//! `fhir::r4b::builder::BuilderError`.
//!
//! ```
//! use fhir::r4b::resources::Observation;
//! use fhir::r4b::coded::Coded;
//! use fhir::r4b::codes::ObservationStatus;
//! use fhir::r4b::types::CodeableConcept;
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
