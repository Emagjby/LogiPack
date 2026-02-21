import { createShipment } from "$lib/server/mockShipments";
import {
	hasShipmentCreateErrors,
	parseShipmentCreateFormData,
	validateShipmentCreateForm,
} from "$lib/server/shipmentCreateForm";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions: Actions = {
	default: async ({ request, params }) => {
		const values = parseShipmentCreateFormData(await request.formData());
		const fieldErrors = validateShipmentCreateForm(values);

		if (hasShipmentCreateErrors(fieldErrors)) {
			return fail(400, { fieldErrors, values });
		}

		let shipmentId: string;
		try {
			const { id } = createShipment({
				clientId: values.client_id,
				currentOfficeId: values.current_office_id || null,
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
			`/${params.lang ?? "en"}/app/admin/shipments/${shipmentId}`,
		);
	},
};
