import { Marked } from 'marked';
import GithubSlugger from 'github-slugger';
import { rewriteHref } from './paths.js';

const escapeAttribute = (value) =>
	value.replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;');

const plainText = (html) =>
	html
		.replace(/<[^>]*>/g, '')
		.replace(/&amp;/g, '&')
		.replace(/&lt;/g, '<')
		.replace(/&gt;/g, '>')
		.replace(/&quot;/g, '"')
		.replace(/&#39;/g, "'")
		.trim();

// Requirement id prefix -> the spec section that defines it, per family. The
// prefixes are normative and permanent (`C0.5`), so these tables are stable;
// database sections 7 and 8 are retired and 14 is per-port, which is why
// neither appears.
//
// `R4.x` is the one prefix two families share — db:R4 (lossless round-trip)
// and model:R4 (resources) — which is why the lookup is keyed by the file the
// id appears in: a model page's `R4.2` must land in the model spec, not the
// database one. That collision is documented in the repository's own
// spec/index.md and is exactly why a single global table would mislink.
const DB_SECTIONS = {
	C0: '/spec/00-conformance/',
	S1: '/spec/01-scope/',
	G2: '/spec/02-schema-generation/',
	M3: '/spec/03-storage-model/',
	R4: '/spec/04-shredding-and-reconstruction/',
	H5: '/spec/05-versioning-and-history/',
	P6: '/spec/06-search/',
	V9: '/spec/09-validation/',
	O10: '/spec/10-operations/',
	T11: '/spec/11-conformance-testing/',
	PR12: '/spec/12-trust-principal-and-audit/',
	X15: '/spec/15-portability-and-dialects/',
	W16: '/spec/16-repository-and-release/'
};

// The model family numbers its requirements R<section>.<n> across fourteen
// sections; the section number picks the file.
const MODEL_SECTIONS = {
	R1: '/model/spec/01-overview/',
	R2: '/model/spec/02-primitive-types/',
	R3: '/model/spec/03-complex-datatypes/',
	R4: '/model/spec/04-resources/',
	R5: '/model/spec/05-code-systems/',
	R6: '/model/spec/06-serialization/',
	R7: '/model/spec/07-validation/',
	R8: '/model/spec/08-code-generation/',
	R9: '/model/spec/09-primitive-extensions/',
	R10: '/model/spec/10-invariants-coverage/',
	R11: '/model/spec/11-choice-types/',
	R12: '/model/spec/12-fhir-releases/',
	R13: '/model/spec/13-assurance/',
	R14: '/model/spec/14-cross-release-conversion/'
};

// The HTTP surface: SV<section>.<n> across four sections.
const SERVER_SECTIONS = {
	SV1: '/server/spec/01-scope-and-conformance/',
	SV2: '/server/spec/02-endpoints/',
	SV3: '/server/spec/03-trust-and-attribution/',
	SV4: '/server/spec/04-operations/'
};

const DB_ID = /^(C0|S1|G2|M3|R4|H5|P6|V9|O10|T11|PR12|X15|W16)\.\d+[a-z]?$/;
const MODEL_ID = /^(R1[0-4]|R[1-9])\.\d+[a-z]?$/;
const SERVER_ID = /^(SV[1-4])\.\d+[a-z]?$/;
// Qualified spellings resolve regardless of which page they appear on —
// spec/index.md asks writers to disambiguate the R4 collision exactly so.
const QUALIFIED_ID = /^(db|model):((?:R1[0-4]|R[1-9]|C0|S1|G2|M3|H5|P6|V9|O10|T11|PR12|X15|W16|SV[1-4])\.\d+[a-z]?)$/;
// The locale and accent fold numbers its requirements bare: L4, L12, L16.
const FOLD_ID = /^L\d+[a-z]?$/;
// Audit findings: F-01 upward. The register is the database family's.
const FINDING_ID = /^F-\d+$/;

/**
 * Route for a requirement id written inside a code span, or null.
 * These ids are almost always in backticks in this project's prose, so the
 * code span is the right place to link them — an inline text match would fire
 * far less often and risk mangling ordinary prose. The file the id appears in
 * decides its family (the `R4.x` collision), and a `db:`/`model:` qualifier
 * overrides that.
 */
function requirementRoute(text, file = '') {
	const qualified = QUALIFIED_ID.exec(text);
	if (qualified) {
		const table = qualified[1] === 'model' ? MODEL_SECTIONS : DB_SECTIONS;
		const prefix = qualified[2].split('.')[0];
		return table[prefix] ?? null;
	}
	const inModel = file.startsWith('fhir/');
	const inServer = file.startsWith('fhir-loco/');
	if (inModel) {
		const model = MODEL_ID.exec(text);
		if (model) return MODEL_SECTIONS[model[1]] ?? null;
	}
	if (inServer) {
		const server = SERVER_ID.exec(text);
		if (server) return SERVER_SECTIONS[server[1]] ?? null;
	}
	// SV ids are unambiguous everywhere — the database register cites them.
	const server = SERVER_ID.exec(text);
	if (server) return SERVER_SECTIONS[server[1]] ?? null;
	if (!inModel && !inServer) {
		const db = DB_ID.exec(text);
		if (db) return DB_SECTIONS[db[1]] ?? null;
		if (FOLD_ID.test(text)) return '/spec/locale-accent-folding/';
	}
	if (FINDING_ID.test(text)) return '/spec/audit/';
	return null;
}

/**
 * Render one content file.
 *
 * @param {string} markdown raw file contents
 * @param {object} options
 * @param {string} options.file content path, used to resolve relative links
 * @param {string} [options.route] the page's own route, so it never links to itself
 * @returns {{ html: string, title: string, headings: Array<{depth: number, id: string, text: string}>, summary: string }}
 */
export function renderMarkdown(markdown, { file, route }) {
	const slugger = new GithubSlugger();
	const headings = [];
	let title = '';

	const marked = new Marked({ gfm: true });
	marked.use({
		renderer: {
			heading({ tokens, depth }) {
				const html = this.parser.parseInline(tokens);
				const text = plainText(html);
				const id = slugger.slug(text);
				if (depth === 1 && !title) title = text;
				if (depth === 2 || depth === 3) headings.push({ depth, id, text });
				// The page title needs no self-anchor.
				const anchor =
					depth === 1
						? ''
						: `<a class="heading-anchor" href="#${id}" aria-label="Link to “${escapeAttribute(text)}”">#</a>`;
				return `<h${depth} id="${id}">${html}${anchor}</h${depth}>\n`;
			},
			code({ text, lang }) {
				const cls = lang ? ` class="language-${escapeAttribute(lang.split(/\s+/)[0])}"` : '';
				const escaped = text
					.replace(/&/g, '&amp;')
					.replace(/</g, '&lt;')
					.replace(/>/g, '&gt;');
				return `<pre><code${cls}>${escaped}</code></pre>\n`;
			},
			codespan({ text }) {
				const escaped = text.replace(/&/g, '&amp;').replace(/</g, '&lt;');
				const target = requirementRoute(text, file);
				// A section never links to itself.
				if (!target || target === route) return `<code>${escaped}</code>`;
				return `<a class="requirement-link" href="${target}"><code>${escaped}</code></a>`;
			},
			// The model family writes its requirement ids bold (**R13.14**) where
			// the database family writes backticks, and both families bold their
			// audit findings (**F-65**). Linking strong tokens too makes every
			// spelling navigable. Safe against nested anchors: no synced document
			// bolds text inside a link (checked at sync time by convention — the
			// content has no `[**`).
			strong({ tokens }) {
				const html = this.parser.parseInline(tokens);
				const target = requirementRoute(plainText(html), file);
				if (!target || target === route) return `<strong>${html}</strong>`;
				return `<a class="requirement-link" href="${target}"><strong>${html}</strong></a>`;
			},
			link({ href, title: linkTitle, tokens }) {
				const html = this.parser.parseInline(tokens);
				const resolved = rewriteHref(href, file);
				const external = /^[a-z][a-z0-9+.-]*:/i.test(resolved) || resolved.startsWith('//');
				const attributes = [
					`href="${escapeAttribute(resolved)}"`,
					linkTitle ? `title="${escapeAttribute(linkTitle)}"` : '',
					external ? 'rel="noopener noreferrer"' : ''
				].filter(Boolean);
				return `<a ${attributes.join(' ')}>${html}</a>`;
			}
		}
	});

	const html = marked.parse(markdown);
	return { html, title, headings, summary: summarize(markdown) };
}

/** A one-line description for <meta name="description">. */
function summarize(markdown) {
	for (const line of markdown.split('\n')) {
		const text = line.trim();
		if (!text || text.startsWith('#') || text.startsWith('---') || text.startsWith('>')) continue;
		// Table rows and list bullets read badly as a description.
		if (text.startsWith('|')) continue;
		const plain = text
			.replace(/\[([^\]]+)\]\([^)]*\)/g, '$1')
			.replace(/[*_`]/g, '')
			.trim();
		if (plain.length < 20) continue;
		return plain.length > 300 ? `${plain.slice(0, 297).trimEnd()}…` : plain;
	}
	return '';
}
