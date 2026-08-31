<script>
	import { page } from '$app/state';
	import { SkipLink, Header, Footer } from 'lily-design-system-svelte-headless';
	import { ThemePicker } from 'lily-design-system-svelte-theme-picker';
	import { TextSizePicker } from 'lily-design-system-svelte-text-size-picker';
	import { SharePicker } from 'lily-design-system-svelte-share-picker';
	import {
		REPOSITORY,
		SITE_NAME,
		SITE_TAGLINE,
		THEMES,
		THEME_LABELS,
		SHARE_TARGETS
	} from '$lib/site.js';
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

	// page.data.title convention: every route's load() returns `title`, the
	// exact text its own <svelte:head><title> uses (src/routes/+page.server.js,
	// src/routes/[...path]/+page.server.js) — one definition per page instead
	// of the <title> tag and this share control disagreeing. A route whose
	// load never returns data has no page.data.title; the only case today is
	// +error.svelte, since its nearest load (in [...path]) throws before
	// returning — falls back to the site title.
	const shareTitle = $derived(page.data.title ?? `${SITE_NAME} — ${SITE_TAGLINE}`);

	// Attribute-based, multi-stylesheet theming: every theme's CSS is loaded
	// (the <link data-theme-stylesheet> block below), not just the active
	// one, so switching never re-fetches or waits on the network — the
	// previous approach let ThemePicker swap the href of one dynamically
	// managed <link>, which fetched a fresh stylesheet on every switch.
	// Only one is ever enabled: ThemePicker already sets `data-theme` on
	// <html> as its single source of truth (every static/themes/*.css file
	// self-documents this — `:root[data-theme="…"]`), so this reads that
	// attribute back rather than trusting onChange's own argument to agree
	// with it. ThemePicker still creates one extra managed <link> of its
	// own (unconditional in the package, no prop disables it); harmless —
	// same cached URL as one of the links below, so no extra fetch.
	function applyStylesheet() {
		if (typeof document === 'undefined') return;
		const theme = document.documentElement.getAttribute('data-theme');
		for (const link of document.querySelectorAll('link[data-theme-stylesheet]')) {
			link.disabled = link.dataset.themeStylesheet !== theme;
		}
	}
</script>

<svelte:head>
	{#each THEMES as theme (theme)}
		<link
			rel="stylesheet"
			href={`/themes/${theme}.css`}
			data-theme-stylesheet={theme}
			disabled={theme !== 'light'}
		/>
	{/each}
</svelte:head>

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
				onChange={applyStylesheet}
			/>
			<SharePicker
				label="Share this page"
				title={shareTitle}
				text={SITE_TAGLINE}
				targets={SHARE_TARGETS}
				copyLabel="Copy link"
				copiedLabel="Link copied"
				copyFailedLabel="Could not copy — copy it from the address bar"
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
			<strong>fhir-rust</strong> — FHIR® resources stored in a SQL database as real relational
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
		<p class="site-footer-trademarks">
			HL7®, and FHIR® are the registered trademarks of Health Level Seven
			International and their use of these trademarks does not constitute
			an endorsement by HL7.
		</p>
	</div>
</Footer>
