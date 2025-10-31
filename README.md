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
*2\r\n$3\r\nGET\r\n$4\r\nname\r\n
```

## Tests

This repository includes integration tests in `tests/test.rs`. The tests start a server process with `cargo run` and then connect a `tokio` TCP client to `127.0.0.1:6381` to send RESP-encoded requests and assert responses.

Run the full test suite with:

```bash
cargo test
```

Notes and troubleshooting for tests:

- The tests use the `serial_test` crate and the `#[serial]` attribute so they do not interfere with each other while spawning a server. Do not run multiple instances of the server on the same port.
- If a test fails with connection errors, ensure nothing else is listening on port 6381 or increase the small startup delay in the tests (`sleep(Duration::from_millis(300))`) if your machine needs a bit more time to start the server.
- If you want to run a single test by name, use e.g. `cargo test test_ping` which will run the test whose name contains `test_ping`.

## Project layout

- `src/main.rs` - application entry point (server implementation and main loop)
- `src/resp_protocol.rs` - RESP parsing and helpers
- `tests/test.rs` - integration tests that exercise PING/SET/GET and a simple echo test

## Contributing / Development notes

- Use `cargo fmt` and `cargo clippy` to keep code style and catch simple lint issues.
- The server is intentionally small — it is meant as a learning project. Consider adding more RESP types, persistence, or other Redis commands as exercises.

If something in the tests or run experience is unclear, open an issue or edit this README with specifics about your environment (OS, rustc version) and the failing output.

