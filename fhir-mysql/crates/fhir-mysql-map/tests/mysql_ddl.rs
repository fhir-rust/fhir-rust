//! Does the emitted DDL actually execute on a live server?
//!
//! The unit tests in `ddl.rs` assert the *shape* of individual statements, which
//! catches a forgotten `bytea` but not an over-long index key, a `TEXT` column
//! keyed without a prefix, or a trigger body the parser rejects. This runs the
//! whole generated schema through a real server.
//!
//! Needs `FHIR_MYSQL_TEST_DSN`, e.g.
//! `mysql://root@127.0.0.1:13306`. Skips silently without it, matching the
//! convention the live store tests already use. The schema is created and
//! dropped per run.
//!
//! Note: verified against MariaDB during development. The emitted DDL is
//! deliberately restricted to syntax common to both — no `CREATE TRIGGER IF NOT
//! EXISTS`, no native `JSON` type — but a MySQL run is still the authority.

use std::process::Command;

/// `(host, port, user)` parsed loosely from a DSN, or `None` to skip.
fn dsn() -> Option<(String, String, String)> {
    let raw = std::env::var("FHIR_MYSQL_TEST_DSN").ok()?;
    let rest = raw.strip_prefix("mysql://").unwrap_or(&raw);
    let (user, hostport) = match rest.split_once('@') {
        Some((u, h)) => (u.split(':').next().unwrap_or("root").to_string(), h),
        None => ("root".to_string(), rest),
    };
    let hostport = hostport.split('/').next().unwrap_or(hostport);
    let (host, port) = match hostport.split_once(':') {
        Some((h, p)) => (h.to_string(), p.to_string()),
        None => (hostport.to_string(), "3306".to_string()),
    };
    Some((host, port, user))
}

fn mysql_cli() -> Option<&'static str> {
    ["mysql", "mariadb"].into_iter().find(|c| {
        Command::new(c)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    })
}

/// Render statements into a client script.
///
/// The append-only DELETE trigger has a compound `BEGIN … END` body containing
/// its own semicolons. A driver sends that as one statement and it is fine, but
/// the command-line client splits on `;`, so compound statements are wrapped in
/// `DELIMITER` blocks. This is a property of the harness, not of the DDL.
fn script_of(statements: &[String]) -> String {
    let mut out = String::new();
    for st in statements {
        if st.contains(" BEGIN ") {
            out.push_str("DELIMITER $$\n");
            out.push_str(st);
            out.push_str("$$\nDELIMITER ;\n");
        } else {
            out.push_str(st);
            out.push_str(";\n");
        }
    }
    out
}

/// Run a script through the client's stdin, returning stderr on failure.
fn run_script(script: &str) -> Result<String, String> {
    use std::io::Write as _;
    use std::process::Stdio;
    let (host, port, user) = dsn().ok_or("no dsn")?;
    let cli = mysql_cli().ok_or("no mysql client")?;
    let mut child = Command::new(cli)
        .args([
            "-h",
            &host,
            "-P",
            &port,
            "-u",
            &user,
            "--batch",
            "--raw",
            "--skip-ssl-verify-server-cert",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(script.as_bytes())
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    // The client warns about passwordless TLS; that is not a failure.
    let fatal: Vec<&str> = err
        .lines()
        .filter(|l| l.starts_with("ERROR") || l.contains("errno"))
        .collect();
    if out.status.success() && fatal.is_empty() {
        return Ok(String::from_utf8_lossy(&out.stdout).to_string());
    }
    Err(if fatal.is_empty() {
        err
    } else {
        fatal.join("\n")
    })
}

/// Run one or more plain (non-compound) statements.
fn run(sql: &str) -> Result<String, String> {
    run_script(&format!("{sql};\n"))
}

fn relmap(version: &str) -> Option<fhir_mysql_map::model::RelMap> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(format!("fhir-mysql-relmap-{version}.json.gz"));
    let bytes = std::fs::read(path).ok()?;
    fhir_mysql_map::model::RelMap::from_gz_bytes(&bytes).ok()
}

fn install(version: &str) {
    if dsn().is_none() {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    }
    if mysql_cli().is_none() {
        eprintln!("skipping: no mysql/mariadb client");
        return;
    }
    let Some(map) = relmap(version) else {
        eprintln!("skipping: no relmap asset for {version}");
        return;
    };

    // A throwaway schema name, so a run cannot disturb a real database.
    let schema = format!("fhir_mysql_ddltest_{version}");
    let _ = run(&format!("DROP SCHEMA IF EXISTS `{schema}`"));

    // Creating every InnoDB table with its foreign key takes tens of minutes on
    // a laptop — MySQL is far slower at this than PostgreSQL or SQLite, which is
    // itself worth knowing (M14.22). So the default run installs a bounded
    // sample of resources, which is enough to exercise every table kind and
    // every column type, and `FHIR_MYSQL_DDL_FULL=1` installs all of them.
    let full = std::env::var("FHIR_MYSQL_DDL_FULL").is_ok();
    let mut map = map;
    if !full {
        let limit: usize = std::env::var("FHIR_MYSQL_DDL_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(25);
        map.resources = map.resources.into_iter().take(limit).collect();
    }

    let statements = fhir_mysql_map::ddl::ddl_in(&map, &schema);
    assert!(
        statements.len() > 50,
        "{version}: only {} statements, asset looks wrong",
        statements.len()
    );

    // Apply in batches: one enormous -e argument would exceed the client's
    // limits, and a failure mid-way should name the batch it happened in.
    let mut applied = 0usize;
    for (i, chunk) in statements.chunks(200).enumerate() {
        if let Err(e) = run_script(&script_of(chunk)) {
            let _ = run(&format!("DROP SCHEMA IF EXISTS `{schema}`"));
            panic!("{version}: batch {i} rejected after {applied} statements:\n{e}");
        }
        applied += chunk.len();
    }

    // Prove a real schema exists rather than an empty one that quietly worked.
    let count = run(&format!(
        "SELECT count(*) FROM information_schema.tables WHERE table_schema = '{schema}'"
    ))
    .expect("count query runs");
    let tables: usize = count
        .lines()
        .last()
        .unwrap_or("0")
        .trim()
        .parse()
        .unwrap_or(0);

    // And that the triggers landed — they are the load-bearing part of M3.17.
    let trg = run(&format!(
        "SELECT count(*) FROM information_schema.triggers WHERE trigger_schema = '{schema}'"
    ))
    .expect("trigger query runs");
    let triggers: usize = trg
        .lines()
        .last()
        .unwrap_or("0")
        .trim()
        .parse()
        .unwrap_or(0);

    let _ = run(&format!("DROP SCHEMA IF EXISTS `{schema}`"));
    assert!(
        tables > 20,
        "{version}: installed but only {tables} tables exist"
    );
    assert!(
        triggers > 0,
        "{version}: no append-only triggers were created"
    );
    eprintln!(
        "{version}: {} statements, {tables} tables, {triggers} triggers ({})",
        statements.len(),
        if full { "full" } else { "sampled" }
    );
}

#[test]
fn r5_schema_installs_on_a_live_server() {
    install("r5");
}

#[test]
fn r4_schema_installs_on_a_live_server() {
    install("r4");
}

#[test]
fn r3_schema_installs_on_a_live_server() {
    install("r3");
}

#[test]
fn append_only_triggers_actually_refuse_writes() {
    if dsn().is_none() || mysql_cli().is_none() {
        eprintln!("skipping: set FHIR_MYSQL_TEST_DSN to run");
        return;
    }
    let schema = "fhir_mysql_trgtest";
    let _ = run(&format!("DROP SCHEMA IF EXISTS `{schema}`"));
    let mut setup = vec![
        format!("CREATE SCHEMA `{schema}`"),
        format!(
            "CREATE TABLE `{schema}`.`patient_history` (\
             `id` VARCHAR(64) NOT NULL, `version_id` BIGINT NOT NULL, \
             PRIMARY KEY (`id`,`version_id`)) ENGINE=InnoDB"
        ),
    ];
    setup.extend(fhir_mysql_map::ddl::append_only_triggers(
        schema,
        "patient_history",
    ));
    setup.push(format!(
        "INSERT INTO `{schema}`.`patient_history` VALUES ('p1', 1)"
    ));
    run_script(&script_of(&setup)).expect("setup succeeds");

    // UPDATE is refused outright — there is no escape hatch.
    let e = run(&format!(
        "UPDATE `{schema}`.`patient_history` SET `version_id` = 2"
    ))
    .expect_err("UPDATE on history should have been refused");
    assert!(e.contains("append-only"), "unexpected error: {e}");

    // DELETE is refused while the session variable is unset.
    let e = run(&format!("DELETE FROM `{schema}`.`patient_history`"))
        .expect_err("DELETE on history should have been refused");
    assert!(e.contains("append-only"), "unexpected error: {e}");

    // With the variable set, the same DELETE is permitted (M3.18 erasure).
    let ok = run(&format!(
        "SET @fhir_mysql_erasure = 'on';\n\
         DELETE FROM `{schema}`.`patient_history`;\n\
         SET @fhir_mysql_erasure = NULL"
    ));
    let _ = run(&format!("DROP SCHEMA IF EXISTS `{schema}`"));
    ok.expect("erasure-flagged DELETE should be permitted");
}

#[test]
fn exact_collation_does_not_pad_trailing_spaces() {
    // The string assertions in `ddl.rs` only prove the collation is *named*.
    // This proves the property that matters: under the PAD SPACE collations
    // ('utf8mb4_bin'), 'Smith' = 'Smith ' is TRUE, which would silently widen
    // `:exact` matching and weaken primary-key identity. PostgreSQL's
    // COLLATE "C" does not pad, and neither must this.
    if dsn().is_none() || mysql_cli().is_none() {
        eprintln!("skipping: set MYSQL_TEST_DSN to run");
        return;
    }
    let ty = fhir_mysql_map::ddl::col_sql(fhir_mysql_map::model::ColTy::TextC);
    assert!(
        ty.contains("utf8mb4_0900_bin"),
        "unexpected collation: {ty}"
    );

    let out = run(
        "SELECT ('Smith' COLLATE utf8mb4_0900_bin) = ('Smith ' COLLATE utf8mb4_0900_bin) AS padded",
    )
    .expect("collation comparison runs");
    let last = out.lines().last().unwrap_or("").trim().to_string();
    assert_eq!(
        last, "0",
        "utf8mb4_0900_bin treated 'Smith' and 'Smith ' as equal — it pads, so it is the wrong collation"
    );
}
