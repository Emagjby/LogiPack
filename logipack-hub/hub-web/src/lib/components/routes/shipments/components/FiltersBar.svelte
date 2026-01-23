<script lang="ts">
	import Button from "$lib/components/ui/Button.svelte";

	let { onClear } = $props<{
		onClear: () => void;
	}>();
</script>

<div class="flex flex-wrap items-center gap-3 pb-2 shrink-0">
	<div class="relative group">
		<select
			class="appearance-none pt-[7px] bg-surface-dark border border-border-dark text-white flex align-center justify-center text-xs rounded h-8 pl-3 pr-8 focus:ring-1 focus:ring-primary focus:border-primary cursor-pointer hover:bg-zinc-800 transition-colors"
		>
			<option>Status: All</option>
			<option>In Transit</option>
			<option>Pending</option>
			<option>Delivered</option>
		</select>
	</div>
	<div class="relative group">
		<select
			class="appearance-none pt-[7px] bg-surface-dark border border-border-dark text-white flex align-center justify-center text-xs rounded h-8 pl-3 pr-8 focus:ring-1 focus:ring-primary focus:border-primary cursor-pointer hover:bg-zinc-800 transition-colors"
		>
			<option>Office: All Locations</option>
			<option>Berlin Hub</option>
			<option>Austin TX</option>
			<option>Tokyo Dist</option>
		</select>
	</div>
	<div class="h-4 w-px bg-border-dark mx-1"></div>
	<Button variant="ghost" size="sm" onclick={onClear}>Clear filters</Button>
</div>
