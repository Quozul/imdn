import fs from "node:fs";
import fsPromises from "node:fs/promises";
import path from "node:path";
import { PassThrough } from "node:stream";
import type { HttpBindings } from "@hono/node-server";
import { RESPONSE_ALREADY_SENT } from "@hono/node-server/utils/response";
import { zValidator } from "@hono/zod-validator";
import { Hono } from "hono";
import sharp from "sharp";
import { z } from "zod";
import { authenticationMiddleware } from "./auth.js";
import type { AppSettings } from "./cli.js";
import { getObjectStream } from "./getObjectStream.js";

const AVAILABLE_FORMATS = ["jpeg", "png", "webp", "gif"] as const;

export function buildThumbnailRouter(settings: AppSettings) {
	return new Hono<{ Bindings: HttpBindings }>().get(
		"/:image_id",
		authenticationMiddleware(settings.secret),
		zValidator(
			"query",
			z.object({
				lte: z.coerce.number().default(512),
				quality: z.coerce.number().optional(),
				format: z.enum(AVAILABLE_FORMATS).default("jpeg"),
			}),
		),
		async (c) => {
			const { image_id } = c.req.param();
			const { lte, format, quality } = c.req.valid("query");

			const { cachePath, cacheHit } = await getCache(
				settings.cacheLocation,
				format,
				lte,
				image_id,
			);

			if (cacheHit && cachePath) {
				const stream = fs.createReadStream(cachePath);
				const { outgoing } = c.env;
				outgoing.writeHead(200, {
					"Content-Type": `image/${format}`, // TODO: The mime type may not be correct
				});
				stream.pipe(outgoing);
				return RESPONSE_ALREADY_SENT;
			}

			if (settings.s3Configuration === null) {
				return new Response("Service Unavailable", { status: 502 });
			}

			const { s3ObjectStream, contentType } = await getObjectStream(
				settings.s3Configuration,
				image_id,
			);

			if (s3ObjectStream === undefined) {
				return new Response("Not Found", { status: 404 });
			}

			if (!contentType.startsWith("image/")) {
				return new Response("Requested resource is not an image", {
					status: 400,
				});
			}

			const transformer = sharp()
				.resize({
					width: lte,
					height: lte,
					fit: sharp.fit.inside,
					position: sharp.strategy.entropy,
					kernel: sharp.kernel.nearest,
				})
				.toFormat(format, {
					mozjpeg: true,
					quality,
				});

			const { outgoing } = c.env;
			outgoing.writeHead(200, {
				"Content-Type": `image/${format}`, // TODO: The mime type may not be correct
			});

			if (cachePath) {
				const tee = new PassThrough();
				const cacheStream = fs.createWriteStream(cachePath);

				tee.pipe(outgoing);
				tee.pipe(cacheStream).on("error", (err) => {
					console.error("Cache stream error", err);
				});

				s3ObjectStream
					.pipe(transformer)
					.on("error", (err) => {
						console.error("Could not transform the image", err);
					})
					.pipe(tee);
			} else {
				s3ObjectStream.pipe(transformer).pipe(outgoing);
			}

			return RESPONSE_ALREADY_SENT;
		},
	);
}

function getCachePath(
	cacheLocation: string,
	format: string,
	lte: number,
	imageId: string,
): string | undefined {
	if (cacheLocation !== undefined) {
		const cacheKey = `${format}_${lte}_${imageId}`;
		return path.join(cacheLocation, cacheKey);
	}
}

async function getCache(
	cacheLocation: string,
	format: string,
	lte: number,
	imageId: string,
): Promise<{ cachePath: string | undefined; cacheHit: boolean }> {
	const cachePath = getCachePath(cacheLocation, format, lte, imageId);

	if (cachePath !== undefined) {
		try {
			await fsPromises.access(
				cachePath,
				fsPromises.constants.R_OK | fsPromises.constants.W_OK,
			);
			return { cachePath, cacheHit: true };
		} catch (_) {
			return { cachePath, cacheHit: false };
		}
	}

	return { cachePath, cacheHit: false };
}
