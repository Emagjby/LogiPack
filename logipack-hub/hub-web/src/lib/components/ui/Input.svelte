<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';

	let {
		class: className = '',
		value = $bindable(''),
		...rest
	} = $props<{
		class?: string;
		value?: string;
	} & HTMLInputAttributes>();
</script>

<input
	class={`w-full bg-background-dark border border-border-dark rounded-md px-3 py-2 text-sm text-white placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary transition-shadow ${className}`}
	bind:value
	{...rest}
/>
