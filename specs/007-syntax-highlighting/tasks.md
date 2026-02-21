# Tasks: Syntax Highlighting for Code Blocks

**Input**: Design documents from `/specs/007-syntax-highlighting/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Tests**: TDD mandatory per constitution. Tests written before implementation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)

---

## Phase 1: Setup

**Purpose**: Download highlight.js assets and create the assets directory

- [X] T001 Create `src/assets/` directory
- [X] T002 [P] Download highlight.js common bundle (minified) and save as `src/assets/highlight.min.js`
- [X] T003 [P] Download highlight.js GitHub theme CSS and save as `src/assets/highlight-github.css`

---

## Phase 2: User Story 1 — Syntax-Highlighted Code Blocks (Priority: P1) 🎯 MVP

**Goal**: Code blocks have syntax highlighting via embedded highlight.js

**Independent Test**: Request a page → HTML includes highlight.js `<script>` and `<link>` tags. Request asset URIs → 200 with correct content.

### Tests for User Story 1

> **Write these tests FIRST, ensure they FAIL before implementation**

- [X] T004 [US1] Write test in `src/markdown.rs`: `wrap_html_page` output contains `<link` with `highlight-github.css` and `<script` with `highlight.min.js` and inline `hljs.highlightAll()`
- [X] T005 [US1] Write test in `src/server.rs`: requesting `/<ASSET_PREFIX>/highlight.min.js` returns 200 with `Content-Type: application/javascript`
- [X] T006 [US1] Write test in `src/server.rs`: requesting `/<ASSET_PREFIX>/highlight-github.css` returns 200 with `Content-Type: text/css`
- [X] T007 [US1] Write test in `src/server.rs`: markdown page HTML contains `highlight.min.js` script tag

### Implementation for User Story 1

- [X] T008 [US1] Add `pub const HLJS_JS: &str = include_str!("assets/highlight.min.js");` in `src/markdown.rs`
- [X] T009 [US1] Add `pub const HLJS_CSS: &str = include_str!("assets/highlight-github.css");` in `src/markdown.rs`
- [X] T010 [US1] Update `wrap_html_page` in `src/markdown.rs`: add `<link>` for highlight CSS in `<head>`, add `<script src>` and `<script>hljs.highlightAll();</script>` before `</body>`
- [X] T011 [US1] Add match arms in `handle_connection` asset interception in `src/server.rs`: `"highlight.min.js"` → HLJS_JS with `application/javascript`, `"highlight-github.css"` → HLJS_CSS with `text/css`
- [X] T012 [US1] Run `cargo test` — all tests pass

**Checkpoint**: Syntax highlighting working — all pages include highlight.js, assets served from GUID URI

---

## Phase 3: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and cleanup

- [X] T013 Run `cargo clippy` — no warnings
- [X] T014 Run `cargo fmt --check` — formatting clean
- [X] T015 Commit all changes on branch `007-syntax-highlighting`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — download assets first
- **Phase 2 (US1)**: Depends on Phase 1 — needs asset files to exist for `include_str!`
- **Phase 3 (Polish)**: Depends on Phase 2

### Parallel Opportunities

- T002 and T003 can run in parallel (independent downloads)
- T004–T007 can run in parallel (independent test blocks)
- T008 and T009 can run in parallel (independent consts)

---

## Notes

- Total tasks: 15
- US1: 9 tasks (4 tests + 5 implementation)
- Setup: 3 tasks
- Polish: 3 tasks
- Key files modified: `src/markdown.rs`, `src/server.rs`
- Key files added: `src/assets/highlight.min.js`, `src/assets/highlight-github.css`
