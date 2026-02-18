<script lang="ts">
	import type { LayoutData } from "./$types";
	import type { Snippet } from "svelte";
	import { page } from "$app/state";
	import SidebarNav from "$lib/components/app/SidebarNav.svelte";
	import Topbar from "$lib/components/app/Topbar.svelte";

	let { data, children }: { data: LayoutData; children: Snippet } = $props();

	let lang = $derived(data.pathname.split("/")[1] || "en");
	let hasRole = $derived(!!data.session?.role);
	let showShell = $derived(hasRole && !page.error);
</script>

{#if showShell}
	<div class="flex h-screen overflow-hidden bg-surface-950">
		<SidebarNav pathname={data.pathname} {lang} />

		<div class="flex min-h-0 min-w-0 flex-1 flex-col">
			<Topbar pathname={data.pathname} session={data.session} {lang} />

			<main class="flex-1 overflow-y-auto p-6">
				<div class="mx-auto max-w-6xl">
					{@render children()}
				</div>
			</main>
		</div>
	</div>
{:else}
	<div class="flex h-screen items-center justify-center bg-surface-950">
		{@render children()}
	</div>
{/if}
