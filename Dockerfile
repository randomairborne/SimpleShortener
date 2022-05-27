FROM rust:alpine as build

WORKDIR /

ENV RUSTFLAGS="--emit=asm"

COPY . .

RUN cargo build --release

# our final base
FROM alpine

WORKDIR /

COPY --from=build /simpleshortener/target/release/simpleshortener .

USER 9999

CMD ["./simpleshortener"]
