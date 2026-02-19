<script lang="ts">
	import type { PageData } from "./$types";
	import { page } from "$app/state";
	import { _ } from "svelte-i18n";
	import {
		formatDateTime,
		shortenHash,
		type StrataPackage,
	} from "$lib/domain/strataPackage";
	import ShipmentStatusBadge from "$lib/components/app/ShipmentStatusBadge.svelte";
	import StrataPackageViewer from "$lib/components/app/StrataPackageViewer.svelte";

	let { data }: { data: PageData } = $props();

	let lang = $derived(data.pathname.split("/")[1] || "en");
	let copiedId = $state(false);
	let selectedPackage = $state<StrataPackage | null>(null);

	async function copyId(id: string) {
		try {
			await navigator.clipboard.writeText(id);
			copiedId = true;
			setTimeout(() => {
				copiedId = false;
			}, 1200);
		} catch {
			// Ignore clipboard errors
		}
	}

	function formatEventType(type: string): string {
		return type
			.split("_")
			.map((word, i) =>
				i === 0
					? word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()
					: word.toLowerCase(),
			)
			.join(" ");
	}

</script>

{#if data.result.state === "error"}
	<!-- Error state -->
	<div class="stagger stagger-1 flex flex-col items-center py-20 text-center">
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-red-500/10"
		>
			<svg
				class="h-6 w-6 text-red-400"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
				stroke-width="1.5"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z"
				/>
			</svg>
		</div>
		<h2 class="mt-4 text-lg font-semibold text-surface-50">
			{$_("shipment.detail.error.title")}
		</h2>
		{#if data.result.message}
			<p class="mt-2 font-mono text-xs text-surface-600">
				{data.result.message}
			</p>
		{/if}
		<a
			href={`/${lang}/app/employee/shipments/${page.params.id}`}
			class="mt-5 rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors hover:bg-accent-hover"
		>
			{$_("shipment.detail.error.retry")}
		</a>
	</div>
{:else if data.result.state === "not_found"}
	<!-- Not-found state -->
	<div class="stagger stagger-1 flex flex-col items-center py-20 text-center">
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-surface-800"
		>
			<svg
				class="h-6 w-6 text-surface-600"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
				stroke-width="1.5"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0l-3-3m3 3l3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z"
				/>
			</svg>
		</div>
		<h2 class="mt-4 text-lg font-semibold text-surface-50">
			{$_("shipment.detail.not_found.title")}
		</h2>
		<p class="mt-1 max-w-sm text-sm text-surface-400">
			{$_("shipment.detail.not_found.description")}
		</p>
		<a
			href={`/${lang}/app/employee/shipments`}
			class="mt-5 rounded-lg bg-accent px-4 py-2 text-sm font-semibold text-surface-950 transition-colors hover:bg-accent-hover"
		>
			{$_("shipment.detail.not_found.back")}
		</a>
	</div>
{:else}
	{@const shipment = data.result.shipment}
	{@const statusHistory = data.result.statusHistory}
	{@const packages = data.result.packages}

	<!-- 1. Header row -->
	<section
		class="stagger stagger-1 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between"
	>
		<div>
			<h1 class="text-2xl font-bold text-surface-50">
				{$_("shipment.detail.title")}
				{shipment.id}
			</h1>
			<p class="mt-1 text-sm text-surface-400">
				{$_("shipment.detail.last_updated", {
					values: { time: formatDateTime(shipment.updated_at, lang) },
				})}
			</p>
		</div>
		<div class="flex items-center gap-2">
			<a
				href={`/${lang}/app/employee/shipments`}
				class="rounded-lg bg-surface-800 px-3 py-2 text-sm font-medium text-surface-400 transition-colors hover:bg-surface-700 hover:text-surface-200 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50"
			>
				{$_("shipment.detail.back")}
			</a>
		</div>
	</section>

	<!-- 2. Core fields panel -->
	<div
		class="stagger stagger-2 mt-6 rounded-xl border border-surface-700/50 bg-surface-900 p-5"
	>
		<dl class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
			<!-- Shipment ID -->
			<div>
				<dt
					class="text-[11px] font-medium uppercase tracking-wider text-surface-600"
				>
					{$_("shipment.meta.id")}
				</dt>
				<dd class="mt-1 flex items-center gap-2 text-sm">
					<span class="font-mono text-accent">{shipment.id}</span>
					<button
						type="button"
						onclick={() => copyId(shipment.id)}
						class="cursor-pointer rounded-md bg-surface-800 px-1.5 py-1 text-[0.62rem] font-medium text-accent transition-colors hover:bg-surface-700"
						title={$_("shipment.detail.copy_id")}
						aria-label={$_("shipment.detail.copy_id")}
					>
						{#if copiedId}
							{$_("shipment.detail.copied")}
						{:else}
							<svg
								class="h-3.5 w-3.5 text-accent"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
								stroke-width="2"
							>
								<rect
									x="9"
									y="9"
									width="11"
									height="11"
									rx="2"
								/>
								<path
									d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
								/>
							</svg>
						{/if}
					</button>
				</dd>
			</div>

			<!-- Client ID -->
			<div>
				<dt
					class="text-[11px] font-medium uppercase tracking-wider text-surface-600"
				>
					{$_("shipment.meta.client_id")}
				</dt>
				<dd class="mt-1 font-mono text-sm text-surface-200">
					{shipment.client_id}
				</dd>
			</div>

			<!-- Status -->
			<div>
				<dt
					class="text-[11px] font-medium uppercase tracking-wider text-surface-600"
				>
					{$_("shipment.meta.status")}
				</dt>
				<dd class="mt-1 text-sm text-surface-200">
					<ShipmentStatusBadge status={shipment.current_status} />
				</dd>
			</div>

			<!-- Current Office -->
			<div>
				<dt
					class="text-[11px] font-medium uppercase tracking-wider text-surface-600"
				>
					{$_("shipment.meta.current_office_id")}
				</dt>
				<dd class="mt-1 font-mono text-sm text-surface-200">
					{shipment.current_office_id}
				</dd>
			</div>

			<!-- Created -->
			<div>
				<dt
					class="text-[11px] font-medium uppercase tracking-wider text-surface-600"
				>
					{$_("shipment.meta.created")}
				</dt>
				<dd class="mt-1 text-sm text-surface-200">
					{formatDateTime(shipment.created_at, lang)}
				</dd>
			</div>

			<!-- Updated -->
			<div>
				<dt
					class="text-[11px] font-medium uppercase tracking-wider text-surface-600"
				>
					{$_("shipment.meta.updated")}
				</dt>
				<dd class="mt-1 text-sm text-surface-200">
					{formatDateTime(shipment.updated_at, lang)}
				</dd>
			</div>
		</dl>
	</div>

	<!-- 3. Status History panel -->
	<div
		class="stagger stagger-3 mt-6 rounded-xl border border-surface-700/50 bg-surface-900"
	>
		<div class="border-b border-surface-700/50 px-5 py-4">
			<h2 class="text-sm font-semibold text-surface-50">
				{$_("shipment.history.title")}
			</h2>
		</div>

		{#if statusHistory.length === 0}
			<div class="px-5 py-8 text-center text-sm text-surface-600">
				{$_("shipment.history.no_notes")}
			</div>
		{:else}
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead>
						<tr>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.history.changed_at")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.history.from_status")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.history.to_status")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.history.office")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.history.actor")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.history.notes")}
							</th>
						</tr>
					</thead>
					<tbody>
						{#each statusHistory as row (row.id)}
							<tr
								class="border-t border-surface-800 transition-colors hover:bg-surface-800/50"
							>
								<td
									class="whitespace-nowrap px-5 py-3 text-xs text-surface-400"
								>
									{formatDateTime(row.changed_at, lang)}
								</td>
								<td class="px-5 py-3">
									{#if row.from_status}
										<ShipmentStatusBadge
											status={row.from_status}
											compact
										/>
									{:else}
										<span
											class="text-xs italic text-surface-600"
											>{$_("common.none")}</span
										>
									{/if}
								</td>
								<td class="px-5 py-3">
									<ShipmentStatusBadge
										status={row.to_status}
										compact
									/>
								</td>
								<td
									class="px-5 py-3 font-mono text-xs text-surface-400"
								>
									{row.office_id ?? $_("common.none")}
								</td>
								<td
									class="px-5 py-3 font-mono text-xs text-surface-400"
								>
									{row.actor_user_id ?? $_("common.none")}
								</td>
								<td
									class="max-w-[200px] truncate px-5 py-3 text-xs text-surface-400"
									title={row.notes ?? undefined}
								>
									{row.notes ??
										$_("shipment.history.no_notes")}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</div>

	<!-- 4. Strata Timeline panel -->
	<div
		class="stagger stagger-4 mt-6 rounded-xl border border-surface-700/50 bg-surface-900"
	>
		<div class="border-b border-surface-700/50 px-5 py-4">
			<h2 class="text-sm font-semibold text-surface-50">
				{$_("shipment.strata.title")}
			</h2>
		</div>

		{#if packages.length === 0}
			<div class="px-5 py-8 text-center text-sm text-surface-600">
				{$_("common.none")}
			</div>
		{:else}
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead>
						<tr>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.strata.seq")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.strata.event_type")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.strata.created_at")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.strata.hash")}
							</th>
							<th
								class="px-5 py-2.5 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("shipment.strata.prev_hash")}
							</th>
						</tr>
					</thead>
					<tbody>
						{#each packages as pkg (pkg.hash)}
							<tr
								class="cursor-pointer border-t border-surface-800 transition-colors hover:bg-surface-800/50"
								onclick={() => {
									selectedPackage = pkg;
								}}
								tabindex="0"
								role="link"
								onkeydown={(e) => {
									if (e.key === "Enter" || e.key === " ") {
										e.preventDefault();
										selectedPackage = pkg;
									}
								}}
							>
								<td
									class="px-5 py-3 font-mono text-xs text-surface-200"
								>
									#{pkg.seq}
								</td>
								<td class="px-5 py-3 text-sm text-surface-200">
									{formatEventType(pkg.event_type)}
								</td>
								<td
									class="whitespace-nowrap px-5 py-3 text-xs text-surface-400"
								>
									{formatDateTime(pkg.created_at, lang)}
								</td>
								<td
									class="px-5 py-3 font-mono text-xs text-accent"
									title={pkg.hash}
								>
									{shortenHash(pkg.hash)}
								</td>
								<td
									class={[
										"px-5 py-3 font-mono text-xs",
										pkg.prev_hash
											? "text-surface-400"
											: "text-surface-600 italic",
									]}
									title={pkg.prev_hash ?? undefined}
								>
									{pkg.prev_hash
										? shortenHash(pkg.prev_hash)
										: $_("common.none")}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</div>

	<!-- 5. Strata Package Viewer modal -->
	<StrataPackageViewer
		pkg={selectedPackage}
		lang={lang}
		onclose={() => {
			selectedPackage = null;
		}}
	/>
{/if}

<style>
	@keyframes fadeSlideUp {
		from {
			opacity: 0;
			transform: translateY(8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.stagger {
		animation: fadeSlideUp 0.4s ease-out both;
	}
	.stagger-1 {
		animation-delay: 0.05s;
	}
	.stagger-2 {
		animation-delay: 0.1s;
	}
	.stagger-3 {
		animation-delay: 0.15s;
	}
	.stagger-4 {
		animation-delay: 0.2s;
	}
</style>
