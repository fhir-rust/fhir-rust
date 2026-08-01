# fhir-rust.github.io

The website for **fhir-rust**: <https://fhir-rust.github.io>

A SvelteKit site, prerendered to static files by `@sveltejs/adapter-static` and
served by GitHub Pages. It renders the project's own Markdown — it does not
restate it. The repository is the source of truth; this site is a view of it.

## What it publishes

| Route | Source |
| --- | --- |
| `/` | the hub — hand-written, in `src/routes/+page.svelte` |
| `/overview/` | `README.md` |
| `/docs/` | `index.md` — every entry point in the repository |
| `/docs/guides/` | `doc/index.md` |
| `/docs/<name>/` | `doc/<name>.md` — 11 guides and tutorials |
| `/spec/` | `spec/index.md` |
| `/spec/<name>/` | `spec/<name>.md` — the normative sections |
| `/conformance/` | `spec/conformance-matrix.md` |

The route map is `routeFor()` in [`src/lib/paths.js`](src/lib/paths.js), and it
is the one place to change if a document moves.

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

Three directories are generated. **Do not edit them by hand** — change the
source and re-run the script.

| Directory | Script | From |
| --- | --- | --- |
| `content/` | `npm run sync:content` | the workspace: `README.md`, `index.md`, `doc/`, `fhir-databases/spec/` |
| `src/lib/lily/` | `npm run sync:lily` | the Lily Design System checkout |
| `static/themes/` | `npm run sync:lily` | Lily's themes |

`npm run sync` runs both. Each script takes its source from an environment
variable when the default location is wrong:

```sh
WORKSPACE=/path/to/fhir-rust npm run sync:content
LILY=/path/to/lily-design-system npm run sync:lily
```

`content/` flattens the workspace into `doc/` and `spec/`, which is the
arrangement the documents' own relative links already assume — `doc/index.md`
links to `../spec/index.md`, and it resolves.

Lily's Svelte components are MIT licensed and vendored rather than installed:
the helpers are not published to npm. The commit they came from is recorded in
`src/lib/lily/VENDOR.md`.

## How links are handled

Markdown links are rewritten at build time by `rewriteHref()` in
[`src/lib/paths.js`](src/lib/paths.js):

- a link to a published document becomes a site route;
- a link to anything else — `AGENTS.md`, `fhir-postgresql/`, the openEHR crates —
  becomes a GitHub URL, so it works instead of 404ing;
- external links are left alone.

Prerendering runs with `handleHttpError: 'fail'`, so a broken *internal* link
fails the build rather than shipping.

Requirement ids written in backticks — `` `C0.5` ``, `` `X15.6` ``, `` `F-01` ``
— are linked to the section that defines them. The prefix table is
`REQUIREMENT_SECTIONS` in [`src/lib/markdown.js`](src/lib/markdown.js); sections
7 and 8 are retired and 14 is per-port, which is why neither appears.

## Repository URLs

`src/lib/site.js` holds every outbound repository URL. **Only `REPO_CRATE` is
verified** — `fhir-rust-crate` is the one sibling checkout with a git remote. At
the time this site was written `fhir-databases` and `fhir-store` had no remote,
so their URLs assume they will be published under the `fhir-rust` organization
that owns this site. If they land elsewhere, change them there; nothing else
hard-codes them.

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
