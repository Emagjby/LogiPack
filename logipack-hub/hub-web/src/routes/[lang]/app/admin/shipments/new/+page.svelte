<script lang="ts">
	import { page } from "$app/state";
	import ShipmentCreateForm from "$lib/components/app/ShipmentCreateForm.svelte";
	import type { ActionData } from "./$types";

	let { form }: { form: ActionData | null } = $props();
	let lang = $derived(page.params.lang || "en");
</script>

<ShipmentCreateForm
	{form}
	cancelHref={`/${lang}/app/admin/shipments`}
	showOfficeInput
/>
