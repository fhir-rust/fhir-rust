//! Finding the live Oracle server, in one place.
//!
//! Copied from `fhir-oracle-store`'s test helper of the same name rather than
//! depended on — a map crate cannot depend on the store crate above it — and
//! extended with the one thing this crate's own live test needs that the
//! store's tests do not: an admin connection, because installing DDL means
//! creating the Oracle *user* it installs into (`M14.5`: user is schema here),
//! and a regular test login has no privilege to do that.
//!
//! Two things used to be true of every live test in this port, and both are
//! `T11.12` failures — "coverage MUST NOT degrade silently".
//!
//! **A bare `cargo test` reported success without touching a database.** Each
//! test read `FHIR_ORACLE_TEST_CONNECT` itself, printed `skipping` to stderr
//! and returned; libtest captures and discards stderr on a *passing* test, so
//! a run that connected to nothing printed `test result: ok` and looked
//! exactly like a green suite. Only the 0.00s gave it away.
//!
//! **`FHIR_ORACLE_REQUIRE_DB` reached one test file, not the suite.** The
//! mechanism existed — `ssl_live.rs` and `fhir-mssql`'s `mssql_ddl.rs` have
//! had it since **F-06** — but a live job that reached no server still went
//! green through every other file, which is `T11.13` exactly.
//!
//! So resolution happens once per binary, here:
//!
//! 1. `FHIR_ORACLE_TEST_CONNECT` when set — CI, and anyone who ran
//!    `eval "$(scripts/db.sh dsn)"`. Nothing is guessed.
//! 2. otherwise the `scripts/db.sh` container, if it is listening on its
//!    documented port. This is what makes a bare `cargo test` real.
//! 3. otherwise skip, saying so where it can be seen and saying what to run —
//!    unless `FHIR_ORACLE_REQUIRE_DB` is set, in which case a skip is a
//!    failure, because a run that was promised a database and reached none
//!    must be red.

#![allow(dead_code)]

use std::sync::OnceLock;

/// Where `scripts/db.sh up` publishes this engine. Kept in step with that
/// script by hand; if it moves, a local run stops finding it and says so
/// rather than pretending to pass.
const HOST: &str = "127.0.0.1";
const PORT: &str = "11521";

/// The connect string for the live suite, or `None` when there is no server.
pub fn connect_string() -> Option<&'static str> {
    static RESOLVED: OnceLock<Option<String>> = OnceLock::new();
    RESOLVED.get_or_init(resolve).as_deref()
}

/// True when this run was promised a database, so a skip is a failure
/// (`T11.12`, `T11.13`).
pub fn require_db() -> bool {
    std::env::var("FHIR_ORACLE_REQUIRE_DB").is_ok_and(|v| v != "0" && !v.is_empty())
}

/// The SYSTEM password, for creating the throwaway user this test installs
/// into. Never a real deployment's password — `db.sh`'s own local default and
/// this repository's CI value are both throwaway constants, committed
/// deliberately for the same reason `fhir-postgresql`'s live audit-suite key
/// is: they protect nothing, and a workflow that pulled a real one from
/// secrets would put it in reach of every fork's pull-request runner.
pub fn admin_password() -> String {
    std::env::var("FHIR_ORACLE_ADMIN_PASSWORD").unwrap_or_else(|_| "Fhir-Oracle-Local-2026".into())
}

fn resolve() -> Option<String> {
    if let Ok(d) = std::env::var("FHIR_ORACLE_TEST_CONNECT")
        && !d.trim().is_empty()
    {
        return Some(d);
    }
    if !listening() {
        assert!(
            !require_db(),
            "FHIR_ORACLE_REQUIRE_DB is set, but no server is reachable at \
             {HOST}:{PORT} and FHIR_ORACLE_TEST_CONNECT is unset. Refusing to skip: \
             this run was supposed to be live."
        );
        eprintln!(
            "\n\
             ==> fhir-oracle-map live tests SKIPPED: nothing is listening on {HOST}:{PORT}\n\
             ==>   start one:  cd fhir-oracle && ./scripts/db.sh up\n\
             ==>   or point at your own:  export FHIR_ORACLE_TEST_CONNECT=<connect-string>\n"
        );
        return None;
    }
    eprintln!("==> fhir-oracle-map live tests using the db.sh container at {HOST}:{PORT}");
    Some(format!("{HOST}:{PORT}/FHIR"))
}

fn listening() -> bool {
    use std::net::{TcpStream, ToSocketAddrs};
    let Ok(mut addrs) = format!("{HOST}:{PORT}").to_socket_addrs() else {
        return false;
    };
    addrs.any(|a| TcpStream::connect_timeout(&a, std::time::Duration::from_millis(500)).is_ok())
}
