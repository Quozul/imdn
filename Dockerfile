FROM node:23-alpine AS base

FROM base AS builder

RUN apk add --no-cache gcompat
WORKDIR /app

COPY package*json ./
RUN npm ci

COPY tsconfig.json src ./

RUN npm run build && \
    npm prune --production

FROM base AS runner
WORKDIR /app

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 hono

COPY --from=builder --chown=hono:nodejs /app/node_modules /app/node_modules
COPY --from=builder --chown=hono:nodejs /app/dist /app/dist
COPY --from=builder --chown=hono:nodejs /app/package.json /app/package.json

USER hono
EXPOSE 3000
VOLUME /var/cache/image-proxy
ENV CACHE_LOCATION=/var/cache/image-proxy

CMD ["node", "/app/dist/index.js"]
