import type { PageServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ parent, url }) => {
	const { session } = await parent();
	const lang = url.pathname.split("/")[1] || "en";

	if (session?.role === "admin") {
		throw redirect(302, `/${lang}/app/admin`);
	}

	if (session?.role === "employee") {
		throw redirect(302, `/${lang}/app/employee`);
	}
};
