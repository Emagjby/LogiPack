<script lang="ts">
	import { onMount } from 'svelte';
	import Button from '$lib/components/ui/Button.svelte';

	type Breadcrumb = { label: string; href?: string };

	const { breadcrumbs = [], titleSuffix = null, devUserSub = null } = $props<{
		breadcrumbs?: Breadcrumb[];
		titleSuffix?: string | null;
		devUserSub?: string | null;
	}>();

	let menuOpen = $state(false);
	let selectedSub = $state('');

	onMount(() => {
		selectedSub = localStorage.getItem('dev_user_sub') ?? '';
	});

	const DEV_UUIDS = {
		admin: '00000000-0000-0000-0000-000000000001',
		employee: '00000000-0000-0000-0000-000000000002'
	} as const;

	const devUsers = [
		{ label: 'Admin', value: `admin+${DEV_UUIDS.admin}@test.com` },
		{ label: 'Employee', value: `employee+${DEV_UUIDS.employee}@test.com` }
	] as const;

	const toggleMenu = () => {
		menuOpen = !menuOpen;
	};

	const closeMenu = () => {
		menuOpen = false;
	};

	const submit = () => {
		if (!selectedSub) return;
		localStorage.setItem('dev_user_sub', selectedSub);
		closeMenu();
		location.reload();
	};
</script>

<header
	class="flex items-center justify-between whitespace-nowrap border-b border-solid border-border-dark bg-background-dark px-6 py-3 h-16 shrink-0 z-20"
>
	<div class="flex items-center gap-8">
		<div class="flex items-center gap-3 text-white">
			<div class="size-8 bg-primary/20 rounded flex items-center justify-center text-primary">
				<span class="material-symbols-outlined text-[20px]">local_shipping</span>
			</div>
			<h2 class="text-white text-lg font-bold leading-tight tracking-tight">LogiPack</h2>
		</div>
		<div class="flex items-center gap-2 text-sm">
			{#each breadcrumbs as crumb, i}
				{#if i > 0}
					<span class="material-symbols-outlined text-muted/60 text-[16px]">chevron_right</span>
				{/if}
				{#if crumb.href}
					<a class="text-muted/60 hover:text-white transition-colors" href={crumb.href}>
						{crumb.label}
					</a>
				{:else}
					<span class={i === breadcrumbs.length - 1 ? 'text-white font-medium' : 'text-muted/60'}>
						{crumb.label}
					</span>
				{/if}
			{/each}
			{#if titleSuffix}
				<span class="material-symbols-outlined text-muted/60 text-[16px]">chevron_right</span>
				<span class="text-white font-medium font-mono">{titleSuffix}</span>
			{/if}
		</div>
	</div>

	<div class="flex items-center gap-6 relative">
		<label class="hidden md:flex flex-col min-w-40 h-9 w-64 group">
			<div
				class="flex w-full flex-1 items-stretch rounded-lg h-full bg-surface-dark border border-border-dark group-focus-within:border-primary transition-colors"
			>
				<div class="text-muted flex items-center justify-center pl-3">
					<span class="material-symbols-outlined text-[20px]">search</span>
				</div>
				<input
					class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg bg-transparent border-none text-white focus:ring-0 placeholder:text-zinc-600 px-3 text-sm font-normal"
					placeholder="Search orders, clients..."
					value=""
				/>
			</div>
		</label>

		<div class="flex items-center gap-4">
			<Button variant="ghost" class="text-muted hover:text-white">
				<span class="material-symbols-outlined">notifications</span>
			</Button>

			<button
				type="button"
				onclick={toggleMenu}
				class="bg-center bg-no-repeat aspect-square bg-cover rounded-full size-8 border border-border-dark"
				aria-label="Select dev actor"
				style="background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuClnR0b3dNMA3T8Ewf9ejCTsCdRvsVNlqm26LuKZPC83apfKUOqh5ZRuaU6EPzMP6Plzqt1N34akAicdOFRqeVQN8b28wvdAbA5pVALp9PWFy6251gWQ6DmB6H4h0vD64NECRwXZ20RPZDd3ffu3nuD13FnxLIAGtteTwkzDBJ0TOq8vtwTELqCilO60GE5uUtmYCY9n5Rac9Y5qfCc1SE0wbkCFSTNF1Vj0RTLBL182VNjy74Ks3mdDViF5DdiOcG-0KhCCWonbL3c');"
			></button>
		</div>

		{#if menuOpen}
			<div
				class="absolute right-0 top-12 w-72 rounded-lg border border-border-dark bg-surface-dark p-3 shadow-lg"
			>
				<div class="text-xs text-muted mb-2">Dev actor (x-dev-user-sub)</div>
				<select
					class="w-full rounded-md bg-background-dark border border-border-dark text-white px-2 py-2 text-sm"
					bind:value={selectedSub}
				>
					<option value="">Select actor…</option>
					{#each devUsers as u}
						<option value={u.value}>{u.label} ({u.value})</option>
					{/each}
				</select>
				<div class="flex items-center justify-between mt-3">
					<button
						type="button"
						class="text-muted text-sm hover:text-white"
						onclick={closeMenu}
					>
						Cancel
					</button>
					<button
						type="button"
						class="rounded-md bg-primary text-white px-3 py-2 text-sm font-medium"
						disabled={!selectedSub}
						onclick={submit}
					>
						Switch
					</button>
				</div>
			</div>
		{/if}
	</div>
</header>
