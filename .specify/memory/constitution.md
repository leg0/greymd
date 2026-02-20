<!--
  Sync Impact Report
  Version change: 0.0.0 → 1.0.0
  Added principles:
    - I. Test-First (NON-NEGOTIABLE)
    - II. Minimal Dependencies
    - III. Minimum Resource Usage
  Removed sections:
    - [SECTION_2_NAME] (not needed)
    - [SECTION_3_NAME] (not needed)
  Removed principles:
    - [PRINCIPLE_4_NAME] (reduced to 3 principles)
    - [PRINCIPLE_5_NAME] (reduced to 3 principles)
  Templates requiring updates:
    ✅ .specify/templates/plan-template.md (no conflicts)
    ✅ .specify/templates/spec-template.md (no conflicts)
    ✅ .specify/templates/tasks-template.md (no conflicts)
  Follow-up TODOs: None
-->

# docsvr Constitution

## Core Principles

### I. Test-First (NON-NEGOTIABLE)

- TDD is mandatory for all feature work: tests MUST be written before implementation.
- Strict red-green-refactor cycle: write a failing test, make it pass with minimal code, then refactor.
- No production code MUST be written without a corresponding failing test.
- `cargo test` MUST pass before any commit.

### II. Minimal Dependencies

- The project MUST use zero external crate dependencies whenever feasible.
- Every proposed dependency MUST be justified with a clear rationale for why the standard library is insufficient.
- Prefer implementing functionality with `std` over adding a crate.

### III. Minimum Resource Usage

- The binary MUST minimize memory allocation and CPU usage.
- Avoid unnecessary copying; prefer borrowing and zero-copy patterns.
- Performance-sensitive paths MUST be measured before and after changes.
- Resource usage regressions MUST be treated as bugs.

## Governance

- This constitution supersedes all other development practices for docsvr.
- Amendments require updating this file, incrementing the version, and documenting the change rationale.
- Versioning follows semantic versioning: MAJOR for principle removals/redefinitions, MINOR for additions, PATCH for clarifications.
- All code changes MUST comply with these principles.

**Version**: 1.0.0 | **Ratified**: 2026-02-20 | **Last Amended**: 2026-02-20
