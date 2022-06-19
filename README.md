# ip_query_tide
a web server for show client IP and ip2region with [tide](https://github.com/http-rs/tide) and [ip2region](https://github.com/lionsoul2014/ip2region).

## Getting started
clone the repository, and cd the project root dictionary:
```rust
cargo run
```
then use a browser or curl to visit http://127.0.0.1:3000/ip  or http://127.0.0.1:3000/ip/2region see the result.
## Release build
in the project root dictionary:
```rust
cargo build --release
```
you can find execute file in `target/release/ip_query` .    
or for linux you can use:
```rust
cargo build --release --target x86_64-unknown-linux-musl
```
you can find execute file in `target/x86_64-unknown-linux-musl/release/ip_query`

## License
Licensed under either of
+ Apache License, Version 2.0 (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
+ MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
at your option.