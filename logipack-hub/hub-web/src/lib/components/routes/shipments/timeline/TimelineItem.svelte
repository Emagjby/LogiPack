<script lang="ts">
	import type { TimelineItem } from '$lib/types/shipment';
	import { createEventDispatcher } from 'svelte';

	let { item, expanded = $bindable(false), index, isLast, isNewest } = $props<{
		item: TimelineItem;
		expanded?: boolean;
		index: number;
		isLast: boolean;
		isNewest: boolean;
	}>();

	const dispatch = createEventDispatcher();

	function toggleExpanded() {
		expanded = !expanded;
		dispatch('toggle', { seq: item.seq, expanded });
	}

	function copyPayload() {
		if (item.scb) {
			navigator.clipboard.writeText(item.scb);
		}
	}
</script>

<div class="timeline-item group" class:timeline-item--expanded={expanded}>
	<div class="timeline-rail" aria-hidden="true">
		{#if index !== 0}
			<div class="timeline-segment timeline-segment--top"></div>
		{/if}
		<div class="timeline-node">
			<div
				class={`node-badge ${
					isNewest
						? 'bg-blue-500/10 border-blue-500/35 text-blue-400 shadow-blue-500/20'
							: expanded
								? 'bg-zinc-800 border-zinc-500 text-white shadow-zinc-600/20'
								: 'bg-surface-dark border-border-dark text-muted group-hover:text-white group-hover:border-zinc-600'
				}`}
			>
						#{item.seq}
			</div>
		</div>
		{#if !isLast}
			<div class="timeline-segment timeline-segment--bottom" class:timeline-segment--muted={expanded}></div>
		{/if}
	</div>

	<div class="timeline-content">
		<button
			type="button"
			class={`event-header ${
				expanded
					? 'bg-zinc-800/35 border-l-2 border-blue-500/90'
					: 'hover:bg-zinc-800/20 border-l-2 border-transparent'
			}`}
			onclick={toggleExpanded}
		>
			<div class="flex items-center justify-between py-2">
				<div class="flex items-center gap-3">
					<span
						class={`text-sm font-bold transition-colors ${
							expanded
									? 'text-white'
									: 'text-zinc-300 group-hover:text-white'
						}`}
					>
						{item.event_type}
					</span>
					{#if item.scb}
						<span
							class="material-symbols-outlined text-[16px] text-zinc-500 transition-transform duration-200"
							class:rotate-180={expanded}
						>
							expand_more
						</span>
					{/if}
				</div>
				<span class="text-xs font-mono text-zinc-500">seq {item.seq}</span>
			</div>

			<div class="flex items-center gap-3 text-xs text-zinc-500 pb-2">
				<span class="flex items-center gap-1.5">
					<span class="material-symbols-outlined text-[14px]">bolt</span>
					{item.event_type}
				</span>
			</div>
		</button>

		{#if item.scb}
			<div
				class="payload-wrap"
				style="max-height: {expanded ? '520px' : '0'}; opacity: {expanded ? '1' : '0'};"
				aria-hidden={!expanded}
			>
				<div class="payload-lane" class:payload-lane--expanded={expanded}>
					<div class="payload-connector" aria-hidden="true"></div>
					<div class="payload-panel">
						<div class="flex items-center justify-between px-3 py-2 border-b border-zinc-700/50 bg-zinc-800/30">
							<span class="text-[10px] uppercase font-semibold text-zinc-500 tracking-wider">
								SCB (base64)
							</span>
							<button
								type="button"
								class="text-[10px] flex items-center gap-1 text-zinc-500 hover:text-zinc-300 transition-colors"
								onclick={copyPayload}
							>
								<span class="material-symbols-outlined text-[12px]">content_copy</span>
								Copy
							</button>
						</div>
						<div class="p-3 font-mono text-xs text-zinc-400 overflow-x-auto whitespace-pre leading-relaxed bg-zinc-900/30">
							{item.scb}
						</div>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.rotate-180 {
		transform: rotate(180deg);
	}

	.timeline-item {
		display: grid;
		grid-template-columns: 44px minmax(0, 1fr);
		column-gap: 12px;
		padding: 6px 0;
	}

	.timeline-rail {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.timeline-node {
		position: relative;
		z-index: 2;
		display: flex;
		align-items: center;
		justify-content: center;
		padding-top: 2px;
		padding-bottom: 2px;
	}

	.node-badge {
		z-index: 2;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 38px;
		height: 24px;
		border-radius: 6px;
		border-width: 1px;
		font-size: 10px;
		font-weight: 600;
		font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New",
			monospace;
		box-shadow: 0 1px 0 rgba(0, 0, 0, 0.5);
		transition: background-color 200ms, border-color 200ms, color 200ms, box-shadow 200ms;
	}

	.timeline-segment {
		width: 1px;
		background: linear-gradient(to bottom, rgba(113, 113, 122, 0.85), rgba(82, 82, 91, 0.85));
	}

	.timeline-segment--top {
		flex: 1 1 auto;
		min-height: 10px;
	}

	.timeline-segment--bottom {
		flex: 1 1 auto;
		min-height: 14px;
		transition: opacity 220ms ease;
	}

	.timeline-segment--muted {
		opacity: 0.22;
	}

	.timeline-content {
		min-width: 0;
	}

	.event-header {
		display: block;
		width: 100%;
		text-align: left;
		border-radius: 0 8px 8px 0;
		margin-left: -8px;
		padding-left: 8px;
		transition: background-color 200ms, border-color 200ms;
	}

	.event-header:focus-visible {
		outline: 2px solid rgba(59, 130, 246, 0.6);
		outline-offset: 2px;
	}

	.payload-wrap {
		overflow: hidden;
		transition: max-height 260ms ease, opacity 220ms ease;
		will-change: max-height, opacity;
	}

	.payload-lane {
		position: relative;
		margin-top: 8px;
		margin-left: 14px;
		padding-left: 18px;
		padding-bottom: 8px;
	}

	.payload-connector {
		position: absolute;
		left: 0;
		top: 16px;
		width: 14px;
		height: 1px;
		background: rgba(82, 82, 91, 0.9);
	}

	.payload-connector::before {
		content: '';
		position: absolute;
		left: -3px;
		top: -2px;
		width: 5px;
		height: 5px;
		border-radius: 999px;
		background: rgba(82, 82, 91, 0.9);
	}

	.payload-panel {
		background: rgba(9, 9, 11, 0.35);
		border: 1px solid rgba(82, 82, 91, 0.55);
		border-radius: 10px;
		overflow: hidden;
		box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.04);
	}

	.payload-lane--expanded .payload-panel {
		background: rgba(24, 24, 27, 0.25);
		border-color: rgba(59, 130, 246, 0.22);
	}

	.timeline-item--expanded .event-header {
		background: rgba(39, 39, 42, 0.35);
	}
</style>
