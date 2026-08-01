<script>
	import { page } from '$app/state';
	import ArticleLayout from '$lib/lily/components/ArticleLayout.svelte';
	import { SITE_NAME } from '$lib/site.js';

	const heading = $derived(page.status === 404 ? 'Page not found' : 'Something went wrong');
</script>

<svelte:head>
	<title>{heading} — {SITE_NAME}</title>
	<meta name="robots" content="noindex" />
</svelte:head>

<ArticleLayout label={heading} class="prose">
	<h1>{heading}</h1>
	{#if page.status === 404}
		<p>
			There is no page at <code>{page.url.pathname}</code>. The documents may have moved or been
			renamed.
		</p>
	{:else}
		<p>{page.error?.message ?? 'The page could not be loaded.'}</p>
	{/if}
	<p>
		Try the <a href="/">home page</a>, the <a href="/docs/">documentation index</a>, the
		<a href="/spec/">specification</a>, or the <a href="/conformance/">conformance matrix</a>.
	</p>
</ArticleLayout>
