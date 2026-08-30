# llms.json and llms.txt

Create AI guidance helper files at the repo root:

- `llms.json` -> JSON
- `llms.txt` -> markdown text

Purpose: Provide AI tools with a clean, curated map of its most important content.

Help large language models (LLMs) read, understand, and cite a site's documentation or resources without getting bogged down 

File size:  < 40k bytes.

## Publishing a copy on a site (e.g. `*.github.io`)

The workspace-root `llms.txt`/`llms.json` use links relative to the
repository root (`README.md`, `fhir/README.md`, …), which only resolve
inside a git checkout. Serving that exact text at `*.github.io/llms.txt`
would ship a page of links that mostly 404 on that domain — a docs site with
its own routing does not serve files at their raw repository paths.

**Done 2026-08-30.** A site that publishes a copy under its own `static/`
(or equivalent) directory must use a website-appropriate version instead:
the same curated map, but with every entry rewritten to wherever it actually
resolves from that site's own domain — its rendered route where the site
publishes that document, and the source repository's URL where it does not
— not a byte-for-byte copy of the workspace-root file. `fhir-rust.github.io`
implements this: `static/llms.txt` and `static/llms.json` are generated from
the monorepo's copies via its `routeFor()`/`sourceUrl()` mapping
([`src/lib/paths.js`](https://github.com/fhir-rust/fhir-rust.github.io/blob/main/src/lib/paths.js)),
not copied verbatim.
