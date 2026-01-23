<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import Drawer from '$lib/components/ui/Drawer.svelte';
	import TimelinePanel from '$lib/components/routes/shipments/timeline/TimelinePanel.svelte';
	import type { TimelineItem } from '$lib/types/shipment';

	let { open, shipmentId, timeline, onClose } = $props<{
		open: boolean;
		shipmentId: string;
		timeline: TimelineItem[];
		onClose: () => void;
	}>();
</script>

<Drawer open={open} class="w-[400px]">
	<div class="flex items-center justify-between px-6 py-5 border-b border-border-dark bg-[#1c1c1f]">
		<div>
			<h3 class="text-lg font-semibold text-white">Event Timeline</h3>
			<p class="text-xs text-muted font-mono mt-0.5">{shipmentId}</p>
		</div>
		<div class="flex items-center gap-2">
			<Button variant="ghost" size="sm" class="p-1" title="Export Log">
				<span class="material-symbols-outlined text-[20px]">download</span>
			</Button>
			<Button variant="ghost" size="sm" class="p-1" onclick={onClose} title="Close">
				<span class="material-symbols-outlined text-[20px]">close</span>
			</Button>
		</div>
	</div>
	<TimelinePanel {timeline} />
	<div class="px-6 py-4 border-t border-border-dark bg-[#1c1c1f]">
		<div class="flex items-center justify-between rounded border border-border-dark bg-zinc-900/30 px-3 py-2">
			<div class="flex items-center gap-2 text-xs font-mono text-muted">
				<span class="material-symbols-outlined text-[16px] text-zinc-500">data_check</span>
				<span>Showing full history</span>
			</div>
			<div class="text-xs font-mono text-zinc-400">
				{timeline.length} events
			</div>
		</div>
	</div>
</Drawer>
