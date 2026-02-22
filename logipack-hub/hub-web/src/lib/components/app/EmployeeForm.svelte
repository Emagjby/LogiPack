<script lang="ts">
	import { enhance } from "$app/forms";
	import { _ } from "svelte-i18n";

	type FormLike = {
		fieldErrors?: {
			email?: string;
		};
		values?: {
			email?: string;
		};
		submitError?: string;
	} | null;

	type EmployeeValues = {
		email: string;
	};

	let {
		form,
		initialValues,
		cancelHref,
		submitLabelKey,
		loading = false,
	}: {
		form: FormLike;
		initialValues: EmployeeValues;
		cancelHref: string;
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
		email: form?.values?.email ?? initialValues.email,
	});
	let emailError = $derived(form?.fieldErrors?.email ?? null);
	let submitError = $derived(form?.submitError ?? null);
	let isBusy = $derived(loading || submitting);
</script>

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

		<fieldset class="m-0 min-w-0 space-y-4 border-0 p-0">
			<div class="space-y-1.5">
				<label for="email" class="text-sm font-medium text-surface-200">
					{$_("admin.employees.form.email")}
					<span class="text-red-400">*</span>
				</label>
				<input
					id="email"
					name="email"
					type="email"
					value={values.email}
					placeholder={$_("admin.employees.form.email_placeholder")}
					required
					autocomplete="email"
					aria-invalid={emailError ? "true" : undefined}
					aria-describedby={emailError
						? "employee_email_hint employee_email_error"
						: "employee_email_hint"}
					disabled={isBusy}
					class={[
						"w-full rounded-lg border bg-surface-800 px-3 py-2 text-sm text-surface-200 placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-70",
						emailError
							? "border-red-500/70 focus-visible:ring-red-400/60"
							: "border-surface-700 focus-visible:ring-accent/50",
					]}
				/>
				<p id="employee_email_hint" class="text-xs text-surface-400">
					{$_("admin.employees.form.email_hint")}
				</p>
				<p
					id="employee_email_error"
					class="min-h-4 text-xs text-red-400"
					aria-live="polite"
				>
					{emailError ? $_(emailError) : ""}
				</p>
			</div>
		</fieldset>

		<div
			class="mt-1 flex flex-col-reverse gap-2 pt-4 sm:flex-row sm:justify-end"
		>
			<a
				href={cancelHref}
				class="inline-flex cursor-pointer items-center justify-center rounded-lg border border-surface-700 bg-surface-800 px-4 py-2 text-sm font-semibold text-surface-200 transition-colors hover:bg-surface-700 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
			>
				{$_("admin.employees.form.cancel")}
			</a>
			<button
				type="submit"
				disabled={isBusy}
				class="inline-flex cursor-pointer items-center justify-center rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors hover:bg-accent-hover focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50 disabled:cursor-not-allowed disabled:bg-surface-700 disabled:text-surface-400"
			>
				{isBusy
					? $_("admin.employees.form.submitting")
					: $_(submitLabelKey)}
			</button>
		</div>
	</form>
</section>
