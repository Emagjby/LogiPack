import type { Actions, PageServerLoad } from './$types';
import { changeStatus, createShipment, getTimeline, listShipments, listStatusOptions } from '$lib/data/shipments';

export const load: PageServerLoad = async () => {
	const shipments = await listShipments();
	const timelines = Object.fromEntries(
		await Promise.all(
			shipments.map(async (shipment) => [shipment.id, await getTimeline(shipment.id)])
		)
	);

	return {
		shipments,
		timelines,
		statusOptions: await listStatusOptions()
	};
};

export const actions: Actions = {
	create: async ({ request }) => {
		const data = await request.formData();
		await createShipment({
			client_id: String(data.get('client_id') ?? ''),
			current_office_id: String(data.get('current_office_id') ?? '') || null,
			notes: String(data.get('notes') ?? '') || null
		});

		return { success: true };
	},
	changeStatus: async ({ request }) => {
		const data = await request.formData();
		const shipmentId = String(data.get('shipmentId') ?? '');
		if (!shipmentId) {
			return { success: false };
		}
		await changeStatus(shipmentId, {
			to_status: String(data.get('to_status') ?? '') as any,
			to_office_id: String(data.get('to_office_id') ?? '') || null,
			notes: String(data.get('notes') ?? '') || null
		});
		return { success: true };
	}
};
