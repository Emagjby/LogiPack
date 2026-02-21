export type ShipmentCreateFormValues = {
	client_id: string;
	current_office_id: string;
	notes: string;
};

export type ShipmentCreateFieldErrors = {
	client_id?: string;
};

export function parseShipmentCreateFormData(
	formData: FormData,
): ShipmentCreateFormValues {
	return {
		client_id: String(formData.get("client_id") ?? "").trim(),
		current_office_id: String(formData.get("current_office_id") ?? "").trim(),
		notes: String(formData.get("notes") ?? "").trim(),
	};
}

export function validateShipmentCreateForm(
	values: ShipmentCreateFormValues,
): ShipmentCreateFieldErrors {
	const fieldErrors: ShipmentCreateFieldErrors = {};
	if (!values.client_id) {
		fieldErrors.client_id = "shipment.form.client_required";
	}
	return fieldErrors;
}

export function hasShipmentCreateErrors(
	fieldErrors: ShipmentCreateFieldErrors,
): boolean {
	return Object.values(fieldErrors).some((value) => Boolean(value));
}
