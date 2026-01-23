<script lang="ts">
	import { goto } from "$app/navigation";
	import type { PageData } from "./$types";
	import Button from "$lib/components/ui/Button.svelte";
	import ShipmentsTable from "$lib/components/routes/shipments/components/ShipmentsTable.svelte";
	import FiltersBar from "$lib/components/routes/shipments/components/FiltersBar.svelte";
	import CreateShipmentDrawer from "$lib/components/routes/shipments/drawers/CreateShipmentDrawer.svelte";
	import TimelineSidebar from "$lib/components/routes/shipments/timeline/TimelineSidebar.svelte";
	import ChangeStatusDrawer from "$lib/components/routes/shipments/drawers/ChangeStatusDrawer.svelte";
	import type {
		ChangeStatusRequest,
		CreateShipmentInput,
		TimelineItem,
		ShipmentStatus
	} from "$lib/types/shipment";

	const { data } = $props<{ data: PageData; form?: { success?: boolean } }>();

	const timelines = $derived(data.timelines ?? {});

	let drawerOpen = $state(false);
	let timelineOpen = $state(false);
	let createOpenedByAction = $state(false);
	let menuOpenId = $state<string | null>(null);
	const initialShipmentId = $derived(data.shipments[0]?.id ?? "");
	let timelineShipmentId = $state("");
	$effect(() => {
		if (!timelineShipmentId) timelineShipmentId = initialShipmentId;
	});
	let timeline = $state<TimelineItem[]>([]);

	let changeOpen = $state(false);
	let changeShipmentId = $state("");
	const emptyChangeForm: ChangeStatusRequest = {
		to_status: 'NEW' as ShipmentStatus,
		to_office_id: null,
		notes: null
	};
	let changeForm = $state<ChangeStatusRequest>({ ...emptyChangeForm });

	const emptyForm: CreateShipmentInput = {
		client_id: "",
		current_office_id: null,
		notes: null
	};

	let createForm = $state<CreateShipmentInput>({ ...emptyForm });

	const handleRowSelect = (id: string) => {
		goto(`/shipments/${id}`);
	};

	const handleTimeline = (id: string) => {
		menuOpenId = id;
		timelineShipmentId = id;
		timeline = data.timelines?.[id] ?? [];
	};

	const handleAction = (id: string) => {
		menuOpenId = menuOpenId === id ? null : id;
		createOpenedByAction = false;
		timelineOpen = false;
	};

	$effect(() => {
		if (!menuOpenId) return;

		const closeMenu = () => {
			menuOpenId = null;
		};

		const onKeyDown = (event: KeyboardEvent) => {
			if (event.key === 'Escape') closeMenu();
		};

		window.addEventListener('scroll', closeMenu, true);
		window.addEventListener('resize', closeMenu);
		window.addEventListener('keydown', onKeyDown);

		return () => {
			window.removeEventListener('scroll', closeMenu, true);
			window.removeEventListener('resize', closeMenu);
			window.removeEventListener('keydown', onKeyDown);
		};
	});

	const closeMenu = () => {
		menuOpenId = null;
	};
</script>

<div
	class="bg-background-light dark:bg-background-dark text-slate-900 dark:text-white font-display overflow-hidden h-screen flex flex-col"
>
	<header
		class="flex items-center justify-between whitespace-nowrap border-b border-solid border-border-dark bg-background-dark px-6 py-3 h-16 shrink-0 z-20"
	>
		<div class="flex items-center gap-8">
			<div class="flex items-center gap-3 text-white">
				<div
					class="size-8 bg-primary/20 rounded flex items-center justify-center text-primary"
				>
					<span class="material-symbols-outlined text-[20px]"
						>local_shipping</span
					>
				</div>
				<h2
					class="text-white text-lg font-bold leading-tight tracking-tight"
				>
					LogiPack
				</h2>
			</div>
			<div class="flex items-center gap-2 text-sm">
				<span class="text-muted/60">Dashboard</span>
				<span
					class="material-symbols-outlined text-muted/60 text-[16px]"
					>chevron_right</span
				>
				<span class="text-white font-medium">Shipments</span>
			</div>
		</div>
		<div class="flex items-center gap-6">
			<label class="hidden md:flex flex-col min-w-40 h-9 w-64 group">
				<div
					class="flex w-full flex-1 items-stretch rounded-lg h-full bg-surface-dark border border-border-dark group-focus-within:border-primary transition-colors"
				>
					<div
						class="text-muted flex items-center justify-center pl-3"
					>
						<span class="material-symbols-outlined text-[20px]"
							>search</span
						>
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
		<main
			class="flex-1 flex flex-col h-full min-w-0 overflow-y-auto bg-background-dark"
		>
			<div
				class="w-full max-w-[1400px] mx-auto p-6 flex flex-col gap-6 h-full"
			>
				<div
					class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 shrink-0"
				>
					<div>
						<h1
							class="text-2xl font-bold tracking-tight text-white"
						>
							All Shipments
						</h1>
						<p class="text-muted text-sm mt-1">
							Manage and track active logistics operations.
						</p>
					</div>
					<div class="flex items-center gap-3">
						<Button variant="secondary">
							<span class="material-symbols-outlined text-[18px]"
								>refresh</span
							>
						</Button>
						<Button
							onclick={() => {
								drawerOpen = true;
								createOpenedByAction = true;
							}}
						>
							<span class="material-symbols-outlined text-[18px]"
								>add</span
							>
							<span>Create Shipment</span>
						</Button>
					</div>
				</div>
				<FiltersBar onClear={() => undefined} />
				<div class="relative flex-1">
					<ShipmentsTable
						shipments={data.shipments}
						selectedId={menuOpenId ?? undefined}
						onSelect={handleRowSelect}
						onAction={handleAction}
						onTimeline={(id) => {
							handleTimeline(id);
							timelineOpen = true;
						}}
						onStatus={(id) => {
							menuOpenId = id;
							changeShipmentId = id;
							changeOpen = true;
						}}
						onClose={closeMenu}
					/>
				</div>
			</div>
		</main>
		<form
			method="POST"
			action="?/create"
			onsubmit={() => {
				drawerOpen = createOpenedByAction ? false : drawerOpen;
				createForm = { ...emptyForm };
				createOpenedByAction = false;
			}}
		>
			<CreateShipmentDrawer
				open={drawerOpen}
				form={createForm}
				onClose={() => (drawerOpen = false)}
				onSubmit={() => (drawerOpen = false)}
			/>
		</form>
		<TimelineSidebar
			open={timelineOpen}
			shipmentId={timelineShipmentId}
			{timeline}
			onClose={() => {
				timelineOpen = false;
				menuOpenId = null;
			}}
		/>

		<form
			method="POST"
			action="?/changeStatus"
			onsubmit={() => {
				changeOpen = false;
				changeForm = { ...emptyChangeForm };
				menuOpenId = null;
			}}
		>
			<input type="hidden" name="shipmentId" value={changeShipmentId} />
			<ChangeStatusDrawer
				open={changeOpen}
				form={changeForm}
				statusOptions={data.statusOptions}
				onClose={() => (changeOpen = false)}
				onSubmit={() => (changeOpen = false)}
			/>
		</form>
	</div>
</div>
