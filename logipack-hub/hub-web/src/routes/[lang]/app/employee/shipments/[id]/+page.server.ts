import type { PageServerLoad } from "./$types";
import { error } from "@sveltejs/kit";
import type { ShipmentStatus } from "$lib/domain/shipmentStatus";
import type { StrataPackage } from "$lib/domain/strataPackage";
import { seededIndex } from "$lib/server/mockUtils";

// ── Data contracts ──────────────────────────────────────────────────

/** Mirrors the `shipments` table. */
export interface ShipmentCore {
	id: string;
	client_id: string;
	current_status: ShipmentStatus | "unknown";
	current_office_id: string;
	created_at: string; // ISO-8601
	updated_at: string; // ISO-8601
}

/** Mirrors the `shipment_status_history` table. */
export interface StatusHistoryRow {
	id: string;
	from_status: ShipmentStatus | "unknown" | null;
	to_status: ShipmentStatus | "unknown";
	changed_at: string; // ISO-8601
	actor_user_id: string | null;
	office_id: string | null;
	notes: string | null;
}

type DetailResult =
	| {
			state: "ok";
			shipment: ShipmentCore;
			statusHistory: StatusHistoryRow[];
			packages: StrataPackage[];
	  }
	| { state: "not_found" }
	| { state: "error"; message: string };

// ── Mock helpers ────────────────────────────────────────────────────

const MOCK_STATUSES: ShipmentStatus[] = [
	"new",
	"accepted",
	"pending",
	"in_transit",
	"delivered",
	"cancelled",
];

const MOCK_OFFICES = [
	{ id: "office-sofia-hq", name: "Sofia HQ" },
	{ id: "office-plovdiv-dc", name: "Plovdiv DC" },
	{ id: "office-varna-port", name: "Varna Port" },
	{ id: "office-vt", name: "Veliko Tarnovo Office" },
];

const MOCK_CLIENTS = [
	{ id: "client-acme", name: "Acme Corp" },
	{ id: "client-nova", name: "Nova Logistics" },
	{ id: "client-techparts", name: "TechParts Ltd" },
	{ id: "client-greenline", name: "GreenLine Express" },
];

const MOCK_ACTORS = [
	"user-emil-ivanov",
	"user-maria-petrova",
	"user-georgi-dimitrov",
	null,
];

/** Produce a deterministic hex hash from seed string. */
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

// ── Status transition chain ─────────────────────────────────────────

const STATUS_CHAINS: Record<string, ShipmentStatus[]> = {
	delivered: ["new", "accepted", "pending", "in_transit", "delivered"],
	in_transit: ["new", "accepted", "pending", "in_transit"],
	pending: ["new", "accepted", "pending"],
	accepted: ["new", "accepted"],
	cancelled: ["new", "accepted", "cancelled"],
	new: ["new"],
};

function generateStatusHistory(
	shipmentId: string,
	currentStatus: ShipmentStatus,
	baseTime: Date,
): StatusHistoryRow[] {
	const chain = STATUS_CHAINS[currentStatus] ?? ["new"];

	const HISTORY_NOTES: (string | null)[] = [
		"Shipment registered in the system",
		"Accepted by dispatcher",
		"Queued for processing",
		null,
		"Signed by recipient",
		"Client requested cancellation",
	];

	return chain.map((status, i) => ({
		id: `sh-${shipmentId}-${i + 1}`,
		from_status: i === 0 ? null : chain[i - 1],
		to_status: status,
		changed_at: new Date(
			baseTime.getTime() + i * (3 + seededIndex(shipmentId + i, 6)) * 3_600_000,
		).toISOString(),
		actor_user_id: MOCK_ACTORS[seededIndex(shipmentId + "actor" + i, MOCK_ACTORS.length)],
		office_id: MOCK_OFFICES[seededIndex(shipmentId + "office" + i, MOCK_OFFICES.length)].id,
		notes: HISTORY_NOTES[i % HISTORY_NOTES.length],
	}));
}

// ── Strata package chain ────────────────────────────────────────────

const EVENT_TYPES = [
	"shipment_created",
	"status_changed",
	"assigned_to_office",
	"picked_up",
	"departed_office",
	"arrived_at_hub",
	"customs_cleared",
	"out_for_delivery",
	"delivery_attempted",
	"delivered",
];

function generateStrataPackages(
	shipmentId: string,
	streamId: string,
	statusHistory: StatusHistoryRow[],
	baseTime: Date,
): StrataPackage[] {
	const count = Math.max(statusHistory.length, 6) + seededIndex(shipmentId + "pkg", 4);
	const packages: StrataPackage[] = [];

	for (let i = 0; i < count; i++) {
		const hash = mockHash(`${shipmentId}-pkg-${i}`);
		const prevHash = i === 0 ? null : packages[i - 1].hash;
		const eventType = EVENT_TYPES[i % EVENT_TYPES.length];
		const offset = i * (2 + seededIndex(shipmentId + "t" + i, 5)) * 3_600_000;
		const createdAt = new Date(baseTime.getTime() + offset).toISOString();

		const historyRow = statusHistory[i] ?? null;
		const officeId =
			historyRow?.office_id ??
			MOCK_OFFICES[seededIndex(shipmentId + "po" + i, MOCK_OFFICES.length)].id;
		const actorId =
			historyRow?.actor_user_id ??
			MOCK_ACTORS[seededIndex(shipmentId + "pa" + i, MOCK_ACTORS.length)];

		// Build realistic "decoded SCB" payload
		const payload: Record<string, unknown> = {
			event_type: eventType,
			shipment_id: shipmentId,
			office_id: officeId,
			actor_user_id: actorId,
			timestamp: createdAt,
			scb_preview: `[binary ${64 + seededIndex(hash, 128)} bytes]`,
			raw_scb_base64:
				Buffer.from(`mock-scb-${shipmentId}-${i}`).toString("base64").slice(0, 44) +
				"==",
		};

		if (historyRow) {
			if (historyRow.from_status !== null) {
				payload.from_status = historyRow.from_status;
			}
			payload.to_status = historyRow.to_status;
			if (historyRow.notes) {
				payload.notes = historyRow.notes;
			}
		}

		packages.push({
			hash,
			prev_hash: prevHash,
			stream_id: streamId,
			seq: i + 1,
			event_type: eventType,
			created_at: createdAt,
			payload_json: payload,
		});
	}

	return packages.sort((a, b) => a.seq - b.seq);
}

// ── Fetch (mock) ────────────────────────────────────────────────────

/**
 * TODO(api): Replace with real hub-api calls once endpoints exist.
 *
 * Expected endpoints:
 *   GET {HUB_API_BASE}/shipments/:id
 *     Response: { shipment: ShipmentCore }
 *
 *   GET {HUB_API_BASE}/shipments/:id/status-history
 *     Response: { history: StatusHistoryRow[] }
 *
 *   GET {HUB_API_BASE}/streams/:stream_id/packages
 *     Response: { packages: StrataPackage[] }
 *
 * Headers: Authorization: Bearer <accessToken>
 * The access token lives in the encrypted session cookie.
 */
async function fetchShipmentDetail(id: string): Promise<DetailResult> {
	// TODO(api): fetch shipments row by id
	// TODO(api): fetch shipment_status_history by shipment_id
	// TODO(api): fetch Strata packages by stream_id
	// TODO(strata): decode SCB to JSON (currently mocked)
	if (!/^SHP-\d{4,}$/i.test(id)) {
		return { state: "not_found" };
	}

	const now = new Date();
	const baseTime = new Date(now.getTime() - 72 * 3_600_000); // 3 days ago

	const statusIdx = seededIndex(id, MOCK_STATUSES.length);
	const officeIdx = seededIndex(id + "office", MOCK_OFFICES.length);
	const clientIdx = seededIndex(id + "client", MOCK_CLIENTS.length);
	const currentStatus = MOCK_STATUSES[statusIdx];

	const shipment: ShipmentCore = {
		id,
		client_id: MOCK_CLIENTS[clientIdx].id,
		current_status: currentStatus,
		current_office_id: MOCK_OFFICES[officeIdx].id,
		created_at: baseTime.toISOString(),
		updated_at: new Date(
			now.getTime() - seededIndex(id + "upd", 48) * 3_600_000,
		).toISOString(),
	};

	const statusHistory = generateStatusHistory(id, currentStatus, baseTime);

	const streamId = `stream-shipment-${id}`;
	const packages = generateStrataPackages(id, streamId, statusHistory, baseTime);

	return { state: "ok", shipment, statusHistory, packages };
}

// ── Load function ───────────────────────────────────────────────────

export const load: PageServerLoad = async ({ parent, params }) => {
	const { session } = await parent();

	// Role guard: admin should not access employee pages
	if (session?.role === "admin") {
		throw error(403, "error.details.employee_only");
	}

	const id = params.id;

	try {
		const result = await fetchShipmentDetail(id);
		return { result };
	} catch (e) {
		return {
			result: {
				state: "error" as const,
				message: e instanceof Error ? e.message : "Unknown error",
			},
		};
	}
};
