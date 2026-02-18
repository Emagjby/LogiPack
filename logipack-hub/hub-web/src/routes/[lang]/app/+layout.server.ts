import type { LayoutServerLoad } from "./$types";
import type { Me, Role } from "$lib/types/console";
import { redirect } from "@sveltejs/kit";

const VALID_ROLES: readonly Role[] = ["admin", "employee"];

export const load: LayoutServerLoad = async ({ locals, url }) => {
	if (!locals.session) {
		throw redirect(302, `/?redirect=${url.pathname}${url.search}`);
	}

	const session = (locals.session ?? null) as Me | null;

	const hasValidRole = session?.role && VALID_ROLES.includes(session.role);
	const isNoAccessRoute = /\/app\/no-access(\/|$)/.test(url.pathname);

	if (!hasValidRole && !isNoAccessRoute) {
		throw redirect(302, `/${url.pathname.split("/")[1] ?? "en"}/app/no-access`);
	}

	return {
		session,
		pathname: url.pathname,
	};
};
