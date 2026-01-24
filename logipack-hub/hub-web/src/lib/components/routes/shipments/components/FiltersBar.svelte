<script lang="ts">
	import Button from "$lib/components/ui/Button.svelte";
	import type { ShipmentStatus, Uuid } from '$lib/types/shipment';

	let { status, officeId, onChange, onClear } = $props<{
		status: ShipmentStatus | 'ALL';
		officeId: Uuid | 'ALL';
		onChange: (next: { status: ShipmentStatus | 'ALL'; officeId: Uuid | 'ALL' }) => void;
		onClear: () => void;
	}>();
</script>

<div class="flex flex-wrap items-center gap-3 pb-2 shrink-0">
	<div class="relative group">
		<select
			class="appearance-none pt-[7px] bg-surface-dark border border-border-dark text-white flex align-center justify-center text-xs rounded h-8 pl-3 pr-8 focus:ring-1 focus:ring-primary focus:border-primary cursor-pointer hover:bg-zinc-800 transition-colors"
			value={status}
			onchange={(e) => onChange({ status: (e.currentTarget as HTMLSelectElement).value as any, officeId })}
		>
			<option value="ALL">Status: All</option>
			<option value="NEW">NEW</option>
			<option value="ACCEPTED">ACCEPTED</option>
			<option value="PROCESSED">PROCESSED</option>
			<option value="IN_TRANSIT">IN TRANSIT</option>
			<option value="DELIVERED">DELIVERED</option>
			<option value="CANCELLED">CANCELLED</option>
		</select>
	</div>
	<div class="relative group">
		<select
			class="appearance-none pt-[7px] bg-surface-dark border border-border-dark text-white flex align-center justify-center text-xs rounded h-8 pl-3 pr-8 focus:ring-1 focus:ring-primary focus:border-primary cursor-pointer hover:bg-zinc-800 transition-colors"
			value={officeId}
			onchange={(e) =>
				onChange({ status, officeId: (e.currentTarget as HTMLSelectElement).value as any })}
		>
			<option value="ALL">Office: All Locations</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1">Berlin Hub</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2">Austin TX</option>
			<option value="aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa3">Tokyo Dist</option>
		</select>
	</div>
	<div class="h-4 w-px bg-border-dark mx-1"></div>
	<Button variant="ghost" size="sm" onclick={onClear}>Clear filters</Button>
</div>
