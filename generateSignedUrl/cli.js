import { parseArgs } from "node:util";
import { generateSignedUrl } from "./nodejs/generateSignedUrl.js";

const { values } = parseArgs({
	args: process.argv.slice(2),
	options: {
		secret: {
			type: "string",
			default: "super-secret-secret",
		},
		origin: {
			type: "string",
			default: "http://localhost:3000",
		},
		resource: {
			type: "string",
		},
		duration: {
			type: "string",
			default: "3600",
		},
	},
});

if (!values.resource) {
	console.error("Resource is required");
	process.exit(1);
}

const duration = Number.parseInt(values.duration.toString());
const baseUrl = new URL(`${values.origin}/api/${values.resource}`);

const signedUrl = generateSignedUrl(baseUrl, duration, values.secret);
console.log(signedUrl.href);
