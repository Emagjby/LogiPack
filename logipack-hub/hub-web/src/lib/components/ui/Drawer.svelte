<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { slide } from 'svelte/transition';

	let {
		open = false,
		class: className = "",
		children,
		...rest
	} = $props<
		{
			open?: boolean;
			class?: string;
			children?: import("svelte").Snippet;
		} & HTMLAttributes<HTMLElement>
	>();
</script>

{#if open}
	<aside
		transition:slide={{ duration: 300, axis: 'x' }}
		class={`border-l h-[100%] border-border-dark bg-surface-dark flex flex-col shadow-2xl z-30 relative shrink-0 ${className}`}
		{...rest}
	>
		{@render children?.()}
	</aside>
{/if}
