# Tasks: Asset Serving via Dedicated URI

**Input**: Design documents from `/specs/006-asset-serving/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Tests**: TDD mandatory per constitution. Tests written before implementation.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Define the GUID constant and make CSS publicly accessible

- [X] T001 Define `pub const ASSET_PREFIX: &str` as a 32-char lowercase hex string in `src/server.rs`
- [X] T002 Change `const CSS: &str` visibility to `pub const CSS: &str` in `src/markdown.rs`

---

## Phase 2: User Story 1 — CSS Served from Dedicated URI (Priority: P1) 🎯 MVP

**Goal**: HTML pages reference CSS via `<link>` tag; CSS is served from `/<guid>/style.css`

**Independent Test**: Request any markdown page → HTML contains `<link>` not `<style>`. Request `/<guid>/style.css` → returns CSS with `text/css`.

### Tests for User Story 1

> **Write these tests FIRST, ensure they FAIL before implementation**

- [X] T003 [US1] Write test in `src/markdown.rs`: `wrap_html_page` with asset prefix produces `<link rel="stylesheet" href="/<prefix>/style.css">` and no `<style>` block
- [X] T004 [US1] Write test in `src/server.rs`: requesting `/<ASSET_PREFIX>/style.css` returns 200 with `Content-Type: text/css` and CSS body
- [X] T005 [US1] Write test in `src/server.rs`: markdown page HTML contains `<link` and `style.css`, not `<style>`
- [X] T006 [US1] Write test in `src/server.rs`: directory listing HTML contains `<link` and `style.css`, not `<style>`

### Implementation for User Story 1

- [X] T007 [US1] Change `wrap_html_page` signature in `src/markdown.rs` to accept `asset_prefix: &str` parameter; replace `<style>{CSS}</style>` with `<link rel="stylesheet" href="/{asset_prefix}/style.css">`
- [X] T008 [US1] Update `serve_file` in `src/server.rs` to pass `ASSET_PREFIX` to `wrap_html_page`
- [X] T009 [US1] Update `serve_directory` in `src/server.rs` to pass `ASSET_PREFIX` to `render_listing` and through to `wrap_html_page`
- [X] T010 [US1] Update `render_listing` in `src/listing.rs` to accept `asset_prefix: &str` and pass it to `wrap_html_page`
- [X] T011 [US1] Add asset interception in `handle_connection` in `src/server.rs`: check if path starts with `/<ASSET_PREFIX>/` before `resolve_path`; serve `style.css` → CSS content with `text/css`, else → 404
- [X] T012 [US1] Update all existing tests in `src/markdown.rs` that assert `<style>` to assert `<link` instead
- [X] T013 [US1] Update all existing tests in `src/server.rs` that assert `<style>` to assert `<link` instead
- [X] T014 [US1] Run `cargo test` — all tests pass

**Checkpoint**: CSS served from dedicated URI, all pages use `<link>` tag

---

## Phase 3: User Story 2 — GUID Path Isolation (Priority: P1)

**Goal**: GUID namespace is fully isolated from filesystem; unknown assets return 404; path traversal blocked

**Independent Test**: Request `/<guid>/unknown.file` → 404. Request `/<guid>/../file` → 404. Normal file requests unaffected.

### Tests for User Story 2

- [X] T015 [US2] Write test in `src/server.rs`: requesting `/<ASSET_PREFIX>/unknown.file` returns 404
- [X] T016 [US2] Write test in `src/server.rs`: requesting `/<ASSET_PREFIX>/` (no filename) returns 404
- [X] T017 [US2] Write test in `src/server.rs`: normal file requests still resolve correctly when GUID prefix is in use

### Implementation for User Story 2

- [X] T018 [US2] Verify asset interception in `handle_connection` already handles unknown assets as 404 (from T011); add path traversal guard if not already covered
- [X] T019 [US2] Run `cargo test` — all tests pass

**Checkpoint**: GUID namespace fully isolated, no filesystem leakage

---

## Phase 4: User Story 3 — Future Asset Extensibility (Priority: P2)

**Goal**: Asset lookup mechanism supports adding new assets without architectural changes

**Independent Test**: Verify the asset match structure can accommodate additional entries.

### Tests for User Story 3

- [X] T020 [US3] Write test in `src/server.rs`: verify asset lookup is a match on filename after prefix strip (structural verification via test for known asset)

### Implementation for User Story 3

- [X] T021 [US3] Add code comment in `src/server.rs` asset match documenting how to add new assets
- [X] T022 [US3] Run `cargo test` — all tests pass

**Checkpoint**: Extensible asset mechanism documented and verified

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and cleanup

- [X] T023 Run `cargo clippy` — no warnings
- [X] T024 Run `cargo fmt --check` — formatting clean
- [X] T025 Update `README.md` to note that CSS is served from a dedicated URI
- [X] T026 Commit all changes on branch `006-asset-serving`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (US1)**: Depends on Phase 1 — core feature
- **Phase 3 (US2)**: Depends on Phase 2 — validates isolation of US1's implementation
- **Phase 4 (US3)**: Depends on Phase 2 — documents extensibility of US1's implementation
- **Phase 5 (Polish)**: Depends on all previous phases

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Signature changes before callers
- All tests pass at each checkpoint

### Parallel Opportunities

- T001 and T002 can run in parallel (different files)
- T003–T006 can run in parallel (all are test-writing tasks in different test blocks)
- T015–T017 can run in parallel (all are test-writing tasks)

---

## Implementation Strategy

### MVP First (User Story 1)

1. Phase 1: Define GUID const + make CSS public (2 tasks)
2. Phase 2: TDD cycle for CSS-from-URI (12 tasks)
3. **STOP and VALIDATE**: `cargo test` passes, pages use `<link>`, CSS served at URI

### Incremental Delivery

1. US1 → CSS served from URI (core value)
2. US2 → Isolation verified (safety)
3. US3 → Extensibility documented (future-proofing)
4. Polish → Clean commit

---

## Notes

- Total tasks: 26
- US1: 12 tasks (4 tests + 8 implementation)
- US2: 5 tasks (3 tests + 2 implementation)
- US3: 3 tasks (1 test + 2 implementation)
- Setup: 2 tasks
- Polish: 4 tasks
- Key files modified: `src/server.rs`, `src/markdown.rs`, `src/listing.rs`
- Key file unchanged: `src/path.rs`, `src/http.rs`, `src/mime.rs`, `src/main.rs`
