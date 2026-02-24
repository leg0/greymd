# Tasks: Custom Themes

**Input**: Design documents from `/specs/010-custom-themes/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Tests**: TDD is mandatory per constitution. Each task follows red-green-refactor.

**Organization**: Tasks grouped by user story. Most code already exists — this feature is primarily about fixing behavior, adding tests, and creating packaging.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

---

## Phase 1: Foundational

**Purpose**: Fix existing `--theme` behavior from error+exit to warning+fallback (FR-005).

- [x] T001 Write test: `--theme` with nonexistent theme name warns to stderr and falls back to default (no theme applied, server still starts). Change `resolve_theme_dir` return handling in `src/main.rs` from `process::exit(1)` to `eprintln!` warning. Make test pass.
- [x] T002 Write test: `resolve_theme_dir` returns `Some(path)` when a theme directory exists at `<prefix>/share/greymd/themes/<name>/`. Create a temp directory structure simulating the install layout, verify resolution. Implement in `src/main.rs`.
- [x] T003 Write test: `resolve_theme_dir` returns `None` when theme name doesn't exist. Verify existing implementation passes in `src/main.rs`.

**Checkpoint**: `--theme` warns and falls back on missing theme. `resolve_theme_dir` fully tested. All tests pass.

---

## Phase 2: User Story 1 — Apply a Theme by Name (Priority: P1) 🎯 MVP

**Goal**: `greymd --theme <name>` selects CSS/JS from `<prefix>/share/greymd/themes/<name>/`, overriding `~/.config/greymd/` per-file.

**Independent Test**: Start server with `--theme` pointing at a temp theme dir, request `/?css2`, verify theme CSS is served.

- [x] T004 [US1] Write integration test: start server with `--theme` pointing at a temp theme dir containing a `css` file. Request a markdown page and verify HTML includes `/?css2` link. Request `/?css2` and verify theme CSS content is returned. Make test pass in `src/main.rs` and `src/server.rs`.
- [x] T005 [US1] Write integration test: start server with `--theme` pointing at a temp theme dir containing a `js` file. Request `/?js` and verify custom JS content is returned (not gzipped built-in). Make test pass in `src/main.rs`.
- [x] T006 [US1] Write test: `--theme` with only `css` in theme dir and `js` in `~/.config/greymd/` — theme CSS takes priority, config JS still used. Verify `pick_asset_path` logic in `src/main.rs`.
- [x] T007 [US1] Write test: `--theme` dir exists but is empty (no css or js) — server starts with no custom overrides. Verify in `src/main.rs`.

**Checkpoint**: `--theme <name>` fully works — serves theme CSS at `/?css2`, theme JS at `/?js`, with per-file fallback to `~/.config/greymd/`. All tests pass.

---

## Phase 3: User Story 2 — List Available Themes (Priority: P2)

**Goal**: `greymd --list-themes` prints all installed theme names from `<prefix>/share/greymd/themes/`.

**Independent Test**: Create temp theme dirs, run `list_themes`, verify output contains all theme names.

- [x] T008 [US2] Write test: `list_themes` with themes present in a temp themes directory — output contains sorted theme names. Refactor `list_themes` in `src/main.rs` to accept a themes dir path (for testability). Make test pass.
- [x] T009 [US2] Write test: `list_themes` with no themes directory — output shows helpful message. Verify in `src/main.rs`.

**Checkpoint**: `--list-themes` lists installed themes or shows helpful message. All tests pass.

---

## Phase 4: Polish & Cross-Cutting Concerns

- [x] T010 Create `scripts/package.sh` that builds a release archive with layout: `greymd-<version>-<target>/bin/greymd` + `greymd-<version>-<target>/share/greymd/themes/` containing all 6 themes from `examples/themes/`. Output both `.tar.gz` and `.zip`.
- [x] T011 Update `print_usage()` in `src/main.rs` to document `--theme` warning-and-fallback behavior.
- [x] T012 Update `README.md` with themes section documenting `--theme <name>`, `--list-themes`, and the release archive layout.

**Checkpoint**: Packaging script produces correct archives. README and `--help` are up to date. All tests pass.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Foundational (Phase 1)**: No dependencies — can start immediately
- **US1 (Phase 2)**: Depends on Phase 1 (needs warning+fallback behavior)
- **US2 (Phase 3)**: Depends on Phase 1 (needs `resolve_theme_dir` tested)
- **Polish (Phase 4)**: Depends on US1 and US2 being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 1
- **US2 (P2)**: Can start after Phase 1 (independent of US1)

### TDD Cycle Per Task

Each task follows strict red-green-refactor:
1. **Red**: Write failing test
2. **Green**: Minimal code to pass
3. **Refactor**: Clean up
4. **Commit**: `cargo test` passes, commit

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Fix warning+fallback behavior
2. Complete Phase 2: `--theme <name>` integration tests
3. **STOP and VALIDATE**: Theme selection works end-to-end

### Incremental Delivery

1. Phase 1 → Warning behavior fixed
2. Phase 2 (US1) → Themes selectable by name (MVP!)
3. Phase 3 (US2) → Theme discovery via `--list-themes`
4. Phase 4 → Packaging and docs

---

## Notes

- Most code already exists in `src/main.rs` from spec-009 implementation
- Key change: lines 69-73 in `src/main.rs` — error+exit → warning+continue
- `pick_asset_path` already has 3 tests — T006/T007 cover additional edge cases
- `resolve_theme_dir` is tricky to test since it uses `current_exe()` — tests should create a temp dir structure and call the function's logic directly or refactor for testability
- `list_themes` currently prints directly — may need refactoring to return data for testability
