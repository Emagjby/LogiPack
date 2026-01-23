<script lang="ts">
	import type { HTMLButtonAttributes } from "svelte/elements";

	type Variant = "primary" | "secondary" | "ghost";
	type Size = "sm" | "md";

	let {
		variant = "primary",
		size = "md",
		type = "button",
		class: className = "",
		children,
		el = $bindable<HTMLButtonElement | null>(null),
		...rest
	} = $props<
		{
			variant?: Variant;
			size?: Size;
			type?: HTMLButtonAttributes["type"];
			class?: string;
			children?: import("svelte").Snippet;
			el?: HTMLButtonElement | null;
		} & HTMLButtonAttributes
	>();

	const resolvedVariant = $derived(() => variant as Variant);
	const resolvedSize = $derived(() => size as Size);

	const base =
		"inline-flex items-center justify-center gap-2 rounded text-sm font-medium transition-colors disabled:opacity-50 disabled:pointer-events-none";

	const variants: Record<Variant, string> = {
		primary:
			"bg-primary hover:bg-blue-600 text-white shadow-[0_0_0_1px_rgba(0,0,0,0.2)]",
		secondary:
			"border border-border-dark bg-surface-dark hover:bg-zinc-800 text-white",
		ghost: "text-muted hover:text-white hover:bg-zinc-800",
	};

	const sizes: Record<Size, string> = {
		sm: "h-8 px-3",
		md: "h-9 px-4",
	};
</script>

<button
	bind:this={el}
	{type}
	class={`${base} ${variants[resolvedVariant()]} ${sizes[resolvedSize()]} ${className}`}
	{...rest}
>
	{@render children?.()}
</button>
