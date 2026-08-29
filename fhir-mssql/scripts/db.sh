#!/usr/bin/env bash
# Local database container for the live test suite.
#
# Most of this project's guarantees are database guarantees, so most of the test
# suite only means anything against a real engine. CI supplies one as a service
# container; this script supplies the same thing on a laptop, with the same
# pinned version, so a green local run and a green CI run mean the same thing.
#
#   scripts/db.sh up        start the container and wait until it answers
#   scripts/db.sh test      up, then run the live suite against it
#   scripts/db.sh dsn       print the export line for a manual run
#   scripts/db.sh client    open an interactive client inside the container
#   scripts/db.sh logs      tail the server log
#   scripts/db.sh status    is it running?
#   scripts/db.sh corpus    lay out the FHIR spec/example corpus the tests need
#   scripts/db.sh down      stop and remove it (data is not persisted)
#
# Podman by default, Docker if that is what is installed. Nothing here writes
# outside the container or the repo's target/ directory.
set -euo pipefail

# SQL Server, not a substitute. This script started `mysql:8.4` until
# 2026-07-31, so a local "live" run exercised the wrong engine entirely and a
# green result meant nothing about the T-SQL this port emits (spec O10.12,
# C0.10; audit F-06).
#
# On arm64 (Apple silicon) `mcr.microsoft.com/mssql/server` does not run; set
# FHIR_MSSQL_IMAGE=mcr.microsoft.com/azure-sql-edge to use the arm64 build,
# which is a *subset* of the product — good evidence, not a conformance claim
# (spec M14.31).
ENGINE="mssql"
IMAGE="${FHIR_MSSQL_IMAGE:-mcr.microsoft.com/mssql/server:2022-latest}"
PORT="11433"
SA_PASSWORD="${FHIR_MSSQL_SA_PASSWORD:-Fhir-Mssql-Local-2026!}"
ENV_VAR="FHIR_MSSQL_TEST_DSN"
NAME="fhir-mssql-db"

cd "$(dirname "$0")/.."
REPO="$PWD"

# ---------------------------------------------------------------- container CLI

if command -v podman >/dev/null 2>&1; then
  CT=podman
elif command -v docker >/dev/null 2>&1; then
  CT=docker
else
  echo "error: neither podman nor docker is installed" >&2
  exit 1
fi

ct() { "$CT" "$@"; }

require_runtime() {
  if ! ct info >/dev/null 2>&1; then
    echo "error: $CT is installed but not responding." >&2
    if [ "$CT" = podman ]; then
      echo "hint: podman machine start" >&2
    else
      echo "hint: start Docker Desktop" >&2
    fi
    exit 1
  fi
}

running() { [ -n "$(ct ps -q --filter "name=^${NAME}$" 2>/dev/null)" ]; }
exists()  { [ -n "$(ct ps -aq --filter "name=^${NAME}$" 2>/dev/null)" ]; }

# One-time setup after the engine answers and before any test connects.
# Overridden per engine below; the default is a no-op.
post_ready() { :; }

# ------------------------------------------------------------------ per engine

case "$ENGINE" in
postgresql)
  RUN_ARGS=(
    # PostgreSQL puts parallel-query and hash-join workspace in /dev/shm, and a
    # container's default 64 MB is not enough for this schema: a full-corpus run
    # dies with `could not resize shared memory segment … No space left on
    # device`, which reads like a disk problem and is not one. CI already passes
    # this; without it here a local run fails where CI passes, which is the one
    # thing this script exists to prevent.
    --shm-size=1g
    -e POSTGRES_PASSWORD=fhir
    -e POSTGRES_USER=fhir
    -e POSTGRES_DB=fhir
    # Installing a full FHIR schema is thousands of tables in one transaction,
    # which exhausts the default lock budget. CI sets the same two knobs; a
    # local run without them fails in a way that looks like a code bug.
    -e POSTGRES_INITDB_ARGS="-c max_locks_per_transaction=512 -c max_parallel_maintenance_workers=0"
    -p "${PORT}:5432"
  )
  # Over TCP, deliberately. During initdb the official image runs a temporary
  # socket-only server; a probe that accepts a Unix socket therefore reports
  # ready while the port the tests connect to is still closed.
  ready() { ct exec "$NAME" pg_isready -h 127.0.0.1 -p 5432 -U fhir -q >/dev/null 2>&1; }
  dsn_line() {
    cat <<EOF
export PGHOST=127.0.0.1 PGPORT=${PORT} PGUSER=fhir PGPASSWORD=fhir
export ${ENV_VAR}=fhir
EOF
  }
  client_cmd() { ct exec -it "$NAME" psql -U fhir -d fhir; }
  ;;
mssql)
  RUN_ARGS=(
    -e ACCEPT_EULA=Y
    -e MSSQL_SA_PASSWORD="$SA_PASSWORD"
    -e MSSQL_PID=Developer
    -p "${PORT}:1433"
  )
  # A real query, not a port check — but not via `sqlcmd`: the arm64
  # substitute image this port uses locally (`azure-sql-edge`) ships **no**
  # client tools at all (confirmed by exec'ing in — neither
  # `/opt/mssql-tools` nor `/opt/mssql-tools18` exists, unlike the full
  # `mcr.microsoft.com/mssql/server` image CI provisions), so the sqlcmd-based
  # check this used to use only ever worked in CI and silently never actually
  # ran locally — `wait_ready` timed out and `up` exited nonzero every time,
  # masked because every local invocation piped through something like `tail`
  # that swallowed the exit code. Found live while chasing an unrelated fix.
  #
  # The replacement queries the server directly over TDS with a tiny
  # `mssql` probe, generated into `target/` the same way the sqlite branch
  # generates its `$WRAPPER` — not a workspace member, so it does not become a
  # second CLI crate (`C0.17`/`C0.18`) — and pinned to the same driver this
  # port's own store depends on (`tiberius` until 2026-08-29, `F-67`).
  MSSQL_PROBE_DIR="$REPO/target/mssql-probe"
  mssql_probe_build() {
    mkdir -p "$MSSQL_PROBE_DIR/src"
    cat > "$MSSQL_PROBE_DIR/Cargo.toml" <<EOF
# Standalone, not a member of the fhir-mssql workspace it lives under —
# without this, cargo refuses to treat target/mssql-probe as its own crate
# ("current package believes it's in a workspace when it's not").
[workspace]
[package]
name = "mssql-probe"
version = "0.0.0"
edition = "2021"
[[bin]]
name = "mssql-probe"
path = "src/main.rs"
[[bin]]
name = "mssql-repl"
path = "src/repl.rs"
[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net"] }
mssql = { version = "0.1", default-features = false, features = ["rustls", "tds73"] }
tokio-util = { version = "0.7", features = ["compat"] }
EOF
    cat > "$MSSQL_PROBE_DIR/src/main.rs" <<'RUST'
// Generated by scripts/db.sh. Runs one batch of SQL over TDS and exits
// nonzero on any connection or statement failure — used both as the
// readiness probe and to run the one-time database setup in post_ready.
use mssql::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ado = std::env::var("MSSQL_PROBE_DSN")?;
    let sql = std::env::var("MSSQL_PROBE_SQL").unwrap_or_else(|_| "SELECT 1".to_string());
    let config = Config::from_ado_string(&ado)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    client.simple_query(sql).await?.into_results().await?;
    Ok(())
}
RUST
    cat > "$MSSQL_PROBE_DIR/src/repl.rs" <<'RUST'
// Generated by scripts/db.sh. A minimal interactive client — `sqlcmd` is not
// available in the arm64 substitute image this port uses locally, so
// `scripts/db.sh client` runs this instead. One statement per line (no `GO`
// batching), fed straight to `simple_query`; result rows print one per line
// as their raw `ColumnData` debug form, which is legible rather than pretty.
use std::io::{self, BufRead, Write};
use mssql::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ado = std::env::var("MSSQL_PROBE_DSN")?;
    let config = Config::from_ado_string(&ado)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let stdin = io::stdin();
    print!("1> ");
    io::stdout().flush()?;
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            print!("1> ");
            io::stdout().flush()?;
            continue;
        }
        match client.simple_query(&line).await {
            Ok(stream) => match stream.into_results().await {
                Ok(results) => {
                    for rows in results {
                        for row in rows {
                            let cells: Vec<String> = row
                                .cells()
                                .map(|(_, data)| format!("{data:?}"))
                                .collect();
                            println!("{}", cells.join(" | "));
                        }
                    }
                }
                Err(e) => eprintln!("error: {e}"),
            },
            Err(e) => eprintln!("error: {e}"),
        }
        print!("1> ");
        io::stdout().flush()?;
    }
    println!();
    Ok(())
}
RUST
  }
  mssql_probe_run() {
    MSSQL_PROBE_DSN="$1" MSSQL_PROBE_SQL="$2" \
      cargo run --quiet --manifest-path "$MSSQL_PROBE_DIR/Cargo.toml" --bin mssql-probe --
  }
  mssql_probe_build
  ready() {
    mssql_probe_run \
      "server=tcp:127.0.0.1,${PORT};user=sa;password=${SA_PASSWORD};TrustServerCertificate=true" \
      "SELECT 1" >/dev/null 2>&1
  }
  # `R4.5` needs one snapshot across `get`'s several statements, which this
  # engine's default READ COMMITTED does not give a transaction on its own —
  # confirmed by a live torn read (audit F-65). `READ_COMMITTED_SNAPSHOT`
  # fixes it with no query-side change (`get`'s existing `BEGIN`/`ROLLBACK
  # TRANSACTION` starts getting one snapshot for free once this is on), but
  # the option cannot be set on `master` at all ("Option
  # 'READ_COMMITTED_SNAPSHOT' cannot be set in database 'master'"), and every
  # DSN here used to omit `database=`, landing in `master` by default. A
  # dedicated database, created once here — before any pooled connection
  # exists, so `ALTER DATABASE` never has to wait out an active transaction —
  # is the fix `M14.25` called for.
  post_ready() {
    local admin_dsn="server=tcp:127.0.0.1,${PORT};user=sa;password=${SA_PASSWORD};TrustServerCertificate=true"
    mssql_probe_run "$admin_dsn" "IF DB_ID('fhir_mssql') IS NULL CREATE DATABASE [fhir_mssql]"
    # `READ_COMMITTED_SNAPSHOT` alone was tried first and found live *not* to
    # fix R4.5: it gives each individual statement inside a READ COMMITTED
    # transaction its own snapshot, not the whole transaction one shared
    # snapshot — `get`'s read still tore. `ALLOW_SNAPSHOT_ISOLATION`, paired
    # with `get` issuing `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` before its
    # `BEGIN TRANSACTION`, is the one that actually gives one instant across
    # every statement in that transaction. Both are enabled: RCSI still helps
    # ordinary reads elsewhere avoid reader/writer blocking, and neither
    # conflicts with the other.
    mssql_probe_run "$admin_dsn" "ALTER DATABASE [fhir_mssql] SET READ_COMMITTED_SNAPSHOT ON"
    mssql_probe_run "$admin_dsn" "ALTER DATABASE [fhir_mssql] SET ALLOW_SNAPSHOT_ISOLATION ON"
  }
  dsn_line() {
    echo "export ${ENV_VAR}='server=tcp:127.0.0.1,${PORT};user=sa;password=${SA_PASSWORD};TrustServerCertificate=true;database=fhir_mssql'"
  }
  # `mssql-repl` (`src/repl.rs`), not `sqlcmd`: one statement per line, no
  # `GO` batching, results printed as raw `ColumnData` — legible, not pretty,
  # but it runs against the local `azure-sql-edge` substitute, which
  # `sqlcmd` cannot.
  client_cmd() {
    MSSQL_PROBE_DSN="server=tcp:127.0.0.1,${PORT};user=sa;password=${SA_PASSWORD};TrustServerCertificate=true;database=fhir_mssql" \
      cargo run --quiet --manifest-path "$MSSQL_PROBE_DIR/Cargo.toml" --bin mssql-repl
  }
  ;;
mysql|mariadb)
  RUN_ARGS=(
    -e MYSQL_ALLOW_EMPTY_PASSWORD=1
    -e MARIADB_ALLOW_EMPTY_ROOT_PASSWORD=1
    -p "${PORT}:3306"
  )
  # Over TCP, deliberately. Both official images run a temporary server with
  # networking disabled while initializing; a socket probe reports ready during
  # that window, and the first test to connect then fails for no visible reason.
  ready() {
    ct exec "$NAME" sh -c \
      'mariadb --protocol=TCP -h 127.0.0.1 -P 3306 -u root -e "select 1" >/dev/null 2>&1 \
       || mysql --protocol=TCP -h 127.0.0.1 -P 3306 -u root -e "select 1" >/dev/null 2>&1'
  }
  dsn_line() { echo "export ${ENV_VAR}=mysql://root@127.0.0.1:${PORT}"; }
  client_cmd() {
    ct exec -it "$NAME" sh -c 'mariadb -u root 2>/dev/null || mysql -u root'
  }
  ;;
sqlite)
  # SQLite has no server. What a container buys here is a *pinned* sqlite3, so
  # the DDL can be checked against a known version rather than whatever the
  # laptop happens to ship — SQLite's DDL features move between releases, and
  # "works on mine" is exactly the failure this project cannot afford.
  WRAPPER="$REPO/target/sqlite3-container"
  RUN_ARGS=(-p "${PORT}:${PORT}")
  ready() { ct exec "$NAME" sqlite3 --version >/dev/null 2>&1; }
  dsn_line() { echo "export ${ENV_VAR}=$WRAPPER"; }
  client_cmd() { ct exec -it "$NAME" sqlite3; }
  ;;
*)
  echo "error: unknown engine $ENGINE" >&2
  exit 1
  ;;
esac

# ------------------------------------------------------------- spec and corpus
#
# Most live tests need the FHIR definitions (to build a relational map) and the
# published examples (to round-trip). CI downloads them from hl7.org. Locally
# they are usually already on disk in a sibling checkout, so this locates them
# rather than making every developer re-download 5,000 files — and exports an
# override so nobody has to.

SPEC_ENV="${ENV_VAR%_TEST_*}_SPEC_DIR"
CORPUS_ENV="${ENV_VAR%_TEST_*}_CORPUS_DIR"
CORPUS_DIR="$REPO/target/test-corpus"

# Locate the FHIR specification packages.
#
# The monorepo path comes first (F-55). The two that follow it are the ancestor
# project's layout and one developer's home directory: neither exists here, so
# before this fix `spec_exports` emitted nothing, the corpus environment
# variables were never set, and following the documented workflow — `db.sh up`
# then `db.sh test` — produced a live suite whose corpus tests could not run.
#
# That is F-39's defect in the shell script. F-39 and F-42 fixed the candidate
# lists inside the Rust tests and did not look here.
find_spec() {
  # An explicit override always wins.
  local from_env="${!SPEC_ENV:-}"
  if [ -n "$from_env" ] && [ -d "$from_env" ]; then
    echo "$from_env"
    return 0
  fi
  for c in \
    "$REPO/../fhir/doc/fhir-specifications" \
    "$REPO/../fhir-rust-crate/doc/fhir-specifications" \
    "$HOME/git/joelparkerhenderson/fhir-rust-crate/doc/fhir-specifications"
  do
    [ -d "$c" ] && { echo "$c"; return 0; }
  done
  return 1
}

# The tests expect `<corpus>/{stu3,r4,r5}/*.json`, while a spec checkout stores
# examples as `<spec>/{r3,r4,r5}/fhir-examples-json`. Symlinks bridge the two
# without copying tens of thousands of files.
corpus() {
  local spec
  if ! spec="$(find_spec)"; then
    echo "error: no FHIR spec directory found." >&2
    echo "hint: set $SPEC_ENV=/path/to/fhir-specifications" >&2
    exit 1
  fi
  rm -rf "$CORPUS_DIR"
  mkdir -p "$CORPUS_DIR"
  local made=0
  for pair in "stu3:r3" "r4:r4" "r5:r5"; do
    local want="${pair%%:*}" have="${pair##*:}"
    local src="$spec/$have/fhir-examples-json"
    if [ -d "$src" ]; then
      ln -s "$src" "$CORPUS_DIR/$want"
      printf '  %-5s -> %s (%s files)\n' "$want" "$src" "$(ls "$src"/*.json 2>/dev/null | wc -l | tr -d " ")"
      made=$((made+1))
    fi
  done
  [ "$made" -gt 0 ] || { echo "error: no example directories under $spec" >&2; exit 1; }
  echo "spec:   $spec"
  echo "corpus: $CORPUS_DIR"
}

# Appended to `dsn` when the inputs can be located, so one eval sets everything.
spec_exports() {
  local spec
  if spec="$(find_spec)"; then
    echo "export $SPEC_ENV=$spec"
    [ -d "$CORPUS_DIR" ] && echo "export $CORPUS_ENV=$CORPUS_DIR"
  fi
  return 0
}

# ---------------------------------------------------------------------- actions

wait_ready() {
  printf 'waiting for %s' "$ENGINE"
  for _ in $(seq 1 120); do
    if ready; then
      echo ' ready'
      return 0
    fi
    printf '.'
    sleep 1
  done
  echo ' timed out' >&2
  echo '--- last 30 log lines ---' >&2
  ct logs --tail 30 "$NAME" >&2 || true
  exit 1
}

up() {
  require_runtime
  if running; then
    echo "$NAME already running"
  else
    exists && ct rm -f "$NAME" >/dev/null
    echo "starting $NAME from $IMAGE"
    if [ "$ENGINE" = sqlite ]; then
      # Idle forever; the container exists only to hold a pinned sqlite3 and a
      # mount of the repo, so tests can exec into it.
      ct run -d --name "$NAME" -v "$REPO:$REPO" -w "$REPO" \
        "${RUN_ARGS[@]}" "$IMAGE" sleep infinity >/dev/null
      ct exec "$NAME" sh -c 'apk add --no-cache sqlite >/dev/null 2>&1' || {
        echo "error: could not install sqlite3 in the container" >&2
        exit 1
      }
      mkdir -p "$(dirname "$WRAPPER")"
      cat >"$WRAPPER" <<EOF
#!/usr/bin/env bash
# Generated by scripts/db.sh — runs the container's pinned sqlite3.
# Paths pass through unchanged because the repo is mounted at the same path
# inside the container, which is why the tests keep their scratch files under
# target/ rather than in TMPDIR.
exec $CT exec -i $NAME sqlite3 "\$@"
EOF
      chmod +x "$WRAPPER"
    else
      ct run -d --name "$NAME" "${RUN_ARGS[@]}" "$IMAGE" >/dev/null
    fi
    wait_ready
  fi
  # Unconditional, not just in the freshly-started branch: idempotent
  # (`IF DB_ID(...) IS NULL`, and re-applying `SET READ_COMMITTED_SNAPSHOT ON`
  # is a no-op), and a container left running from before this existed would
  # otherwise never get it.
  post_ready
  echo
  echo "to use it in this shell:"
  dsn_line
  spec_exports
}

down() {
  require_runtime
  exists && ct rm -f "$NAME" >/dev/null && echo "$NAME removed" || echo "$NAME not present"
  [ "$ENGINE" = sqlite ] && rm -f "$REPO/target/sqlite3-container" || true
}

status() {
  require_runtime
  if running; then
    ct ps --filter "name=^${NAME}$" --format 'table {{.Names}}\t{{.Image}}\t{{.Status}}\t{{.Ports}}'
    ready && echo "accepting connections" || echo "up but not ready yet"
  else
    echo "$NAME is not running"
  fi
}

run_tests() {
  up >/dev/null
  echo "running the live suite against $IMAGE"
  # Rebuild if the directory is missing **or** its links no longer resolve.
  # Checking only for the directory is what let F-55 persist: the links pointed
  # into a checkout that had been removed, so the directory existed, nothing
  # rebuilt it, and the corpus tests failed with "no examples ran".
  corpus_ok() {
    [ -d "$CORPUS_DIR" ] || return 1
    local any=0
    for l in "$CORPUS_DIR"/*; do
      [ -e "$l" ] || return 1
      any=1
    done
    [ "$any" = 1 ]
  }
  corpus_ok || corpus >/dev/null 2>&1 || true
  eval "$(dsn_line)"
  eval "$(spec_exports)"
  # A database was just started, so a test that skips for want of one is a
  # failure, not a pass (spec T11.12, M14.30).
  export FHIR_MSSQL_REQUIRE_DB=1
  cargo test --workspace "$@"
}

case "${1:-up}" in
up)      up ;;
down)    down ;;
status)  status ;;
dsn)     dsn_line; spec_exports ;;
corpus)  corpus ;;
logs)    require_runtime; ct logs -f "$NAME" ;;
client)  require_runtime; running || { echo "not running; run: scripts/db.sh up" >&2; exit 1; }; client_cmd ;;
test)    shift; run_tests "$@" ;;
*)       sed -n '2,20p' "$0" | sed 's/^# \{0,1\}//' ; exit 1 ;;
esac
