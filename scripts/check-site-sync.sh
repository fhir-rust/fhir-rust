#!/usr/bin/env bash
# Is fhir-rust.github.io's content/ and static/llms.* still what the monorepo
# would produce right now?
#
# Why this exists. fhir-rust.github.io/content/ and static/llms.{txt,json}
# are vendored, committed snapshots, not build-time output — the site's own
# deploy.yml says so explicitly ("No sync step: content/ and static/ are
# committed"), and it has to be that way: the deployed repo is a git-subtree
# push of this directory with no sibling checkout of the monorepo root, so
# `npm run sync` has nothing to read from there. That leaves this monorepo,
# which does have both trees in one checkout, as the only place a drift
# between root docs and the site's copy of them can be caught before it ships.
#
# It already drifted once (found auditing the site 2026-09-03): the site's
# copy of spec/databases/audit.md still called F-98 open after F-102 closed
# it, was missing F-99 entirely, and fhir-loco/README.md's mirror still said
# conditional delete was unimplemented the same day HTTP support for it
# shipped. None of that was caught because nothing ran the sync scripts and
# diffed the result — this script is that check.
#
# This does not run in fhir-rust.github.io's own CI (it cannot: the source
# tree doesn't exist there). It runs here, against the one checkout that has
# both sides, and it mutates the working tree if it finds drift — run it
# locally before committing a change to either side, not as a read-only gate.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
site_dir="$repo_root/fhir-rust.github.io"

if [[ ! -d "$site_dir" ]]; then
  echo "check-site-sync: no fhir-rust.github.io/ here, nothing to check."
  exit 0
fi

cd "$site_dir"

if [[ ! -d node_modules ]]; then
  echo "check-site-sync: node_modules missing — run 'npm ci' in $site_dir first." >&2
  exit 1
fi

# Only content/ and llms.* come from this monorepo — deliberately not
# `npm run sync`'s third leg, sync:lily-themes, which vendors theme CSS from
# a *different* project's sibling checkout ($LILY, default
# ~/git/lilydesignsystem/lily-design-system) that a monorepo checkout, CI's
# included, has no reason to have. That drift is real but out of scope here.
npm run --silent sync:content
npm run --silent sync:llms

drift="$(git -C "$repo_root" status --porcelain -- fhir-rust.github.io/content fhir-rust.github.io/static/llms.txt fhir-rust.github.io/static/llms.json)"

if [[ -n "$drift" ]]; then
  echo "check-site-sync: FAIL — fhir-rust.github.io/content or static/llms.* was" >&2
  echo "  stale relative to the monorepo root. 'npm run sync' has updated it in" >&2
  echo "  the working tree below; review and commit the result." >&2
  echo "$drift" >&2
  exit 1
fi

echo "check-site-sync: OK — fhir-rust.github.io's content and llms.* match the monorepo root."
