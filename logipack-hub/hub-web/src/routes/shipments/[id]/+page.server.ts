import type { Actions, PageServerLoad } from './$types';
import {
	changeStatus,
	getShipment,
	getTimeline,
	listStatusOptions
} from '$lib/data/shipments';

export const load: PageServerLoad = async ({ params }) => {
	const shipment = await getShipment(params.id);
	return {
		shipment,
		timeline: await getTimeline(params.id),
		statusOptions: await listStatusOptions()
	};
};

export const actions: Actions = {
	changeStatus: async ({ request, params }) => {
		const data = await request.formData();
		await changeStatus(params.id, {
			to_status: String(data.get('to_status') ?? '') as any,
			to_office_id: String(data.get('to_office_id') ?? '') || null,
			notes: String(data.get('notes') ?? '') || null
		});
		return { success: true };
	}
};
