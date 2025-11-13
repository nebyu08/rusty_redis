# 🦀 rusty_redis

A lightweight Redis-style key-value store built in Rust using Tokio. Supports RESP protocol, async clients, persistence, and key expiration.

---

## Features

### Core Commands

| Command | Description |
|---------|-------------|
| `SET key value` | Set a key-value pair |
| `SET key value EX seconds` | Set key-value with expiration time |
| `GET key` | Retrieve value or return Null |
| `PING` | Returns PONG |

###  Protocol Support

Fully implements **RESP2** (REdis Serialization Protocol):
- Bulk strings
- Arrays
- Simple strings
- Null responses
- Error messages

###  Async Server

- **Tokio-based** async runtime
- Handles many clients concurrently with non-blocking I/O
- High-performance concurrent request handling

###  Persistence

Every successful SET operation triggers a JSON snapshot write:

- **Durable**: Data persisted to disk
- **Crash-safe**: Atomic writes with temporary file `.tmp` pattern
- **Human-readable**: JSON format for easy inspection

Snapshot format includes:
- `value`: The stored value
- `expiration`: Optional expiration time (remaining seconds)

---

## Installation & Usage

(Add installation and usage instructions here)

## License

(Add license information here)
