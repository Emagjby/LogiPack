import { createShipment } from "$lib/server/mockShipments";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

type NewShipmentFormValues = {
	client_id: string;
	current_office_id: string;
	notes: string;
};

type NewShipmentFieldErrors = {
	client_id?: string;
};

export const actions: Actions = {
	default: async ({ request, params }) => {
		const formData = await request.formData();
		const values: NewShipmentFormValues = {
			client_id: String(formData.get("client_id") ?? "").trim(),
			current_office_id: String(formData.get("current_office_id") ?? "").trim(),
			notes: String(formData.get("notes") ?? "").trim(),
		};

		const fieldErrors: NewShipmentFieldErrors = {};
		if (!values.client_id) {
			fieldErrors.client_id = "admin.shipments.new.client_required";
		}

		if (fieldErrors.client_id) {
			return fail(400, { fieldErrors, values });
		}

		try {
			const { id } = createShipment({
				clientId: values.client_id,
				currentOfficeId: values.current_office_id || null,
				notes: values.notes || null,
			});

			throw redirect(303, `/${params.lang ?? "en"}/app/admin/shipments/${id}`);
		} catch {
			return fail(500, {
				fieldErrors: { client_id: undefined },
				values,
			});
		}
	},
};
