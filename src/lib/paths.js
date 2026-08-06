// Mapping between vendored content files and site routes.
// Content paths are always relative to content/, e.g. "doc/storage-model.md".

import { ORG_OPENEHR, REPOSITORY, REPO_OPENEHR } from './site.js';

/** Resolve `href` (as written inside `fromFile`) to a content path. */
export function contentPath(href, fromFile) {
	const from = fromFile.includes('/') ? fromFile.slice(0, fromFile.lastIndexOf('/')) : '';
	const segments = href.startsWith('/')
		? href.slice(1).split('/')
		: [...from.split('/'), ...href.split('/')];
	const out = [];
	for (const segment of segments) {
		if (segment === '' || segment === '.') continue;
		if (segment === '..') out.pop();
		else out.push(segment);
	}
	return out.join('/');
}

/** Site route for a content path, or null when the file is not published. */
export function routeFor(path) {
	switch (path) {
		// The hub at "/" is this site's own page, so the repository README —
		// which is the project's own five-minute introduction — gets its own
		// route rather than competing with it.
		case 'README.md':
			return '/overview/';
		// The documentation index: every entry point in the repository.
		case 'index.md':
			return '/docs/';
		// doc/index.md is the narrower "learning material" index, one level in.
		case 'doc/index.md':
			return '/docs/guides/';
		// The four-family specification root: which spec governs which code.
		case 'spec/index.md':
			return '/specs/';
		case 'spec/publishing.md':
			return '/specs/publishing/';
		// The database core keeps the routes it always had: /spec/… predates
		// the monorepo merge and is linked from outside this site.
		case 'spec/databases/index.md':
			return '/spec/';
		// The status document to trust. It earns a top-level route because it
		// is the thing a reader choosing a port actually needs.
		case 'spec/databases/conformance-matrix.md':
			return '/conformance/';
		// The model family: the crate README is the landing page, its spec a
		// tree below it.
		case 'fhir/README.md':
			return '/model/';
		case 'fhir/spec/index.md':
			return '/model/spec/';
		case 'fhir/spec/README.md':
			return '/model/spec/about/';
		// The HTTP surface and the shared persistence core.
		case 'fhir-loco/README.md':
			return '/server/';
		case 'fhir-loco/spec/index.md':
			return '/server/spec/';
		case 'fhir-store/README.md':
			return '/store/';
		case 'examples/index.md':
			return '/examples/';
	}
	const doc = /^doc\/([\w.-]+)\.md$/.exec(path);
	if (doc) return `/docs/${doc[1]}/`;
	const spec = /^spec\/databases\/([\w.-]+)\.md$/.exec(path);
	if (spec) return `/spec/${spec[1]}/`;
	const model = /^fhir\/spec\/([\w.-]+)\.md$/.exec(path);
	if (model) return `/model/spec/${model[1]}/`;
	const server = /^fhir-loco\/spec\/([\w.-]+)\.md$/.exec(path);
	if (server) return `/server/spec/${server[1]}/`;
	const example = /^examples\/([\w.-]+)\.md$/.exec(path);
	if (example) return `/examples/${example[1]}/`;
	return null;
}

// Unpublished paths still deserve a working link. Everything the synced
// documents reference lives in one of two repositories; send readers there
// rather than leaving a relative href that 404s on this site.
const looksLikeFile = (path) => /\.[a-z0-9]+$/i.test(path);

const blobOrTree = (repo, path) =>
	path ? `${repo}/${looksLikeFile(path) ? 'blob' : 'tree'}/main/${path}` : repo;

/** GitHub URL for a content path this site does not publish. */
export function sourceUrl(path) {
	// The openEHR family is a separate workspace. Its `openehr` crate is a
	// repository of its own, so `openehr/spec/index.md` is `spec/index.md`
	// there; the rest of that family is unpublished and gets the org page.
	if (path === 'openehr' || path.startsWith('openehr/')) {
		return blobOrTree(REPO_OPENEHR, path.slice('openehr/'.length));
	}
	if (path.startsWith('openehr')) return ORG_OPENEHR;
	return blobOrTree(REPOSITORY, path);
}

/** Rewrite a Markdown link into a site link, leaving external links untouched. */
export function rewriteHref(href, fromFile) {
	if (!href) return href;
	if (/^[a-z][a-z0-9+.-]*:/i.test(href) || href.startsWith('//') || href.startsWith('#'))
		return href;
	const hashAt = href.indexOf('#');
	const hash = hashAt === -1 ? '' : href.slice(hashAt);
	const target = hashAt === -1 ? href : href.slice(0, hashAt);
	if (!target) return href;
	const path = contentPath(target, fromFile);
	const route = routeFor(path);
	// The fragment survives either way: GitHub slugs Markdown headings with the
	// same algorithm github-slugger implements, so the anchor lands there too.
	return (route ?? sourceUrl(path)) + hash;
}
