//! Does the emitted T-SQL actually execute on SQL Server?
//!
//! The unit tests in `ddl.rs` assert the *shape* of individual statements, which
//! catches a stray backquote but not a reserved word, an unindexable column, or
//! a trigger the parser rejects. This runs the generated schema through the real
//! engine — which is the only way the SQLite and MySQL ports found their real
//! bugs, and this DDL was written without a server to try it on.
//!
//! Needs `FHIR_MSSQL_TEST_DSN`, e.g.
//! `server=tcp:127.0.0.1,11433;user=sa;password=…;TrustServerCertificate=true`.
//! Skips silently without one.
//!
//! Verified against `azure-sql-edge`, which is the SQL Server build that runs on
//! arm64. It is a subset of the full product, so a pass here is good evidence
//! and not a conformance claim.

use std::sync::Arc;

use fhir_mssql_map::model::RelMap;
use mssql_driver::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

mod common;

fn dsn() -> Option<String> {
    common::dsn().map(str::to_string)
}

/// Is a database mandatory in this run?
///
/// Skipping without a DSN is right on a laptop and wrong in CI, where a skip is
/// indistinguishable from a pass in the summary (spec T11.12, T11.13, M14.30).
/// This test is the port's *only* live evidence, so a run that quietly checks
/// nothing is exactly the failure that let `--test mysql_ddl` — a target that
/// does not exist in this package — sit in the pipeline unnoticed (audit F-06).
///
/// CI sets `FHIR_MSSQL_REQUIRE_DB=1`; then every skip below becomes a panic.
fn require_db() -> bool {
    std::env::var("FHIR_MSSQL_REQUIRE_DB").is_ok_and(|v| v != "0" && !v.is_empty())
}

/// Skip, or fail loudly when `require_db()` says a database was promised.
macro_rules! skip_or_fail {
    ($($arg:tt)*) => {{
        let reason = format!($($arg)*);
        assert!(
            !require_db(),
            "FHIR_MSSQL_REQUIRE_DB is set, so this test must run: {reason}"
        );
        eprintln!("skipping: {reason}");
        return;
    }};
}

fn relmap(version: &str) -> Option<Arc<RelMap>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(format!("fhir-mssql-relmap-{version}.json.gz"));
    let bytes = std::fs::read(path).ok()?;
    RelMap::from_gz_bytes(&bytes).ok().map(Arc::new)
}

/// A map trimmed to named resource types.
///
/// Named rather than counted: "the first six" is alphabetical and silently
/// excludes `Observation`, which then fails in a way that looks like a store bug
/// and is not. That mistake cost time in the MySQL port.
fn sampled(version: &str, schema: &str, want: &[&str]) -> Option<Arc<RelMap>> {
    let mut m = (*relmap(version)?).clone();
    m.resources.retain(|k, _| want.contains(&k.as_str()));
    assert!(!m.resources.is_empty(), "none of {want:?} are in {version}");
    m.schema = schema.to_string();
    Some(Arc::new(m))
}

async fn connect() -> Option<Client<tokio_util::compat::Compat<TcpStream>>> {
    let cfg = Config::from_ado_string(&dsn()?).expect("DSN parses");
    let tcp = TcpStream::connect(cfg.get_addr()).await.ok()?;
    tcp.set_nodelay(true).ok()?;
    Client::connect(cfg, tcp.compat_write()).await.ok()
}

#[tokio::test]
async fn generated_ddl_installs_on_sql_server() {
    if dsn().is_none() {
        skip_or_fail!("set FHIR_MSSQL_TEST_DSN to run");
    }
    let Some(mut client) = connect().await else {
        skip_or_fail!("cannot reach the server named by FHIR_MSSQL_TEST_DSN");
    };
    let schema = "fhir_mssql_ddltest";
    let Some(map) = sampled("r5", schema, &["Observation", "Patient"]) else {
        skip_or_fail!("no r5 relmap asset");
    };

    // Start clean. Dropping a schema means dropping what is in it first, which
    // T-SQL will not do for you.
    //
    // **Foreign keys must go first.** A base table cannot be dropped while a
    // child table's FK still references it, so a flat `DROP TABLE` batch aborts
    // at the first such table and silently leaves the rest. `sys.tables` has no
    // guaranteed order, so whether that happened varied run to run: this test
    // failed roughly two runs in three with "There is already an object named
    // 'observation'" — reported against statement 8 of 131, eight statements
    // away from the cleanup that actually failed.
    //
    // A flaky live gate is worse than a failing one, because the habit it
    // teaches is to re-run it.
    let cleanup_sql = format!(
        "DECLARE @sql NVARCHAR(MAX) = N'';
             SELECT @sql = @sql + N'ALTER TABLE [{schema}].['
                 + OBJECT_NAME(parent_object_id) + N'] DROP CONSTRAINT ['
                 + name + N'];'
               FROM sys.foreign_keys WHERE schema_id = SCHEMA_ID('{schema}');
             EXEC sp_executesql @sql;
             SET @sql = N'';
             SELECT @sql = @sql + N'DROP TABLE [{schema}].[' + name + N'];'
               FROM sys.tables WHERE schema_id = SCHEMA_ID('{schema}');
             EXEC sp_executesql @sql;
             IF SCHEMA_ID('{schema}') IS NOT NULL EXEC('DROP SCHEMA [{schema}]');"
    );
    if let Err(e) = client.simple_query(&cleanup_sql).await {
        panic!("could not clear schema [{schema}] before installing: {e}");
    }

    // And prove it worked, rather than discovering it downstream. This is the
    // assertion whose absence turned a cleanup bug into a misattributed DDL
    // failure.
    let left: i32 = client
        .query(
            "SELECT COUNT(*) FROM sys.tables WHERE schema_id = SCHEMA_ID(@P1)",
            &[&schema],
        )
        .await
        .expect("count leftover tables")
        .into_row()
        .await
        .expect("a row")
        .and_then(|r| r.get(0))
        .unwrap_or(0);
    assert_eq!(left, 0, "cleanup left {left} table(s) in [{schema}]");

    let statements = fhir_mssql_map::ddl::ddl(&map);
    assert!(
        statements.len() > 20,
        "only {} statements — the asset looks wrong",
        statements.len()
    );

    let mut applied = 0usize;
    for s in &statements {
        if let Err(e) = client.simple_query(s.as_str()).await {
            panic!(
                "statement {} of {} was rejected by SQL Server:\n{e}\n\n{s}",
                applied + 1,
                statements.len()
            );
        }
        applied += 1;
    }

    let rows = client
        .simple_query(format!(
            "SELECT count(*) FROM sys.tables WHERE schema_id = SCHEMA_ID('{schema}')"
        ))
        .await
        .expect("count query")
        .into_first_result()
        .await
        .expect("rows");
    let tables: i32 = rows[0].get(0).unwrap_or(0);
    assert!(
        tables > 10,
        "schema installed but only {tables} tables exist"
    );

    // Triggers are the enforcement behind M3.17; a schema with tables but no
    // triggers would look healthy while guaranteeing nothing.
    let rows = client
        .simple_query(format!(
            "SELECT count(*) FROM sys.triggers t
               JOIN sys.tables tb ON t.parent_id = tb.object_id
              WHERE tb.schema_id = SCHEMA_ID('{schema}')"
        ))
        .await
        .expect("trigger query")
        .into_first_result()
        .await
        .expect("rows");
    let triggers: i32 = rows[0].get(0).unwrap_or(0);
    assert!(triggers > 0, "no append-only triggers were installed");

    eprintln!("{applied} statements, {tables} tables, {triggers} triggers");
}
