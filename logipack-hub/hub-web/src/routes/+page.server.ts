import type { PageServerLoad } from "./$types";
import { env } from "$env/dynamic/private";

export const load: PageServerLoad = async ({ fetch }) => {
	const protocol = env.LOGIPACK_API_HTTPS === "true" ? "https" : "http";
	const apiBaseUrl = `${protocol}://${env.LOGIPACK_API_URL}:${env.LOGIPACK_API_PORT}`;

	try {
		const res = await fetch(`${apiBaseUrl}/health`, {
			method: "GET",
			headers: {
				"x-dev-secret": env.LOGIPACK_DEV_SECRET || "",
			},
		});
		console.log("Used dev secret:", env.LOGIPACK_DEV_SECRET);
		if (!res.ok) {
			throw new Error(`API responded with status ${res.status}`);
		}
		const text = await res.text();

		return {
			api: {
				ok: res.ok,
				status: res.status,
				text,
			},
		};
	} catch (err) {
		return {
			api: {
				ok: false,
				status: 0,
				text: err instanceof Error ? err.message : String(err),
			},
		};
	}
};
