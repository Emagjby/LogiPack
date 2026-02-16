import { init, addMessages, locale } from "svelte-i18n";
import en from "./locales/en.json";
import bg from "./locales/bg.json";

const VALID_LOCALES = ["en", "bg"] as const;
type Locale = (typeof VALID_LOCALES)[number];

let initialized = false;

export function ensureI18n(lang: Locale): Promise<void> {
	return new Promise((resolve) => {
		if (!initialized) {
			addMessages("en", en);
			addMessages("bg", bg);
			init({
				fallbackLocale: "en",
				initialLocale: lang,
			});
			initialized = true;
		} else {
			locale.set(lang);
		}
		resolve();
	});
}
