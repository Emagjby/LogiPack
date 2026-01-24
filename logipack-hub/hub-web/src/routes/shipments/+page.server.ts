import type { Actions, PageServerLoad } from './$types';
import { changeStatus, createShipment, getTimeline, listShipments, listStatusOptions } from '$lib/data/shipments';

export const load: PageServerLoad = async (event) => {
	const shipments = await listShipments(event);
	const timelines = Object.fromEntries(
		await Promise.all(
			shipments.map(async (shipment) => [shipment.id, await getTimeline(event, shipment.id)])
		)
	);

	return {
		shipments,
		timelines,
		statusOptions: await listStatusOptions()
	};
};

export const actions: Actions = {
	create: async (event) => {
		const data = await event.request.formData();
		const devUserSub = String(data.get('dev_user_sub') ?? '').trim();
		if (!devUserSub) {
			return { success: false, error: 'Missing dev_user_sub. Select an actor in the header.' };
		}
		await createShipment(
			event,
			{
				client_id: String(data.get('client_id') ?? ''),
				current_office_id: String(data.get('current_office_id') ?? '') || null,
				notes: String(data.get('notes') ?? '') || null
			},
			devUserSub
		);

		return { success: true };
	},
	changeStatus: async (event) => {
		const data = await event.request.formData();
		const devUserSub = String(data.get('dev_user_sub') ?? '').trim();
		if (!devUserSub) {
			return { success: false, error: 'Missing dev_user_sub. Select an actor in the header.' };
		}
		const shipmentId = String(data.get('shipmentId') ?? '');
		if (!shipmentId) {
			return { success: false };
		}
		await changeStatus(
			event,
			shipmentId,
			{
				to_status: String(data.get('to_status') ?? '') as any,
				to_office_id: String(data.get('to_office_id') ?? '') || null,
				notes: String(data.get('notes') ?? '') || null
			},
			devUserSub
		);
		return { success: true };
	}
};
