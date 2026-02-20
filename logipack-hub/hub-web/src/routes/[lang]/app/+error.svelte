<script lang="ts">
	import { page } from "$app/state";
	import { _ } from "svelte-i18n";

	let status = $derived(page.status);
	let lang = $derived(page.params.lang ?? "en");
	let appHref = $derived(`/${lang}/app`);
	let titleKey = $derived(status === 403 ? "forbidden.subheadline" : "error.headline");
	let errorMessage = $derived(page.error?.message ?? "");
	let detailKey = $derived.by(() => {
		if (errorMessage.startsWith("error.details.")) {
			return errorMessage;
		}

		if (status === 403) {
			return "error.details.employee_only";
		}

		return null;
	});

	function reload() {
		location.reload();
	}
</script>

<div class="mx-auto flex min-h-[60vh] w-full max-w-2xl items-center justify-center px-4 py-8">
	<section
		class="w-full rounded-xl border border-surface-700/50 bg-surface-900 p-8 shadow-lg shadow-black/20"
	>
		<p
			class="inline-flex items-center rounded-full border border-red-500/20 bg-red-500/10 px-2.5 py-1 text-xs font-medium uppercase tracking-wide text-red-400"
		>
			{$_("error.status", { values: { status } })}
		</p>
		<h1 class="mt-2 text-2xl font-semibold text-surface-50">{$_(titleKey)}</h1>

		{#if errorMessage && !detailKey && errorMessage !== $_(titleKey)}
			<p class="mt-3 text-sm text-surface-400">{errorMessage}</p>
		{/if}

		<p class="mt-3 text-sm text-surface-400">
			{detailKey ? $_(detailKey) : $_("error.helper")}
		</p>

		<div class="mt-6 flex flex-wrap gap-3">
			<button
				type="button"
				onclick={reload}
				class="inline-flex cursor-pointer items-center rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors duration-150 hover:bg-accent-hover focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
			>
				{$_("error.reload")}
			</button>

			<a
				href={appHref}
				class="inline-flex cursor-pointer items-center rounded-lg border border-surface-700 bg-surface-800 px-4 py-2 text-sm font-semibold text-surface-200 transition-colors duration-150 hover:bg-surface-700 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
			>
				{$_("error.back_to_app")}
			</a>
		</div>
	</section>
</div>
