FROM rust:1-slim-bullseye as build

WORKDIR /simple_shortener

COPY . .

ENV RUSTFLAGS="--emit=asm"

RUN cargo build --release

# our final base
FROM debian:bullseye-slim

WORKDIR /

COPY --from=build /simple_shortener/target/release/simple_shortener .

RUN adduser --home /nonexistent --no-create-home --disabled-password simple_shortener
USER simple_shortener

CMD ["./simple_shortener"]
