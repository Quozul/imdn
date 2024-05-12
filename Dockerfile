FROM rust:latest

WORKDIR /usr/src/imdn
COPY . .

RUN cargo install --path .

CMD ["imdn"]
