//! Finding the live PostgreSQL server, in one place.
//!
//! Every live test used to open with the same four lines: read
//! `FHIR_POSTGRESQL_TEST_DB`, print `skipping` to stderr if it was absent,
//! `return`, and then `unsafe { set_var("PGDATABASE", &db) }`. Fourteen copies
//! of that, and two consequences worth naming.
//!
//! **A plain `cargo test` reported success without touching a database.** The
//! `eprintln!` goes to a stream libtest captures and discards on a passing
//! test, so the run printed `test result: ok. 6 passed` in 0.00s and looked
//! exactly like a green suite. That is the shape of **F-91** — suites that were
//! never really running — and the reason a developer has to remember an
//! incantation before the tests mean anything is the reason they eventually
//! do not.
//!
//! **The `SAFETY` comments were not true.** They said "single-threaded at this
//! point"; libtest runs test functions on many threads at once, so fourteen
//! scattered `set_var` calls could race a concurrent `pg_config` reading the
//! same variables. Resolving once per process, before any connection, is not a
//! proof of soundness — `set_var` is still `unsafe` in a process that has
//! threads — but it replaces fourteen racing writers with one, and in the
//! common paths (CI, and `eval "$(scripts/db.sh dsn)"`) it writes nothing at
//! all, because everything is already set.
//!
//! What this does **not** do is invent a server. If nothing is listening, the
//! tests still skip — but they say so where it can be seen, and they say what
//! to run. A run that must not skip sets `FHIR_POSTGRESQL_REQUIRE_DB`, which
//! turns the skip into a failure; the live CI job sets it, so "the service
//! container never came up" is red rather than green.

use std::sync::OnceLock;

/// The container `scripts/db.sh up` starts. Kept in step with that script by
/// hand; if it moves, a local run stops finding it and says so rather than
/// pretending to pass.
const LOCAL: [(&str, &str); 5] = [
    ("PGHOST", "127.0.0.1"),
    ("PGPORT", "15432"),
    ("PGUSER", "fhir"),
    ("PGPASSWORD", "fhir"),
    // A loopback dev container serves plaintext, and since F-17 the default
    // is `require`, which verifies. The opt-out is explicit here for the same
    // reason it is explicit in db.sh: nobody reading a green local suite
    // should believe it exercised a verified connection.
    ("PGSSLMODE", "disable"),
];

/// The database name for the live suite, or `None` when there is no server.
///
/// Resolution order:
///
/// 1. `FHIR_POSTGRESQL_TEST_DB` — CI, and anyone who ran
///    `eval "$(scripts/db.sh dsn)"`. Nothing is guessed and nothing is written
///    except `PGDATABASE`, which `pg_config` needs and the variable names.
/// 2. Otherwise, if `scripts/db.sh`'s container is listening on its documented
///    port, adopt it. This is the case that makes a bare `cargo test` work.
/// 3. Otherwise `None`, with a visible reason.
pub fn test_db() -> Option<&'static str> {
    static RESOLVED: OnceLock<Option<String>> = OnceLock::new();
    RESOLVED.get_or_init(resolve).as_deref()
}

fn resolve() -> Option<String> {
    if let Ok(db) = std::env::var("FHIR_POSTGRESQL_TEST_DB") {
        set_if_unset("PGDATABASE", &db);
        return Some(db);
    }
    let (host, port) = (LOCAL[0].1, LOCAL[1].1);
    if !listening(host, port) {
        // A job that exists to exercise a live server must not report success
        // when it never reached one. The `database` job sets this, so a
        // mis-provisioned service container is a red build rather than a green
        // one that ran nothing — F-06 (a pipeline that started the wrong
        // engine) and F-91 (suites that were never really running) are both
        // this failure, found late.
        assert!(
            std::env::var_os("FHIR_POSTGRESQL_REQUIRE_DB").is_none(),
            "FHIR_POSTGRESQL_REQUIRE_DB is set, but no server is reachable at \
             {host}:{port} and FHIR_POSTGRESQL_TEST_DB is unset. Refusing to \
             skip: this run was supposed to be live."
        );
        // Printed through the panic-free path libtest still shows on a
        // *passing* run only if `--nocapture` is on, so say it loudly enough
        // that the one-line summary is not the whole story a reader gets.
        eprintln!(
            "\n\
             ==> fhir-postgresql live tests SKIPPED: nothing is listening on \
             {host}:{port}\n\
             ==>   start one:  cd fhir-postgresql && ./scripts/db.sh up\n\
             ==>   or point at your own:  export FHIR_POSTGRESQL_TEST_DB=<dbname>\n"
        );
        return None;
    }
    for (k, v) in LOCAL {
        set_if_unset(k, v);
    }
    let db = "fhir";
    set_if_unset("PGDATABASE", db);
    eprintln!("==> fhir-postgresql live tests using the db.sh container at {host}:{port}");
    Some(db.to_string())
}

fn listening(host: &str, port: &str) -> bool {
    use std::net::{TcpStream, ToSocketAddrs};
    let Ok(mut addrs) = format!("{host}:{port}").to_socket_addrs() else {
        return false;
    };
    addrs.any(|a| TcpStream::connect_timeout(&a, std::time::Duration::from_millis(500)).is_ok())
}

fn set_if_unset(key: &str, value: &str) {
    if std::env::var_os(key).is_some() {
        return;
    }
    // SAFETY: reached only from `test_db`'s `OnceLock` initializer, so exactly
    // one thread in this process ever runs it, and only for variables that were
    // unset. It is not a soundness proof — a concurrent reader in another
    // thread is still possible — but it is one writer instead of fourteen, and
    // it does not run at all when the environment is already configured.
    unsafe { std::env::set_var(key, value) };
}
