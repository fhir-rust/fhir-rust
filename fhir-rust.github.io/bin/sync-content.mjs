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
// Source: $WORKSPACE if set, else this site's own parent directory —
// spec/monorepo-github-pages/ (fhir-rust) moved this site to live inside the
// monorepo, at fhir-rust.github.io/, so the monorepo root is one level up,
// not a sibling checkout.
// Run after the prose changes:  npm run sync:content

import { cp, lstat, mkdir, readdir, readFile, readlink, rm, symlink, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const workspace = resolve(process.env.WORKSPACE ?? join(siteRoot, '..'));

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
//
// The single-topic specs below are the cross-family documents spec/index.md
// itself links to (agent skills, agents directory casing, Dependabot,
// funding, git tags, HL7 trademark fair use, the AI-guidance llms.txt/
// llms.json files, the monorepo/GitHub-Pages structure, the current Node
// version, professionalization, the MSRV policy, the serde_json
// float-roundtrip decision, the special-files list, and Trusted Publishing)
// — added here after they'd accumulated for weeks with no sync-script
// update, so spec/index.md's own links 404'd on this site while resolving
// fine in the repository. `agent-skills` and `llms-json-and-llms-txt` hit
// the same gap a second time (added to spec/index.md 2026-08-30, missed
// here until the next day); `monorepo-github-pages` and
// `node-current-version` a third (added 2026-08-31, caught only once this
// site itself moved inside the monorepo and its own gate ran against its
// own content) -- both are named in the array below AND in routeFor()'s
// switch in src/lib/paths.js; a spec dir needs both or its links resolve
// nowhere. Each is a single `index.md` (one also carries a template file,
// `special-files-for-public-repos/AI_STATEMENT.md`) — flat, matching every
// other entry in this list, not nested further.
const dirs = [
	'doc',
	'spec/databases',
	'spec/agent-skills',
	'spec/agents-directory-name-is-lowercase',
	'spec/dependabot',
	'spec/free-open-source-funding',
	'spec/git-tags-name-published-versions',
	'spec/hl7-trademarks-fair-use',
	'spec/llms-json-and-llms-txt',
	'spec/monorepo-github-pages',
	'spec/node-current-version',
	'spec/professionalization',
	'spec/rust-msrv-n-minus-2',
	'spec/serde-json-float-roundtrip-arbitrary-precision',
	'spec/special-files-for-public-repos',
	'spec/trusted-publishing',
	'fhir/spec',
	'fhir-loco/spec'
];

// node:fs/promises' cp() resolves a symlink's target to an absolute path
// when it recreates it at the destination (verified directly against this
// Node version, not assumed) -- fine for a build artifact nobody commits,
// wrong for one that is: every "README.md -> index.md" symlink in spec/*
// synced this way baked in this machine's own home directory path, which
// broke the moment a different machine (hosted CI) tried to read it. Copy
// a symlink by hand instead, preserving its relative target verbatim.
async function copyPreservingSymlinks(from, to) {
	if ((await lstat(from)).isSymbolicLink()) {
		await symlink(await readlink(from), to);
	} else {
		await cp(from, to);
	}
}

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
		await copyPreservingSymlinks(join(source, name), join(contentDir, dir, name));
		count += 1;
	}
}

// --- examples: fhir/examples/*.rs -> content/examples/<name>.md -------------
//
// Each example opens with a `//!` doc-comment tutorial (that is the house
// style, checked in the crate's own CI), so the page writes itself: the
// comment becomes the prose, the rest becomes the listing. The first line of
// the comment is the title.
//
// These pages are committed, git-tracked files under fhir-rust.github.io/ --
// as directly readable on GitHub as any other Markdown in the repository,
// not merely a build intermediate the live site's own footer disclaimer
// covers. scripts/check-trademarks.sh holds every .md file in the repo to
// the same rule (spec/hl7-trademarks-fair-use/), generated or not, so the
// generator carries it: mark the first prose use of each HL7 word mark and
// append the disclaimer, exactly what a human author would do by hand.
// Never applied to the fenced Rust listing below the prose -- inserting a
// ® into copy-pasted source would corrupt it, not just be non-compliant.

const WORD_MARKS = ['HL7', 'FHIR', 'CDA'];
const TRADEMARK_DISCLAIMER =
	'HL7®, and FHIR® are the registered trademarks of Health Level Seven ' +
	'International and their use of these trademarks does not constitute ' +
	'an endorsement by HL7.';

function markWordMarks(text) {
	let out = text;
	for (const mark of WORD_MARKS) {
		const m = new RegExp(`\\b${mark}\\b(?!®)`).exec(out);
		if (m) out = out.slice(0, m.index + mark.length) + '®' + out.slice(m.index + mark.length);
	}
	return out;
}

function usesAnyWordMark(text) {
	return WORD_MARKS.some((mark) => new RegExp(`\\b${mark}\\b`).test(text));
}

function trademarkSection() {
	return ['## Trademarks', '', TRADEMARK_DISCLAIMER, ''].join('\n');
}

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
		let body = prose.slice(prose.indexOf(titleLine) + 1).join('\n');

		// Rustdoc intra-doc links ([`Bundle`]: fhir::r5::resources::Bundle)
		// only resolve inside rustdoc. Drop the reference definitions whose
		// target is a Rust path rather than a URL, then unwrap the now-orphaned
		// bracket references — otherwise they render as dead `fhir:…` links,
		// which the first build of these pages shipped.
		body = body
			.split('\n')
			.filter((line) => !/^\[[^\]]+\]:\s+(?!https?:\/\/)\S+\s*$/.test(line.trim()))
			.join('\n')
			.replace(/\[([^\]]+)\](?!\()/g, '$1');

		const pageProse = markWordMarks(`# \`${stem}\` — ${title}\n\n${body.trim()}`);

		const markdown = [
			pageProse,
			'',
			'## The program',
			'',
			'```rust',
			code,
			'```',
			'',
			`*Source: [\`fhir/examples/${name}\`](../fhir/examples/${name}) in the repository.*`,
			'',
			...(usesAnyWordMark(pageProse) ? [trademarkSection()] : [])
		].join('\n');

		await writeFile(join(outExamples, `${stem}.md`), markdown);
		exampleIndex.push({ stem, title });
		count += 1;
	}

	const indexProse = markWordMarks(
		[
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
		].join('\n')
	);
	const index = [indexProse, ...(usesAnyWordMark(indexProse) ? [trademarkSection()] : [])].join(
		'\n'
	);
	await writeFile(join(outExamples, 'index.md'), index);
	count += 1;
}

console.log(`Synced ${count} files from ${workspace} into content/.`);
