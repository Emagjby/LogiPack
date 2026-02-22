<script lang="ts">
	import { page } from "$app/state";
	import { _ } from "svelte-i18n";
	import type { PageData } from "./$types";

	let { data }: { data: PageData } = $props();

	let lang = $derived(page.params.lang || "en");
	let events = $derived(data.result.state === "ok" ? data.result.events : []);
	let dateTimeFormat = $derived.by(
		() =>
			new Intl.DateTimeFormat(lang, {
				month: "short",
				day: "numeric",
				year: "numeric",
				hour: "2-digit",
				minute: "2-digit",
				hour12: false,
			}),
	);

	function formatTime(iso: string): string {
		const timestamp = new Date(iso);
		if (Number.isNaN(timestamp.getTime())) return "—";
		return dateTimeFormat.format(timestamp);
	}

	function shipmentHref(shipmentId: string): string {
		return `/${lang}/app/admin/shipments/${shipmentId}`;
	}
</script>

{#if data.result.state === "error"}
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
			{$_("admin.audit.error.headline")}
		</h2>
		<a
			href={`/${lang}/app/admin/audit`}
			class="mt-5 rounded-lg bg-accent px-3 py-1.5 text-xs font-semibold text-surface-950 transition-colors hover:bg-accent-hover"
		>
			{$_("admin.audit.retry")}
		</a>
	</div>
{:else}
	<section class="stagger stagger-1">
		<h1 class="text-2xl font-bold text-surface-50">
			{$_("admin.audit.headline")}
		</h1>
		<p class="mt-1 text-sm text-surface-400">
			{$_("admin.audit.subtitle")}
		</p>
	</section>

	{#if data.result.state === "empty"}
		<div
			class="stagger stagger-2 mt-6 flex flex-col items-center rounded-xl border border-surface-700/50 bg-surface-900 py-20 text-center"
		>
			<div
				class="flex h-12 w-12 items-center justify-center rounded-full bg-surface-800"
			>
				<svg
					class="h-6 w-6 text-surface-500"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					stroke-width="1.5"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 7.5m6 4.125l2.25 2.25m0 0l2.25-2.25M12 13.875V7.5M3.75 7.5h16.5"
					/>
				</svg>
			</div>
			<p class="mt-4 text-sm text-surface-400">
				{$_("admin.audit.empty.headline")}
			</p>
		</div>
	{:else}
		<div
			class="stagger stagger-2 mt-4 overflow-hidden rounded-xl border border-surface-700/50 bg-surface-900"
		>
			<div class="overflow-x-auto">
				<table class="w-full min-w-[720px]">
					<thead>
						<tr>
							<th
								class="px-5 py-3 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("admin.audit.col.time")}
							</th>
							<th
								class="px-5 py-3 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("admin.audit.col.actor")}
							</th>
							<th
								class="px-5 py-3 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("admin.audit.col.action")}
							</th>
							<th
								class="px-5 py-3 text-left text-[11px] font-medium uppercase tracking-wider text-surface-600"
							>
								{$_("admin.audit.col.entity")}
							</th>
						</tr>
					</thead>
					<tbody>
						{#each events as event (event.id)}
							<tr class="border-t border-surface-800 transition-colors hover:bg-surface-800/50">
								<td class="px-5 py-3 text-sm text-surface-400">
									{formatTime(event.at)}
								</td>
								<td class="px-5 py-3 text-sm font-medium text-surface-50">
									{event.actor}
								</td>
								<td class="px-5 py-3 text-sm text-surface-200">
									{event.action}
								</td>
								<td class="px-5 py-3 text-sm text-surface-200">
									{#if event.entity_type === "shipment" && event.entity_id}
										<a
											href={shipmentHref(event.entity_id)}
											class="text-accent transition-colors hover:text-accent-hover hover:underline"
										>
											{event.entity_label ||
												$_("admin.audit.entity.shipment", {
													values: { id: event.entity_id },
												})}
										</a>
									{:else if event.entity_label}
										{event.entity_label}
									{:else if event.entity_type && event.entity_id}
										{event.entity_type}:{event.entity_id}
									{:else}
										<span class="text-surface-600">—</span>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}
{/if}
