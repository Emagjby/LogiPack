<script lang="ts">
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import type { CreateShipmentInput } from '$lib/types/shipment';

	let { form } = $props<{ form: CreateShipmentInput }>();

	const formIds = {
		clientId: 'create-shipment-client-id',
		officeId: 'create-shipment-office-id',
		notes: 'create-shipment-notes',
		devUserSub: 'create-shipment-dev-user-sub'
	};
</script>

<div class="flex-1 overflow-y-auto p-6 space-y-6">
	<input type="hidden" name="dev_user_sub" id={formIds.devUserSub} />
	<div class="space-y-2">
		<label class="block text-sm font-medium text-zinc-300" for={formIds.clientId}
			>Client ID</label
		>
		<Input
			id={formIds.clientId}
			name="client_id"
			placeholder="UUID"
			bind:value={form.client_id}
		/>
		<p class="text-xs text-muted">UUID of the client.</p>
	</div>

	<div class="space-y-2">
		<label class="block text-sm font-medium text-zinc-300" for={formIds.officeId}
			>Current Office (optional)</label
		>
		<Select
			id={formIds.officeId}
			name="current_office_id"
			bind:value={form.current_office_id}
		>
			<option value={null}>Unassigned</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1">Berlin Hub (Berlin - Gate 4)</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2">Austin TX (Austin - Dock 2)</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa3">Tokyo Dist (Tokyo - Bay 1)</option>
		</Select>
	</div>

	<div class="space-y-2">
		<label class="block text-sm font-medium text-zinc-300" for={formIds.notes}
			>Notes</label
		>
		<Textarea
			id={formIds.notes}
			name="notes"
			placeholder="Special handling instructions, gate codes, etc."
			rows={4}
			bind:value={form.notes}
		/>
	</div>
</div>
