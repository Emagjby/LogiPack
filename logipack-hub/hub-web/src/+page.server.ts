import { redirect } from "@sveltejs/kit";

const SUPPORTED = new Set(["en", "bg"]);

function acceptLang(header: string | null) {
	const base = (header ?? "")
		.split(",")
		.map((p) => p.split(";")[0].trim().toLowerCase())
		.map((l) => l.split("-")[0])
		.find((l) => SUPPORTED.has(l));
	return base ?? null;
}

export const load = ({
	cookies,
	request,
}: {
	cookies: any;
	request: Request;
}) => {
	const cookie = cookies.get("lang");
	const lang =
		(cookie && SUPPORTED.has(cookie) ? cookie : null) ??
		acceptLang(request.headers.get("accept-language")) ??
		"en";

	throw redirect(302, `/${lang}`);
};
