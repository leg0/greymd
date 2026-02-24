# Implementation Plan: Math Rendering

**Branch**: `011-math-rendering` | **Date**: 2026-02-24 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/011-math-rendering/spec.md`

## Summary

Add optional LaTeX math rendering behind a Cargo feature flag (`math`, off by default). When enabled, `$...$` becomes inline MathML and `$$...$$` (at line start) becomes block MathML via the `latex2mathml` crate. Invalid LaTeX falls back to raw text. Release archives publish both standard and math-enabled variants per platform. The `--help` output indicates whether math is compiled in.

## Technical Context

**Language/Version**: Rust edition 2024 (rustc 1.85+)
**Primary Dependencies**: None (std only); `latex2mathml` optional behind `math` feature; `miniz_oxide` build-time only
**Storage**: N/A (server-side conversion, no storage)
**Testing**: `cargo test` (both `--features math` and default)
**Target Platform**: Linux (primary), Windows (secondary)
**Project Type**: Single binary crate
**Constraints**: Zero runtime dependencies for default build (constitution principle II). `latex2mathml` is pure Rust with zero transitive deps — acceptable as optional.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | TDD for all math parsing and rendering code. Tests run under both `--features math` and default. |
| II. Minimal Dependencies | ✅ PASS | `latex2mathml` is optional, pure Rust, zero transitive deps. Default build has zero new deps. Feature flag justification: implementing a LaTeX parser from scratch would be ~3000 lines of code for a solved problem. |
| III. Minimum Resource Usage | ✅ PASS | Math conversion only runs when `$`/`$$` delimiters are found. No extra allocations on non-math pages. No additional assets to serve. |

## Project Structure

### Documentation (this feature)

```text
specs/011-math-rendering/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── checklists/
│   └── requirements.md  # Quality checklist
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── main.rs          # --help conditional math line (MODIFY)
├── markdown.rs      # render_inline: $...$ parsing; render_body: $$...$$ parsing (MODIFY)
├── server.rs        # No changes needed
├── listing.rs       # No changes needed
└── ...

Cargo.toml           # Add [features] section and optional latex2mathml dep (MODIFY)

.github/workflows/
└── release.yml      # Add math variant builds (MODIFY)

examples/
└── math-demo.md     # Demo file showcasing math rendering (NEW)
```

**Structure Decision**: Single binary crate. No new modules needed. Changes are confined to `markdown.rs` (parsing + conditional conversion), `main.rs` (help text), `Cargo.toml` (feature flag), and `release.yml` (dual variants).
