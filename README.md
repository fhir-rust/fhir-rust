# fhir-rust.github.io

The website for **fhir-rust**: <https://fhir-rust.github.io>

A SvelteKit site, prerendered to static files by `@sveltejs/adapter-static` and
served by GitHub Pages. It renders the monorepo's own Markdown — it does not
restate it. The repository is the source of truth; this site is a view of it.

## What it publishes

The source is the fhir-rust **monorepo** (four families in one repository):

| Route | Source |
| --- | --- |
| `/` | the hub — hand-written, in `src/routes/+page.svelte` |
| `/overview/` | `README.md` |
| `/docs/` | `index.md` — every entry point in the repository |
| `/docs/guides/` | `doc/index.md` |
| `/docs/<name>/` | `doc/<name>.md` — the guides and six tutorials |
| `/specs/` | `spec/index.md` — the four-family specification root |
| `/specs/publishing/` | `spec/publishing.md` |
| `/specs/<slug>/` | the single-topic cross-family specs `spec/index.md` links to — agent skills, agents-directory casing, Dependabot, funding, git tags, HL7 trademark fair use, llms.txt/llms.json, professionalization, MSRV, the serde_json float-roundtrip decision, the special-files list, Trusted Publishing |
| `/spec/` | `spec/databases/index.md` — the database core |
| `/spec/<name>/` | `spec/databases/<name>.md` — the normative sections, the audit register, the fold |
| `/conformance/` | `spec/databases/conformance-matrix.md` |
| `/model/` | `fhir/README.md` — the model family |
| `/model/spec/…` | `fhir/spec/*.md` — 14 sections |
| `/server/` | `fhir-loco/README.md` — the HTTP surface |
| `/server/spec/…` | `fhir-loco/spec/*.md` — 4 sections |
| `/store/` | `fhir-store/README.md` — the shared persistence core |
| `/examples/` | generated from `fhir/examples/*.rs` — see below |

The route map is `routeFor()` in [`src/lib/paths.js`](src/lib/paths.js), and it
is the one place to change if a document moves. `/spec/…` keeps its pre-merge
URLs on purpose: they are linked from outside this site.

**Examples are the one transformation.** Each `fhir/examples/*.rs` opens with a
`//!` doc-comment tutorial (the crate's house style), so the sync turns it into
a page: the comment as prose, the program as a listing, plus a generated index.
Everything else is copied verbatim.

## Building it

```sh
npm install
npm run dev        # http://localhost:5173
npm run build      # -> build/
npm run preview    # serve build/ as GitHub Pages will
```

`npm run build` is all CI does. The vendored directories below are committed, so
the site builds from a bare checkout with no sibling repositories present.

## Vendored, not fetched

These are generated. **Do not edit them by hand** — change the source and
re-run the script.

| Directory / file | Script | From |
| --- | --- | --- |
| `content/` | `npm run sync:content` | the monorepo: `README.md`, `index.md`, `doc/`, `spec/`, `fhir/`, `fhir-loco/`, `fhir-store/` |
| `src/lib/lily/` | `npm run sync:lily` | the Lily Design System checkout |
| `static/themes/` | `npm run sync:lily` | Lily's themes |
| `static/llms.txt`, `static/llms.json` | `npm run sync:llms` | the monorepo's `llms.txt`/`llms.json` ([spec](https://github.com/fhir-rust/fhir-rust/blob/main/spec/llms-json-and-llms-txt/index.md)), with every link rewritten to wherever it resolves from *this* domain via `routeFor()`/`sourceUrl()` (see [`src/lib/paths.js`](src/lib/paths.js)) — the workspace copies use repository-relative links, which don't resolve on a live site |

`npm run sync` runs all three. Each script takes its source from an
environment variable when the default location is wrong:

```sh
WORKSPACE=/path/to/fhir-rust npm run sync:content   # default: ../fhir-rust
WORKSPACE=/path/to/fhir-rust npm run sync:llms       # default: ../fhir-rust
LILY=/path/to/lily-design-system npm run sync:lily
```

`content/` mirrors the monorepo's layout rather than flattening it, so every
document's own relative links resolve exactly as they do in the repository.

Lily's Svelte components are MIT licensed and vendored rather than installed:
the helpers are not published to npm. The commit they came from is recorded in
`src/lib/lily/VENDOR.md`.

## How links are handled

Markdown links are rewritten at build time by `rewriteHref()` in
[`src/lib/paths.js`](src/lib/paths.js):

- a link to a published document becomes a site route;
- a link to anything else — `AGENTS.md`, a port's source tree, the openEHR
  crates — becomes a GitHub URL, so it works instead of 404ing;
- external links are left alone.

Prerendering runs with `handleHttpError: 'fail'`, so a broken *internal* link
fails the build rather than shipping.

Requirement ids are linked to the section that defines them, **per family**:
the database core's `` `C0.5` ``/`` `X15.6` `` backtick spelling, the model
spec's bold `**R13.14**` spelling, the server's `SV2.14`, and audit findings
(`F-65`) in either spelling. The lookup is keyed by the file the id appears in
because `R4.x` deliberately exists in two families — the monorepo's own
spec/index.md documents that collision — and the `db:`/`model:` qualified
spellings resolve anywhere. The tables are in
[`src/lib/markdown.js`](src/lib/markdown.js); database sections 7 and 8 are
retired and 14 is per-port, which is why neither appears.

## Repository URLs

`src/lib/site.js` holds every outbound repository URL. The project merged into
one monorepo — `fhir-rust/fhir-rust` — which is the source tree's own `origin`;
anonymous HTTP to it 404s today, which a private repository also does, so it is
unverified rather than known-absent. Every rewritten source link resolves
through `REPOSITORY`, so it all starts working the moment the repository is
public — nothing else hard-codes it.

## Deploying

[`.github/workflows/deploy.yml`](.github/workflows/deploy.yml) builds on every
push to `main` and deploys with `actions/deploy-pages`. It needs **Settings →
Pages → Source: GitHub Actions** set once on the repository.

`paths.base` is empty because this is an organization-pages repository, served
from the root. A project-pages repository would need `paths: { base: ... }` and
a `BASE_PATH` in the workflow — see the comment in `svelte.config.js`.

## Accessibility

The site is built from Lily's headless components, which carry the semantics and
no styling: `SkipLink`, `Header`, `Footer`, `ArticleLayout`, `BreadcrumbNav`,
`ContentsNav`, `PaginationNav`, and the theme and text-size pickers. Themes and
text size persist in `localStorage`, and the theme picker follows the system
preference until a reader chooses otherwise.
