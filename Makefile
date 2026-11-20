# Repository-level tasks that don't belong to any one family's own tooling
# (cargo covers the Rust crates; this covers cross-cutting operations on the
# monorepo as a whole).

.PHONY: github-pages ci-minimum-supported-rust-version test-benchmarks test-coverage

# Publish fhir-rust.github.io/ (a monorepo subtree, spec/monorepo-github-pages/)
# to the standalone, read-only GitHub Pages export repo. See bin/make-github-pages
# for what this actually runs and why.
github-pages:
	bin/make-github-pages

# Check and test every workspace on the MSRV (N-2) and N-1 toolchains.
ci-minimum-supported-rust-version:
	bin/make-ci-minimum-supported-rust-version

# Criterion micro-benchmarks for the audit chain (fhir-store; pure CPU, no database).
test-benchmarks:
	bin/test-benchmarks

# Line coverage for fhir-store with cargo-tarpaulin (cargo install cargo-tarpaulin).
# Config: fhir-store/tarpaulin.toml; HTML report in fhir-store/target/tarpaulin/.
test-coverage:
	bin/test-coverage
