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

You can then talk to the server with a Redis client or via raw RESP over netcat.

Examples:

- Using `redis-cli` (if installed):

```bash
redis-cli -p 6381 PING
# -> PONG
```

- Using `nc`/`netcat` with raw RESP bytes:

```bash
echo -ne "*1\r\n$4\r\nPING\r\n" | nc 127.0.0.1 6381
# -> +PONG\r\n
```

RESP examples for SET/GET (raw):

```text
SET name nebiyu (RESP encoded):
*3\r\n$3\r\nSET\r\n$4\r\nname\r\n$6\r\nnebiyu\r\n

GET name (RESP encoded):
## rusty_redis — minimal Redis-like server (demo)

A tiny, educational Redis-like server written in Rust. It implements the RESP protocol and a few basic commands so you can explore how Redis works end-to-end.

Key points
- RESP parser/serializer (RESP protocol support)
- Async TCP server using Tokio (default: 127.0.0.1:6381)
- Simple commands: PING, SET, GET (string values)
- Background DB thread communicates via mpsc/oneshot (no persistence)

Quick start
- Build & run: `cargo run`
- Connect with: `redis-cli -p 6381` (or send raw RESP over TCP)

Tests
- Integration tests live in `tests/test.rs`. Run them with `cargo test`.

Limitations
- Learning/demo only: not production-ready (no persistence, limited commands, no clustering).

If you want more commands or persistence, feel free to open a PR or extend the code.
- `src/resp_protocol.rs` - RESP parsing and helpers

