import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const apiBaseUrl = process.env.LOGIPACK_API_URL ?? 'http://127.0.0.1:3000';

	try {
		const res = await fetch(`${apiBaseUrl}/health`);
		const text = await res.text();

		return {
			api: {
				ok: res.ok,
				status: res.status,
				text
			}
		};
	} catch (err) {
		return {
			api: {
				ok: false,
				status: 0,
				text: err instanceof Error ? err.message : String(err)
			}
		};
	}
};
