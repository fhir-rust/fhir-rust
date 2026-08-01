//! Support for the generated resource/datatype builders.
//!
//! `#[derive(fhir_derive_macros::Builder)]` generates a `<Type>Builder` with a
//! chainable setter per field and a `build()` that returns
//! [`Result<T, BuilderError>`], failing if a required (`1..1`) field was not set.
//! Optional (`0..1`) and repeating fields default to absent/empty.
//!
//! The error type is release-independent, so it is defined once here and
//! re-exported as [`r4::builder`](crate::r4::builder) and
//! [`r5::builder`](crate::r5::builder); see those modules for worked examples.
//!
//! ```
//! use fhir::builder::BuilderError;
//!
//! let err = BuilderError::missing("status");
//! assert_eq!(err.missing_field, "status");
//! assert_eq!(err.to_string(), "required field `status` was not set");
//! ```

use std::fmt;

/// Error returned by a builder's `build()` when a required field is missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuilderError {
    /// The name of the required field that was not set.
    pub missing_field: String,
}

impl BuilderError {
    /// A missing required field.
    #[must_use]
    pub fn missing(field: &str) -> Self {
        Self {
            missing_field: field.to_string(),
        }
    }
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "required field `{}` was not set", self.missing_field)
    }
}

impl std::error::Error for BuilderError {}
