<script lang="ts">
	import type { PageData } from "./$types";
	import { _ } from "svelte-i18n";

	let { data }: { data: PageData } = $props();
	let userEmail = $derived(data.session?.email ?? "unknown");

	function logout() {
		window.location.href = "/logout";
	}
</script>

<div class="flex flex-col items-center py-16 text-center">
	<!-- Icon badge -->
	<div
		class="flex h-14 w-14 items-center justify-center rounded-full border border-surface-800 bg-surface-900"
	>
		<svg
			class="h-7 w-7 text-surface-400"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
		>
			<path
				d="M12 2l8 4v6c0 5.25-3.5 9.74-8 11-4.5-1.26-8-5.75-8-11V6l8-4z"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
			<path
				d="M12 9v4m0 3h.01"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
		</svg>
	</div>

	<!-- Title -->
	<h1 class="mt-5 text-xl font-semibold text-surface-50">
		{$_("no-access.headline")}
	</h1>

	<!-- Body -->
	<p class="mt-2 max-w-sm text-sm text-surface-400">
		{$_("no-access.subheadline")}
	</p>

	<!-- Email pill -->
	<span
		class="mt-4 inline-flex items-center gap-2 rounded-full border border-surface-700/50 bg-surface-800 px-3 py-1.5 text-xs text-surface-400"
	>
		<svg
			class="h-3.5 w-3.5 text-surface-500"
			viewBox="0 0 20 20"
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
		>
			<circle cx="10" cy="7" r="4" />
			<path d="M3 18c0-3.3 2.7-6 7-6s7 2.7 7 6" />
		</svg>
		{userEmail}
	</span>

	<!-- Help note -->
	<p class="mt-3 text-[11px] text-surface-600">
		{$_("no-access.hint")}
	</p>

	<!-- Logout -->
	<button
		onclick={logout}
		class="mt-6 rounded-md border border-surface-700 bg-surface-800 px-4 py-2 text-xs font-medium text-surface-400 transition-colors duration-150 hover:bg-surface-700 hover:text-surface-200 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50 cursor-pointer"
	>
		{$_("logout")}
	</button>
</div>
