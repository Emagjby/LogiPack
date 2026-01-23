<script lang="ts">
	import Drawer from "$lib/components/ui/Drawer.svelte";
	import Button from "$lib/components/ui/Button.svelte";
	import ChangeStatusForm from "$lib/components/routes/shipments/forms/ChangeStatusForm.svelte";
	import type {
		ChangeStatusRequest,
		ShipmentStatus,
	} from "$lib/types/shipment";

	let { open, form, statusOptions, onClose, onSubmit } = $props<{
		open: boolean;
		form: ChangeStatusRequest;
		statusOptions: ShipmentStatus[];
		onClose: () => void;
		onSubmit: () => void;
	}>();
</script>

<Drawer {open} class="w-[400px]">
	<div
		class="flex items-center justify-between px-6 py-5 border-b border-border-dark bg-[#1c1c1f]"
	>
		<h3 class="text-lg font-semibold text-white">Change Status</h3>
		<Button variant="ghost" size="sm" class="p-1" onclick={onClose}>
			<span class="material-symbols-outlined text-[20px]">close</span>
		</Button>
	</div>
	<ChangeStatusForm {form} {statusOptions} />
	<div
		class="p-6 border-t border-border-dark bg-[#1c1c1f] flex flex-col gap-4"
	>
		<div
			class="flex items-start gap-2 text-amber-500/80 bg-amber-500/5 p-2 rounded border border-amber-500/10"
		>
			<span class="material-symbols-outlined text-[16px] mt-0.5"
				>info</span
			>
			<p class="text-xs leading-5">
				Status changes are audited and immutable.
			</p>
		</div>
		<div class="flex gap-3">
			<Button variant="secondary" class="flex-1" onclick={onClose}
				>Cancel</Button
			>
			<Button type="submit" class="flex-1" onclick={onSubmit}
				>Apply Changes</Button
			>
		</div>
	</div>
</Drawer>
