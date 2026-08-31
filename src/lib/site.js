// Constants shared by the client-side components. Keep this free of any
// content imports so it stays cheap to ship to the browser.

export const SITE_URL = 'https://fhir-rust.github.io';
export const SITE_NAME = 'fhir-rust';
export const SITE_TAGLINE = 'FHIR in Rust, stored as real relational tables';

// Repository URLs.
//
// The project merged into ONE monorepo — fhir-rust/fhir-rust — holding all
// four families (the model in fhir/, the six database ports, fhir-store,
// fhir-loco). That is the source tree's own `origin`; anonymous HTTP to it
// 404s today, which a private repository also does, so it is unverified
// rather than known-absent. Every rewritten source link resolves through
// REPOSITORY, so it all starts working the moment the repository is public —
// nothing else hard-codes it. (An earlier revision pointed at the pre-merge
// fhir-rust-crate / fhir-databases / fhir-store split.)
export const REPO_MONO = 'https://github.com/fhir-rust/fhir-rust';
export const REPO_SITE = 'https://github.com/fhir-rust/fhir-rust.github.io';

// The openEHR family lives in a separate workspace, and only its `openehr`
// crate is published. Paths below `openehr/` resolve into that repository;
// anything else in that family (openehr-store, openehr-sqlite, …) has no
// repository yet and falls back to the organization page, which at least
// exists.
export const REPO_OPENEHR = 'https://github.com/openehr-rust/openehr';
export const ORG_OPENEHR = 'https://github.com/openehr-rust';

/** The repository whose Markdown this site renders. */
export const REPOSITORY = REPO_MONO;

/** Themes vendored into static/themes/ by bin/sync-lily-themes.mjs. */
export const THEMES = [
	'light',
	'dark',
	'corporate',
	'business',
	'dim',
	'nord',
	'lofi',
	'night'
];

export const THEME_LABELS = {
	light: 'Light',
	dark: 'Dark',
	corporate: 'Corporate',
	business: 'Business',
	dim: 'Dim',
	nord: 'Nord',
	lofi: 'Lo-fi',
	night: 'Night'
};

/**
 * SharePicker destinations. The package ships no third-party endpoints by
 * design (each consumer's choice of network is editorial), so this site
 * supplies its own — technical/OSS-audience networks plus email.
 */
export const SHARE_TARGETS = [
	{
		id: 'mastodon',
		label: 'Mastodon',
		href: (url, title) =>
			`https://mastodonshare.com/?text=${encodeURIComponent(title)}&url=${encodeURIComponent(url)}`
	},
	{
		id: 'bluesky',
		label: 'Bluesky',
		href: (url, title) =>
			`https://bsky.app/intent/compose?text=${encodeURIComponent(`${title} ${url}`)}`
	},
	{
		id: 'linkedin',
		label: 'LinkedIn',
		href: (url) => `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(url)}`
	},
	{
		id: 'email',
		label: 'Email',
		href: (url, title) =>
			`mailto:?subject=${encodeURIComponent(title)}&body=${encodeURIComponent(url)}`,
		newTab: false
	}
];
