//! Schema upgrade (T26): install a reduced map, upgrade to the full one,
//! verify new tables/columns appear, destructive changes are guarded, and
//! data survives. Gated on FHIR_POSTGRESQL_TEST_DB.

use std::path::PathBuf;
use std::sync::Arc;

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

#[tokio::test]
async fn upgrade_applies_diff() {
    let Some(_db) = common::test_db() else {
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };

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
    let Some(_db) = common::test_db() else {
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };

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

/// `O10.4b` (**F-90**): a relocated column reaches the diff as ADD + DROP,
/// and the guard must tell it apart from a genuine removal. The surgery
/// mirrors what `G2.6a`'s force-split does to a shape: one unsearched
/// column leaves the base table for a child table of its own, same element
/// path. The map is deliberately not shred-consistent afterwards —
/// `upgrade` only reads table shapes, and nothing is written through it.
/// `Patient.multipleBirth[x]` relocated out of the base table into a table of
/// its own — the shape a `G2.6a` force-split produces.
///
/// It must be a *faithful* relocation, not merely a plausible one. An earlier
/// version of this helper moved the single column `multiple_birth_boolean` and
/// stopped there, which is enough to make the DDL diff report a move — and so
/// enough for the two `O10.4b` tests below, which only ever check that the
/// upgrade refuses or proceeds. It is not enough to *write* through the
/// resulting map: `shred` routes an element by `Elem.table` in the node arena,
/// not by which table happens to list the column, so it kept sending
/// `multipleBirthBoolean` to the base table and the insert panicked on a
/// column that was no longer there. `O10.4c` is the first caller that shreds
/// through the moved map, which is why it was the first to notice.
///
/// A force-split choice owns its table for **every** variant, so both
/// `multiple_birth_boolean` and `multiple_birth_integer` move, and the choice
/// element is repointed at the new table.
fn with_multiple_birth_moved(full: &fhir_postgresql_map::RelMap) -> fhir_postgresql_map::RelMap {
    use fhir_postgresql_map::model::{Table, TableKind};
    let mut m = full.clone();
    let rm = m.resources.get_mut("Patient").expect("Patient is mapped");
    let base = &mut rm.tables[0];
    let mut cols = Vec::new();
    let mut i = 0;
    while i < base.cols.len() {
        if base.cols[i].name.starts_with("multiple_birth") {
            cols.push(base.cols.remove(i));
        } else {
            i += 1;
        }
    }
    assert!(
        cols.iter().any(|c| c.name == "multiple_birth_boolean"),
        "multiple_birth_boolean in the base table"
    );
    let moved_to = u32::try_from(rm.tables.len()).expect("table index fits");
    rm.tables.push(Table {
        norm_cols: Vec::new(),
        adjunct_cols: Vec::new(),
        name: "patient_multiple_birth_moved".into(),
        kind: TableKind::Elem,
        path: "Patient.multipleBirth[x]".into(),
        cols,
    });
    let mut repointed = 0;
    for node in &mut rm.nodes {
        for e in &mut node.elems {
            if e.json == "multipleBirth" {
                e.table = Some(moved_to);
                repointed += 1;
            }
        }
    }
    assert_eq!(repointed, 1, "exactly one multipleBirth choice element");
    m
}

/// A moved column whose source holds data refuses by name — with the
/// destructive flag SET, because acknowledging a drop is not acknowledging
/// a relocation (`O10.4b`).
#[tokio::test]
async fn a_data_bearing_moved_column_refuses_despite_the_flag() {
    let Some(_db) = common::test_db() else {
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };

    let full = fhir_postgresql_gen::generate(&defs, "movetest").expect("generate");
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");
    store
        .put(&json!({"resourceType": "Patient", "id": "mb",
                     "multipleBirthBoolean": true}))
        .await
        .expect("seed");

    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, Arc::new(with_multiple_birth_moved(&full)))
        .await
        .expect("connect moved");
    let err = store
        .upgrade("moved-sum", true)
        .await
        .expect_err("a data-bearing move must refuse even with allow_destructive");
    let msg = err.to_string();
    assert!(
        msg.contains("moved column") && msg.contains("multiple_birth_boolean"),
        "the refusal must name the moved column: {msg}"
    );
    assert!(
        msg.contains("re-put") || msg.contains("reload"),
        "the refusal must name the disposition: {msg}"
    );
}

/// The same move over an empty source proceeds: that drop abandons nothing.
#[tokio::test]
async fn a_moved_column_with_no_data_proceeds() {
    let Some(_db) = common::test_db() else {
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };

    let full = fhir_postgresql_gen::generate(&defs, "movetest2").expect("generate");
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");
    store
        .put(&json!({"resourceType": "Patient", "id": "nb",
                     "name": [{"family": "Quiet"}]}))
        .await
        .expect("seed without multipleBirth");

    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, Arc::new(with_multiple_birth_moved(&full)))
        .await
        .expect("connect moved");
    let report = store
        .upgrade("moved-sum", true)
        .await
        .expect("an empty-source move is an ordinary destructive upgrade");
    assert!(report.additive > 0, "the new table must have been created");
}

/// The same relocation, carried rather than refused (`O10.4c`).
///
/// The move is the synthetic one the two tests above already use — this port
/// has no *real* pre-`G2.6a` fixture to point at, because that force-split was
/// driven by InnoDB's row limit and did not relocate anything in PostgreSQL.
/// `fhir-sqlite` tests against its real stored map; here the relocation has to
/// be constructed, and `with_multiple_birth_moved` is the construction both
/// refusal tests are already gated on.
///
/// What it pins is the whole `O10.4c` contract: the plain upgrade still
/// refuses, the opt-in carries the data, the resource comes back
/// byte-identical, `version_id` and `last_updated` survive because a
/// representation change is not a new version, and no history entry is
/// written for it.
#[tokio::test]
async fn reshred_carries_data_across_a_moved_column() {
    let Some(_db) = common::test_db() else {
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };

    let full = fhir_postgresql_gen::generate(&defs, "reshredtest").expect("generate");
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");
    let doc = json!({"resourceType": "Patient", "id": "mb",
                     "multipleBirthBoolean": true,
                     "name": [{"family": "Twin"}]});
    let put = store.put(&doc).await.expect("seed");
    assert_eq!(put.version_id, 1);
    let before = store
        .history("Patient", "mb")
        .await
        .expect("history before");

    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, Arc::new(with_multiple_birth_moved(&full)))
        .await
        .expect("connect moved");

    // Without the opt-in the refusal still fires: O10.4c is a door, not a
    // change of default.
    let err = store
        .upgrade("moved-sum", true)
        .await
        .expect_err("the move holds data; the plain upgrade must still refuse");
    assert!(err.to_string().contains("moved column"), "got: {err}");

    let report = store
        .upgrade_with(
            "moved-sum",
            fhir_postgresql_store::UpgradeOpts {
                allow_destructive: true,
                reshred_moved: true,
            },
        )
        .await
        .expect("re-shred upgrade");
    assert_eq!(
        report.reshredded, 1,
        "one resource crossed the shape change"
    );

    let got = store
        .get("Patient", "mb")
        .await
        .expect("get")
        .expect("still there");
    assert_eq!(
        got.resource.get("multipleBirthBoolean"),
        doc.get("multipleBirthBoolean"),
        "the relocated element must survive the move"
    );
    assert_eq!(got.resource.get("name"), doc.get("name"));
    assert_eq!(
        got.version_id, 1,
        "a representation change is not a new version"
    );
    let after = store.history("Patient", "mb").await.expect("history after");
    assert_eq!(
        after.len(),
        before.len(),
        "the re-shred must not write a history entry"
    );
}

/// Rerunning a completed re-shred carries nothing: the sources are empty and
/// the map already matches, so `reshredded` is zero and the upgrade is the
/// no-op every other upgrade path promises to be.
#[tokio::test]
async fn a_second_reshred_upgrade_carries_nothing() {
    let Some(_db) = common::test_db() else {
        return;
    };
    let Some(defs) = spec_defs() else {
        eprintln!("skipping: no spec dir");
        return;
    };

    let full = fhir_postgresql_gen::generate(&defs, "reshredtest2").expect("generate");
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");
    store
        .put(&json!({"resourceType": "Patient", "id": "mb",
                     "multipleBirthBoolean": false}))
        .await
        .expect("seed");

    let moved = Arc::new(with_multiple_birth_moved(&full));
    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, Arc::clone(&moved))
        .await
        .expect("connect moved");
    let opts = fhir_postgresql_store::UpgradeOpts {
        allow_destructive: true,
        reshred_moved: true,
    };
    let first = store
        .upgrade_with("moved-sum", opts)
        .await
        .expect("first upgrade");
    assert_eq!(first.reshredded, 1);

    let cfg = fhir_postgresql_store::pg_config(None).expect("cfg");
    let store = Store::connect(cfg, moved).await.expect("reconnect moved");
    let second = store
        .upgrade_with("moved-sum", opts)
        .await
        .expect("second upgrade");
    assert_eq!(second.reshredded, 0, "nothing left to carry");
    assert_eq!(second.additive, 0, "and nothing left to add");
}
