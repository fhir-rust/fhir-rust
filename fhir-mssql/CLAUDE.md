# CLAUDE.md — fhir-mssql

Start with [`AGENTS.md`](AGENTS.md) in this directory, then
[`../CLAUDE.md`](../CLAUDE.md) for the monorepo-wide traps.

**Engine:** Microsoft SQL Server · **Level:** Store

Four things to know before editing anything here:

1. The pure-Rust core in `map/src` and `gen/src` is byte-identical across all
   six ports. Change it in all six or not at all (`X15.1`, `W16.7`).
2. There **is** a store now (`crates/fhir-mssql-store/src/mssql.rs`,
   `mssql_search.rs`), live-verified against `azure-sql-edge` by 33 tests, 0
   `#[ignore]`d (`F-65`). `get` needs `SET TRANSACTION ISOLATION LEVEL
   SNAPSHOT` for `R4.5` — do not simplify it back to a bare `BEGIN
   TRANSACTION`, or to `READ_COMMITTED_SNAPSHOT` alone, both of which were
   tried and, live, still tore. See `M14.25` in
   [`spec/14-mssql-dialect.md`](spec/14-mssql-dialect.md). The DSN must name
   a database (`;database=fhir_mssql`, `scripts/db.sh up` prints it) — `master`
   refuses `ALLOW_SNAPSHOT_ISOLATION`, so a DSN without one silently loses the
   fix.
3. No `conditional_create_audited`, `put_audited`, `transact_audited`. `upgrade`
   and `backfill_norm` **do** exist now (closes this port's share of `F-15`),
   live-verified by `tests/upgrade.rs` (9 tests, 0 `#[ignore]`d). Unlike
   `fhir-mysql`/`fhir-mariadb`, `upgrade` is one transaction — T-SQL DDL is
   transactional, so a failed upgrade rolls back rather than leaving a
   half-applied schema (`M14.35`). Table drops in the destructive diff MUST be
   ordered children before their base table, or `DROP TABLE` fails with error
   3726 against a live server (`M14.36`, found running the tests). `deny.toml`'s
   `tiberius`-chain advisory ignores now reach a shipping crate, not a
   dev-dependency — see `M14.34` before "fixing" the comments back to the old
   (false) scope, and see `F-67` before assuming `native-tls` is a free swap
   (it fails the handshake on this host).
4. Normative behaviour is [`../spec/`](../spec/databases/index.md), not this directory.
   Check [`../spec/audit.md`](../spec/databases/audit.md) before reporting a defect — it
   may already be tracked.
