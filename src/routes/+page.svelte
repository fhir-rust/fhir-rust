<script>
	import Card from '$lib/lily/components/Card.svelte';
	import SectionHeading from '$lib/lily/components/SectionHeading.svelte';
	import {
		REPO_CRATE,
		REPO_DATABASES,
		REPO_STORE,
		SITE_NAME,
		SITE_TAGLINE,
		SITE_URL
	} from '$lib/site.js';

	let { data } = $props();

	// The guides index is reached from the nav; the hub lists the readable ones.
	const guides = $derived(data.guides.filter((guide) => guide.route !== '/docs/guides/'));

	// The six ports and their conformance levels, as stated by the project
	// README. These are levels defined in spec/00-conformance.md: what has been
	// *verified for that port*, not what its code contains. The conformance
	// matrix breaks them down requirement by requirement and is the document to
	// trust — which is why every row below links to it rather than restating it.
	const ports = [
		{
			name: 'fhir-postgresql',
			engine: 'PostgreSQL 18',
			level: 'Reference',
			note: 'full store, full test suite'
		},
		{
			name: 'fhir-sqlite',
			engine: 'SQLite 3',
			level: 'Store',
			note: 'native, embeddable, no server'
		},
		{ name: 'fhir-mysql', engine: 'MySQL 8.4', level: 'Store', note: '' },
		{ name: 'fhir-mariadb', engine: 'MariaDB 11.4', level: 'Store', note: '' },
		{
			name: 'fhir-mssql',
			engine: 'SQL Server',
			level: 'Scaffold',
			note: 'DDL only, no store'
		},
		{
			name: 'fhir-oracle',
			engine: 'Oracle Database',
			level: 'Scaffold',
			note: 'DDL is still MySQL’s'
		}
	];

	const description =
		'Store FHIR resources in a SQL database as real relational tables — typed columns, child tables, foreign keys, check constraints — not JSON blobs. Get them back losslessly.';
</script>

<svelte:head>
	<title>{SITE_NAME} — {SITE_TAGLINE}</title>
	<meta name="description" content={description} />
	<link rel="canonical" href={`${SITE_URL}/`} />
	<meta property="og:title" content={`${SITE_NAME} — ${SITE_TAGLINE}`} />
	<meta property="og:description" content={description} />
	<meta property="og:type" content="website" />
	<meta property="og:url" content={`${SITE_URL}/`} />
</svelte:head>

<div class="home-hero">
	<h1>FHIR in Rust</h1>
	<p class="home-hero-lead">
		Store <a href="https://hl7.org/fhir/" rel="noopener noreferrer">FHIR</a> resources in a SQL
		database as <strong>real relational tables</strong> — typed columns, child tables, foreign keys,
		check constraints — not JSON blobs. Get them back losslessly.
	</p>
	<p class="home-hero-actions">
		<a class="button-primary" href="/docs/tutorial-01-getting-started/">Start the tutorial</a>
		<a class="button-secondary" href="/docs/choosing-an-engine/">Choose an engine</a>
	</p>
	<p class="home-hero-note">
		<strong>Pre-release.</strong> Each port is described at its own conformance level. The
		<a href="/conformance/">conformance matrix</a> is the status document to trust, and the
		<a href="/spec/audit/">audit findings</a> list what is currently broken, with evidence.
	</p>
</div>

<section class="home-section">
	<SectionHeading
		heading="Three projects"
		eyebrow="What is here"
		subtitle="One data model, one storage engine, one HTTP surface — deliberately separate."
	/>
	<div class="card-grid">
		<Card heading="fhir-databases" href={REPO_DATABASES} headingLevel={3} class="home-card">
			<p>
				Six FHIR-to-relational libraries, one specification, one engine. Schemas are generated from
				the FHIR specification; the pure-Rust core is identical across all six ports.
			</p>
			<p class="card-links">
				<a href="/overview/">Overview</a> · <a href="/conformance/">Conformance</a> ·
				<a href="/spec/">Specification</a>
			</p>
		</Card>

		<Card heading="fhir-rust-crate" href={REPO_CRATE} headingLevel={3} class="home-card">
			<p>
				The HL7 FHIR data model in Rust, plus the generator that produces it from the official
				specification JSON. Three releases are modelled: R5, R4 and R3 (STU3).
			</p>
			<p class="card-links">
				<a href={REPO_CRATE} rel="noopener noreferrer">Read the crate on GitHub</a>
			</p>
		</Card>

		<Card heading="fhir-store" href={REPO_STORE} headingLevel={3} class="home-card">
			<p>
				A FHIR RESTful API server — Rust, Axum and Loco — over the storage libraries. The split is
				the point: storage guarantees live in the library, HTTP lives here.
			</p>
			<p class="card-links">
				<a href={REPO_STORE} rel="noopener noreferrer">Read the server on GitHub</a>
			</p>
		</Card>
	</div>
</section>

<section class="home-section">
	<SectionHeading
		heading="Six ports"
		eyebrow="fhir-databases"
		subtitle="Conformance levels, not feature lists: what has been verified for that port."
	/>
	<table class="port-table">
		<thead>
			<tr>
				<th scope="col">Port</th>
				<th scope="col">Database</th>
				<th scope="col">Status</th>
			</tr>
		</thead>
		<tbody>
			{#each ports as port (port.name)}
				<tr>
					<th scope="row"><code>{port.name}</code></th>
					<td>{port.engine}</td>
					<td>
						<span class="level" data-level={port.level.toLowerCase()}>{port.level}</span>
						{#if port.note}<span class="level-note">{port.note}</span>{/if}
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<p class="section-footnote">
		The levels are defined in <a href="/spec/00-conformance/">Conformance</a>, and broken down
		requirement by requirement in the <a href="/conformance/">conformance matrix</a> — read that
		before choosing one.
	</p>
</section>

<section class="home-section">
	<SectionHeading
		heading="Guides and tutorials"
		eyebrow="Learn it"
		subtitle="The specification decides what must be true; these explain how it works."
	/>
	<ul class="link-list">
		{#each guides as guide (guide.route)}
			<li><a href={guide.route}>{guide.title}</a></li>
		{/each}
	</ul>
</section>

<section class="home-section">
	<SectionHeading
		heading="The specification"
		eyebrow="Implement or audit it"
		subtitle="One copy, shared by all six ports. Requirement ids are permanent."
	/>
	<ul class="link-list link-list-columns">
		{#each data.spec as section (section.route)}
			<li><a href={section.route}>{section.title}</a></li>
		{/each}
	</ul>
	<p class="section-footnote">
		Non-normative companions: <a href="/conformance/">conformance matrix</a> ·
		<a href="/spec/audit/">audit findings</a> ·
		<a href="/spec/locale-accent-folding/">locale and accent folding</a>.
	</p>
</section>
