# Tasks: Math Rendering

**Input**: Design documents from `/specs/011-math-rendering/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Tests**: TDD is mandatory per constitution principle I. Each implementation task includes writing a failing test first.

**Organization**: Tasks grouped by user story. Constitution requires test-first for all code.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Add the feature flag and optional dependency.

- [x] T001 Add `[features]` section with `math = ["dep:latex2mathml"]` (not in default) and `latex2mathml = { version = "0.2", optional = true }` to `Cargo.toml`. Verify `cargo build` and `cargo build --features math` both succeed.
- [x] T002 Verify `cargo test` still passes (159 tests) with no feature flags. Verify `cargo test --features math` also passes. No code changes — just confirm the dependency compiles cleanly.

**Checkpoint**: Feature flag wired up. Both builds compile. All 159 existing tests pass in both modes.

---

## Phase 2: User Story 1 — Inline Math Rendering (Priority: P1) 🎯 MVP

**Goal**: `$...$` in markdown produces inline MathML when built with `--features math`.

**Independent Test**: Render `$E = mc^2$` and verify HTML contains `<math>` tags.

- [x] T003 [US1] Write test in `src/markdown.rs`: `render_inline("$E = mc^2$")` with `math` feature produces output containing `<math`. Without `math` feature, output contains `$E = mc^2$` as plain text. Use `#[cfg(feature = "math")]` and `#[cfg(not(feature = "math"))]` for separate test functions. Tests fail initially.
- [x] T004 [US1] Write test in `src/markdown.rs`: `render_inline("Price is $5 and $10")` does NOT produce `<math` tags — the space rule (FR-009: no space after opening `$`, no space before closing `$`) prevents false matches. Also test `$x^2$` (valid, no spaces) does match. Test fails initially.
- [x] T005 [US1] Write test in `src/markdown.rs`: `render_inline("use \\`$x$\\` in code")` — `$` inside inline code spans MUST NOT be treated as math. Specifically: `` `$x$` `` renders as `<code>$x$</code>`, not math. Test fails initially.
- [x] T006 [US1] Implement inline math parsing in `render_inline()` in `src/markdown.rs`. After the backtick handling block, add `$` detection: find opening `$` (not preceded by `\`, not followed by space), find matching closing `$` (not preceded by space, not followed by `$`), extract LaTeX, and call `latex2mathml::latex_to_mathml(latex, DisplayStyle::Inline)` gated by `#[cfg(feature = "math")]`. On `Err`, wrap raw LaTeX in `<code>`. Without math feature, emit raw `$...$` text. Make T003-T005 pass.
- [x] T007 [US1] Write test in `src/markdown.rs`: `render_inline("\\$100")` (escaped dollar) renders as literal `$100`, not a math delimiter. Implement `\$` escape handling in `render_inline`. Make test pass.

**Checkpoint**: Inline `$...$` math works. Code spans take priority. Dollar-space rule prevents currency false positives. Escaped `\$` works. All tests pass in both feature modes.

---

## Phase 3: User Story 2 — Display Math Rendering (Priority: P1)

**Goal**: `$$...$$` at line start produces block MathML when built with `--features math`.

**Independent Test**: Render `$$\sum_{i=1}^{n} i$$` on its own line and verify block `<math display="block">`.

- [x] T008 [US2] Write test in `src/markdown.rs`: `render_body("$$\\sum x$$\n")` with `math` feature produces output containing `<math` and `display="block"`. Without math feature, output contains `$$\sum x$$` as plain text. Test fails initially.
- [x] T009 [US2] Write test in `src/markdown.rs`: multi-line display math — input `"$$\n\\frac{a}{b}\n$$\n"` collects everything between `$$` lines and renders as a single block equation. Test fails initially.
- [x] T010 [US2] Write test in `src/markdown.rs`: `$$` inside a fenced code block is NOT treated as display math. E.g., `` "```\n$$x$$\n```\n" `` renders as code, not math. Test fails initially (should already pass due to fenced code priority, but verify).
- [x] T011 [US2] Implement display math parsing in `render_body()` in `src/markdown.rs`. Add a new `BlockState::DisplayMath` variant. When a line starts with `$$` (and state is not fenced/indented code), enter display math state and collect lines until a closing `$$` line. On close, call `latex2mathml::latex_to_mathml(collected, DisplayStyle::Block)` gated by `#[cfg(feature = "math")]`. On `Err`, wrap in `<pre><code>`. Without math feature, emit raw `$$...$$` as paragraph text. Make T008-T010 pass.
- [x] T012 [US2] Write test in `src/markdown.rs`: empty `$$` on one line (`"$$$$\n"` or `"$$\n$$\n"`) is passed through as literal text, not treated as math. Implement and make test pass.

**Checkpoint**: Block `$$...$$` math works. Multi-line supported. Code blocks take priority. Empty `$$` handled. All tests pass in both modes.

---

## Phase 4: User Story 3 — Graceful Degradation (Priority: P1)

**Goal**: Default build (no `math` feature) passes through `$`/`$$` as plain text.

**Independent Test**: Build without `math`, render file with `$x^2$`, verify raw text in output.

- [x] T013 [US3] Write integration test in `src/server.rs`: start server (default build, no math feature), serve a `.md` file containing `$x^2$` and `$$\sum x$$`, verify the response HTML contains the raw LaTeX text as-is. Make test pass. (This should already pass from T006/T011 `#[cfg(not(feature = "math"))]` paths, but verify end-to-end.)

**Checkpoint**: Default build passes through math content unmodified. Verified end-to-end.

---

## Phase 5: User Story 4 — Math Support in Help (Priority: P2)

**Goal**: `--help` shows math support status.

- [x] T014 [US4] Add a conditional line to `print_usage()` in `src/main.rs`: when `math` feature is enabled, print `"  Math rendering:          Enabled (LaTeX → MathML)"`. When disabled, omit the line entirely. Use `#[cfg(feature = "math")]`. No test needed — this is a print statement verified by inspection.

**Checkpoint**: `--help` indicates math support when compiled in.

---

## Phase 6: User Story 5 — Release Variants (Priority: P2)

**Goal**: Release workflow produces both standard and math variants per platform.

- [x] T015 [US5] Update `.github/workflows/release.yml` matrix to include math variants. Add entries with `features: "math"` and artifact names `greymd-math-linux-x64` and `greymd-math-windows-x64`. Modify build step to conditionally pass `--features ${{ matrix.features }}`. Both standard and math variants include themes in the same `bin/` + `share/` layout. Test both `cargo build` and `cargo build --features math` pass before committing.

**Checkpoint**: Release workflow produces 4 archives: standard + math for each platform.

---

## Phase 7: Polish & Cross-Cutting Concerns

- [x] T016 Add math CSS font hint: when `math` feature is enabled, append `math { font-family: "STIX Two Math", "Latin Modern Math", math; }` to the built-in CSS in `build.rs` or as a conditional string in `wrap_html_page()` in `src/markdown.rs`. Use `#[cfg(feature = "math")]`.
- [x] T017 Create `examples/math-demo.md` showcasing inline math, display math, mixed content, and edge cases (code blocks with `$`, escaped `\$`, currency).
- [x] T018 Update `README.md`: add Math Rendering section documenting `--features math`, supported syntax (`$...$`, `$$...$$`), browser requirements, and release variant naming.
- [x] T019 Run `cargo test` and `cargo test --features math` — all tests pass. Run `cargo clippy` and `cargo clippy --features math` — no warnings. Final validation.

**Checkpoint**: CSS font hint included. Demo file, README updated. All tests pass in both modes. Clippy clean.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **US1 Inline Math (Phase 2)**: Depends on Phase 1 (feature flag must exist)
- **US2 Display Math (Phase 3)**: Depends on Phase 1 (can run in parallel with US1 if needed, but sequential is safer since both modify `markdown.rs`)
- **US3 Degradation (Phase 4)**: Depends on Phase 2 and Phase 3 (tests verify both paths exist)
- **US4 Help (Phase 5)**: Depends on Phase 1 only (independent of US1/US2)
- **US5 Release (Phase 6)**: Depends on Phase 1 only (independent of code changes)
- **Polish (Phase 7)**: Depends on all user stories

### TDD Cycle Per Task

Each implementation task follows strict red-green-refactor:
1. **Red**: Write failing test
2. **Green**: Minimal code to pass
3. **Refactor**: Clean up
4. **Commit**: `cargo test` and `cargo test --features math` both pass

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Feature flag setup
2. Complete Phase 2: Inline `$...$` math
3. **STOP and VALIDATE**: Inline math renders in browser

### Incremental Delivery

1. Phase 1 → Feature flag wired
2. Phase 2 (US1) → Inline math works (MVP!)
3. Phase 3 (US2) → Display math works
4. Phase 4 (US3) → Degradation verified
5. Phase 5 (US4) → Help text updated
6. Phase 6 (US5) → Release variants configured
7. Phase 7 → Polish, demo, docs

---

## Notes

- All tests must run under BOTH `cargo test` (no features) and `cargo test --features math`
- `render_inline` in `src/markdown.rs` is the hot path for inline math — `$` detection must be added after backtick handling (precedence: code > math)
- `render_body` in `src/markdown.rs` handles block-level constructs — `$$` detection goes alongside fenced code block handling
- `BlockState` enum needs a new `DisplayMath` variant for `$$` blocks
- `latex2mathml` API: `latex_to_mathml(latex: &str, DisplayStyle::Inline|Block) -> Result<String, LatexError>`
- The `\$` escape must be handled before `$` delimiter detection in `render_inline`
