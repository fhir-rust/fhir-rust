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
		case 'spec/index.md':
			return '/spec/';
		// The status document to trust. It earns a top-level route because it
		// is the thing a reader choosing a port actually needs.
		case 'spec/conformance-matrix.md':
			return '/conformance/';
	}
	const doc = /^doc\/([\w.-]+)\.md$/.exec(path);
	if (doc) return `/docs/${doc[1]}/`;
	const spec = /^spec\/([\w.-]+)\.md$/.exec(path);
	if (spec) return `/spec/${spec[1]}/`;
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
