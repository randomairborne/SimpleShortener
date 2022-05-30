FROM rust:alpine as build

WORKDIR /

ENV SQLX_OFFLINE=true
ENV RUSTFLAGS="--emit=asm"

COPY . .

RUN apk add musl-dev openssl-dev pkgconf
RUN cargo build --release

# our final base
FROM alpine

WORKDIR /

COPY --from=build /simpleshortener/target/release/simpleshortener .

USER 9999
EXPOSE 3000

CMD ["./simpleshortener"]
