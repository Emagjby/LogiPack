import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ locals, cookies }) => {
	const authenticated = locals.user ?? cookies.get("lp_session");

	if (authenticated) {
		redirect(302, "/app");
	}

	return {
		appName: "LogiPack",
		loginUrl: "/login",
	};
};
