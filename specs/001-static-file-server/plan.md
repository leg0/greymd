# Implementation Plan: Static File Server

**Branch**: `001-static-file-server` | **Date**: 2026-02-20 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-static-file-server/spec.md`

## Summary

Build a local-only HTTP file server in Rust (edition 2024) with zero external dependencies. The binary accepts an optional directory path (defaults to cwd), binds to 127.0.0.1:8080, and serves files with correct Content-Type headers. Thread-per-connection model using `std::thread::spawn` for concurrent request handling.

## Technical Context

**Language/Version**: Rust 2024 edition (rustc 1.85+)
**Primary Dependencies**: None (zero external crates per constitution)
**Storage**: Filesystem (read-only access to served directory)
**Testing**: `cargo test` (unit tests + integration tests)
**Target Platform**: Linux (localhost only, no TLS)
**Project Type**: Single binary crate
**Performance Goals**: <100ms response for files under 1MB (SC-004)
**Constraints**: Zero external dependencies, localhost-only, minimal memory/CPU usage
**Scale/Scope**: Single user, localhost, concurrent connections via thread-per-connection

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | TDD: tests written before implementation, `cargo test` before commits |
| II. Minimal Dependencies | ✅ PASS | Zero external crates — HTTP parsing, MIME mapping, path handling all implemented with `std` |
| III. Minimum Resource Usage | ✅ PASS | Borrowing for path/header handling, read files into buffer only when serving, thread-per-connection (threads cleaned up on disconnect) |

## Project Structure

### Documentation (this feature)

```text
specs/001-static-file-server/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── main.rs          # Entry point, CLI arg parsing, server startup
├── server.rs        # TcpListener, connection accept loop, thread spawning
├── http.rs          # HTTP/1.1 request parsing and response formatting
├── mime.rs          # File extension → Content-Type mapping
└── path.rs          # Path resolution, canonicalization, traversal prevention
```

**Structure Decision**: Flat module layout under `src/`. Each module has a single responsibility. No nested directories — the project is small enough that flat files are clearer. Tests live inline (`#[cfg(test)] mod tests`) per Rust convention.

## Complexity Tracking

No constitution violations. No complexity justifications needed.
