# Repository-level tasks that don't belong to any one family's own tooling
# (cargo covers the Rust crates; this covers cross-cutting operations on the
# monorepo as a whole).

.PHONY: github-pages test-minimum-supported-rust-version bench coverage

# Publish fhir-rust.github.io/ (a monorepo subtree, spec/monorepo-github-pages/)
# to the standalone, read-only GitHub Pages export repo. See bin/make-github-pages
# for what this actually runs and why.
github-pages:
	bin/make-github-pages

# Check and test every workspace on the MSRV (N-2) and N-1 toolchains.
test-minimum-supported-rust-version:
	bin/make-test-minimum-supported-rust-version

# Criterion micro-benchmarks for the audit chain (fhir-store; pure CPU, no database).
bench:
	cd fhir-store && cargo bench --bench chain

# Line coverage for fhir-store with cargo-tarpaulin (cargo install cargo-tarpaulin).
# Config: fhir-store/tarpaulin.toml; HTML report in fhir-store/target/tarpaulin/.
coverage:
	cd fhir-store && cargo tarpaulin
