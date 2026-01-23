<script lang="ts">
	import Panel from '$lib/components/ui/Panel.svelte';
	import type { ShipmentDetail } from '$lib/types/shipment';

	let { shipment } = $props<{ shipment: ShipmentDetail }>();
</script>

<div class="lg:col-span-2 flex flex-col gap-4">
	<h3 class="text-sm font-semibold text-white">Cargo Manifest</h3>
	<Panel>
		<div class="p-6 text-sm text-muted">
			Manifest items are not part of the current `ShipmentDetail` DTO.
		</div>
	</Panel>
</div>
