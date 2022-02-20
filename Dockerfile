FROM rust:1-slim-bullseye as build

WORKDIR /simpleshortener

COPY . .

ENV RUSTFLAGS="--emit=asm"

RUN cargo build --release

# our final base
FROM debian:bullseye-slim

WORKDIR /

COPY --from=build /simpleshortene/target/release/simpleshortener .

RUN adduser --home /nonexistent --no-create-home --disabled-password simpleshortener
USER simpleshortener

CMD ["./simpleshortener"]
