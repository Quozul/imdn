import assert from "node:assert";
import { describe, it } from "node:test";
import { generateSignedUrl } from "./generateSignedUrl.js";

describe("Auth test", () => {
	it("Should add three parameters to the URL", async (context) => {
		// Given
		const baseUrl = new URL("http://localhost:3000/image/example.jpg");
		const expirationDuration = 3_600; // in seconds
		const secret = "super-secret";
		const expectedSignedUrl =
			"http://localhost:3000/image/example.jpg?is=0&ex=e10&hm=e2e7dc91f104bf0f60cfa7657fff1fc773d7cffc1ac99f65392fb6370b45c2ab";

		context.mock.timers.enable({ apis: ["Date"] });

		// When
		const signedUrl = generateSignedUrl(baseUrl, expirationDuration, secret);

		// Then
		assert(signedUrl.href === expectedSignedUrl);
	});

	it("Should add three parameters to the URL while preserving the new parameters", async (context) => {
		// Given
		const baseUrl = new URL(
			"http://localhost:3000/image/example.jpg?key=value",
		);
		const expirationDuration = 3_600; // in seconds
		const secret = "super-secret";
		const expectedSignedUrl =
			"http://localhost:3000/image/example.jpg?key=value&is=0&ex=e10&hm=049cd6fcb6b95546f79a0483fe4547591590edd9dfc52cdcabaa22163ebb08c5";

		context.mock.timers.enable({ apis: ["Date"] });

		// When
		const signedUrl = generateSignedUrl(baseUrl, expirationDuration, secret);

		// Then
		assert(signedUrl.toString() === expectedSignedUrl);
	});

	it("Should add expiration date timestamp", async (context) => {
		// Given
		const baseUrl = new URL("http://localhost:3000/image/example.jpg");
		const expirationDuration = 3_600; // in seconds
		const secret = "super-secret";
		const expectedSignedUrl =
			"http://localhost:3000/image/example.jpg?is=67893702&ex=67894512&hm=6c86f9340ef78ca946370ac862b1232b87a4e175dd18080bba10c821257b8198";

		context.mock.timers.enable({ apis: ["Date"], now: 1737045762000 });

		// When
		const signedUrl = generateSignedUrl(baseUrl, expirationDuration, secret);

		// Then
		assert(signedUrl.toString() === expectedSignedUrl);
	});
});
