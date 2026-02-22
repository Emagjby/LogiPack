export type EmployeeFormValues = {
	email: string;
};

export type EmployeeFieldErrors = {
	email?: string;
};

export function parseEmployeeFormData(formData: FormData): EmployeeFormValues {
	return {
		email: String(formData.get("email") ?? "").trim().toLowerCase(),
	};
}

export function validateEmployeeForm(
	values: EmployeeFormValues,
): EmployeeFieldErrors {
	const fieldErrors: EmployeeFieldErrors = {};
	const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

	if (!values.email) {
		fieldErrors.email = "employee.form.email_required";
	} else if (!emailPattern.test(values.email)) {
		fieldErrors.email = "employee.form.email_invalid";
	}

	return fieldErrors;
}

export function hasEmployeeFormErrors(fieldErrors: EmployeeFieldErrors): boolean {
	return Object.values(fieldErrors).some((value) => Boolean(value));
}
