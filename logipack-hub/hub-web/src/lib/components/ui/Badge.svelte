<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";

	let {
		class: className = "",
		children,
		...rest
	} = $props<
		{
			class?: string;
			children?: import("svelte").Snippet;
		} & HTMLAttributes<HTMLSpanElement>
	>();
</script>

<span
	class={`inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-medium ${className}`}
	{...rest}
>
	{@render children?.()}
</span>
