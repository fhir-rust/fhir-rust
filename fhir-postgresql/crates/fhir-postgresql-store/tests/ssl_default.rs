//! `O10.7`: the database connection carries PHI and MUST default to verifying
//! the server certificate.
//!
//! This exists because the default was wrong for the whole life of the port
//! (**F-17**) and nothing noticed. `SslPolicy` derived `Default` with
//! `#[default]` on `Prefer`, one token, in a file nobody re-reads — and the
//! failure mode of getting it wrong is silent: an unverified link looks
//! identical to a verified one from the application's side.
//!
//! These run without a database, deliberately. A security default should not be
//! gated behind having provisioned PostgreSQL, or it becomes another check that
//! silently skips (`T11.12`).

use fhir_postgresql_store::SslPolicy;

/// The requirement, stated as an equality so it cannot drift.
#[test]
fn the_default_verifies_the_server_certificate() {
    assert_eq!(
        SslPolicy::default(),
        SslPolicy::Require,
        "O10.7: the default MUST verify the server certificate. `Prefer` does \
         not — it accepts plaintext if the server declines TLS, and accepts a \
         forged certificate if it does not."
    );
}

/// Absent configuration must not be weaker than present configuration.
#[test]
fn an_unset_pgsslmode_is_the_verifying_default() {
    // SAFETY: single-threaded test binary; no other thread reads the env here.
    unsafe { std::env::remove_var("PGSSLMODE") };
    assert_eq!(
        SslPolicy::from_env().expect("unset is not an error"),
        SslPolicy::Require
    );
}

/// The escape hatches still work — this is a change of default, not the removal
/// of a mode. A deployment that genuinely wants libpq's behaviour can still ask
/// for it, and now has to ask.
#[test]
fn the_weaker_modes_remain_reachable_but_must_be_asked_for() {
    assert_eq!(
        SslPolicy::parse("prefer").expect("valid"),
        SslPolicy::Prefer
    );
    assert_eq!(SslPolicy::parse("allow").expect("valid"), SslPolicy::Prefer);
    assert_eq!(
        SslPolicy::parse("disable").expect("valid"),
        SslPolicy::Disable
    );
}

/// `verify-ca` and `verify-full` collapse into `Require`, which validates both
/// certificate and hostname — stricter than libpq for `verify-ca`, and the safe
/// direction to err in (`M14.27`).
#[test]
fn every_verifying_libpq_mode_maps_to_require() {
    for m in ["require", "verify-ca", "verify-full"] {
        assert_eq!(
            SslPolicy::parse(m).expect("valid"),
            SslPolicy::Require,
            "{m}"
        );
    }
}

/// An unknown mode must be an error, never a silent downgrade.
#[test]
fn an_unknown_mode_is_refused_rather_than_downgraded() {
    assert!(SslPolicy::parse("verify-nothing").is_err());
    assert!(SslPolicy::parse("").is_err());
}
