<script lang="ts">
	import type { PageData } from "./$types";

	const { data } = $props<{ data: PageData }>();
</script>

<h1>LogiPack Hub Web</h1>
<p>Phase 3 scaffold is up.</p>
<a href="/shipments">Go to Shipments</a>

<h2>API health</h2>
{#if data.api.ok}
	<p><strong>OK</strong>: {data.api.text}</p>
{:else}
	<p><strong>DOWN</strong>: {data.api.text}</p>
{/if}
