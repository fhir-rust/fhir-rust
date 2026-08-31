# Dependabot

Enable GitHub Dependabot dependabot_security_updates at the repo level. 

Enable GitHub Dependabot .github/dependabot.yml for scheduled update PRs.

## Status in this repository

Both items are done. `.github/dependabot.yml` exists and registers one
`cargo` entry per Cargo workspace (`/fhir`, `/fhir-store`, `/fhir-loco`, and
each of the six database ports), plus a `github-actions` entry at the root —
because this is a nine-workspace monorepo (see `AGENTS.md`) and Dependabot
scopes `cargo` to a single directory's own lockfile, one entry per workspace
is required for full coverage. Alerts and automated security fixes
(`dependabot_security_updates`) are enabled as a repository setting.
Scheduled version-update PRs run weekly (Mondays) at GitHub's default cap of
5 open PRs per directory — raised from an initial `open-pull-requests-limit:
0` after the first hour with default limits opened 47 PRs, many major
version bumps each triggering a port's full live-database CI, which does not
suit a repository that upgrades deliberately under an MSRV floor
(`rust-msrv-n-minus-2`). The `dtolnay/rust-toolchain` action pin is excluded
from automated bumps because that ref is the Rust toolchain version itself,
not an action revision — it moves by hand alongside a recomputed MSRV.
