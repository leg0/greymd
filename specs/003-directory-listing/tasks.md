# Tasks: Directory Listing

**Input**: Design documents from `/specs/003-directory-listing/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: TDD enforced per constitution (Principle I: Test-First, NON-NEGOTIABLE). Tests MUST be written and fail before implementation.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (US1, US2, US3, US4)
- Exact file paths included in descriptions

---

## Phase 1: Setup

**Purpose**: Create module structure and modify path resolution

- [ ] T001 Add `mod listing;` declaration to `src/main.rs`
- [ ] T002 Create empty `src/listing.rs` with module-level comment

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Modify path resolution to support directories, which ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Tests for Foundational

> **Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T003 [P] Test that `resolve_path` returns `Directory` variant for valid directory paths in `src/path.rs`
- [ ] T004 [P] Test that `resolve_path` returns `File` variant for valid file paths (existing behavior preserved) in `src/path.rs`
- [ ] T005 [P] Test that `resolve_path` rejects directory traversal for directories (same security as files) in `src/path.rs`

### Implementation for Foundational

- [ ] T006 Change `resolve_path` return type to `ResolvedPath` enum with `File(PathBuf)` and `Directory(PathBuf)` variants in `src/path.rs`
- [ ] T007 Update `handle_connection` in `src/server.rs` to match on `ResolvedPath` enum (file branch keeps existing behavior)

**Checkpoint**: Path resolution supports both files and directories, all existing tests still pass

---

## Phase 3: User Story 1 — Browse Directory Contents (Priority: P1) 🎯 MVP

**Goal**: When a directory is requested, return an HTML page listing `.md` files and subdirectories as clickable links

**Independent Test**: Request a directory path, verify HTML listing with clickable entries

### Tests for User Story 1

> **Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T008 [P] [US1] Test `render_listing` generates valid HTML5 page with entries as links in `src/listing.rs`
- [ ] T009 [P] [US1] Test `render_listing` only includes `.md` files and subdirectories (excludes `.txt`, `.html`, etc.) in `src/listing.rs`
- [ ] T010 [P] [US1] Test `render_listing` escapes special characters in filenames in `src/listing.rs`
- [ ] T011 [P] [US1] Test `render_listing` shows trailing `/` for directory entries in `src/listing.rs`
- [ ] T012 [US1] Test integration: requesting a directory path returns HTML listing via server in `src/server.rs`

### Implementation for User Story 1

- [ ] T013 [US1] Implement `collect_entries(dir: &Path) -> Vec<DirectoryEntry>` to read directory, filter to `.md` files and subdirs in `src/listing.rs`
- [ ] T014 [US1] Implement `render_listing(path: &str, entries: &[DirectoryEntry], show_parent: bool) -> String` to generate HTML page in `src/listing.rs`
- [ ] T015 [US1] Add directory branch to `handle_connection` in `src/server.rs`: call `collect_entries` + `render_listing`, serve as `text/html`

**Checkpoint**: Requesting any directory path returns a browsable HTML listing of `.md` files and subdirectories

---

## Phase 4: User Story 2 — Navigate to Parent Directory (Priority: P1)

**Goal**: Subdirectory listings include a `..` link; root listing does not

**Independent Test**: Request subdirectory listing — verify `..` link present. Request root — verify no `..` link.

### Tests for User Story 2

- [ ] T016 [P] [US2] Test `render_listing` includes `..` link when `show_parent` is true in `src/listing.rs`
- [ ] T017 [P] [US2] Test `render_listing` omits `..` link when `show_parent` is false in `src/listing.rs`
- [ ] T018 [US2] Test integration: subdirectory listing has parent link, root listing does not in `src/server.rs`

### Implementation for User Story 2

- [ ] T019 [US2] Pass `show_parent` flag from `handle_connection` based on whether request path is root (`/`) in `src/server.rs`

**Checkpoint**: Users can navigate up via `..` links from any subdirectory; root has no `..`

---

## Phase 5: User Story 3 — Root URL + Auto-Serve (Priority: P1)

**Goal**: Root URL shows listing; auto-serve single `.md` or `index.md` when applicable

**Independent Test**: Navigate to `/` — verify listing. Test auto-serve with single `.md` file and with `index.md`.

### Tests for User Story 3

> **Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T020 [P] [US3] Test integration: requesting `/` returns directory listing of root in `src/server.rs`
- [ ] T021 [P] [US3] Test auto-serve: directory with single `.md` file serves that file as HTML in `src/server.rs`
- [ ] T022 [P] [US3] Test auto-serve: directory with multiple `.md` files including `index.md` serves `index.md` as HTML in `src/server.rs`
- [ ] T023 [P] [US3] Test fallback: directory with multiple `.md` files and no `index.md` shows listing in `src/server.rs`
- [ ] T024 [P] [US3] Test that `index.html` is NOT auto-served (listing shown instead) in `src/server.rs`

### Implementation for User Story 3

- [ ] T025 [US3] Implement auto-serve logic in `handle_connection`: check `.md` file count, check for `index.md`, fall back to listing in `src/server.rs`
- [ ] T026 [US3] Ensure root path `/` resolves to served root directory in `src/path.rs` or `src/server.rs`

**Checkpoint**: Root URL works, auto-serve kicks in for single `.md` or `index.md`, fallback to listing otherwise

---

## Phase 6: User Story 4 — Sorted and Organized Listing (Priority: P2)

**Goal**: Directories grouped before files, alphabetically sorted case-insensitive

**Independent Test**: Request directory with mixed entries, verify sorted output

### Tests for User Story 4

- [ ] T027 [P] [US4] Test `collect_entries` returns directories before files in `src/listing.rs`
- [ ] T028 [P] [US4] Test `collect_entries` sorts entries alphabetically case-insensitive within each group in `src/listing.rs`

### Implementation for User Story 4

- [ ] T029 [US4] Implement sorting in `collect_entries`: partition dirs/files, sort each group by lowercased name in `src/listing.rs`

**Checkpoint**: Directory listings are consistently sorted and grouped

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Edge cases, regression, code quality

- [ ] T030 [P] Test empty directory returns valid HTML page with no entries in `src/listing.rs`
- [ ] T031 [P] Test directory path without trailing slash still returns listing in `src/server.rs`
- [ ] T032 Test that non-`.md` files are still served when requested directly (no regression) in `src/server.rs`
- [ ] T033 Run `cargo clippy` and fix any warnings
- [ ] T034 Run `cargo fmt` to ensure consistent formatting
- [ ] T035 Verify zero external dependencies in `Cargo.toml`
- [ ] T036 Run full `cargo test` suite — all tests pass with zero warnings

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2 — MVP delivery
- **US2 (Phase 4)**: Depends on Phase 3 (extends listing render)
- **US3 (Phase 5)**: Depends on Phase 3 (extends server handler with auto-serve)
- **US4 (Phase 6)**: Depends on Phase 3 (extends collect_entries)
- **Polish (Phase 7)**: Depends on all user stories complete

### Within Each User Story

- Tests MUST be written and FAIL before implementation (Constitution Principle I)
- Unit tests before integration tests
- Core functions before server integration

### Parallel Opportunities

- T003, T004, T005 can all run in parallel (independent test functions)
- T008–T011 can all run in parallel (independent listing tests)
- T016, T017 can run in parallel
- T020–T024 can all run in parallel (independent integration tests)
- T027, T028 can run in parallel
- US2 and US4 can proceed in parallel after US1 is complete

---

## Implementation Strategy

### MVP First (User Stories 1–3)

1. Complete Phase 1: Setup (T001–T002)
2. Complete Phase 2: Foundational — path resolution (T003–T007)
3. Complete Phase 3: US1 — core listing (T008–T015)
4. Complete Phase 4: US2 — parent navigation (T016–T019)
5. Complete Phase 5: US3 — root URL + auto-serve (T020–T026)
6. **STOP and VALIDATE**: Browse real directories in browser

### Incremental Delivery

1. Setup + Foundational → path resolution supports directories
2. Add US1 → directory browsing works → MVP!
3. Add US2 → parent navigation → usable
4. Add US3 → root URL + auto-serve → complete navigation
5. Add US4 → sorted listings → polished
6. Polish → edge cases, clippy, fmt → ship it

---

## Notes

- Primary new file: `src/listing.rs` (directory listing HTML generation)
- Modified files: `src/path.rs` (ResolvedPath enum), `src/server.rs` (directory handler + auto-serve)
- `src/main.rs` only gets `mod listing;` added
- Reuses `markdown::escape_html` and `markdown::wrap_html_page` from spec 2
- Constitution enforces: test first, zero deps, minimal allocation
