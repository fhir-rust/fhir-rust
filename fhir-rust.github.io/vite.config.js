import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		sveltekit({
			// The vendored Lily components are written in TypeScript.
			preprocess: vitePreprocess(),

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
		})
	]
});
