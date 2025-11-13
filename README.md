# rusty_redis

A tiny Redis-like server written in Rust that speaks the RESP (REdis Serialization Protocol).

This project is a small learning implementation of a Redis-style in-memory server. It implements basic RESP command parsing and a handful of commands used by the tests:

- PING -> responds with +PONG
- SET <key> <value> -> stores a string value and responds with +OK
- GET <key> -> returns the stored value as a bulk string (e.g. $4\r\nrust\r\n)

The server binary is built with Cargo and the repository includes integration tests under `tests/test.rs` which exercise the server by spawning `cargo run` and talking RESP over TCP (default: 127.0.0.1:6381).

## Requirements

- Rust toolchain (rustc + cargo). Install via https://rustup.rs if needed.

## Build

To build the project:

```bash
cargo build --release
```

## Run the server

Start the server locally (it will listen on 127.0.0.1:6381 by default):

```bash
cargo run --quiet
```
