<script lang="ts">
	import "./layout.css";
	import favicon from "$lib/assets/favicon.svg";

	let { children, data } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	{#if data.lang === "bg"}
		<link
			rel="preload"
			href="/fonts/inter-cyrillic.woff2"
			as="font"
			type="font/woff2"
			crossorigin="anonymous"
		/>
		<link
			rel="preload"
			href="/fonts/inter-cyrillic-ext.woff2"
			as="font"
			type="font/woff2"
			crossorigin="anonymous"
		/>
	{/if}
</svelte:head>

{@render children()}
