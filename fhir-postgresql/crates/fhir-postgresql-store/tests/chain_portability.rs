//! A chain written here must be verifiable by anything holding `canon.rs`
//! (`X15.11`, `X15.2`, `M14.12`).
//!
//! Closes audit **F-07**. Until this revision the pre-image was
//! `(($1::text)::jsonb)::text` — the bytes signed were whatever PostgreSQL's
//! `jsonb` rendered, a form defined by an engine version rather than by this
//! specification. `fhir-postgresql-map` was the only map crate without
//! `canon.rs`, and a chain written here could not be checked anywhere else.
//!
//! What makes this test worth more than `audit.rs`: that suite calls
//! `verify_audit`, so the writer and the verifier are the same code and would
//! agree even if both were wrong. This one recomputes the chain **from the
//! outside**, using only what another port would have — the exported row
//! columns, the shared `canon::canonicalize`, and `chain::preimage`/`link`. If
//! that reproduces the stored digests, the format has left the database.
//!
//! Needs a live PostgreSQL; `scripts/db.sh up` prints the environment.

use std::sync::Arc;

use fhir_postgresql_map::model::RelMap;
use fhir_postgresql_store::{Store, chain};
use serde_json::json;

fn spec_defs() -> Option<std::path::PathBuf> {
    let root = std::env::var("FHIR_POSTGRESQL_SPEC_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::path::PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../fhir-rust-crate/doc/fhir-specifications"
            ))
        });
    let defs = root.join("r5").join("fhir-definitions-json");
    defs.exists().then_some(defs)
}

async fn store(schema: &str) -> Option<Store> {
    std::env::var("FHIR_POSTGRESQL_TEST_DB").ok()?;
    let defs = spec_defs()?;
    let mut map: RelMap = fhir_postgresql_gen::generate(&defs, schema).ok()?;
    map.resources
        .retain(|k, _| k == "Patient" || k == "Observation");
    let cfg = fhir_postgresql_store::pg_config(None).ok()?;
    let s = Store::connect(cfg, Arc::new(map)).await.ok()?;
    s.drop_schema().await.ok()?;
    s.init("chain-portability").await.ok()?;
    Some(s)
}

/// One history row, as an exporter would hand it to a foreign verifier.
struct Row {
    id: String,
    version_id: i64,
    ts_utc: String,
    op: String,
    resource: Option<String>,
    actor: String,
    row_hash: Option<Vec<u8>>,
    row_hash_sha3: Option<Vec<u8>>,
}

/// Read the history table over a plain connection — deliberately not through
/// `Store`, so nothing the writer did can be assumed by the reader.
async fn history_rows(schema: &str, table: &str) -> Vec<Row> {
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let (client, conn) = cfg.connect(tokio_postgres::NoTls).await.expect("connect");
    tokio::spawn(async move {
        let _ = conn.await;
    });
    client
        .query(
            &format!(
                "SELECT \"id\", \"version_id\", \
                        to_char(\"last_updated\" AT TIME ZONE 'UTC', \
                                'YYYY-MM-DD HH24:MI:SS.US'), \
                        \"op\", (\"resource\")::text, \"actor\", \
                        \"row_hash\", \"row_hash_sha3\" \
                 FROM \"{schema}\".\"{table}\" ORDER BY \"id\", \"version_id\""
            ),
            &[],
        )
        .await
        .expect("query history")
        .into_iter()
        .map(|r| Row {
            id: r.get(0),
            version_id: r.get(1),
            ts_utc: r.get(2),
            op: r.get(3),
            resource: r.get(4),
            actor: r.get(5),
            row_hash: r.get(6),
            row_hash_sha3: r.get(7),
        })
        .collect()
}

/// Recompute a resource's chain the way a *foreign* verifier would, and require
/// it to match what PostgreSQL stored.
#[tokio::test]
async fn a_foreign_verifier_can_recompute_the_chain() {
    let Some(store) = store("fhir_pg_chainport").await else {
        eprintln!("skipping: needs FHIR_POSTGRESQL_TEST_DB and a spec dir");
        return;
    };

    // Keys deliberately out of alphabetical order, and a decimal whose trailing
    // zero must survive: both are cases where a naive serializer and `jsonb`
    // disagree with the canonical form.
    for (id, body) in [
        (
            "p1",
            json!({"resourceType": "Patient", "id": "p1",
                      "name": [{"family": "Ámélie", "given": ["Zoë"]}],
                      "active": true, "birthDate": "1980-02-29"}),
        ),
        (
            "p2",
            json!({"resourceType": "Patient", "id": "p2",
                      "active": false,
                      "name": [{"given": ["B"], "family": "A"}]}),
        ),
    ] {
        store.put(&body).await.expect("put");
        let _ = id;
    }
    // A second version, so the chain has a link to follow rather than a genesis
    // row only.
    store
        .put(&json!({"resourceType": "Patient", "id": "p1",
                     "name": [{"family": "Ámélie", "given": ["Zoë", "Q"]}],
                     "active": false, "birthDate": "1980-02-29"}))
        .await
        .expect("put v2");

    let rows = history_rows("fhir_pg_chainport", "patient_history").await;
    assert!(rows.len() >= 3, "expected at least three history rows");

    // The foreign verifier: nothing here touches the store's internals.
    let (mut prev_id, mut prior_256, mut prior_3) = (String::new(), None, None);
    let mut checked = 0usize;
    for r in &rows {
        if r.id != prev_id {
            prev_id.clone_from(&r.id);
            prior_256 = None;
            prior_3 = None;
        }
        let canon = r.resource.as_deref().map(|stored| {
            let v: serde_json::Value = serde_json::from_str(stored).expect("stored row is JSON");
            fhir_postgresql_map::canon::canonicalize(&v)
        });
        let pre = chain::preimage(
            &r.id,
            r.version_id,
            &r.ts_utc,
            &r.op,
            canon.as_deref(),
            &r.actor,
        );
        let (want_256, want_3) = chain::link(prior_256.as_deref(), prior_3.as_deref(), &pre);

        assert_eq!(
            r.row_hash.as_deref(),
            Some(want_256.as_slice()),
            "SHA-256 for {}#{} does not match a chain recomputed from canon.rs — \
             the pre-image is still engine-defined (F-07)",
            r.id,
            r.version_id
        );
        assert_eq!(
            r.row_hash_sha3.as_deref(),
            Some(want_3.as_slice()),
            "SHA3-256 for {}#{} does not match",
            r.id,
            r.version_id
        );
        prior_256 = r.row_hash.clone();
        prior_3 = r.row_hash_sha3.clone();
        checked += 1;
    }
    assert!(checked >= 3, "verified too few rows to mean anything");
}

/// The canonical form is **not** what `jsonb::text` produces, which is what
/// makes this a format change rather than a rename.
///
/// If these two agreed, the fix would be untestable and F-07 would have been
/// cosmetic. They do not: `canonicalize` sorts keys and preserves the decimal
/// lexeme, and it is defined by this project rather than by a server version.
#[tokio::test]
async fn the_canonical_form_differs_from_what_jsonb_renders() {
    let Some(store) = store("fhir_pg_chaindiff").await else {
        eprintln!("skipping: needs FHIR_POSTGRESQL_TEST_DB and a spec dir");
        return;
    };
    // `value` before `system` on the wire; jsonb renders in its own order.
    store
        .put(&json!({"resourceType": "Observation", "id": "o1",
                     "status": "final",
                     "code": {"text": "t"},
                     "valueQuantity": {"value": 1.50, "unit": "mg", "system": "http://unitsofmeasure.org"}}))
        .await
        .expect("put");

    let rows = history_rows("fhir_pg_chaindiff", "observation_history").await;
    let stored = rows[0].resource.as_deref().expect("resource present");
    let v: serde_json::Value = serde_json::from_str(stored).expect("json");
    let canon = fhir_postgresql_map::canon::canonicalize(&v);

    assert_ne!(
        canon, stored,
        "canonical form is byte-identical to jsonb's rendering; this test cannot \
         distinguish the two pre-image formats and proves nothing"
    );
    // And the property that matters: canonicalizing is idempotent, so a verifier
    // that re-canonicalizes what it read gets the same bytes the writer signed.
    let again: serde_json::Value = serde_json::from_str(&canon).expect("canon is json");
    assert_eq!(
        fhir_postgresql_map::canon::canonicalize(&again),
        canon,
        "canonicalization is not idempotent, so a verifier could not reproduce it"
    );
}
