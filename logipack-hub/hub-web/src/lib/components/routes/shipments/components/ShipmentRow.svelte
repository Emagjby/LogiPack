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
		<div class="flex items-center gap-2">
			<span class="font-mono text-white text-sm">{shipment.id}</span>
			<button
				type="button"
				class="opacity-0 group-hover:opacity-100 text-muted hover:text-white transition-opacity"
				title="Copy ID"
				onclick={(event) => {
					event.stopPropagation();
					navigator.clipboard.writeText(shipment.id);
				}}
			>
				<span class="material-symbols-outlined text-[14px]">content_copy</span>
			</button>
		</div>
	</td>
	<td class="px-4 py-2.5">
		<ShipmentStatusBadge status={shipment.current_status} />
	</td>
	<td class="px-4 py-2.5 text-sm text-zinc-300 font-mono">{shipment.current_office_id ?? '--'}</td>
	<td class="px-4 py-2.5 text-sm text-zinc-300 font-mono">{shipment.client_id}</td>
	<td class="px-4 py-2.5 text-sm text-muted text-right font-mono">{shipment.created_at}</td>
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
