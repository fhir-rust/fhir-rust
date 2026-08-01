import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/**
 * Configured per the SvelteKit GitHub Pages guidance:
 * https://svelte.dev/docs/kit/adapter-static#GitHub-Pages
 *
 * - `fallback: '404.html'` replaces the default GitHub 404 page with this
 *   site's own error page.
 * - `static/.nojekyll` stops GitHub from running Jekyll over the output.
 * - `paths.base` stays empty: the repository is named after the organization
 *   (fhir-rust.github.io), so the site is served from the root rather than
 *   from /<repo-name>/. A project-pages repo would need
 *   `paths: { base: process.env.BASE_PATH }` and a BASE_PATH in the workflow.
 *
 * @type {import('@sveltejs/kit').Config}
 */
export default {
	// The vendored Lily components are written in TypeScript.
	preprocess: vitePreprocess(),
	kit: {
		// GitHub Pages serves plain files: prerender everything.
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: '404.html',
			strict: true
		}),
		prerender: {
			handleHttpError: 'fail',
			// The documents link to headings by anchor across many files; a
			// stale anchor is worth a warning, not a failed publish.
			handleMissingId: 'warn'
		}
	}
};
