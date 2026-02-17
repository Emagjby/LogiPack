import type { RequestHandler } from "./$types";
import { redirect, error } from "@sveltejs/kit";
import {
	AUTH0_DOMAIN,
	AUTH0_CLIENT_ID,
	AUTH0_CLIENT_SECRET,
	AUTH0_CALLBACK_URL,
	SESSION_SECRET,
	HUB_API_BASE,
} from "$env/static/private";
import { EncryptJWT } from "jose";

function safeRedirectPath(raw: string, fallback = "/app"): string {
	try {
		const decoded = decodeURIComponent(raw);
		if (decoded.startsWith("/") && !decoded.startsWith("//")) return decoded;
	} catch { }
	return fallback;
}

function withQuery(path: string, params: Record<string, string>) {
	const u = new URL(path, "http://local");
	for (const [k, v] of Object.entries(params)) u.searchParams.set(k, v);
	return u.pathname + u.search;
}

const enc = new TextEncoder();

async function deriveKey(secret: string): Promise<Uint8Array> {
	const data = enc.encode(secret);
	const hashBuffer = await crypto.subtle.digest("SHA-256", data);
	return new Uint8Array(hashBuffer);
}

function decodeJwtPayload(jwt: string): Record<string, unknown> {
	const parts = jwt.split(".");
	if (parts.length !== 3) return {};
	const payload = parts[1]!;
	const json = Buffer.from(payload, "base64url").toString("utf-8");
	return JSON.parse(json);
}

async function getMgmtToken(signal: AbortSignal): Promise<string> {
	const res = await fetch(`https://${AUTH0_DOMAIN}/oauth/token`, {
		method: "POST",
		headers: { "content-type": "application/json" },
		signal,
		body: JSON.stringify({
			grant_type: "client_credentials",
			client_id: AUTH0_CLIENT_ID,
			client_secret: AUTH0_CLIENT_SECRET,
			audience: `https://${AUTH0_DOMAIN}/api/v2/`,
		}),
	});

	if (!res.ok) {
		const body = await res.text();
		console.error("MGMT token failed:", res.status, body);
		throw error(502, "Auth management token failed.");
	}

	const json = (await res.json()) as { access_token: string };
	return json.access_token;
}

async function getUserFromMgmt(
	userId: string,
	mgmtToken: string,
	signal: AbortSignal,
) {
	const res = await fetch(
		`https://${AUTH0_DOMAIN}/api/v2/users/${encodeURIComponent(userId)}`,
		{
			headers: { Authorization: `Bearer ${mgmtToken}` },
			signal,
		},
	);

	if (!res.ok) {
		const body = await res.text();
		console.error("MGMT user fetch failed:", res.status, body);
		throw error(502, "Failed to read updated user profile.");
	}

	return (await res.json()) as {
		name?: string;
		nickname?: string;
		email?: string;
	};
}

async function getMeRole(
	accessToken: string,
	signal: AbortSignal,
): Promise<string> {
	const res = await fetch(`${HUB_API_BASE}/me`, {
		headers: { Authorization: `Bearer ${accessToken}` },
		signal,
	});

	if (res.status === 404) {
		return "";
	}

	if (!res.ok) {
		const body = await res.text();
		console.error("/me failed:", res.status, body);
		throw error(502, "Failed to load user role.");
	}

	const json = (await res.json()) as { role: string };
	return json.role ?? "";
}

export const GET: RequestHandler = async ({ url, cookies }) => {
	const code = url.searchParams.get("code");
	const state = url.searchParams.get("state") ?? "/app";
	if (!code) throw error(400, "Missing ?code");

	const tokenRes = await fetch(`https://${AUTH0_DOMAIN}/oauth/token`, {
		method: "POST",
		headers: { "content-type": "application/json" },
		signal: AbortSignal.timeout(10000),
		body: JSON.stringify({
			grant_type: "authorization_code",
			client_id: AUTH0_CLIENT_ID,
			client_secret: AUTH0_CLIENT_SECRET,
			code,
			redirect_uri: AUTH0_CALLBACK_URL,
		}),
	});

	if (!tokenRes.ok) {
		const body = await tokenRes.text();
		console.error("Token exchange failed:", tokenRes.status, body);
		throw error(502, "Authentication failed, please try again.");
	}

	const tokens = (await tokenRes.json()) as {
		access_token: string;
		id_token?: string;
		refresh_token?: string;
		expires_in: number;
		token_type: string;
	};

	let name = "";
	let email = "";
	try {
		let sub = "";

		if (tokens.id_token) {
			const claims = decodeJwtPayload(tokens.id_token);
			sub = (claims.sub as string) ?? "";
			name = (claims.name as string) ?? (claims.nickname as string) ?? "";
			email = (claims.email as string) ?? "";
		}

		const looksBad =
			!name ||
			(!!name &&
				!!email &&
				name.trim().toLowerCase() === email.trim().toLowerCase());

		if (looksBad && sub) {
			const mgmtToken = await getMgmtToken(AbortSignal.timeout(10000));
			const u = await getUserFromMgmt(
				sub,
				mgmtToken,
				AbortSignal.timeout(10000),
			);
			name = u.name ?? u.nickname ?? name;
			email = u.email ?? email;
		}

		if (!name || !email) {
			const userinfoRes = await fetch(`https://${AUTH0_DOMAIN}/userinfo`, {
				headers: { Authorization: `Bearer ${tokens.access_token}` },
				signal: AbortSignal.timeout(5000),
			});
			if (userinfoRes.ok) {
				const info = (await userinfoRes.json()) as Record<string, unknown>;
				if (!name)
					name = (info.name as string) ?? (info.nickname as string) ?? "User";
				if (!email) email = (info.email as string) ?? "";
			}
		}

		const ensureRes = await fetch(`${HUB_API_BASE}/ensure-user`, {
			method: "POST",
			headers: {
				"content-type": "application/json",
				Authorization: `Bearer ${tokens.access_token}`,
			},
			signal: AbortSignal.timeout(10000),
			body: JSON.stringify({ name, email }),
		});

		if (!ensureRes.ok) {
			const bodyText = await ensureRes.text();
			console.error("ensure-user failed:", ensureRes.status, bodyText);

			if (ensureRes.status === 409) {
				// TODO: HANDLE THIS BETTER.
				throw redirect(
					303,
					withQuery("/", {
						err: "account_conflict",
						code: "email_already_linked",
					}),
				);
			}

			if (ensureRes.status === 400) {
				// TODO: HANDLE THIS BETTER.
				throw redirect(303, withQuery("/", { err: "invalid_profile" }));
			}

			// TODO: HANDLE THIS BETTER.
			throw error(502, "Failed to provision user account.");
		}
	} catch (e: unknown) {
		if (e && typeof e === "object" && "status" in e) throw e;
		console.error("ensure-user call failed:", e);
		throw error(502, "Failed to provision user account.");
	}

	let role = "";
	try {
		role = await getMeRole(tokens.access_token, AbortSignal.timeout(5000));
	} catch (e) {
		console.error("Failed to get user role:", e);
	}

	const expiresAt = Math.floor(Date.now() / 1000) + (tokens.expires_in ?? 3600);
	const encryptionKey = await deriveKey(SESSION_SECRET);

	const session = await new EncryptJWT({
		access_token: tokens.access_token,
		refresh_token: tokens.refresh_token,
		id_token: tokens.id_token,
		expires_at: expiresAt,
		role,
		name,
		email,
	})
		.setProtectedHeader({ alg: "dir", enc: "A256GCM" })
		.setIssuedAt()
		.setExpirationTime("7d")
		.encrypt(encryptionKey);

	cookies.set("lp_session", session, {
		path: "/",
		httpOnly: true,
		sameSite: "lax",
		secure: url.protocol === "https:",
		maxAge: 60 * 60 * 24 * 7,
	});

	throw redirect(302, safeRedirectPath(state));
};
