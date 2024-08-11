FROM rust:alpine AS build

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/imdn
COPY . .

RUN cargo build --release

FROM alpine

COPY --from=build /usr/src/imdn/target/release/imdn /usr/bin/imdn

ENV PORT 8080

CMD ["imdn", "-r", "/usr/src/imdn/images", "-c", "/usr/src/imdn/cache"]
