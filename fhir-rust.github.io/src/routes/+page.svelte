<script>
	import { Card, SectionHeading } from 'lily-design-system-svelte-headless';
	import { REPOSITORY, SITE_URL } from '$lib/site.js';

	let { data } = $props();

	// The guides index is reached from the nav; the hub lists the readable ones.
	const guides = $derived(data.guides.filter((guide) => guide.route !== '/docs/guides/'));

	// The six ports and their conformance levels, as stated by the project
	// README (revised 2026-08-06 after the comprehensive audit brought both
	// former scaffolds to Store level). These are levels defined in
	// spec/databases/00-conformance.md: what has been *verified for that
	// port*, not what its code contains. The conformance matrix breaks them
	// down requirement by requirement and is the document to trust — which is
	// why every row below links to it rather than restating it.
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
			level: 'Store',
			note: 'live-verified; a standing TLS advisory risk (F-67)'
		},
		{
			name: 'fhir-oracle',
			engine: 'Oracle Database',
			level: 'Store',
			note: 'live-verified; no upgrade path yet, snapshot reads open'
		}
	];

	const description =
		'FHIR in Rust: the complete R2–R6 data model as typed, serde-serializable Rust, and six database libraries that store resources as real relational tables — not JSON blobs — and give them back losslessly.';
</script>

<svelte:head>
	<title>{data.title}</title>
	<meta name="description" content={description} />
	<link rel="canonical" href={`${SITE_URL}/`} />
	<meta property="og:title" content={data.title} />
	<meta property="og:description" content={description} />
	<meta property="og:type" content="website" />
	<meta property="og:url" content={`${SITE_URL}/`} />
</svelte:head>

<div class="home-hero">
	<h1>FHIR in Rust</h1>
	<p class="home-hero-lead">
		Model <a href="https://hl7.org/fhir/" rel="noopener noreferrer">FHIR</a> in typed Rust —
		five releases, R2 through R6 — and store it in a SQL database as
		<strong>real relational tables</strong> — typed columns, child tables, foreign keys, check
		constraints — not JSON blobs. Get it back losslessly.
	</p>
	<p class="home-hero-actions">
		<a class="button-primary" href="/docs/tutorial-01-getting-started/">Start the tutorial</a>
		<a class="button-secondary" href="/examples/">Run the examples</a>
	</p>
	<p class="home-hero-note">
		<strong>Pre-release.</strong> Each database port is described at its own conformance level.
		The <a href="/conformance/">conformance matrix</a> is the status document to trust, and the
		<a href="/spec/audit/">audit findings</a> list every known divergence, with evidence.
	</p>
</div>

<section class="home-section">
	<SectionHeading
		heading="Four families, one repository"
		eyebrow="What is here"
		subtitle="A data model, six storage libraries, a shared persistence core, and an HTTP surface — deliberately separate, composing in one direction."
	/>
	<div class="card-grid">
		<Card heading="fhir — the model" href="/model/" headingLevel={3} class="home-card">
			<p>
				The complete HL7 FHIR data model as Rust types — five releases (R2–R6) as separate
				crates, generated from the official specification, with validation, builders, choice
				enums, an XML bridge, a REST client, and cross-release conversion with a loss report.
			</p>
			<p class="card-links">
				<a href="/model/">Overview</a> · <a href="/model/spec/">Specification</a> ·
				<a href="/examples/">Examples</a>
			</p>
		</Card>

		<Card
			heading="fhir-&lt;engine&gt; — the databases"
			href="/spec/"
			headingLevel={3}
			class="home-card"
		>
			<p>
				Six FHIR-to-relational libraries — PostgreSQL, SQLite, MySQL, MariaDB, SQL Server,
				Oracle — one specification, one shared pure-Rust core, identical across all six and
				gated in CI. Schemas are generated from the FHIR specification packages.
			</p>
			<p class="card-links">
				<a href="/overview/">Overview</a> · <a href="/conformance/">Conformance</a> ·
				<a href="/spec/">Specification</a>
			</p>
		</Card>

		<Card
			heading="fhir-store — the persistence core"
			href="/store/"
			headingLevel={3}
			class="home-card"
		>
			<p>
				The engine-agnostic half of persistence, shared by every port: the tamper-evident audit
				chain (SHA-256 + SHA3-256, optional HMAC), attribution, and the result types. No driver,
				no socket, no HTTP.
			</p>
			<p class="card-links">
				<a href="/store/">Overview</a> ·
				<a href="/spec/12-trust-principal-and-audit/">Trust &amp; audit spec</a>
			</p>
		</Card>

		<Card heading="fhir-loco — the server" href="/server/" headingLevel={3} class="home-card">
			<p>
				A FHIR RESTful API — Rust, Loco, Axum — mounted over a store. CRUD, vread, history,
				search, CapabilityStatement, PASETO authentication. The split is the point: storage
				guarantees live in the libraries, HTTP lives here.
			</p>
			<p class="card-links">
				<a href="/server/">Overview</a> · <a href="/server/spec/">Specification</a>
			</p>
		</Card>
	</div>
</section>

<section class="home-section">
	<SectionHeading
		heading="Six ports"
		eyebrow="The databases"
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
		heading="Examples"
		eyebrow="Run it"
		subtitle="Complete programs from the model crate — each one a tutorial in its header comment."
	/>
	<ul class="link-list link-list-columns">
		{#each data.examples as example (example.route)}
			<li><a href={example.route}>{example.title}</a></li>
		{/each}
	</ul>
	<p class="section-footnote">
		Run one from a checkout with <code>cargo run --example &lt;name&gt;</code> — see the
		<a href="/examples/">examples index</a>.
	</p>
</section>

<section class="home-section">
	<SectionHeading
		heading="The specifications"
		eyebrow="Implement or audit it"
		subtitle="Four families, four bodies of requirements. Requirement ids are permanent."
	/>
	<div class="card-grid">
		<Card heading="Database core" href="/spec/" headingLevel={3} class="home-card">
			<ul class="link-list">
				{#each data.spec as section (section.route)}
					<li><a href={section.route}>{section.title}</a></li>
				{/each}
			</ul>
		</Card>
		<Card heading="Model" href="/model/spec/" headingLevel={3} class="home-card">
			<ul class="link-list">
				{#each data.model as section (section.route)}
					<li><a href={section.route}>{section.title}</a></li>
				{/each}
			</ul>
		</Card>
	</div>
	<p class="section-footnote">
		The <a href="/specs/">specification root</a> says which spec governs which code and how the
		requirement-id namespaces relate. Non-normative companions:
		<a href="/conformance/">conformance matrix</a> · <a href="/spec/audit/">audit findings</a> ·
		<a href="/spec/locale-accent-folding/">locale and accent folding</a> ·
		<a href="/specs/publishing/">publishing readiness</a>. The
		<a href="/server/spec/">server specification</a> covers the HTTP surface.
	</p>
</section>

<section class="home-section">
	<SectionHeading
		heading="On crates.io, and in one repository"
		eyebrow="Use it"
		subtitle="The model family is published; the database ports are pre-release."
	/>
	<p>
		The model is <code>fhir = "3"</code> — five releases behind cargo features, <code>r5</code> on
		by default. The database ports and the server are pre-release in
		<a href={REPOSITORY}>the fhir-rust repository</a>, each at the conformance level the matrix
		states.
	</p>
</section>
