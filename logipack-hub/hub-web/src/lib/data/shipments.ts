import type {
	ChangeStatusRequest,
	Client,
	CreateShipmentInput,
	Office,
	ShipmentDetail,
	ShipmentListItem,
	ShipmentStatus,
	TimelineItem,
	Uuid
} from '$lib/types/shipment';

const clients: Client[] = [
	{ id: '11111111-1111-1111-1111-111111111111', name: 'Acme Corp', email: null, phone: null },
	{ id: '22222222-2222-2222-2222-222222222222', name: 'Globex Inc', email: null, phone: null },
	{ id: '33333333-3333-3333-3333-333333333333', name: 'Stark Ind', email: null, phone: null }
];

const offices: Office[] = [
	{ id: 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', name: 'Berlin Hub', city: 'Berlin', address: 'Gate 4' },
	{ id: 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', name: 'Austin TX', city: 'Austin', address: 'Dock 2' },
	{ id: 'cccccccc-cccc-cccc-cccc-cccccccccccc', name: 'Tokyo Dist', city: 'Tokyo', address: 'Bay 1' },
	{ id: 'dddddddd-dddd-dddd-dddd-dddddddddddd', name: 'Osaka Hub', city: 'Osaka', address: 'Receiving' }
];

const nowIso = () => new Date().toISOString();

const details: ShipmentDetail[] = [
	{
		id: 'LP-8842-XJ',
		client: clients[0],
		current_status: 'IN_TRANSIT',
		current_office: offices[0],
		final_destination_office: offices[0],
		created_at: nowIso(),
		updated_at: nowIso()
	},
	{
		id: 'LP-8843-XY',
		client: clients[1],
		current_status: 'NEW',
		current_office: offices[1],
		final_destination_office: offices[1],
		created_at: nowIso(),
		updated_at: nowIso()
	},
	{
		id: 'LP-8844-AB',
		client: clients[1],
		current_status: 'DELIVERED',
		current_office: offices[2],
		final_destination_office: offices[3],
		created_at: nowIso(),
		updated_at: nowIso()
	},
	{
		id: 'LP-8845-ZZ',
		client: clients[0],
		current_status: 'IN_TRANSIT',
		current_office: offices[0],
		final_destination_office: offices[0],
		created_at: nowIso(),
		updated_at: nowIso()
	},
	{
		id: 'LP-9002-CA',
		client: clients[2],
		current_status: 'NEW',
		current_office: null,
		final_destination_office: null,
		created_at: nowIso(),
		updated_at: nowIso()
	}
];

const shipments: ShipmentListItem[] = details.map((d) => ({
	id: d.id,
	client_id: d.client.id,
	current_status: d.current_status,
	current_office_id: d.current_office?.id ?? null,
	created_at: d.created_at,
	updated_at: d.updated_at
}));

const timelines: Record<string, TimelineItem[]> = {
	'LP-8842-XJ': [
		{ seq: 4, event_type: 'LOCATION_SCAN', scb: btoa('Berlin Hub Gate 4') },
		{ seq: 3, event_type: 'STATUS_CHANGED', scb: btoa('PENDING -> IN_TRANSIT') },
		{ seq: 2, event_type: 'MANIFEST_UPDATED', scb: btoa('3 items') },
		{ seq: 1, event_type: 'SHIPMENT_CREATED', scb: btoa('created') }
	],
	'LP-8843-XY': [
		{ seq: 3, event_type: 'ROUTE_ASSIGNED', scb: btoa('Austin TX') },
		{ seq: 2, event_type: 'MANIFEST_UPDATED', scb: btoa('2 items') },
		{ seq: 1, event_type: 'SHIPMENT_CREATED', scb: btoa('created') }
	],
	'LP-8844-AB': [
		{ seq: 3, event_type: 'DELIVERED', scb: btoa('Osaka Hub') },
		{ seq: 2, event_type: 'DEPARTED', scb: btoa('Tokyo Dist') },
		{ seq: 1, event_type: 'SHIPMENT_CREATED', scb: btoa('created') }
	],
	'LP-8845-ZZ': [
		{ seq: 2, event_type: 'DEPARTED', scb: btoa('Prague Hub') },
		{ seq: 1, event_type: 'SHIPMENT_CREATED', scb: btoa('created') }
	]
};

const statusOrder: ShipmentStatus[] = [
	'NEW',
	'ACCEPTED',
	'PROCESSED',
	'IN_TRANSIT',
	'DELIVERED',
	'CANCELLED'
];

const statusCycle: Record<ShipmentStatus, ShipmentStatus> = {
	NEW: 'ACCEPTED',
	ACCEPTED: 'PROCESSED',
	PROCESSED: 'IN_TRANSIT',
	IN_TRANSIT: 'DELIVERED',
	DELIVERED: 'DELIVERED',
	CANCELLED: 'CANCELLED'
};

const cloneShipments = () => shipments.map((shipment) => ({ ...shipment }));

const getDetailById = (id: string): ShipmentDetail | undefined =>
	details.find((shipment) => shipment.id === id);

const getOfficeById = (id: Uuid | null): Office | null => {
	if (!id) return null;
	return offices.find((office) => office.id === id) ?? null;
};

export async function listShipments(): Promise<ShipmentListItem[]> {
	return cloneShipments();
}

export async function getShipment(id: string): Promise<ShipmentDetail> {
	const found = getDetailById(id);
	if (!found) {
		throw new Error(`Shipment ${id} not found`);
	}
	return { ...found };
}

export async function getTimeline(id: string): Promise<TimelineItem[]> {
	return timelines[id] ? timelines[id].map((item) => ({ ...item })) : [];
}

export async function createShipment(input: CreateShipmentInput): Promise<ShipmentDetail> {
	const nextId = `LP-90${String(shipments.length + 10).padStart(2, '0')}-NX`;
	const createdClient = clients.find((client) => client.id === input.client_id) ?? clients[0];
	const currentOffice = getOfficeById(input.current_office_id);

	const createdDetail: ShipmentDetail = {
		id: nextId,
		client: createdClient,
		current_status: 'NEW',
		current_office: currentOffice,
		final_destination_office: null,
		created_at: nowIso(),
		updated_at: nowIso()
	};

	const createdListItem: ShipmentListItem = {
		id: nextId,
		client_id: createdClient.id,
		current_status: createdDetail.current_status,
		current_office_id: currentOffice?.id ?? null,
		created_at: createdDetail.created_at,
		updated_at: createdDetail.updated_at
	};

	details.unshift(createdDetail);
	shipments.unshift(createdListItem);

	timelines[nextId] = [{ seq: 1, event_type: 'SHIPMENT_CREATED', scb: btoa(input.notes ?? '') }];

	return { ...createdDetail };
}

export async function changeStatus(id: string, input: ChangeStatusRequest): Promise<ShipmentDetail> {
	const detail = getDetailById(id);
	const listItem = shipments.find((shipment) => shipment.id === id);
	if (!detail || !listItem) {
		throw new Error(`Shipment ${id} not found`);
	}

	const previousStatus = detail.current_status;
	const nextStatus = input.to_status || statusCycle[detail.current_status];
	const nextOffice = getOfficeById(input.to_office_id);

	detail.current_status = nextStatus;
	detail.current_office = nextOffice;
	detail.updated_at = nowIso();

	listItem.current_status = nextStatus;
	listItem.current_office_id = nextOffice?.id ?? null;
	listItem.updated_at = detail.updated_at;

	const history = timelines[id] ?? [];
	history.unshift({
		seq: history.length + 1,
		event_type: 'STATUS_CHANGED',
		scb: btoa(`${previousStatus} -> ${nextStatus}\n${input.notes ?? ''}`)
	});
	timelines[id] = history;

	return { ...detail };
}

export async function listStatusOptions(): Promise<ShipmentStatus[]> {
	return [...statusOrder];
}
