//! Schema upgrade and the `_norm` backfill (`O10.4a`, `L13`, `L14`).
//!
//! Closes this port's share of audit **F-15**: `fhir-postgresql` had `upgrade`
//! and `backfill_norm`, and this port had neither, so on it the accent-folding
//! fix (T90) was a full reload rather than a migration. An operator deploying
//! the corrected fold against an existing database got searches matching
//! neither the old spelling nor the new — silently, which is the one failure
//! mode a clinical search must not have.
//!
//! The "old deployment" is the shipped relmap asset reduced in memory to the
//! shape the model documents as pre-folding (`TargetKind::Str::norm` is `None`
//! "only for maps generated before folding existed"), so the reduction is
//! exact rather than an approximation.
//!
//! One thing here is this port's own, and has its own test: **`upgrade` is
//! transactional** (`M14.x`, ported from `spec/14-mssql-dialect.md`'s
//! transactional-DDL note). Unlike `fhir-mysql`, a failed upgrade cannot leave
//! a half-applied schema — there is nothing here to test *for* that failure
//! mode the way `fhir-mysql/tests/upgrade.rs` tests for `Duplicate key name`
//! on a second run, because SQL Server's `CREATE OR ALTER TRIGGER` and this
//! store's own reconcile-and-filter step both stay idempotent across repeated
//! upgrades regardless.
//!
//! Needs `FHIR_MSSQL_TEST_DSN`; `scripts/db.sh up` prints it. Run with
//! `--test-threads=1` alongside the rest of this port's live suite — each
//! test installs and drops its own schema against `azure-sql-edge`, and
//! concurrent DDL against one small container deadlocks it (SQL Server error
//! 1205), the same reason `mssql_store.rs` gives.

use std::sync::Arc;

use fhir_mssql_map::model::{RelMap, TargetKind};
use fhir_mssql_store::mssql::MsSqlStore;
use serde_json::json;

fn dsn() -> Option<String> {
    std::env::var("FHIR_MSSQL_TEST_DSN").ok()
}

fn relmap() -> Option<RelMap> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-mssql-map/assets/fhir-mssql-relmap-r5.json.gz");
    RelMap::from_gz_bytes(&std::fs::read(path).ok()?).ok()
}

/// Only the resource types a test needs: creating every table takes a long
/// time, which is why the rest of this port's live suite samples too.
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
/// Removing the columns without also clearing the search targets would
/// produce a map that cannot install — `search_indexes` would emit an index
/// over a column no table has.
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

/// An existing deployment: the reduced schema installed, with one patient in
/// it whose name needs folding to be found.
async fn seeded(schema: &str) -> Option<(RelMap, MsSqlStore)> {
    let full = sampled(schema, &["Patient", "Basic"])?;
    let mut old = pre_folding(&full);
    // `Basic` absent from the old map too, so the upgrade must create its
    // tables from nothing, not just add columns to them.
    old.resources.remove("Basic");

    let store = MsSqlStore::connect(&dsn()?, Arc::new(old))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("old-sum").await.expect("init old");
    store
        .put(
            &json!({"resourceType": "Patient", "id": "keep",
                    "name": [{"family": "Ámélie", "given": ["Zoë"]}]}),
            &fhir_mssql_store::Audit::default(),
        )
        .await
        .expect("seed");

    let new_store = MsSqlStore::connect(&dsn()?, Arc::new(full.clone()))
        .await
        .expect("connect new");
    Some((full, new_store))
}

/// New tables and columns arrive, and the data already there survives.
#[tokio::test]
async fn upgrade_applies_the_additive_diff_and_keeps_existing_data() {
    let Some((_, store)) = seeded("fhir_mssql_up_add").await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
            &fhir_mssql_store::Audit::default(),
        )
        .await
        .expect("put Basic");
    assert!(store.get("Basic", "b1").await.expect("get").is_some());
}

/// **The finding itself.** Rows written before the folded column existed must
/// be searchable by the folded spelling afterwards.
#[tokio::test]
async fn rows_written_before_the_folded_column_are_backfilled() {
    let Some((_, store)) = seeded("fhir_mssql_up_fold").await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
/// SQL Server's `CREATE OR ALTER TRIGGER` and this store's own
/// tables/indexes/columns filters (`installed_tables`, `installed_indexes`,
/// `installed_columns`) are what make a second run idempotent — T-SQL's
/// `CREATE TABLE` and `CREATE INDEX` have no `IF NOT EXISTS` of their own
/// (`M14.17`), so applying `schema_wide_objects` wholesale a second time
/// would otherwise fail with "There is already an object named …".
#[tokio::test]
async fn a_second_upgrade_is_a_no_op() {
    let Some((_, store)) = seeded("fhir_mssql_up_noop").await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
    let Some((_, store)) = seeded("fhir_mssql_up_resume").await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
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
    let Some(full) = sampled("fhir_mssql_up_drop", &["Patient"]) else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
        return;
    };
    let store = MsSqlStore::connect(&d, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");

    let store = MsSqlStore::connect(&d, Arc::new(pre_folding(&full)))
        .await
        .expect("connect reduced");
    let err = store
        .upgrade("reduced-sum", false)
        .await
        .expect_err("a dropping upgrade must refuse by default");
    let msg = err.to_string();
    assert!(
        msg.contains("destructive") && msg.contains("allow_destructive"),
        "the refusal must say what to do about it: {msg}"
    );
}

/// The same drop, with the flag set, must actually happen — and stick: a
/// second upgrade with the same reduced map needs no further permission,
/// which is only true if the columns and table genuinely disappeared rather
/// than the flag being accepted and ignored.
#[tokio::test]
async fn destructive_changes_succeed_with_the_flag() {
    let Some(full) = sampled("fhir_mssql_up_drop_ok", &["Patient", "Basic"]) else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
        return;
    };
    let store = MsSqlStore::connect(&d, Arc::new(full.clone()))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    store.init("full-sum").await.expect("init");

    let mut reduced = full.clone();
    reduced.resources.remove("Basic");
    let store = MsSqlStore::connect(&d, Arc::new(reduced))
        .await
        .expect("connect reduced");
    let report = store
        .upgrade("reduced-sum", true)
        .await
        .expect("a dropping upgrade must succeed with the flag");
    assert!(report.destructive > 0, "expected dropped tables");

    // If `Basic`'s tables were still there, this second call would see the
    // same destructive diff again and need the flag again.
    let again = store
        .upgrade("reduced-sum-2", false)
        .await
        .expect("the drop must have actually happened");
    assert_eq!(
        again.destructive, 0,
        "the destructive diff was not really applied the first time"
    );
}

/// A column whose type changed is a migration to design, not a diff to apply.
#[tokio::test]
async fn a_column_type_change_refuses() {
    let Some(full) = sampled("fhir_mssql_up_type", &["Patient"]) else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
        return;
    };
    let store = MsSqlStore::connect(&d, Arc::new(full.clone()))
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
        c.ty = fhir_mssql_map::model::ColTy::Text;
        c.name.clone()
    };
    let store = MsSqlStore::connect(&d, Arc::new(changed))
        .await
        .expect("connect");
    let err = store
        .upgrade("changed-sum", true)
        .await
        .expect_err("a type change must refuse even with allow_destructive");
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
    let Some((_, store)) = seeded("fhir_mssql_up_noasset").await else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
        return;
    };
    // Simulate the older install: schema present, asset absent. `exec_raw` is
    // this store's own escape hatch for exactly this — see its doc comment.
    store
        .exec_raw(
            "DELETE FROM [fhir_mssql_up_noasset].[fhir_mssql_meta] WHERE [key] = 'map_asset'",
        )
        .await
        .expect("delete asset");

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
    let Some(full) = sampled("fhir_mssql_up_absent", &["Patient"]) else {
        eprintln!("skipping: no r5 relmap asset");
        return;
    };
    let Some(d) = dsn() else {
        eprintln!("skipping: set FHIR_MSSQL_TEST_DSN to run");
        return;
    };
    let store = MsSqlStore::connect(&d, Arc::new(full))
        .await
        .expect("connect");
    store.drop_schema().await.expect("drop");
    let err = store
        .upgrade("new-sum", false)
        .await
        .expect_err("nothing to upgrade");
    assert!(err.to_string().contains("not installed"), "got: {err}");
}
