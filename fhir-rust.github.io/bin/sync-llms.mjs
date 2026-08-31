#!/usr/bin/env node
// Generate this site's llms.txt / llms.json from the monorepo's copies.
//
// The workspace-root llms.txt/llms.json (spec/llms-json-and-llms-txt/) use
// links relative to the repository root -- correct there, but they only
// resolve inside a git checkout. Serving that text verbatim at
// <this site>/llms.txt would ship links that mostly 404 on this domain, since
// this site renders the monorepo's Markdown under its own routes rather than
// at their raw repository paths (see src/lib/paths.js). So this script does
// not copy the files: it rewrites every entry to wherever it actually
// resolves from this site's own domain -- a site route where the document is
// published here, the source repository otherwise -- using the exact same
// routeFor()/sourceUrl() mapping every other synced page's links go through.
//
// Per spec/llms-json-and-llms-txt/index.md, "Publishing a copy on a site".
//
// Source: $WORKSPACE if set, else the sibling checkout ../fhir-rust.
// Run after either workspace file changes:  npm run sync:llms

import { readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { routeFor, sourceUrl } from '../src/lib/paths.js';
import { REPOSITORY, SITE_URL } from '../src/lib/site.js';

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const workspace = resolve(process.env.WORKSPACE ?? join(siteRoot, '..', 'fhir-rust'));

if (!existsSync(join(workspace, 'llms.txt'))) {
	console.error(`No monorepo at ${workspace}. Set WORKSPACE=/path/to/fhir-rust.`);
	process.exit(1);
}

// routeFor() only knows the content *files* this site actually renders,
// keyed by their exact repository path -- a directory, or a file this site
// doesn't vendor but that still has an obvious page here, needs a manual
// nudge instead of the generic per-file rule.
const OVERRIDES = {
	// The model crate's examples/ directory becomes the generated
	// /examples/ index (see bin/sync-content.mjs) -- not a GitHub tree link.
	'fhir/examples': '/examples/'
};

/** Resolve a workspace-root-relative path to where it resolves from this site. */
function resolveLink(path) {
	const trimmed = path.replace(/\/$/, '');
	if (trimmed in OVERRIDES) return SITE_URL + OVERRIDES[trimmed];
	const route = routeFor(trimmed);
	return route ? SITE_URL + route : sourceUrl(trimmed);
}

const isUrl = (target) => /^[a-z][a-z0-9+.-]*:/i.test(target);

// --- llms.txt ----------------------------------------------------------

let txt = await readFile(join(workspace, 'llms.txt'), 'utf8');

const SITE_NOTE =
	`HL7® and FHIR® are registered trademarks of Health Level Seven International;\n` +
	`this project is not affiliated with or endorsed by HL7. This is ${SITE_URL}'s\n` +
	`copy of the monorepo's curated map: each link below resolves at this site's\n` +
	`own page where one is published, and at the source repository\n` +
	`(${REPOSITORY}) otherwise.`;

txt = txt.replace(
	/HL7® and FHIR® are registered trademarks[\s\S]*?its root\./,
	SITE_NOTE
);

txt = txt.replace(/\[([^\]]+)\]\(([^)]+)\)/g, (whole, title, target) =>
	isUrl(target) ? whole : `[${title}](${resolveLink(target)})`
);

await writeFile(join(siteRoot, 'static', 'llms.txt'), txt);

// --- llms.json -----------------------------------------------------------

const json = JSON.parse(await readFile(join(workspace, 'llms.json'), 'utf8'));

// notes[0] is the workspace copy's "paths are relative to the repo root"
// note -- not true of this copy, so it's replaced rather than kept. The
// rest (family composition order, the R4.x collision) is general knowledge
// and carries over unchanged.
json.notes[0] =
	`This is ${SITE_URL}'s copy of the monorepo's curated map: each entry's ` +
	`url resolves at this site's own page where one is published, and at ` +
	`the source repository (${REPOSITORY}) otherwise.`;

for (const section of json.sections) {
	for (const link of section.links) {
		link.url = isUrl(link.path) ? link.path : resolveLink(link.path);
		delete link.path;
	}
}

await writeFile(join(siteRoot, 'static', 'llms.json'), JSON.stringify(json, null, '\t') + '\n');

console.log('Synced llms.txt and llms.json into static/.');
