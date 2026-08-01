// The documents, read from the vendored Markdown in content/.
//
// Everything here runs at build time only: it is imported from *.server.js
// modules, so neither the Markdown nor the renderer reaches the browser.

import { renderMarkdown } from './markdown.js';
import { routeFor } from './paths.js';

const raw = import.meta.glob('/content/**/*.md', {
	eager: true,
	query: '?raw',
	import: 'default'
});

/** @type {Record<string, string>} content path -> Markdown source */
const sources = Object.fromEntries(
	Object.entries(raw).map(([key, value]) => [key.replace('/content/', ''), value])
);

const firstHeading = (markdown) => {
	const match = /^#\s+(.+)$/m.exec(markdown);
	return match ? match[1].trim() : '';
};

const has = (file) => Object.hasOwn(sources, file);

const entry = (file) => ({
	file,
	route: /** @type {string} */ (routeFor(file)),
	title: firstHeading(sources[file]) || file
});

// The learning material, in the order doc/index.md presents it: tutorials
// first and in sequence, then the reference pages. Alphabetical order would
// interleave them, and these are meant to be read through.
const GUIDE_ORDER = [
	'doc/index.md',
	'doc/tutorial-01-getting-started.md',
	'doc/tutorial-02-storage-model.md',
	'doc/tutorial-03-querying-sql.md',
	'doc/tutorial-04-search.md',
	'doc/tutorial-05-history-and-audit.md',
	'doc/tutorial-06-porting.md',
	'doc/choosing-an-engine.md',
	'doc/storage-model.md',
	'doc/trust-boundary.md',
	'doc/examples.md',
	'doc/faq.md'
];

/** The guides and tutorials, in reading order. */
export const guides = GUIDE_ORDER.filter(has).map(entry);

// Spec sections sort by their leading number; the non-numbered companions
// (the fold, the matrix, the audit) follow, and are not normative sections.
const specNumber = (file) => {
	const match = /^spec\/(\d+)-/.exec(file);
	return match ? Number(match[1]) : Number.POSITIVE_INFINITY;
};

const specFiles = Object.keys(sources)
	.filter((file) => file.startsWith('spec/') && file !== 'spec/index.md')
	.sort((a, b) => specNumber(a) - specNumber(b) || a.localeCompare(b));

/** The specification sections, numbered ones first. */
export const specSections = specFiles.map(entry);

/** Normative numbered sections only — what the spec index page lists. */
export const specNormative = specSections.filter(
	(section) => specNumber(section.file) !== Number.POSITIVE_INFINITY
);

/** The non-normative companions: the fold, the matrix, the audit. */
export const specCompanions = specSections.filter(
	(section) => specNumber(section.file) === Number.POSITIVE_INFINITY
);

// Previous/next walk within a sequence. The two standalone pages — the
// project README and the documentation index — belong to neither.
const SEQUENCES = [
	{ label: 'Guides', items: guides },
	{ label: 'Specification', items: [entry('spec/index.md')].filter((e) => has(e.file)).concat(specSections) }
];

/** Every route this site publishes from content/, in prerender order. */
export function routes() {
	return Object.keys(sources)
		.map((file) => ({ file, route: routeFor(file) }))
		.filter((item) => item.route !== null);
}

/**
 * Render one document for a page load.
 *
 * @param {string} route e.g. "/docs/storage-model/"
 */
export function document(route) {
	const item = routes().find((candidate) => candidate.route === route);
	if (!item) return null;

	const rendered = renderMarkdown(sources[item.file], { file: item.file, route });

	const sequence = SEQUENCES.find((candidate) =>
		candidate.items.some((step) => step.route === route)
	);
	const index = sequence ? sequence.items.findIndex((step) => step.route === route) : -1;
	const sibling = (offset) => {
		const step = sequence?.items[index + offset];
		return index === -1 || !step ? null : { title: step.title, route: step.route };
	};

	return {
		route,
		file: item.file,
		title: rendered.title || item.file,
		summary: rendered.summary,
		html: rendered.html,
		headings: rendered.headings,
		section: sequence?.label ?? 'Documentation',
		previous: sibling(-1),
		next: sibling(1)
	};
}
