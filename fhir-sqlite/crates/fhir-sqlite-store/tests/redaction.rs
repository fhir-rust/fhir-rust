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
//! Needs no server, so unlike the PostgreSQL original this always runs.

use std::sync::{Arc, Mutex};

use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::{Audit, StoreError, sqlite::SqliteStore};
use serde_json::json;

/// A distinctive value planted in the resource. If it appears in a log line or
/// an error message, something is carrying PHI it should not.
///
/// Deliberately not a realistic name: a real one risks colliding with something
/// the engine legitimately logs, and a false positive here is expensive to
/// diagnose.
const MARKER: &str = "Zzyzxbergenstein";

fn relmap() -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-sqlite-map/assets/fhir-sqlite-relmap-r5.json.gz");
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok().map(Arc::new)
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/test-scratch")
        .join(format!("redact-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
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
    let Some(map) = relmap() else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let store = SqliteStore::open(scratch("phi").join("fhir.sqlite"), map)
        .await
        .expect("open");
    store.init("redaction-checksum").await.expect("init");

    let sink = Captured::default();
    let writer = sink.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(move || writer.clone())
        .with_max_level(tracing::Level::TRACE)
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    // A full write/read/search/delete cycle over a resource carrying the marker
    // in several shapes: a child-table string, a grandchild, and a scalar.
    let patient = json!({
        "resourceType": "Patient",
        "id": "redact-1",
        "active": true,
        "name": [{"family": MARKER, "given": [MARKER]}],
        "telecom": [{"system": "phone", "value": "555-0100"}]
    });
    store.put(&patient, &Audit::cli()).await.expect("create");
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
        .delete("Patient", "redact-1", &Audit::cli())
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
    let Some(map) = relmap() else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let store = SqliteStore::open(scratch("reject").join("fhir.sqlite"), map)
        .await
        .expect("open");
    store.init("redaction-checksum").await.expect("init");

    let bad = json!({
        "resourceType": "Patient",
        "id": "redact-2",
        "name": [{"family": MARKER, "notAnElement": MARKER}]
    });
    let err = store
        .put(&bad, &Audit::cli())
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
