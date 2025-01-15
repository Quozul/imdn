import crypto from "node:crypto";

/**
 * Generate a signed URL.
 * @param {URL} baseUrl - The non-signed URL of the resource.
 * @param {number} expiryDuration - Expiration duration in seconds.
 * @param {string} secret - The secret to use for the signature.
 * @returns {URL} - A signed URL.
 * @example
 * ```js
 * const signedUrl = generateSignedUrl(new URL("http://localhost:3000/api/thumbnail/example.jpg?lte=512"), 3_600, "super-secret-secret");
 * ```
 */
export function generateSignedUrl(baseUrl, expiryDuration, secret) {
	const issuedTimestamp = Math.floor(Date.now() / 1000);
	const expiryTimestamp = issuedTimestamp + expiryDuration;

	baseUrl.searchParams.set("is", issuedTimestamp.toString(16));
	baseUrl.searchParams.set("ex", expiryTimestamp.toString(16));

	const hmac = crypto
		.createHmac("sha256", secret)
		.update(baseUrl.toString())
		.digest("hex");

	baseUrl.searchParams.set("hm", hmac);

	return baseUrl;
}
