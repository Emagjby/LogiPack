import { ensureI18n } from "$lib/i18n";
import type { LayoutLoad } from "./$types";

const VALID_LOCALES = ["en", "bg"] as const;
type Locale = (typeof VALID_LOCALES)[number];

function isValidLocale(lang: string): lang is Locale {
	return VALID_LOCALES.includes(lang as Locale);
}

export const load: LayoutLoad = async ({ params }) => {
	const lang = isValidLocale(params.lang) ? params.lang : "en";
	await ensureI18n(lang);

	return {
		lang,
	};
};
