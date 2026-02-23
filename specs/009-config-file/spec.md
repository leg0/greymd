# Feature Specification: Configuration & Customization

**Feature Branch**: `009-config-file`
**Created**: 2026-02-23
**Status**: Draft
**Input**: User description: "Make it possible to configure port number, path to css and js with simple config file"

## Clarifications

### Session 2026-02-23

- Q: When both home and local configs specify `css` (or `js`), does local replace home or do both stack? → A: Local replaces home — only one custom CSS/JS file is active at a time. Users can use `@import` in their CSS for layering.
- Q: How should gzip work when custom CSS/JS is appended? → A: Custom CSS is served at a separate `/?css2` endpoint (uncompressed), included after the built-in `/?css` link. Custom JS replaces the built-in JS entirely — `/?js` serves either the embedded gzipped highlight.js or the custom JS file (uncompressed), never both.
- Q: Should there be a `/?js2` endpoint? → A: No. Custom JS replaces embedded JS at `/?js`, not served alongside it.
- Q: How should config file parsing handle blank lines and structure? → A: Parsing follows HTTP header rules exactly — an empty line terminates the config. Lines before the first empty line are parsed as `key: value` pairs.
- Q: What happens when a configured CSS/JS file can't be read at request time? → A: Return HTTP 404 for `/?css2` or `/?js` when the custom file is unreadable.
- Q: Security concern — config file pointing to arbitrary files is a vulnerability. How to simplify? → A: Remove config file entirely. Custom CSS/JS are well-known files at `~/.config/greymd/css` and `~/.config/greymd/js`. Port auto-selects a random port if default is busy. No `.greymd` config file.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Automatic Port Selection (Priority: P1)

A user starts greymd but the default port (8080) is already in use by another application. Instead of failing with an error, greymd automatically picks a random available port and tells the user which port it chose.

**Why this priority**: Port conflicts are the most common friction point. Auto-selection eliminates the problem entirely with zero user action needed.

**Independent Test**: Can be tested by binding port 8080 first, then starting greymd and verifying it successfully starts on a different port and prints the chosen port.

**Acceptance Scenarios**:

1. **Given** port 8080 is available, **When** the user starts greymd, **Then** the server listens on port 8080 (default)
2. **Given** port 8080 is in use, **When** the user starts greymd, **Then** the server picks a random available port and prints `Listening on http://127.0.0.1:<port>`

---

### User Story 2 - Custom Stylesheet (Priority: P2)

A user wants to customize the appearance of rendered markdown pages. They place a CSS file at `~/.config/greymd/css`. When greymd serves pages, it includes this custom CSS after the built-in styles via a separate `/?css2` endpoint, allowing the user's rules to override defaults.

**Why this priority**: Visual customization is the most impactful personalization — users in different contexts (presentations, corporate branding) need different styles.

**Independent Test**: Can be tested by placing a CSS file at `~/.config/greymd/css`, requesting `/?css2` from the server, and verifying the response contains the custom CSS content.

**Acceptance Scenarios**:

1. **Given** `~/.config/greymd/css` exists, **When** the browser requests `/?css2`, **Then** the response contains the content of that file (uncompressed, `text/css`)
2. **Given** `~/.config/greymd/css` exists, **When** greymd serves a markdown page, **Then** the HTML head includes `<link>` tags for both `/?css` and `/?css2`
3. **Given** `~/.config/greymd/css` does not exist, **When** greymd serves a markdown page, **Then** the HTML head includes only the `/?css` link (no `/?css2`)
4. **Given** `~/.config/greymd/css` does not exist, **When** the browser requests `/?css2`, **Then** greymd returns HTTP 404

---

### User Story 3 - Custom JavaScript (Priority: P3)

A user wants to use a different JavaScript file — for example, a different syntax highlighter or custom interactivity. They place a JS file at `~/.config/greymd/js`. The custom JS completely replaces the built-in highlight.js at `/?js`.

**Why this priority**: JS customization follows the same pattern as CSS but is less commonly needed.

**Independent Test**: Can be tested by placing a JS file at `~/.config/greymd/js`, requesting `/?js`, and verifying the response contains the custom JS content instead of the built-in highlight.js.

**Acceptance Scenarios**:

1. **Given** `~/.config/greymd/js` exists, **When** the browser requests `/?js`, **Then** the response contains the content of that file (uncompressed, `application/javascript`), replacing the built-in highlight.js entirely
2. **Given** `~/.config/greymd/js` does not exist, **When** the browser requests `/?js`, **Then** the response contains the built-in gzipped highlight.js (default behavior)

### Edge Cases

- What happens when `~/.config/greymd/` directory doesn't exist? greymd uses built-in defaults for everything. No error.
- What happens when `~/.config/greymd/css` is a symlink? Followed normally — this lets users point to their preferred stylesheet.
- What happens when `~/.config/greymd/css` or `js` is a directory instead of a file? Returns 404 for that endpoint.
- What happens when the custom file is modified while greymd is running? The file is re-read on every request, so changes are picked up on the next browser refresh.
- What happens when `HOME` / `USERPROFILE` is not set? greymd uses built-in defaults. No error.
- What happens when all ports are exhausted during auto-selection? greymd prints an error and exits.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: greymd MUST try the default port (8080) first
- **FR-002**: If the default port is in use, greymd MUST automatically bind to a random available port
- **FR-004**: greymd MUST check for a custom CSS file at `~/.config/greymd/css` (using `HOME` on Unix, `USERPROFILE` on Windows)
- **FR-005**: greymd MUST check for a custom JS file at `~/.config/greymd/js`
- **FR-006**: When `~/.config/greymd/css` exists, greymd MUST serve its content at `/?css2` (uncompressed) and add a `<link>` tag for `/?css2` after the built-in `/?css` in the HTML head
- **FR-007**: When `~/.config/greymd/js` exists, greymd MUST serve its content at `/?js` instead of the built-in highlight.js (replacement, not append; served uncompressed)
- **FR-008**: Custom CSS/JS files MUST be re-read on every request so users can edit and refresh
- **FR-009**: If the custom CSS/JS file cannot be read at request time, greymd MUST return HTTP 404 for that endpoint
- **FR-010**: Built-in CSS at `/?css` MUST remain gzip-compressed regardless of custom CSS presence
- **FR-011**: Built-in JS at `/?js` MUST remain gzip-compressed when no custom JS file exists

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: greymd never fails to start due to a port conflict when no explicit `--port` is given
- **SC-002**: Users can customize page appearance by placing a CSS file at a well-known path, with changes visible on browser refresh without restarting greymd
- **SC-003**: Users can replace the built-in JS by placing a file at a well-known path, with changes visible on browser refresh

## Assumptions

- "Home directory" means the value of the `HOME` environment variable on Unix or `USERPROFILE` on Windows
- The config directory follows XDG conventions: `~/.config/greymd/`
- Custom files use fixed names: `css` and `js` (no file extension needed)
- Custom files are read as raw bytes, not parsed
- Symlinks to custom files are followed normally
