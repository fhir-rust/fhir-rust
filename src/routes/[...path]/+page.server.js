import { error } from '@sveltejs/kit';
import { document, routes } from '$lib/docs.js';
import { SITE_NAME } from '$lib/site.js';

/** Prerender every document this site publishes, without relying on crawling. */
export function entries() {
	return routes().map(({ route }) => ({ path: route.replace(/^\/|\/$/g, '') }));
}

export function load({ params }) {
	// A rest parameter keeps the trailing slash that trailingSlash: 'always' adds.
	const route = `/${params.path.replace(/\/+$/, '')}/`;
	const doc = document(route);
	if (!doc) error(404, `No page at ${route}`);
	// page.data.title convention (see src/routes/+layout.svelte): the exact
	// <title> text, read by the layout for SharePicker. A failed load (the
	// error(404) above) never reaches this return, so the error page has no
	// page.data.title — the layout's fallback covers it.
	return { doc, title: `${doc.title} — ${SITE_NAME}` };
}
