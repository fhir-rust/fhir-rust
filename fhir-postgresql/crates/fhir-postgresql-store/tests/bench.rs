//! Bulk-load benchmark + index audit. Gated on FHIR_POSTGRESQL_BENCH=<n resources>
//! (and FHIR_POSTGRESQL_TEST_DB); prints throughput and EXPLAIN verdicts, and fails
//! if the canonical searches fall back to sequential scans on child tables.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use fhir_postgresql_store::Store;
use serde_json::json;

mod common;

fn spec_defs() -> Option<PathBuf> {
    let root = std::env::var("FHIR_POSTGRESQL_SPEC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../fhir/doc/fhir-specifications"
            ))
        });
    let defs = root.join("r5").join("fhir-definitions-json");
    defs.exists().then_some(defs)
}

fn patient(i: usize) -> serde_json::Value {
    json!({
        "resourceType": "Patient",
        "id": format!("bench-p{i}"),
        "active": i.is_multiple_of(2),
        "gender": (["male", "female", "other", "unknown"][i % 4]),
        "birthDate": format!("{}-{:02}-{:02}", 1930 + (i % 90), 1 + (i % 12), 1 + (i % 28)),
        "identifier": [{"system": "http://bench.example/mrn", "value": format!("MRN{i}")}],
        "name": [{"family": format!("Family{}", i % 5000), "given": [format!("Given{}", i % 977)]}]
    })
}

fn observation(i: usize) -> serde_json::Value {
    json!({
        "resourceType": "Observation",
        "id": format!("bench-o{i}"),
        "status": "final",
        "code": {"coding": [{"system": "http://loinc.org",
                             "code": format!("{}-{}", 1000 + (i % 300), i % 10)}]},
        "subject": {"reference": format!("Patient/bench-p{}", i / 2)},
        "effectiveDateTime": format!("20{:02}-{:02}-{:02}T10:00:00Z",
                                     10 + (i % 16), 1 + (i % 12), 1 + (i % 28)),
        "valueQuantity": {"value": (i % 500) as f64 / 3.0,
                          "system": "http://unitsofmeasure.org", "code": "mg/dL"}
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn bench_bulk_load_and_index_audit() {
    let (Some(_db), Ok(n)) = (
        common::test_db(),
        std::env::var("FHIR_POSTGRESQL_BENCH").map(|v| v.parse::<usize>().unwrap_or(0)),
    ) else {
        eprintln!("skipping: set FHIR_POSTGRESQL_BENCH=<n>");
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };
    let map = Arc::new(fhir_postgresql_gen::generate(&defs, "benchtest").expect("generate"));
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Arc::new(Store::connect(cfg, map).await.expect("connect"));
    store.drop_schema().await.expect("drop");
    store.init("bench").await.expect("init");

    // Concurrent load: half patients, half observations.
    let workers = 12usize;
    let counter = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicUsize::new(0));
    let started = std::time::Instant::now();
    let mut handles = Vec::new();
    for _ in 0..workers {
        let store = store.clone();
        let counter = counter.clone();
        let failed = failed.clone();
        handles.push(tokio::spawn(async move {
            loop {
                let i = counter.fetch_add(1, Ordering::Relaxed);
                if i >= n {
                    break;
                }
                let res = if i.is_multiple_of(2) {
                    patient(i / 2)
                } else {
                    observation(i)
                };
                if store.put(&res).await.is_err() {
                    failed.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }
    for h in handles {
        h.await.expect("worker");
    }
    let secs = started.elapsed().as_secs_f64();
    let ok = n - failed.load(Ordering::Relaxed);
    println!(
        "BENCH load: {ok}/{n} resources in {secs:.1}s → {:.0} res/s ({workers} workers)",
        ok as f64 / secs
    );
    assert_eq!(failed.load(Ordering::Relaxed), 0, "load failures");

    // Read latency sample.
    let started = std::time::Instant::now();
    let reads = 500.min(n / 4).max(1);
    for i in 0..reads {
        store
            .get("Patient", &format!("bench-p{}", i * 7 % (n / 4).max(1)))
            .await
            .expect("get");
    }
    println!(
        "BENCH read: {reads} reads, {:.2} ms avg",
        started.elapsed().as_secs_f64() * 1000.0 / reads as f64
    );

    // EXPLAIN audit: canonical searches must hit indexes, not seq scans.
    let (client, conn) = fhir_postgresql_store::pg_config(None)
        .expect("cfg")
        .connect(tokio_postgres::NoTls)
        .await
        .expect("conn");
    tokio::spawn(conn);
    let cases = [
        (
            "token (identifier)",
            "SELECT p.\"id\" FROM \"benchtest\".\"patient\" p WHERE EXISTS (SELECT 1 FROM \"benchtest\".\"patient_identifier\" c WHERE c.\"rid\" = p.\"id\" AND (c.\"system\" = 'http://bench.example/mrn' AND (c.\"value\")::text = 'MRN77'))",
            "patient_identifier",
        ),
        (
            "reference (subject)",
            "SELECT p.\"id\" FROM \"benchtest\".\"observation\" p WHERE (p.\"subject_ref_type\" = 'Patient' AND p.\"subject_ref_id\" = 'bench-p42')",
            "observation",
        ),
        (
            "date (birthdate range)",
            "SELECT p.\"id\" FROM \"benchtest\".\"patient\" p WHERE (p.\"birth_date_sort\" >= '1980-01-01' AND p.\"birth_date_sort\" < '1981-01-01')",
            "patient",
        ),
    ];
    for (label, sql, table) in cases {
        let rows = client
            .query(&format!("EXPLAIN {sql}"), &[])
            .await
            .expect("explain");
        let plan: Vec<String> = rows.iter().map(|r| r.get::<_, String>(0)).collect();
        let plan_text = plan.join("\n");
        let seq_on_table = plan_text.contains(&format!("Seq Scan on {table}"));
        let uses_index = plan_text.contains("Index Scan") || plan_text.contains("Bitmap");
        println!(
            "BENCH explain {label}: {}",
            if uses_index && !seq_on_table {
                "index ok"
            } else {
                "SEQ SCAN"
            }
        );
        assert!(
            uses_index && !seq_on_table,
            "{label} plans a seq scan:\n{plan_text}"
        );
    }
}
