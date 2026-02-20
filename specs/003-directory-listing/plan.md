# Implementation Plan: Directory Listing

**Branch**: `003-directory-listing` | **Date**: 2026-02-20 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/003-directory-listing/spec.md`

## Summary

When a request path resolves to a directory, the server generates an HTML page listing `.md` files and subdirectories with clickable navigation links. Auto-serves a single `.md` file or `index.md` if present. Directories grouped before files, sorted alphabetically. Integrates with existing path security and Markdown rendering.

## Technical Context

**Language/Version**: Rust 2024 (edition 2024, rustc 1.91.1)
**Primary Dependencies**: None (zero external crates)
**Storage**: Filesystem (read-only directory listing via `std::fs::read_dir`)
**Testing**: `cargo test` (unit + integration tests in-module)
**Target Platform**: Linux localhost (single binary)
**Project Type**: Single binary crate
**Performance Goals**: List 1000 entries in under 200ms (SC-004)
**Constraints**: Zero external dependencies, minimal memory allocation
**Scale/Scope**: Local documentation server, directories typically < 1000 entries

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | TDD enforced: tests written before each component |
| II. Minimal Dependencies | ✅ PASS | Uses only `std::fs::read_dir` and existing modules |
| III. Minimum Resource Usage | ✅ PASS | Single pass over directory entries, sorted in-place, no caching |

## Project Structure

### Documentation (this feature)

```text
specs/003-directory-listing/
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
├── main.rs              # CLI entry point, module declarations (no changes)
├── server.rs            # Request handler (modify to detect directories + auto-serve logic)
├── http.rs              # HttpRequest/HttpResponse (no changes)
├── path.rs              # Path resolution (modify to support directory resolution)
├── mime.rs              # MIME type mapping (no changes)
├── markdown.rs          # Markdown renderer (no changes, used via auto-serve)
└── listing.rs           # NEW: Directory listing HTML generation
```

**Structure Decision**: Add a single new module `listing.rs` for directory listing generation. Modify `server.rs` to detect when a resolved path is a directory and route to the listing/auto-serve logic. Modify `path.rs` to allow directory resolution (currently only resolves files).
