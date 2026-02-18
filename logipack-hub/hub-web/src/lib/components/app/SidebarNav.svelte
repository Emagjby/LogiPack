<script lang="ts">
	let { pathname, lang }: { pathname: string; lang: string } = $props();

	interface NavItem {
		label: string;
		href: string;
		icon: "dashboard" | "shipments";
	}

	const navItems: NavItem[] = $derived([
		{
			label: "Dashboard",
			href: `/${lang}/app/employee`,
			icon: "dashboard" as const,
		},
		{
			label: "Shipments",
			href: `/${lang}/app/employee/shipments`,
			icon: "shipments" as const,
		},
	]);

	function isActive(href: string): boolean {
		return (
			pathname === href ||
			(href !== `/${lang}/app/employee` && pathname.startsWith(href))
		);
	}
</script>

<aside
	class="flex h-full w-60 flex-col border-r border-surface-700/50 bg-surface-900"
>
	<!-- Brand block -->
	<div class="px-5 pt-4.25 pb-3.5">
		<div class="flex items-center gap-2.5">
			<img
				src="https://raw.githubusercontent.com/Emagjby/logipack-assets/refs/heads/main/logipack-crate-green.png"
				alt="LogiPack"
				class="h-6 w-6 rounded-sm object-cover"
			/>
			<span class="text-sm font-semibold tracking-tight text-surface-50"
				>LogiPack</span
			>
		</div>
	</div>

	<!-- Divider -->
	<div class="mx-4 h-px bg-surface-800"></div>

	<!-- Navigation section -->
	<nav class="flex-1 overflow-y-auto px-3 py-4">
		<span
			class="mb-2 block px-3 text-[10px] font-medium uppercase tracking-widest text-surface-600"
		>
			Operations
		</span>

		{#each navItems as item (item.href)}
			{@const active = isActive(item.href)}
			<a
				href={item.href}
				class={[
					"group relative flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2 text-[13px] font-medium transition-all duration-150",
					"focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50 focus-visible:ring-offset-1 focus-visible:ring-offset-surface-900",
					active
						? "bg-surface-800/80 text-surface-50"
						: "text-surface-400 hover:bg-surface-800/40 hover:text-surface-200",
				]}
			>
				{#if active}
					<div
						class="absolute left-0 top-1/2 h-4 w-[3px] -translate-y-1/2 rounded-full bg-accent"
					></div>
				{/if}

				{#if item.icon === "dashboard"}
					<svg
						class={[
							"h-[18px] w-[18px] shrink-0",
							active
								? "text-accent"
								: "text-surface-500 group-hover:text-surface-300",
						]}
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
				{:else if item.icon === "shipments"}
					<svg
						class={[
							"h-[18px] w-[18px] shrink-0",
							active
								? "text-accent"
								: "text-surface-500 group-hover:text-surface-300",
						]}
						viewBox="0 0 20 20"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
					>
						<path d="M3 7l7-4 7 4v6l-7 4-7-4V7z" />
						<path d="M3 7l7 4m0 0l7-4m-7 4v7" />
					</svg>
				{/if}

				<span>{item.label}</span>
			</a>
		{/each}
	</nav>

	<!-- Bottom section -->
	<div class="mt-auto px-5 py-3">
		<span class="text-[10px] text-surface-700">v0.1.0</span>
	</div>
</aside>
