import type { PageServerLoad } from "./$types";
import type { ShipmentStatus } from "$lib/domain/shipmentStatus";
import type { StrataPackage } from "$lib/domain/strataPackage";
import { seededIndex } from "$lib/server/mockUtils";
import {
	getShipment,
	getShipmentTimeline,
	type MockTimelineEvent,
} from "$lib/server/mockShipments";

type ShipmentCore = {
	id: string;
	client_id: string;
	current_status: ShipmentStatus | "unknown";
	current_office_id: string | null;
	created_at: string;
	updated_at: string;
};

type StatusHistoryRow = {
	id: string;
	from_status: ShipmentStatus | "unknown" | null;
	to_status: ShipmentStatus | "unknown";
	changed_at: string;
	actor_user_id: string | null;
	office_id: string | null;
	notes: string | null;
};

type DetailResult =
	| {
			state: "ok";
			shipment: ShipmentCore;
			statusHistory: StatusHistoryRow[];
			packages: StrataPackage[];
	  }
	| { state: "not_found" }
	| { state: "error"; message: string };

const MOCK_ACTORS = [
	"user-emil-ivanov",
	"user-maria-petrova",
	"user-georgi-dimitrov",
	null,
] as const;

const STATUS_CHAINS: Record<string, ShipmentStatus[]> = {
	delivered: ["new", "accepted", "pending", "in_transit", "delivered"],
	in_transit: ["new", "accepted", "pending", "in_transit"],
	pending: ["new", "accepted", "pending"],
	accepted: ["new", "accepted"],
	cancelled: ["new", "accepted", "cancelled"],
	new: ["new"],
};

function mockHash(seed: string): string {
	let h = 0x811c9dc5;
	for (let i = 0; i < seed.length; i++) {
		h ^= seed.charCodeAt(i);
		h = Math.imul(h, 0x01000193);
	}
	const a = (h >>> 0).toString(16).padStart(8, "0");
	const b = ((h ^ 0xdeadbeef) >>> 0).toString(16).padStart(8, "0");
	const c = ((h ^ 0xcafebabe) >>> 0).toString(16).padStart(8, "0");
	const d = ((h ^ 0xfeedface) >>> 0).toString(16).padStart(8, "0");
	return `${a}${b}${c}${d}`;
}

function generateStatusHistory(
	shipmentId: string,
	shipmentStatus: ShipmentStatus,
	officeId: string | null,
	timeline: MockTimelineEvent[],
): StatusHistoryRow[] {
	const chain = STATUS_CHAINS[shipmentStatus] ?? ["new"];
	const fallbackTime = new Date().toISOString();

	return chain.map((status, index) => {
		const timelineRow = timeline[index];
		return {
			id: `sh-${shipmentId}-${index + 1}`,
			from_status: index === 0 ? null : chain[index - 1],
			to_status: status,
			changed_at: timelineRow?.createdAt ?? fallbackTime,
			actor_user_id:
				MOCK_ACTORS[seededIndex(`${shipmentId}-actor-${index}`, MOCK_ACTORS.length)],
			office_id: officeId,
			notes: timelineRow?.scbPreview ?? null,
		};
	});
}

function toStrataPackages(
	shipmentId: string,
	officeId: string | null,
	statusHistory: StatusHistoryRow[],
	timeline: MockTimelineEvent[],
): StrataPackage[] {
	// Expects timeline pre-sorted by `seq`.
	const sorted = timeline;
	const streamId = `stream-shipment-${shipmentId}`;

	return sorted.map((event, index) => {
		const hash = mockHash(`${shipmentId}-pkg-${event.seq}`);
		const prevHash = index === 0 ? null : mockHash(`${shipmentId}-pkg-${sorted[index - 1].seq}`);
		const historyRow = statusHistory[index] ?? null;
		const payload: Record<string, unknown> = {
			event_type: event.eventType,
			shipment_id: shipmentId,
			office_id: officeId,
			actor_user_id: historyRow?.actor_user_id ?? null,
			timestamp: event.createdAt,
			scb_preview: event.scbPreview,
			raw_scb_base64:
				Buffer.from(`mock-scb-${shipmentId}-${event.seq}`).toString("base64").slice(0, 44) +
				"==",
		};

		if (historyRow?.from_status) {
			payload.from_status = historyRow.from_status;
		}
		if (historyRow) {
			payload.to_status = historyRow.to_status;
			if (historyRow.notes) payload.notes = historyRow.notes;
		}

		return {
			hash,
			prev_hash: prevHash,
			stream_id: streamId,
			seq: event.seq,
			event_type: event.eventType,
			created_at: event.createdAt,
			payload_json: payload,
		};
	});
}

function fetchAdminShipmentDetail(id: string): DetailResult {
	const shipment = getShipment(id);
	if (!shipment) {
		return { state: "not_found" };
	}

	const timeline = getShipmentTimeline(id);
	const statusHistory = generateStatusHistory(
		shipment.id,
		shipment.status,
		shipment.currentOfficeId,
		timeline,
	);

	return {
		state: "ok",
		shipment: {
			id: shipment.id,
			client_id: shipment.clientId,
			current_status: shipment.status,
			current_office_id: shipment.currentOfficeId,
			created_at: shipment.createdAt,
			updated_at: shipment.updatedAt,
		},
		statusHistory,
		packages: toStrataPackages(
			shipment.id,
			shipment.currentOfficeId,
			statusHistory,
			timeline,
		),
	};
}

export const load: PageServerLoad = async ({ params }) => {
	try {
		const result = fetchAdminShipmentDetail(params.id);
		return { result };
	} catch (error) {
		return {
			result: {
				state: "error" as const,
				message:
					error instanceof Error
						? error.message
						: "Unable to load shipment detail right now.",
			},
		};
	}
};
