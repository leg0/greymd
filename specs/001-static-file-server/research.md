# Research: Static File Server

## Concurrency Model

**Decision**: Thread-per-connection using `std::thread::spawn`

**Rationale**: Simplest model that supports concurrent requests with zero external dependencies. Each accepted TCP connection is handled in a new OS thread. For a localhost-only tool with minimal concurrent users, thread overhead is negligible. Aligns with constitution principle III (minimum resource usage) — threads are cleaned up when the connection closes, no persistent pool overhead.

**Alternatives considered**:
- Single-threaded sequential: Simplest but blocks all clients while serving a large file. Rejected because even localhost use can have multiple browser tabs/requests in flight.
- Thread pool: More resource-efficient under high load but adds complexity (work queue, pool sizing). Rejected because localhost usage won't hit thread limits.
- Async (tokio/async-std): Would require external dependencies. Rejected per constitution principle II.

## HTTP/1.1 Parsing

**Decision**: Hand-written HTTP/1.1 request parser using `std::io::BufRead`

**Rationale**: HTTP/1.1 GET requests are simple enough to parse by hand. We only need to extract the method, path, and a few headers. No need for a full HTTP parser — we ignore request bodies entirely (GET-only server).

**Alternatives considered**:
- httparse crate: Well-tested, fast, zero-copy. Rejected per constitution principle II (zero dependencies).
- Full HTTP/1.1 compliance: Chunked transfer encoding, keep-alive, pipelining. Rejected — MVP only needs single request-response per connection.

## Path Security

**Decision**: Canonicalize resolved path and verify it starts with the served directory's canonical path

**Rationale**: Using `std::fs::canonicalize` on both the root directory and the resolved file path, then checking the prefix relationship, prevents all directory traversal attacks including those using symlinks. This is the standard defense.

**Alternatives considered**:
- String-based filtering (reject `..`): Fragile — doesn't handle encoded paths or symlinks. Rejected.
- chroot: Requires root privileges. Rejected.

## MIME Type Mapping

**Decision**: Hard-coded lookup table mapping file extensions to Content-Type strings

**Rationale**: A simple `match` on the file extension covers common types (html, css, js, json, png, jpg, gif, svg, txt, md, xml, pdf, wasm). Unknown extensions default to `application/octet-stream`. No need for content sniffing or a full MIME database.

**Alternatives considered**:
- mime_guess crate: Comprehensive but adds a dependency. Rejected per constitution principle II.
- Content sniffing: Complex, security-sensitive (MIME confusion attacks). Rejected.
