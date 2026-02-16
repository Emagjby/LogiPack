import type { RequestHandler } from "./$types";
import { redirect, error } from "@sveltejs/kit";
import {
	AUTH0_DOMAIN,
	AUTH0_CLIENT_ID,
	AUTH0_CLIENT_SECRET,
	AUTH0_CALLBACK_URL,
	SESSION_SECRET,
} from "$env/static/private";
import { EncryptJWT } from "jose";

function safeRedirectPath(raw: string, fallback = "/app"): string {
	try {
		const decoded = decodeURIComponent(raw);
		if (decoded.startsWith("/") && !decoded.startsWith("//")) return decoded;
	} catch { }
	return fallback;
}

const enc = new TextEncoder();

// Derive a 256-bit (32-byte) key from SESSION_SECRET using Web Crypto API
async function deriveKey(secret: string): Promise<Uint8Array> {
	const data = enc.encode(secret);
	const hashBuffer = await crypto.subtle.digest("SHA-256", data);
	return new Uint8Array(hashBuffer);
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

	const expiresAt = Math.floor(Date.now() / 1000) + (tokens.expires_in ?? 3600);
	const encryptionKey = await deriveKey(SESSION_SECRET);

	const session = await new EncryptJWT({
		access_token: tokens.access_token,
		refresh_token: tokens.refresh_token,
		id_token: tokens.id_token,
		expires_at: expiresAt,
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
