# Repository-level tasks that don't belong to any one family's own tooling
# (cargo covers the Rust crates; this covers cross-cutting operations on the
# monorepo as a whole).

.PHONY: github-pages

# Publish fhir-rust.github.io/ (a monorepo subtree, spec/monorepo-github-pages/)
# to the standalone, read-only GitHub Pages export repo. See bin/make-github-pages
# for what this actually runs and why.
github-pages:
	bin/make-github-pages
