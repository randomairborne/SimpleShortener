FROM rust:1-slim-bullseye as build

WORKDIR /simple_shortner

COPY . .

ENV RUSTFLAGS="--emit=asm"

RUN cargo build --release

# our final base
FROM debian:bullseye-slim

WORKDIR /

COPY --from=build /simple_shortner/target/release/simple_shortner .

RUN adduser --home /nonexistent --no-create-home --disabled-password simple_shortner
USER simple_shortner

CMD ["./simple_shortner"]
