import { parseArgs } from "node:util";
import { S3Client } from "@aws-sdk/client-s3";

export type AppSettings = {
	s3Configuration: {
		bucketName: string;
		s3Client: S3Client;
	} | null;
	cacheLocation: string;
	cdnOrigin: string;
	secret: string;
};

export function getProgramSettings(): AppSettings {
	const { values } = parseArgs({
		args: process.argv.slice(2),
		options: {
			"s3-bucket": {
				type: "string",
			},
			"s3-endpoint": {
				type: "string",
			},
			"s3-region": {
				type: "string",
			},
			"cache-location": {
				type: "string",
				default: "/var/cache/image-proxy",
			},
			origin: {
				type: "string",
				default: "http://localhost:3000",
			},
		},
	});

	const partialSettings: AppSettings = {
		s3Configuration: null,
		cacheLocation: values["cache-location"],
		cdnOrigin: values.origin,
		secret: process.env.SECRET || "super-secret-secret",
	};

	const bucketName = values["s3-bucket"];
	const endpoint = values["s3-endpoint"];
	const region = values["s3-region"];

	if (bucketName && endpoint && region) {
		const client = new S3Client({
			endpoint: endpoint,
			forcePathStyle: true,
			region: region,
		});
		return {
			...partialSettings,
			s3Configuration: {
				bucketName,
				s3Client: client,
			},
		};
	}

	return partialSettings;
}
