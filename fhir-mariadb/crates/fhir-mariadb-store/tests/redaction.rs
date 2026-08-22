//! Redaction (spec T11.7, O10.2), ported from the PostgreSQL suite.
//!
//! Two promises that are easy to make and easy to break silently: logs do not
//! contain resource content, and errors do not echo submitted data. Both
//! regress the first time someone adds a helpful `{resource:?}` to an error
//! path, so they are asserted rather than trusted.
//!
//! For a component holding PHI the stakes are specific. A log line has
//! different retention, different access control, and different export paths
//! from the database — a name that reaches it has left the system that was
//! audited and entered one that was not. An error message is worse still: it
//! lands in a log, a response, and a ticket at once.
//!
//! Needs `FHIR_MARIADB_TEST_DSN`; `scripts/db.sh up` prints it.

use std::sync::{Arc, Mutex};

use fhir_mariadb_map::model::RelMap;
use fhir_mariadb_store::StoreError;
use fhir_mariadb_store::mariadb::MariaDbStore;
use serde_json::json;

mod common;

/// A distinctive value planted in the resource. If it appears in a log line or
/// an error message, something is carrying PHI it should not.
///
/// Deliberately not a realistic name: a real one risks colliding with something
/// the engine legitimately logs, and a false positive here is expensive to
/// diagnose.
const MARKER: &str = "Zzyzxbergenstein";

fn dsn() -> Option<String> {
    common::dsn().map(str::to_string)
}

fn sampled(schema: &str) -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-mariadb-map/assets/fhir-mariadb-relmap-r5.json.gz");
    let bytes = std::fs::read(path).ok()?;
    let mut m = RelMap::from_gz_bytes(&bytes).ok()?;
    m.resources.retain(|k, _| k == "Patient");
    m.schema = schema.to_string();
    Some(Arc::new(m))
}

async fn fresh(schema: &str) -> Option<MariaDbStore> {
    let store = MariaDbStore::connect(&dsn()?, sampled(schema)?)
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("redaction-checksum").await.expect("init");
    Some(store)
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
    let Some(store) = fresh("fhir_mariadb_redact_phi").await else {
        eprintln!("skipping: set FHIR_MARIADB_TEST_DSN to run");
        return;
    };

    let sink = Captured::default();
    let writer = sink.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(move || writer.clone())
        .with_max_level(tracing::Level::TRACE)
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    let audit = fhir_mariadb_store::Audit::default();
    let patient = json!({
        "resourceType": "Patient",
        "id": "redact-1",
        "active": true,
        "name": [{"family": MARKER, "given": [MARKER]}],
        "telecom": [{"system": "phone", "value": "555-0100"}]
    });
    store.put(&patient, &audit).await.expect("create");
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
    store.history("Patient", "redact-1").await.expect("history");
    store
        .delete("Patient", "redact-1", &audit)
        .await
        .expect("delete");

    // A control line, so "the marker is absent" means the sink was working
    // rather than that nothing was captured at all. Without this the assertion
    // below passes trivially on any code path that never logs (T11.12).
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
}

/// A rejected write must describe the **rule**, not the value.
///
/// The path is useful to a client and belongs in the message (R4.3). The data
/// at that path is not theirs to be told back, and it is the half that ends up
/// in a bug report.
#[tokio::test]
async fn a_rejected_write_names_the_path_without_echoing_the_value() {
    let Some(store) = fresh("fhir_mariadb_redact_reject").await else {
        eprintln!("skipping: set FHIR_MARIADB_TEST_DSN to run");
        return;
    };

    let bad = json!({
        "resourceType": "Patient",
        "id": "redact-2",
        "name": [{"family": MARKER, "notAnElement": MARKER}]
    });
    let err = store
        .put(&bad, &fhir_mariadb_store::Audit::default())
        .await
        .expect_err("an unknown element must be rejected, never silently dropped");
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
        "a rejected resource is a shred error, not a database error: {err:?}"
    );
}
