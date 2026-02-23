# Tasks: Configuration & Customization

**Input**: Design documents from `/specs/009-config-file/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Tests**: Required per constitution (Test-First NON-NEGOTIABLE). Strict red-green-refactor: one test → implement → refactor, then next test.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)

---

## Phase 1: Foundational (Blocking Prerequisites)

**Purpose**: Shared infrastructure changes that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T001 Write test for `config_dir()` returning correct path when `HOME` is set, then implement the function in `src/server.rs`. It returns `Option<PathBuf>` for `~/.config/greymd/`.
- [ ] T002 Write test for `config_dir()` returning None when neither `HOME` nor `USERPROFILE` is set, then make it pass in `src/server.rs`.
- [ ] T003 Change `server::start` signature from `(root: &Path, port: u16)` to `(listener: TcpListener, root: &Path, has_custom_css: bool, css_path: PathBuf, js_path: PathBuf)` in `src/server.rs`. Move `TcpListener::bind` out into `src/main.rs`. Update all existing callers and tests.

**Checkpoint**: Foundation ready — existing tests pass with new signatures, config_dir works.

---

## Phase 2: User Story 1 — Automatic Port Selection (Priority: P1) 🎯 MVP

**Goal**: greymd never fails to start due to port conflicts — try 8080, fall back to random.

**Independent Test**: Bind port 8080 first, start greymd, verify it starts on a different port.

- [ ] T004 [US1] Write test in `src/main.rs`: `bind_listener()` returns a listener on port 8080 when available. Implement `bind_listener() -> TcpListener` to make it pass.
- [ ] T005 [US1] Write test in `src/main.rs`: when port 8080 is busy (pre-bind in test), `bind_listener()` returns a listener on a different port. Implement the fallback to `TcpListener::bind("127.0.0.1:0")` to make it pass.
- [ ] T006 [US1] Update `main()` in `src/main.rs` to call `bind_listener()`, print actual port via `listener.local_addr()`, and pass listener to `server::start`. Remove `DEFAULT_PORT` const.

**Checkpoint**: greymd starts on 8080 or auto-selects random port. All tests pass.

---

## Phase 3: User Story 2 — Custom Stylesheet (Priority: P2)

**Goal**: Users place CSS at `~/.config/greymd/css` and it's served at `/?css2`, included after built-in CSS.

**Independent Test**: Create temp dir as fake config, place CSS file, request `/?css2`, verify content.

- [ ] T007 [US2] Write test in `src/server.rs`: request `/?css2` with a custom CSS file present returns 200 with file content as `text/css` (uncompressed). Implement the `/?css2` route in `handle_connection` to make it pass.
- [ ] T008 [US2] Write test in `src/server.rs`: request `/?css2` when custom CSS file does NOT exist returns 404. Make it pass (should already work from T007 implementation).
- [ ] T009 [US2] Write test in `src/markdown.rs`: `wrap_html_page` with `has_custom_css: true` includes `<link>` for `/?css2` after `/?css`. Add the `has_custom_css: bool` parameter and implement to make it pass.
- [ ] T010 [US2] Write test in `src/markdown.rs`: `wrap_html_page` with `has_custom_css: false` does NOT include `/?css2` link. Verify it passes.
- [ ] T011 [US2] Add `has_custom_css: bool` parameter to `render_listing` in `src/listing.rs`, conditionally emit `/?css2` link. Update all callers in `src/server.rs`.
- [ ] T012 [US2] In `src/main.rs`, check if `css_path` file exists at startup to set `has_custom_css`. Pass to `server::start`.

**Checkpoint**: Custom CSS served at `/?css2`, HTML includes the extra link when file exists. All tests pass.

---

## Phase 4: User Story 3 — Custom JavaScript (Priority: P3)

**Goal**: Users place JS at `~/.config/greymd/js` and it replaces built-in highlight.js at `/?js`.

**Independent Test**: Create temp dir as fake config, place JS file, request `/?js`, verify custom content.

- [ ] T013 [US3] Write test in `src/server.rs`: request `/?js` with custom JS file present returns 200 with custom file content as `application/javascript` (uncompressed, no `Content-Encoding: gzip`). Modify the `/?js` route in `handle_connection` to make it pass.
- [ ] T014 [US3] Write test in `src/server.rs`: request `/?js` when custom JS file does NOT exist returns built-in gzipped highlight.js (existing behavior). Verify it passes.

**Checkpoint**: Custom JS replaces built-in at `/?js`. Built-in JS still served when no custom file. All tests pass.

---

## Phase 5: Polish & Cross-Cutting Concerns

- [ ] T015 Update `print_usage()` in `src/main.rs` to mention `~/.config/greymd/css` and `~/.config/greymd/js` customization paths
- [ ] T016 Update `README.md` with customization section documenting the well-known paths

---

## Dependencies & Execution Order

### Phase Dependencies

- **Foundational (Phase 1)**: No dependencies — can start immediately
- **US1 (Phase 2)**: Depends on Phase 1 (needs new server::start signature)
- **US2 (Phase 3)**: Depends on Phase 1 (needs config_dir, new server::start signature)
- **US3 (Phase 4)**: Depends on Phase 1 (needs config_dir, new server::start signature)
- **Polish (Phase 5)**: Depends on all user stories being complete

### User Story Dependencies

- **US1**: Independent — only needs foundational phase
- **US2**: Independent — only needs foundational phase
- **US3**: Independent — only needs foundational phase

### TDD Cycle Per Task

Each task follows strict red-green-refactor:
1. Write ONE failing test
2. Write minimal code to make it pass
3. Refactor if needed
4. Commit
5. Move to next task

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Foundational (T001–T003)
2. Complete Phase 2: US1 — Auto Port Selection (T004–T006)
3. **STOP and VALIDATE**: greymd starts without port conflicts
4. Commit and verify all tests pass

### Incremental Delivery

1. Foundational → server signature + config paths ready
2. US1 → port auto-selection works → commit
3. US2 → custom CSS at `/?css2` → commit
4. US3 → custom JS replaces built-in → commit
5. Polish → docs updated → commit

---

## Notes

- Each task is one TDD cycle: test → implement → refactor → commit
- Zero new dependencies — all implemented with `std`
- Custom files re-read per request (edit-and-refresh workflow)
- `has_custom_css` checked once at startup for HTML generation; actual file served fresh per-request
