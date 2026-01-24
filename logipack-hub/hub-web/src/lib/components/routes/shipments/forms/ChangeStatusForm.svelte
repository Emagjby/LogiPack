<script lang="ts">
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import type { ChangeStatusRequest, ShipmentStatus } from '$lib/types/shipment';

	let { form, statusOptions } = $props<{
		form: ChangeStatusRequest;
		statusOptions: ShipmentStatus[];
	}>();

	const formIds = {
		status: 'change-status-status',
		office: 'change-status-office',
		notes: 'change-status-notes',
		devUserSub: 'change-status-dev-user-sub'
	};
</script>

<div class="flex-1 overflow-y-auto p-6 space-y-6">
	<input type="hidden" name="dev_user_sub" id={formIds.devUserSub} />
	<div class="space-y-2">
		<label class="block text-sm font-medium text-zinc-300" for={formIds.status}
			>New Status</label
		>
		<Select id={formIds.status} name="to_status" bind:value={form.to_status}>
			{#each statusOptions as option}
				<option value={option}>{option.replace('_', ' ')}</option>
			{/each}
		</Select>
	</div>

	<div class="space-y-2">
		<label class="block text-sm font-medium text-zinc-300" for={formIds.office}>
			New Office
			<span class="text-muted font-normal text-xs ml-1">(Optional)</span>
		</label>
		<Select id={formIds.office} name="to_office_id" bind:value={form.to_office_id}>
			<option value={null}>Keep current location</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1">Berlin Hub (Berlin - Gate 4)</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2">Austin TX (Austin - Dock 2)</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa3">Tokyo Dist (Tokyo - Bay 1)</option>
		</Select>
	</div>

	<div class="space-y-2">
		<label class="block text-sm font-medium text-zinc-300" for={formIds.notes}>Notes</label>
		<Textarea
			id={formIds.notes}
			name="notes"
			placeholder="Reason for status change..."
			rows={4}
			bind:value={form.notes}
		/>
	</div>
</div>
