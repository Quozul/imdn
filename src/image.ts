import type { HttpBindings } from "@hono/node-server";
import { RESPONSE_ALREADY_SENT } from "@hono/node-server/utils/response";
import { Hono } from "hono";
import mime from "mime";
import { authenticationMiddleware } from "./auth.js";
import type { AppSettings } from "./cli.js";
import { getObjectStream } from "./getObjectStream.js";

export function buildImageRouter(settings: AppSettings) {
	return new Hono<{ Bindings: HttpBindings }>().get(
		"/:image_id",
		authenticationMiddleware(settings.secret),
		async (c) => {
			if (settings.s3Configuration === null) {
				return new Response("Service Unavailable", { status: 502 });
			}

			const { image_id } = c.req.param();
			const s3ObjectStream = await getObjectStream(
				settings.s3Configuration,
				image_id,
			);

			if (s3ObjectStream === undefined) {
				return new Response("Not Found", { status: 404 });
			}

			const { outgoing } = c.env;

			const contentType = mime.getType(image_id) ?? "application/octet-stream";
			outgoing.writeHead(200, {
				"Content-Type": contentType,
			});
			s3ObjectStream.pipe(outgoing);
			return RESPONSE_ALREADY_SENT;
		},
	);
}
