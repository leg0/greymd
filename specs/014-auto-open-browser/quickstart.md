# Quickstart: Auto-Open Browser on Startup

**Feature**: 014-auto-open-browser  
**Branch**: `014-auto-open-browser`

## Prerequisites

- Rust toolchain (edition 2024 support)
- `cargo` available on PATH

## Build

```sh
cargo build
```

## Run

```sh
# Default: opens browser automatically
cargo run

# Serve a specific directory
cargo run -- ./my-docs

# Suppress browser launch
cargo run -- --no-browser

# Suppress browser + serve a directory
cargo run -- --no-browser ./my-docs
```

## Test

```sh
# Run all tests
cargo test

# Run only browser module tests
cargo test browser

# Run only main module tests (includes CLI flag parsing)
cargo test --bin greymd
```

## Verify the Feature

1. **Auto-open works**: Run `cargo run` — your default browser should open to `http://127.0.0.1:8080` (or the fallback port shown in terminal).
2. **Correct port on fallback**: Start another instance while the first is running — the browser should open to the different port.
3. **`--no-browser` works**: Run `cargo run -- --no-browser` — no browser should open, but the URL is still printed.
4. **Graceful failure**: If testing in a headless environment (e.g., SSH session), greymd should start normally with no errors.
