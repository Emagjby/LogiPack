import type { Actions, PageServerLoad } from './$types';
import {
	changeStatus,
	getShipment,
	getTimeline,
	listStatusOptions
} from '$lib/data/shipments';

export const load: PageServerLoad = async (event) => {
	const shipment = await getShipment(event, event.params.id);
	return {
		shipment,
		timeline: await getTimeline(event, event.params.id),
		statusOptions: await listStatusOptions()
	};
};

export const actions: Actions = {
	changeStatus: async (event) => {
		const data = await event.request.formData();
		const devUserSub = String(data.get('dev_user_sub') ?? '').trim();
		if (!devUserSub) {
			return { success: false, error: 'Missing dev_user_sub. Select an actor in the header.' };
		}
		await changeStatus(
			event,
			event.params.id,
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
