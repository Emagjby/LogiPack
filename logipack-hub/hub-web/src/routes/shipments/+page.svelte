<script lang="ts">
	import { goto } from "$app/navigation";
	import type { PageData } from "./$types";
	import Button from "$lib/components/ui/Button.svelte";
	import AppHeader from "$lib/components/layout/AppHeader.svelte";
	import ShipmentsTable from "$lib/components/routes/shipments/components/ShipmentsTable.svelte";
	import FiltersBar from "$lib/components/routes/shipments/components/FiltersBar.svelte";
	import CreateShipmentDrawer from "$lib/components/routes/shipments/drawers/CreateShipmentDrawer.svelte";
	import TimelineSidebar from "$lib/components/routes/shipments/timeline/TimelineSidebar.svelte";
	import ChangeStatusDrawer from "$lib/components/routes/shipments/drawers/ChangeStatusDrawer.svelte";
	import type {
		ChangeStatusRequest,
		CreateShipmentInput,
		TimelineItem,
		ShipmentStatus,
		Uuid
	} from "$lib/types/shipment";

	const { data, form } = $props<{ data: PageData; form?: { success?: boolean; error?: string } }>();

	const timelines = $derived(data.timelines ?? {});

	let statusFilter = $state<ShipmentStatus | 'ALL'>('ALL');
	let officeFilter = $state<Uuid | 'ALL'>('ALL');

	const pageSize = 10;
	let page = $state(1);

	const filteredShipments = $derived(
		data.shipments.filter((s: (typeof data.shipments)[number]) => {
			if (statusFilter !== 'ALL' && s.current_status !== statusFilter) return false;
			if (officeFilter !== 'ALL' && (s.current_office_id ?? null) !== officeFilter) return false;
			return true;
		})
	);

	const totalPages = $derived(Math.max(1, Math.ceil(filteredShipments.length / pageSize)));
	const showPagination = $derived(filteredShipments.length > pageSize);

	$effect(() => {
		if (page > totalPages) page = totalPages;
		if (page < 1) page = 1;
	});

	const pagedShipments = $derived(filteredShipments.slice((page - 1) * pageSize, page * pageSize));

	const filteredTimelines = $derived(
		Object.fromEntries(filteredShipments.map((s: (typeof data.shipments)[number]) => [s.id, timelines[s.id] ?? []]))
	);

	$effect(() => {
		page = 1;
	});

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
		timeline = filteredTimelines[id] ?? [];
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
	<AppHeader breadcrumbs={[{ label: 'Dashboard' }, { label: 'Shipments' }]} />
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
				{#if form?.error}
					<div class="rounded-md border border-red-500/40 bg-red-500/10 px-4 py-2 text-sm text-red-200">
						{form.error}
					</div>
				{/if}
				<FiltersBar
					status={statusFilter}
					officeId={officeFilter}
					onChange={({ status, officeId }) => {
						statusFilter = status;
						officeFilter = officeId;
						page = 1;
					}}
					onClear={() => {
						statusFilter = 'ALL';
						officeFilter = 'ALL';
						page = 1;
					}}
				/>
				<div class="relative flex-1">
					<ShipmentsTable
						shipments={pagedShipments}
						selectedId={menuOpenId ?? undefined}
						page={page}
						totalPages={totalPages}
						showPagination={showPagination}
						onPageChange={(next) => (page = next)}
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
				const sub = localStorage.getItem('dev_user_sub') ?? '';
				const input = document.getElementById('create-shipment-dev-user-sub') as HTMLInputElement | null;
				if (input) input.value = sub;
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
				const sub = localStorage.getItem('dev_user_sub') ?? '';
				const input = document.getElementById('change-status-dev-user-sub') as HTMLInputElement | null;
				if (input) input.value = sub;
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
