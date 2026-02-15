<script lang="ts">
	import type { PageProps } from "./$types";

	let { data }: PageProps = $props();

	// Hardcoded trusted SVG strings — safe for {@html}
	const featureIcons: Record<string, string> = {
		timeline: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>`,
		office: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="2" width="16" height="20" rx="2"/><line x1="9" y1="6" x2="9" y2="6.01"/><line x1="15" y1="6" x2="15" y2="6.01"/><line x1="9" y1="10" x2="9" y2="10.01"/><line x1="15" y1="10" x2="15" y2="10.01"/><line x1="9" y1="14" x2="9" y2="14.01"/><line x1="15" y1="14" x2="15" y2="14.01"/><line x1="9" y1="18" x2="15" y2="18"/></svg>`,
		roles: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><path d="M12 8v4"/><circle cx="12" cy="16" r="0.5"/></svg>`,
		admin: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09a1.65 1.65 0 0 0-1.08-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09a1.65 1.65 0 0 0 1.51-1.08 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1.08z"/></svg>`,
	};

	const principles = [
		"Append-only timeline",
		"Role-based access",
		"Data-dense UI",
		"SSR-first",
	];

	const stepPills: Record<number, { label: string; color: string }> = {
		1: { label: "Created", color: "bg-blue-500/10 text-blue-400" },
		2: { label: "In Transit", color: "bg-amber-500/10 text-amber-400" },
		3: { label: "Audited", color: "bg-accent/10 text-accent" },
	};
</script>

<!-- Skip link -->
<a
	href="#main"
	class="sr-only focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-[60] focus:rounded-lg focus:bg-accent focus:px-4 focus:py-2 focus:text-surface-950 focus:font-semibold focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-surface-950"
>
	Skip to main content
</a>

<!-- Topbar -->
<header class="fixed top-4 left-1/2 -translate-x-1/2 w-[calc(100%-2rem)] z-50">
	<nav
		aria-label="Primary navigation"
		class="mx-auto flex md:max-w-6xl items-center justify-between rounded-xl border border-white/10 bg-surface-900/80 px-5 py-3 backdrop-blur-md"
	>
		<span class="text-lg font-bold text-surface-50">LogiPack</span>
		<a
			href={data.loginUrl}
			class="cursor-pointer rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors duration-200 hover:bg-accent-hover focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-surface-950"
		>
			Log in
		</a>
	</nav>
</header>

<main id="main" class="relative min-h-dvh overflow-x-clip bg-surface-950">
	<!-- Glow container -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden">
		<div
			class="absolute -top-32 -left-32 h-125 w-125 rounded-full bg-accent/5 blur-3xl"
		></div>
		<div
			class="absolute -right-32 -bottom-32 h-125 w-125 rounded-full bg-blue-500/5 blur-3xl"
		></div>
	</div>

	<!-- Hero -->
	<section aria-label="Hero" class="relative overflow-hidden pt-28 pb-16">
		<div
			class="mx-auto max-w-6xl px-6 lg:grid lg:grid-cols-2 lg:items-center lg:gap-12"
		>
			<!-- Left column -->
			<div>
				<h1
					class="text-4xl font-bold leading-tight text-surface-50 md:text-5xl"
				>
					Shipment tracking with a full
					<span
						class="decoration-accent/40 underline underline-offset-4 decoration-2"
						>audit timeline</span
					>.
				</h1>
				<p class="mt-4 max-w-lg text-lg text-surface-400">
					LogiPack is an internal operations console for logistics
					teams. Track shipments across offices, manage access by
					role, and inspect every change in a chronological timeline.
				</p>
				<div class="mt-8 flex items-center gap-4">
					<a
						href={data.loginUrl}
						class="cursor-pointer rounded-lg bg-accent px-6 py-3 font-semibold text-surface-950 transition-colors duration-200 hover:bg-accent-hover focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-surface-950"
					>
						Log in
					</a>
					<a
						href="#features"
						class="cursor-pointer text-surface-400 underline underline-offset-4 transition-colors duration-200 hover:text-surface-50 focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-surface-950"
					>
						See features
					</a>
				</div>
			</div>

			<!-- Right column — Product Preview Mock -->
			<div class="relative mt-10 lg:mt-0" aria-hidden="true">
				<!-- Subtle glow behind the preview -->
				<div
					class="absolute -inset-4 rounded-2xl bg-accent/5 blur-2xl"
				></div>

				<!-- Preview frame -->
				<div
					class="relative overflow-hidden rounded-2xl border border-white/10 bg-surface-900/80 shadow-2xl shadow-black/20 backdrop-blur-sm"
				>
					<!-- Window chrome bar -->
					<div
						class="flex items-center gap-2 border-b border-white/5 bg-surface-900 px-4 py-2.5"
					>
						<div class="flex gap-1.5">
							<div
								class="h-2.5 w-2.5 rounded-full bg-surface-700"
							></div>
							<div
								class="h-2.5 w-2.5 rounded-full bg-surface-700"
							></div>
							<div
								class="h-2.5 w-2.5 rounded-full bg-surface-700"
							></div>
						</div>
						<div class="ml-3 flex-1">
							<div
								class="mx-auto max-w-[200px] rounded-md bg-surface-800 px-3 py-1 text-center text-xs text-surface-600"
							>
								app.logipack.io/shipments
							</div>
						</div>
					</div>

					<!-- Search bar -->
					<div class="border-b border-white/5 px-4 py-3">
						<div
							class="flex items-center gap-2 rounded-lg border border-white/10 bg-surface-950/50 px-3 py-2"
						>
							<svg
								class="h-4 w-4 text-surface-600"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
								><circle cx="11" cy="11" r="8" /><line
									x1="21"
									y1="21"
									x2="16.65"
									y2="16.65"
								/></svg
							>
							<span class="text-xs text-surface-600"
								>Search shipments...</span
							>
						</div>
					</div>

					<!-- Mini shipments table -->
					<div class="px-4 pt-3 pb-1">
						<table class="w-full text-xs">
							<thead>
								<tr class="text-left text-surface-600">
									<th class="pb-2 pr-3 font-medium">ID</th>
									<th class="pb-2 pr-3 font-medium">Status</th
									>
									<th
										class="hidden pb-2 pr-3 font-medium sm:table-cell"
										>Office</th
									>
									<th class="pb-2 font-medium text-right"
										>Updated</th
									>
								</tr>
							</thead>
							<tbody class="text-surface-400">
								<tr class="border-t border-white/5">
									<td
										class="py-2 pr-3 font-mono text-surface-50"
										>SHP-2847</td
									>
									<td class="py-2 pr-3">
										<span
											class="inline-flex items-center rounded-full bg-amber-500/10 px-2 py-0.5 text-[11px] font-medium text-amber-400"
											>In Transit</span
										>
									</td>
									<td class="hidden py-2 pr-3 sm:table-cell"
										>Sofia HQ</td
									>
									<td class="py-2 text-right text-surface-600"
										>2 min ago</td
									>
								</tr>
								<tr class="border-t border-white/5">
									<td
										class="py-2 pr-3 font-mono text-surface-50"
										>SHP-2846</td
									>
									<td class="py-2 pr-3">
										<span
											class="inline-flex items-center rounded-full bg-accent/10 px-2 py-0.5 text-[11px] font-medium text-accent"
											>Delivered</span
										>
									</td>
									<td class="hidden py-2 pr-3 sm:table-cell"
										>Plovdiv DC</td
									>
									<td class="py-2 text-right text-surface-600"
										>18 min ago</td
									>
								</tr>
								<tr class="border-t border-white/5">
									<td
										class="py-2 pr-3 font-mono text-surface-50"
										>SHP-2845</td
									>
									<td class="py-2 pr-3">
										<span
											class="inline-flex items-center rounded-full bg-blue-500/10 px-2 py-0.5 text-[11px] font-medium text-blue-400"
											>Processing</span
										>
									</td>
									<td class="hidden py-2 pr-3 sm:table-cell"
										>Varna Port</td
									>
									<td class="py-2 text-right text-surface-600"
										>1 hr ago</td
									>
								</tr>
								<tr class="border-t border-white/5">
									<td
										class="py-2 pr-3 font-mono text-surface-50"
										>SHP-2844</td
									>
									<td class="py-2 pr-3">
										<span
											class="inline-flex items-center rounded-full bg-red-500/10 px-2 py-0.5 text-[11px] font-medium text-red-400"
											>On Hold</span
										>
									</td>
									<td class="hidden py-2 pr-3 sm:table-cell"
										>Sofia HQ</td
									>
									<td class="py-2 text-right text-surface-600"
										>3 hrs ago</td
									>
								</tr>
							</tbody>
						</table>
					</div>

					<!-- Mini timeline panel -->
					<div class="border-t border-white/5 px-4 py-3">
						<div class="mb-2 flex items-center gap-2">
							<span
								class="text-[11px] font-medium uppercase tracking-wider text-surface-600"
								>Timeline</span
							>
							<span
								class="rounded bg-surface-800 px-1.5 py-0.5 text-[10px] text-surface-600"
								>SHP-2847</span
							>
						</div>
						<div class="space-y-2">
							<div class="flex items-start gap-2.5">
								<div
									class="mt-1 h-1.5 w-1.5 shrink-0 rounded-full bg-accent"
								></div>
								<div
									class="flex flex-1 items-baseline justify-between gap-2"
								>
									<span class="text-xs text-surface-400"
										>Status changed to
										<span class="text-surface-200"
											>In Transit</span
										></span
									>
									<span
										class="shrink-0 text-[11px] text-surface-600"
										>14:32</span
									>
								</div>
							</div>
							<div class="flex items-start gap-2.5">
								<div
									class="mt-1 h-1.5 w-1.5 shrink-0 rounded-full bg-surface-600"
								></div>
								<div
									class="flex flex-1 items-baseline justify-between gap-2"
								>
									<span class="text-xs text-surface-400"
										>Assigned to <span
											class="text-surface-200"
											>Sofia HQ</span
										></span
									>
									<span
										class="shrink-0 text-[11px] text-surface-600"
										>14:30</span
									>
								</div>
							</div>
							<div class="flex items-start gap-2.5">
								<div
									class="mt-1 h-1.5 w-1.5 shrink-0 rounded-full bg-surface-600"
								></div>
								<div
									class="flex flex-1 items-baseline justify-between gap-2"
								>
									<span class="text-xs text-surface-400"
										>Shipment <span class="text-surface-200"
											>created</span
										> by M. Ivanov</span
									>
									<span
										class="shrink-0 text-[11px] text-surface-600"
										>14:28</span
									>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Principles strip -->
	<div class="mx-auto max-w-6xl px-6 pb-12">
		<div class="flex flex-wrap items-center justify-center gap-3">
			{#each principles as principle (principle)}
				<span
					class="rounded-full border border-white/5 bg-surface-900/40 px-4 py-1.5 text-xs text-surface-400"
				>
					{principle}
				</span>
			{/each}
		</div>
	</div>

	<!-- Features -->
	<section
		id="features"
		aria-label="Features"
		class="mx-auto max-w-6xl px-6 pb-16"
	>
		<div class="mb-8">
			<h2 class="text-sm uppercase tracking-widest text-surface-400">
				What you get
			</h2>
			<p class="mt-2 text-surface-600 text-sm">
				Four pillars of the LogiPack operations console.
			</p>
		</div>

		<!-- Featured card (Shipment timeline) — full width -->
		{#if data.features.length > 0}
			{@const featured = data.features[0]}
			{@const featuredIcon = featureIcons[featured.icon] ?? ""}
			<div
				class="group relative mb-4 overflow-hidden rounded-xl border border-white/10 bg-surface-900/50 p-6 transition-all duration-200 hover:-translate-y-0.5 hover:border-accent/20 hover:shadow-lg hover:shadow-accent/5 md:flex md:items-start md:gap-6 md:p-8"
			>
				<div
					class="mb-4 flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-accent/10 text-accent md:mb-0"
				>
					{@html featuredIcon}
				</div>
				<div class="flex-1">
					<div class="flex flex-wrap items-center gap-2">
						<h3 class="text-lg font-semibold text-surface-50">
							{featured.title}
						</h3>
						<span
							class="rounded-full bg-accent/10 px-2.5 py-0.5 text-[11px] font-medium text-accent"
							>Core feature</span
						>
					</div>
					<p class="mt-1.5 text-sm leading-relaxed text-surface-400">
						{featured.description}
					</p>
				</div>
			</div>
		{/if}

		<!-- Remaining 3 cards — 3-column grid on md -->
		<div class="grid gap-4 sm:grid-cols-2 md:grid-cols-3">
			{#each data.features.slice(1) as feature (feature.icon)}
				{@const iconSvg = featureIcons[feature.icon] ?? ""}
				<div
					class="group relative overflow-hidden rounded-xl border border-white/10 bg-surface-900/50 p-6 transition-all duration-200 hover:-translate-y-0.5 hover:border-white/20 hover:shadow-lg hover:shadow-accent/5"
				>
					<div
						class="mb-3 flex h-9 w-9 items-center justify-center rounded-lg bg-surface-800 text-accent"
					>
						{@html iconSvg}
					</div>
					<h3 class="font-semibold text-surface-50">
						{feature.title}
					</h3>
					<p class="mt-1 text-sm text-surface-400">
						{feature.description}
					</p>
				</div>
			{/each}
		</div>
	</section>

	<!-- How it works -->
	<section
		aria-label="How it works"
		class="mx-auto max-w-6xl px-6 pt-10 pb-16"
	>
		<div class="mb-8">
			<h2 class="text-sm uppercase tracking-widest text-surface-400">
				How it works
			</h2>
			<p class="mt-2 text-sm text-surface-600">
				Three steps from intake to accountability.
			</p>
		</div>

		<!-- Steps with connector -->
		<div class="relative">
			<div class="grid gap-6 md:grid-cols-3">
				{#each data.steps as step (step.number)}
					{@const pill = stepPills[step.number]}
					<div
						class="group relative rounded-xl border border-white/10 bg-surface-900/50 p-6 transition-all duration-200 hover:-translate-y-0.5 hover:border-white/20 hover:shadow-lg hover:shadow-accent/5"
					>
						<!-- Step number badge -->
						<div
							class="mb-4 flex h-9 w-9 items-center justify-center rounded-full border border-accent/20 bg-accent/10 text-sm font-bold text-accent"
						>
							{step.number}
						</div>

						<h3 class="font-semibold text-surface-50">
							{step.title}
						</h3>
						<p class="mt-1 text-sm text-surface-400">
							{step.description}
						</p>

						{#if pill}
							<span
								class="mt-3 inline-flex items-center rounded-full px-2.5 py-0.5 text-[11px] font-medium {pill.color}"
							>
								{pill.label}
							</span>
						{/if}
					</div>
				{/each}
			</div>
		</div>

		<!-- Mini example timeline -->
		<div class="mt-8" aria-hidden="true">
			<div
				class="overflow-hidden rounded-xl border border-white/10 bg-surface-900/50 px-5 py-4"
			>
				<div class="mb-3 flex items-center gap-2">
					<span
						class="text-[11px] font-medium uppercase tracking-wider text-surface-600"
						>Example flow</span
					>
					<span
						class="rounded bg-surface-800 px-1.5 py-0.5 text-[10px] text-surface-600"
						>SHP-2617e317...</span
					>
				</div>
				<div
					class="flex flex-col md:flex-row md:items-center md:justify-between md:px-12"
				>
					<!-- Event 1 -->
					<div class="flex items-center gap-2.5 md:flex-1">
						<div
							class="h-2 w-2 shrink-0 rounded-full bg-blue-400"
						></div>
						<div>
							<span class="text-xs text-surface-50">Created</span>
							<span class="ml-1.5 text-[11px] text-surface-600"
								>14:28</span
							>
						</div>
					</div>

					<!-- Connector arrow (desktop) -->
					<div
						class="hidden shrink-0 px-2 text-surface-700 md:block"
						aria-hidden="true"
					>
						<svg
							class="h-4 w-4"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<line x1="5" y1="12" x2="19" y2="12" />
							<polyline points="12 5 19 12 12 19" />
						</svg>
					</div>

					<!-- Event 2 -->
					<div class="flex items-center gap-2.5 md:flex-1">
						<div
							class="h-2 w-2 shrink-0 rounded-full bg-amber-400"
						></div>
						<div>
							<span class="text-xs text-surface-50"
								>In Transit</span
							>
							<span class="ml-1.5 text-[11px] text-surface-600"
								>14:32</span
							>
						</div>
					</div>

					<!-- Connector arrow (desktop) -->
					<div
						class="hidden shrink-0 px-2 text-surface-700 md:block"
						aria-hidden="true"
					>
						<svg
							class="h-4 w-4"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<line x1="5" y1="12" x2="19" y2="12" />
							<polyline points="12 5 19 12 12 19" />
						</svg>
					</div>

					<!-- Event 3 -->
					<div class="flex items-center gap-2.5">
						<div
							class="h-2 w-2 shrink-0 rounded-full bg-accent"
						></div>
						<div>
							<span class="text-xs text-surface-50"
								>Delivered</span
							>
							<span class="ml-1.5 text-[11px] text-surface-600"
								>15:10</span
							>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>
</main>

<!-- Footer -->
<footer class="border-t border-surface-800 bg-surface-950">
	<div class="mx-auto flex max-w-6xl items-center justify-between px-6 py-6">
		<span class="text-sm text-surface-600">&copy; 2026 LogiPack</span>
		<div class="flex gap-4">
			<a
				href="/privacy"
				class="cursor-pointer text-sm text-surface-600 transition-colors duration-200 hover:text-surface-400 focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-surface-950"
			>
				Privacy
			</a>
			<a
				href="/terms"
				class="cursor-pointer text-sm text-surface-600 transition-colors duration-200 hover:text-surface-400 focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-surface-950"
			>
				Terms
			</a>
		</div>
	</div>
</footer>

<style>
	@media (prefers-reduced-motion: reduce) {
		:global(*) {
			transition-duration: 0.01ms !important;
			animation-duration: 0.01ms !important;
		}
	}

	:global(html) {
		scroll-behavior: smooth;
		overscroll-behavior: none;
		overflow-x: hidden;
	}

	main {
		background-image: linear-gradient(
				to right,
				rgba(148, 163, 184, 0.03) 1px,
				transparent 1px
			),
			linear-gradient(
				to bottom,
				rgba(148, 163, 184, 0.03) 1px,
				transparent 1px
			);
		background-size: 40px 40px;
	}
</style>
