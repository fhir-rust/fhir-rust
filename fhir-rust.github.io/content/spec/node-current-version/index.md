# Node current version

Node.js® is a free, open-source, cross-platform JavaScript runtime environment that lets developers create servers, web apps, command line tools and scripts.

Node current version is 26.

- If file `deploy.yml` exists and contains `node-version`, then set `node-version: 26`, then verify green (pnpm ci + pnpm run build success).
- If file `package.json` exists, then set: engines.node: "=26", then verify green locally.
- If file `.npmrc` exists, then set `engine-strict=true` so a gap can't occur, then verify failure (EBADENGINE) under node version 25, then verify success under node version 26.
- If file `.nvmrc` exists, then pin the exact local dev version, scoped to this project only.
- If file `.tool-versions` exists, then pin the exact local dev version, scoped to this project only.

## Status in this repository

Not applicable in `fhir-rust` itself: this is a nine-workspace Rust monorepo
(`AGENTS.md`) with no `deploy.yml`, `package.json`, `.npmrc`, `.nvmrc`, or
`.tool-versions` anywhere in the tree (`find . -iname deploy.yml -o -iname
package.json -o -iname .npmrc -o -iname .nvmrc -o -iname .tool-versions`
returns nothing) — none of this document's conditions have a file to act on.

The checklist is applied where those files actually exist: the sibling site
repository, `fhir-rust.github.io`. There the package manager is npm, not
pnpm — the checklist's `pnpm ci`/`pnpm run build` step was run as `npm ci`/
`npm run build` instead, matching that repository's own `package-lock.json`
and `deploy.yml`. Applied 2026-08-31: `deploy.yml`'s `node-version: 26`
verified green in hosted CI; `package.json`'s `engines.node: "=26"` (not the
open-ended `>=26` an earlier pass had used) verified to reject Node 25.9.0
(`EBADENGINE`) and accept 26.8.1; `.npmrc`'s `engine-strict=true` is what
makes that rejection a hard failure rather than a warning; `.nvmrc` and
`.tool-versions` both pin `26.8.1`, scoped to that repository only — neither
touches this machine's global `nvm`/`mise` default.
