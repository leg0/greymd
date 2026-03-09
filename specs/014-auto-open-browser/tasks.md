# Tasks: Auto-Open Browser on Startup

**Input**: Design documents from `/specs/014-auto-open-browser/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-contract.md

**Tests**: Included — constitution principle I (Test-First) is NON-NEGOTIABLE.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Declare the new module so subsequent phases can add code to it

- [x] T001 Add `mod browser;` declaration to src/main.rs and create empty src/browser.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Implement the core cross-platform browser-opening function via TDD, with testable seam from the start

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### TDD Cycle 1: Testable helper with non-existent command (red → green)

- [x] T002 Write test `open_with_nonexistent_command_does_not_panic` in src/browser.rs — call `open_with("nonexistent-command-xxxxx", &["http://example.com"])` and verify it does not panic. This drives the creation of the testable `open_with(command, args)` helper that silently ignores spawn errors.
- [x] T003 Implement `fn open_with(command: &str, args: &[&str])` in src/browser.rs — use `std::process::Command::new(command).args(args).spawn().ok();` to launch and silently ignore errors. This is the core seam all other code delegates to.

### TDD Cycle 2: Public API with platform dispatch (red → green)

- [x] T004 Write test `open_does_not_panic_with_valid_url` in src/browser.rs — call `open("http://127.0.0.1:9999")` and assert it does not panic
- [x] T005 Write test `open_does_not_panic_with_empty_url` in src/browser.rs — call `open("")` and assert it does not panic (edge case: graceful handling of degenerate input)
- [x] T006 Implement `pub fn open(url: &str)` in src/browser.rs — delegates to `open_with()` using `cfg!(target_os)` to select the platform command: `xdg-open` (Linux), `open` (macOS), `cmd /c start "" <url>` (Windows) per research.md R-001

### TDD Cycle 3: Non-blocking behavior (red → green)

- [x] T007 Write test `open_does_not_block_caller` in src/browser.rs — call `open()` and assert the function returns within 1 second, verifying non-blocking behavior per FR-007

**Checkpoint**: `browser::open()` and `open_with()` exist, compile, and all tests pass. The `open_with()` seam enables failure-path testing without platform-specific commands.

---

## Phase 3: User Story 1 — Automatic Browser Launch (Priority: P1) 🎯 MVP

**Goal**: When greymd starts, automatically open the default browser to the serving URL

**Independent Test**: Start greymd without flags → browser opens to the correct `http://127.0.0.1:<port>` URL

### TDD Cycle 4: URL format construction (red → green)

- [x] T008 [US1] Write test `url_format_matches_listener_address` in src/main.rs — given a `TcpListener` bound to a known address, verify that the URL string constructed as `format!("http://{}", addr)` produces the expected URL (e.g., `http://127.0.0.1:8080`). This tests the URL construction logic, not the browser launch itself.
- [x] T009 [US1] Integrate `browser::open()` call in the `main()` function in src/main.rs — insert `browser::open(&format!("http://{}", addr));` after `println!("Listening on http://{}", addr);` and before `server::start(...)`, per research.md R-004

**Checkpoint**: greymd now opens the browser on startup. User Story 1 is fully functional and testable independently.

---

## Phase 4: User Story 2 — Opt-Out via `--no-browser` Flag (Priority: P2)

**Goal**: Users can suppress browser launch with `--no-browser`

**Independent Test**: Start greymd with `--no-browser` → no browser opens, URL still printed, server works normally

### TDD Cycle 5: Flag detection (red → green)

- [x] T010 [US2] Write test `no_browser_flag_detected` in src/main.rs — given args containing `"--no-browser"`, verify the flag is parsed as `true`
- [x] T011 [US2] Write test `no_browser_flag_absent_defaults_false` in src/main.rs — given args without `"--no-browser"`, verify the flag is parsed as `false`
- [x] T012 [US2] Add `--no-browser` flag parsing in src/main.rs using `args.iter().any(|a| a == "--no-browser")` pattern consistent with existing `--version`/`--help` handling, per research.md R-003

### TDD Cycle 6: Flag filtered from positional args (red → green)

- [x] T013 [US2] Write test `no_browser_flag_filtered_from_positional_args` in src/main.rs — given args `["--no-browser", "./docs"]`, verify `"--no-browser"` is removed and `"./docs"` is extracted as the root directory
- [x] T014 [US2] Filter `"--no-browser"` from the args vector before positional argument extraction in src/main.rs — add to existing argument filtering alongside `"--theme"` filtering
- [x] T015 [US2] Wrap the `browser::open()` call in `main()` with `if !no_browser { ... }` conditional in src/main.rs

**Checkpoint**: `--no-browser` flag works. User Stories 1 AND 2 both function independently.

---

## Phase 5: User Story 3 — Graceful Failure Validation (Priority: P3)

**Goal**: Verify browser launch failures are silently ignored — no errors, no delays, server unaffected

**Independent Test**: Run greymd in a headless environment (no DISPLAY, no browser) → server starts with no errors or warnings

**Note**: The `open_with()` seam and silent error handling were built into Phase 2 by design. This phase validates the behavior through the public API and documents the contract.

- [x] T016 [US3] Write test `open_with_invalid_binary_is_silent` in src/browser.rs — call `open_with("__no_such_browser__", &["http://localhost"])` and verify: no panic, no error output, function returns normally. This extends T002's basic non-panic check by additionally asserting silent failure (no stderr output), validating FR-006 through the testable seam established in Phase 2.

**Checkpoint**: All three user stories are independently functional. Graceful failure is tested and verified.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, help text, and final validation

- [x] T017 Update `print_usage()` in src/main.rs to include `--no-browser` flag in the help output, per contracts/cli-contract.md updated usage section
- [x] T018 Run full `cargo test` suite to verify zero regressions across all ~80+ existing tests
- [x] T019 Run quickstart.md validation: build with `cargo build`, test all four scenarios (auto-open, fallback port, `--no-browser`, graceful failure)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all user stories. Builds `open_with()` seam and `open()` API via 3 TDD cycles.
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2)
- **User Story 2 (Phase 4)**: Depends on User Story 1 (Phase 3) — extends the integration point
- **User Story 3 (Phase 5)**: Depends on Foundational (Phase 2) — validates graceful failure through `open_with()` seam
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Depends on Foundational phase — no other story dependencies
- **User Story 2 (P2)**: Depends on US1 integration point existing in main.rs (needs the `browser::open()` call to wrap with conditional)
- **User Story 3 (P3)**: Depends on `open_with()` seam from Foundational — no refactoring needed, just adds a validation test

### Within Each Phase

- Each TDD cycle is atomic: one test → minimal code to pass → verify
- `cargo test` must pass after every green step
- Implementation follows strict red-green-refactor rhythm

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001)
1. Complete Phase 2: Foundational (T002-T007) — 3 TDD cycles
3. Complete Phase 3: User Story 1 (T008-T009)
4. **STOP and VALIDATE**: Start greymd → browser opens → correct URL
5. This is a usable feature already

### Incremental Delivery

1. Setup + Foundational → `browser::open()` and `open_with()` exist and are tested
2. Add User Story 1 → Browser opens on startup → **MVP ready!**
3. Add User Story 2 → `--no-browser` flag works → Safe for scripts/CI
4. Add User Story 3 → Graceful failure validated → Robust for all environments
5. Polish → Help text updated, full regression validation

---

## Notes

- TDD cycles are atomic: one test → one implementation → verify (Uncle Bob approved)
- `open_with()` seam built in Phase 2 by design — not a late refactor
- Tests use inline `#[cfg(test)]` modules (existing project convention)
- `cargo test` is the heartbeat of each cycle, not a separate task
- Commit after each phase checkpoint
