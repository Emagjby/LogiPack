import type { RequestHandler } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import {
	AUTH0_DOMAIN,
	AUTH0_CLIENT_ID,
	AUTH0_LOGOUT_URL,
} from "$env/static/private";

export const GET: RequestHandler = async ({ cookies }) => {
	cookies.delete("lp_session", { path: "/" });

	const u = new URL(`https://${AUTH0_DOMAIN}/v2/logout`);
	u.searchParams.set("client_id", AUTH0_CLIENT_ID);
	u.searchParams.set("returnTo", AUTH0_LOGOUT_URL);

	throw redirect(302, u.toString());
};
