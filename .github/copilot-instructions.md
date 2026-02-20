# Copilot Instructions for docsvr

## Build & Run

```sh
cargo build          # compile
cargo run            # run the binary
cargo test           # run all tests
cargo test <name>    # run a single test by name
cargo clippy         # lint
cargo fmt --check    # check formatting
```

Rust edition is **2024** (requires rustc 1.85+).

## Architecture

This is an early-stage Rust binary crate (`src/main.rs` entrypoint). No libraries or modules have been added yet.

## SpecKit Workflow

This repo uses **SpecKit** (`.github/agents/` + `.github/prompts/`) for spec-driven development. The workflow is:

1. `speckit.constitution` — Define project principles in `.specify/memory/constitution.md`
2. `speckit.specify` — Create a feature spec from a description → `specs/<feature>/spec.md`
3. `speckit.clarify` — Resolve spec ambiguities (up to 5 questions)
4. `speckit.plan` — Generate technical plan, data model, contracts
5. `speckit.tasks` — Break plan into executable tasks → `tasks.md`
6. `speckit.checklist` — Validate requirements quality
7. `speckit.analyze` — Cross-check spec/plan/tasks consistency
8. `speckit.implement` — Execute tasks phase-by-phase
9. `speckit.taskstoissues` — Push tasks to GitHub Issues

Feature artifacts live under `specs/<number>-<short-name>/`. The constitution at `.specify/memory/constitution.md` is the authority for project principles — it has not been filled in yet.
