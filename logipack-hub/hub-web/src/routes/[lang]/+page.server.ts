import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ locals, cookies, params }) => {
	const authenticated = locals.session ?? cookies.get("lp_session");

	const lang = params.lang ?? "en";

	if (authenticated) {
		redirect(302, `/${lang}/app`);
	}

	return {
		appName: "LogiPack",
		loginUrl: `/${lang}/login`,
	};
};
