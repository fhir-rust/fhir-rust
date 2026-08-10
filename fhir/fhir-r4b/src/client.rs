//! An async FHIR R4B REST client (feature `client`).
//!
//! The REST protocol does not vary by release, so the implementation lives once
//! in [`::fhir_core::client`]; this module pins it to R4B.
//!
//! ```no_run
//! # async fn demo() -> Result<(), fhir::r4b::client::ClientError> {
//! use fhir::r4b::client::Client;
//!
//! let client = Client::new("https://hapi.fhir.org/baseR4");
//! let patient = client.read("Patient", "example").await?;
//! println!("{patient:?}");
//! # Ok(()) }
//! ```

/// An async FHIR R4B REST client.
pub type Client = ::fhir_core::client::ReleaseClient<super::R4B>;

/// An error from a FHIR R4B REST interaction.
pub type ClientError = ::fhir_core::client::ReleaseClientError<super::R4B>;
