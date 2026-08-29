//! Does the emitted PL/SQL actually execute on Oracle? (**F-51**.)
//!
//! The unit tests in `ddl.rs` assert the *shape* of individual statements,
//! which catches a stray quote but not a reserved word, an identifier over
//! the length budget, or a trigger the parser rejects. `C0.8` defines four
//! conformance levels and `C0.9` requires the claimed one be justified *by
//! tests that run*; **F-08** proved the full R5 schema installs on Oracle
//! 26ai with 0 invalid objects, but by hand, with `podman exec … sqlplus` —
//! a transcript in an audit entry is not a test, and nothing here noticed
//! when that stopped being true. This is the test `C0.9` asks for, on the
//! model of `fhir-mssql`'s `mssql_ddl.rs`.
//!
//! **Oracle's user-is-schema unification (`M14.5`) is why this file needs an
//! admin connection and `fhir-mssql`'s equivalent does not.** SQL Server lets
//! one login create and drop many schemas inside a database; Oracle has no
//! such thing; installing into a schema means creating the *user* that IS
//! it, which needs `CREATE USER`, a SYSTEM-level privilege no regular test
//! login holds. So this test connects twice: once as SYSTEM to provision a
//! throwaway user, once as that user to install the schema and verify it —
//! matching exactly what this port's own `scripts/db.sh` (`post_ready`) and
//! `.github/workflows/fhir-oracle-ci.yml` (`Create the version users`) already
//! do in shell for the `R3`/`R4`/`R5` users the store's tests use, done here
//! in Rust for a schema this test owns end to end.
//!
//! Needs `FHIR_ORACLE_TEST_CONNECT` (or the `scripts/db.sh` container) and
//! `FHIR_ORACLE_ADMIN_PASSWORD` (defaulted for the local container). Skips
//! loudly without them; fails rather than skips when `FHIR_ORACLE_REQUIRE_DB`
//! is set (`T11.12`, `T11.13`).
//!
//! Verified against `gvenzl/oracle-free:23-slim-faststart`, the same image
//! `fhir-oracle-store`'s live suite uses (**F-68**).

use std::sync::Arc;

use fhir_oracle_map::model::RelMap;
use oracle::Connection;

mod common;

/// A throwaway user, distinct from the `R3`/`R4`/`R5` users the store's own
/// live tests use — this test creates and drops it itself, so it must never
/// collide with schemas another suite depends on staying put.
const TEST_USER: &str = "DDLTEST";

/// A map trimmed to named resource types, for speed — this proves the
/// generator's *output* executes, not that a full ~9,600-statement install is
/// fast, which **F-08** already measured by hand. Named rather than counted:
/// "the first two" is arbitrary and has, in a sibling port, silently excluded
/// exactly the resource whose table shape was the one worth testing.
fn sampled() -> Option<Arc<RelMap>> {
    let mut m = RelMap::bundled("r5").ok()?;
    m.resources
        .retain(|k, _| matches!(k.as_str(), "Patient" | "Observation"));
    assert!(
        !m.resources.is_empty(),
        "Patient/Observation are not in the r5 map"
    );
    // M14.5: the connecting user IS the schema, and Oracle folds an unquoted
    // identifier to uppercase for authentication regardless of how it was
    // created — found live, the hard way, in the store's own tests.
    m.schema = TEST_USER.to_string();
    Some(Arc::new(m))
}

fn admin_connect() -> Option<Connection> {
    let cs = common::connect_string()?;
    Connection::connect("system", common::admin_password(), cs).ok()
}

fn user_connect() -> Option<Connection> {
    let cs = common::connect_string()?;
    Connection::connect(TEST_USER, common::admin_password(), cs).ok()
}

#[test]
fn generated_ddl_installs_on_oracle() {
    if common::connect_string().is_none() {
        assert!(
            !common::require_db(),
            "FHIR_ORACLE_REQUIRE_DB is set, so this test must run: no server reachable"
        );
        eprintln!("skipping: no live Oracle server configured");
        return;
    }
    let Some(admin) = admin_connect() else {
        assert!(
            !common::require_db(),
            "FHIR_ORACLE_REQUIRE_DB is set, so this test must run: SYSTEM connect failed"
        );
        eprintln!("skipping: could not connect as SYSTEM to provision {TEST_USER}");
        return;
    };

    // Start clean, at the user level — Oracle has no schema independent of
    // the user that owns it, so "drop the schema" means "drop the user".
    // SQLCODE -1918 is ORA-01918 ("user does not exist"), which is not a
    // failure here: the first run on a fresh container has nothing to drop.
    admin
        .execute(
            &format!(
                "BEGIN EXECUTE IMMEDIATE 'DROP USER {TEST_USER} CASCADE'; \
                 EXCEPTION WHEN OTHERS THEN IF SQLCODE != -1918 THEN RAISE; END IF; END;"
            ),
            &[],
        )
        .unwrap_or_else(|e| panic!("could not clear a pre-existing {TEST_USER}: {e}"));
    let admin_password = common::admin_password();
    admin
        .execute(
            &format!("CREATE USER {TEST_USER} IDENTIFIED BY \"{admin_password}\""),
            &[],
        )
        .unwrap_or_else(|e| panic!("could not create {TEST_USER}: {e}"));
    admin
        .execute(
            &format!(
                "GRANT CREATE SESSION, CREATE TABLE, CREATE TRIGGER, CREATE SEQUENCE, \
                 UNLIMITED TABLESPACE TO {TEST_USER}"
            ),
            &[],
        )
        .unwrap_or_else(|e| panic!("could not grant privileges to {TEST_USER}: {e}"));
    admin.commit().expect("commit user creation");

    let Some(conn) = user_connect() else {
        panic!("created {TEST_USER} but could not connect as it");
    };

    let map = sampled().expect("Patient/Observation are in the bundled r5 map");
    let statements = fhir_oracle_map::ddl::ddl_in(&map, TEST_USER);
    assert!(
        statements.len() > 10,
        "only {} statements — the asset or the sample looks wrong",
        statements.len()
    );

    let mut applied = 0usize;
    for s in &statements {
        if let Err(e) = conn.execute(s, &[]) {
            panic!(
                "statement {} of {} was rejected by Oracle:\n{e}\n\n{s}",
                applied + 1,
                statements.len()
            );
        }
        applied += 1;
    }
    conn.commit().expect("commit the installed schema");

    // Prove it, rather than trust that no error means the right thing exists —
    // exactly the gap between "the DDL ran" and "the DDL did what F-08 says".
    let tables: i64 = conn
        .query_row_as("SELECT COUNT(*) FROM USER_TABLES", &[])
        .expect("count USER_TABLES");
    assert!(
        tables > 5,
        "schema installed but only {tables} tables exist"
    );

    // Triggers are the enforcement behind M3.17; a schema with tables but no
    // triggers would look healthy while guaranteeing nothing.
    let triggers: i64 = conn
        .query_row_as("SELECT COUNT(*) FROM USER_TRIGGERS", &[])
        .expect("count USER_TRIGGERS");
    assert!(triggers > 0, "no append-only triggers were installed");

    eprintln!("{applied} statements, {tables} tables, {triggers} triggers");
}
