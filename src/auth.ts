import crypto from "node:crypto";
import { createMiddleware } from "hono/factory";
import { HTTPException } from "hono/http-exception";

export function validateSignedUrl(url: string, secret: string): boolean {
	const parsedUrl = new URL(url);
	const params = parsedUrl.searchParams;

	const issuedParam = params.get("is");
	const expiryParam = params.get("ex");

	if (!issuedParam || !expiryParam) {
		return false;
	}

	const expiryTimestamp = Number.parseInt(expiryParam, 16);
	const signature = params.get("hm");

	const now = Math.floor(Date.now() / 1000);
	if (now > expiryTimestamp) {
		return false;
	}

	params.delete("hm");
	const reconstructedBase = `${parsedUrl.origin}${parsedUrl.pathname}?${params.toString()}`;
	const expectedSignature = crypto
		.createHmac("sha256", secret)
		.update(reconstructedBase)
		.digest("hex");

	return signature === expectedSignature;
}

export const authenticationMiddleware = (secret: string) =>
	createMiddleware(async (c, next) => {
		if (!validateSignedUrl(c.req.url, secret)) {
			const status = 401;
			const res = new Response("Unauthorized", { status });
			throw new HTTPException(status, { res });
		}
		await next();
	});
