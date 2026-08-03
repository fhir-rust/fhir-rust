//! Auditability (spec T11.8). Gated on FHIR_POSTGRESQL_TEST_DB.
//!
//! Five properties, each of which an auditor asks about directly:
//! every change records who made it (M3.15), every read leaves a disclosure
//! record (PR12.5), the history chain verifies (M3.16), and the database
//! itself refuses to let history be rewritten (M3.17).

use std::path::PathBuf;
use std::sync::Arc;

use fhir_postgresql_store::{AccessRecord, Audit, Store};
use serde_json::{Value, json};

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

async fn test_store(schema: &str) -> Option<Arc<Store>> {
    let db = std::env::var("FHIR_POSTGRESQL_TEST_DB").ok()?;
    let defs = spec_defs()?;
    // SAFETY: single test binary, set before any threads matter.
    unsafe { std::env::set_var("PGDATABASE", &db) };
    let map = Arc::new(fhir_postgresql_gen::generate(&defs, schema).expect("generate"));
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, map).await.expect("connect");
    store.drop_schema().await.expect("drop");
    assert!(store.init("audit-checksum").await.expect("init"));
    Some(Arc::new(store))
}

fn patient(id: &str, family: &str) -> Value {
    json!({
        "resourceType": "Patient",
        "id": id,
        "name": [{"family": family}]
    })
}

fn nurse() -> Audit {
    Audit::principal("practitioner-42", "header:X-Fhir-Postgresql-Principal")
        .with_client(Some("10.0.0.7".to_string()))
        .with_request_id(Some("req-abc".to_string()))
        .with_reason(Some("treatment".to_string()))
}

/// Installing a full R5 schema is ~7,355 tables; five of them at once
/// exhausts the server's lock budget. These properties are independent of
/// each other, so they share one schema and run in sequence.
#[tokio::test]
async fn audit_trail_is_complete_and_tamper_evident() {
    let Some(store) = test_store("audit_all").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };
    every_change_records_who_made_it(&store).await;
    an_unattributed_write_says_so(&store).await;
    reads_leave_a_disclosure_record(&store).await;
    the_hash_chain_verifies_and_catches_tampering(&store).await;
    the_database_refuses_to_rewrite_history(&store).await;
    erasure_leaves_a_verifiable_hole(&store).await;
}

/// GDPR Art. 17 against append-only history (M3.18): the record goes, the
/// fact that it went does not.
async fn erasure_leaves_a_verifiable_hole(store: &Store) {
    for n in 0..3 {
        store
            .put_audited(&patient("p6", &format!("Erase{n}")), None, &nurse())
            .await
            .expect("write");
    }
    // Scoped to p6: an earlier step left p4 deliberately tampered, so the
    // global chain is *supposed* to be broken by now.
    let breaks_for = |bs: Vec<fhir_postgresql_store::ChainBreak>| {
        bs.into_iter().filter(|b| b.id == "p6").collect::<Vec<_>>()
    };
    assert!(breaks_for(store.verify_audit().await.expect("verify")).is_empty());

    let who = Audit::principal("dpo-7", "cli").with_reason(Some("art-17-request".to_string()));
    let report = store.purge("Patient", "p6", &who).await.expect("purge");
    assert!(report.existed);
    assert_eq!(report.versions_erased, 3);

    // The resource is gone.
    assert!(store.get("Patient", "p6").await.expect("read").is_none());

    // The tombstone remains, naming who erased it and why.
    let rows = store
        .raw_history_audit("Patient", "p6")
        .await
        .expect("audit rows");
    assert_eq!(rows.len(), 1, "only the tombstone survives");
    assert_eq!(rows[0].1, "dpo-7");
    assert_eq!(rows[0].5.as_deref(), Some("art-17-request"));

    // And it is not mistaken for tampering: an erasure is recorded, not a
    // chain break. Reporting it as a break would train an operator to ignore
    // the report.
    assert!(
        breaks_for(store.verify_audit().await.expect("verify")).is_empty(),
        "an erasure must not read as tampering"
    );
}

async fn every_change_records_who_made_it(store: &Store) {
    store
        .put_audited(&patient("p1", "One"), None, &nurse())
        .await
        .expect("create");
    store
        .put_audited(&patient("p1", "Two"), None, &nurse())
        .await
        .expect("update");
    store
        .delete_audited("Patient", "p1", &nurse())
        .await
        .expect("delete");

    let rows = store
        .raw_history_audit("Patient", "p1")
        .await
        .expect("audit rows");
    assert_eq!(rows.len(), 3, "create, update, delete");
    for (version, actor, source, client, request_id, reason) in &rows {
        assert_eq!(actor, "practitioner-42", "version {version}");
        assert_eq!(
            source.as_deref(),
            Some("header:X-Fhir-Postgresql-Principal")
        );
        assert_eq!(client.as_deref(), Some("10.0.0.7"));
        assert_eq!(request_id.as_deref(), Some("req-abc"));
        assert_eq!(reason.as_deref(), Some("treatment"));
    }
}

async fn an_unattributed_write_says_so(store: &Store) {
    // The plain `put` is what CLI and tests use; it must still record
    // *something*, because a blank actor is indistinguishable from a lost one.
    store.put(&patient("p2", "Anon")).await.expect("create");
    let rows = store
        .raw_history_audit("Patient", "p2")
        .await
        .expect("audit rows");
    assert_eq!(rows[0].1, "unauthenticated");
}

async fn reads_leave_a_disclosure_record(store: &Store) {
    store.put(&patient("p3", "Seen")).await.expect("create");
    store.get("Patient", "p3").await.expect("read");
    store
        .log_access(&AccessRecord {
            audit: nurse(),
            interaction: "read".to_string(),
            rtype: Some("Patient".to_string()),
            id: Some("p3".to_string()),
            version_id: Some(1),
            outcome: "ok".to_string(),
            result_count: None,
        })
        .await
        .expect("log");

    let seen = store
        .access_log_for("Patient", "p3")
        .await
        .expect("access log");
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].0, "practitioner-42");
    assert_eq!(seen[0].1, "read");
    assert_eq!(seen[0].2, "ok");
}

async fn the_hash_chain_verifies_and_catches_tampering(store: &Store) {
    for n in 0..4 {
        store
            .put_audited(&patient("p4", &format!("Gen{n}")), None, &nurse())
            .await
            .expect("write");
    }
    assert!(
        store.verify_audit().await.expect("verify").is_empty(),
        "an untouched chain must verify"
    );

    // Rewrite one row behind the application's back, as an attacker with
    // database access would. The trigger forbids UPDATE, so this has to
    // disable it first — which is exactly the "deliberate DBA act" M3.17
    // describes, and the chain is what notices afterwards.
    store
        .execute_raw_for_test(
            "ALTER TABLE patient_history DISABLE TRIGGER ALL;\n\
             UPDATE patient_history SET resource = jsonb_set(resource, '{name,0,family}', '\"Forged\"') \
               WHERE id = 'p4' AND version_id = 2;\n\
             ALTER TABLE patient_history ENABLE TRIGGER ALL;",
        )
        .await
        .expect("tamper");

    let breaks = store.verify_audit().await.expect("verify");
    // Every configured algorithm must catch it on its own (M3.16a). A chain
    // that only ever failed under SHA-256 would prove nothing about SHA-3,
    // and the whole point of a second design family is that it is not
    // standing idle.
    assert!(
        breaks.iter().all(|b| b.id == "p4" && b.version_id == 2),
        "only the tampered version should break: {breaks:?}"
    );
    for algorithm in ["sha256", "sha3-256"] {
        assert!(
            breaks.iter().any(|b| b.algorithm == algorithm),
            "{algorithm} did not detect the tamper: {breaks:?}"
        );
    }
    // The keyed layer is configuration-dependent, so assert on what is
    // actually configured rather than on a fixed count. Keyed, it must also
    // fire: a MAC that stayed silent while both digests broke would mean the
    // tag was recomputed from the forged row, which is the failure the key
    // exists to prevent.
    let keyed = std::env::var("FHIR_POSTGRESQL_CHAIN_KEY").is_ok();
    let mac_breaks = breaks
        .iter()
        .filter(|b| b.algorithm == "hmac-sha256")
        .count();
    if keyed {
        assert_eq!(mac_breaks, 1, "the keyed tag must catch it too: {breaks:?}");
    } else {
        assert_eq!(mac_breaks, 0, "no key configured, so no tag to check");
    }
    assert_eq!(
        breaks.len(),
        if keyed { 3 } else { 2 },
        "one break per configured layer: {breaks:?}"
    );
}

async fn the_database_refuses_to_rewrite_history(store: &Store) {
    store.put(&patient("p5", "Fixed")).await.expect("create");

    let update = store
        .execute_raw_for_test("UPDATE patient_history SET actor = 'someone-else' WHERE id = 'p5'")
        .await;
    let err = update
        .expect_err("UPDATE on history must be refused")
        .to_string();
    assert!(
        err.contains("append-only"),
        "the refusal should say why: {err}"
    );

    let delete = store
        .execute_raw_for_test("DELETE FROM patient_history WHERE id = 'p5'")
        .await;
    assert!(delete.is_err(), "DELETE on history must be refused");
}

/// The witness makes deletion visible, which the per-row MAC cannot.
///
/// A MAC proves a row was not rewritten. It says nothing about a row that is
/// simply gone — a chain missing its last version verifies perfectly, because
/// nothing left behind refers to what was removed. Only a value recorded
/// outside the database closes that gap.
#[tokio::test]
async fn the_witness_changes_when_history_is_truncated() {
    let Some(store) = test_store("witness").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };
    store.put(&patient("w1", "One")).await.expect("create");
    store.put(&patient("w1", "Two")).await.expect("update");
    let before = store.chain_witness().await.expect("witness");

    // Same data, no changes: the witness must be stable, or an operator
    // comparing it daily would drown in false alarms.
    assert_eq!(
        before,
        store.chain_witness().await.expect("witness"),
        "witness must be deterministic over unchanged history"
    );

    // Truncate: drop the newest version, the way an attacker covering a
    // change would. Every remaining row still verifies.
    store
        .execute_raw_for_test(
            "ALTER TABLE patient_history DISABLE TRIGGER ALL;\n\
             DELETE FROM patient_history WHERE id = 'w1' AND version_id = 2;\n\
             ALTER TABLE patient_history ENABLE TRIGGER ALL;",
        )
        .await
        .expect("truncate");

    let breaks = store.verify_audit().await.expect("verify");
    assert!(
        breaks.is_empty(),
        "a truncated chain still verifies — that is the gap: {breaks:?}"
    );
    assert_ne!(
        before,
        store.chain_witness().await.expect("witness"),
        "the witness must notice the missing version"
    );
}

/// Rotation is additive against a live database, not just in unit tests.
///
/// The failure this guards against is subtle and expensive: turn over a key,
/// and if the retired one is no longer loadable every historical row stops
/// verifying at once. That looks exactly like mass tampering, which is the
/// worst possible false positive for a control whose whole job is to be
/// believed.
///
/// Keys are passed explicitly rather than through the environment. Mutating
/// `FHIR_POSTGRESQL_CHAIN_KEY` is process-global and races every test running beside
/// this one — which is how an unrelated suite started failing.
#[tokio::test]
async fn a_rotated_key_still_verifies_history_it_signed() {
    use fhir_postgresql_store::chain::{ChainKey, KeyRing};

    let Some(setup) = test_store("rotate").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };
    let map = Arc::new(
        fhir_postgresql_gen::generate(&spec_defs().expect("defs"), "rotate").expect("generate"),
    );
    drop(setup);
    let connect = |ring: KeyRing| {
        let map = map.clone();
        async move {
            let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
            Store::connect(cfg, map)
                .await
                .expect("connect")
                .with_chain_keys(ring)
        }
    };
    let k1 = ChainKey::from_hex("k1", &"11".repeat(32)).expect("k1");
    let k2 = ChainKey::from_hex("k2", &"22".repeat(32)).expect("k2");

    // Signed under k1.
    let store = connect(KeyRing::new(vec![k1.clone()])).await;
    store.put(&patient("r1", "Signed")).await.expect("create");
    assert_eq!(store.chain_key_id(), Some("k1"));
    assert!(store.verify_audit().await.expect("verify").is_empty());

    // k2 signs; k1 retired but still loadable.
    let rotated = connect(KeyRing::new(vec![k2.clone(), k1.clone()])).await;
    assert_eq!(rotated.chain_key_id(), Some("k2"), "the new key signs");
    rotated
        .put(&patient("r1", "Rotated"))
        .await
        .expect("update");
    assert!(
        rotated.verify_audit().await.expect("verify").is_empty(),
        "rows signed under k1 and k2 must both verify"
    );

    // Drop k1: its rows become unverifiable, which is a gap in coverage and
    // never a finding.
    let partial = connect(KeyRing::new(vec![k2])).await;
    assert!(
        partial.verify_audit().await.expect("verify").is_empty(),
        "a key we no longer hold is a gap in coverage, never a finding"
    );
}

/// The checkpoint reaches the `audit_checkpoint` target, which is what makes
/// "a deployment already shipping logs has a witness for free" true.
///
/// Asserting the target specifically matters: an operator routes and retains
/// on that name, so a checkpoint logged anywhere else is invisible to the
/// pipeline meant to preserve it. The line must also carry no PHI, since it
/// is expected to outlive ordinary logs and travel where patient data
/// must not.
#[tokio::test]
async fn checkpoints_are_logged_on_their_own_target_without_phi() {
    use std::sync::{Arc as StdArc, Mutex};

    #[derive(Clone, Default)]
    struct Capture(StdArc<Mutex<Vec<u8>>>);
    impl std::io::Write for Capture {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().expect("lock").extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let Some(store) = test_store("checkpoint").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };
    // A distinctive family name: if it appears in the checkpoint line, the
    // line is carrying patient data it must not.
    store
        .put(&patient("c1", "Zzyzxbergenstein"))
        .await
        .expect("create");

    let sink = Capture::default();
    let made = sink.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(move || made.clone())
        .with_ansi(false)
        .with_target(true)
        .finish();
    // `set_default` rather than `with_default`: the emission is async, and a
    // closure cannot await. `#[tokio::test]` runs on a current-thread
    // runtime, so the await stays on the thread this guard applies to.
    let guard = tracing::subscriber::set_default(subscriber);
    store.emit_checkpoint("test").await;
    drop(guard);

    let logged = String::from_utf8(sink.0.lock().expect("lock").clone()).expect("utf8");
    assert!(
        logged.contains("audit_checkpoint"),
        "checkpoint must land on its own target: {logged}"
    );
    assert!(
        logged.contains("witness="),
        "checkpoint must carry the witness value: {logged}"
    );
    assert!(
        !logged.contains("Zzyzxbergenstein"),
        "checkpoint leaked patient data: {logged}"
    );
}

/// Re-signing must refuse to launder forged history, and must let a
/// compromised key be dropped once it has run.
///
/// The refusal is the important half. If re-signing worked on a tampered
/// chain it would hand forged rows the new key's authority — turning the
/// recovery procedure into the attack.
#[tokio::test]
async fn resigning_refuses_tampered_history_and_frees_the_old_key() {
    use fhir_postgresql_store::chain::{ChainKey, KeyRing};

    let Some(setup) = test_store("resign").await else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set or spec missing");
        return;
    };
    let map = Arc::new(
        fhir_postgresql_gen::generate(&spec_defs().expect("defs"), "resign").expect("gen"),
    );
    drop(setup);
    let connect = |ring: KeyRing| {
        let map = map.clone();
        async move {
            let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
            Store::connect(cfg, map)
                .await
                .expect("connect")
                .with_chain_keys(ring)
        }
    };
    let k1 = ChainKey::from_hex("k1", &"11".repeat(32)).expect("k1");
    let k2 = ChainKey::from_hex("k2", &"22".repeat(32)).expect("k2");
    let audit = fhir_postgresql_store::Audit::cli();

    let old = connect(KeyRing::new(vec![k1.clone()])).await;
    old.put(&patient("s1", "One")).await.expect("create");
    old.put(&patient("s2", "Two")).await.expect("create");

    // k2 signs, k1 still loadable: this is where re-signing is done from.
    let both = connect(KeyRing::new(vec![k2.clone(), k1.clone()])).await;
    let signed = both
        .resign_history(&audit, "k1 suspected compromised")
        .await
        .expect("resign");
    assert!(signed >= 2, "every history row is counter-signed: {signed}");

    // The point of the exercise: k1 can now be dropped and history still
    // verifies, because the counter-signatures stand in for its tags.
    let without_k1 = connect(KeyRing::new(vec![k2.clone()])).await;
    assert!(
        without_k1.verify_audit().await.expect("verify").is_empty(),
        "counter-signed history must verify without the retired key"
    );

    // Now tamper, and confirm re-signing refuses rather than blessing it.
    without_k1
        .execute_raw_for_test(
            "ALTER TABLE patient_history DISABLE TRIGGER ALL;\n\
             UPDATE patient_history SET resource = jsonb_set(resource, '{name,0,family}', '\"Forged\"') \
               WHERE id = 's1' AND version_id = 1;\n\
             ALTER TABLE patient_history ENABLE TRIGGER ALL;",
        )
        .await
        .expect("tamper");
    let err = without_k1
        .resign_history(&audit, "attempt after tampering")
        .await
        .expect_err("must refuse");
    assert!(
        format!("{err}").contains("refusing to re-sign"),
        "unexpected error: {err}"
    );
}
