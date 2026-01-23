<script lang="ts">
	import type { ShipmentDetail } from '$lib/types/shipment';

	let { shipment } = $props<{ shipment: ShipmentDetail }>();
</script>

<div class="flex flex-wrap items-center gap-x-8 gap-y-3 text-sm">
	<div class="flex items-center gap-2.5">
		<span class="material-symbols-outlined text-muted text-[18px]">domain</span>
		<span class="text-muted">Client:</span>
		<span class="text-white font-medium">{shipment.client.name}</span>
	</div>
	<div class="flex items-center gap-2.5">
		<span class="material-symbols-outlined text-muted text-[18px]">location_on</span>
		<span class="text-muted">Current Office:</span>
		<span class="text-white font-medium">{shipment.current_office?.name ?? '--'}</span>
	</div>
	<div class="flex items-center gap-2.5">
		<span class="material-symbols-outlined text-muted text-[18px]">flag</span>
		<span class="text-muted">Final Destination:</span>
		<span class="text-white font-medium">--</span>
	</div>
	<div class="flex items-center gap-2.5">
		<span class="material-symbols-outlined text-muted text-[18px]">schedule</span>
		<span class="text-muted">Created At:</span>
		<span class="text-white font-medium font-mono">{shipment.created_at}</span>
	</div>
</div>
