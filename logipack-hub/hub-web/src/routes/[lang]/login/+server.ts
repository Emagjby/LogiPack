import type { RequestHandler } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import {
	AUTH0_DOMAIN,
	AUTH0_CLIENT_ID,
	AUTH0_CALLBACK_URL,
	AUTH0_AUDIENCE,
} from "$env/static/private";

const SUPPORTED = new Set(["en", "bg"]);

export const GET: RequestHandler = async ({ url, cookies }) => {
	const raw = url.searchParams.get("returnTo") ?? "/app";
	const returnTo = raw.startsWith("/") && !raw.startsWith("//") ? raw : "/app";

	const cookieLocale = cookies.get("lang");
	const locale =
		cookieLocale && SUPPORTED.has(cookieLocale) ? cookieLocale : "en";

	const authorize = new URL(`https://${AUTH0_DOMAIN}/authorize`);
	authorize.searchParams.set("response_type", "code");
	authorize.searchParams.set("client_id", AUTH0_CLIENT_ID);
	authorize.searchParams.set("redirect_uri", AUTH0_CALLBACK_URL);
	authorize.searchParams.set("ui_locales", locale);
	authorize.searchParams.set("audience", AUTH0_AUDIENCE);
	authorize.searchParams.set("scope", "openid profile email offline_access");
	authorize.searchParams.set("state", encodeURIComponent(returnTo));

	throw redirect(302, authorize.toString());
};
