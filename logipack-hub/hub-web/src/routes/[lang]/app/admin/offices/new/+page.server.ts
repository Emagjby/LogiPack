import { createMockOffice } from "$lib/server/mockOffices";
import {
	hasOfficeFormErrors,
	parseOfficeFormData,
	validateOfficeForm,
} from "$lib/server/officeForm";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions: Actions = {
	default: async ({ request, params }) => {
		const values = parseOfficeFormData(await request.formData());
		const fieldErrors = validateOfficeForm(values);

		if (hasOfficeFormErrors(fieldErrors)) {
			return fail(400, { fieldErrors, values });
		}

		let officeId: string;
		try {
			const { id } = createMockOffice(values);
			officeId = id;
		} catch {
			return fail(500, {
				fieldErrors: {},
				submitError: "admin.offices.new.submit_failed",
				values,
			});
		}

		throw redirect(303, `/${params.lang ?? "en"}/app/admin/offices/${officeId}`);
	},
};
