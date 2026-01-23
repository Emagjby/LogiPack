<script lang="ts">
	import Badge from '$lib/components/ui/Badge.svelte';
	import type { ShipmentStatus } from '$lib/types/shipment';

	const statusStyles: Record<ShipmentStatus, string> = {
		NEW: 'bg-zinc-700/30 text-zinc-300 border border-zinc-600/30',
		ACCEPTED: 'bg-indigo-500/10 text-indigo-300 border border-indigo-500/20',
		PROCESSED: 'bg-sky-500/10 text-sky-300 border border-sky-500/20',
		IN_TRANSIT: 'bg-blue-500/10 text-blue-400 border border-blue-500/20',
		DELIVERED: 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20',
		CANCELLED: 'bg-rose-500/10 text-rose-400 border border-rose-500/20'
	};

	const dotStyles: Record<ShipmentStatus, string> = {
		NEW: 'bg-zinc-300',
		ACCEPTED: 'bg-indigo-300',
		PROCESSED: 'bg-sky-300',
		IN_TRANSIT: 'bg-blue-400',
		DELIVERED: 'bg-emerald-400',
		CANCELLED: 'bg-rose-400'
	};

	let { status, class: className = '' } = $props<{
		status: ShipmentStatus;
		class?: string;
	}>();

	const safeStatus = () => (status ?? 'NEW') as ShipmentStatus;
</script>

<Badge class={`${statusStyles[safeStatus()]} ${className}`}>
	<span class={`w-1.5 h-1.5 rounded-full ${dotStyles[safeStatus()]}`}></span>
	{safeStatus().replace('_', ' ')}
</Badge>
