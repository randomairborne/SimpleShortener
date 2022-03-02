FROM rust:1-slim-bullseye as build

WORKDIR /

ENV RUSTFLAGS="--emit=asm"

RUN USER=0 cargo new simpleshortener

COPY Cargo.toml Cargo.lock /simpleshortener/

WORKDIR /simpleshortener

RUN cargo build --release

COPY src migrations sqlx-data.json /simpleshortener/

RUN touch src/main.rs && cargo build --release

# our final base
FROM debian:bullseye-slim

WORKDIR /

COPY --from=build /simpleshortener/target/release/simpleshortener .

USER 9999

CMD ["./simpleshortener"]
