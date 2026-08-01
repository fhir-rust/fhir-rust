// Constants shared by the client-side components. Keep this free of any
// content imports so it stays cheap to ship to the browser.

export const SITE_URL = 'https://fhir-rust.github.io';
export const SITE_NAME = 'fhir-rust';
export const SITE_TAGLINE = 'FHIR in Rust, stored as real relational tables';

// Repository URLs.
//
// Verified against the GitHub API on 2026-08-01: REPO_CRATE and REPO_SITE
// exist. The old fhir-rust-crate/fhir-rust-crate now 301s to the fhir-rust
// organization, so the canonical URL is used here.
export const REPO_CRATE = 'https://github.com/fhir-rust/fhir-rust-crate';
export const REPO_SITE = 'https://github.com/fhir-rust/fhir-rust.github.io';

// NOT YET PUBLISHED. These two 404 today: neither sibling checkout has a git
// remote, and neither repository exists in the fhir-rust organization. The
// names assume they will be published alongside the crate, which is where
// fhir-rust-crate landed. Every rewritten source link and every "Edit this
// page on GitHub" resolves through REPO_DATABASES, so it starts working the
// moment that repository exists — and nothing else hard-codes it.
export const REPO_DATABASES = 'https://github.com/fhir-rust/fhir-databases';
export const REPO_STORE = 'https://github.com/fhir-rust/fhir-store';

// The openEHR family lives in a separate workspace, and only its `openehr`
// crate is published. Paths below `openehr/` resolve into that repository;
// anything else in that family (openehr-store, openehr-sqlite, …) has no
// repository yet and falls back to the organization page, which at least
// exists.
export const REPO_OPENEHR = 'https://github.com/openehr-rust/openehr';
export const ORG_OPENEHR = 'https://github.com/openehr-rust';

/** The repository whose Markdown this site renders. */
export const REPOSITORY = REPO_DATABASES;

/** Themes vendored into static/themes/ by bin/sync-lily.mjs. */
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
