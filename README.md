# Yari

a learning project that tries to match the [Redis][redis-home] API one-to-one (+ a multiple interfaces) while focusing on using major libraries and tools from the rust framework. 

## Roadmap
### Commands plan (just basics, not all flags) - [DOCS][redis-commands]
[
- [X] GET
- [X] SET
- [X] DEL
- [X] EXPIRE (Including randomized active expiration)

### Interfaces to be implemented
- [X] HTTP
- [ ] Raw TCP w/ Tokio
- [ ] CLI
- [ ] GRPC

## Libraries to experiment with

- [X] [Tokio][https://tokio.rs/]
- [X] [Warp][https://github.com/seanmonstar/warp]
- [ ] [Clap][https://github.com/clap-rs/clap]
- [ ] [Clippy][https://github.com/rust-lang/rust-clippy]


[redis-commands]:   https://redis.io/commands
[redis-home]:       https://redis.io
[redis-url]:        http://www.iana.org/assignments/uri-schemes/prov/redis