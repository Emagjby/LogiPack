<script lang="ts">
	import type { HTMLSelectAttributes } from 'svelte/elements';

	let {
		class: className = '',
		value = $bindable(''),
		children,
		...rest
	} = $props<{
		class?: string;
		value?: string;
		children?: import('svelte').Snippet;
	} & HTMLSelectAttributes>();
</script>

<div class={`relative ${className}`}>
	<select
		class="w-full appearance-none bg-background-dark border border-border-dark rounded-md px-3 py-2 text-sm text-white focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary transition-shadow"
		bind:value
		{...rest}
	>
		{@render children?.()}
	</select>
	<span
		class="material-symbols-outlined absolute right-3 top-2.5 text-muted pointer-events-none text-[18px]"
	>
		expand_more
	</span>
</div>
