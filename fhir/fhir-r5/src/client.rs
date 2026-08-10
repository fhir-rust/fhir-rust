//! An async FHIR R5 REST client (feature `client`).
//!
//! The REST protocol does not vary by release, so the implementation lives once
//! in [`::fhir_core::client`]; this module pins it to R5.
//!
//! ```no_run
//! # async fn demo() -> Result<(), fhir::r5::client::ClientError> {
//! use fhir::r5::client::Client;
//!
//! let client = Client::new("https://hapi.fhir.org/baseR5");
//! let patient = client.read("Patient", "example").await?;
//! println!("{patient:?}");
//! # Ok(()) }
//! ```

/// An async FHIR R5 REST client.
pub type Client = ::fhir_core::client::ReleaseClient<super::R5>;

/// An error from a FHIR R5 REST interaction.
pub type ClientError = ::fhir_core::client::ReleaseClientError<super::R5>;
