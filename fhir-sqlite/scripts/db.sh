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

ENGINE="sqlite"
IMAGE="docker.io/library/alpine:3.21"
PORT="19999"
ENV_VAR="FHIR_SQLITE_SQLITE3"
NAME="fhir-sqlite-db"

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
