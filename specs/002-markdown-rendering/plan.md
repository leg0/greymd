# Implementation Plan: Markdown-to-HTML Rendering

**Branch**: `002-markdown-rendering` | **Date**: 2026-02-20 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-markdown-rendering/spec.md`

## Summary

When a `.md` file is requested, convert its Markdown content to HTML on the fly and serve a complete HTML5 page. The Markdown parser is built from scratch with zero external dependencies, supporting a CommonMark subset: headings, paragraphs, bold/italic/combined, inline code, code blocks (with language class), links, images, lists, blockquotes, and horizontal rules. All HTML characters in content are escaped (no raw HTML passthrough).

## Technical Context

**Language/Version**: Rust 2024 (edition 2024, rustc 1.91.1)
**Primary Dependencies**: None (zero external crates)
**Storage**: Filesystem (read-only, `.md` files)
**Testing**: `cargo test` (unit + integration tests in-module)
**Target Platform**: Linux localhost (single binary)
**Project Type**: Single binary crate
**Performance Goals**: Render 100KB Markdown in under 100ms (SC-004)
**Constraints**: Zero external dependencies, minimal memory allocation, prefer borrowing
**Scale/Scope**: Local documentation server, single user, files typically < 100KB

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | TDD enforced: tests written before each parser component |
| II. Minimal Dependencies | ✅ PASS | Zero external crates; parser built with `std` only |
| III. Minimum Resource Usage | ✅ PASS | Single-pass parser, `String` output with pre-allocated capacity, no intermediate AST |

## Project Structure

### Documentation (this feature)

```text
specs/002-markdown-rendering/
├── plan.md              # This file
├── research.md          # Phase 0: architecture decisions
├── data-model.md        # Phase 1: entities
├── quickstart.md        # Phase 1: test scenarios
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Phase 2 output (speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── main.rs              # CLI entry point, module declarations
├── server.rs            # TCP listener, request handler (modify to detect .md)
├── http.rs              # HttpRequest/HttpResponse (no changes expected)
├── path.rs              # Path resolution (no changes expected)
├── mime.rs              # MIME type mapping (no changes expected)
└── markdown.rs          # NEW: Markdown-to-HTML parser
```

**Structure Decision**: Add a single new module `markdown.rs` containing the parser. Modify `server.rs` to detect `.md` extension and route through the parser before serving. Flat module structure continues from spec 1 — no subdirectories needed.
