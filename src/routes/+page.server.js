import { examples, guides, modelSections, specNormative } from '$lib/docs.js';

/** The hub is this site's own page, so it takes only the lists it links to. */
export function load() {
	const slim = ({ route, title }) => ({ route, title });
	return {
		guides: guides.map(slim),
		spec: specNormative.map(slim),
		model: modelSections.map(slim),
		examples: examples.map(slim)
	};
}
