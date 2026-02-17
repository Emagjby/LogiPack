import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals }) => {
	const s = locals.session as {
		role: string;
		email: string;
		name: string;
	} | null;

	return {
		role: s?.role ?? null,
		email: s?.email ?? null,
		name: s?.name ?? null,
	};
};
