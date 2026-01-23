<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';

	let { class: className = '', children, ...rest } = $props<{
		class?: string;
		children?: import('svelte').Snippet;
	} & HTMLAttributes<HTMLTableElement>>();
</script>

<table class={`w-full text-left border-collapse ${className}`} {...rest}>
	{@render children?.()}
</table>
