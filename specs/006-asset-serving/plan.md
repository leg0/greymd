# Implementation Plan: Asset Serving via Dedicated URI

**Branch**: `006-asset-serving` | **Date**: 2026-02-20 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/006-asset-serving/spec.md`

## Summary

Move the embedded CSS from an inline `<style>` block to a dedicated URI served under a compile-time GUID namespace (`/<guid>/style.css`). HTML pages emit a `<link>` tag instead. The server intercepts requests matching the GUID prefix before filesystem resolution, serving built-in assets with correct MIME types. The mechanism is extensible for future assets (e.g., syntax highlighting JS).

## Technical Context

**Language/Version**: Rust 2024 edition (rustc 1.91.1)
**Primary Dependencies**: Zero external crates (std only)
**Storage**: N/A (assets compiled into binary as `const &str`)
**Testing**: `cargo test` (binary crate, all tests inline)
**Target Platform**: Linux (localhost HTTP server)
**Project Type**: Single binary crate
**Performance Goals**: Asset served from memory, no filesystem I/O for built-in assets
**Constraints**: Zero dependencies, TDD mandatory, minimal allocations
**Scale/Scope**: Single-user local dev server

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | Tests written before implementation; existing tests adapted for `<link>` |
| II. Minimal Dependencies | ✅ PASS | No new dependencies; GUID is a hardcoded `const &str` |
| III. Minimum Resource Usage | ✅ PASS | CSS served from static memory, no copies; pages smaller without inline CSS |

## Project Structure

### Documentation (this feature)

```text
specs/006-asset-serving/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── checklists/
│   └── requirements.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── main.rs        # Entry point, module declarations
├── http.rs        # HTTP request/response types
├── server.rs      # Request routing — add GUID prefix check before resolve_path
├── markdown.rs    # wrap_html_page changes: <style> → <link>, CSS const stays here
├── listing.rs     # render_listing uses wrap_html_page (no direct changes needed)
├── mime.rs        # MIME type mapping (already has text/css)
└── path.rs        # Path resolution (unchanged)
```

**Structure Decision**: All changes within existing modules. New `const ASSET_PREFIX: &str` in server.rs. The `wrap_html_page` signature gains an `asset_prefix` parameter to generate the `<link>` tag. No new modules needed.

## Key Design Decisions

### 1. GUID Constant Location

The GUID is defined as `pub const ASSET_PREFIX: &str` in `server.rs` since that's where request routing happens. It's a 32-character lowercase hex string (UUID v4 format without dashes).

### 2. Request Interception Point

In `handle_connection`, check if `request.path` starts with `/<ASSET_PREFIX>/` **before** calling `resolve_path`. This ensures:
- No filesystem access for asset requests
- No path traversal risk (assets are a simple name lookup)
- No collision with user content

### 3. Asset Lookup

A simple `match` on the asset filename after stripping the prefix:
- `"style.css"` → return CSS with `text/css`
- anything else → 404

Extensible by adding more arms to the match.

### 4. wrap_html_page Signature Change

Current: `wrap_html_page(title, body)` — embeds `<style>{CSS}</style>`
New: `wrap_html_page(title, body, asset_prefix)` — emits `<link rel="stylesheet" href="/{asset_prefix}/style.css">`

Both `serve_file` and `serve_directory` (and `render_listing` via `serve_directory`) pass `ASSET_PREFIX` through.

### 5. Test Adaptation

Existing tests that assert `contains("<style>")` must be updated to assert `contains("<link")` and `contains("style.css")` instead. New tests:
- Request `/<guid>/style.css` → 200 with CSS content
- Request `/<guid>/unknown` → 404
- HTML pages contain `<link>` not `<style>`
