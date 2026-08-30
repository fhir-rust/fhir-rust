#!/usr/bin/env node
// Vendor Lily Design System's theme CSS.
//
// Lily's Svelte components (SkipLink, Header, Footer, ArticleLayout, Card,
// SectionHeading, ContentsNav/List/ListItem, BreadcrumbNav/List/ListItem,
// PaginationNav/List/ListItem) and the theme/text-size picker helpers are
// installed from npm (`lily-design-system-svelte-headless`,
// `lily-design-system-svelte-theme-picker`,
// `lily-design-system-svelte-text-size-picker`) — see package.json. Those
// packages are headless: no CSS ships with them by design, and each
// consumer supplies its own theme stylesheets via ThemePicker's `themesUrl`
// prop. Lily's theme CSS itself has no npm package (as of 2026-08-30), so
// this script still vendors it from a sibling checkout.
//
// Source: $LILY if set, else ~/git/lilydesignsystem/lily-design-system.
// Run after Lily's themes change:  npm run sync:lily-themes

import { cp, mkdir, readdir, rm, writeFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { homedir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const lily = resolve(
	process.env.LILY ?? join(homedir(), 'git', 'lilydesignsystem', 'lily-design-system')
);

const themes = join(lily, 'themes');

if (!existsSync(themes)) {
	console.error(`No Lily checkout at ${lily}. Set LILY=/path/to/lily-design-system.`);
	process.exit(1);
}

// Themes offered by the picker. Keep this in step with THEMES in src/lib/site.js
// — the picker lists what site.js names, and 404s on anything not copied here.
const themeNames = ['light', 'dark', 'corporate', 'business', 'dim', 'nord', 'lofi', 'night'];

const themeDir = join(siteRoot, 'static', 'themes');
await rm(themeDir, { recursive: true, force: true });
await mkdir(themeDir, { recursive: true });
const available = new Set(await readdir(themes));
const themeFiles = [];
for (const name of themeNames) {
	const file = `${name}.css`;
	if (!available.has(file)) {
		console.error(`missing theme: ${file}`);
		process.exit(1);
	}
	await cp(join(themes, file), join(themeDir, file));
	themeFiles.push(file);
}

let commit = 'unknown';
try {
	commit = execFileSync('git', ['-C', lily, 'rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
} catch {
	// A tarball checkout has no git metadata; provenance is best-effort.
}

await writeFile(
	join(themeDir, 'VENDOR.md'),
	`# Vendored Lily Design System themes

These files are copied verbatim from the Lily Design System (MIT licence) by
\`bin/sync-lily-themes.mjs\`. Do not edit them here — change them upstream and
re-run \`npm run sync:lily-themes\`.

Lily's Svelte components and picker helpers are installed from npm instead of
vendored (\`lily-design-system-svelte-headless\`,
\`lily-design-system-svelte-theme-picker\`,
\`lily-design-system-svelte-text-size-picker\` in package.json) — only the
theme CSS has no npm package yet.

- Source: <https://github.com/LilyDesignSystem>
- Commit: \`${commit}\`
- Themes: ${themeFiles.length} files
`
);

console.log(`Vendored ${themeFiles.length} themes from ${lily} (${commit.slice(0, 9)}).`);
