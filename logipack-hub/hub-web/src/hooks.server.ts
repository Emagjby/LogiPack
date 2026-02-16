import type { Handle } from "@sveltejs/kit";

const SUPPORTED = ["en", "bg"] as const;
type Lang = (typeof SUPPORTED)[number];

function parseAcceptLanguage(header: string | null): string[] {
	if (!header) return [];
	return header
		.split(",")
		.map((part) => part.split(";")[0]?.trim().toLowerCase())
		.filter(Boolean);
}

function pickSupported(preferred: string[]): Lang | null {
	for (const raw of preferred) {
		const base = raw.split("-")[0];
		if (SUPPORTED.includes(base as Lang)) return base as Lang;
	}
	return null;
}

function setLangCookie(event: Parameters<Handle>[0]["event"], lang: Lang) {
	// persist preference for future visits to "/"
	if (event.cookies.get("lang") !== lang) {
		event.cookies.set("lang", lang, {
			path: "/",
			sameSite: "lax",
			httpOnly: false, // set true if you never need to read it on client
			secure: event.url.protocol === "https:",
			maxAge: 60 * 60 * 24 * 365, // 1 year
		});
	}
}

export const handle: Handle = async ({ event, resolve }) => {
	const { url, cookies, request } = event;

	const seg = url.pathname.split("/")[1];

	// If already localized route, set locals + cookie and continue
	if (SUPPORTED.includes(seg as Lang)) {
		const lang = seg as Lang;
		event.locals.lang = lang;
		setLangCookie(event, lang);
		return resolve(event);
	}

	// Decide best lang for root / non-lang paths
	const cookieLang = cookies.get("lang");
	const accept = pickSupported(
		parseAcceptLanguage(request.headers.get("accept-language")),
	);

	const lang =
		(SUPPORTED.includes(cookieLang as Lang) ? (cookieLang as Lang) : null) ??
		accept ??
		"en";

	// Redirect "/" (and optionally other non-lang paths)
	if (url.pathname === "/") {
		return new Response(null, {
			status: 302,
			headers: { location: `/${lang}` },
		});
	}

	return new Response(null, {
		status: 302,
		headers: { location: `/${lang}${url.pathname}` },
	});
};
