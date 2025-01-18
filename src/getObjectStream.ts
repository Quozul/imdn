import type { Readable } from "node:stream";
import { GetObjectCommand } from "@aws-sdk/client-s3";
import type { AppSettings } from "./cli.js";

export async function getObjectStream(
	s3Configuration: NonNullable<AppSettings["s3Configuration"]>,
	key: string,
): Promise<{ s3ObjectStream: Readable | undefined; contentType: string }> {
	const response = await s3Configuration.s3Client.send(
		new GetObjectCommand({
			Bucket: s3Configuration.bucketName,
			Key: key,
		}),
	);
	const contentType = response.ContentType ?? "application/octet-stream";
	const s3ObjectStream = response.Body as Readable | undefined;
	return { s3ObjectStream, contentType };
}
