<script lang="ts">
	import { page } from "$app/state";
	import { _ } from "svelte-i18n";
	import EmployeeForm from "$lib/components/app/EmployeeForm.svelte";
	import type { ActionData, PageData } from "./$types";

	let {
		data,
		form,
	}: {
		data: PageData;
		form: ActionData | null;
	} = $props();
	let lang = $derived(page.params.lang || "en");
</script>

<section
	class="stagger stagger-1 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between"
>
	<div>
		<h1 class="text-2xl font-bold text-surface-50">
			{$_("admin.employees.new.headline")}
		</h1>
		<p class="mt-1 max-w-2xl text-sm text-surface-400">
			{$_("admin.employees.new.subtitle")}
		</p>
	</div>
	<a
		href={`/${lang}/app/admin/employees`}
		class="inline-flex cursor-pointer items-center justify-center rounded-lg border border-surface-700 bg-surface-800 px-4 py-2 text-sm font-semibold text-surface-200 transition-colors hover:bg-surface-700 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
	>
		{$_("admin.employees.new.back_to_list")}
	</a>
</section>

<EmployeeForm
	{form}
	initialValues={data.initialValues}
	cancelHref={`/${lang}/app/admin/employees`}
	submitLabelKey="admin.employees.new.submit"
/>
