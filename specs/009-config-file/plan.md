# Implementation Plan: Configuration & Customization

**Branch**: `009-config-file` | **Date**: 2026-02-23 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/009-config-file/spec.md`

## Summary

Add automatic port fallback (try default, pick random if busy) and support custom CSS/JS from well-known paths (`~/.config/greymd/css` and `~/.config/greymd/js`). No config file — zero configuration needed. Custom CSS served at `/?css2` (appended via extra `<link>` tag). Custom JS replaces built-in highlight.js at `/?js`.

## Technical Context

**Language/Version**: Rust edition 2024, rustc 1.85+
**Primary Dependencies**: None (zero runtime crate dependencies per constitution)
**Storage**: Filesystem — reads well-known custom CSS/JS files
**Testing**: `cargo test` (135 tests currently passing)
**Target Platform**: Linux x64, Windows x64
**Project Type**: Single binary crate
**Constraints**: Zero runtime dependencies, minimal memory/CPU usage
**Scale/Scope**: Local dev tool, single user

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | Port fallback, custom asset detection, asset serving all testable. |
| II. Minimal Dependencies | ✅ PASS | No new dependencies. Port binding uses `std::net::TcpListener`. File reading uses `std::fs::read`. Home dir uses `std::env::var`. |
| III. Minimum Resource Usage | ✅ PASS | Custom files read per-request but are small. Port binding is a one-time startup operation. No new allocations on hot path when no custom files exist. |

All gates pass.

## Project Structure

### Source Code (repository root)

```text
src/
├── main.rs          # CLI arg parsing (--port), port binding with fallback, calls server::start
├── server.rs        # Asset routing (/?css, /?css2, /?js), custom file detection
├── http.rs          # HttpRequest/HttpResponse types
├── markdown.rs      # wrap_html_page — conditionally adds /?css2 link
├── listing.rs       # render_listing — conditionally adds /?css2 link
├── mime.rs          # MIME type lookup
├── path.rs          # Path resolution and security
└── assets/          # CSS/JS source files for build.rs
```

**Structure Decision**: No new modules needed. Changes spread across main.rs and server.rs.

## Design

### Port Auto-Selection (`src/main.rs`)

```text
1. Try TcpListener::bind("127.0.0.1:8080")
2. If busy: TcpListener::bind("127.0.0.1:0") (OS picks random port)
3. Print actual bound port via listener.local_addr()
```

### Custom Assets (`src/server.rs`)

**Detection**: At startup, compute the config directory path:
- `std::env::var("HOME")` (Unix) or `std::env::var("USERPROFILE")` (Windows) + `/.config/greymd/`
- Store `Option<PathBuf>` for css_path and js_path based on whether the directory exists
- Actually, don't check existence at startup — just store the paths and let each request try to read

**Routing changes**:
- `/?css` → built-in gzipped CSS (unchanged)
- `/?css2` → read `~/.config/greymd/css`, serve as `text/css`. 404 if unreadable.
- `/?js` → if `~/.config/greymd/js` readable: serve its content as `application/javascript` (uncompressed). Otherwise: built-in gzipped highlight.js.

### HTML Changes (`src/markdown.rs`, `src/listing.rs`)

`wrap_html_page` and `render_listing` need to know whether custom CSS exists to add the `/?css2` link tag. Pass `has_custom_css: bool` parameter.

For JS: the `/?js` script tag stays the same — it just serves different content depending on whether the custom file exists.

### CLI Changes (`src/main.rs`)

- Change `server::start` signature to accept the `TcpListener` (already bound) instead of port number
- Pass `has_custom_css` and custom paths to server

## Complexity Tracking

No constitution violations — table not needed.
