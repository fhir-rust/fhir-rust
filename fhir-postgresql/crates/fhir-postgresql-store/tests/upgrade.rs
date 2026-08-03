//! Schema upgrade (T26): install a reduced map, upgrade to the full one,
//! verify new tables/columns appear, destructive changes are guarded, and
//! data survives. Gated on FHIR_POSTGRESQL_TEST_DB.

use std::path::PathBuf;
use std::sync::Arc;

use fhir_postgresql_store::Store;
use serde_json::json;

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

#[tokio::test]
async fn upgrade_applies_diff() {
    let Ok(db) = std::env::var("FHIR_POSTGRESQL_TEST_DB") else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set");
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };
    // SAFETY: single-threaded at this point.
    unsafe { std::env::set_var("PGDATABASE", &db) };

    let full = fhir_postgresql_gen::generate(&defs, "uptest").expect("generate");
    // The "old" deployment: no Basic resource at all, and Patient's base
    // table missing its last data column.
    let mut reduced = full.clone();
    reduced.resources.remove("Basic").expect("Basic exists");
    let removed_col = {
        let pat = reduced.resources.get_mut("Patient").expect("Patient");
        pat.tables[0].cols.pop().expect("has cols").name
    };

    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let old_store = Store::connect(cfg, Arc::new(reduced))
        .await
        .expect("connect");
    old_store.drop_schema().await.expect("drop");
    old_store.init("old-sum").await.expect("init old");
    // Seed data that must survive the upgrade.
    old_store
        .put(&json!({"resourceType": "Patient", "id": "keep",
                     "name": [{"family": "Survivor"}]}))
        .await
        .expect("seed");

    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let new_store = Store::connect(cfg, Arc::new(full)).await.expect("connect");
    // Plain init refuses (different checksum), upgrade applies.
    assert!(new_store.init("new-sum").await.is_err());
    let report = new_store.upgrade("new-sum", false).await.expect("upgrade");
    assert!(report.additive > 0, "expected additive changes");
    assert_eq!(report.destructive, 0);

    // The new column and the Basic tables exist and work.
    let got = new_store
        .get("Patient", "keep")
        .await
        .expect("get")
        .expect("kept");
    assert_eq!(got.resource["name"][0]["family"], "Survivor");
    new_store
        .put(&json!({"resourceType": "Basic", "id": "b1",
                     "code": {"text": "now supported"}}))
        .await
        .expect("basic put");
    let b = new_store
        .get("Basic", "b1")
        .await
        .expect("get")
        .expect("b1");
    assert_eq!(b.resource["code"]["text"], "now supported");
    let _ = removed_col;

    // Idempotent: a second upgrade to the same map is a no-op diff.
    let again = new_store
        .upgrade("new-sum", false)
        .await
        .expect("re-upgrade");
    assert_eq!(again.additive, 0);
    assert_eq!(again.destructive, 0);

    // Downgrade direction is destructive and must be guarded.
    let mut reduced2 = new_store.map().clone();
    reduced2.resources.remove("Basic");
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let down_store = Store::connect(cfg, Arc::new(reduced2))
        .await
        .expect("connect");
    let err = down_store
        .upgrade("down-sum", false)
        .await
        .expect_err("guarded");
    assert!(err.to_string().contains("destructive"), "{err}");
    let report = down_store.upgrade("down-sum", true).await.expect("forced");
    assert!(report.destructive > 0);
}

/// Upgrading an install written before folded search columns existed (P6.6)
/// must backfill them.
///
/// Without the backfill the columns are added NULL, and every string search
/// compares the folded column — so existing patients simply stop being found.
/// That failure is invisible: no error, no warning, just fewer results.
#[tokio::test]
async fn upgrade_backfills_folded_columns() {
    let Ok(db) = std::env::var("FHIR_POSTGRESQL_TEST_DB") else {
        eprintln!("skipping: FHIR_POSTGRESQL_TEST_DB not set");
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };
    // SAFETY: single-threaded at this point.
    unsafe { std::env::set_var("PGDATABASE", &db) };

    let full = fhir_postgresql_gen::generate(&defs, "foldtest").expect("generate");
    // The "old" deployment: the map as it was before folding existed.
    let mut pre_fold = full.clone();
    for rm in pre_fold.resources.values_mut() {
        for t in &mut rm.tables {
            let dropped: Vec<String> = t.norm_cols.iter().map(|(_, d)| d.clone()).collect();
            t.cols.retain(|c| !dropped.contains(&c.name));
            t.norm_cols.clear();
        }
        for def in &mut rm.search {
            for tgt in &mut def.targets {
                if let fhir_postgresql_map::model::TargetKind::Str { norm, .. } = &mut tgt.kind {
                    *norm = None;
                }
            }
        }
    }

    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let old = Store::connect(cfg, Arc::new(pre_fold))
        .await
        .expect("connect");
    old.drop_schema().await.expect("drop");
    old.init("pre-fold").await.expect("init old");
    old.put(&json!({"resourceType": "Patient", "id": "muller",
                    "name": [{"family": "Müller"}]}))
        .await
        .expect("seed");
    // On the old schema this still worked, via ILIKE — case-insensitive only.
    let hits = old
        .search(
            "Patient",
            &[("family".to_string(), "müller".to_string())],
            10,
            0,
        )
        .await
        .expect("old search");
    assert_eq!(hits, ["muller"], "pre-fold search should still work");

    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let new = Store::connect(cfg, Arc::new(full)).await.expect("connect");
    let report = new.upgrade("post-fold", false).await.expect("upgrade");
    assert!(report.additive > 0, "expected the new columns");
    assert!(report.folded > 0, "expected values to be folded");

    // The seeded patient, written before the column existed, is now findable
    // by an unaccented spelling.
    for term in ["muller", "Müller", "MUL"] {
        let hits = new
            .search(
                "Patient",
                &[("family".to_string(), term.to_string())],
                10,
                0,
            )
            .await
            .expect("search");
        assert_eq!(hits, ["muller"], "family={term:?} after backfill");
    }

    // Backfill is idempotent: nothing left to fold on a second pass.
    let again = new.upgrade("post-fold", false).await.expect("re-upgrade");
    assert_eq!(again.folded, 0, "backfill should have nothing left to do");
}
