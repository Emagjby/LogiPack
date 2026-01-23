<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';

	type Placement = 'top' | 'bottom';
	type Anchor = {
		top: number;
		left: number;
		placement: Placement;
	};

	let { open, anchor, onClose, onTimeline, onStatus } = $props<{
		open: boolean;
		anchor: Anchor | null;
		onClose: () => void;
		onTimeline: () => void;
		onStatus: () => void;
	}>();
</script>

{#if open && anchor}
	<div
		class="fixed z-[2147483647]"
		style={`top:${anchor.top}px; left:${anchor.left}px;`}
	>
		<div
			class={`min-w-44 rounded-lg border border-border-dark bg-[#141417] shadow-2xl ring-1 ring-black/40 p-1 ${
				anchor.placement === 'top' ? '-translate-y-full' : ''
			}`}
			role="menu"
			aria-label="Shipment actions"
			onpointerdown={(event) => event.stopPropagation()}
			onclick={(event) => event.stopPropagation()}
			onkeydown={(event) => event.stopPropagation()}
			tabindex="-1"
		>
			<Button
				variant="ghost"
				size="sm"
				class="w-full justify-start px-2.5 py-2 text-left rounded-md hover:bg-zinc-800/70"
				role="menuitem"
				onclick={() => {
					onTimeline();
					onClose();
				}}
			>
				<span class="material-symbols-outlined text-[18px]">history</span>
				<span>Timeline</span>
			</Button>
			<Button
				variant="ghost"
				size="sm"
				class="w-full justify-start px-2.5 py-2 text-left rounded-md hover:bg-zinc-800/70"
				role="menuitem"
				onclick={() => {
					onStatus();
					onClose();
				}}
			>
				<span class="material-symbols-outlined text-[18px]">swap_horiz</span>
				<span>Change status</span>
			</Button>
		</div>
	</div>
{/if}
