# Verifying a claim before you write it down

This repository is about clinical software; its main historical failure mode
(**F-01**, **F-27**, and others in `spec/databases/audit.md`) is confident
text — a README, a `tasks.md` checkbox, a dialect annex — that nothing in the
code actually backs up. Run these before writing "works", "supports", or
checking a box. Substitute `<engine>` for the port (`postgresql`, `sqlite`,
`mysql`, `mariadb`, `mssql`, `oracle`) and `<op>` for the operation
(`put`, `get`, `delete`, `history`, `vread`, `search`, `verify_audit`,
`purge`, `log_access`, `upgrade`, `backfill_norm`, …).

## Does the operation actually exist?

```sh
grep -rn "pub async fn <op>" fhir-<engine>/crates/*-store/src/
```

No hit means the operation doesn't exist for that port — not "probably", not
"should by analogy with the other ports." Check.

## Is there a test, and does it run without a database?

```sh
ls fhir-<engine>/crates/*-store/tests/
grep -rn "TEST_DSN\|return Ok(())" fhir-<engine>/crates/*/tests/
```

A test file existing is not the same as it exercising anything: a live-only
test that silently returns `Ok(())` when no DSN is set looks green in CI
without a database ever being touched. `FHIR_<PORT>_REQUIRE_DB=1` (set by
every live CI job) is what turns a skip into a hard failure — check whether
the job you're relying on actually sets it.

## Does CI provision the right engine?

```sh
grep -n "image:" .github/workflows/fhir-<engine>-ci.yml
```

Since the **F-49** consolidation, family CI files live at the repo root
(`fhir-<port>-ci.yml` per port, plus `fhir-ci.yml`, `fhir-loco-ci.yml`,
`fhir-store-ci.yml`), path-filtered to their family. There is no per-port
`fhir-<engine>/.github/workflows/` directory any more — that path is always
empty post-F-49, not merely sometimes empty, so the root-level file above is
the only place to look. A job that provisions the wrong database (this
repo's history: MySQL provisioned for `mssql`/`oracle` jobs invoking a test
target that didn't exist, **F-06**) can pass for reasons that have nothing to
do with the port working.

## Does the shared core actually match across ports?

```sh
./scripts/check-shared-core.sh          # summary
./scripts/check-shared-core.sh --diff   # the offending diffs
```

## What conformance level is actually earned?

Don't infer this from reading the code and deciding it looks plausible.
Read [`spec/databases/conformance-matrix.md`](../../spec/databases/conformance-matrix.md)
— it's the document a conformance claim must be justified against (`C0.9`),
refreshed by someone actually reading each port's store surface, `ddl.rs`,
test directory, and CI config. If a README and the matrix disagree, the
matrix is right (that mismatch was **F-01**) — treat a stale README as the
defect to fix, not as a second source of truth.

## If the honest answer is "it's shared from a port where it works"

That is **`?`** in the conformance matrix, not `•`. Shared code existing in
a port that has never been live-tested against that port's own database is
not the same claim as "verified here."

## The general rule

If you can't point at the `grep` hit, the test file, or the CI config that
backs a sentence, don't write the sentence yet — go find it, or write what
you actually know (including "not verified," which is itself useful and
required — `T11.12`, rule 5 in `AGENTS.md`).
