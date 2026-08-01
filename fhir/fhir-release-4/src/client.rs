//! An async FHIR R4 REST client (feature `client`).
//!
//! The REST protocol does not vary by release, so the implementation lives once
//! in [`::fhir_core::client`]; this module pins it to R4.
//!
//! ```no_run
//! # async fn demo() -> Result<(), fhir::r4::client::ClientError> {
//! use fhir::r4::client::Client;
//!
//! let client = Client::new("https://hapi.fhir.org/baseR4");
//! let patient = client.read("Patient", "example").await?;
//! println!("{patient:?}");
//! # Ok(()) }
//! ```

/// An async FHIR R4 REST client.
pub type Client = ::fhir_core::client::ReleaseClient<super::R4>;

/// An error from a FHIR R4 REST interaction.
pub type ClientError = ::fhir_core::client::ReleaseClientError<super::R4>;
