import { createShipment } from "$lib/server/mockShipments";
import { resolveEmployeeOffice } from "$lib/server/employeeOffice";
import {
	hasShipmentCreateErrors,
	parseShipmentCreateFormData,
	validateShipmentCreateForm,
} from "$lib/server/shipmentCreateForm";
import { error, fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ parent }) => {
	const { session } = await parent();

	if (session?.role === "admin") {
		throw error(403, "error.details.employee_only");
	}

	const office = resolveEmployeeOffice(session);
	return {
		office: {
			assignedId: office.id ?? "",
			label: office.name ?? office.id ?? null,
			isAvailable: Boolean(office.id),
			isLoading: false,
		},
	};
};

export const actions: Actions = {
	default: async ({ locals, params, request }) => {
		const session = (locals.session ?? null) as { role?: string } | null;
		if (session?.role === "admin") {
			throw error(403, "error.details.employee_only");
		}

		const office = resolveEmployeeOffice(session);

		const values = parseShipmentCreateFormData(await request.formData());
		values.current_office_id = office.id ?? "";

		const fieldErrors = validateShipmentCreateForm(values);
		if (hasShipmentCreateErrors(fieldErrors)) {
			return fail(400, { fieldErrors, values });
		}
		if (!office.id) {
			return fail(422, {
				fieldErrors: {},
				submitError: "employee.shipments.new.office_required",
				values,
			});
		}

		let shipmentId: string;
		try {
			const { id } = createShipment({
				clientId: values.client_id,
				currentOfficeId: office.id,
				notes: values.notes || null,
			});
			shipmentId = id;
		} catch {
			return fail(500, {
				fieldErrors: {},
				submitError: "admin.shipments.new.submit_failed",
				values,
			});
		}

		throw redirect(
			303,
			`/${params.lang ?? "en"}/app/employee/shipments/${shipmentId}`,
		);
	},
};
