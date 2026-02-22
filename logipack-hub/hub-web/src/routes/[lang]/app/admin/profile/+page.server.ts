import type { PageServerLoad } from "./$types";
import { error } from "@sveltejs/kit";
import { AUTH0_CLIENT_ID, AUTH0_DOMAIN } from "$env/static/private";
import { createRemoteJWKSet, jwtVerify } from "jose";

type JwtClaims = Record<string, unknown>;

const logoutRouteModules = import.meta.glob("/src/routes/logout/+server.ts");
const hasLogoutRoute = Object.keys(logoutRouteModules).length > 0;
const auth0Issuer = `https://${AUTH0_DOMAIN}/`;
const auth0Jwks = createRemoteJWKSet(
	new URL(`${auth0Issuer}.well-known/jwks.json`),
);

function isObject(value: unknown): value is Record<string, unknown> {
	return !!value && typeof value === "object" && !Array.isArray(value);
}

function toNonEmptyString(value: unknown): string | null {
	if (typeof value !== "string") return null;
	const trimmed = value.trim();
	return trimmed.length > 0 ? trimmed : null;
}

function toStringArray(value: unknown): string[] {
	if (!Array.isArray(value)) return [];
	return value
		.map((entry) => toNonEmptyString(entry))
		.filter((entry): entry is string => !!entry);
}

function getFirstString(
	source: Record<string, unknown>,
	keys: string[],
): string | null {
	for (const key of keys) {
		const value = toNonEmptyString(source[key]);
		if (value) return value;
	}
	return null;
}

function uniqueStrings(values: string[]): string[] {
	return [...new Set(values)];
}

async function getJwtClaims(
	rawSession: Record<string, unknown>,
): Promise<JwtClaims> {
	const idToken = getFirstString(rawSession, ["id_token", "idToken"]);
	if (!idToken) return {};
	try {
		const { payload } = await jwtVerify(idToken, auth0Jwks, {
			issuer: auth0Issuer,
			audience: AUTH0_CLIENT_ID,
			algorithms: ["RS256"],
		});
		return payload as JwtClaims;
	} catch {
		return {};
	}
}

export const load: PageServerLoad = async ({ parent }) => {
	const { session, pathname } = await parent();

	if (session?.role !== "admin") {
		throw error(403, "error.details.admin_only");
	}

	const rawSession: Record<string, unknown> = isObject(session) ? session : {};
	const claims = await getJwtClaims(rawSession);

	const name =
		getFirstString(rawSession, ["name"]) ??
		getFirstString(claims, ["name", "preferred_username", "nickname"]);

	const userId =
		getFirstString(rawSession, ["user_id", "userId", "sub", "id"]) ??
		getFirstString(claims, ["sub", "user_id", "userId"]);

	const roles = uniqueStrings(
		[
			...toStringArray(rawSession["roles"]),
			...toStringArray(claims["roles"]),
			...toStringArray(claims["https://logipack/roles"]),
			...(toNonEmptyString(rawSession["role"])
				? [rawSession["role"] as string]
				: []),
		],
	);

	return {
		pathname,
		hasLogoutRoute,
		profile: {
			name,
			userId,
			roles,
		},
	};
};
