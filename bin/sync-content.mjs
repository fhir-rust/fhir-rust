#!/usr/bin/env node
// Vendor the monorepo's Markdown into content/ so this site builds standalone.
//
// The source is the fhir-rust monorepo (one repository, four families):
//
//   fhir-rust/
//     README.md  index.md  doc/       <- the prose and tutorials
//     spec/                           <- the four-family specification root
//       index.md  publishing.md
//       databases/                    <- the database core (§0–§16)
//     fhir/                           <- the model family
//       README.md  spec/  examples/
//     fhir-loco/                      <- the HTTP surface
//       README.md  spec/
//     fhir-store/README.md            <- the shared persistence core
//
// content/ mirrors that layout rather than flattening it, so every document's
// own relative links resolve exactly as they do in the repository —
// doc/index.md links to ../spec/databases/index.md, and it lands.
//
// Examples are the one transformation: each fhir/examples/*.rs becomes a
// Markdown page — its `//!` header as prose, the program as a fenced block —
// plus a generated examples/index.md. Everything else is copied verbatim.
//
// Source: $WORKSPACE if set, else the sibling checkout ../fhir-rust.
// Run after the prose changes:  npm run sync:content

import { cp, mkdir, readdir, readFile, rm, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const workspace = resolve(process.env.WORKSPACE ?? join(siteRoot, '..', 'fhir-rust'));

if (!existsSync(join(workspace, 'index.md'))) {
	console.error(`No monorepo at ${workspace}. Set WORKSPACE=/path/to/fhir-rust.`);
	process.exit(1);
}

// Single files, copied to the same path inside content/.
const files = [
	'README.md',
	'index.md',
	'spec/index.md',
	'spec/publishing.md',
	'fhir/README.md',
	'fhir-loco/README.md',
	'fhir-store/README.md'
];

// Directory -> same directory inside content/. Only *.md is copied.
const dirs = ['doc', 'spec/databases', 'fhir/spec', 'fhir-loco/spec'];

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
	await mkdir(join(contentDir, dirname(file)), { recursive: true });
	await cp(from, join(contentDir, file));
	count += 1;
}

for (const dir of dirs) {
	const source = join(workspace, dir);
	if (!existsSync(source)) {
		console.warn(`skip (missing): ${dir}/`);
		continue;
	}
	await mkdir(join(contentDir, dir), { recursive: true });
	for (const name of await readdir(source)) {
		if (!name.endsWith('.md')) continue;
		await cp(join(source, name), join(contentDir, dir, name));
		count += 1;
	}
}

// --- examples: fhir/examples/*.rs -> content/examples/<name>.md -------------
//
// Each example opens with a `//!` doc-comment tutorial (that is the house
// style, checked in the crate's own CI), so the page writes itself: the
// comment becomes the prose, the rest becomes the listing. The first line of
// the comment is the title.

const examplesDir = join(workspace, 'fhir', 'examples');
const outExamples = join(contentDir, 'examples');
const exampleIndex = [];

if (existsSync(examplesDir)) {
	await mkdir(outExamples, { recursive: true });
	const names = (await readdir(examplesDir)).filter((n) => n.endsWith('.rs')).sort();
	for (const name of names) {
		const stem = name.slice(0, -3);
		const source = await readFile(join(examplesDir, name), 'utf8');
		const lines = source.split('\n');

		const prose = [];
		let i = 0;
		for (; i < lines.length; i += 1) {
			const line = lines[i];
			if (line.startsWith('//!')) {
				prose.push(line.slice(line.startsWith('//! ') ? 4 : 3));
			} else if (line.trim() === '') {
				// blank lines inside the header block are fine; stop at code
				if (lines[i + 1]?.startsWith('//!')) prose.push('');
				else break;
			} else break;
		}
		const code = lines.slice(i).join('\n').trim();

		// First sentence of the header is the title; the rest stays prose.
		const titleLine = prose.find((l) => l.trim() !== '') ?? stem;
		const title = titleLine.replace(/\.\s*$/, '');
		const body = prose.slice(prose.indexOf(titleLine) + 1).join('\n');

		const markdown = [
			`# \`${stem}\` — ${title}`,
			'',
			body.trim(),
			'',
			'## The program',
			'',
			'```rust',
			code,
			'```',
			'',
			`*Source: [\`fhir/examples/${name}\`](../fhir/examples/${name}) in the repository.*`,
			''
		].join('\n');

		await writeFile(join(outExamples, `${stem}.md`), markdown);
		exampleIndex.push({ stem, title });
		count += 1;
	}

	const index = [
		'# Examples',
		'',
		'Runnable programs from the model crate’s `examples/` directory. Each is',
		'a tutorial in its header comment and a complete program below it; run',
		'one from a checkout with `cargo run --example <name>` (some need extra',
		'cargo features — the page says which).',
		'',
		...exampleIndex.map(({ stem, title }) => `- [\`${stem}\`](${stem}.md) — ${title}`),
		'',
		'The database family’s worked examples are a guide of their own:',
		'[Examples](../doc/examples.md).',
		''
	].join('\n');
	await writeFile(join(outExamples, 'index.md'), index);
	count += 1;
}

console.log(`Synced ${count} files from ${workspace} into content/.`);
