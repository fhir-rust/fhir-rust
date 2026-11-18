#!/usr/bin/env bash
# Line coverage with cargo-tarpaulin (`cargo install cargo-tarpaulin`).
# No database server needed for this port. Writes target/coverage/{html,lcov.info}.
# A coverage figure is a measurement: name the date and machine when quoting it (W16.10).
set -euo pipefail
cd "$(dirname "$0")/.."
exec cargo tarpaulin --workspace --engine llvm --timeout 600 \
  --out Html --out Lcov --output-dir target/coverage "$@"
