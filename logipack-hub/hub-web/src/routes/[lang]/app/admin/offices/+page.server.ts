import type { PageServerLoad } from "./$types";
import {
	filterMockOfficesByQuery,
	listMockOffices,
} from "$lib/server/mockOffices";

export const load: PageServerLoad = async ({ url }) => {
	const q = url.searchParams.get("q")?.trim() ?? "";

	try {
		return {
			offices: filterMockOfficesByQuery(listMockOffices(), q),
			q,
			loadError: false,
		};
	} catch {
		return {
			offices: [],
			q,
			loadError: true,
		};
	}
};
