import type { PageServerLoad } from "./$types";
import { error } from "@sveltejs/kit";
import type { ShipmentRow } from "$lib/domain/shipmentStatus";

// ── Result discriminant returned to the page ────────────────────────
type ShipmentsResult =
	| { state: "ok"; shipments: ShipmentRow[] }
	| { state: "empty"; shipments: [] }
	| { state: "error"; shipments: []; message: string };

/**
 * TODO: Replace with real hub-api call once the endpoint exists.
 *
 * Expected endpoint: GET {HUB_API_BASE}/shipments
 * Headers:           Authorization: Bearer <accessToken>
 * Response shape:    { shipments: ShipmentRow[] }
 *
 * The access token lives in the encrypted session cookie.
 * See `src/routes/callback/+server.ts` for the existing fetch pattern
 * using `HUB_API_BASE` from `$env/static/private`.
 */
async function fetchShipments(): Promise<ShipmentsResult> {
	// ── Mock data (structured like real API response) ────────────
	const mock: ShipmentRow[] = [
		{
			id: "SHP-1042",
			status: "in_transit",
			office: "Sofia HQ",
			updatedAt: new Date(Date.now() - 2 * 60_000).toISOString(),
		},
		{
			id: "SHP-1041",
			status: "delivered",
			office: "Plovdiv DC",
			updatedAt: new Date(Date.now() - 18 * 60_000).toISOString(),
		},
		{
			id: "SHP-1040",
			status: "pending",
			office: "Varna Port",
			updatedAt: new Date(Date.now() - 60 * 60_000).toISOString(),
		},
		{
			id: "SHP-1039",
			status: "in_transit",
			office: "Veliko Tarnovo Office",
			updatedAt: new Date(Date.now() - 3 * 3_600_000).toISOString(),
		},
		{
			id: "SHP-1038",
			status: "delivered",
			office: "Sofia HQ",
			updatedAt: new Date(Date.now() - 5 * 3_600_000).toISOString(),
		},
		{
			id: "SHP-1037",
			status: "cancelled",
			office: "Plovdiv DC",
			updatedAt: new Date(Date.now() - 8 * 3_600_000).toISOString(),
		},
		{
			id: "SHP-1036",
			status: "new",
			office: "Varna Port",
			updatedAt: new Date(Date.now() - 12 * 3_600_000).toISOString(),
		},
		{
			id: "SHP-1035",
			status: "accepted",
			office: "Sofia HQ",
			updatedAt: new Date(Date.now() - 24 * 3_600_000).toISOString(),
		},
	];

	return mock.length > 0
		? { state: "ok", shipments: mock }
		: { state: "empty", shipments: [] };
}

export const load: PageServerLoad = async ({ parent }) => {
	const { session } = await parent();
	const activeOffice = "Sofia HQ";

	// Role guard: admin should not access employee pages
	if (session?.role === "admin") {
		throw error(403, "error.details.employee_only");
	}

	try {
		const result = await fetchShipments();
		return { result, activeOffice };
	} catch (e) {
		return {
			activeOffice,
			result: {
				state: "error" as const,
				shipments: [] as [],
				message: e instanceof Error ? e.message : "Unknown error",
			},
		};
	}
};
