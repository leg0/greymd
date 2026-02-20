# Tasks: Markdown Tables

**Input**: Design documents from `/specs/005-markdown-tables/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Organization**: Tasks grouped by user story. Constitution requires TDD — tests written before implementation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Verify baseline before any changes.

- [X] T001 Verify all 103 tests pass and clippy is clean (`cargo test && cargo clippy`)

---

## Phase 2: Foundational (Helper Functions + Alignment Enum)

**Purpose**: Build the shared infrastructure that all user stories depend on: `Alignment` enum, separator validation, cell splitting, and table rendering function.

**⚠️ CRITICAL**: All user stories depend on these helpers.

### Tests

- [X] T002 Write test: `is_table_separator` returns true for `|---|---|` and `| :--- | ---: | :---: |` in src/markdown.rs
- [X] T003 Write test: `is_table_separator` returns false for `| no dashes |` and regular text in src/markdown.rs
- [X] T004 Write test: `split_table_cells` splits `| a | b | c |` into `["a", "b", "c"]` in src/markdown.rs
- [X] T005 Write test: `split_table_cells` handles no leading/trailing pipes `a | b | c` in src/markdown.rs
- [X] T006 Write test: `split_table_cells` preserves pipes inside backtick code spans in src/markdown.rs
- [X] T007 Write test: `split_table_cells` handles empty cells `| a || c |` in src/markdown.rs

### Implementation

- [X] T008 Add `Alignment` enum (Left, Center, Right) to src/markdown.rs
- [X] T009 Implement `is_table_separator(line: &str) -> bool` in src/markdown.rs
- [X] T010 Implement `split_table_cells(line: &str) -> Vec<&str>` with backtick-aware scanning in src/markdown.rs
- [X] T011 Implement `parse_alignment(separator: &str) -> Vec<Alignment>` in src/markdown.rs
- [X] T012 Run `cargo test` — all foundational tests pass

**Checkpoint**: Helper functions ready. Table rendering can now be built.

---

## Phase 3: User Story 1 — Render Basic Tables (Priority: P1) 🎯 MVP

**Goal**: Parse pipe-delimited tables and render as `<table>` with `<thead>` and `<tbody>`.

**Independent Test**: Create markdown with a table, verify HTML output contains correct `<table>`, `<th>`, `<td>` elements.

### Tests

- [X] T013 [US1] Write test: basic 3-column table renders `<table>`, `<thead>`, `<tbody>`, 3 `<th>`, correct `<td>` count in src/markdown.rs
- [X] T014 [US1] Write test: table with inline formatting (bold, code, links) in cells renders correctly in src/markdown.rs
- [X] T015 [US1] Write test: header-only table (no data rows) renders `<table>` with `<thead>` and empty `<tbody>` in src/markdown.rs
- [X] T016 [US1] Write test: lines without separator row are NOT treated as table (rendered as paragraphs) in src/markdown.rs
- [X] T017 [US1] Write test: table followed by paragraph renders both correctly in src/markdown.rs
- [X] T018 [US1] Write test: table preceded by heading renders both correctly in src/markdown.rs

### Implementation

- [X] T019 [US1] Implement `render_table(header: &str, separator: &str, data_rows: &[&str]) -> String` in src/markdown.rs
- [X] T020 [US1] Add table detection block in `render_body` main loop (lookahead for separator row) in src/markdown.rs
- [X] T021 [US1] Run `cargo test` — all US1 tests pass

**Checkpoint**: Basic tables render correctly. Verify with sample markdown.

---

## Phase 4: User Story 2 — Column Alignment (Priority: P2)

**Goal**: Parse alignment markers from separator row and apply `style="text-align: ..."` to cells.

**Independent Test**: Create table with `:---`, `:---:`, `---:` in separator, verify `style` attributes on rendered cells.

### Tests

- [X] T022 [US2] Write test: `parse_alignment` returns correct alignment for `:---`, `:---:`, `---:`, `---` in src/markdown.rs
- [X] T023 [US2] Write test: rendered table cells have correct `style="text-align: ..."` attributes in src/markdown.rs
- [X] T024 [US2] Write test: default alignment (no colons) produces no `style` attribute or `text-align: left` in src/markdown.rs

### Implementation

- [X] T025 [US2] Integrate `parse_alignment` into `render_table` to apply `style` attributes on `<th>` and `<td>` in src/markdown.rs
- [X] T026 [US2] Run `cargo test` — all US2 tests pass

**Checkpoint**: Column alignment works correctly.

---

## Phase 5: User Story 3 — Styled Tables (Priority: P3)

**Goal**: Add CSS rules for tables to the embedded stylesheet.

**Independent Test**: Verify `const CSS` contains `table`, `th`, `td` rules with borders and padding.

### Tests

- [X] T027 [US3] Write test: HTML output contains CSS rules for `table`, `th`, `td` with border and padding in src/markdown.rs

### Implementation

- [X] T028 [US3] Add table CSS rules to `const CSS` in src/markdown.rs: `border-collapse`, cell borders, padding, header background
- [X] T029 [US3] Run `cargo test` — US3 test passes

**Checkpoint**: Tables are visually styled.

---

## Phase 6: Edge Cases

**Purpose**: Handle mismatched columns, empty cells, and pipe-in-code edge cases.

### Tests

- [X] T030 Write test: row with fewer columns than header pads with empty `<td>` in src/markdown.rs
- [X] T031 Write test: row with more columns than header truncates to header count in src/markdown.rs
- [X] T032 Write test: empty cell (consecutive pipes) renders empty `<td>` in src/markdown.rs
- [X] T033 Write test: cell containing pipe in inline code renders correctly in src/markdown.rs
- [X] T034 Write test: table with no leading/trailing pipes on rows renders correctly in src/markdown.rs

### Implementation

- [X] T035 Verify edge cases are handled by existing `split_table_cells` and `render_table` logic in src/markdown.rs (fix if needed)
- [X] T036 Run `cargo test` — all edge case tests pass

**Checkpoint**: All edge cases handled.

---

## Phase 7: Polish & Cross-Cutting Concerns

- [X] T037 Run `cargo clippy` — zero warnings
- [X] T038 Run `cargo fmt --check` — no formatting issues
- [X] T039 Run full `cargo test` — all tests pass (expected: ~120+)
- [X] T040 Verify zero external dependencies in Cargo.toml
- [X] T041 Verify existing non-table tests still pass (no regressions)
- [X] T042 Run quickstart.md validation: serve sample table markdown, verify in browser

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — verify baseline
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2 — core table rendering
- **US2 (Phase 4)**: Depends on Phase 2 — alignment is independent of US1 rendering logic but uses same helpers
- **US3 (Phase 5)**: Depends on Phase 2 — CSS addition is independent
- **Edge Cases (Phase 6)**: Depends on Phase 3 (needs table rendering working)
- **Polish (Phase 7)**: Depends on all phases complete

### User Story Dependencies

- **US1 (P1)**: Depends on foundational helpers — core table parsing/rendering
- **US2 (P2)**: Depends on foundational helpers — alignment parsing already built in Phase 2
- **US3 (P3)**: Independent — CSS-only change

### Parallel Opportunities

- T002–T007 can run in parallel (independent test functions)
- T013–T018 can run in parallel (independent US1 tests)
- T022–T024 can run in parallel (independent US2 tests)
- T030–T034 can run in parallel (independent edge case tests)
- US2 and US3 can proceed in parallel after Phase 2

---

## Implementation Strategy

### MVP First (Phase 2 + US1)

1. Complete Phase 1: Verify baseline
2. Complete Phase 2: Helper functions + Alignment enum
3. Complete Phase 3: US1 — basic table rendering
4. **STOP and VALIDATE**: Test with sample markdown
5. Tables are now functional

### Incremental Delivery

1. Setup + Foundational → helpers ready
2. Add US1 → tables render → MVP!
3. Add US2 → alignment works
4. Add US3 → tables are styled
5. Edge cases → robustness
6. Polish → clean and verified

---

## Notes

- All changes in a single file: `src/markdown.rs`
- Table detection uses one-line lookahead in existing `render_body` loop
- `split_table_cells` must track backtick state for pipe-in-code safety
- `render_table` calls existing `render_inline` for cell content formatting
- Total: 42 tasks across 7 phases
