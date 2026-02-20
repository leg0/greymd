# Feature Specification: Directory Listing

**Feature Branch**: `003-directory-listing`
**Created**: 2026-02-20
**Status**: Draft
**Input**: User description: "Directory listing — When a directory is requested, return an HTML page listing its contents with navigable links"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Browse Directory Contents (Priority: P1)

A developer navigates to a directory path in the browser while using docsvr. Instead of a 404 error, they see a clean HTML page listing all files and subdirectories in that directory, with clickable links to navigate into subdirectories or open files.

**Why this priority**: This is the core purpose of the feature — without it, users can only access files by knowing exact paths.

**Independent Test**: Start docsvr with a directory containing files and subdirectories, request the directory path in a browser, verify the response lists all entries as clickable links.

**Acceptance Scenarios**:

1. **Given** docsvr is serving a directory, **When** user requests a path that is a directory, **Then** the response is an HTML page listing all files and subdirectories in that directory
2. **Given** a directory contains files and subdirectories, **When** user views the listing, **Then** each entry is a clickable link to the file or subdirectory
3. **Given** a directory listing is displayed, **When** user clicks a file link, **Then** the file is served (raw or rendered Markdown depending on extension)
4. **Given** a directory listing is displayed, **When** user clicks a subdirectory link, **Then** the subdirectory listing is displayed

---

### User Story 2 - Navigate to Parent Directory (Priority: P1)

A developer browsing a subdirectory can navigate back to the parent directory via a visible link, except when already at the root directory.

**Why this priority**: Navigation without a parent link is frustrating — users would need to manually edit the URL bar.

**Independent Test**: Request a subdirectory listing, verify a parent directory link is present and navigates correctly. Request the root directory listing, verify no parent link is shown.

**Acceptance Scenarios**:

1. **Given** user is viewing a subdirectory listing, **When** the page loads, **Then** a parent directory link (`..`) is displayed at the top of the listing
2. **Given** user clicks the parent directory link, **Then** the parent directory listing is displayed
3. **Given** user is viewing the root directory listing, **When** the page loads, **Then** no parent directory link is displayed

---

### User Story 3 - Root URL Shows Directory Listing (Priority: P1)

When a user navigates to the root URL (`/`) of the server, they see a directory listing of the served root directory, providing an entry point for browsing.

**Why this priority**: The root URL is the natural starting point — without this, users have no way to discover available files.

**Independent Test**: Start docsvr, navigate to `http://localhost:8080/`, verify the root directory contents are listed.

**Acceptance Scenarios**:

1. **Given** docsvr is running, **When** user navigates to `/`, **Then** the root directory listing is displayed
2. **Given** the root directory contains files and subdirectories, **When** the listing is displayed, **Then** all entries are shown as clickable links

---

### User Story 4 - Sorted and Organized Listing (Priority: P2)

The directory listing shows entries in a predictable, organized order so developers can quickly find what they're looking for.

**Why this priority**: An unsorted listing makes it harder to find files, especially in large directories. Important for usability but not a functional blocker.

**Independent Test**: Request a directory with various files and subdirectories, verify entries are sorted with directories grouped before files.

**Acceptance Scenarios**:

1. **Given** a directory contains both files and subdirectories, **When** the listing is displayed, **Then** subdirectories appear before files
2. **Given** entries within each group, **When** the listing is displayed, **Then** entries are sorted alphabetically (case-insensitive)

---

### Edge Cases

- What happens when a directory is empty?
  - The listing page is displayed with no entries (just the page title and parent link if applicable)
- What happens when a directory path is requested without a trailing slash?
  - The server returns the directory listing (does not require trailing slash)
- What happens when a file and directory have the same name (e.g., `docs` file and `docs/` directory)?
  - The filesystem determines precedence; the directory listing shows whatever the OS reports
- What happens with hidden files (starting with `.`)?
  - Hidden files and directories are included in the listing (no filtering)
- What happens with symlinks?
  - Symlinks are listed if they resolve within the served root directory (consistent with existing path security)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: When a request path resolves to a directory, the system MUST return an HTML page listing the `.md` files and subdirectories in that directory (non-`.md` files are excluded from the listing)
- **FR-002**: Each file in the listing MUST be a clickable link that serves that file
- **FR-003**: Each subdirectory in the listing MUST be a clickable link that shows that subdirectory's listing
- **FR-004**: Subdirectory listings MUST include a parent directory link (`..`) at the top, except for the root directory
- **FR-005**: The root URL (`/`) MUST display the listing of the served root directory
- **FR-006**: Directory listings MUST show subdirectories grouped before files
- **FR-007**: Entries within each group MUST be sorted alphabetically (case-insensitive)
- **FR-008**: The directory listing page MUST be a valid HTML5 document with the directory path as the page title
- **FR-009**: The response Content-Type for directory listings MUST be `text/html`
- **FR-010**: Filenames containing special characters (`<`, `>`, `&`, `"`) MUST be properly escaped in the HTML output
- **FR-011**: Existing file-serving and Markdown-rendering behavior MUST not be affected
- **FR-012**: When a directory contains exactly one `.md` file, requesting that directory MUST auto-serve the `.md` file (rendered as HTML) instead of showing a listing
- **FR-013**: When a directory contains multiple `.md` files and one is named `index.md`, requesting that directory MUST auto-serve `index.md` (rendered as HTML)
- **FR-014**: When a directory contains multiple `.md` files and none is named `index.md`, the directory listing MUST be shown
- **FR-015**: `index.html` MUST NOT be auto-served when a directory is requested

### Key Entities

- **Directory Entry**: A file or subdirectory within the listed directory, with a name and type (file or directory)
- **Directory Listing Page**: The HTML page containing all entries as navigable links

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All files and subdirectories in a directory are displayed as clickable links in the listing
- **SC-002**: Users can navigate the entire directory tree starting from the root URL using only clicks (no URL editing)
- **SC-003**: Directory listings render correctly in major browsers (Chrome, Firefox, Safari)
- **SC-004**: Listing a directory with 1000 entries completes in under 200ms
- **SC-005**: Existing file serving and Markdown rendering continue to work identically (no regressions)

## Clarifications

### Session 2026-02-20

- Q: Index file behavior when directory is requested? → A: Auto-serve if single .md file; if multiple, serve index.md if it exists; fall back to listing. Do not auto-serve index.html.
- Q: Which entries to show in directory listing? → A: Only list .md files and subdirectories (exclude non-.md files)

## Assumptions

- The listing only shows `.md` files and subdirectories; non-Markdown files are excluded from the listing (but are still served when requested directly by path)
- Directory entries are distinguished from files visually by a trailing `/` in their display name
- No icons or file size/date metadata are shown (keep it minimal; styling is spec 4)
- The listing page uses minimal HTML with no CSS (styling deferred to spec 4)
- The feature integrates with existing path security — directory traversal outside the root is still prevented
- When a directory is requested: if it contains exactly one `.md` file, that file is auto-served (rendered as HTML). If it contains multiple `.md` files and one is named `index.md`, that file is auto-served. Otherwise, the directory listing is shown. `index.html` is never auto-served.
