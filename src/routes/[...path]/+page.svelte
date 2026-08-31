<script>
	import {
		ArticleLayout,
		BreadcrumbNav,
		BreadcrumbList,
		BreadcrumbListItem,
		ContentsNav,
		ContentsList,
		ContentsListItem,
		PaginationNav,
		PaginationList,
		PaginationListItem
	} from 'lily-design-system-svelte-headless';
	import { REPOSITORY, SITE_URL } from '$lib/site.js';

	let { data } = $props();

	const doc = $derived(data.doc);
	const url = $derived(`${SITE_URL}${doc.route}`);
	// content/ flattens the workspace, so the source path is the content path.
	const source = $derived(`${REPOSITORY}/blob/main/${doc.file}`);
	// Where the breadcrumb's middle rung points, per sequence.
	const parent = $derived(
		doc.section === 'Specification'
			? { label: 'Specification', href: '/spec/' }
			: doc.section === 'Guides'
				? { label: 'Guides', href: '/docs/guides/' }
				: { label: 'Documentation', href: '/docs/' }
	);
	// A page that *is* its own parent gets no self-referential rung.
	const showParent = $derived(parent.href !== doc.route);
</script>

<svelte:head>
	<title>{data.title}</title>
	<meta name="description" content={doc.summary} />
	<link rel="canonical" href={url} />
	<meta property="og:title" content={doc.title} />
	<meta property="og:description" content={doc.summary} />
	<meta property="og:type" content="article" />
	<meta property="og:url" content={url} />
</svelte:head>

<BreadcrumbNav label="Breadcrumb" class="doc-breadcrumb">
	<BreadcrumbList>
		<BreadcrumbListItem><a href="/">Home</a></BreadcrumbListItem>
		{#if showParent}
			<BreadcrumbListItem><a href={parent.href}>{parent.label}</a></BreadcrumbListItem>
		{/if}
		<BreadcrumbListItem current>{doc.title}</BreadcrumbListItem>
	</BreadcrumbList>
</BreadcrumbNav>

{#if doc.headings.length > 2}
	<ContentsNav label="On this page" class="doc-contents">
		<h2>On this page</h2>
		<ContentsList>
			{#each doc.headings as heading (heading.id)}
				<ContentsListItem data-depth={heading.depth}>
					<a href={`#${heading.id}`}>{heading.text}</a>
				</ContentsListItem>
			{/each}
		</ContentsList>
	</ContentsNav>
{/if}

<ArticleLayout label={doc.title} class="prose">
	{@html doc.html}
</ArticleLayout>

{#if doc.previous || doc.next}
	<PaginationNav label="Document navigation" class="doc-pagination">
		<PaginationList>
			{#if doc.previous}
				<PaginationListItem>
					<a href={doc.previous.route} rel="prev">
						<span class="direction">Previous</span>
						<span class="title">{doc.previous.title}</span>
					</a>
				</PaginationListItem>
			{/if}
			{#if doc.next}
				<PaginationListItem>
					<a href={doc.next.route} rel="next">
						<span class="direction">Next</span>
						<span class="title">{doc.next.title}</span>
					</a>
				</PaginationListItem>
			{/if}
		</PaginationList>
	</PaginationNav>
{/if}

<p class="doc-source">
	<a href={source} rel="noopener noreferrer">Edit this page on GitHub</a> — the repository is the
	source of truth; this site renders it.
</p>
