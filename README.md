# Simple URL shortener

A very simple URL shortener, which is easy to configure and quite speedy.
Later it is planned to add some analytics.

If you have any issues you can contact me on discord, `valkyrie_pilot#2707`, or via email [valk@vky.one](valk@vky.one)

You can edit links at /simpleshortener/ on the domain you use to host it.

This branch is for testing a new significant feature, do not use it in production!

## Install
Create this config file:
```toml
# Port to run SimpleShortener on. Can be overridden with the `PORT` environment variable.
port = 24529
# Where to keep files
database = "postgres://username:password@localhost/database"
# A key:value list of username:sha256-hashed passwords
users = { admin = "fc8252c8dc55839967c58b9ad755a59b61b67c13227ddae4bd3f78a38bf394f7" }

# Uncomment to enable TLS
# This also requires the TLS-enabled binary:
# if the default binary is downloaded, TLS is entirely ignored
# [tls]
# port = 443
# certfile = "/path/to/cert.pem"
# keyfile = "/path/to/key.pem"
```

then run with `./target/bin/simpleshortener /path/to/config.toml`


## Building
You can build from source with [rust](https://rust-lang.org)

### Without TLS support
```bash
git clone --branch nodb https://github.com/randomairborne/SimpleShortener.git
cargo build --release
```

### With TLS support
```bash
git clone --branch nodb https://github.com/randomairborne/SimpleShortener.git
cargo build --release --features tls
```
