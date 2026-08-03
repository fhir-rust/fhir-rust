//! Does the emitted DDL actually execute in SQLite?
//!
//! The unit tests in `ddl.rs` assert the *shape* of individual statements, which
//! catches a forgotten `bytea` but not a syntax error, a reserved word, or a
//! constraint SQLite rejects only when it tries to build the table. This runs
//! the whole generated schema — all of R5's several thousand tables — through
//! the real engine.
//!
//! Skips silently when the `sqlite3` binary or the relmap asset is unavailable,
//! matching the convention the live tests already use.

use std::path::PathBuf;
use std::process::{Command, Stdio};

fn relmap_path(version: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(format!("fhir-sqlite-relmap-{version}.json.gz"))
}

/// Which `sqlite3` to use.
///
/// `FHIR_SQLITE_SQLITE3` lets `scripts/db.sh` point this at a wrapper that runs
/// a *pinned* sqlite3 inside a container. SQLite's DDL features move between
/// releases, so "passes on my laptop" is not the same claim as "passes on the
/// version CI uses"; this is how the two are made the same claim.
fn sqlite3_bin() -> String {
    std::env::var("FHIR_SQLITE_SQLITE3").unwrap_or_else(|_| "sqlite3".to_string())
}

/// Scratch directory for database files.
///
/// Deliberately under the workspace `target/` rather than `TMPDIR`: when
/// `sqlite3` runs inside a container (see `sqlite3_bin`), the repo is mounted at
/// the same path but the host's temp dir is not, so a `TMPDIR` path would exist
/// for the test and not for the engine.
fn scratch(name: &str) -> std::path::PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/test-scratch")
        .join(format!("{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn have_sqlite3() -> bool {
    Command::new(sqlite3_bin())
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Feed a script to `sqlite3` and return stderr if it failed.
fn run_script(db: &std::path::Path, script: &str) -> Result<(), String> {
    use std::io::Write as _;
    let sql = db.with_extension("sql");
    std::fs::write(&sql, script).map_err(|e| e.to_string())?;

    let mut child = Command::new(sqlite3_bin())
        .arg(db)
        // Stop at the first error instead of reporting only the last.
        .arg("-bail")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(format!(".read {}\n", sql.display()).as_bytes())
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() && out.stderr.is_empty() {
        return Ok(());
    }
    Err(String::from_utf8_lossy(&out.stderr).to_string())
}

fn install(version: &str) {
    if !have_sqlite3() {
        eprintln!("skipping: no usable sqlite3 ({})", sqlite3_bin());
        return;
    }
    let path = relmap_path(version);
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("skipping: no relmap at {}", path.display());
        return;
    };
    let map = fhir_sqlite_map::model::RelMap::from_gz_bytes(&bytes).expect("relmap decodes");
    let statements = fhir_sqlite_map::ddl::ddl(&map);
    assert!(
        statements.len() > 100,
        "{version}: only {} statements, asset looks wrong",
        statements.len()
    );

    let dir = scratch(&format!("ddl-{version}"));
    let main = dir.join("main.sqlite");
    let attached = dir.join(format!("{version}.sqlite"));

    // The DDL uses qualified names, so the version's database must be attached
    // under its schema name first — this is the port's replacement for
    // `CREATE SCHEMA` (M14.15/M14.17).
    let mut script = String::new();
    script.push_str("PRAGMA foreign_keys = ON;\n");
    script.push_str(&format!(
        "ATTACH DATABASE '{}' AS \"{}\";\n",
        attached.display(),
        map.schema
    ));
    // One transaction: SQLite's DDL is transactional, which is why the staged
    // schema-and-rename dance PostgreSQL needs is not carried over (M14.16).
    script.push_str("BEGIN;\n");
    for s in &statements {
        script.push_str(s);
        script.push_str(";\n");
    }
    script.push_str("COMMIT;\n");

    if let Err(e) = run_script(&main, &script) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("{version}: generated DDL was rejected by SQLite:\n{e}");
    }

    // Prove it is a real schema and not an empty file that silently succeeded.
    let check = dir.join("check.sqlite");
    let verify = format!(
        "ATTACH DATABASE '{}' AS \"{}\";\n\
         SELECT count(*) FROM \"{}\".sqlite_master WHERE type='table';\n",
        attached.display(),
        map.schema,
        map.schema
    );
    let out = Command::new(sqlite3_bin())
        .arg(&check)
        .arg("-bail")
        .arg(&verify)
        .output()
        .expect("sqlite3 runs");
    let tables: usize = String::from_utf8_lossy(&out.stdout)
        .trim()
        .lines()
        .last()
        .unwrap_or("0")
        .trim()
        .parse()
        .unwrap_or(0);
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        tables > 100,
        "{version}: schema installed but only {tables} tables exist"
    );
    eprintln!(
        "{version}: {} statements, {tables} tables installed",
        statements.len()
    );
}

#[test]
fn r5_schema_installs_in_real_sqlite() {
    install("r5");
}

#[test]
fn r4_schema_installs_in_real_sqlite() {
    install("r4");
}

#[test]
fn r3_schema_installs_in_real_sqlite() {
    install("r3");
}

#[test]
fn append_only_trigger_actually_refuses_an_update() {
    // The trigger is the load-bearing part of M3.17, and a trigger that parses
    // but never fires would be worse than none at all.
    if !have_sqlite3() {
        eprintln!("skipping: no usable sqlite3 ({})", sqlite3_bin());
        return;
    }
    let dir = scratch("trg");

    let triggers = fhir_sqlite_map::ddl::append_only_triggers("r5", "patient_history").join(";\n");
    let erasure = fhir_sqlite_map::ddl::schema_wide_objects("r5")
        .into_iter()
        .find(|s| s.contains("fhir_sqlite_erasure"))
        .expect("erasure table is emitted");

    // Each scenario gets its own pair of database files: the attached file
    // persists, so sharing one would make the second setup fail on a table that
    // already exists rather than on the behaviour under test.
    let scenario = |name: &str, body: &str| -> Result<(), String> {
        let main = dir.join(format!("{name}-main.sqlite"));
        let att = dir.join(format!("{name}-r5.sqlite"));
        run_script(
            &main,
            &format!(
                "ATTACH DATABASE '{}' AS \"r5\";\n\
                 CREATE TABLE \"r5\".\"patient_history\" (\"id\" TEXT NOT NULL, \"version_id\" INTEGER NOT NULL, PRIMARY KEY (\"id\",\"version_id\"));\n\
                 {erasure};\n\
                 {triggers};\n\
                 INSERT INTO \"r5\".\"patient_history\" VALUES ('p1', 1);\n\
                 {body}",
                att.display()
            ),
        )
    };

    // An UPDATE must be refused outright — there is no escape hatch.
    let e = scenario(
        "upd",
        "UPDATE \"r5\".\"patient_history\" SET \"version_id\" = 2;\n",
    )
    .expect_err("UPDATE on history should have been refused");
    assert!(e.contains("append-only"), "unexpected error: {e}");

    // A DELETE must be refused too, while the erasure flag is absent.
    let e = scenario("del", "DELETE FROM \"r5\".\"patient_history\";\n")
        .expect_err("DELETE on history should have been refused");
    assert!(e.contains("append-only"), "unexpected error: {e}");

    // With the flag row present, the same DELETE is permitted (M3.18 erasure).
    let ok = scenario(
        "erase",
        "BEGIN;\n\
         INSERT INTO \"r5\".\"fhir_sqlite_erasure\" VALUES ('t');\n\
         DELETE FROM \"r5\".\"patient_history\";\n\
         DELETE FROM \"r5\".\"fhir_sqlite_erasure\";\n\
         COMMIT;\n",
    );
    let _ = std::fs::remove_dir_all(&dir);
    ok.expect("erasure-flagged DELETE should be permitted");
}
