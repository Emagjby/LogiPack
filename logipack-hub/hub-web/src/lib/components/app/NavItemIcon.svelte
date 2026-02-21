<script lang="ts">
	type IconName =
		| "dashboard"
		| "shipments"
		| "clients"
		| "offices"
		| "employees"
		| "audit"
		| "profile";

	let {
		name,
		active = false,
	}: {
		name: IconName;
		active?: boolean;
	} = $props();

	let iconClass = $derived([
		"h-[18px] w-[18px] shrink-0",
		active ? "text-accent" : "text-surface-600 group-hover:text-surface-400",
	]);
</script>

{#if name === "dashboard"}
	<svg
		class={iconClass}
		viewBox="0 0 20 20"
		fill="none"
		stroke="currentColor"
		stroke-width="1.5"
	>
		<rect x="3" y="3" width="6" height="6" rx="1" />
		<rect x="11" y="3" width="6" height="6" rx="1" />
		<rect x="3" y="11" width="6" height="6" rx="1" />
		<rect x="11" y="11" width="6" height="6" rx="1" />
	</svg>
{:else if name === "shipments"}
	<svg
		class={iconClass}
		viewBox="0 0 20 20"
		fill="none"
		stroke="currentColor"
		stroke-width="1.5"
	>
		<path d="M3 7l7-4 7 4v6l-7 4-7-4V7z" />
		<path d="M3 7l7 4m0 0l7-4m-7 4v7" />
	</svg>
{:else if name === "clients"}
	<svg
		class={iconClass}
		viewBox="0 0 20 20"
		fill="none"
		stroke="currentColor"
		stroke-width="1.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	>
		<circle cx="10" cy="7" r="3" />
		<path d="M4 16a6 6 0 0112 0" />
	</svg>
{:else if name === "offices"}
	<svg
		class={iconClass}
		viewBox="0 0 20 20"
		fill="none"
		stroke="currentColor"
		stroke-width="1.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	>
		<path d="M4 17h12" />
		<path d="M6 17V5h8v12" />
		<rect x="8" y="8" width="1.5" height="1.5" rx="0.3" />
		<rect x="10.5" y="8" width="1.5" height="1.5" rx="0.3" />
		<rect x="8" y="11" width="1.5" height="1.5" rx="0.3" />
		<rect x="10.5" y="11" width="1.5" height="1.5" rx="0.3" />
	</svg>
{:else if name === "employees"}
	<svg
		class={iconClass}
		viewBox="0 0 20 20"
		fill="none"
		stroke="currentColor"
		stroke-width="1.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	>
		<circle cx="7" cy="7" r="2.5" />
		<circle cx="13.5" cy="8" r="2" />
		<path d="M2.5 16a4.5 4.5 0 019 0" />
		<path d="M11 16a3.2 3.2 0 016.4 0" />
	</svg>
{:else if name === "audit"}
	<svg
		class={iconClass}
		viewBox="0 0 20 20"
		fill="none"
		stroke="currentColor"
		stroke-width="1.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	>
		<rect x="4" y="3" width="12" height="14" rx="2" />
		<path d="M7 7h6" />
		<path d="M7 10h6" />
		<path d="M7 13h3" />
		<path d="M12.5 13.5l1 1 2-2" />
	</svg>
{:else}
	<svg
		class={iconClass}
		viewBox="0 0 20 20"
		fill="none"
		stroke="currentColor"
		stroke-width="1.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	>
		<circle cx="10" cy="7" r="3" />
		<path d="M4.5 16a5.5 5.5 0 0111 0" />
	</svg>
{/if}
