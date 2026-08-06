<script>
	import { page } from '$app/state';
	import SkipLink from '$lib/lily/components/SkipLink.svelte';
	import Header from '$lib/lily/components/Header.svelte';
	import Footer from '$lib/lily/components/Footer.svelte';
	import ThemePicker from '$lib/lily/helpers/ThemePicker.svelte';
	import TextSizePicker from '$lib/lily/helpers/TextSizePicker.svelte';
	import { REPOSITORY, SITE_NAME, THEMES, THEME_LABELS } from '$lib/site.js';
	import '../styles/site.css';

	let { children } = $props();

	const links = [
		{ href: '/overview/', label: 'Overview' },
		{ href: '/docs/', label: 'Docs' },
		{ href: '/examples/', label: 'Examples' },
		{ href: '/model/', label: 'Model' },
		{ href: '/spec/', label: 'Spec' },
		{ href: '/conformance/', label: 'Conformance' }
	];

	const current = (href) => (page.url.pathname === href ? 'page' : undefined);
</script>

<SkipLink href="#main" label="Skip to main content" />

<Header label="Site header" class="site-header">
	<div class="site-header-inner">
		<a class="site-brand" href="/">
			<img src="/favicon.svg" alt="" aria-hidden="true" width="28" height="28" />
			<span>{SITE_NAME}</span>
		</a>
		<nav class="site-nav" aria-label="Main">
			{#each links as link (link.href)}
				<a href={link.href} aria-current={current(link.href)}>{link.label}</a>
			{/each}
			<a href={REPOSITORY}>GitHub</a>
		</nav>
		<div class="site-tools">
			<TextSizePicker
				label="Text size"
				sizes={['small', 'medium', 'large', 'x-large']}
				storageKey="fhir-rust-text-size"
			/>
			<ThemePicker
				label="Theme"
				themesUrl="/themes/"
				themes={THEMES}
				themeLabels={THEME_LABELS}
				storageKey="fhir-rust-theme"
				detectFromSystem
			/>
		</div>
	</div>
</Header>

<main id="main" class="site-main">
	{@render children()}
</main>

<Footer label="Site footer" class="site-footer">
	<div class="site-footer-inner">
		<p>
			<strong>fhir-rust</strong> — FHIR resources stored in a SQL database as real relational
			tables, and given back losslessly. Pre-release: the
			<a href="/conformance/">conformance matrix</a> is the status document to trust. Built with the
			<a href="https://github.com/LilyDesignSystem">Lily Design System</a>.
		</p>
		<div class="site-footer-links">
			<a href={REPOSITORY}>GitHub</a>
			<a href="/spec/">Specification</a>
			<a href="/spec/audit/">Audit findings</a>
			<a href="/sitemap.xml">Sitemap</a>
		</div>
	</div>
</Footer>
