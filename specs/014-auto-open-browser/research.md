# Research: Auto-Open Browser on Startup

**Feature**: 014-auto-open-browser  
**Date**: 2026-03-09

## R-001: Cross-Platform Browser Opening via Standard Library

**Decision**: Use `std::process::Command` to invoke OS-native URL-opening commands.

**Rationale**: The project constitution mandates zero external crate dependencies. Every major OS provides a built-in command to open a URL in the default browser. Using `std::process::Command` keeps the implementation entirely within the Rust standard library.

**Approach by platform** (detected via `cfg!(target_os = ...)`):

| Platform | Command | Arguments |
|----------|---------|-----------|
| Linux / FreeBSD | `xdg-open` | `<url>` |
| macOS | `open` | `<url>` |
| Windows | `cmd` | `/c`, `start`, `""`, `<url>` |

**Windows note**: The empty `""` argument after `start` is critical — `start` interprets its first quoted argument as a window title. Without it, a URL containing characters like `:` or `/` can cause `start` to misinterpret the URL.

**Alternatives considered**:
- `open` crate (v5.x): Well-tested, handles edge cases. **Rejected** — adds an external dependency, violating constitution principle II.
- `webbrowser` crate: Feature-rich with fallback chains. **Rejected** — same dependency concern.
- `explorer.exe` on Windows: Works but doesn't respect default browser if user has changed it from Edge. `cmd /c start` is more reliable.

## R-002: Non-Blocking Browser Launch

**Decision**: Use `Command::spawn()` (not `Command::status()` or `Command::output()`).

**Rationale**: `spawn()` returns immediately after launching the child process. The browser process runs independently. `status()` and `output()` would block until the browser process exits (which may never happen), violating FR-007 (must not block server).

**Approach**:
- Call `Command::new(...).args(...).spawn()` — returns `Result<Child>`
- Discard the `Child` handle (drop it) — the child process continues running independently
- On spawn failure, silently ignore the error (FR-006)
- No need to redirect stdin/stdout/stderr — default inheritance is acceptable since the browser process detaches from the terminal on its own

**Alternatives considered**:
- Spawning a separate Rust thread for the launch: Unnecessary since `spawn()` is already non-blocking. **Rejected** — adds complexity without benefit.
- Using `Command::status()` in a thread: Would work but holds a thread alive until the browser exits. **Rejected** — wasteful per constitution principle III.

## R-003: CLI Flag Parsing Pattern

**Decision**: Add `--no-browser` flag using the existing manual arg-parsing pattern in `main.rs`.

**Rationale**: The codebase uses `env::args().collect::<Vec<String>>()` with `iter().any()` for boolean flags and `.windows(2)` for key-value pairs. Adding `--no-browser` as a boolean flag is consistent with `--version` and `--help`.

**Approach**:
- Check: `args.iter().any(|a| a == "--no-browser")`
- Filter `--no-browser` from the args list before positional argument extraction (same as existing flag handling)
- Update `print_usage()` to document the new flag

**Alternatives considered**:
- Introducing `clap` or `structopt` for proper CLI parsing: Would improve long-term maintainability. **Rejected** — violates constitution principle II (minimal dependencies) and is disproportionate for one flag addition.
- Using `-B` as short flag: Common short flags for browser-related options vary. **Rejected** — no established convention; `--no-browser` is self-documenting.

## R-004: Integration Point in Startup Flow

**Decision**: Open the browser after the listener binds but before entering the accept loop.

**Rationale**: FR-003 requires the browser to open after the server is ready to accept connections. The `TcpListener::bind()` call completes before `server::start()` enters the accept loop. Opening the browser between these two calls ensures:
1. The port is known (correct URL)
2. The server will accept the browser's connection (listener is bound)
3. No race condition — the listener queue buffers the connection until `accept()` is called

**Approach** (in `main.rs`):
```
let listener = bind_listener();
let addr = listener.local_addr().unwrap();
println!("Listening on http://{}", addr);
if !no_browser {
    browser::open(format!("http://{}", addr));
}
server::start(listener, &root, css_path, js_path);
```

**Alternatives considered**:
- Opening the browser from within `server::start()` after the first `accept()`: Would guarantee the server is actively accepting, but adds complexity and delays the browser launch. **Rejected** — bind is sufficient; the OS listener queue handles the first connection.
