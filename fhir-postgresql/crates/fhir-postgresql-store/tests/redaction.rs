//! Redaction (spec T11.7, O10.2, A7.11). Gated on FHIR_POSTGRESQL_TEST_DB.
//!
//! Two promises that are easy to make and easy to break silently:
//! logs do not contain resource content, and client-visible diagnostics do
//! not echo stored data. Both are the kind of thing that regresses the first
//! time someone adds a helpful `{resource:?}` to an error path, so they are
//! asserted rather than trusted.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use fhir_postgresql_store::{Store, StoreError};
use serde_json::json;

/// A distinctive value planted in the resource. If it appears in a log line
/// or an error message, something is carrying PHI it should not.
const MARKER: &str = "Zzyzxbergenstein";

fn spec_defs() -> Option<PathBuf> {
    let root = std::env::var("FHIR_POSTGRESQL_SPEC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../fhir-rust-crate/doc/fhir-specifications"
            ))
        });
    let defs = root.join("r5").join("fhir-definitions-json");
    defs.exists().then_some(defs)
}

async fn test_store(schema: &str) -> Option<Arc<Store>> {
    let db = std::env::var("FHIR_POSTGRESQL_TEST_DB").ok()?;
    let defs = spec_defs()?;
    // SAFETY: single test binary, set before any threads matter.
    unsafe { std::env::set_var("PGDATABASE", &db) };
    let map = Arc::new(fhir_postgresql_gen::generate(&defs, schema).expect("generate"));
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, map).await.expect("connect");
    store.drop_schema().await.expect("drop");
    assert!(store.init("redaction-checksum").await.expect("init"));
    Some(Arc::new(store))
}

/// A `tracing` sink that keeps every line, so the test can search them.
#[derive(Clone, Default)]
struct Captured(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for Captured {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().expect("lock").extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Captured {
    fn text(&self) -> String {
        String::from_utf8_lossy(&self.0.lock().expect("lock")).into_owned()
    }
}

#[tokio::test]
async fn phi_reaches_neither_the_log_nor_the_error() {
    let Some(store) = test_store("redaction").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };

    let sink = Captured::default();
    let writer = sink.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(move || writer.clone())
        .with_max_level(tracing::Level::TRACE)
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    // A full write/read/search cycle over a resource carrying the marker.
    let patient = json!({
        "resourceType": "Patient",
        "id": "redact-1",
        "name": [{"family": MARKER, "given": [MARKER]}],
        "telecom": [{"system": "phone", "value": "555-0100"}]
    });
    store.put(&patient).await.expect("create");
    store.get("Patient", "redact-1").await.expect("read");
    store
        .search(
            "Patient",
            &[("family".to_string(), MARKER.to_string())],
            10,
            0,
        )
        .await
        .expect("search");
    store.delete("Patient", "redact-1").await.expect("delete");

    // A control line, so "the marker is absent" means the sink was working
    // rather than that nothing was captured at all. Without this the
    // assertion below passes trivially on any code path that never logs.
    const SENTINEL: &str = "redaction-test-sink-alive";
    tracing::info!(target: "redaction_test", "{SENTINEL}");

    let logged = sink.text();
    assert!(
        logged.contains(SENTINEL),
        "the log sink captured nothing, so this test proves nothing"
    );
    assert!(
        !logged.contains(MARKER),
        "a log line carried resource content:\n{logged}"
    );

    // A rejected write must describe the *rule*, not the value. The path is
    // useful to a client; the data at that path is not theirs to be told.
    let bad = json!({
        "resourceType": "Patient",
        "id": "redact-2",
        "name": [{"family": MARKER, "notAnElement": MARKER}]
    });
    let err = store.put(&bad).await.expect_err("unknown element rejected");
    let msg = err.to_string();
    assert!(
        msg.contains("notAnElement"),
        "the error should name the offending path: {msg}"
    );
    assert!(
        !msg.contains(MARKER),
        "the error echoed the submitted value: {msg}"
    );
    assert!(
        matches!(err, StoreError::Shred(_)),
        "a rejected resource is a shred error, which the API renders as 400"
    );
}
