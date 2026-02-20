# Tasks: Static File Server

**Input**: Design documents from `/specs/001-static-file-server/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: TDD is mandatory per constitution principle I. Tests are written before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup

**Purpose**: Project initialization and module structure

- [X] T001 Create module declarations in src/main.rs (declare mod server, http, mime, path)
- [X] T002 [P] Create empty module file src/server.rs
- [X] T003 [P] Create empty module file src/http.rs
- [X] T004 [P] Create empty module file src/mime.rs
- [X] T005 [P] Create empty module file src/path.rs

---

## Phase 2: Foundational (HTTP Parsing + Path Security)

**Purpose**: Core infrastructure that MUST be complete before any user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 Write tests for HTTP/1.1 request parsing in src/http.rs: parse method, path, and version from a raw request line; handle malformed requests returning None
- [X] T007 Implement HttpRequest struct and parsing logic in src/http.rs to pass T006 tests
- [X] T008 Write tests for HTTP response formatting in src/http.rs: format status line, Content-Type header, Content-Length header, and body into a byte vector
- [X] T009 Implement HttpResponse struct and formatting logic in src/http.rs to pass T008 tests
- [X] T010 Write tests for path resolution and traversal prevention in src/path.rs: resolve URL path against root dir, reject `..` traversal, reject symlinks outside root, handle URL-encoded paths
- [X] T011 Implement resolve_path function in src/path.rs to pass T010 tests (canonicalize both root and target, verify prefix relationship)

**Checkpoint**: HTTP parsing and path security are independently testable with `cargo test`

---

## Phase 3: User Story 1 + 4 — Serve Files & CLI Interface (Priority: P1) 🎯 MVP

**Goal**: User can run `docsvr [directory]` and request files over HTTP on localhost

**Independent Test**: Run `docsvr /tmp/test`, curl a file, verify 200 response with file contents

### Tests for User Stories 1 + 4

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T012 [US1] Write tests for TcpListener binding and connection acceptance in src/server.rs: bind to 127.0.0.1:port, accept connection, spawn thread
- [X] T013 [US1] Write tests for file serving in src/server.rs: given a valid file path, read file and return HttpResponse with 200 status and file bytes
- [X] T014 [US4] Write tests for CLI argument parsing in src/main.rs: no args defaults to cwd, valid directory arg uses that path, invalid directory prints error and exits, --help prints usage

### Implementation for User Stories 1 + 4

- [X] T015 [US4] Implement CLI argument parsing in src/main.rs: parse args, validate directory, print usage for --help, default to current directory when no args
- [X] T016 [US1] Implement TCP listener setup and connection accept loop in src/server.rs: bind to 127.0.0.1:8080, accept connections, spawn std::thread per connection
- [X] T017 [US1] Implement request handler in src/server.rs: read request from stream, resolve path, read file, write response; print "Listening on http://127.0.0.1:8080" on startup
- [X] T018 [US1] Wire main.rs to call server::start with parsed root directory and port

**Checkpoint**: `docsvr /tmp/test` serves files, `cargo test` passes. MVP complete.

---

## Phase 4: User Story 2 — Correct Content Types (Priority: P2)

**Goal**: Files are served with correct Content-Type headers based on file extension

**Independent Test**: Serve files with different extensions, verify Content-Type headers with curl -I

### Tests for User Story 2

- [X] T019 [US2] Write tests for MIME type mapping in src/mime.rs: .html→text/html, .css→text/css, .js→application/javascript, .json→application/json, .png→image/png, .jpg→image/jpeg, .gif→image/gif, .svg→image/svg+xml, .txt→text/plain, .md→text/markdown, .xml→application/xml, .pdf→application/pdf, .wasm→application/wasm, unknown→application/octet-stream

### Implementation for User Story 2

- [X] T020 [US2] Implement content_type_for function in src/mime.rs: match file extension to MIME type string, default to application/octet-stream
- [X] T021 [US2] Integrate mime::content_type_for into server request handler in src/server.rs: set Content-Type header based on requested file's extension

**Checkpoint**: All file types return correct Content-Type. `cargo test` passes.

---

## Phase 5: User Story 3 — Handle Missing Files & Errors (Priority: P3)

**Goal**: Server returns proper HTTP error responses for missing files, traversal attacks, and bad methods

**Independent Test**: curl non-existent paths, traversal attempts, and POST requests; verify 404/405 responses

### Tests for User Story 3

- [X] T022 [US3] Write tests for error responses in src/server.rs: 404 for non-existent file, 404 for directory traversal attempt, 405 for POST/PUT/DELETE requests

### Implementation for User Story 3

- [X] T023 [US3] Implement 404 error response in src/server.rs request handler: return "Not Found" HTML body when file doesn't exist or path resolution fails
- [X] T024 [US3] Implement 405 error response in src/server.rs request handler: check method before path resolution, return "Method Not Allowed" for non-GET
- [X] T025 [US3] Implement port-in-use error handling in src/server.rs: catch TcpListener::bind error, print clear message about port conflict, exit with non-zero status

**Checkpoint**: All error cases return correct HTTP status codes. `cargo test` passes.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and cleanup

- [X] T026 Run all quickstart.md test scenarios manually and verify expected output
- [X] T027 Run `cargo clippy` and fix any warnings in all src/*.rs files
- [X] T028 Run `cargo fmt` to ensure consistent formatting across all src/*.rs files
- [X] T029 Verify `cargo tree` shows zero external dependencies

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1+US4 (Phase 3)**: Depends on Foundational (http.rs, path.rs)
- **US2 (Phase 4)**: Can start after Foundational (mime.rs is independent), but integrates into server.rs after Phase 3
- **US3 (Phase 5)**: Depends on Phase 3 (needs working server to add error handling)
- **Polish (Phase 6)**: Depends on all user stories complete

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Implementation makes tests pass
- Refactor if needed (red-green-refactor per constitution)

### Parallel Opportunities

```bash
# Phase 1: All module files in parallel
T002, T003, T004, T005

# Phase 2: http.rs and path.rs can be developed in parallel
T006+T007 (http parsing) || T010+T011 (path security)
# T008+T009 (http response) depends on T006+T007

# Phase 4: MIME module is independent until integration
T019+T020 can start after Phase 2, before Phase 3 completes
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 4)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (HTTP + Path)
3. Complete Phase 3: Serve files + CLI
4. **STOP and VALIDATE**: `docsvr /tmp/test` works, curl returns files
5. This is a usable tool at this point

### Incremental Delivery

1. Setup + Foundational → Core infrastructure ready
2. US1+US4 → File serving works (MVP!)
3. US2 → Correct Content-Type headers
4. US3 → Proper error handling
5. Polish → Clippy clean, formatted, validated

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Constitution: TDD mandatory — all test tasks must be completed before corresponding implementation
- Constitution: Zero dependencies — all functionality uses std only
- Constitution: Minimum resources — prefer &str over String, read files into Vec<u8> once
- Commit after each task or logical group
