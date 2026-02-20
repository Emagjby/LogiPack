<script lang="ts">
	import { page } from "$app/state";
	import { _ } from "svelte-i18n";

	const ALLOWED_DETAIL_KEYS = new Set([
		"error.details.employee_only",
		"error.details.admin_only",
	]);

	let lang = $derived(page.params.lang ?? "en");
	let appHref = $derived(`/${lang}/app`);
	let detailKey = $derived.by(() => {
		const candidate = page.url.searchParams.get("detail");
		return candidate && ALLOWED_DETAIL_KEYS.has(candidate)
			? candidate
			: "error.details.employee_only";
	});
</script>

<div class="mx-auto flex min-h-[60vh] w-full max-w-2xl items-center justify-center px-4 py-8">
	<section
		class="w-full rounded-xl border border-surface-700/50 bg-surface-900 p-8 shadow-lg shadow-black/20"
	>
		<h1 class="text-2xl font-semibold text-surface-50">{$_("forbidden.subheadline")}</h1>
		<p class="mt-3 text-sm text-surface-400">
			{$_(detailKey)}
		</p>

		<a
			href={appHref}
			class="mt-6 inline-flex cursor-pointer items-center rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors duration-150 hover:bg-accent-hover focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
		>
			{$_("forbidden.back_to_app")}
		</a>
	</section>
</div>
