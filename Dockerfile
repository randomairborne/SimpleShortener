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
FROM alpine

COPY --from=builder /build/target/release/simple_shortener /usr/bin/simpleshortener

ENTRYPOINT ["/usr/bin/simpleshortener"]
CMD []
