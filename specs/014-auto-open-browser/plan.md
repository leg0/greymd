# Implementation Plan: Auto-Open Browser on Startup

**Branch**: `014-auto-open-browser` | **Date**: 2026-03-09 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/014-auto-open-browser/spec.md`

## Summary

When greymd starts, it should automatically open the user's default browser to the serving URL (e.g., `http://127.0.0.1:8080`). A `--no-browser` CLI flag suppresses this. The browser launch is non-blocking and fails silently in headless environments. Implementation uses only `std::process::Command` with OS-specific commands — no new external dependencies.

## Technical Context

**Language/Version**: Rust, edition 2024 (Cargo.toml)  
**Primary Dependencies**: None (stdlib only; `latex2mathml` optional for math feature)  
**Storage**: N/A  
**Testing**: `cargo test` with inline `#[cfg(test)]` modules (~80+ existing tests)  
**Target Platform**: Windows, macOS, Linux (cross-platform CLI)  
**Project Type**: CLI tool / local HTTP server  
**Performance Goals**: Browser opens within 2 seconds; server startup not delayed  
**Constraints**: Zero new external crates (constitution: Minimal Dependencies); non-blocking launch  
**Scale/Scope**: Single-user local tool; one browser launch per invocation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | Tests will be written before implementation per TDD cycle. New `browser.rs` module will have `#[cfg(test)]` tests. CLI flag parsing tests added to `main.rs` tests. |
| II. Minimal Dependencies | ✅ PASS | Uses only `std::process::Command` for OS-specific browser commands (`xdg-open`, `open`, `cmd /c start`). Zero new crates. |
| III. Minimum Resource Usage | ✅ PASS | Browser launched via `Command::spawn()` (fire-and-forget, no wait). Single thread spawn for launch — no persistent resource usage. |

**Gate result**: PASS — no violations.

## Project Structure

### Documentation (this feature)

```text
specs/014-auto-open-browser/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── cli-contract.md
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── main.rs         # MODIFY: parse --no-browser flag, call browser::open() after bind
├── browser.rs      # NEW: cross-platform browser::open(url) function
├── server.rs       # NO CHANGE
├── http.rs         # NO CHANGE
├── markdown.rs     # NO CHANGE
├── listing.rs      # NO CHANGE
├── mime.rs         # NO CHANGE
├── path.rs         # NO CHANGE
└── assets/         # NO CHANGE
```

**Structure Decision**: Add one new module (`browser.rs`) following the existing flat module pattern. Modify `main.rs` for CLI flag and integration. No structural changes needed.

## Complexity Tracking

No constitution violations — this section is not applicable.
