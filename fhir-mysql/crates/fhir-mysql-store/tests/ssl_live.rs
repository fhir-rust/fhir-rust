//! `O10.7` against a live server: TLS is configurable, and verification
//! actually verifies (**F-54**).
//!
//! # Why the third test is the one that matters
//!
//! Proving that a verifying connection *succeeds* proves almost nothing: if
//! certificate checking were a no-op, it would succeed too. So the load-bearing
//! assertion here is that `VERIFY_IDENTITY` **fails** against a server whose
//! certificate does not check out — which is exactly what a stock MySQL
//! container gives us, since its auto-generated certificate is self-signed and
//! issued to a name that is not `127.0.0.1`.
//!
//! A green run of tests 1 and 2 with test 3 removed would be consistent with
//! shipping the plaintext-only build this port had before **F-54**.
//!
//! Needs `FHIR_MYSQL_TEST_DSN`. Skips loudly without one, and fails rather than
//! skips when `FHIR_MYSQL_REQUIRE_DB` is set (`T11.12`, `T11.13`).

use std::sync::Arc;

use fhir_mysql_map::model::RelMap;
use fhir_mysql_store::mysql::MySqlStore;
use fhir_mysql_store::ssl::SslMode;

mod common;

fn dsn() -> Option<String> {
    common::dsn().map(str::to_string)
}

fn require_db() -> bool {
    std::env::var("FHIR_MYSQL_REQUIRE_DB").is_ok_and(|v| v != "0" && !v.is_empty())
}

macro_rules! skip_or_fail {
    ($reason:expr) => {{
        assert!(
            !require_db(),
            "FHIR_MYSQL_REQUIRE_DB is set, so this test must run: {}",
            $reason
        );
        eprintln!("SKIPPING: {}", $reason);
        return;
    }};
}

fn map() -> Arc<RelMap> {
    Arc::new(RelMap {
        fhir_version: "5.0.0".into(),
        schema: "fhir_mysql_ssltest".into(),
        resources: Default::default(),
    })
}

#[tokio::test]
async fn tls_is_configurable_and_verification_is_not_a_no_op() {
    let Some(dsn) = dsn() else {
        skip_or_fail!("set FHIR_MYSQL_TEST_DSN to run");
    };
    // A private CA would make VERIFY_IDENTITY legitimately succeed and break
    // test 3's premise, so this test insists on the stock certificate.
    // SAFETY: single-threaded test binary.
    unsafe { std::env::remove_var("FHIR_MYSQL_SSL_CA") };

    // 1. The server is reachable at all. Without this, test 3's failure could
    //    just be "no server".
    MySqlStore::connect_with(&dsn, map(), SslMode::Disabled)
        .await
        .expect(
            "plaintext connection: the server must be reachable for this test to mean anything",
        );

    // 2. TLS engages. This also proves the `rustls-tls` feature is compiled
    //    in — with `features = ["minimal"]` alone, as this port shipped before
    //    F-54, there is no TLS at all and this fails.
    MySqlStore::connect_with(&dsn, map(), SslMode::Required)
        .await
        .expect("REQUIRED: the server offers TLS and the driver must use it");

    // 3. **Verification rejects a certificate that does not check out.**
    let verified = MySqlStore::connect_with(&dsn, map(), SslMode::VerifyIdentity).await;
    assert!(
        verified.is_err(),
        "VERIFY_IDENTITY accepted a stock MySQL container's self-signed \
         certificate. Either certificate checking is a no-op — in which case \
         the O10.7 default is decorative — or this server has been given a \
         certificate that genuinely validates for this hostname, in which case \
         this test needs rewriting rather than deleting."
    );
}

/// The default must be the verifying mode, checked here as well as in the unit
/// tests because this is the file someone reads when asking "is the live
/// connection encrypted?"
#[test]
fn the_default_mode_verifies() {
    assert_eq!(SslMode::default(), SslMode::VerifyIdentity);
}
