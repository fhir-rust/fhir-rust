# Local containers

Most of this project's guarantees are database guarantees, so most of the test
suite only means anything against a real engine. CI supplies one as a service
container. `scripts/db.sh` supplies the same thing on a laptop, from the same
pinned image, so a green local run and a green CI run are the same claim.

Podman is used by default; Docker is used instead if that is what is installed.
Nothing is written outside the container and the repo's `target/` directory, and
no data is persisted — `down` then `up` gives a clean database every time.

## The engine

| | |
|---|---|
| Image | `docker.io/library/mysql:8.4` |
| Host port | `13306` |
| Container | `fhir-mysql-db` |

MySQL 8.4 specifically. This port uses MySQL-only syntax — notably the
`utf8mb4_0900_bin` collation, which does not exist in MariaDB — so verifying
against a MariaDB instance is not a substitute (spec M14.0a, M14.0b). The live
tests shell out to a `mysql` or `mariadb` client on the host; install one, or
use `scripts/db.sh client` for manual work.

## Usage

```sh
scripts/db.sh up        # start it, wait until it actually answers
scripts/db.sh test      # up, lay out the corpus, run the live suite
scripts/db.sh dsn       # print the exports for a manual run
scripts/db.sh corpus    # lay out the FHIR definitions and examples
scripts/db.sh client    # interactive client inside the container
scripts/db.sh status    # running? accepting connections?
scripts/db.sh logs      # follow the server log
scripts/db.sh down      # stop and remove
```

The usual loop is one command:

```sh
scripts/db.sh test
```

To run a subset by hand, take the environment from the script and then use
`cargo test` directly:

```sh
eval "$(scripts/db.sh dsn)"
cargo test -p fhir-mysql-store --test live -- --nocapture --test-threads=1
```

## Spec and example inputs

Live tests need two things besides a server: the FHIR **definitions**, to build
a relational map, and the published **examples**, to round-trip. CI downloads
both from hl7.org. Locally they are usually already in a sibling checkout, so
`scripts/db.sh corpus` finds them and links them into the layout the tests
expect — `target/test-corpus/{stu3,r4,r5}` — rather than copying tens of
thousands of files.

Discovery order, first hit wins:

1. `FHIR_MYSQL_SPEC_DIR`, if set.
2. `../fhir-rust-crate/doc/fhir-specifications` next to this repo.
3. `~/git/joelparkerhenderson/fhir-rust-crate/doc/fhir-specifications`.

Set `FHIR_MYSQL_SPEC_DIR` to point somewhere else. The tests themselves fall back to
the same sibling-checkout path relative to their own crate, so they work with no
environment set at all — but they **skip silently** when the inputs are missing,
which is the behaviour to remember when a suite reports success suspiciously
fast. A live test that finishes in 0.00s did not run.

## Environment the tests read

| Variable | Meaning |
|---|---|
| `FHIR_MYSQL_TEST_DSN` | Set by `dsn`. Without it the live tests skip. |
| `FHIR_MYSQL_SPEC_DIR` | FHIR definitions directory. |
| `FHIR_MYSQL_CORPUS_DIR` | Example corpus directory. |
| `FHIR_MYSQL_TEST_CORPUS_LIMIT` | Examples per version (default 400; use 10–25 for a fast loop). |
| `FHIR_MYSQL_CHAIN_KEY` | Hex key enabling the keyed-MAC audit tests. |
| `FHIR_MYSQL_BENCH` | Set to a row count to enable the benchmark test. |

## Why the readiness probe uses TCP

The official PostgreSQL and MySQL images run a *temporary* server while they
initialize, with networking disabled. A readiness probe that accepts a Unix
socket therefore reports success while the port the tests connect to is still
closed, and the first test to connect fails for no visible reason. `db.sh`
probes over TCP inside the container specifically to avoid that false ready —
it was an actual flake here before it was a comment.

## Troubleshooting

**`podman is installed but not responding`** — the VM is not running:
`podman machine start`.

**Port already in use** — something else is on `13306`, often a
Homebrew-installed server. Stop it, or edit `PORT` at the top of
`scripts/db.sh`.

**A live test passes in 0.00s** — it skipped. Check `scripts/db.sh status` and
that `scripts/db.sh corpus` has been run.
