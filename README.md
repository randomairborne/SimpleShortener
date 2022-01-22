# Very simple URL shortener
```json
{
  "port": 80,
  "default": "https://example.com",
  "urls": {
    "nerd": "https://randomairborne.dev"
  }
}
```
Default is the site your root redirects to. Add more key:value pairs in the urls set to add more links. For this one example.com redirects to itself, and example.com/nerd redirects to randomairborne.dev\
Build with [rust](https://rust-lang.org)
```bash
git switch panel
cargo build --release
```
then run with `./target/bin/simple_shortener /path/to/config.json`

## Using the docker image
The docker image can be run with ```docker run -v `pwd`/config.json:/config.json randomairborne/simpleshortener:panel``` to set its configuration file.
Make sure to expose your specified port!