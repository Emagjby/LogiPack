<script lang="ts">
	import flagGB from "flagpack-core/svg/m/GB-UKM.svg";
	import flagBG from "flagpack-core/svg/m/BG.svg";

	let {
		pathname,
		lang,
		align = "right",
		fullWidth = false,
		showCurrentLabel = false,
	}: {
		pathname: string;
		lang: string;
		align?: "right" | "left";
		fullWidth?: boolean;
		showCurrentLabel?: boolean;
	} = $props();

	const locales = [
		{ code: "en", flag: flagGB, label: "English" },
		{ code: "bg", flag: flagBG, label: "Bulgarian" },
	] as const;

	type Locale = (typeof locales)[number]["code"];

	let dropdownOpen = $state(false);
	let currentLocale = $derived(
		locales.find((locale) => locale.code === lang) ?? locales[0],
	);

	function changeLanguage(nextLang: Locale): void {
		dropdownOpen = false;
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
		window.location.href = `${nextPath}${window.location.search}${window.location.hash}`;
	}

	function handleClickOutside(event: MouseEvent): void {
		const target = event.target as HTMLElement;
		if (!target.closest("[data-lang-dropdown]")) {
			dropdownOpen = false;
		}
	}

	function handleEscapeClose(event: KeyboardEvent): void {
		if (event.key === "Escape") {
			dropdownOpen = false;
		}
	}
</script>

<svelte:window onclick={handleClickOutside} />

<div class={["relative", fullWidth && "w-full"]} data-lang-dropdown>
	<button
		type="button"
		aria-label="Change language"
		aria-expanded={dropdownOpen}
		aria-haspopup="menu"
		class={[
			"flex cursor-pointer items-center rounded-lg border border-white/10 px-2.5 py-2 text-sm font-medium text-surface-400 transition-colors duration-200 hover:border-white/20 hover:text-surface-50 focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-surface-950",
			fullWidth ? "w-full justify-between gap-3" : "gap-1.5",
		]}
		onclick={() => (dropdownOpen = !dropdownOpen)}
		onkeydown={handleEscapeClose}
	>
		<span class="flex min-w-0 items-center gap-2">
			<img
				src={currentLocale.flag}
				alt={currentLocale.label}
				class="h-4 w-5 rounded-sm object-cover"
			/>
			{#if showCurrentLabel}
				<span class="truncate text-surface-200">{currentLocale.label}</span>
			{/if}
		</span>
		<svg
			class={[
				"h-3.5 w-3.5 text-surface-400 transition-transform duration-200",
				dropdownOpen && "rotate-180",
			]}
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
		>
			<polyline points="6 9 12 15 18 9" />
		</svg>
	</button>

	{#if dropdownOpen}
		<ul
			role="menu"
			aria-label="Select language"
			class={[
				"absolute z-20 mt-1.5 overflow-hidden rounded-lg border border-white/10 bg-surface-900/95 py-1 shadow-xl shadow-black/20 backdrop-blur-md",
				fullWidth ? "w-full" : "min-w-[8rem]",
				align === "left" ? "left-0" : "right-0",
			]}
			onkeydown={handleEscapeClose}
		>
			{#each locales as locale (locale.code)}
				<li role="none">
					<button
						type="button"
						role="menuitem"
						aria-current={locale.code === lang ? "true" : undefined}
						class={[
							"flex w-full cursor-pointer items-center gap-2.5 px-3 py-1.5 text-sm transition-colors duration-150",
							locale.code === lang
								? "font-medium text-accent"
								: "text-surface-200 hover:bg-white/5 hover:text-surface-50",
						]}
						onclick={() => changeLanguage(locale.code)}
					>
						<img
							src={locale.flag}
							alt={locale.label}
							class="h-4 w-5 rounded-sm object-cover"
						/>
						{locale.label}
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</div>
