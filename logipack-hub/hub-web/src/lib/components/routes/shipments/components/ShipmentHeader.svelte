<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import ShipmentStatusBadge from '$lib/components/features/shipments/badges/ShipmentStatusBadge.svelte';
	import ShipmentMeta from '$lib/components/routes/shipments/components/ShipmentMeta.svelte';
	import type { ShipmentDetail } from '$lib/types/shipment';

	let { shipment } = $props<{
		shipment: ShipmentDetail;
	}>();
</script>

<div class="flex flex-col gap-6 border-b border-border-dark pb-6 shrink-0">
	<div class="flex flex-col sm:flex-row sm:items-start justify-between gap-4">
		<div class="space-y-1">
			<div class="flex items-center gap-3">
				<h1 class="text-2xl font-bold tracking-tight text-white font-mono">
					Shipment #{shipment.id}
				</h1>
				<ShipmentStatusBadge status={shipment.current_status} />
			</div>
			<p class="text-muted text-sm">Review details and manage logistics.</p>
		</div>
		<div class="flex items-center gap-3">
			<Button variant="secondary" size="sm" class="px-3" title="Refresh Data">
				<span class="material-symbols-outlined text-[18px]">refresh</span>
			</Button>
		</div>
	</div>
	<ShipmentMeta {shipment} />
</div>
