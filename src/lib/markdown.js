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

// Requirement id prefix -> the spec section that defines it. The prefixes are
// normative and permanent (`C0.5`), so this table is stable; sections 7 and 8
// are retired and 14 is per-port, which is why neither appears.
const REQUIREMENT_SECTIONS = {
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

const PREFIXED_ID = /^(C0|S1|G2|M3|R4|H5|P6|V9|O10|T11|PR12|X15|W16)\.\d+[a-z]?$/;
// The locale and accent fold numbers its requirements bare: L4, L12, L16.
const FOLD_ID = /^L\d+[a-z]?$/;
// Audit findings: F-01 .. F-27.
const FINDING_ID = /^F-\d+$/;

/**
 * Route for a requirement id written inside a code span, or null.
 * These ids are almost always in backticks in this project's prose, so the
 * code span is the right place to link them — an inline text match would fire
 * far less often and risk mangling ordinary prose.
 */
function requirementRoute(text) {
	const prefixed = PREFIXED_ID.exec(text);
	if (prefixed) return REQUIREMENT_SECTIONS[prefixed[1]] ?? null;
	if (FOLD_ID.test(text)) return '/spec/locale-accent-folding/';
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
				const target = requirementRoute(text);
				// A section never links to itself.
				if (!target || target === route) return `<code>${escaped}</code>`;
				return `<a class="requirement-link" href="${target}"><code>${escaped}</code></a>`;
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
