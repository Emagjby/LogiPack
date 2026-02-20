<script lang="ts">
	import type { PageData } from "./$types";
	import { _ } from "svelte-i18n";

	let { data }: { data: PageData } = $props();
	let lang = $derived(data.pathname.split("/")[1] || "en");
</script>

<div class="flex flex-col items-center justify-center py-20 text-center">
	<span
		class="inline-flex items-center gap-2 rounded-full bg-amber-500/10 px-3 py-1.5 text-sm font-medium text-amber-400"
	>
		<svg
			class="h-4 w-4"
			viewBox="0 0 20 20"
			fill="currentColor"
		>
			<path
				fill-rule="evenodd"
				d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 5a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 5zm0 9a1 1 0 100-2 1 1 0 000 2z"
				clip-rule="evenodd"
			/>
		</svg>
		{$_("admin.in_development")}
	</span>

	<h1 class="mt-6 text-2xl font-bold text-surface-50">
		{$_("admin.headline")}
	</h1>

	<p class="mt-2 max-w-sm text-sm text-surface-400">
		{$_("admin.subheadline")}
	</p>

	<a
		href="/{lang}/app/admin/shipments"
		class="mt-6 cursor-pointer rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors hover:bg-accent-hover"
	>
		{$_("admin.go_to_shipments")}
	</a>
</div>
