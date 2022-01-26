# Simple URL shortener
```toml
# Port to run SimpleShortener on. Can be overridden with the `PORT` environment variable.
port = 24529
# Postgres database URL.
database = "postgres://username:password@localhost/database"
# A key:value list of username:sha256-hashed passwords
users = { IT = "86ae7f34fa6f2df4487f293e671b4f12290cfb116b728d95d31b703759daf2c7", marketing = "b3bd546e40e984a3067961591feea0c1a253051896e653bba6b8302317987ed3" }

```
Build with [rust](https://rust-lang.org)
```bash
git clone https://github.com/randomairborne/SimpleShortener.git
SQLX_OFFLINE="true" cargo build --release
```
then run with `./target/bin/simple_shortener /path/to/config.toml`

If you have any issues you can contact me on discord, `valkyrie_pilot#2707`
