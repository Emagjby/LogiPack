import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';


function apiBaseUrl() {
	const protocol = env.LOGIPACK_API_HTTPS === 'true' ? 'https' : 'http';
	const host = env.LOGIPACK_API_URL;
	const port = env.LOGIPACK_API_PORT;
	if (!host || !port) {
		throw new Error('Missing LOGIPACK_API_URL/LOGIPACK_API_PORT');
	}
	return `${protocol}://${host}:${port}`;
}

export async function apiFetch(event: RequestEvent, path: string, init: RequestInit = {}) {
	const devSecret = env.LOGIPACK_DEV_SECRET ?? '';

	const headers = new Headers(init.headers);
	headers.set('x-dev-secret', devSecret);

	const res = await event.fetch(`${apiBaseUrl()}${path}`, {
		...init,
		headers
	});

	return res;
}

export function requireDevUserSub(_: RequestEvent) {
	throw error(400, 'Dev actor must be sent from client');
}

export function getDevUserSub(_: RequestEvent) {
	return null;
}
