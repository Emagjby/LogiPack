<script lang="ts">
	import Drawer from "$lib/components/ui/Drawer.svelte";
	import Button from "$lib/components/ui/Button.svelte";
	import CreateShipmentForm from "$lib/components/routes/shipments/forms/CreateShipmentForm.svelte";
	import type { CreateShipmentInput } from "$lib/types/shipment";

	let { open, form, onClose, onSubmit } = $props<{
		open: boolean;
		form: CreateShipmentInput;
		onClose: () => void;
		onSubmit: () => void;
	}>();
</script>

<Drawer {open} class="w-[400px]">
	<div
		class="flex items-center justify-between px-6 py-5 border-b border-border-dark bg-[#1c1c1f]"
	>
		<h3 class="text-lg font-semibold text-white">New Shipment</h3>
		<Button variant="ghost" size="sm" class="p-1" onclick={onClose}>
			<span class="material-symbols-outlined text-[20px]">close</span>
		</Button>
	</div>
	<CreateShipmentForm {form} />
	<div class="p-6 border-t border-border-dark bg-[#1c1c1f] flex gap-3">
		<Button variant="secondary" class="flex-1" onclick={onClose}>
			Cancel
		</Button>
		<Button type="submit" class="flex-1" onclick={onSubmit}
			>Create Shipment</Button
		>
	</div>
</Drawer>
