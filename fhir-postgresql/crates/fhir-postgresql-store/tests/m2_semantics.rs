//! M2 semantics: versioning, history, soft delete, vread, optimistic
//! concurrency, and idempotent init. Gated on FHIR_POSTGRESQL_TEST_DB.

use std::path::PathBuf;
use std::sync::Arc;

use fhir_postgresql_store::{ResourceStatus, Store, StoreError};
use serde_json::{Value, json};

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

async fn test_store() -> Option<Store> {
    let _db = common::test_db()?;
    let defs = spec_defs()?;
    let map = Arc::new(fhir_postgresql_gen::generate(&defs, "m2test").expect("generate"));
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, map).await.expect("connect");
    store.drop_schema().await.expect("drop");
    assert!(store.init("m2-checksum").await.expect("init"));
    Some(store)
}

fn patient(id: &str, family: &str) -> Value {
    json!({
        "resourceType": "Patient",
        "id": id,
        "active": true,
        "name": [{"family": family}]
    })
}

#[tokio::test]
async fn m2_lifecycle_and_concurrency() {
    let Some(store) = test_store().await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };

    // -- create → update → delete → history/vread --
    let p1 = store.put(&patient("p1", "One")).await.expect("create");
    assert!(p1.created);
    assert_eq!(p1.version_id, 1);
    let p2 = store.put(&patient("p1", "Two")).await.expect("update");
    assert!(!p2.created);
    assert_eq!(p2.version_id, 2);
    assert!(store.delete("Patient", "p1").await.expect("delete"));

    let hist = store.history("Patient", "p1").await.expect("history");
    let ops: Vec<char> = hist.iter().map(|h| h.op).collect();
    let vids: Vec<i64> = hist.iter().map(|h| h.version_id).collect();
    assert_eq!(ops, vec!['D', 'U', 'C']);
    assert_eq!(vids, vec![3, 2, 1]);

    let v1 = store
        .vread("Patient", "p1", 1)
        .await
        .expect("vread")
        .expect("v1");
    assert_eq!(v1.resource.as_ref().unwrap()["name"][0]["family"], "One");
    let v2 = store
        .vread("Patient", "p1", 2)
        .await
        .expect("vread")
        .expect("v2");
    assert_eq!(v2.resource.as_ref().unwrap()["name"][0]["family"], "Two");
    let v3 = store
        .vread("Patient", "p1", 3)
        .await
        .expect("vread")
        .expect("v3");
    assert_eq!(v3.op, 'D');
    assert!(v3.resource.is_none());
    assert!(
        store
            .vread("Patient", "p1", 9)
            .await
            .expect("vread")
            .is_none()
    );

    // Read of a deleted id: gone (410-shaped), not merely unknown (404).
    assert!(store.get("Patient", "p1").await.expect("get").is_none());
    assert_eq!(
        store.status("Patient", "p1").await.expect("status"),
        ResourceStatus::Deleted(3)
    );
    assert_eq!(
        store.status("Patient", "never").await.expect("status"),
        ResourceStatus::Unknown
    );

    // Recreate after delete: version numbering continues.
    let p4 = store.put(&patient("p1", "Four")).await.expect("recreate");
    assert!(p4.created);
    assert_eq!(p4.version_id, 4);
    assert_eq!(
        store.status("Patient", "p1").await.expect("status"),
        ResourceStatus::Active(4)
    );

    // -- optimistic concurrency --
    // Wrong expectation → conflict, nothing written.
    let err = store
        .put_if(&patient("p1", "Stale"), Some(2))
        .await
        .expect_err("stale write must fail");
    assert!(matches!(
        err,
        StoreError::Conflict {
            expected: 2,
            found: 4
        }
    ));
    // Right expectation succeeds.
    let p5 = store
        .put_if(&patient("p1", "Five"), Some(4))
        .await
        .expect("matched write");
    assert_eq!(p5.version_id, 5);
    // Create-only (If-None-Exist shape): id exists → conflict.
    let err = store
        .put_if(&patient("p1", "Six"), Some(0))
        .await
        .expect_err("create-only on existing id");
    assert!(matches!(
        err,
        StoreError::Conflict {
            expected: 0,
            found: 5
        }
    ));

    // Two racing conditional writers: exactly one wins.
    let store = Arc::new(store);
    let a = {
        let s = store.clone();
        tokio::spawn(async move { s.put_if(&patient("p1", "A"), Some(5)).await })
    };
    let b = {
        let s = store.clone();
        tokio::spawn(async move { s.put_if(&patient("p1", "B"), Some(5)).await })
    };
    let (ra, rb) = (a.await.expect("join"), b.await.expect("join"));
    let wins = [&ra, &rb].iter().filter(|r| r.is_ok()).count();
    assert_eq!(wins, 1, "exactly one racing writer must win: {ra:?} {rb:?}");
    assert_eq!(
        store.status("Patient", "p1").await.expect("status"),
        ResourceStatus::Active(6)
    );

    // -- idempotent init --
    assert!(
        !store.init("m2-checksum").await.expect("re-init"),
        "re-init must no-op"
    );
    let err = store
        .init("tampered")
        .await
        .expect_err("mismatched checksum");
    assert!(err.to_string().contains("different map"));
}
