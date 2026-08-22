//! Finding the live Oracle server, in one place.
//!
//! Two things used to be true of every live test in this port, and both are
//! `T11.12` failures — "coverage MUST NOT degrade silently".
//!
//! **A bare `cargo test` reported success without touching a database.** Each
//! test read `FHIR_ORACLE_TEST_CONNECT` itself, printed `skipping` to stderr and returned;
//! libtest captures and discards stderr on a *passing* test, so a run that
//! connected to nothing printed `test result: ok` and looked exactly like a
//! green suite. Only the 0.00s gave it away.
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

/// The DSN for the live suite, or `None` when there is no server.
pub fn dsn() -> Option<&'static str> {
    static RESOLVED: OnceLock<Option<String>> = OnceLock::new();
    RESOLVED.get_or_init(resolve).as_deref()
}

/// True when this run was promised a database, so a skip is a failure
/// (`T11.12`, `T11.13`).
pub fn require_db() -> bool {
    std::env::var("FHIR_ORACLE_REQUIRE_DB").is_ok_and(|v| v != "0" && !v.is_empty())
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
             ==> fhir-oracle live tests SKIPPED: nothing is listening on {HOST}:{PORT}\n\
             ==>   start one:  cd fhir-oracle && ./scripts/db.sh up\n\
             ==>   or point at your own:  export FHIR_ORACLE_TEST_CONNECT=<dsn>\n"
        );
        return None;
    }
    set_if_unset("FHIR_ORACLE_TEST_USER", "r5");
    set_if_unset("FHIR_ORACLE_TEST_PASSWORD", "Fhir-Oracle-Local-2026");
    eprintln!("==> fhir-oracle live tests using the db.sh container at {HOST}:{PORT}");
    Some(format!("{HOST}:{PORT}/FHIR"))
}

fn listening() -> bool {
    use std::net::{TcpStream, ToSocketAddrs};
    let Ok(mut addrs) = format!("{HOST}:{PORT}").to_socket_addrs() else {
        return false;
    };
    addrs.any(|a| TcpStream::connect_timeout(&a, std::time::Duration::from_millis(500)).is_ok())
}

fn set_if_unset(key: &str, value: &str) {
    if std::env::var_os(key).is_some() {
        return;
    }
    // SAFETY: reached only from `dsn`'s `OnceLock` initializer, so exactly one
    // thread in this process ever runs it, and only for variables that were
    // unset. Not a soundness proof — `set_var` is unsafe in a threaded process
    // — but one writer instead of one per test, and none at all when the
    // environment is already configured.
    unsafe { std::env::set_var(key, value) };
}
