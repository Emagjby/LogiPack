<script lang="ts">
	import { page } from "$app/state";
	import { _ } from "svelte-i18n";

	const AUTH_DETAIL_KEYS = new Set([
		"error.details.auth_management_token_failed",
		"error.details.auth_user_profile_read_failed",
		"error.details.auth_user_role_load_failed",
		"error.details.auth_missing_code",
		"error.details.auth_exchange_failed",
		"error.details.auth_provision_failed",
		"error.details.auth_account_conflict",
		"error.details.auth_invalid_profile",
		"error.details.auth_unknown",
	]);

	let lang = $derived(page.params.lang ?? "en");
	let status = $derived(page.url.searchParams.get("status") ?? "500");
	let backHref = $derived(status === "409" ? "/logout" : `/${lang}`);
	let detailParam = $derived(
		page.url.searchParams.get("detail") ?? "error.details.auth_unknown",
	);
	let detailKey = $derived(
		AUTH_DETAIL_KEYS.has(detailParam) ? detailParam : "error.details.auth_unknown",
	);

	function backToLanding() {
		window.location.href = backHref;
	}
</script>

<div class="min-h-dvh bg-surface-950 px-4 py-10">
	<div class="mx-auto flex min-h-[70vh] w-full max-w-2xl items-center justify-center">
		<section
			aria-live="polite"
			class="w-full rounded-xl border border-surface-700/50 bg-surface-900 p-8 shadow-lg shadow-black/20"
		>
			<p
				class="inline-flex items-center rounded-full border border-red-500/20 bg-red-500/10 px-2.5 py-1 text-xs font-medium uppercase tracking-wide text-red-400"
			>
				{$_("error.status", { values: { status } })}
			</p>
			<h1 class="mt-2 text-2xl font-semibold text-surface-50">{$_("auth_error.headline")}</h1>
			<p class="mt-3 text-sm text-surface-400">{$_("auth_error.helper")}</p>
			<p class="mt-2 text-sm text-surface-400">{$_(detailKey)}</p>

			<button
				type="button"
				onclick={backToLanding}
				class="mt-6 inline-flex cursor-pointer items-center rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors duration-150 hover:bg-accent-hover focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
			>
				{$_("auth_error.back_to_landing")}
			</button>
		</section>
	</div>
</div>
