<script lang="ts">
	import { _ } from "svelte-i18n";

	let { pathname, lang }: { pathname: string; lang: string } = $props();
	const locales = [
		{ code: "en", labelKey: "navbar.locale_en" },
		{ code: "bg", labelKey: "navbar.locale_bg" },
	] as const;

	type Locale = (typeof locales)[number]["code"];

	interface NavItem {
		labelKey: string;
		href: string;
		icon: "dashboard" | "shipments";
	}

	const navItems: NavItem[] = $derived([
		{
			labelKey: "navbar.item.dashboard",
			href: `/${lang}/app/employee`,
			icon: "dashboard" as const,
		},
		{
			labelKey: "navbar.item.shipments",
			href: `/${lang}/app/employee/shipments`,
			icon: "shipments" as const,
		},
	]);

	function isActive(href: string): boolean {
		return (
			pathname === href ||
			(href !== `/${lang}/app/employee` && pathname.startsWith(href))
		);
	}

	function changeLanguage(nextLang: Locale): void {
		if (nextLang === lang) return;

		const secure = window.location.protocol === "https:" ? "; secure" : "";
		document.cookie = `lang=${nextLang}; path=/; max-age=31536000; samesite=lax${secure}`;

		const localePattern = locales
			.map((locale) =>
				locale.code.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"),
			)
			.join("|");
		const nextPath = pathname.replace(
			new RegExp(`^/(${localePattern})(?=/|$)`),
			`/${nextLang}`,
		);
		window.location.href = `${nextPath}${window.location.search}`;
	}
</script>

<aside
	class="flex h-full w-60 flex-col border-r border-surface-700/50 bg-surface-900"
>
	<!-- Brand block -->
	<div class="px-5 pt-4.25 pb-3.5">
		<div class="flex items-center gap-2.5">
			<img
				src="https://raw.githubusercontent.com/Emagjby/logipack-assets/refs/heads/main/logipack-crate-green.png"
				alt="LogiPack"
				class="h-6 w-6 rounded-sm object-cover"
			/>
			<span class="text-sm font-semibold tracking-tight text-surface-50"
				>LogiPack</span
			>
		</div>
	</div>

	<!-- Divider -->
	<div class="mx-4 h-px bg-surface-800"></div>

	<!-- Navigation section -->
	<nav class="flex-1 overflow-y-auto px-3 py-4">
		<span
			class="mb-2 block px-3 text-[10px] font-medium uppercase tracking-widest text-surface-600"
		>
			{$_("navbar.section.operations")}
		</span>

		{#each navItems as item (item.href)}
			{@const active = isActive(item.href)}
			<a
				href={item.href}
				class={[
					"group relative flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2 text-[13px] font-medium transition-all duration-150",
					"focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50 focus-visible:ring-offset-1 focus-visible:ring-offset-surface-900",
					active
						? "bg-surface-800/80 text-surface-50"
						: "text-surface-400 hover:bg-surface-800/40 hover:text-surface-200",
				]}
			>
				{#if active}
					<div
						class="absolute left-0 top-1/2 h-4 w-[3px] -translate-y-1/2 rounded-full bg-accent"
					></div>
				{/if}

				{#if item.icon === "dashboard"}
					<svg
						class={[
							"h-[18px] w-[18px] shrink-0",
							active
								? "text-accent"
								: "text-surface-500 group-hover:text-surface-400",
						]}
						viewBox="0 0 20 20"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
					>
						<rect x="3" y="3" width="6" height="6" rx="1" />
						<rect x="11" y="3" width="6" height="6" rx="1" />
						<rect x="3" y="11" width="6" height="6" rx="1" />
						<rect x="11" y="11" width="6" height="6" rx="1" />
					</svg>
				{:else if item.icon === "shipments"}
					<svg
						class={[
							"h-[18px] w-[18px] shrink-0",
							active
								? "text-accent"
								: "text-surface-500 group-hover:text-surface-400",
						]}
						viewBox="0 0 20 20"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
					>
						<path d="M3 7l7-4 7 4v6l-7 4-7-4V7z" />
						<path d="M3 7l7 4m0 0l7-4m-7 4v7" />
					</svg>
				{/if}

				<span>{$_(item.labelKey)}</span>
			</a>
		{/each}
	</nav>

	<!-- Bottom section -->
	<div class="mt-auto px-3 py-3">
		<div class="rounded-lg border border-surface-800 bg-surface-900/40 px-2.5 py-2">
			<span class="mb-1.5 block text-[10px] font-medium uppercase tracking-widest text-surface-600">
				{$_("navbar.language")}
			</span>
			<div class="grid grid-cols-2 gap-1.5">
				{#each locales as locale (locale.code)}
					<button
						type="button"
						onclick={() => changeLanguage(locale.code)}
						class={[
							"cursor-pointer rounded-md px-2 py-1.5 text-[11px] font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50",
							lang === locale.code
								? "bg-surface-700 text-surface-100"
								: "bg-surface-800/70 text-surface-400 hover:bg-surface-800 hover:text-surface-200",
						]}
						aria-label={$_(locale.labelKey)}
						aria-pressed={lang === locale.code}
					>
						{$_(locale.labelKey)}
					</button>
				{/each}
			</div>
		</div>
		<span class="mt-2 block px-2 text-[10px] text-surface-700">v0.1.0</span>
	</div>
</aside>
