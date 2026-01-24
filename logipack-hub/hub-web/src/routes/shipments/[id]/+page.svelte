<script lang="ts">
	import type { PageData } from './$types';
	import Button from '$lib/components/ui/Button.svelte';
	import AppHeader from '$lib/components/layout/AppHeader.svelte';
	import ShipmentHeader from '$lib/components/routes/shipments/components/ShipmentHeader.svelte';
	import CargoManifest from '$lib/components/routes/shipments/components/CargoManifest.svelte';
	const { data } = $props<{ data: PageData; form?: { success?: boolean } }>();
</script>

<div class="bg-background-light dark:bg-background-dark text-slate-900 dark:text-white font-display overflow-hidden h-screen flex flex-col">
	<AppHeader
		breadcrumbs={[{ label: 'Dashboard' }, { label: 'Shipments', href: '/shipments' }]}
		titleSuffix={`#${data.shipment.id}`}
	/>
	<div class="flex flex-1 overflow-hidden relative">
		<main class="flex-1 flex flex-col h-full min-w-0 overflow-y-auto bg-background-dark">
			<div class="w-full max-w-[1400px] mx-auto p-6 flex flex-col gap-8 h-full">
				<ShipmentHeader shipment={data.shipment} />
				<div class="grid grid-cols-1 gap-6 items-start">
					<CargoManifest shipment={data.shipment} />
				</div>
			</div>
		</main>

	</div>
</div>
