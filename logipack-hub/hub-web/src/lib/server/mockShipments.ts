import type { ShipmentStatus } from "$lib/domain/shipmentStatus";
import { seededIndex } from "$lib/server/mockUtils";

export interface MockShipment {
	id: string;
	clientId: string;
	currentOfficeId: string | null;
	notes: string | null;
	status: ShipmentStatus;
	createdAt: string;
	updatedAt: string;
}

export interface MockTimelineEvent {
	seq: number;
	eventType: string;
	scbPreview: string;
	createdAt: string;
}

export interface CreateShipmentInput {
	clientId: string;
	currentOfficeId?: string | null;
	notes?: string | null;
}

const mockShipments = new Map<string, MockShipment>();
const mockTimelineEvents = new Map<string, MockTimelineEvent[]>();
let seeded = false;

const OFFICE_NAMES = ["Sofia HQ", "Plovdiv DC", "Varna Port", "Burgas Hub"] as const;

type SeedShipment = {
	id: string;
	status: ShipmentStatus;
	office: (typeof OFFICE_NAMES)[number];
	clientId: string;
	notes?: string | null;
	updatedMinutesAgo: number;
	createdMinutesAgo: number;
};

const SEED_SHIPMENTS: SeedShipment[] = [
	{
		id: "SHP-2104",
		status: "in_transit",
		office: "Sofia HQ",
		clientId: "client-acme",
		updatedMinutesAgo: 4,
		createdMinutesAgo: 420,
	},
	{
		id: "SHP-2103",
		status: "pending",
		office: "Plovdiv DC",
		clientId: "client-techparts",
		updatedMinutesAgo: 13,
		createdMinutesAgo: 480,
	},
	{
		id: "SHP-2102",
		status: "delivered",
		office: "Varna Port",
		clientId: "client-greenline",
		updatedMinutesAgo: 22,
		createdMinutesAgo: 560,
	},
	{
		id: "SHP-2101",
		status: "accepted",
		office: "Burgas Hub",
		clientId: "client-nova",
		updatedMinutesAgo: 38,
		createdMinutesAgo: 610,
	},
	{
		id: "SHP-2100",
		status: "new",
		office: "Sofia HQ",
		clientId: "client-acme",
		updatedMinutesAgo: 58,
		createdMinutesAgo: 710,
	},
	{
		id: "SHP-2099",
		status: "in_transit",
		office: "Plovdiv DC",
		clientId: "client-techparts",
		updatedMinutesAgo: 95,
		createdMinutesAgo: 820,
	},
	{
		id: "SHP-2098",
		status: "cancelled",
		office: "Varna Port",
		clientId: "client-nova",
		updatedMinutesAgo: 138,
		createdMinutesAgo: 930,
	},
	{
		id: "SHP-2097",
		status: "pending",
		office: "Burgas Hub",
		clientId: "client-greenline",
		updatedMinutesAgo: 185,
		createdMinutesAgo: 1020,
	},
	{
		id: "SHP-2096",
		status: "accepted",
		office: "Sofia HQ",
		clientId: "client-acme",
		updatedMinutesAgo: 272,
		createdMinutesAgo: 1210,
	},
	{
		id: "SHP-2095",
		status: "delivered",
		office: "Plovdiv DC",
		clientId: "client-techparts",
		updatedMinutesAgo: 360,
		createdMinutesAgo: 1360,
	},
	{
		id: "SHP-2094",
		status: "new",
		office: "Varna Port",
		clientId: "client-nova",
		updatedMinutesAgo: 430,
		createdMinutesAgo: 1480,
	},
	{
		id: "SHP-2093",
		status: "in_transit",
		office: "Burgas Hub",
		clientId: "client-greenline",
		updatedMinutesAgo: 525,
		createdMinutesAgo: 1620,
	},
];

function normalizeOptional(value: string | null | undefined): string | null {
	const normalized = value?.trim();
	return normalized ? normalized : null;
}

function statusEventChain(status: ShipmentStatus): string[] {
	if (status === "new") return ["shipment_created", "status_new"];
	if (status === "accepted")
		return ["shipment_created", "status_new", "status_accepted"];
	if (status === "pending")
		return [
			"shipment_created",
			"status_new",
			"status_accepted",
			"status_pending",
		];
	if (status === "in_transit")
		return [
			"shipment_created",
			"status_new",
			"status_accepted",
			"status_pending",
			"status_in_transit",
		];
	if (status === "delivered")
		return [
			"shipment_created",
			"status_new",
			"status_accepted",
			"status_pending",
			"status_in_transit",
			"status_delivered",
		];
	return [
		"shipment_created",
		"status_new",
		"status_accepted",
		"status_cancelled",
	];
}

function buildTimeline(
	shipmentId: string,
	status: ShipmentStatus,
	createdAtIso: string,
): MockTimelineEvent[] {
	const createdAt = new Date(createdAtIso).getTime();
	const chain = statusEventChain(status);
	const maxEvents = 2 + seededIndex(shipmentId + status, 3);
	const eventTypes = chain.slice(0, Math.max(2, Math.min(chain.length, maxEvents)));

	return eventTypes.map((eventType, index) => ({
		seq: index + 1,
		eventType,
		scbPreview: `SCB preview ${eventType} #${index + 1}`,
		createdAt: new Date(
			createdAt + index * (15 + seededIndex(`${shipmentId}-${index}`, 35)) * 60_000,
		).toISOString(),
	}));
}

function parseShipmentNumber(id: string): number | null {
	const match = /^SHP-(\d+)$/i.exec(id.trim());
	if (!match) return null;
	return Number(match[1]);
}

function nextShipmentId(): string {
	let maxNumber = 0;
	for (const id of mockShipments.keys()) {
		const number = parseShipmentNumber(id);
		if (number !== null) {
			maxNumber = Math.max(maxNumber, number);
		}
	}
	return `SHP-${String(maxNumber + 1).padStart(4, "0")}`;
}

function ensureSeedData(): void {
	// Seed only once per process to keep IDs stable in-memory.
	if (seeded) return;
	const now = Date.now();

	for (const seed of SEED_SHIPMENTS) {
		const createdAt = new Date(now - seed.createdMinutesAgo * 60_000).toISOString();
		const updatedAt = new Date(now - seed.updatedMinutesAgo * 60_000).toISOString();
		const shipment: MockShipment = {
			id: seed.id,
			clientId: seed.clientId,
			currentOfficeId: seed.office,
			notes: normalizeOptional(seed.notes),
			status: seed.status,
			createdAt,
			updatedAt,
		};

		mockShipments.set(seed.id, shipment);
		mockTimelineEvents.set(seed.id, buildTimeline(seed.id, shipment.status, shipment.createdAt));
	}
	seeded = true;
}

export function createShipment(input: CreateShipmentInput): { id: string } {
	ensureSeedData();

	const clientId = input.clientId.trim();
	if (!clientId) {
		throw new Error("clientId is required");
	}

	const id = nextShipmentId();
	const now = new Date().toISOString();

	const shipment: MockShipment = {
		id,
		clientId,
		currentOfficeId: normalizeOptional(input.currentOfficeId),
		notes: normalizeOptional(input.notes),
		status: "new",
		createdAt: now,
		updatedAt: now,
	};

	mockShipments.set(id, shipment);
	mockTimelineEvents.set(id, buildTimeline(id, shipment.status, shipment.createdAt));

	return { id };
}

export function getShipment(id: string): MockShipment | null {
	ensureSeedData();
	return mockShipments.get(id) ?? null;
}

export function listShipments(): MockShipment[] {
	ensureSeedData();
	return [...mockShipments.values()].sort((a, b) =>
		b.updatedAt.localeCompare(a.updatedAt),
	);
}

export function getShipmentTimeline(id: string): MockTimelineEvent[] {
	ensureSeedData();
	const timeline = mockTimelineEvents.get(id) ?? [];
	return [...timeline].sort((a, b) => a.seq - b.seq);
}
