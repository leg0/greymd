# Implementation Plan: HTML Styling

**Branch**: `004-html-styling` | **Date**: 2026-02-20 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/004-html-styling/spec.md`

## Summary

Add embedded CSS styling to all HTML pages served by docsvr. A compile-time CSS string constant is injected into the `<head>` of every page via the existing `wrap_html_page` function. Covers typography, code blocks, lists, blockquotes, links, horizontal rules, responsive max-width, and viewport meta tag. No new modules — changes are localized to `markdown.rs` (the `wrap_html_page` function).

## Technical Context

**Language/Version**: Rust 2024 edition (rustc 1.91.1)
**Primary Dependencies**: None (zero external crates)
**Storage**: N/A (purely presentational)
**Testing**: `cargo test` (binary crate, 94 tests currently passing)
**Target Platform**: Linux (local dev server)
**Project Type**: Single binary crate
**Performance Goals**: No measurable impact — CSS is a static string constant
**Constraints**: Zero external dependencies, no network requests for resources
**Scale/Scope**: Single `const` string + modify one function signature

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | Tests verify `<style>` presence, viewport meta tag, no `<link>` tags. Written before implementation. |
| II. Minimal Dependencies | ✅ PASS | CSS is a raw string constant. No crates added. |
| III. Minimum Resource Usage | ✅ PASS | Single static `&str` compiled into binary. No runtime allocation for CSS content. |

## Project Structure

### Documentation (this feature)

```text
specs/004-html-styling/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Entry point, module declarations (no changes)
├── http.rs              # HTTP request/response (no changes)
├── markdown.rs          # wrap_html_page modified to inject CSS + viewport meta
├── listing.rs           # Uses wrap_html_page (inherits styling automatically)
├── mime.rs              # MIME types (no changes)
├── path.rs              # Path resolution (no changes)
└── server.rs            # Request handler (no changes)
```

**Structure Decision**: No new files or modules. The CSS constant and `wrap_html_page` modification both live in `markdown.rs`. The listing module already calls `wrap_html_page`, so it inherits styling automatically.

## Design

### Approach

1. Define `const CSS: &str` in `markdown.rs` containing the full stylesheet.
2. Modify `wrap_html_page` to inject `<meta name="viewport" ...>` and `<style>{CSS}</style>` into `<head>`.
3. Both markdown rendering and directory listing pages automatically get styled since they both call `wrap_html_page`.

### CSS Rules (mapping to FRs)

| FR | CSS Target | Rule Summary |
|----|-----------|--------------|
| FR-003 | `h1`–`h6` | Distinct sizes, margin-top/bottom, border-bottom on h1/h2 |
| FR-004 | `p` | `line-height: 1.6`, margin spacing |
| FR-005 | `pre`, `pre code` | Background `#f6f8fa`, border, `overflow-x: auto`, monospace |
| FR-006 | `code` (inline) | Background `#f0f0f0`, padding, monospace |
| FR-007 | `a` | Color `#0366d6` |
| FR-008 | `ul`, `ol`, `li` | Padding-left, line-height |
| FR-009 | `blockquote` | Left border `#dfe2e5`, padding-left, color |
| FR-010 | `hr` | Border style, margin |
| FR-011 | `body` | `max-width: 48em`, `margin: 0 auto`, `padding: 1em 2em` |
| FR-012 | `body` | System font stack, `font-size: 16px`, `color: #24292e` |
| FR-013 | `li` | `padding: 0.25em 0` (applies to listing `<ul>` too) |
| FR-014 | `<meta>` | `viewport` tag in `<head>` |

### Impact on Existing Tests

Existing tests that check for exact HTML output (e.g., `test_wrap_html_page`, `test_wrap_html_page_empty_body`) will need updates to account for the new `<style>` block and viewport meta tag. The assertions check for `<body>` content which won't change, but tests checking exact `<head>` structure will need adjustment.

## Complexity Tracking

No constitution violations. No complexity justifications needed.
