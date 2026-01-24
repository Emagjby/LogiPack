<script lang="ts">
	import ShipmentStatusBadge from '$lib/components/features/shipments/badges/ShipmentStatusBadge.svelte';
	import ShipmentRowMenu from '$lib/components/routes/shipments/components/ShipmentRowMenu.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import type { ShipmentListItem } from '$lib/types/shipment';

	type MenuAnchor = {
		top: number;
		left: number;
		placement: 'top' | 'bottom';
	};

	let menuButtonEl = $state<HTMLButtonElement | null>(null);
	let menuAnchor: MenuAnchor | null = $state(null);

	let {
		shipment,
		selected = false,
		onSelect,
		onAction,
		onTimeline,
		onStatus,
		onClose
	} = $props<{
		shipment: ShipmentListItem;
		selected?: boolean;
		onSelect: (id: string) => void;
		onAction: (id: string) => void;
		onTimeline: (id: string) => void;
		onStatus: (id: string) => void;
		onClose: () => void;
	}>();
</script>

<tr
	class={`group hover:bg-zinc-800/50 transition-colors cursor-pointer ${
		selected ? 'bg-zinc-800/40' : ''
	}`}
	onclick={() => onSelect(shipment.id)}
>
	<td class="px-4 py-2.5">
		<button
			type="button"
			class="flex items-center gap-2 font-mono text-white text-sm"
			title={shipment.id}
			onclick={(event) => {
				event.stopPropagation();
				navigator.clipboard.writeText(shipment.id);
			}}
		>
			<span>{shipment.id.slice(0, 5)}...</span>
			<span class="opacity-0 group-hover:opacity-100 text-muted group-hover:text-white transition-opacity">
				<span class="material-symbols-outlined text-[14px]">content_copy</span>
			</span>
		</button>
	</td>
	<td class="px-4 py-2.5">
		<ShipmentStatusBadge status={shipment.current_status} />
	</td>
	<td class="px-4 py-2.5">
		{#if shipment.current_office_id}
			<button
				type="button"
				class="flex items-center gap-2 text-sm text-zinc-300 font-mono"
				title={shipment.current_office_id}
				onclick={(event) => {
					event.stopPropagation();
					navigator.clipboard.writeText(shipment.current_office_id);
				}}
			>
				<span>{shipment.current_office_id.slice(0, 5)}...</span>
				<span class="opacity-0 group-hover:opacity-100 text-muted group-hover:text-white transition-opacity">
					<span class="material-symbols-outlined text-[14px]">content_copy</span>
				</span>
			</button>
		{:else}
			<span class="text-sm text-zinc-300 font-mono">--</span>
		{/if}
	</td>
	<td class="px-4 py-2.5">
		<button
			type="button"
			class="flex items-center gap-2 text-sm text-zinc-300 font-mono"
			title={shipment.client_id}
			onclick={(event) => {
				event.stopPropagation();
				navigator.clipboard.writeText(shipment.client_id);
			}}
		>
			<span>{shipment.client_id.slice(0, 5)}...</span>
			<span class="opacity-0 group-hover:opacity-100 text-muted group-hover:text-white transition-opacity">
				<span class="material-symbols-outlined text-[14px]">content_copy</span>
			</span>
		</button>
	</td>
	<td class="px-4 py-2.5 text-sm text-muted text-right font-mono" title={shipment.created_at}>
		{new Date(shipment.created_at).toLocaleString(undefined, {
			year: '2-digit',
			month: 'short',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		})}
	</td>
	<td class="px-4 py-2.5 text-right relative">
		<Button
			bind:el={menuButtonEl}
			variant="ghost"
			size="sm"
			class="p-1"
			onclick={(event) => {
				event.stopPropagation();

				const rect = menuButtonEl ? menuButtonEl.getBoundingClientRect() : null;
				if (rect) {
					const menuWidth = 176;
					const menuHeight = 96;
					const gap = 8;

					const fitsBelow = rect.bottom + gap + menuHeight <= window.innerHeight;
					const placement: MenuAnchor['placement'] = fitsBelow ? 'bottom' : 'top';
					const top = fitsBelow ? rect.bottom + gap : rect.top - gap;
					const left = Math.min(window.innerWidth - menuWidth - 8, Math.max(8, rect.right - menuWidth));

					menuAnchor = { top, left, placement };
				}

				onAction(shipment.id);
			}}
		>
			<span class={`material-symbols-outlined text-[18px] ${selected ? 'text-white' : ''}`}>
				more_horiz
			</span>
		</Button>
		<ShipmentRowMenu
			open={selected}
			anchor={menuAnchor}
			onClose={onClose}
			onTimeline={() => onTimeline(shipment.id)}
			onStatus={() => onStatus(shipment.id)}
		/>
	</td>
</tr>
