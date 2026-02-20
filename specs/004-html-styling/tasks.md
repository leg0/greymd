# Tasks: HTML Styling

**Input**: Design documents from `/specs/004-html-styling/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story. Constitution requires TDD — tests are written before implementation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: No new files or dependencies needed. Verify baseline.

- [X] T001 Verify all 94 tests pass and clippy is clean before starting (`cargo test && cargo clippy`)

---

## Phase 2: Foundational (CSS Constant + wrap_html_page)

**Purpose**: Define the CSS constant and modify `wrap_html_page` to inject it. This is the single shared change that all user stories depend on.

**⚠️ CRITICAL**: All user stories inherit styling via `wrap_html_page` — this phase must complete first.

- [X] T002 Write test: `wrap_html_page` output contains `<style>` block in src/markdown.rs
- [X] T003 Write test: `wrap_html_page` output contains `<meta name="viewport"` in src/markdown.rs
- [X] T004 Write test: `wrap_html_page` output does NOT contain `<link` or `@import` in src/markdown.rs
- [X] T005 Update existing `test_wrap_html_page` and `test_wrap_html_page_empty_body` to account for new `<head>` content in src/markdown.rs
- [X] T006 Define `const CSS: &str` with the full stylesheet in src/markdown.rs (body, typography, headings, code, links, lists, blockquotes, hr, max-width, images)
- [X] T007 Modify `wrap_html_page` to inject viewport meta tag and `<style>{CSS}</style>` into `<head>` in src/markdown.rs
- [X] T008 Run `cargo test` — all tests pass including new and updated ones

**Checkpoint**: Foundation ready — every HTML page now has embedded CSS and viewport meta.

---

## Phase 3: User Story 1 — Readable Markdown Pages (Priority: P1) 🎯 MVP

**Goal**: Rendered markdown pages have styled headings, paragraphs, code blocks, links, lists, blockquotes, and horizontal rules.

**Independent Test**: Serve a `.md` file and verify styled output in browser. Programmatically: test that rendered HTML contains `<style>` with expected CSS rules.

### Tests for User Story 1

- [X] T009 [US1] Write test: rendered markdown page contains CSS rules for `h1`, `pre`, `code`, `a`, `blockquote` in src/markdown.rs
- [X] T010 [US1] Write test: rendered markdown page contains system font stack in CSS in src/markdown.rs

### Implementation for User Story 1

- [X] T011 [US1] Verify CSS constant includes rules for: h1–h6 sizes/margins (FR-003), p line-height (FR-004), pre/code backgrounds (FR-005/FR-006), link color (FR-007), ul/ol indentation (FR-008), blockquote border (FR-009), hr styling (FR-010) in src/markdown.rs
- [X] T012 [US1] Run `cargo test` — US1 tests pass

**Checkpoint**: Markdown pages are fully styled. Verify by serving a sample `.md` file.

---

## Phase 4: User Story 2 — Styled Directory Listings (Priority: P2)

**Goal**: Directory listing pages have styled entries with adequate spacing and consistent look.

**Independent Test**: Navigate to a directory and verify listing is styled with the same CSS.

### Tests for User Story 2

- [X] T013 [US2] Write test: directory listing HTML contains `<style>` block in src/server.rs (integration test)
- [X] T014 [US2] Write test: directory listing HTML contains viewport meta tag in src/server.rs (integration test)

### Implementation for User Story 2

- [X] T015 [US2] Verify listing pages inherit CSS via `wrap_html_page` — no code changes expected, just test confirmation in src/listing.rs
- [X] T016 [US2] Run `cargo test` — US2 tests pass

**Checkpoint**: Directory listings are styled. Listing module calls `wrap_html_page` so this should work automatically.

---

## Phase 5: User Story 3 — Consistent Look (Priority: P3) + User Story 4 — Responsive Width (Priority: P3)

**Goal**: Both page types share identical base styling. Content constrained to comfortable max-width.

**Independent Test**: Compare markdown page and listing page source — same `<style>` block. Verify `max-width` in CSS.

### Tests for User Stories 3 & 4

- [X] T017 [US3] Write test: markdown page and listing page contain identical `<style>` blocks in src/server.rs
- [X] T018 [US4] Write test: CSS contains `max-width` rule in src/markdown.rs

### Implementation for User Stories 3 & 4

- [X] T019 [US3] Verify both page types produce identical CSS (single `wrap_html_page` ensures this) — no code changes expected
- [X] T020 [US4] Verify CSS constant includes `max-width: 48em`, `margin: 0 auto`, `padding` on body in src/markdown.rs
- [X] T021 [US4] Run `cargo test` — US3+US4 tests pass

**Checkpoint**: All page types share consistent styling with responsive max-width.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Edge cases, linting, formatting, final validation.

- [X] T022 Verify CSS includes `overflow-x: auto` on `pre` for wide code blocks (edge case) in src/markdown.rs
- [X] T023 Verify CSS includes `img { max-width: 100% }` for image overflow prevention in src/markdown.rs
- [X] T024 Run `cargo clippy` — zero warnings
- [X] T025 Run `cargo fmt --check` — no formatting issues
- [X] T026 Run full `cargo test` — all tests pass (expected: ~100+)
- [X] T027 Verify zero external dependencies in Cargo.toml
- [X] T028 Run quickstart.md validation: build, serve a sample directory, verify in browser

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — verify baseline
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2
- **US2 (Phase 4)**: Depends on Phase 2 (independent of US1)
- **US3+US4 (Phase 5)**: Depends on Phase 2 (independent of US1/US2)
- **Polish (Phase 6)**: Depends on all user stories complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — no dependencies on other stories
- **US2 (P2)**: Can start after Phase 2 — independent of US1 (listing already uses `wrap_html_page`)
- **US3 (P3)**: Can start after Phase 2 — validates consistency between US1 and US2
- **US4 (P3)**: Can start after Phase 2 — validates max-width CSS rule

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Verify tests pass after implementation
- Story complete before moving to next priority

### Parallel Opportunities

- T009 and T010 can run in parallel (different test aspects, same file)
- T013 and T014 can run in parallel (different test aspects)
- T017 and T018 can run in parallel (different test aspects)
- US1, US2, US3, US4 can all proceed in parallel after Phase 2

---

## Implementation Strategy

### MVP First (Phase 2 + US1)

1. Complete Phase 1: Verify baseline
2. Complete Phase 2: CSS constant + `wrap_html_page` modification
3. Complete Phase 3: US1 — verify markdown pages are styled
4. **STOP and VALIDATE**: Test independently
5. All other stories are essentially free since they use `wrap_html_page`

### Key Insight

This feature has a single implementation point (`wrap_html_page` + CSS constant). Once Phase 2 is done, all user stories are effectively implemented. The remaining phases are primarily verification/testing.

---

## Notes

- Single modification point: `wrap_html_page` in src/markdown.rs
- CSS is a `const &str` — zero runtime allocation
- Both markdown and listing pages call `wrap_html_page` — automatic inheritance
- Existing tests checking `<body>` content won't break; tests checking `<head>` structure need updates
- Total: 28 tasks across 6 phases
