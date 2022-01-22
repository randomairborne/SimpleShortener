###
# Builder to compile the shortener
###
FROM rust:latest AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

###
# Now generate our smaller image
###
FROM debian:latest
COPY --from=builder /build/target/release/simple_shortener /usr/bin/simpleshortener

CMD ["/usr/bin/simpleshortener"]
