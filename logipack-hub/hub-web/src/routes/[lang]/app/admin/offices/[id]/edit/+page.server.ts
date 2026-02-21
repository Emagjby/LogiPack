import { getMockOfficeById, updateMockOffice } from "$lib/server/mockOffices";
import {
	hasOfficeFormErrors,
	parseOfficeFormData,
	validateOfficeForm,
} from "$lib/server/officeForm";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

const EMPTY_VALUES = {
	name: "",
	city: "",
	address: "",
};

export const load: PageServerLoad = async ({ params }) => {
	const office = getMockOfficeById(params.id);
	if (!office) {
		return {
			officeId: params.id,
			notFound: true as const,
			initialValues: EMPTY_VALUES,
		};
	}

	return {
		officeId: office.id,
		initialValues: {
			name: office.name,
			city: office.city,
			address: office.address,
		},
	};
};

export const actions: Actions = {
	default: async ({ request, params }) => {
		const values = parseOfficeFormData(await request.formData());
		const fieldErrors = validateOfficeForm(values);

		if (hasOfficeFormErrors(fieldErrors)) {
			return fail(400, { fieldErrors, values });
		}

		try {
			const updatedOffice = updateMockOffice(params.id, values);
			if (!updatedOffice) {
				return fail(404, {
					fieldErrors: {},
					submitError: "admin.offices.detail.not_found",
					values,
				});
			}
		} catch {
			return fail(500, {
				fieldErrors: {},
				submitError: "admin.offices.edit.submit_failed",
				values,
			});
		}

		redirect(303, `/${params.lang ?? "en"}/app/admin/offices/${params.id}`);
	},
};
