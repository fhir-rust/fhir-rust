import { routes } from '$lib/docs.js';
import { SITE_URL } from '$lib/site.js';

export const prerender = true;

export function GET() {
	const urls = ['/', ...routes().map(({ route }) => route)];
	const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls.map((url) => `\t<url><loc>${SITE_URL}${url}</loc></url>`).join('\n')}
</urlset>
`;
	return new Response(body, { headers: { 'content-type': 'application/xml' } });
}
