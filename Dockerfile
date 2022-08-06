FROM rust:alpine as build

WORKDIR /build

ENV SQLX_OFFLINE="true"

COPY . .

RUN apk add musl-dev openssl-dev pkgconf
RUN cargo build --release

# our final base
FROM alpine

WORKDIR /

COPY --from=build /build/target/release/simpleshortener /usr/bin/simpleshortener

USER 9999
EXPOSE 8080
ENV log=warn
ENV DATABASE_URL=sqlite:///data/links.db
VOLUME [ "/data" ]

CMD ["simpleshortener"]
