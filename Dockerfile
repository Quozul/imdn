FROM rust:alpine AS build

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/imdn
COPY . .

RUN cargo build --release

FROM alpine

COPY --from=build /usr/src/imdn/target/release/imdn /usr/bin/imdn

ENV PORT 8080

CMD ["imdn", "-c", "/usr/src/imdn/cache", "local", "/usr/src/imdn/images"]
