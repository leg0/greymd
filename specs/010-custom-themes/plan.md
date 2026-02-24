# Implementation Plan: Custom Themes

**Branch**: `010-custom-themes` | **Date**: 2026-02-24 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/010-custom-themes/spec.md`

## Summary

Add `--theme <name>` flag to select bundled themes from `<prefix>/share/greymd/themes/` and `--list-themes` to discover them. Theme files (css/js) override `~/.config/greymd/` per-file. If the theme is not found, print a warning and fall back to default. Release archives (.tar.gz, .zip) include 6 bundled themes. `cargo install` does not include themes.

Most of the code already exists from the spec-009 implementation — `resolve_theme_dir`, `pick_asset_path`, `list_themes`, `--theme` parsing, and 3 `pick_asset_path` tests are already in `src/main.rs`. The main work is:
1. Change `--theme` from error+exit to warning+fallback when theme not found
2. Add tests for `resolve_theme_dir` and `list_themes`
3. Create a packaging script that builds release archives with the correct layout

## Technical Context

**Language/Version**: Rust edition 2024 (rustc 1.85+)
**Primary Dependencies**: None (std only); miniz_oxide for build-time gzip
**Storage**: Filesystem (`<prefix>/share/greymd/themes/`)
**Testing**: `cargo test`
**Target Platform**: Linux (primary), macOS/Windows (secondary)
**Project Type**: Single binary crate
**Constraints**: Zero runtime dependencies (constitution principle II)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | TDD for all new code; 3 existing `pick_asset_path` tests, need tests for warning behavior and `resolve_theme_dir` |
| II. Minimal Dependencies | ✅ PASS | No new dependencies. Theme resolution uses `std::env::current_exe()` and `std::fs` |
| III. Minimum Resource Usage | ✅ PASS | No new allocations on the hot path — theme dir resolved once at startup, files re-read per-request (existing behavior) |

## Project Structure

### Documentation (this feature)

```text
specs/010-custom-themes/
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
├── main.rs          # --theme/--list-themes CLI, pick_asset_path, resolve_theme_dir (MODIFY)
├── server.rs        # No changes needed — already accepts css_path/js_path
├── markdown.rs      # No changes needed
├── listing.rs       # No changes needed
└── ...

examples/themes/     # Source theme CSS files (already exist)
├── default/css
├── catppuccin-latte/css
├── catppuccin-frappe/css
├── catppuccin-macchiato/css
├── catppuccin-mocha/css
├── tokyo-night/css
└── README.md

scripts/
└── package.sh       # NEW: builds release archives with bin/ + share/ layout
```

**Structure Decision**: Single binary crate. No new modules needed. Changes are confined to `src/main.rs` (warning behavior) and a new `scripts/package.sh` for release packaging.
