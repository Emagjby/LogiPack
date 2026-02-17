import type { LayoutServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";

export const load: LayoutServerLoad = async ({ locals, params }) => {
	const s = locals.session as {
		role: string;
		email: string;
		name: string;
	} | null;

	const lang = params.lang ?? "en";

	if (!s) throw redirect(302, `/${lang}/login`);

	return {
		role: s.role,
		email: s.email,
		name: s.name,
	};
};
