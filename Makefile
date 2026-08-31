# Repository-level tasks that don't belong to any one family's own tooling
# (cargo covers the Rust crates; this covers cross-cutting operations on the
# monorepo as a whole).

.PHONY: github-pages

# Publish fhir-rust.github.io/ (a monorepo subtree, spec/monorepo-github-pages/)
# to the standalone, read-only GitHub Pages export repo. This pushes a
# subdirectory of the current repo out to a different branch (here `main`)
# on the remote named `github-pages`, using git's subtree mechanism -- the
# only way to publish that subtree; there is no other route to the live
# site from a monorepo commit.
#
# The remote is created on first use if a fresh clone doesn't have it yet.
github-pages:
	git remote get-url github-pages >/dev/null 2>&1 || \
		git remote add github-pages git@github.com:fhir-rust/fhir-rust.github.io.git
	git subtree push --prefix=fhir-rust.github.io github-pages main
