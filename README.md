# Very simple URL shortener
```toml
# Port to run SimpleShortener on. Can be overridden with the `PORT` environment variable.
port = 24529
# Postgres database URL.
database = "postgres://username:password@localhost/database"
# A key:value list of username:sha256-hashed passwords
users = { "IT" = "ASecurePassword", "marketing" = "AnotherSecurePassword" }

```
Build with [rust](https://rust-lang.org)
```bash
git switch panel
cargo build --release
```
then run with `./target/bin/simple_shortener /path/to/config.toml`
