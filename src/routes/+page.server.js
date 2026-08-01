import { guides, specNormative } from '$lib/docs.js';

/** The hub is this site's own page, so it takes only the lists it links to. */
export function load() {
	return {
		guides: guides.map(({ route, title }) => ({ route, title })),
		spec: specNormative.map(({ route, title }) => ({ route, title }))
	};
}
