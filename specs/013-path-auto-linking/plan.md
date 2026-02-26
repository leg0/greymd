# Implementation Plan: Path Auto-Linking

**Branch**: `013-path-auto-linking` | **Date**: 2026-02-28 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/013-path-auto-linking/spec.md`

## Summary

Add pattern-based auto-linking of relative `.md` file paths and directory paths in rendered markdown. Tokens ending with `.md` or `/` are automatically wrapped in `<a>` tags during inline rendering. Detection is purely syntactic — no filesystem access. Non-`.md` extensions (`.js`, `.rs`, `.toml`, etc.) are NOT linked.

## Technical Context

**Language/Version**: Rust 1.85+ (edition 2024)
**Primary Dependencies**: None (std only)
**Storage**: N/A
**Testing**: `cargo test`
**Target Platform**: Linux, Windows
**Project Type**: Single binary crate
**Performance Goals**: No measurable regression on markdown rendering
**Constraints**: Zero external dependencies, changes limited to `src/markdown.rs`
**Scale/Scope**: Single function modification + integration into existing `render_inline()`

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First | ✅ PASS | Tests written before implementation code |
| II. Minimal Dependencies | ✅ PASS | No new dependencies — pure string pattern matching with std |
| III. Minimum Resource Usage | ✅ PASS | Single-pass character scanning, no allocations beyond the output string already being built |

## Project Structure

### Documentation (this feature)

```text
specs/013-path-auto-linking/
├── plan.md              # This file
├── research.md          # Phase 0: design decisions
├── spec.md              # Feature specification
└── checklists/
    └── requirements.md  # Quality checklist
```

### Source Code (repository root)

```text
src/
├── markdown.rs          # MODIFIED: update try_parse_path() to enforce .md-only + integration in render_inline()
├── main.rs              # UNCHANGED
├── server.rs            # UNCHANGED
├── http.rs              # UNCHANGED
├── listing.rs           # UNCHANGED
└── ...
```

**Structure Decision**: This feature modifies a single file (`src/markdown.rs`). The existing `try_parse_path()` function is updated to enforce `.md`-only extension matching. The integration point in `render_inline()` remains the same: after URL auto-linking and before the final character escape fallback.
