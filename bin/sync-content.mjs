#!/usr/bin/env node
// Vendor the project's Markdown into content/ so this site builds standalone.
//
// The workspace is laid out as a directory of sibling checkouts:
//
//   fhir-rust/
//     README.md  index.md  doc/       <- the prose
//     fhir-databases/spec/            <- the normative specification
//     fhir-rust.github.io/            <- this site
//
// content/ flattens that into two directories, doc/ and spec/, which is the
// arrangement the documents' own relative links already assume: doc/index.md
// links to ../spec/index.md, and index.md links to doc/ and spec/ alike.
//
// Source: $WORKSPACE if set, else the parent directory of this site.
// Run after the prose changes:  npm run sync:content

import { cp, mkdir, readdir, rm } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const workspace = resolve(process.env.WORKSPACE ?? join(siteRoot, '..'));

if (!existsSync(join(workspace, 'index.md'))) {
	console.error(`No workspace at ${workspace}. Set WORKSPACE=/path/to/fhir-rust.`);
	process.exit(1);
}

// Single files copied to the root of content/.
const files = ['README.md', 'index.md'];

// Directory -> destination inside content/. Only *.md is copied.
const dirs = [
	['doc', 'doc'],
	[join('fhir-databases', 'spec'), 'spec']
];

const contentDir = join(siteRoot, 'content');
await rm(contentDir, { recursive: true, force: true });
await mkdir(contentDir, { recursive: true });

let count = 0;

for (const file of files) {
	const from = join(workspace, file);
	if (!existsSync(from)) {
		console.warn(`skip (missing): ${file}`);
		continue;
	}
	await cp(from, join(contentDir, file));
	count += 1;
}

for (const [from, to] of dirs) {
	const source = join(workspace, from);
	if (!existsSync(source)) {
		console.warn(`skip (missing): ${from}/`);
		continue;
	}
	await mkdir(join(contentDir, to), { recursive: true });
	for (const name of await readdir(source)) {
		if (!name.endsWith('.md')) continue;
		await cp(join(source, name), join(contentDir, to, name));
		count += 1;
	}
}

console.log(`Synced ${count} Markdown files from ${workspace} into content/.`);
