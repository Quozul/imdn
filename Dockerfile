FROM rust:alpine AS build

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/imdn
COPY . .

RUN cargo build --release

ENV PORT 8080

FROM alpine

COPY --from=build /usr/src/imdn/target/release/imdn /usr/bin/imdn

CMD ["imdn"]
