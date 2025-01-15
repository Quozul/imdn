import type { HttpBindings } from "@hono/node-server";
import { serve } from "@hono/node-server";
import { Hono } from "hono";
import { getProgramSettings } from "./cli.js";
import { buildImageRouter } from "./image.js";
import { buildThumbnailRouter } from "./thumbnail.js";

const settings = getProgramSettings();

const app = new Hono<{ Bindings: HttpBindings }>()
	.basePath("/api")
	.route("/image", buildImageRouter(settings))
	.route("/thumbnail", buildThumbnailRouter(settings))
	.notFound((c) => c.json({ message: "Not Found", ok: false }, 404));

const port = Number.parseInt(process.env.PORT || "3000");
console.log(`Server is running on http://localhost:${port}`);

serve({
	fetch: app.fetch,
	port,
});
