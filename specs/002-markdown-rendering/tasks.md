# Tasks: Markdown-to-HTML Rendering

**Input**: Design documents from `/specs/002-markdown-rendering/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: TDD enforced per constitution (Principle I: Test-First, NON-NEGOTIABLE). Tests MUST be written and fail before implementation.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (US1, US2, US3, US4)
- Exact file paths included in descriptions

---

## Phase 1: Setup

**Purpose**: Create module structure for the Markdown parser

- [X] T001 Add `mod markdown;` declaration to `src/main.rs`
- [X] T002 Create empty `src/markdown.rs` with module-level doc comment

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core utilities needed by ALL user stories before any Markdown element parsing

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Tests for Foundational

> **Write these tests FIRST, ensure they FAIL before implementation**

- [X] T003 [P] Test HTML escaping of `<`, `>`, `&`, `"` in text content in `src/markdown.rs`
- [X] T004 [P] Test HTML5 page wrapper generation (doctype, html, head, body) in `src/markdown.rs`
- [X] T005 [P] Test title extraction from first `#` heading, with filename fallback in `src/markdown.rs`

### Implementation for Foundational

- [X] T006 [P] Implement `escape_html(text: &str) -> String` in `src/markdown.rs`
- [X] T007 [P] Implement `wrap_html_page(title: &str, body: &str) -> String` in `src/markdown.rs`
- [X] T008 Implement `extract_title(source: &str, filename: &str) -> String` in `src/markdown.rs`

**Checkpoint**: Foundation ready — HTML escaping, page wrapper, and title extraction all tested and passing

---

## Phase 3: User Story 1 — Render Markdown as HTML (Priority: P1) 🎯 MVP

**Goal**: When a `.md` file is requested, serve a complete HTML page with Markdown rendered as HTML elements

**Independent Test**: Start docsvr, request a `.md` file, verify response is `text/html` with headings, paragraphs, inline formatting, links, and code blocks rendered

### Tests for User Story 1

> **Write these tests FIRST, ensure they FAIL before implementation**

- [X] T009 [P] [US1] Test heading rendering (h1–h6) from `# ` through `###### ` prefixes in `src/markdown.rs`
- [X] T010 [P] [US1] Test paragraph rendering: text separated by blank lines becomes `<p>` elements in `src/markdown.rs`
- [X] T011 [P] [US1] Test inline formatting: `**bold**` → `<strong>`, `*italic*` → `<em>`, `***both***` → `<strong><em>`, `` `code` `` → `<code>` in `src/markdown.rs`
- [X] T012 [P] [US1] Test link rendering: `[text](url)` → `<a href="url">text</a>` in `src/markdown.rs`
- [X] T013 [P] [US1] Test fenced code block rendering: triple backtick blocks → `<pre><code>` with language class attribute in `src/markdown.rs`
- [X] T014 [P] [US1] Test indented code block rendering: 4-space indented lines → `<pre><code>` in `src/markdown.rs`
- [X] T015 [US1] Test `render(source: &str, filename: &str) -> String` produces complete HTML page in `src/markdown.rs`

### Implementation for User Story 1

- [X] T016 [P] [US1] Implement heading parser: detect `# ` through `###### ` prefixes, emit `<h1>`–`<h6>` in `src/markdown.rs`
- [X] T017 [US1] Implement inline formatting parser with delimiter stack: bold, italic, combined bold+italic, inline code in `src/markdown.rs`
- [X] T018 [US1] Implement link parser: detect `[text](url)` pattern within inline content in `src/markdown.rs`
- [X] T019 [P] [US1] Implement fenced code block parser: detect opening/closing ` ``` `, capture language, emit `<pre><code class="language-X">` in `src/markdown.rs`
- [X] T020 [P] [US1] Implement indented code block parser: detect 4-space indent, emit `<pre><code>` in `src/markdown.rs`
- [X] T021 [US1] Implement paragraph parser: accumulate text lines, emit `<p>` on blank line or block transition in `src/markdown.rs`
- [X] T022 [US1] Implement top-level `render(source: &str, filename: &str) -> String` that orchestrates block parsing and wraps in HTML page in `src/markdown.rs`
- [X] T023 [US1] Modify `handle_connection` in `src/server.rs` to detect `.md` extension, read as string, call `markdown::render()`, serve as `text/html`

**Checkpoint**: Requesting any `.md` file returns a rendered HTML page with headings, paragraphs, formatting, links, and code blocks

---

## Phase 4: User Story 2 — Non-Markdown Files Unchanged (Priority: P1)

**Goal**: Non-`.md` files continue to be served as raw content exactly as before

**Independent Test**: Request `.html`, `.txt`, `.css`, image files — verify raw content served with original Content-Type

### Tests for User Story 2

- [X] T024 [US2] Test that `.html` files are served as raw content (not double-wrapped in HTML) via integration test in `src/server.rs`
- [X] T025 [P] [US2] Test that `.txt` and `.css` files are served unchanged via integration test in `src/server.rs`

### Implementation for User Story 2

- [X] T026 [US2] Verify `handle_connection` in `src/server.rs` only routes `.md` files through renderer; no changes needed if T023 correctly branches on extension

**Checkpoint**: Non-Markdown files served identically to spec 1 behavior — no regressions

---

## Phase 5: User Story 3 — Lists and Block Elements (Priority: P2)

**Goal**: Unordered lists, ordered lists, nested lists, blockquotes, and horizontal rules render correctly

**Independent Test**: Request a `.md` file with lists and blockquotes, verify correct `<ul>`, `<ol>`, `<li>`, `<blockquote>`, `<hr>` elements

### Tests for User Story 3

> **Write these tests FIRST, ensure they FAIL before implementation**

- [X] T027 [P] [US3] Test unordered list rendering: `- item` → `<ul><li>item</li></ul>` in `src/markdown.rs`
- [X] T028 [P] [US3] Test ordered list rendering: `1. item` → `<ol><li>item</li></ol>` in `src/markdown.rs`
- [X] T029 [P] [US3] Test nested list rendering (2–3 levels deep, mixed ordered/unordered) in `src/markdown.rs`
- [X] T030 [P] [US3] Test blockquote rendering: `> text` → `<blockquote><p>text</p></blockquote>` in `src/markdown.rs`
- [X] T031 [P] [US3] Test horizontal rule rendering: `---`, `***`, `___` → `<hr>` in `src/markdown.rs`

### Implementation for User Story 3

- [X] T032 [US3] Implement unordered list parser: detect `-`/`*`/`+` prefixes, manage `<ul><li>` open/close in `src/markdown.rs`
- [X] T033 [US3] Implement ordered list parser: detect `N.` prefix, manage `<ol><li>` open/close in `src/markdown.rs`
- [X] T034 [US3] Implement list nesting: track indent level, maintain list type stack, emit nested `<ul>`/`<ol>` in `src/markdown.rs`
- [X] T035 [US3] Implement blockquote parser: detect `> ` prefix, wrap content in `<blockquote><p>` in `src/markdown.rs`
- [X] T036 [P] [US3] Implement horizontal rule parser: detect `---`/`***`/`___` (3+ chars, standalone line) → `<hr>` in `src/markdown.rs`

**Checkpoint**: Lists, blockquotes, and horizontal rules all render correctly alongside existing elements

---

## Phase 6: User Story 4 — Images (Priority: P2)

**Goal**: Inline images render as `<img>` elements with correct src and alt attributes

**Independent Test**: Request a `.md` file with `![alt](url)` syntax, verify `<img>` elements in output

### Tests for User Story 4

- [X] T037 [US4] Test image rendering: `![alt text](image.png)` → `<img src="image.png" alt="alt text">` in `src/markdown.rs`
- [X] T038 [P] [US4] Test image with HTML characters in alt text are escaped in `src/markdown.rs`

### Implementation for User Story 4

- [X] T039 [US4] Implement image parser: detect `![alt](url)` pattern within inline content, emit `<img src="url" alt="alt">` in `src/markdown.rs`

**Checkpoint**: Images render correctly alongside all other Markdown elements

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Edge cases, code quality, final validation

- [X] T040 [P] Test and verify empty `.md` file returns valid HTML page with empty body in `src/markdown.rs`
- [X] T041 [P] Test and verify CRLF line endings produce identical output to LF in `src/markdown.rs`
- [X] T042 Test HTML escaping in all contexts: headings, list items, blockquotes, link text, image alt text in `src/markdown.rs`
- [X] T043 Run `cargo clippy` and fix any warnings
- [X] T044 Run `cargo fmt` to ensure consistent formatting
- [X] T045 Verify zero external dependencies in `Cargo.toml`
- [X] T046 Run full `cargo test` suite — all tests pass with zero warnings

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2 — MVP delivery
- **US2 (Phase 4)**: Depends on Phase 3 (T023 modifies server.rs, US2 verifies no regression)
- **US3 (Phase 5)**: Depends on Phase 3 (extends block parser in render function)
- **US4 (Phase 6)**: Depends on Phase 3 (extends inline parser)
- **Polish (Phase 7)**: Depends on all user stories complete

### Within Each User Story

- Tests MUST be written and FAIL before implementation (Constitution Principle I)
- Block parsers before integration
- Inline parsers before block parsers that use them (links/images inside list items)
- `render()` function last (orchestrates all parsers)

### Parallel Opportunities

- T003, T004, T005 can run in parallel (independent test functions)
- T006, T007 can run in parallel (independent utility functions)
- T009–T014 can all run in parallel (independent test functions)
- T016, T019, T020 can run in parallel (independent block parsers)
- T027–T031 can all run in parallel (independent test functions)
- US3 and US4 can proceed in parallel after US1 is complete

---

## Parallel Example: User Story 1

```text
# Launch all tests for US1 together:
T009: Test heading rendering in src/markdown.rs
T010: Test paragraph rendering in src/markdown.rs
T011: Test inline formatting in src/markdown.rs
T012: Test link rendering in src/markdown.rs
T013: Test fenced code blocks in src/markdown.rs
T014: Test indented code blocks in src/markdown.rs

# Then launch independent parsers together:
T016: Implement heading parser in src/markdown.rs
T019: Implement fenced code block parser in src/markdown.rs
T020: Implement indented code block parser in src/markdown.rs
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2)

1. Complete Phase 1: Setup (T001–T002)
2. Complete Phase 2: Foundational (T003–T008)
3. Complete Phase 3: User Story 1 — core rendering (T009–T023)
4. Complete Phase 4: User Story 2 — regression check (T024–T026)
5. **STOP and VALIDATE**: Test with real `.md` files in browser

### Incremental Delivery

1. Setup + Foundational → utilities ready
2. Add US1 → headings, paragraphs, formatting, links, code blocks → MVP!
3. Add US2 → verify no regression → confidence
4. Add US3 → lists, blockquotes, horizontal rules → richer docs
5. Add US4 → images → complete feature
6. Polish → edge cases, clippy, fmt → ship it

---

## Notes

- All tasks in `src/markdown.rs` except T023–T026 (server integration)
- Single-pass parser per research.md — no intermediate AST
- HTML escaping applied in all text content (no raw HTML passthrough)
- Constitution enforces: test first, zero deps, minimal allocation
