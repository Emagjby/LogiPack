<script lang="ts">
	import Panel from '$lib/components/ui/Panel.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import ShipmentRow from '$lib/components/routes/shipments/components/ShipmentRow.svelte';
	import type { ShipmentListItem } from '$lib/types/shipment';

	let {
		shipments,
		selectedId,
		onSelect,
		onAction,
		onTimeline,
		onStatus,
		onClose,
		page = 1,
		totalPages = 1,
		showPagination = false,
		onPageChange
	} = $props<{
		shipments: ShipmentListItem[];
		selectedId?: string;
		onSelect: (id: string) => void;
		onAction: (id: string) => void;
		onTimeline: (id: string) => void;
		onStatus: (id: string) => void;
		onClose: () => void;
		page?: number;
		totalPages?: number;
		showPagination?: boolean;
		onPageChange?: (next: number) => void;
	}>();
</script>

<Panel class="flex-1 flex flex-col">
	<div class="overflow-auto flex-1">
		<Table>
			<thead class="bg-[#1c1c1f] sticky top-0 z-10">
				<tr>
					<th
						class="px-4 py-3 text-xs font-semibold text-muted uppercase tracking-wider border-b border-border-dark w-[140px]"
					>
						Shipment ID
					</th>
					<th
						class="px-4 py-3 text-xs font-semibold text-muted uppercase tracking-wider border-b border-border-dark w-[160px]"
					>
						Status
					</th>
					<th
						class="px-4 py-3 text-xs font-semibold text-muted uppercase tracking-wider border-b border-border-dark"
					>
						Current Office
					</th>
					<th
						class="px-4 py-3 text-xs font-semibold text-muted uppercase tracking-wider border-b border-border-dark"
					>
						Client
					</th>
					<th
						class="px-4 py-3 text-xs font-semibold text-muted uppercase tracking-wider border-b border-border-dark text-right"
					>
						Created At
					</th>
					<th class="px-4 py-3 border-b border-border-dark w-10"></th>
				</tr>
			</thead>
			<tbody class="divide-y divide-border-dark">
				{#each shipments as shipment (shipment.id)}
					<ShipmentRow
						{shipment}
						selected={selectedId === shipment.id}
						onSelect={onSelect}
						onAction={onAction}
						onTimeline={onTimeline}
						onStatus={onStatus}
						onClose={onClose}
					/>
				{/each}
			</tbody>
		</Table>
	</div>
	{#if showPagination}
		<div class="bg-[#1c1c1f] px-4 py-2 border-t border-border-dark flex items-center justify-between">
			<span class="text-xs text-muted">Page {page} of {totalPages}</span>
			<div class="flex items-center gap-2">
				<button
					class="p-1 rounded text-muted hover:text-white disabled:opacity-50 hover:bg-zinc-800"
					disabled={page <= 1}
					onclick={() => onPageChange?.(page - 1)}
				>
					<span class="material-symbols-outlined text-[18px]">chevron_left</span>
				</button>
				<button
					class="p-1 rounded text-muted hover:text-white disabled:opacity-50 hover:bg-zinc-800"
					disabled={page >= totalPages}
					onclick={() => onPageChange?.(page + 1)}
				>
					<span class="material-symbols-outlined text-[18px]">chevron_right</span>
				</button>
			</div>
		</div>
	{/if}
</Panel>
