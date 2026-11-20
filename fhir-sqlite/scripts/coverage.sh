#!/usr/bin/env bash
# Line coverage with cargo-llvm-cov (`cargo install cargo-llvm-cov`).
# No database server needed for this port. The store's operational suite is the
# measured surface and must remain at or above 95% line coverage. Writes
# target/coverage/{html,lcov.info}.
# A coverage figure is a measurement: name the date and machine when quoting it (W16.10).
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/coverage
cargo llvm-cov --offline --locked -p fhir-sqlite-store --all-features \
  --remap-path-prefix --ignore-filename-regex '(^|/)(tests|\.cargo)/' \
  --fail-under-lines 95 --lcov --output-path target/coverage/lcov.info "$@" \
  -- --test-threads=1
exec cargo llvm-cov --remap-path-prefix \
  --ignore-filename-regex '(^|/)(tests|\.cargo)/' report \
  --html --output-dir target/coverage/html
