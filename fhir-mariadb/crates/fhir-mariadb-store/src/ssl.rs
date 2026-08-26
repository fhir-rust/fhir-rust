//! TLS to the database (`O10.7`, **F-54**).
//!
//! # Why this exists
//!
//! Until 2026-08-03 this port set no TLS options at all, so every connection
//! was plaintext and nothing could change that — no environment variable, no
//! DSN parameter, no API. Both this port and `fhir-mariadb` are **Store**-level,
//! so real patient data crossed that link.
//!
//! It stayed hidden because `tasks.md` said it was done: `T32 Encrypted
//! database transport (O10.7)` was ticked off and described `SslPolicy`, a
//! rustls connector and `PGSSLROOTCERT` trust anchors — all of it
//! `fhir-postgresql`'s text, none of it present here (**F-27**).
//!
//! # The vocabulary is MariaDB's, not libpq's
//!
//! `fhir-postgresql` reads `PGSSLMODE`. That is a libpq name and this port does
//! not speak libpq, so reusing it would invite a deployment to set the wrong
//! variable and believe it took effect — the failure mode `O10.7` is least able
//! to tolerate, because an unencrypted link is invisible from the application.
//!
//! `FHIR_MARIADB_SSL_MODE` therefore takes MariaDB's own `--ssl-mode` values, and
//! `FHIR_MARIADB_SSL_CA` names a root certificate.
//!
//! # `PREFERRED` is refused, not approximated
//!
//! MariaDB's client has a `PREFERRED` mode: encrypt if the server offers it,
//! carry on in the clear otherwise. `mysql_async` cannot express it — passing
//! `SslOpts` makes TLS mandatory, and omitting them makes it impossible; there
//! is no third state.
//!
//! Rather than silently pick one, `PREFERRED` is an error naming the two modes
//! that do exist. Choosing `REQUIRED` would refuse connections that used to
//! work, and choosing `DISABLED` would hand back a plaintext link to someone who
//! asked for encryption — a silent downgrade, which is worse than a failure to
//! start.
//!
//! # The default verifies
//!
//! `VERIFY_IDENTITY`. `O10.7` requires a verifying default, and note that
//! MariaDB's own `REQUIRED` does **not** verify — it encrypts and validates
//! nothing, so it does not survive an active attacker. Defaulting to `REQUIRED`
//! would look secure and satisfy nothing.

use mysql_async::{OptsBuilder, SslOpts};

use crate::StoreError;

/// How this port secures the database connection, in MariaDB's vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SslMode {
    /// No TLS. Appropriate only for a loopback connection on a host where
    /// nothing else runs.
    Disabled,
    /// TLS required, certificate **not** validated. This is MariaDB's own
    /// `REQUIRED`, and it is weaker than it sounds: an attacker presenting any
    /// certificate gets an encrypted connection to themselves.
    Required,
    /// TLS required, certificate chain validated, hostname **not** checked.
    VerifyCa,
    /// TLS required, certificate chain and hostname both validated.
    ///
    /// The default (`O10.7`).
    #[default]
    VerifyIdentity,
}

impl SslMode {
    /// Parse a MariaDB `--ssl-mode` value.
    ///
    /// # Errors
    /// On an unknown value, and on `PREFERRED`, which this driver cannot
    /// express. Both are errors rather than a fallback: a silent downgrade to
    /// plaintext is the outcome this module exists to prevent.
    pub fn parse(s: &str) -> Result<Self, StoreError> {
        match s.trim().to_ascii_uppercase().as_str() {
            "DISABLED" => Ok(Self::Disabled),
            "REQUIRED" => Ok(Self::Required),
            "VERIFY_CA" => Ok(Self::VerifyCa),
            "VERIFY_IDENTITY" => Ok(Self::VerifyIdentity),
            "PREFERRED" => Err(StoreError::Other(
                "ssl-mode PREFERRED cannot be expressed by this driver, which \
                 either requires TLS or does not use it. Choose REQUIRED (or \
                 better, VERIFY_IDENTITY) to encrypt, or DISABLED to accept a \
                 plaintext link deliberately."
                    .to_string(),
            )),
            other => Err(StoreError::Other(format!(
                "unknown ssl-mode {other:?}; expected DISABLED, REQUIRED, \
                 VERIFY_CA, or VERIFY_IDENTITY"
            ))),
        }
    }

    /// The mode from `FHIR_MARIADB_SSL_MODE`, or the verifying default.
    ///
    /// # Errors
    /// Propagates a malformed value rather than falling back, so a typo fails
    /// loudly instead of quietly selecting something weaker than intended.
    pub fn from_env() -> Result<Self, StoreError> {
        match std::env::var("FHIR_MARIADB_SSL_MODE") {
            Ok(v) => Self::parse(&v),
            Err(_) => Ok(Self::default()),
        }
    }

    /// Does this mode verify the server's certificate?
    #[must_use]
    pub fn verifies(self) -> bool {
        matches!(self, Self::VerifyCa | Self::VerifyIdentity)
    }

    /// Apply this mode to a connection builder.
    ///
    /// `root_ca` is the path from `FHIR_MARIADB_SSL_CA`, for a server whose
    /// certificate is signed by a private CA.
    #[must_use]
    pub fn apply(self, b: OptsBuilder, root_ca: Option<&str>) -> OptsBuilder {
        let opts = match self {
            Self::Disabled => return b.ssl_opts(None),
            // `SslOpts::default()` verifies. The two weaker modes are reached
            // by explicitly turning checks off, so that adding a mode cannot
            // accidentally inherit a permissive base.
            Self::Required => SslOpts::default()
                .with_danger_accept_invalid_certs(true)
                .with_danger_skip_domain_validation(true),
            Self::VerifyCa => SslOpts::default().with_danger_skip_domain_validation(true),
            Self::VerifyIdentity => SslOpts::default(),
        };
        // `PathOrBuf<'static>` — the driver keeps the path, so it must own it.
        let opts = match root_ca {
            Some(p) => opts.with_root_certs(vec![std::path::PathBuf::from(p).into()]),
            None => opts,
        };
        b.ssl_opts(opts)
    }
}

/// The root-certificate path from `FHIR_MARIADB_SSL_CA`, if set.
#[must_use]
pub fn root_ca_from_env() -> Option<String> {
    std::env::var("FHIR_MARIADB_SSL_CA")
        .ok()
        .filter(|s| !s.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `O10.7`. Mutation-verified: moving `#[default]` to any other variant
    /// fails this.
    #[test]
    fn the_default_verifies() {
        assert_eq!(SslMode::default(), SslMode::VerifyIdentity);
        assert!(SslMode::default().verifies());
    }

    /// MariaDB's `REQUIRED` encrypts without validating, so it must not count as
    /// verifying however reassuring the name is.
    #[test]
    fn required_is_not_verifying() {
        assert!(!SslMode::parse("REQUIRED").expect("valid").verifies());
        assert!(!SslMode::Disabled.verifies());
    }

    #[test]
    fn the_vocabulary_is_the_engines_own() {
        for (s, want) in [
            ("DISABLED", SslMode::Disabled),
            ("required", SslMode::Required),
            ("Verify_Ca", SslMode::VerifyCa),
            ("VERIFY_IDENTITY", SslMode::VerifyIdentity),
        ] {
            assert_eq!(SslMode::parse(s).expect(s), want, "{s}");
        }
    }

    /// A silent downgrade is the failure this module exists to prevent, so
    /// both an unknown value and the inexpressible one must be errors.
    #[test]
    fn preferred_and_unknown_modes_are_refused() {
        let e = SslMode::parse("PREFERRED").expect_err("must refuse");
        assert!(format!("{e}").contains("cannot be expressed"), "{e}");
        assert!(
            SslMode::parse("verify-full").is_err(),
            "that is libpq's word"
        );
        assert!(SslMode::parse("").is_err());
    }
}
