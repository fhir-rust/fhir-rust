//! **F-85**: a root-level extension's attach path is `""`, which this
//! engine stores as NULL (`''` is NULL, `M14.39`). Until 2026-08-10 the
//! `"path"` column was `CLOB NOT NULL`, so a Patient with a US-Core-style
//! root extension was refused outright with `ORA-01400` — found by probing
//! before designing F-47 step 5's conversion, not by any of the seven
//! store tests, none of which stored a root-level extension.
//!
//! Fresh installs fix it via the nullable bounded column `create_table`
//! now emits; upgraded installs via the step-5 conversion (that half is
//! asserted in `upgrade.rs`). Needs `FHIR_ORACLE_TEST_*`; run with
//! `--test-threads=1` (`M14.5`: one shared `R5` schema).

use std::sync::Arc;

use fhir_oracle_map::model::RelMap;
use fhir_oracle_store::oracle::OracleStore;
use serde_json::json;

mod common;

#[tokio::test]
async fn a_root_level_extension_round_trips_on_a_fresh_install() {
    // `common::dsn()` resolves the connect string and, when it adopts the
    // db.sh container, fills in the user and password it publishes.
    let (Some(connect), Ok(user), Ok(password)) = (
        common::dsn(),
        std::env::var("FHIR_ORACLE_TEST_USER"),
        std::env::var("FHIR_ORACLE_TEST_PASSWORD"),
    ) else {
        eprintln!("skipping: set FHIR_ORACLE_TEST_* to run");
        return;
    };
    let mut m = RelMap::bundled("r5").expect("r5 map");
    m.resources.retain(|k, _| k == "Patient");
    m.schema = "R5".into();
    let store = OracleStore::connect(&user, &password, connect, Arc::new(m))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("f85").await.expect("init");

    let r = json!({"resourceType": "Patient", "id": "p1",
        "extension": [{"url": "http://x.example/e", "valueString": "root"}]});
    store
        .put(&r, &fhir_oracle_store::Audit::default())
        .await
        .expect("a root-level extension must store (F-85)");
    let got = store
        .get("Patient", "p1")
        .await
        .expect("get")
        .expect("stored");
    assert_eq!(got["extension"][0]["valueString"], "root");
}
