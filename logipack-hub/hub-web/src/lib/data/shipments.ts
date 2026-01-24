import type {
	ChangeStatusRequest,
	CreateShipmentInput,
	ShipmentDetail,
	ShipmentListItem,
	ShipmentStatus,
	TimelineItem
} from '$lib/types/shipment';
import { apiFetch } from '$lib/server/api';
import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export async function listShipments(event: RequestEvent): Promise<ShipmentListItem[]> {
	const res = await apiFetch(event, '/shipments', { method: 'GET' });
	if (!res.ok) {
		throw error(res.status, await res.text());
	}
	return (await res.json()) as ShipmentListItem[];
}

export async function getShipment(event: RequestEvent, id: string): Promise<ShipmentDetail> {
	const res = await apiFetch(event, `/shipments/${encodeURIComponent(id)}`, { method: 'GET' });
	if (!res.ok) {
		throw error(res.status, await res.text());
	}
	return (await res.json()) as ShipmentDetail;
}

export async function getTimeline(event: RequestEvent, id: string): Promise<TimelineItem[]> {
	const res = await apiFetch(event, `/shipments/${encodeURIComponent(id)}/timeline`, { method: 'GET' });
	if (!res.ok) {
		throw error(res.status, await res.text());
	}
	return (await res.json()) as TimelineItem[];
}

export async function createShipment(event: RequestEvent, input: CreateShipmentInput, devUserSub: string) {
	const res = await apiFetch(event, '/shipments', {
		method: 'POST',
		headers: { 'content-type': 'application/json', 'x-dev-user-sub': devUserSub },
		body: JSON.stringify(input)
	});


	if (!res.ok) {
		throw error(res.status, await res.text());
	}

	return (await res.json()) as { shipment_id: string };
}

export async function changeStatus(event: RequestEvent, id: string, input: ChangeStatusRequest, devUserSub: string) {
	const payload = {
		shipment_id: id,
		to_status: input.to_status,
		to_office_id: input.to_office_id,
		notes: input.notes
	};

	const res = await apiFetch(event, `/shipments/${encodeURIComponent(id)}/status`, {
		method: 'POST',
		headers: { 'content-type': 'application/json', 'x-dev-user-sub': devUserSub },
		body: JSON.stringify(payload)
	});

	if (!res.ok) {
		throw error(res.status, await res.text());
	}
}

export async function listStatusOptions(): Promise<ShipmentStatus[]> {
	return ['NEW', 'ACCEPTED', 'PROCESSED', 'IN_TRANSIT', 'DELIVERED', 'CANCELLED'];
}
