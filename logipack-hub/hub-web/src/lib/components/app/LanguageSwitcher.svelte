<script lang="ts">
	import { _ } from "svelte-i18n";

	let {
		pathname,
		lang,
		showLabel = true,
		variant = "segmented",
	}: {
		pathname: string;
		lang: string;
		showLabel?: boolean;
		variant?: "segmented" | "dropdown";
	} = $props();

	const locales = [
		{ code: "en", labelKey: "navbar.locale_en" },
		{ code: "bg", labelKey: "navbar.locale_bg" },
	] as const;

	type Locale = (typeof locales)[number]["code"];

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
		window.location.href = `${nextPath}${window.location.search}${window.location.hash}`;
	}
</script>

{#if variant === "dropdown"}
	<div>
		{#if showLabel}
			<label
				for="profile-language"
				class="mb-1.5 block text-[10px] font-medium uppercase tracking-widest text-surface-600"
			>
				{$_("navbar.language")}
			</label>
		{/if}
		<select
			id="profile-language"
			class="w-full cursor-pointer rounded-lg border border-surface-700/70 bg-surface-900 px-3 py-2 text-sm font-medium text-surface-50 outline-none transition-colors focus:border-accent/70 focus:ring-1 focus:ring-accent/50"
			value={lang}
			onchange={(event) =>
				changeLanguage((event.currentTarget as HTMLSelectElement).value as Locale)}
			aria-label={$_("navbar.language")}
		>
			{#each locales as locale (locale.code)}
				<option value={locale.code}>{$_(locale.labelKey)}</option>
			{/each}
		</select>
	</div>
{:else}
	<div class="rounded-lg border border-surface-800 bg-surface-900/40 px-2.5 py-2">
		{#if showLabel}
			<span class="mb-1.5 block text-[10px] font-medium uppercase tracking-widest text-surface-600">
				{$_("navbar.language")}
			</span>
		{/if}
		<div
			class="grid grid-cols-2 gap-1.5"
			role="group"
			aria-label={$_("navbar.language")}
		>
			{#each locales as locale (locale.code)}
				<button
					type="button"
					onclick={() => changeLanguage(locale.code)}
					class={[
						"cursor-pointer rounded-md px-2 py-1.5 text-[11px] font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent/50",
						lang === locale.code
							? "bg-surface-700 text-surface-50"
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
{/if}
