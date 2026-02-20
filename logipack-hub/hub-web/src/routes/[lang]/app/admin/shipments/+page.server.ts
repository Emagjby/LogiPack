import type { PageServerLoad } from "./$types";
import type { ShipmentRow } from "$lib/domain/shipmentStatus";
import { listShipments } from "$lib/server/mockShipments";

type AdminShipmentsResult =
	| { state: "ok"; shipments: ShipmentRow[] }
	| { state: "empty"; shipments: [] }
	| { state: "error"; shipments: []; message?: string };

async function fetchAdminShipments(): Promise<AdminShipmentsResult> {
	const shipments: ShipmentRow[] = listShipments().map((shipment) => ({
		id: shipment.id,
		status: shipment.status,
		office: shipment.currentOfficeId ?? "—",
		updatedAt: shipment.updatedAt,
	}));

	return shipments.length > 0
		? { state: "ok", shipments }
		: { state: "empty", shipments: [] };
}

export const load: PageServerLoad = async () => {
	try {
		const result = await fetchAdminShipments();
		return { result };
	} catch {
		return {
			result: {
				state: "error" as const,
				shipments: [] as [],
				message: "shipments.error.generic",
			},
		};
	}
};
