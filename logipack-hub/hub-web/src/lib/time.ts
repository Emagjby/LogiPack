export function ago(mins: number, now: Date = new Date()): string {
	const m = Math.max(0, Math.floor(mins));
	const d = new Date(now.getTime() - m * 60_000);

	const hh = String(d.getHours()).padStart(2, "0");
	const mm = String(d.getMinutes()).padStart(2, "0");
	return `${hh}:${mm}`;
}
