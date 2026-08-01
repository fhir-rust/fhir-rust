//! The R5 REST client against a mock server.
//!
//! An integration test in the facade rather than a unit test in
//! fhir-core: the client is generic over the release, so exercising it
//! needs a concrete release model, which the core crate does not have.

#![cfg(all(feature = "client", feature = "r5"))]

use fhir::client::{Client, ClientError, ReleaseClientError};
use fhir::r5::resources::Resource;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn read_returns_resource() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/Patient/pat-1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(::serde_json::json!({
                "resourceType": "Patient", "id": "pat-1", "active": true
            })),
        )
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let resource = client.read("Patient", "pat-1").await.unwrap();
    match resource {
        Resource::Patient(p) => assert_eq!(p.id.unwrap().0, "pat-1"),
        other => panic!("expected Patient, got {other:?}"),
    }
}

#[tokio::test]
async fn search_returns_bundle() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/Patient"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(::serde_json::json!({
                "resourceType": "Bundle", "type": "searchset",
                "entry": [{ "resource": { "resourceType": "Patient", "id": "a" } }]
            })),
        )
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let bundle = client
        .search("Patient", &[("name", "chalmers")])
        .await
        .unwrap();
    assert_eq!(bundle.iter_resources().count(), 1);
}

/// Spec 13 acceptance 5: an id is data, not a path.
#[tokio::test]
async fn a_hostile_id_cannot_retarget_the_request() {
    let server = MockServer::start().await;
    // The *encoded* path is what the server must see. If `id` were
    // interpolated raw, this would read `/Patient/other` instead.
    Mock::given(method("GET"))
        .and(path("/Patient/..%2FPatient%2Fother"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(::serde_json::json!({
                "resourceType": "Patient", "id": "safe"
            })),
        )
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let resource = client
        .read("Patient", "../Patient/other")
        .await
        .expect("the encoded path is the one requested");
    match resource {
        Resource::Patient(p) => assert_eq!(p.id.unwrap().0, "safe"),
        other => panic!("expected Patient, got {other:?}"),
    }
}

/// Spec 13 acceptance 4: a server that accepts the connection and then
/// stops talking must produce a timeout, not a hang.
#[tokio::test]
async fn a_stalled_server_times_out() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/Patient/slow"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(std::time::Duration::from_secs(30))
                .set_body_json(::serde_json::json!({"resourceType": "Patient"})),
        )
        .mount(&server)
        .await;

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(150))
        .build()
        .expect("client");
    let client = Client::with_http(server.uri(), http);
    let err = client
        .read("Patient", "slow")
        .await
        .expect_err("should time out");
    match err {
        ClientError::Http(e) => assert!(e.is_timeout(), "expected a timeout, got {e}"),
        other => panic!("expected a transport timeout, got {other:?}"),
    }
}

/// The body cap is a ceiling on what a peer can make us allocate.
#[tokio::test]
async fn an_oversized_body_is_refused() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/Patient/big"))
        .respond_with(ResponseTemplate::new(200).set_body_string("x".repeat(4096)))
        .mount(&server)
        .await;

    let client = Client::new(server.uri()).with_max_body(1024);
    match client.read("Patient", "big").await {
        Err(ClientError::BodyTooLarge { limit }) => assert_eq!(limit, 1024),
        other => panic!("expected BodyTooLarge, got {other:?}"),
    }
}

/// `Debug` output reaches logs and panic messages, so it must not carry
/// a resource (spec R13.10).
#[test]
fn debug_output_does_not_leak_the_body() {
    let err: ClientError = ReleaseClientError::Status {
        status: 500,
        body: "{\"resourceType\":\"Patient\",\"name\":[{\"family\":\"Sensitive\"}]}".to_string(),
    };
    let rendered = format!("{err:?}");
    assert!(!rendered.contains("Sensitive"), "leaked: {rendered}");
    assert!(rendered.contains("body_len"));
}

#[tokio::test]
async fn error_status_parses_operation_outcome() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/Patient/missing"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(::serde_json::json!({
                "resourceType": "OperationOutcome",
                "issue": [{ "severity": "error", "code": "not-found",
                            "diagnostics": "no such Patient" }]
            })),
        )
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let err = client.read("Patient", "missing").await.unwrap_err();
    match err {
        ClientError::Outcome { status, outcome } => {
            assert_eq!(status, 404);
            assert_eq!(outcome.issue.len(), 1);
        }
        other => panic!("expected Outcome, got {other:?}"),
    }
}
