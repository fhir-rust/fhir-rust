//! Do the append-only trigger and the boolean CHECK actually enforce
//! anything live, or only look correct in the generated SQL text? (**F-97**.)
//!
//! `oracle_ddl.rs` proves the schema *installs*; it does not prove either
//! constraint *works*, and both have already failed exactly this way once.
//! `M14.29a` is explicit about why reading is not enough: the trigger's first
//! version "was written, installed, and observed letting an ordinary DELETE
//! through with no error" — `NVL(x, '') != 'y'` is NULL, not TRUE, when `x`
//! is the empty string Oracle treats as NULL, and the `ELSIF` never fired. It
//! looked correct. `M14.8`'s unit test (`ddl.rs`) only asserts the CHECK
//! clause's *text* is present, which the same failure mode could survive —
//! a CHECK that parses but never fires would pass that test too.
//!
//! Both constraints are generic — `append_only_triggers` takes only a schema
//! and a table name, and the boolean CHECK only needs one `Bool` column — so
//! neither test needs the full generated Patient schema. A minimal synthetic
//! table exercises the exact PL/SQL and DDL the generator emits, in
//! isolation from shredding concerns this finding is not about.
//!
//! Needs `FHIR_ORACLE_TEST_CONNECT` (or the `scripts/db.sh` container) and
//! `FHIR_ORACLE_ADMIN_PASSWORD`, the same as `oracle_ddl.rs`. Skips loudly
//! without them; fails rather than skips when `FHIR_ORACLE_REQUIRE_DB` is
//! set (`T11.12`, `T11.13`). Each `#[test]` provisions its own throwaway
//! user — `TRIGTEST`, `BOOLTEST` — distinct from each other and from
//! `oracle_ddl.rs`'s `DDLTEST`, because libtest runs the functions in one
//! binary concurrently by default and a shared user was tried first: 3 of 3
//! runs failed, reproducibly, until each test got its own.

use oracle::Connection;

mod common;

/// Each test gets its own dedicated user, not a shared `CONSTRAINTTEST` —
/// found necessary, not assumed: libtest runs the two `#[test]` functions in
/// this binary concurrently by default, and one shared user provisioned by
/// both at once is exactly the "flaky live gate is worse than a failing one"
/// trap `mssql_ddl.rs`'s own history warns about. Reproduced 3 of 3 runs
/// before this split; 0 of many after.
fn admin_connect() -> Option<Connection> {
    let cs = common::connect_string()?;
    Connection::connect("system", common::admin_password(), cs).ok()
}

fn user_connect(user: &str) -> Option<Connection> {
    let cs = common::connect_string()?;
    Connection::connect(user, common::admin_password(), cs).ok()
}

/// Provision a fresh throwaway user, exactly as `oracle_ddl.rs` does — see
/// that file for why an admin connection is needed at all (`M14.5`).
fn provision(admin: &Connection, user: &str) {
    let admin_password = common::admin_password();
    admin
        .execute(
            &format!(
                "BEGIN EXECUTE IMMEDIATE 'DROP USER {user} CASCADE'; \
                 EXCEPTION WHEN OTHERS THEN IF SQLCODE != -1918 THEN RAISE; END IF; END;"
            ),
            &[],
        )
        .unwrap_or_else(|e| panic!("could not clear a pre-existing {user}: {e}"));
    admin
        .execute(
            &format!("CREATE USER {user} IDENTIFIED BY \"{admin_password}\""),
            &[],
        )
        .unwrap_or_else(|e| panic!("could not create {user}: {e}"));
    admin
        .execute(
            &format!(
                "GRANT CREATE SESSION, CREATE TABLE, CREATE TRIGGER, \
                 UNLIMITED TABLESPACE TO {user}"
            ),
            &[],
        )
        .unwrap_or_else(|e| panic!("could not grant privileges to {user}: {e}"));
    admin.commit().expect("commit user creation");
}

/// `SQLCODE`/`ORA-` prefix from an `oracle::Error`'s `Display`, so a test can
/// assert *which* error fired rather than merely that execution failed — the
/// same distinction `M14.29a`'s own bug hid: a silently-succeeding forbidden
/// DELETE and a correctly-rejected one both "work" if you only check
/// `is_err()` on the wrong statement.
fn ora_code(e: &oracle::Error) -> String {
    let s = e.to_string();
    s.find("ORA-").map(|i| s[i..i + 9].to_string()).unwrap_or(s)
}

#[test]
fn append_only_trigger_rejects_update_and_undeclared_delete_but_allows_declared_erasure() {
    let Some(admin) = admin_connect() else {
        assert!(
            !common::require_db(),
            "FHIR_ORACLE_REQUIRE_DB is set, so this test must run: SYSTEM connect failed"
        );
        eprintln!("skipping: no live Oracle server configured");
        return;
    };
    let user = "TRIGTEST";
    provision(&admin, user);
    let conn =
        user_connect(user).unwrap_or_else(|| panic!("created {user} but could not connect as it"));

    let table = "trig_test";
    conn.execute(
        &format!("CREATE TABLE \"{table}\" (id NUMBER(10) PRIMARY KEY, val VARCHAR2(10))"),
        &[],
    )
    .expect("create the scratch table");
    for stmt in fhir_oracle_map::ddl::append_only_triggers(user, table) {
        conn.execute(&stmt, &[])
            .expect("install the append-only trigger");
    }
    conn.execute(&format!("INSERT INTO \"{table}\" VALUES (1, 'a')"), &[])
        .expect("seed one row");
    conn.commit().expect("commit setup");

    // M3.17: an ordinary UPDATE must be refused outright, unconditionally.
    let err = conn
        .execute(
            &format!("UPDATE \"{table}\" SET val = 'b' WHERE id = 1"),
            &[],
        )
        .expect_err("UPDATE must be rejected by the trigger");
    assert_eq!(
        ora_code(&err),
        "ORA-20001",
        "wrong error for a forbidden UPDATE: {err}"
    );

    // M3.17 + M3.18: a DELETE with no erasure declaration must also be
    // refused. This is the exact assertion M14.29a's bug would fail:
    // the old trigger let this DELETE through silently.
    let err = conn
        .execute(&format!("DELETE FROM \"{table}\" WHERE id = 1"), &[])
        .expect_err("undeclared DELETE must be rejected by the trigger");
    assert_eq!(
        ora_code(&err),
        "ORA-20002",
        "wrong error for an undeclared DELETE: {err}"
    );

    let count: i64 = conn
        .query_row_as(&format!("SELECT COUNT(*) FROM \"{table}\""), &[])
        .expect("count rows after the rejected DELETE");
    assert_eq!(
        count, 1,
        "the rejected DELETE must not have removed the row"
    );

    // M3.18: the declared-erasure escape hatch must still work, in the same
    // transaction the annex requires (M14.29).
    conn.execute(
        "BEGIN DBMS_APPLICATION_INFO.SET_CLIENT_INFO('fhir_oracle_erasure=on'); END;",
        &[],
    )
    .expect("set the erasure declaration");
    conn.execute(&format!("DELETE FROM \"{table}\" WHERE id = 1"), &[])
        .expect("declared erasure DELETE must be allowed");
    conn.execute(
        "BEGIN DBMS_APPLICATION_INFO.SET_CLIENT_INFO(NULL); END;",
        &[],
    )
    .expect("clear the erasure declaration");
    conn.commit().expect("commit the declared erasure");

    let count: i64 = conn
        .query_row_as(&format!("SELECT COUNT(*) FROM \"{table}\""), &[])
        .expect("count rows after the declared-erasure DELETE");
    assert_eq!(
        count, 0,
        "the declared-erasure DELETE should have removed the row"
    );

    eprintln!(
        "append-only trigger: UPDATE and undeclared DELETE both refused; declared erasure allowed"
    );
}

#[test]
fn boolean_check_rejects_out_of_range_values() {
    let Some(admin) = admin_connect() else {
        assert!(
            !common::require_db(),
            "FHIR_ORACLE_REQUIRE_DB is set, so this test must run: SYSTEM connect failed"
        );
        eprintln!("skipping: no live Oracle server configured");
        return;
    };
    let user = "BOOLTEST";
    provision(&admin, user);
    let conn =
        user_connect(user).unwrap_or_else(|| panic!("created {user} but could not connect as it"));

    let table = "bool_test";
    let ty = fhir_oracle_map::ddl::col_sql(fhir_oracle_map::model::ColTy::Bool);
    conn.execute(
        &format!(
            "CREATE TABLE \"{table}\" (\
             id NUMBER(10) PRIMARY KEY, \
             active {ty}, \
             CONSTRAINT bool_test_active_ck CHECK (active IN (0, 1)))"
        ),
        &[],
    )
    .expect("create the scratch table with the CHECK create_table would emit");

    // M14.8: 0 and 1 are the only legal encodings of a FHIR boolean here.
    conn.execute(&format!("INSERT INTO \"{table}\" VALUES (1, 0)"), &[])
        .expect("0 must be accepted");
    conn.execute(&format!("INSERT INTO \"{table}\" VALUES (2, 1)"), &[])
        .expect("1 must be accepted");

    // The value M14.8's whole point is to keep out: an unconstrained
    // NUMBER(1) accepts anything from -9 to 9, which is not a boolean.
    let err = conn
        .execute(&format!("INSERT INTO \"{table}\" VALUES (3, 2)"), &[])
        .expect_err("2 must be rejected by the CHECK constraint");
    assert_eq!(
        ora_code(&err),
        "ORA-02290",
        "wrong error for a CHECK violation: {err}"
    );
    conn.commit().expect("commit the accepted rows");

    let count: i64 = conn
        .query_row_as(&format!("SELECT COUNT(*) FROM \"{table}\""), &[])
        .expect("count rows");
    assert_eq!(
        count, 2,
        "only the two legal rows should have been inserted"
    );

    eprintln!("boolean CHECK: 0 and 1 accepted, 2 rejected with ORA-02290");
}
