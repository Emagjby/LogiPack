<script lang="ts">
	import { page } from "$app/state";
	import OfficeForm from "$lib/components/app/OfficeForm.svelte";
	import type { ActionData } from "./$types";

	let { form }: { form: ActionData | null } = $props();
	let lang = $derived(page.params.lang || "en");

	const initialValues = {
		name: "",
		city: "",
		address: "",
	};
</script>

<OfficeForm
	{form}
	{initialValues}
	headlineKey="admin.offices.new.headline"
	subtitleKey="admin.offices.new.subtitle"
	submitLabelKey="admin.offices.new.submit"
	cancelHref={`/${lang}/app/admin/offices`}
/>
