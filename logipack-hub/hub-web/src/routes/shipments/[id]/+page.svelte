<script lang="ts">
	import type { PageData } from './$types';
	import Button from '$lib/components/ui/Button.svelte';
	import ShipmentHeader from '$lib/components/routes/shipments/components/ShipmentHeader.svelte';
	import CargoManifest from '$lib/components/routes/shipments/components/CargoManifest.svelte';
	const { data } = $props<{ data: PageData; form?: { success?: boolean } }>();
</script>

<div class="bg-background-light dark:bg-background-dark text-slate-900 dark:text-white font-display overflow-hidden h-screen flex flex-col">
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
				<span class="text-muted/60">Dashboard</span>
				<span class="material-symbols-outlined text-muted/60 text-[16px]">chevron_right</span>
				<a class="text-muted/60 hover:text-white transition-colors" href="/shipments">
					Shipments
				</a>
				<span class="material-symbols-outlined text-muted/60 text-[16px]">chevron_right</span>
				<span class="text-white font-medium font-mono">#{data.shipment.id}</span>
			</div>
		</div>
		<div class="flex items-center gap-6">
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
				<div
					class="bg-center bg-no-repeat aspect-square bg-cover rounded-full size-8 border border-border-dark"
					data-alt="User profile avatar showing a generic silhouette"
					style="background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuClnR0b3dNMA3T8Ewf9ejCTsCdRvsVNlqm26LuKZPC83apfKUOqh5ZRuaU6EPzMP6Plzqt1N34akAicdOFRqeVQN8b28wvdAbA5pVALp9PWFy6251gWQ6DmB6H4h0vD64NECRwXZ20RPZDd3ffu3nuD13FnxLIAGtteTwkzDBJ0TOq8vtwTELqCilO60GE5uUtmYCY9n5Rac9Y5qfCc1SE0wbkCFSTNF1Vj0RTLBL182VNjy74Ks3mdDViF5DdiOcG-0KhCCWonbL3c');"
				></div>
			</div>
		</div>
	</header>
	<div class="flex flex-1 overflow-hidden relative">
		<main class="flex-1 flex flex-col h-full min-w-0 overflow-y-auto bg-background-dark">
			<div class="w-full max-w-[1400px] mx-auto p-6 flex flex-col gap-8 h-full">
				<ShipmentHeader shipment={data.shipment} />
				<div class="grid grid-cols-1 gap-6 items-start">
					<CargoManifest shipment={data.shipment} />
				</div>
			</div>
		</main>

	</div>
</div>
