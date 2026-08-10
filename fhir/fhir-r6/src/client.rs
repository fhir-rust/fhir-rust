//! An async FHIR R6 REST client (feature `client`).
//!
//! The REST protocol does not vary by release, so the implementation lives once
//! in [`::fhir_core::client`]; this module pins it to R6.
//!
//! ```no_run
//! # async fn demo() -> Result<(), fhir::r6::client::ClientError> {
//! use fhir::r6::client::Client;
//!
//! let client = Client::new("https://hapi.fhir.org/baseR5");
//! let patient = client.read("Patient", "example").await?;
//! println!("{patient:?}");
//! # Ok(()) }
//! ```

/// An async FHIR R6 REST client.
pub type Client = ::fhir_core::client::ReleaseClient<super::R6>;

/// An error from a FHIR R6 REST interaction.
pub type ClientError = ::fhir_core::client::ReleaseClientError<super::R6>;
