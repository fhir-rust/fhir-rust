//! `O10.7`: the database connection carries PHI and MUST default to verifying
//! the server certificate — so an *unset* environment variable must not be
//! weaker than a set one.
//!
//! This lives here rather than in `src/ssl.rs` for one mechanical reason:
//! mutating the environment is `unsafe` in edition 2024, the crate declares
//! `#![forbid(unsafe_code)]`, and `forbid` cannot be lifted by an `allow`. An
//! integration test is a separate crate, so the rule that protects the library
//! does not have to be weakened to test the library. `fhir-postgresql` reached
//! the same arrangement first, in its own `tests/ssl_default.rs`.
//!
//! It runs without a database, deliberately. A security default should not be
//! gated behind having provisioned MARIADB, or it becomes another check that
//! silently skips (`T11.12`).

use fhir_mariadb_store::ssl::SslMode;

/// Absent configuration must not be weaker than present configuration.
#[test]
fn an_unset_env_var_is_the_verifying_default() {
    // SAFETY: single-threaded test binary; no other thread reads the env here.
    unsafe { std::env::remove_var("FHIR_MARIADB_SSL_MODE") };
    assert_eq!(
        SslMode::from_env().expect("unset is not an error"),
        SslMode::VerifyIdentity
    );
}
