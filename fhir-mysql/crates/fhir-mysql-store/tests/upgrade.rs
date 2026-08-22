//! Schema upgrade and the `_norm` backfill (`O10.4a`, `L13`, `L14`).
//!
//! Closes this port's share of audit **F-15**: `fhir-postgresql` had `upgrade`
//! and `backfill_norm` and the other five had neither, so on them the
//! accent-folding fix (T90) was a full reload rather than a migration. An
//! operator deploying the corrected fold against an existing database got
//! searches matching neither the old spelling nor the new — silently.
//!
//! The "old deployment" is the shipped relmap asset reduced in memory to the
//! shape the model documents as pre-folding (`TargetKind::Str::norm` is `None`
//! "only for maps generated before folding existed"), so the reduction is exact
//! rather than an approximation.
//!
//! Two things here are MySQL's doing and not choices, and each has its own test:
//!
//! - **No transactional DDL.** A failed upgrade leaves a partial schema. That
//!   cannot be prevented, so it is reported (`M14.35`).
//! - **`CREATE INDEX` is not idempotent** — MySQL has no `IF NOT EXISTS` for it
//!   — so reconciling the access-log indexes wholesale fails on the second run
//!   with `Duplicate key name` (audit **F-28**). `a_second_upgrade_is_a_no_op`
//!   is what catches that.
//!
//! Needs `FHIR_MYSQL_TEST_DSN`; `scripts/db.sh up` prints it.

use std::sync::Arc;

use fhir_mysql_map::model::{RelMap, TargetKind};
use fhir_mysql_store::mysql::MySqlStore;
use serde_json::json;

fn dsn() -> Option<String> {
    std::env::var("FHIR_MYSQL_TEST_DSN").ok()
}

fn relmap() -> Option<RelMap> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-mysql-map/assets/fhir-mysql-relmap-r5.json.gz");
    RelMap::from_gz_bytes(&std::fs::read(path).ok()?).ok()
}

/// Only the resource types a test needs: creating every InnoDB table takes tens
/// of minutes, which is why the rest of this suite samples too.
fn sampled(schema: &str, want: &[&str]) -> Option<RelMap> {
    let mut m = relmap()?;
    m.resources.retain(|k, _| want.contains(&k.as_str()));
    assert!(
        !m.resources.is_empty(),
        "none of {want:?} are in the r5 map"
    );
    m.schema = schema.to_string();
    Some(m)
}

/// The map as it would have been before folding existed: `Patient`'s `_norm`
/// columns gone, its `norm_cols` pairings gone, and its string search targets
/// pointing at the unfolded column.
///
/// Removing the columns without also clearing the search targets would produce a
/// map that cannot install — `search_indexes` would emit an index over a column
/// no table has.
fn pre_folding(m: &RelMap) -> RelMap {
    let mut m = m.clone();
    if let Some(rm) = m.resources.get_mut("Patient") {
        for t in &mut rm.tables {
            let norm: Vec<String> = t.norm_cols.iter().map(|(_, d)| d.clone()).collect();
            t.cols.retain(|c| !norm.contains(&c.name));
            t.norm_cols.clear();
        }
        for def in &mut rm.search {
            for target in &mut def.targets {
                if let TargetKind::Str { norm, .. } = &mut target.kind {
                    *norm = None;
                }
            }
        }
    }
    m
}

/// An existing deployment: the reduced schema installed, with one patient in it
/// whose name needs folding to be found.
async fn seeded(schema: &str) -> Option<(RelMap, MySqlStore)> {
    let full = sampled(schema, &["Patient", "Basic"])?;
    let old = pre_folding(&full);
    // `Basic` absent from the old map, so the upgrade must create its tables.
    let mut old = old;
    old.resources.remove("Basic");

    let store = MySqlStore::connect(&dsn()?, Arc::new(old))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("old-sum").await.expect("init old");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "keep",
                    "name": [{"family": "Ámélie", "given": ["Zoë"]}]}),
            &fhir_mysql_store::Audit::default(),
        )
        .await
        .expect("seed");

    let new_store = MySqlStore::connect(&dsn()?, Arc::new(full.clone()))
        .await
        .expect("connect new");
    Some((full, new_store))
}

/// New tables and columns arrive, and the data already there survives.
#[tokio::test]
async fn upgrade_applies_the_additive_diff_and_keeps_existing_data() {
    let Some((_, store)) = seeded("fhir_mysql_up_add").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    let report = store.upgrade("new-sum", false).await.expect("upgrade");
    assert!(report.additive > 0, "expected additive changes");
    assert_eq!(report.destructive, 0, "nothing was dropped");

    let got = store
        .get("Patient", "keep")
        .await
        .expect("get")
        .expect("kept");
    assert_eq!(got["name"][0]["family"], "Ámélie");

    store
        .put(
            &json!({"resourceType": "Basic", "id": "b1", "code": {"text": "now mapped"}}),
            &fhir_mysql_store::Audit::default(),
        )
        .await
        .expect("put Basic");
    assert!(store.get("Basic", "b1").await.expect("get").is_some());
}

/// **The finding itself.** Rows written before the folded column existed must be
/// searchable by the folded spelling afterwards.
#[tokio::test]
async fn rows_written_before_the_folded_column_are_backfilled() {
    let Some((_, store)) = seeded("fhir_mysql_up_fold").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    let report = store.upgrade("new-sum", false).await.expect("upgrade");
    assert!(
        report.folded > 0,
        "the upgrade added folded columns but backfilled nothing"
    );

    for spelling in ["amelie", "AMELIE", "Amélie"] {
        let hits = store
            .search("Patient", &[("name".into(), spelling.into())], 10, 0)
            .await
            .expect("search");
        assert!(
            hits.contains(&"keep".to_string()),
            "searching {spelling:?} did not find the patient seeded before the \
             folded column existed — this is exactly F-15"
        );
    }
}

/// A re-upgrade must report nothing and change nothing.
///
/// This is the one that catches **F-28**: MySQL has no
/// `CREATE INDEX IF NOT EXISTS`, so reconciling `schema_wide_objects` wholesale
/// fails here with `Duplicate key name`.
#[tokio::test]
async fn a_second_upgrade_is_a_no_op() {
    let Some((_, store)) = seeded("fhir_mysql_up_noop").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    store.upgrade("new-sum", false).await.expect("first");

    let again = store.upgrade("new-sum", false).await.expect("second");
    assert_eq!(again.additive, 0, "a re-upgrade added something");
    assert_eq!(again.destructive, 0);
    assert_eq!(again.folded, 0, "a re-upgrade re-folded values");
    assert!(store.get("Patient", "keep").await.expect("get").is_some());
}

/// The backfill is resumable, so running it again finds nothing left.
#[tokio::test]
async fn the_backfill_is_resumable() {
    let Some((_, store)) = seeded("fhir_mysql_up_resume").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    let first = store
        .upgrade("new-sum", false)
        .await
        .expect("upgrade")
        .folded;
    assert!(first > 0);
    assert_eq!(
        store.backfill_norm().await.expect("second backfill"),
        0,
        "the backfill re-folded values it had already written"
    );
}

/// Dropping tables and columns needs saying so explicitly.
#[tokio::test]
async fn destructive_changes_are_refused_without_the_flag() {
    let Some(full) = sampled("fhir_mysql_up_drop", &["Patient"]) else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    let store = MySqlStore::connect(&d, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");

    let store = MySqlStore::connect(&d, Arc::new(pre_folding(&full)))
        .await
        .expect("connect reduced");
    let err = store
        .upgrade("reduced-sum", false)
        .await
        .expect_err("a dropping upgrade must refuse by default");
    let msg = err.to_string();
    assert!(
        msg.contains("destructive") && msg.contains("--allow-destructive"),
        "the refusal must say what to do about it: {msg}"
    );
}

/// A column whose type changed is a migration to design, not a diff to apply.
#[tokio::test]
async fn a_column_type_change_refuses() {
    let Some(full) = sampled("fhir_mysql_up_type", &["Patient"]) else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    let store = MySqlStore::connect(&d, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");

    let mut changed = full.clone();
    let col = {
        let rm = changed.resources.get_mut("Patient").expect("Patient");
        let c = rm.tables[0]
            .cols
            .iter_mut()
            .find(|c| c.name == "active")
            .expect("patient.active exists");
        c.ty = fhir_mysql_map::model::ColTy::Text;
        c.name.clone()
    };
    let store = MySqlStore::connect(&d, Arc::new(changed))
        .await
        .expect("connect");
    let err = store
        .upgrade("changed-sum", true)
        .await
        .expect_err("a type change must refuse even with --allow-destructive");
    let msg = err.to_string();
    assert!(
        msg.contains(&col) && msg.contains("manual migration"),
        "the refusal must name the column and say what it needs: {msg}"
    );
}

/// An install predating the stored map asset cannot be diffed, and must say
/// *that* rather than "not installed" — the remedies differ.
#[tokio::test]
async fn an_install_without_a_stored_map_asset_says_so() {
    let Some((full, store)) = seeded("fhir_mysql_up_noasset").await else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    // Simulate the older install: schema present, asset absent. Done over a
    // separate connection because the store deliberately exposes no raw exec.
    {
        use mysql_async::prelude::Queryable;
        let pool = mysql_async::Pool::new(dsn().expect("dsn").as_str());
        let mut c = pool.get_conn().await.expect("conn");
        c.query_drop(
            "DELETE FROM `fhir_mysql_up_noasset`.`fhir_mysql_meta` WHERE `key` = 'map_asset'",
        )
        .await
        .expect("delete asset");
    }
    let _ = full;

    let err = store
        .upgrade("new-sum", false)
        .await
        .expect_err("no asset means no diff");
    assert!(
        err.to_string().contains("predates upgrade support"),
        "must distinguish this from 'not installed': {err}"
    );
}

/// Upgrading something never installed is a different error again.
#[tokio::test]
async fn upgrading_an_uninstalled_schema_says_it_is_not_installed() {
    let Some(full) = sampled("fhir_mysql_up_absent", &["Patient"]) else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    };
    let store = MySqlStore::connect(&d, Arc::new(full))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    let err = store
        .upgrade("new-sum", false)
        .await
        .expect_err("nothing to upgrade");
    assert!(err.to_string().contains("not installed"), "got: {err}");
}

/// `O10.4b` (**F-90**): a relocated column reaches the diff as ADD + DROP,
/// and the guard must tell it apart from a genuine removal. The surgery
/// mirrors what `G2.6a`'s force-split does to a shape: the element's columns
/// leave the base table for a child table of their own, same element path.
///
/// This used to end with "the map is deliberately not shred-consistent
/// afterwards — `upgrade` only reads table shapes, and nothing is written
/// through it". That was true when the only callers were the two refusal
/// tests below, and it stopped being true the moment `O10.4c` shredded
/// through the moved map: `shred` routes an element by `Elem.table` in the
/// node arena, not by which table lists the column, so the old helper kept
/// aiming `multipleBirthBoolean` at the base table and the insert failed on
/// a column that was no longer there.
///
/// So the relocation is now faithful. A force-split choice owns its table for
/// **every** variant, so both `multiple_birth_boolean` and
/// `multiple_birth_integer` move, and the choice element is repointed at the
/// new table.
fn with_multiple_birth_moved(full: &RelMap) -> RelMap {
    use fhir_mysql_map::model::{Table, TableKind};
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
    let Some(full) = sampled("fhir_mysql_up_moved", &["Patient"]) else {
        eprintln!("skipping: no dsn or map");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: no dsn");
        return;
    };
    let store = MySqlStore::connect(&d, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "mb",
                    "multipleBirthBoolean": true}),
            &fhir_mysql_store::Audit::default(),
        )
        .await
        .expect("seed");

    let store = MySqlStore::connect(&d, Arc::new(with_multiple_birth_moved(&full)))
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
    let Some(full) = sampled("fhir_mysql_up_moved2", &["Patient"]) else {
        eprintln!("skipping: no dsn or map");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: no dsn");
        return;
    };
    let store = MySqlStore::connect(&d, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "nb",
                    "name": [{"family": "Quiet"}]}),
            &fhir_mysql_store::Audit::default(),
        )
        .await
        .expect("seed without multipleBirth");

    let store = MySqlStore::connect(&d, Arc::new(with_multiple_birth_moved(&full)))
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
/// Pins the contract: the plain upgrade still refuses, the opt-in carries the
/// data, the resource comes back byte-identical, `version_id` and
/// `last_updated` survive because a representation change is not a new
/// version, and no history entry is written for it.
#[tokio::test]
async fn reshred_carries_data_across_a_moved_column() {
    let Some(full) = sampled("fhir_mysql_up_reshred", &["Patient"]) else {
        eprintln!("skipping: no dsn or map");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: no dsn");
        return;
    };
    let store = MySqlStore::connect(&d, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");
    let doc = json!({"resourceType": "Patient", "id": "mb",
                     "multipleBirthBoolean": true,
                     "name": [{"family": "Twin"}]});
    store
        .put(&doc, &fhir_mysql_store::Audit::default())
        .await
        .expect("seed");
    let before = store
        .history("Patient", "mb")
        .await
        .expect("history before");

    let store = MySqlStore::connect(&d, Arc::new(with_multiple_birth_moved(&full)))
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
            fhir_mysql_store::UpgradeOpts {
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
        got.get("multipleBirthBoolean"),
        doc.get("multipleBirthBoolean"),
        "the relocated element must survive the move"
    );
    assert_eq!(got.get("name"), doc.get("name"));
    let after = store.history("Patient", "mb").await.expect("history after");
    assert_eq!(
        after.len(),
        before.len(),
        "the re-shred must not write a history entry"
    );
}

/// Rerunning a completed re-shred carries nothing: the sources are empty and
/// the map already matches.
#[tokio::test]
async fn a_second_reshred_upgrade_carries_nothing() {
    let Some(full) = sampled("fhir_mysql_up_reshred2", &["Patient"]) else {
        eprintln!("skipping: no dsn or map");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: no dsn");
        return;
    };
    let store = MySqlStore::connect(&d, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "mb",
                    "multipleBirthBoolean": false}),
            &fhir_mysql_store::Audit::default(),
        )
        .await
        .expect("seed");

    let moved = Arc::new(with_multiple_birth_moved(&full));
    let opts = fhir_mysql_store::UpgradeOpts {
        allow_destructive: true,
        reshred_moved: true,
    };
    let store = MySqlStore::connect(&d, Arc::clone(&moved))
        .await
        .expect("connect moved");
    let first = store
        .upgrade_with("moved-sum", opts)
        .await
        .expect("first upgrade");
    assert_eq!(first.reshredded, 1);

    let store = MySqlStore::connect(&d, moved).await.expect("reconnect");
    let second = store
        .upgrade_with("moved-sum", opts)
        .await
        .expect("second upgrade");
    assert_eq!(second.reshredded, 0, "nothing left to carry");
    assert_eq!(second.additive, 0, "and nothing left to add");
}
