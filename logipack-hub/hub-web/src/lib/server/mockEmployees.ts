export type MockEmployee = {
	id: string;
	user_id: string;
	user_display_name: string | null;
	full_name: string;
	email: string;
	office_id: string | null;
	office_ids: string[];
	created_at: string;
	updated_at: string;
};

export type CreateMockEmployeeInput = {
	email: string;
};

const SEED_EMPLOYEES: MockEmployee[] = [
	{
		id: "EMP-1001",
		user_id: "user-emil-ivanov",
		user_display_name: "Emil Ivanov",
		full_name: "Emil Ivanov",
		email: "emil.ivanov@logipack.dev",
		office_id: "c2f4c5c1-2d59-4f05-8e0c-c3ef5f0c1fd3",
		office_ids: ["c2f4c5c1-2d59-4f05-8e0c-c3ef5f0c1fd3"],
		created_at: "2025-12-18T09:20:00.000Z",
		updated_at: "2026-01-20T14:05:00.000Z",
	},
	{
		id: "EMP-1002",
		user_id: "user-maria-petrova",
		user_display_name: "Maria Petrova",
		full_name: "Maria Petrova",
		email: "maria.petrova@logipack.dev",
		office_id: "74f26d91-9a2a-4a2d-894e-9e3cf58ea6c7",
		office_ids: ["74f26d91-9a2a-4a2d-894e-9e3cf58ea6c7"],
		created_at: "2025-12-22T11:10:00.000Z",
		updated_at: "2026-01-22T10:25:00.000Z",
	},
	{
		id: "EMP-1003",
		user_id: "user-georgi-dimitrov",
		user_display_name: "Georgi Dimitrov",
		full_name: "Georgi Dimitrov",
		email: "georgi.dimitrov@logipack.dev",
		office_id: "15b3a4f0-50f4-4de8-b9e3-35d88c2c1d46",
		office_ids: ["15b3a4f0-50f4-4de8-b9e3-35d88c2c1d46"],
		created_at: "2026-01-05T08:45:00.000Z",
		updated_at: "2026-02-02T16:10:00.000Z",
	},
	{
		id: "EMP-1004",
		user_id: "user-radostina-stoyanova",
		user_display_name: null,
		full_name: "Radostina Stoyanova",
		email: "radostina.stoyanova@logipack.dev",
		office_id: "9f9a45f8-e2ce-4699-a00f-889b4d6dd1ca",
		office_ids: ["9f9a45f8-e2ce-4699-a00f-889b4d6dd1ca"],
		created_at: "2026-01-11T13:05:00.000Z",
		updated_at: "2026-02-07T12:15:00.000Z",
	},
	{
		id: "EMP-1005",
		user_id: "user-kaloyan-atanasov",
		user_display_name: "Kaloyan Atanasov",
		full_name: "Kaloyan Atanasov",
		email: "kaloyan.atanasov@logipack.dev",
		office_id: null,
		office_ids: [],
		created_at: "2026-01-17T07:30:00.000Z",
		updated_at: "2026-02-10T09:40:00.000Z",
	},
];

/**
 * Process-wide in-memory mock stores shared across requests.
 * State resets on server restart; this is intentional for local mock/testing flows.
 */
const createdEmployees = new Map<string, MockEmployee>();
const updatedEmployees = new Map<string, MockEmployee>();
const deletedEmployeeIds = new Set<string>();

function sanitizeEmailLocalPart(raw: string): string {
	const localPart = raw.split("@")[0] ?? raw;
	const normalized = localPart
		.trim()
		.toLowerCase()
		.replace(/[^a-z0-9._-]/g, ".")
		.replace(/\.+/g, ".")
		.replace(/^\.|\.$/g, "");
	return normalized || "employee";
}

function toEmployeeEmail(email: string): string {
	return email.trim().toLowerCase();
}

function shortStableHash(input: string): string {
	let hash = 2166136261;
	for (let i = 0; i < input.length; i++) {
		hash ^= input.charCodeAt(i);
		hash = Math.imul(hash, 16777619);
	}
	return (hash >>> 0).toString(36).slice(0, 6);
}

function toEmployeeUserId(email: string): string {
	const normalizedBase = sanitizeEmailLocalPart(email).replace(/[._]+/g, "-");
	const base = normalizedBase.startsWith("user-")
		? normalizedBase.slice(5)
		: normalizedBase;
	const suffix = shortStableHash(toEmployeeEmail(email));
	return `user-${base}-${suffix}`;
}

function toEmployeeDisplayName(email: string): string {
	const parts = sanitizeEmailLocalPart(email)
		.split(/[._-]+/)
		.filter(Boolean)
		.slice(0, 4);

	if (parts.length === 0) return "New Employee";

	return parts
		.map((part) => part.charAt(0).toUpperCase() + part.slice(1))
		.join(" ");
}

function createEmployeeId(): string {
	return typeof crypto !== "undefined" && "randomUUID" in crypto
		? `EMP-${crypto.randomUUID()}`
		: `EMP-${Date.now()}`;
}

function listSeedEmployees(): MockEmployee[] {
	return SEED_EMPLOYEES.filter((employee) => !deletedEmployeeIds.has(employee.id))
		.map((employee) => ({
			...employee,
			office_ids: [...employee.office_ids],
		}));
}

export function listMockEmployees(): MockEmployee[] {
	const employeesById = new Map(
		listSeedEmployees().map((employee) => [employee.id, employee]),
	);

	for (const employee of updatedEmployees.values()) {
		if (deletedEmployeeIds.has(employee.id)) continue;
		employeesById.set(employee.id, employee);
	}

	// Invariant: createdEmployees must override updatedEmployees (same rule updateMockEmployee-style helpers must follow).
	for (const employee of createdEmployees.values()) {
		if (deletedEmployeeIds.has(employee.id)) continue;
		employeesById.set(employee.id, employee);
	}

	return [...employeesById.values()]
		.map((employee) => ({
			...employee,
			office_ids: [...employee.office_ids],
		}))
		.sort((a, b) => {
			const aLabel = (a.user_display_name ?? a.full_name).toLowerCase();
			const bLabel = (b.user_display_name ?? b.full_name).toLowerCase();
			return aLabel.localeCompare(bLabel);
		});
}

export function getMockEmployeeById(id: string): MockEmployee | null {
	if (deletedEmployeeIds.has(id)) return null;

	const createdEmployee = createdEmployees.get(id);
	if (createdEmployee) {
		return {
			...createdEmployee,
			office_ids: [...createdEmployee.office_ids],
		};
	}

	const updatedEmployee = updatedEmployees.get(id);
	if (updatedEmployee) {
		return {
			...updatedEmployee,
			office_ids: [...updatedEmployee.office_ids],
		};
	}

	const seedEmployee = SEED_EMPLOYEES.find((employee) => employee.id === id);
	return seedEmployee
		? {
				...seedEmployee,
				office_ids: [...seedEmployee.office_ids],
			}
		: null;
}

export function deleteMockEmployee(id: string): boolean {
	if (createdEmployees.has(id)) {
		createdEmployees.delete(id);
		return true;
	}

	if (updatedEmployees.has(id)) {
		updatedEmployees.delete(id);
		deletedEmployeeIds.add(id);
		return true;
	}

	const existsInSeed = SEED_EMPLOYEES.some((employee) => employee.id === id);
	if (!existsInSeed) return false;

	deletedEmployeeIds.add(id);
	return true;
}

export function assignMockEmployeeOffice(
	id: string,
	officeId: string,
): MockEmployee | null {
	if (deletedEmployeeIds.has(id)) return null;
	const normalizedOfficeId = officeId.trim();
	if (!normalizedOfficeId) return null;

	const existing = getMockEmployeeById(id);
	if (!existing) return null;

	const updatedEmployee: MockEmployee = {
		...existing,
		office_id: normalizedOfficeId,
		office_ids: [normalizedOfficeId],
		updated_at: new Date().toISOString(),
	};

	const storeValue: MockEmployee = {
		...updatedEmployee,
		office_ids: [...updatedEmployee.office_ids],
	};

	if (createdEmployees.has(existing.id)) {
		createdEmployees.set(existing.id, storeValue);
		return {
			...storeValue,
			office_ids: [...storeValue.office_ids],
		};
	}

	updatedEmployees.set(existing.id, storeValue);
	return {
		...storeValue,
		office_ids: [...storeValue.office_ids],
	};
}

export function createMockEmployee(
	input: CreateMockEmployeeInput,
): { id: string } {
	const now = new Date().toISOString();
	const id = createEmployeeId();
	const email = toEmployeeEmail(input.email);
	const userId = toEmployeeUserId(email);
	const fullName = toEmployeeDisplayName(email);

	createdEmployees.set(id, {
		id,
		user_id: userId,
		user_display_name: fullName || null,
		full_name: fullName,
		email,
		office_id: null,
		office_ids: [],
		created_at: now,
		updated_at: now,
	});

	return { id };
}
