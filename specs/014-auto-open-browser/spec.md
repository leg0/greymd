# Feature Specification: Auto-Open Browser on Startup

**Feature Branch**: `014-auto-open-browser`  
**Created**: 2026-03-09  
**Status**: Draft  
**Input**: User description: "I want greymd to launch the default browser with the URL that greymd is serving the content at."

## Clarifications

### Session 2026-03-09

- Q: Should the browser open by default (opt-out via `--no-browser`) or require an explicit flag to activate (opt-in via `--open`)? → A: Open by default; users suppress with `--no-browser`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Automatic Browser Launch (Priority: P1)

As a user, when I start greymd to view my Markdown files, I want my default web browser to automatically open at the URL where greymd is serving content. This eliminates the manual step of copying the URL from the terminal and pasting it into a browser, making the workflow feel seamless and instant.

**Why this priority**: This is the core and only feature being requested. Without it, the user must manually open a browser and navigate to the printed URL every time they start greymd. Automating this step removes friction from the most common workflow: launching greymd to immediately view content.

**Independent Test**: Can be fully tested by starting greymd and observing whether the default browser opens to the correct URL. Delivers immediate value by eliminating a manual step from every greymd session.

**Acceptance Scenarios**:

1. **Given** greymd is not running, **When** the user starts greymd normally (e.g., `greymd` or `greymd ./docs`), **Then** the default system browser opens automatically to the URL greymd is serving on (e.g., `http://127.0.0.1:8080`).
2. **Given** greymd starts on its preferred port, **When** the browser opens, **Then** the URL in the browser matches the exact address and port greymd is listening on.
3. **Given** greymd's preferred port is occupied and it falls back to a random port, **When** the browser opens, **Then** the URL reflects the actual port greymd bound to, not the preferred port.

---

### User Story 2 - Opt-Out of Browser Launch (Priority: P2)

As a user running greymd in an automated script, CI environment, or secondary instance, I want the ability to suppress the automatic browser launch so that greymd does not attempt to open a browser when it is unwanted or when no desktop environment is available.

**Why this priority**: While auto-open is the default desired behavior, there are legitimate scenarios (scripting, remote servers, running multiple instances) where opening a browser is disruptive or impossible. Providing an opt-out ensures the feature doesn't break existing workflows.

**Independent Test**: Can be fully tested by starting greymd with the opt-out flag and confirming no browser opens, while verifying the server still starts and serves content normally.

**Acceptance Scenarios**:

1. **Given** the user wants to suppress browser launch, **When** the user starts greymd with a `--no-browser` flag, **Then** greymd starts and serves content normally without opening a browser.
2. **Given** the user starts greymd with `--no-browser`, **When** the server is ready, **Then** the URL is still printed to the terminal so the user can manually navigate if desired.

---

### User Story 3 - Graceful Failure When No Browser Available (Priority: P3)

As a user running greymd in a headless environment (e.g., a remote server via SSH, a container), I want greymd to continue working normally even if no browser can be launched, without errors or delays.

**Why this priority**: The server's primary purpose is serving Markdown as HTML. Browser launch is a convenience feature that must never interfere with the core functionality. Graceful degradation ensures greymd remains reliable in all environments.

**Independent Test**: Can be fully tested by running greymd in an environment with no display/browser available and confirming the server starts and serves content without error messages or delays.

**Acceptance Scenarios**:

1. **Given** greymd is running in a headless environment with no browser available, **When** greymd attempts to open the browser, **Then** the failure is silently ignored and the server continues to operate normally.
2. **Given** the browser launch fails for any reason, **When** the server is ready, **Then** there is no error message displayed to the user and no delay in server startup.

---

### Edge Cases

- What happens when greymd is started multiple times in quick succession? Each instance should attempt to open the browser independently, since each binds to a different port.
- What happens when the system's default browser is not configured? The launch attempt should fail silently without affecting server operation.
- What happens on operating systems that don't support a "default browser" concept? The feature should degrade gracefully with no impact on core functionality.
- What happens if the browser launch takes a long time (e.g., browser is not yet installed and OS prompts for installation)? The server must not block or wait for the browser process to complete.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST automatically open the user's default web browser to the serving URL when greymd starts.
- **FR-002**: The browser MUST be opened with the exact URL that greymd is listening on, including the correct port (whether default or fallback).
- **FR-003**: The browser launch MUST occur after the server is ready to accept connections, ensuring the page loads successfully on first try.
- **FR-004**: System MUST support a `--no-browser` command-line flag that suppresses the automatic browser launch.
- **FR-005**: When `--no-browser` is used, the server MUST still print the serving URL to the terminal.
- **FR-006**: If the browser launch fails (no browser available, headless environment, permission error), the system MUST continue operating normally without displaying an error to the user.
- **FR-007**: The browser launch MUST NOT block or delay the server from accepting and handling incoming connections.
- **FR-008**: The feature MUST work on Windows, macOS, and Linux desktop environments.

### Assumptions

- The default behavior is to open the browser. Users who don't want it must explicitly pass `--no-browser`.
- The browser is opened exactly once per greymd invocation, at startup.
- The feature uses the operating system's standard mechanism for opening URLs in the default browser.
- No configuration file setting is needed for this feature in the initial version; the command-line flag is sufficient.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: When a user starts greymd without `--no-browser`, the default browser opens to the correct URL within 2 seconds of the server becoming ready, 100% of the time on supported desktop environments.
- **SC-002**: The server remains fully functional and responsive regardless of whether the browser launch succeeds or fails — no degradation in serving performance.
- **SC-003**: Users can suppress browser launch via `--no-browser` with zero impact on any other greymd behavior.
- **SC-004**: On headless systems or environments without a browser, greymd starts and serves content with no visible errors or warnings related to browser launch.
