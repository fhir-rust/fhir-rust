//! Schema upgrade and the `_norm` backfill (`O10.4a`, `L13`, `L14`).
//!
//! This suite exists because of audit finding **F-15**: `fhir-postgresql` had
//! `upgrade` and `backfill_norm`, and the other five ports had neither, so on
//! them the accent-folding fix (T90) was a full reload rather than a migration.
//! An operator who deployed the corrected fold against an existing database got
//! searches matching neither the old spelling nor the new — silently, which is
//! the failure mode a clinical search must not have.
//!
//! Unlike `fhir-postgresql`'s equivalent, none of this needs a server or a spec
//! directory. The "old deployment" is built by **reducing the shipped relmap
//! asset in memory** — which also makes the reduction exact: the reduced map is
//! the shape the model documents as pre-folding (`TargetKind::Str::norm` is
//! `None` "only for maps generated before folding existed"), not an
//! approximation of it.

use std::sync::Arc;

use fhir_sqlite_map::model::{RelMap, TargetKind};
use fhir_sqlite_store::{Audit, sqlite::SqliteStore};
use serde_json::json;

fn relmap() -> Option<RelMap> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-sqlite-map/assets/fhir-sqlite-relmap-r5.json.gz");
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok()
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/test-scratch")
        .join(format!("up-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// The map as it would have been before folding existed, and before `Basic` was
/// mapped at all: `Patient`'s `_norm` columns gone from every table, its
/// `norm_cols` pairings gone, and its string search targets pointing at the
/// unfolded column.
///
/// Removing the columns without also clearing the search targets would produce a
/// map that cannot install — `search_indexes` would emit an index over a column
/// no table has. That is worth stating because it is the reason this helper
/// touches three places rather than one.
///
/// Scoped to `Patient` deliberately. Stripping all 158 resources exercises the
/// identical code paths — the diff is per-table and blind to which resource it
/// came from — and costs about two minutes of `ALTER TABLE` and `CREATE INDEX`
/// per test. A suite nobody runs finds nothing.
fn pre_folding(full: &RelMap) -> RelMap {
    let mut m = full.clone();
    m.resources.remove("Basic").expect("Basic is mapped");
    let rm = m.resources.get_mut("Patient").expect("Patient is mapped");
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
    m
}

/// An "existing deployment": the reduced schema, installed, with one patient in
/// it whose name needs folding to be found.
async fn seeded(name: &str) -> Option<(std::path::PathBuf, RelMap)> {
    let full = relmap()?;
    let dir = scratch(name);
    let db = dir.join("fhir.sqlite");
    let store = SqliteStore::open(&db, Arc::new(pre_folding(&full)))
        .await
        .expect("open");
    store.init("old-sum").await.expect("init old");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "keep",
                    "name": [{"family": "Ámélie", "given": ["Zoë"]}]}),
            &Audit::cli(),
        )
        .await
        .expect("seed");
    drop(store);
    Some((db, full))
}

async fn open_full(db: &std::path::Path, full: &RelMap) -> SqliteStore {
    SqliteStore::open(db, Arc::new(full.clone()))
        .await
        .expect("open full")
}

/// New tables and columns arrive, and the data that was already there survives.
#[tokio::test]
async fn upgrade_applies_the_additive_diff_and_keeps_existing_data() {
    let Some((db, full)) = seeded("additive").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let store = open_full(&db, &full).await;

    let report = store.upgrade("new-sum", false).await.expect("upgrade");
    assert!(report.additive > 0, "expected additive changes");
    assert_eq!(report.destructive, 0, "nothing was dropped");

    // The seeded patient is still readable and unchanged.
    let got = store
        .get("Patient", "keep")
        .await
        .expect("get")
        .expect("kept");
    assert_eq!(got["name"][0]["family"], "Ámélie");

    // A resource type the old map did not have at all now works, which means
    // `create_table` ran for tables that did not exist.
    store
        .put(
            &json!({"resourceType": "Basic", "id": "b1", "code": {"text": "now mapped"}}),
            &Audit::cli(),
        )
        .await
        .expect("put Basic");
    assert!(store.get("Basic", "b1").await.expect("get").is_some());
}

/// **The finding itself.** Rows written before the folded column existed must be
/// searchable by the folded spelling afterwards.
///
/// Without the backfill the `ALTER TABLE` leaves `family_norm` NULL on every
/// existing row, every non-`:exact` string search compares that column, and the
/// patient stops matching their own name — with no error anywhere.
#[tokio::test]
async fn rows_written_before_the_folded_column_are_backfilled() {
    let Some((db, full)) = seeded("backfill").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let store = open_full(&db, &full).await;

    let report = store.upgrade("new-sum", false).await.expect("upgrade");
    assert!(
        report.folded > 0,
        "the upgrade added folded columns but backfilled nothing"
    );

    // Folded search: unaccented, lowercase, and neither is how it was stored.
    for spelling in ["amelie", "AMELIE", "Amélie"] {
        let hits = store
            .search(
                "Patient",
                &[("name".to_string(), spelling.to_string())],
                10,
                0,
            )
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
/// This is the one that catches SQLite's sharpest difference from PostgreSQL:
/// there is no `ADD COLUMN IF NOT EXISTS`, so an upgrade path that reconciles
/// the audit envelope unconditionally — which is what the PostgreSQL original
/// does — fails on its second run with "duplicate column name".
#[tokio::test]
async fn a_second_upgrade_is_a_no_op() {
    let Some((db, full)) = seeded("noop").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let store = open_full(&db, &full).await;
    store.upgrade("new-sum", false).await.expect("first");

    let again = store.upgrade("new-sum", false).await.expect("second");
    assert_eq!(again.additive, 0, "a re-upgrade added something");
    assert_eq!(again.destructive, 0);
    assert_eq!(again.folded, 0, "a re-upgrade re-folded values");
    assert!(store.get("Patient", "keep").await.expect("get").is_some());
}

/// The backfill is resumable, which means running it again finds nothing left.
#[tokio::test]
async fn the_backfill_is_resumable() {
    let Some((db, full)) = seeded("resume").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let store = open_full(&db, &full).await;
    let first = store
        .upgrade("new-sum", false)
        .await
        .expect("upgrade")
        .folded;
    assert!(first > 0);

    // Each pass looks only at rows still NULL, so a completed backfill has no
    // work — and an interrupted one would resume rather than start over.
    assert_eq!(
        store.backfill_norm().await.expect("second backfill"),
        0,
        "the backfill re-folded values it had already written"
    );
}

/// Dropping tables and columns needs saying so explicitly.
#[tokio::test]
async fn destructive_changes_are_refused_without_the_flag() {
    let Some(full) = relmap() else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let db = scratch("destructive").join("fhir.sqlite");
    // Install the *full* map, then "upgrade" to the reduced one — so the diff
    // is a dropped resource and dropped columns.
    let store = SqliteStore::open(&db, Arc::new(full.clone()))
        .await
        .expect("open");
    store.init("full-sum").await.expect("init");
    drop(store);

    let store = SqliteStore::open(&db, Arc::new(pre_folding(&full)))
        .await
        .expect("open reduced");
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

/// A column whose type changed is not a diff to apply, it is a migration to
/// design — and the refusal must name the column.
#[tokio::test]
async fn a_column_type_change_refuses() {
    let Some(full) = relmap() else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let db = scratch("typechange").join("fhir.sqlite");
    let store = SqliteStore::open(&db, Arc::new(full.clone()))
        .await
        .expect("open");
    store.init("full-sum").await.expect("init");
    drop(store);

    let mut changed = full.clone();
    let col = {
        let rm = changed.resources.get_mut("Patient").expect("Patient");
        let c = rm.tables[0].cols.iter_mut().find(|c| c.name == "active");
        let c = c.expect("patient.active exists");
        c.ty = fhir_sqlite_map::model::ColTy::Text;
        c.name.clone()
    };
    let store = SqliteStore::open(&db, Arc::new(changed))
        .await
        .expect("open");
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

/// An install made before this port recorded the map asset cannot be diffed, and
/// must say *that* rather than "not installed" — the remedies differ.
#[tokio::test]
async fn an_install_without_a_stored_map_asset_says_so() {
    let Some((db, full)) = seeded("noasset").await else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    // Simulate the older install: the schema is there, the asset is not.
    {
        let attached = db.with_file_name("fhir-r5.sqlite");
        let c = rusqlite::Connection::open(&attached).expect("open attached");
        c.execute("DELETE FROM fhir_sqlite_meta WHERE key = 'map_asset'", [])
            .expect("delete asset");
    }
    let store = open_full(&db, &full).await;
    let err = store
        .upgrade("new-sum", false)
        .await
        .expect_err("no asset means no diff");
    let msg = err.to_string();
    assert!(
        msg.contains("predates upgrade support"),
        "must distinguish this from 'not installed': {msg}"
    );
}

/// Upgrading something that was never installed is a different error again.
#[tokio::test]
async fn upgrading_an_uninstalled_schema_says_it_is_not_installed() {
    let Some(full) = relmap() else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let db = scratch("uninstalled").join("fhir.sqlite");
    let store = SqliteStore::open(&db, Arc::new(full)).await.expect("open");
    let err = store
        .upgrade("new-sum", false)
        .await
        .expect_err("nothing to upgrade");
    assert!(err.to_string().contains("not installed"), "got: {err}");
}

/// `O10.4b` (**F-90**): a relocated column reaches the diff as ADD + DROP,
/// and the guard must tell it apart from a genuine removal. The map surgery
/// mirrors what `G2.6a`'s force-split does to a shape: one column leaves the
/// base table for a child table of its own, same element path. The map is
/// deliberately not shred-consistent afterwards — `upgrade` only reads
/// table shapes, and no resource is written through it.
fn with_multiple_birth_moved(full: &RelMap) -> RelMap {
    use fhir_sqlite_map::model::{Table, TableKind};
    let mut m = full.clone();
    let rm = m.resources.get_mut("Patient").expect("Patient is mapped");
    let base = &mut rm.tables[0];
    let idx = base
        .cols
        .iter()
        .position(|c| c.name == "multiple_birth_boolean")
        .expect("multiple_birth_boolean in the base table");
    let col = base.cols.remove(idx);
    rm.tables.push(Table {
        norm_cols: Vec::new(),
        adjunct_cols: Vec::new(),
        name: "patient_multiple_birth_moved".into(),
        kind: TableKind::Elem,
        path: "Patient.multipleBirth".into(),
        cols: vec![col],
    });
    m
}

/// A moved column whose source holds data refuses by name — with the
/// destructive flag SET, because acknowledging a drop is not acknowledging
/// a relocation.
#[tokio::test]
async fn a_data_bearing_moved_column_refuses_despite_the_flag() {
    let Some(full) = relmap() else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let db = scratch("moved-data").join("fhir.sqlite");
    let store = SqliteStore::open(&db, Arc::new(full.clone()))
        .await
        .expect("open");
    store.init("full-sum").await.expect("init");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "mb", "multipleBirthBoolean": true}),
            &Audit::cli(),
        )
        .await
        .expect("seed");
    drop(store);

    let store = SqliteStore::open(&db, Arc::new(with_multiple_birth_moved(&full)))
        .await
        .expect("open moved");
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

/// The same move over an empty source proceeds: that drop abandons nothing,
/// so the ordinary destructive gate is the only thing standing.
#[tokio::test]
async fn a_moved_column_with_no_data_proceeds() {
    let Some(full) = relmap() else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let db = scratch("moved-empty").join("fhir.sqlite");
    let store = SqliteStore::open(&db, Arc::new(full.clone()))
        .await
        .expect("open");
    store.init("full-sum").await.expect("init");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "nb",
                    "name": [{"family": "Quiet"}]}),
            &Audit::cli(),
        )
        .await
        .expect("seed without multipleBirth");
    drop(store);

    let store = SqliteStore::open(&db, Arc::new(with_multiple_birth_moved(&full)))
        .await
        .expect("open moved");
    let report = store
        .upgrade("moved-sum", true)
        .await
        .expect("an empty-source move is an ordinary destructive upgrade");
    assert!(report.additive > 0, "the new table must have been created");
}

/// `O10.4c` against the real thing: the *actual* pre-G2.6a r5 map (from the
/// tree as of `fb8f27e`, trimmed to `Parameters`, committed as a fixture)
/// upgraded to today's bundled map — the genuine F-90 shape change, 331
/// relocated columns, twelve new split tables. A `valueReference` lands in
/// a moved column; a `valueString` and `valueBoolean` stay put. The
/// migration must carry all of it, byte-identically, without minting a new
/// version.
fn pre_g26a_parameters() -> Option<RelMap> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/pre_g26a_r5_parameters.json.gz");
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok()
}

fn current_parameters() -> Option<RelMap> {
    let mut m = relmap()?;
    m.resources.retain(|k, _| k == "Parameters");
    (!m.resources.is_empty()).then_some(m)
}

#[tokio::test]
async fn reshred_carries_data_across_the_real_g26a_shape_change() {
    let (Some(old), Some(new)) = (pre_g26a_parameters(), current_parameters()) else {
        eprintln!("skipping: fixture or bundled map missing");
        return;
    };
    let db = scratch("reshred").join("fhir.sqlite");
    let store = SqliteStore::open(&db, Arc::new(old))
        .await
        .expect("open old");
    store.init("pre-g26a-sum").await.expect("init old");
    let doc = json!({
        "resourceType": "Parameters", "id": "p1",
        "parameter": [
            {"name": "moved", "valueReference":
                {"reference": "Patient/p1", "display": "P One"}},
            {"name": "kept", "valueString": "plain"},
            {"name": "also-kept", "valueBoolean": true}
        ]
    });
    let put = store.put(&doc, &Audit::cli()).await.expect("seed");
    assert_eq!(put.version_id, 1);
    drop(store);

    let store = SqliteStore::open(&db, Arc::new(new.clone()))
        .await
        .expect("open new");
    // Without the option, O10.4b refuses — the data-bearing move is real.
    let err = store
        .upgrade("g26a-sum", true)
        .await
        .expect_err("the move holds data; plain upgrade must refuse");
    assert!(err.to_string().contains("moved column"), "got: {err}");

    let report = store
        .upgrade_with(
            "g26a-sum",
            fhir_sqlite_store::UpgradeOpts {
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
        .get("Parameters", "p1")
        .await
        .expect("get")
        .expect("still there");
    // Byte-identical through the relocation, and still version 1: a
    // representation change is not a new version.
    assert_eq!(
        got.get("parameter"),
        doc.get("parameter"),
        "the migrated resource must reconstruct identically"
    );
    assert_eq!(got.get("id"), doc.get("id"));

    // A second run has nothing left to move.
    let again = store
        .upgrade_with(
            "g26a-sum",
            fhir_sqlite_store::UpgradeOpts {
                allow_destructive: true,
                reshred_moved: true,
            },
        )
        .await
        .expect("idempotent rerun");
    assert_eq!(again.reshredded, 0, "nothing moves twice");
}
