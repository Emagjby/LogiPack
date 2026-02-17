export type LpSession = {
	access_token: string;
	refresh_token?: string;
	id_token?: string;
	expires_at: number;
	role: string; // normalize to "" if missing
	name: string; // normalize to "" if missing
	email: string; // normalize to "" if missing
};

function isObject(v: unknown): v is Record<string, unknown> {
	return !!v && typeof v === "object";
}

export function parseSession(payload: unknown): LpSession | null {
	if (!isObject(payload)) return null;

	const access_token = payload.access_token;
	const expires_at = payload.expires_at;

	if (typeof access_token !== "string" || !access_token) return null;
	if (typeof expires_at !== "number" || !Number.isFinite(expires_at))
		return null;

	return {
		access_token,
		refresh_token:
			typeof payload.refresh_token === "string"
				? payload.refresh_token
				: undefined,
		id_token:
			typeof payload.id_token === "string" ? payload.id_token : undefined,
		expires_at,
		role: typeof payload.role === "string" ? payload.role : "",
		name: typeof payload.name === "string" ? payload.name : "",
		email: typeof payload.email === "string" ? payload.email : "",
	};
}
