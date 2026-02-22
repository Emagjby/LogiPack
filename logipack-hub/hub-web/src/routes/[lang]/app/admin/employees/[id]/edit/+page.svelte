<script lang="ts">
	import { _ } from "svelte-i18n";
</script>

<section
	class="rounded-2xl border border-surface-800 bg-surface-900/55 p-6 shadow-[0_8px_30px_rgb(0,0,0,0.2)]"
>
	<h1 class="text-xl font-semibold tracking-tight text-surface-50">
		{$_("admin.employees.edit.headline")}
	</h1>
	<p class="mt-2 text-sm text-surface-400">
		{$_("admin.employees.edit.todo")}
	</p>
</section>
