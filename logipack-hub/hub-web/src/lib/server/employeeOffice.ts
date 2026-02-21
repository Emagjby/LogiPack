import { decodeJwt } from "jose";

type JwtClaims = Record<string, unknown>;

function isObject(value: unknown): value is Record<string, unknown> {
	return !!value && typeof value === "object" && !Array.isArray(value);
}

function toNonEmptyString(value: unknown): string | null {
	if (typeof value !== "string") return null;
	const trimmed = value.trim();
	return trimmed.length > 0 ? trimmed : null;
}

function getFirstString(
	source: Record<string, unknown>,
	keys: string[],
): string | null {
	for (const key of keys) {
		const value = toNonEmptyString(source[key]);
		if (value) return value;
	}
	return null;
}

function toStringArray(value: unknown): string[] {
	if (!Array.isArray(value)) return [];
	return value
		.map((entry) => toNonEmptyString(entry))
		.filter((entry): entry is string => !!entry);
}

function getJwtClaims(rawSession: Record<string, unknown>): JwtClaims {
	const idToken = getFirstString(rawSession, ["id_token", "idToken"]);
	if (!idToken) return {};
	try {
		return decodeJwt(idToken) as JwtClaims;
	} catch {
		return {};
	}
}

export function resolveEmployeeOffice(
	session: unknown,
): { id: string | null; name: string | null } {
	const rawSession: Record<string, unknown> = isObject(session) ? session : {};
	const claims = getJwtClaims(rawSession);

	let currentOfficeId =
		getFirstString(rawSession, ["current_office_id", "currentOfficeId"]) ??
		getFirstString(claims, ["current_office_id", "currentOfficeId"]) ??
		getFirstString(rawSession, ["office_id", "officeId"]) ??
		getFirstString(claims, ["office_id", "officeId"]);

	let currentOfficeName =
		getFirstString(rawSession, ["current_office_name", "currentOfficeName"]) ??
		getFirstString(claims, ["current_office_name", "currentOfficeName"]);

	const currentOfficeObject =
		(isObject(rawSession["current_office"]) && rawSession["current_office"]) ||
		(isObject(rawSession["currentOffice"]) && rawSession["currentOffice"]);
	if (isObject(currentOfficeObject)) {
		currentOfficeId =
			currentOfficeId ??
			getFirstString(currentOfficeObject, ["id", "office_id", "officeId"]);
		currentOfficeName =
			currentOfficeName ??
			getFirstString(currentOfficeObject, ["name", "office_name", "officeName"]);
	}

	const officeIds = [
		...toStringArray(rawSession["office_ids"]),
		...toStringArray(rawSession["officeIds"]),
		...toStringArray(rawSession["office_ids_list"]),
		...toStringArray(rawSession["officeIdsList"]),
		...toStringArray(claims["office_ids"]),
		...toStringArray(claims["officeIds"]),
	];
	if (!currentOfficeId && officeIds.length > 0) {
		currentOfficeId = officeIds[0] ?? null;
	}

	return {
		id: currentOfficeId,
		name: currentOfficeName,
	};
}
