<script lang="ts">
	import { enhance } from "$app/forms";
	import { _ } from "svelte-i18n";

	type FormLike = {
		fieldErrors?: {
			name?: string;
			city?: string;
			address?: string;
		};
		values?: {
			name?: string;
			city?: string;
			address?: string;
		};
		submitError?: string;
	} | null;

	type OfficeValues = {
		name: string;
		city: string;
		address: string;
	};

	let {
		form,
		initialValues,
		cancelHref,
		headlineKey,
		subtitleKey,
		submitLabelKey,
		loading = false,
	}: {
		form: FormLike;
		initialValues: OfficeValues;
		cancelHref: string;
		headlineKey: string;
		subtitleKey: string;
		submitLabelKey: string;
		loading?: boolean;
	} = $props();

	let submitting = $state(false);

	const enhanceSubmit = () => {
		submitting = true;
		return async ({ update }: { update: () => Promise<void> }) => {
			try {
				await update();
			} finally {
				submitting = false;
			}
		};
	};

	let values = $derived({
		name: form?.values?.name ?? initialValues.name,
		city: form?.values?.city ?? initialValues.city,
		address: form?.values?.address ?? initialValues.address,
	});
	let nameError = $derived(form?.fieldErrors?.name ?? null);
	let cityError = $derived(form?.fieldErrors?.city ?? null);
	let addressError = $derived(form?.fieldErrors?.address ?? null);
	let submitError = $derived(form?.submitError ?? null);
	let isBusy = $derived(loading || submitting);
</script>

<section class="stagger stagger-1">
	<h1 class="text-2xl font-bold text-surface-50">
		{$_(headlineKey)}
	</h1>
	<p class="mt-1 max-w-2xl text-sm text-surface-400">
		{$_(subtitleKey)}
	</p>
</section>

<section
	class="stagger stagger-2 mt-6 rounded-xl border border-surface-700/50 bg-surface-900 p-5 sm:p-6"
>
	<form method="POST" class="space-y-5" use:enhance={enhanceSubmit}>
		{#if submitError}
			<div
				class="rounded-lg border border-red-500/40 bg-red-500/10 px-3 py-2 text-sm text-red-300"
				aria-live="polite"
			>
				{$_(submitError)}
			</div>
		{/if}

		<div class="flex flex-wrap items-center gap-2">
			<span
				class="rounded-full bg-surface-800 px-2.5 py-1 text-xs font-medium text-surface-400"
			>
				{$_("admin.offices.form.id_auto_assigned")}
			</span>
		</div>

		<fieldset class="m-0 min-w-0 space-y-4 border-0 p-0">
			<div class="grid gap-4 md:grid-cols-2">
				<div class="space-y-1.5">
					<label for="name" class="text-sm font-medium text-surface-200">
						{$_("admin.offices.form.name")}
						<span class="text-red-400">*</span>
					</label>
					<input
						id="name"
						name="name"
						type="text"
						value={values.name}
						placeholder={$_("admin.offices.form.name_placeholder")}
						required
						autocomplete="off"
						aria-invalid={nameError ? "true" : undefined}
						aria-describedby={nameError
							? "office_name_hint office_name_error"
							: "office_name_hint"}
						disabled={isBusy}
						class={[
							"w-full rounded-lg border bg-surface-800 px-3 py-2 text-sm text-surface-200 placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-70",
							nameError
								? "border-red-500/70 focus-visible:ring-red-400/60"
								: "border-surface-700 focus-visible:ring-accent/50",
						]}
					/>
					<p id="office_name_hint" class="text-xs text-surface-400">
						{$_("admin.offices.form.name_hint")}
					</p>
					<p
						id="office_name_error"
						class="min-h-4 text-xs text-red-400"
						aria-live="polite"
					>
						{nameError ? $_(nameError) : ""}
					</p>
				</div>

				<div class="space-y-1.5">
					<label for="city" class="text-sm font-medium text-surface-200">
						{$_("admin.offices.form.city")}
						<span class="text-red-400">*</span>
					</label>
					<input
						id="city"
						name="city"
						type="text"
						value={values.city}
						placeholder={$_("admin.offices.form.city_placeholder")}
						required
						autocomplete="address-level2"
						aria-invalid={cityError ? "true" : undefined}
						aria-describedby={cityError
							? "office_city_hint office_city_error"
							: "office_city_hint"}
						disabled={isBusy}
						class={[
							"w-full rounded-lg border bg-surface-800 px-3 py-2 text-sm text-surface-200 placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-70",
							cityError
								? "border-red-500/70 focus-visible:ring-red-400/60"
								: "border-surface-700 focus-visible:ring-accent/50",
						]}
					/>
					<p id="office_city_hint" class="text-xs text-surface-400">
						{$_("admin.offices.form.city_hint")}
					</p>
					<p
						id="office_city_error"
						class="min-h-4 text-xs text-red-400"
						aria-live="polite"
					>
						{cityError ? $_(cityError) : ""}
					</p>
				</div>

				<div class="space-y-1.5 md:col-span-2">
					<label for="address" class="text-sm font-medium text-surface-200">
						{$_("admin.offices.form.address")}
						<span class="text-red-400">*</span>
					</label>
					<input
						id="address"
						name="address"
						type="text"
						value={values.address}
						placeholder={$_("admin.offices.form.address_placeholder")}
						required
						autocomplete="street-address"
						aria-invalid={addressError ? "true" : undefined}
						aria-describedby={addressError
							? "office_address_hint office_address_error"
							: "office_address_hint"}
						disabled={isBusy}
						class={[
							"w-full rounded-lg border bg-surface-800 px-3 py-2 text-sm text-surface-200 placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-70",
							addressError
								? "border-red-500/70 focus-visible:ring-red-400/60"
								: "border-surface-700 focus-visible:ring-accent/50",
						]}
					/>
					<p id="office_address_hint" class="text-xs text-surface-400">
						{$_("admin.offices.form.address_hint")}
					</p>
					<p
						id="office_address_error"
						class="min-h-4 text-xs text-red-400"
						aria-live="polite"
					>
						{addressError ? $_(addressError) : ""}
					</p>
				</div>
			</div>
		</fieldset>

		<div
			class="mt-1 flex flex-col-reverse gap-2 pt-4 sm:flex-row sm:justify-end"
		>
			<a
				href={cancelHref}
				class="inline-flex cursor-pointer items-center justify-center rounded-lg border border-surface-700 bg-surface-800 px-4 py-2 text-sm font-semibold text-surface-200 transition-colors hover:bg-surface-700 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
			>
				{$_("admin.offices.form.cancel")}
			</a>
			<button
				type="submit"
				disabled={isBusy}
				class="inline-flex cursor-pointer items-center justify-center rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors hover:bg-accent-hover focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50 disabled:cursor-not-allowed disabled:bg-surface-700 disabled:text-surface-400"
			>
				{$_(submitLabelKey)}
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
