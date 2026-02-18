import type { PageServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ parent }) => {
	const { session } = await parent();
	const role = session?.role;

	if (role === "admin") {
		throw redirect(302, "/app/admin");
	}

	if (role === "employee") {
		throw redirect(302, "/app/employee");
	}

	throw redirect(302, "/app/no-access");
};
