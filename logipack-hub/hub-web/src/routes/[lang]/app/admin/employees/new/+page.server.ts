import { createMockEmployee } from "$lib/server/mockEmployees";
import {
	hasEmployeeFormErrors,
	parseEmployeeFormData,
	validateEmployeeForm,
} from "$lib/server/employeeForm";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

const EMPTY_VALUES = {
	email: "",
};

export const load: PageServerLoad = async () => {
	return {
		initialValues: EMPTY_VALUES,
	};
};

export const actions: Actions = {
	default: async ({ request, params }) => {
		const values = parseEmployeeFormData(await request.formData());
		const fieldErrors = validateEmployeeForm(values);

		if (hasEmployeeFormErrors(fieldErrors)) {
			return fail(400, { fieldErrors, values });
		}

		let employeeId: string;
		try {
			const { id } = createMockEmployee(values);
			employeeId = id;
		} catch {
			return fail(500, {
				fieldErrors: {},
				submitError: "admin.employees.new.submit_failed",
				values,
			});
		}

		throw redirect(
			303,
			`/${params.lang ?? "en"}/app/admin/employees/${employeeId}`,
		);
	},
};
