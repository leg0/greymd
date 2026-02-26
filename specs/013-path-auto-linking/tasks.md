# Tasks: Path Auto-Linking

**Input**: Design documents from `/specs/013-path-auto-linking/`
**Prerequisites**: plan.md, spec.md, research.md

**Tests**: TDD is mandatory per constitution principle I. Each implementation task is preceded by its test.

**Organization**: This is a refinement of an existing implementation — the code currently links any file extension but must be narrowed to `.md`-only per the updated spec. US1 and US2 are inseparable P1 stories. US3 is P2 edge cases.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: No setup needed — all changes are in existing `src/markdown.rs`. This phase verifies baseline.

**Checkpoint**: Existing `cargo test` passes (baseline).

---

## Phase 2: Core — Restrict to `.md`-only (US1 + US2)

**Purpose**: Update `try_parse_path()` to only match tokens ending with `.md` (or trailing `/` for directories). Update the code-span handler to apply the same rule. Update existing tests to reflect the new behavior.

- [x] T001 [US1] Write new tests in `src/markdown.rs` for `.md`-only matching: `examples/math-demo.md` IS linked, `README.md` IS linked, `./README.md` and `../docs/guide.md` ARE linked, `src/utils/` IS linked (trailing slash). `highlight.js` is NOT linked, `src/main.rs` is NOT linked, `Cargo.toml` is NOT linked.
- [x] T002 [US2] Write tests in `src/markdown.rs` for non-interference with `.md`-only rule: code span `examples/math-demo.md` (path-only) IS linked inside `<code>`, code span `src/main.rs` (non-`.md` path-only) is NOT linked, code span `run examples/demo.md` (extra text) is NOT linked. Existing explicit link, image, URL, fenced code block tests remain passing.
- [x] T003 [US1] Update `try_parse_path()` in `src/markdown.rs` to enforce `.md`-only extension matching. After scanning and stripping trailing punctuation, validate: return `Some(path)` if token ends with `/` OR last segment ends with `.md` (case-sensitive). Return `None` for all other extensions. Keep the existing dot-prefix guard (`.` must be followed by `/`).
- [x] T004 [US1] Update existing tests in `src/markdown.rs` that assert non-`.md` paths ARE linked (e.g., `src/main.rs`, `Cargo.toml`) — change them to assert NOT linked. Verify `cargo test` passes with all new + updated tests.

**Checkpoint**: `.md` paths and directories are linked. Non-`.md` extensions are not. `cargo test` passes.

---

## Phase 3: Edge Cases (US3)

**Purpose**: Verify edge cases work correctly under the `.md`-only rule.

- [x] T005 [US3] Write tests in `src/markdown.rs` for edge cases: trailing period after `.md` path (`docs/setup.md.` → link excludes trailing `.`), parenthesized path (`(docs/guide.md)` → link excludes parens), multiple `.md` paths on one line (`docs/a.md, docs/b.md` → two separate links), `hello/world` NOT linked, `1/2` NOT linked, `and/or` NOT linked, `highlight.js` NOT linked, `$1.20` NOT linked, path in heading text IS linked.
- [x] T006 [US3] Fix any edge case test failures by adjusting `try_parse_path()` in `src/markdown.rs`. Verify all tests from T001, T002, T004, and T005 pass. Run `cargo test` to confirm no regressions.

**Checkpoint**: All edge cases handled. `cargo test` passes.

---

## Phase 4: Polish & Validation

**Purpose**: Final validation and documentation.

- [x] T007 [P] Update `README.md` to clarify path auto-linking applies to `.md` files and directories only.
- [x] T008 Final validation: `cargo test`, `cargo test --features math`, `cargo clippy`. All must pass.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: Verify baseline
- **Phase 2 (Core)**: Tests T001+T002 first, then implementation T003+T004
- **Phase 3 (Edge Cases)**: Depends on Phase 2 (core `.md`-only logic exists)
- **Phase 4 (Polish)**: Depends on Phase 3

### Parallel Opportunities

- T001 and T002 (test writing) can be done in one pass since they're in the same file
- T007 (README) can run in parallel with T008 (validation)

---

## Implementation Strategy

### MVP (Phase 2)

1. Write failing tests for `.md`-only matching and non-interference
2. Update `try_parse_path()` to enforce `.md`-only
3. Update existing tests that assumed any-extension matching
4. **VALIDATE**: All new + existing tests pass

### Incremental (Phase 3-4)

5. Add edge case tests, fix any failures
6. README update, final validation
