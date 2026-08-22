# Testing

The governing section is [`spec/11-conformance-testing.md`](../spec/databases/11-conformance-testing.md).
This file is how to work with it.

## `cargo test` passing means little

Unit tests run with no database and no FHIR specification packages, because the
corpus- and spec-driven tests skip themselves when their inputs are absent. That
is convenient locally and **misleading in CI**.

Most of what these libraries guarantee is a database guarantee — snapshot
isolation, row locks, the append-only trigger, index-using search plans, the
hash chain — and none of it is exercised without a server. The live suite is the
gate that matters:

```sh
cd fhir-postgresql
scripts/db.sh up        # start the pinned engine in a container
scripts/db.sh test      # up, then run the live suite
scripts/db.sh corpus    # lay out the FHIR spec/example corpora
scripts/db.sh down      # stop and remove (data is not persisted)
```

**You do not have to export anything.** As of 2026-08-22 every live test in
`fhir-postgresql`, `fhir-mysql`, `fhir-mariadb`, `fhir-mssql` and `fhir-oracle`
resolves its server itself, in `tests/common/mod.rs`: the port's `*_TEST_DSN`
(or `*_TEST_DB` / `*_TEST_CONNECT`) when set, otherwise the `scripts/db.sh`
container if it is listening on its documented port. So `./scripts/db.sh up`
followed by a bare `cargo test` runs the live suite for real. `fhir-sqlite`
needs none of this — it has no server.

A skip is printed where it can be seen, and **`FHIR_<PORT>_REQUIRE_DB=1` makes
a skip a failure**. Every live CI job in all five ports now sets it, on GitHub
Actions and Woodpecker alike, so a job that reaches no server is red rather than
a green run that checked nothing (`T11.12`, `T11.13`). Before that, the flag
reached `fhir-mssql`'s `mssql_ddl` and the two `ssl_live` files and nothing
else, and every other suite in every port would report `test result: ok` having
connected to nothing — the 0.00s was the only tell.

`fhir-oracle`'s Woodpecker `database.yaml` is still the removed-gate note from
2026-07-31, and its text is stale: that port has had a store, a driver and a
live GitHub Actions job since 2026-08-12. Its GitHub live job does set
`FHIR_ORACLE_REQUIRE_DB`.

## The four rules that are easy to break

**A test must be able to fail** (`T11.10`). Verify it by mutation: revert the
fix, or break the code it guards, and watch the test go red. A test not verified
this way is presumed decorative. This matters most for the tamper-evidence
tests, where a control that cannot fail is indistinguishable from one that
works — and telling them apart is the control's entire value.

**Pin the narrowest assertion** (`T11.11`). "At least 20" tolerates losing four
of twenty-four; "more than zero" tolerates losing all but one. Prefer an exact
value or a named set. Where the expected set is large, commit a snapshot so a
regression names what changed, and keep regeneration an explicit reviewed step
so a shrinking baseline cannot be adopted by accident.

**A skip must be loud** (`T11.12`). A check that cannot run — corpus absent,
database unreachable, path unresolved — must say so and must fail if it ends up
checking nothing. A skip is indistinguishable from a pass in a CI summary. The
original corpus test resolved its inputs through an absolute path into one
machine's temp directory: it skipped silently in CI for its entire life.
Resolve inputs relative to the repository or an environment variable.

**When two artifacts must agree, test the agreement itself** (`U11a`). Where
one component describes what another emits — the relational map naming columns
that `ddl.rs` writes, an asset naming what the generator produces — each side's
own tests pass while the two contradict each other, because each is internally
consistent. The contradiction shows up only at runtime, on a path unit tests do
not reach. Assert the correspondence directly, over the whole set rather than a
sample: `tests/adjuncts_in_ddl.rs` walks every table of every resource. And
guard against the loop finding nothing to check — assert the count is non-zero
exactly when it should be, or a dialect that legitimately produces none turns
the test vacuous without anyone noticing (`T11.12` again, one level up).

**A test must be deterministic** (`T11.15`). Same tree, same engine, same
verdict. An intermittently-passing test is worse than a failing one: it teaches
you to re-run, and a re-run looks exactly like a fix. If a live test clears
shared state first, make the teardown fail loudly and assert the state is
actually clean — a discarded teardown error does not disappear, it comes back
later disguised as a failure in correct code. `fhir-mssql`'s DDL test failed two
runs in three and blamed a `CREATE TABLE` that was fine (**F-52**).

**An ignored test is a tracked gap** (`T11.14`). `fhir-oracle` had eleven
`#[ignore]`d MySQL-asserting tests — correctly ignored, correctly recorded in
its `tasks.md`, and now **replaced** with Oracle-asserting ones (**F-08**). That
is the point of tracking: the entry is what made them findable when the port
caught up. Ignoring without recording turns a known gap into a forgotten one.

## The test taxonomy

| Kind | Where | Needs a database |
| --- | --- | --- |
| Unit | `#[cfg(test)]` in `map/src/*.rs` | no |
| DDL shape | `map/tests/<engine>_ddl.rs` unit assertions | no |
| DDL execution | `map/tests/<engine>_ddl.rs` live | **yes** |
| Round-trip corpus | `gen/tests/corpus.rs`, `roundtrip.rs` | no |
| Round-trip property | `gen/tests/proptest_roundtrip.rs` | no |
| Store semantics | `store/tests/*.rs` | **yes** |
| Fuzz | `fuzz/fuzz_targets/*.rs` | no |

The DDL split is worth understanding. Unit assertions catch a stray backquote;
they do not catch a reserved word, an unindexable column, or a trigger the
parser rejects. Running the generated schema through the real engine is the only
thing that found the SQLite and MySQL ports' real bugs.

## Coverage by port

Only `fhir-postgresql` has the full suite: `concurrency.rs`, `audit.rs`,
`redaction.rs`, `upgrade.rs`, `live.rs`, `m2_semantics.rs`,
`search_semantics.rs`, `bench.rs`. The other stores have one test file each, and
two ports have none.

The consequence is in the [conformance matrix](../spec/databases/conformance-matrix.md):
`R4.5`, `H5.4`, `T11.6`, `T11.7`, and `T11.8` are `?` outside PostgreSQL. Those
are the concurrency and audit guarantees — the ones §13 maps to HIPAA
§164.312(b) and (c). Porting `concurrency.rs`, `redaction.rs`, and `audit.rs` to
the other stores is the highest-value test work available.

## Writing a store test

```rust
// Skips without a DSN — and says so, per T11.12.
let Some(dsn) = std::env::var("FHIR_SQLITE_TEST_DB").ok() else {
    eprintln!("skipping: FHIR_SQLITE_TEST_DB not set");
    return;
};
```

That pattern is correct locally and wrong as a CI gate. In CI the DSN must be
set and its absence must fail. `fhir-mssql` shows the shape: a `require_db()`
helper reads `FHIR_MSSQL_REQUIRE_DB`, and a `skip_or_fail!` macro panics instead
of returning when it is set. Verify it both ways before trusting it (`T11.10`) —
without the variable the test skips and passes; with it and no DSN it must fail.

## Fuzzing

Targets live in `fuzz/`, outside the workspace, because `libfuzzer-sys` needs
nightly and a normal `cargo build` must not.

```sh
cd fhir-postgresql/fuzz
cargo +nightly fuzz run shred -- -max_total_time=60
cargo +nightly fuzz run search_sql -- -max_total_time=60
```

`T11.9` requires these be **run, not merely committed**, with a bounded budget
and a committed seed corpus, and that a crash, panic, abort, or **stack
overflow** fail the build. A stack overflow is not unwindable — `catch_unwind`
does not catch it, a worker thread cannot contain it, and the process ends. The
sibling `fhir` crate's XML reader aborted on ~160 KB of nested input and nothing
detected it for the life of the module.

Nothing currently shows these running in CI; that is `?` for `T11.9` in every
port.

## Benchmarks

`doc/benchmarks.md` per port, with a regression gate against the recorded
baseline (`T11.5`). A number in that file must name what measured it and when
(`W16.10`) — a throughput figure inherited by text substitution is not a
measurement of the port carrying it.
