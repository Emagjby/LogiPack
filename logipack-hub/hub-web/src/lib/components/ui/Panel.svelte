<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';

	let { class: className = '', children, ...rest } = $props<{
		class?: string;
		children?: import('svelte').Snippet;
	} & HTMLAttributes<HTMLDivElement>>();
</script>

<div
	class={`border border-border-dark rounded-lg overflow-hidden bg-surface-dark shadow-sm ${className}`}
	{...rest}
>
	{@render children?.()}
</div>
