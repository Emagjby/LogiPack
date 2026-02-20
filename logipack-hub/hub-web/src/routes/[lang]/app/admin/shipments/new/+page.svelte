<script lang="ts">
	import { page } from "$app/state";
	import { _ } from "svelte-i18n";
	import type { ActionData, PageData } from "./$types";

	let {
		data,
		form,
	}: {
		data: PageData;
		form: ActionData | null;
	} = $props();

	let lang = $derived(page.params.lang || "en");
	let values = $derived({
		client_id: form?.values?.client_id ?? "",
		current_office_id: form?.values?.current_office_id ?? "",
		notes: form?.values?.notes ?? "",
	});
	let clientIdError = $derived(form?.fieldErrors?.client_id ?? null);
</script>

<section class="stagger stagger-1">
	<h1 class="text-2xl font-bold text-surface-50">
		{$_("admin.shipments.new.headline")}
	</h1>
	<p class="mt-1 max-w-2xl text-sm text-surface-400">
		{$_("admin.shipments.new.subtitle")}
	</p>
</section>

<section
	class="stagger stagger-2 mt-6 rounded-xl border border-surface-700/50 bg-surface-900 p-5 sm:p-6"
>
	<form method="POST" class="space-y-5">
		<div class="flex flex-wrap items-center gap-2">
			<span
				class="rounded-full bg-surface-800 px-2.5 py-1 text-xs font-medium text-surface-400"
			>
				{$_("admin.shipments.new.context.status_auto")}
			</span>
		</div>

		<div class="grid gap-4 md:grid-cols-2">
			<div class="space-y-1.5">
				<label
					for="client_id"
					class="text-sm font-medium text-surface-200"
				>
					{$_("admin.shipments.new.client_id")}
					<span class="text-red-400">*</span>
				</label>
				<input
					id="client_id"
					name="client_id"
					type="text"
					value={values.client_id}
					aria-invalid={clientIdError ? "true" : undefined}
					aria-describedby={clientIdError
						? "client_id_error"
						: undefined}
					class={[
						"w-full rounded-lg border bg-surface-800 px-3 py-2 text-sm text-surface-200 placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-1",
						clientIdError
							? "border-red-500/70 focus-visible:ring-red-400/60"
							: "border-surface-700 focus-visible:ring-accent/50",
					]}
				/>
				{#if clientIdError}
					<p
						id="client_id_error"
						class="text-xs text-red-400"
						aria-live="polite"
					>
						{$_(clientIdError)}
					</p>
				{/if}
			</div>

			<div class="space-y-1.5">
				<label
					for="current_office_id"
					class="text-sm font-medium text-surface-200"
				>
					{$_("admin.shipments.new.current_office_id")}
					<span class="text-xs font-normal text-surface-400">
						({$_("admin.shipments.new.optional")})
					</span>
				</label>
				<input
					id="current_office_id"
					name="current_office_id"
					type="text"
					value={values.current_office_id}
					class="w-full rounded-lg border border-surface-700 bg-surface-800 px-3 py-2 text-sm text-surface-200 placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
				/>
				<p class="text-xs text-surface-400">
					{$_("admin.shipments.new.current_office_hint")}
				</p>
			</div>
		</div>

		<div class="space-y-1.5">
			<label for="notes" class="text-sm font-medium text-surface-200">
				{$_("admin.shipments.new.notes")}
				<span class="text-xs font-normal text-surface-400">
					({$_("admin.shipments.new.optional")})
				</span>
			</label>
			<textarea
				id="notes"
				name="notes"
				rows="5"
				placeholder={$_("admin.shipments.new.notes_placeholder")}
				class="min-h-[120px] w-full rounded-lg border border-surface-700 bg-surface-800 px-3 py-2 text-sm text-surface-200 placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
				>{values.notes}</textarea
			>
		</div>

		<div
			class="mt-1 flex flex-col-reverse gap-2 pt-4 sm:flex-row sm:justify-end"
		>
			<a
				href={`/${lang}/app/admin/shipments`}
				class="inline-flex cursor-pointer items-center justify-center rounded-lg border border-surface-700 bg-surface-800 px-4 py-2 text-sm font-semibold text-surface-200 transition-colors hover:bg-surface-700 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
			>
				{$_("admin.shipments.new.cancel")}
			</a>
			<button
				type="submit"
				class="inline-flex cursor-pointer items-center justify-center rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors hover:bg-accent-hover focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
			>
				{$_("admin.shipments.new.create")}
			</button>
		</div>
	</form>
</section>

<style>
	@keyframes fadeSlideUp {
		from {
			opacity: 0;
			transform: translateY(8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.stagger {
		animation: fadeSlideUp 0.4s ease-out both;
	}
	.stagger-1 {
		animation-delay: 0.05s;
	}
	.stagger-2 {
		animation-delay: 0.1s;
	}
</style>
