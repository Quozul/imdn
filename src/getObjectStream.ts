import type { Readable } from "node:stream";
import { GetObjectCommand } from "@aws-sdk/client-s3";
import type { AppSettings } from "./cli.js";

export async function getObjectStream(
	s3Configuration: NonNullable<AppSettings["s3Configuration"]>,
	key: string,
): Promise<Readable | undefined> {
	const response = await s3Configuration.s3Client.send(
		new GetObjectCommand({
			Bucket: s3Configuration.bucketName,
			Key: key,
		}),
	);
	return response.Body as Readable;
}
