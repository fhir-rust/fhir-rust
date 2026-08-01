// Constants shared by the client-side components. Keep this free of any
// content imports so it stays cheap to ship to the browser.

export const SITE_URL = 'https://fhir-rust.github.io';
export const SITE_NAME = 'fhir-rust';
export const SITE_TAGLINE = 'FHIR in Rust, stored as real relational tables';

// Repository URLs.
//
// Only REPO_CRATE is verified: fhir-rust-crate is the one sibling checkout
// with a git remote (git@github.com:fhir-rust-crate/fhir-rust-crate.git).
// fhir-databases and fhir-store have no remote yet, so the URLs below assume
// they will be published under the same organization that owns this site.
// If they land elsewhere, change them here — nothing else hard-codes them.
export const REPO_DATABASES = 'https://github.com/fhir-rust/fhir-databases';
export const REPO_CRATE = 'https://github.com/fhir-rust-crate/fhir-rust-crate';
export const REPO_STORE = 'https://github.com/fhir-rust/fhir-store';
export const REPO_SITE = 'https://github.com/fhir-rust/fhir-rust.github.io';

// The openEHR family lives in a separate workspace. The documents synced into
// content/ mention it, so unresolved openehr* links need somewhere to go.
export const REPO_OPENEHR = 'https://github.com/openehr-rust/openehr-databases';

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
