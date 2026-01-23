<script lang="ts">
	import TimelineItem from '$lib/components/routes/shipments/timeline/TimelineItem.svelte';
	import type { TimelineItem as TimelineItemType } from '$lib/types/shipment';

	let { timeline } = $props<{ timeline: TimelineItemType[] }>();

	let expandedSequence = $state<number | null>(null);

	function handleToggle(event: CustomEvent<{ seq: number; expanded: boolean }>) {
		if (event.detail.expanded) {
			expandedSequence = event.detail.seq;
		} else {
			expandedSequence = null;
		}
	}
</script>

<div class="flex-1 overflow-y-auto p-6">
	<div class="relative">
		{#each timeline as item, index (item.seq)}
			<TimelineItem
				{item}
				index={index}
				isLast={index === timeline.length - 1}
				isNewest={index === 0}
				expanded={expandedSequence === item.seq}
				on:toggle={handleToggle}
			/>
		{/each}
	</div>
</div>
