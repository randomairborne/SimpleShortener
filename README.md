# Simple URL shortener

A very simple URL shortener, which is easy to configure and quite speedy.
Later it is planned to add some analytics.

If you have any issues you can contact me on discord, `valkyrie_pilot#2707`

You can edit links at /simpleshortener/

## Docker install
```bash
mkdir -p /opt/simpleshortener/
cd /opt/simpleshortener
curl -fsSl https://raw.githubusercontent.com/randomairborne/SimpleShortener/master/docker-compose.yml -o docker-compose.yml
echo "users = { admin = \"fc8252c8dc55839967c58b9ad755a59b61b67c13227ddae4bd3f78a38bf394f7\"}" > config.toml
# Replace users with the proper username and password
nano config.toml
docker-compose up -d
```

## Hardware install
Create this config file:
```toml
# Port to run SimpleShortener on. Can be overridden with the `PORT` environment variable.
port = 24529
# Postgres database URL.
database = "postgres://username:password@localhost/database"
# A key:value list of username:sha256-hashed passwords
users = { admin = "fc8252c8dc55839967c58b9ad755a59b61b67c13227ddae4bd3f78a38bf394f7" }
```

## Building
You can build from source with [rust](https://rust-lang.org)
```bash
git clone https://github.com/randomairborne/SimpleShortener.git
SQLX_OFFLINE="true" cargo build --release
```
then run with `./target/bin/simpleshortener /path/to/config.toml`

