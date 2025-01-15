FROM rust:1.84-alpine AS build

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release --locked && \
    cp /usr/src/app/target/release/imdn /usr/bin/imdn

FROM alpine

COPY --from=build /usr/bin/imdn /usr/bin/imdn

ENV PORT=8080
VOLUME /usr/src/imdn/cache
VOLUME /usr/src/imdn/images

CMD ["imdn", "-c", "/usr/src/imdn/cache", "local", "/usr/src/imdn/images"]
