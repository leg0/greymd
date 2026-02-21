# Implementation Plan: Syntax Highlighting for Code Blocks

**Branch**: `007-syntax-highlighting` | **Date**: 2026-02-21 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/007-syntax-highlighting/spec.md`

## Summary

Add client-side syntax highlighting to fenced code blocks by embedding the highlight.js common bundle (~40KB) and GitHub theme CSS into the binary. Serve them as built-in assets from the existing GUID namespace. Add `<link>`, `<script>`, and inline `hljs.highlightAll()` to all HTML pages.

## Technical Context

**Language/Version**: Rust 2024 edition (rustc 1.91.1)
**Primary Dependencies**: Zero Rust crate deps; highlight.js embedded as static content
**Storage**: N/A (assets compiled into binary as `const &str`)
**Testing**: `cargo test` (binary crate, all tests inline)
**Target Platform**: Linux (localhost HTTP server)
**Project Type**: Single binary crate
**Performance Goals**: Assets served from memory, client-side highlighting
**Constraints**: Zero Rust deps, TDD mandatory, highlight.js files embedded verbatim
**Scale/Scope**: Single-user local dev server

## Constitution Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | Tests for asset serving and HTML tags written before implementation |
| II. Minimal Dependencies | ✅ PASS | No Rust crate deps added; highlight.js is embedded static content, not a build dependency |
| III. Minimum Resource Usage | ✅ PASS | ~45KB added to binary (JS + CSS), served from memory with no copies |

## Project Structure

### Source Code (repository root)

```text
src/
├── main.rs        # Unchanged
├── http.rs        # Unchanged
├── server.rs      # Add 2 asset match arms (highlight.min.js, highlight-github.css)
├── markdown.rs    # Add HLJS_JS + HLJS_CSS consts, update wrap_html_page to emit <script>/<link>
├── listing.rs     # Unchanged (inherits via wrap_html_page)
├── mime.rs        # Unchanged (already maps .js and .css)
└── path.rs        # Unchanged
```

**Structure Decision**: Minimal changes — 2 new consts in markdown.rs, 2 new match arms in server.rs, updated HTML template in wrap_html_page.

## Key Design Decisions

### 1. Asset Storage

The highlight.js JS and CSS are stored as `pub const HLJS_JS: &str` and `pub const HLJS_CSS: &str` in `markdown.rs` alongside the existing `CSS` const. The content is the minified files from the highlight.js CDN, embedded via `include_str!` or as raw string literals.

### 2. Asset Serving

Two new match arms in the `handle_connection` asset interception (server.rs):
- `"highlight.min.js"` → `HLJS_JS` with `application/javascript`
- `"highlight-github.css"` → `HLJS_CSS` with `text/css`

### 3. HTML Template Changes

`wrap_html_page` adds to `<head>`:
- `<link rel="stylesheet" href="/<guid>/highlight-github.css">`

And before `</body>`:
- `<script src="/<guid>/highlight.min.js"></script>`
- `<script>hljs.highlightAll();</script>`

Script at end of body ensures DOM is ready before highlighting runs.

### 4. File Embedding Approach

Download the files from the highlight.js CDN, save them under `src/assets/`, and use `include_str!` to embed them at compile time. This avoids massive string literals in source files.

### 5. Test Strategy

- Test that `wrap_html_page` output contains the new `<script>` and `<link>` tags
- Test that `/<guid>/highlight.min.js` returns 200 with JS content type
- Test that `/<guid>/highlight-github.css` returns 200 with CSS content type
- Update any existing tests that count tags or check for specific HTML structure
