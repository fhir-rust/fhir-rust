import { examples, guides, modelSections, specNormative } from '$lib/docs.js';
import { SITE_NAME, SITE_TAGLINE } from '$lib/site.js';

/** The hub is this site's own page, so it takes only the lists it links to. */
export function load() {
	const slim = ({ route, title }) => ({ route, title });
	return {
		// page.data.title convention (see src/routes/+layout.svelte): the exact
		// <title> text, read by the layout for SharePicker.
		title: `${SITE_NAME} — ${SITE_TAGLINE}`,
		guides: guides.map(slim),
		spec: specNormative.map(slim),
		model: modelSections.map(slim),
		examples: examples.map(slim)
	};
}
