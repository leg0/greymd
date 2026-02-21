# Feature Specification: Syntax Highlighting for Code Blocks

**Feature Branch**: `007-syntax-highlighting`
**Created**: 2026-02-21
**Status**: Draft
**Input**: User description: "Add syntax highlighting for fenced code blocks using embedded highlight.js (common bundle, GitHub theme)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Syntax-Highlighted Code Blocks (Priority: P1)

A user views a markdown page containing fenced code blocks with language hints (e.g., ` ```rust `). The code is rendered with syntax highlighting — keywords, strings, comments, and other tokens are visually distinct using color. The highlighting uses a GitHub-like color theme consistent with the existing page style.

**Why this priority**: This is the entire feature — making code blocks readable with colored syntax.

**Independent Test**: Request a markdown page with a fenced ` ```rust ` block. Verify the HTML includes the highlight.js script and GitHub theme CSS via `<script>` and `<link>` tags pointing to the GUID asset path. Verify the page loads correctly in a browser with colored syntax.

**Acceptance Scenarios**:

1. **Given** docsvr is running, **When** a user requests a `.md` page with a fenced code block specifying a language, **Then** the HTML includes `<script>` and `<link>` tags for highlight.js and its theme CSS.
2. **Given** docsvr is running, **When** a user requests `/<guid>/highlight.min.js`, **Then** the server responds with `200 OK`, `Content-Type: application/javascript`, and the highlight.js library content.
3. **Given** docsvr is running, **When** a user requests `/<guid>/highlight-github.css`, **Then** the server responds with `200 OK`, `Content-Type: text/css`, and the GitHub theme CSS.
4. **Given** a page has loaded with highlight.js, **When** the browser executes the script, **Then** all `<code>` elements with a `language-*` class are highlighted.

---

### User Story 2 - Auto-Detection for Unlabeled Code Blocks (Priority: P2)

A user views a markdown page with fenced code blocks that have no language hint (plain ` ``` `). highlight.js attempts auto-detection of the language and applies best-effort highlighting.

**Why this priority**: Nice-to-have; highlight.js does this automatically with no extra work.

**Independent Test**: Request a page with an unlabeled fenced code block containing recognizable code (e.g., Python). Verify highlight.js is loaded and auto-detection can run.

**Acceptance Scenarios**:

1. **Given** a page with an unlabeled fenced code block, **When** the page loads, **Then** highlight.js attempts auto-detection and applies highlighting if confident.

---

### Edge Cases

- Pages with no code blocks: highlight.js is still loaded (simplicity over optimization) but does nothing.
- Code blocks with unknown/unsupported language hints: highlight.js falls back to no highlighting for that block.
- Very large code blocks: highlight.js handles them client-side; no server-side concern.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The binary MUST embed the highlight.js common bundle (minified JS, ~40KB) as a compile-time constant.
- **FR-002**: The binary MUST embed the highlight.js GitHub theme CSS as a compile-time constant.
- **FR-003**: The server MUST serve `/<guid>/highlight.min.js` with `Content-Type: application/javascript`.
- **FR-004**: The server MUST serve `/<guid>/highlight-github.css` with `Content-Type: text/css`.
- **FR-005**: All generated HTML pages MUST include a `<link>` tag for the highlight.js GitHub theme CSS.
- **FR-006**: All generated HTML pages MUST include a `<script>` tag loading highlight.js from the GUID asset path.
- **FR-007**: All generated HTML pages MUST include an inline `<script>hljs.highlightAll();</script>` to trigger highlighting on page load.
- **FR-008**: The highlight.js `<script>` tag MUST appear before the `hljs.highlightAll()` call.

### Key Entities

- **highlight.min.js**: The highlight.js common bundle (~40KB minified), embedded as a const, served at `/<guid>/highlight.min.js`.
- **highlight-github.css**: The GitHub theme CSS for highlight.js, embedded as a const, served at `/<guid>/highlight-github.css`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every served HTML page includes `<script>` and `<link>` tags for highlight.js assets.
- **SC-002**: Requesting highlight.js asset URIs returns correct content with correct MIME types.
- **SC-003**: Fenced code blocks with language hints display syntax-colored output in a browser.
- **SC-004**: All existing tests continue to pass (adapted for new `<script>`/`<link>` tags).

## Assumptions

- The highlight.js common bundle covers ~40 popular languages (Rust, Python, JS, Go, C/C++, Java, SQL, Bash, etc.).
- The GitHub theme CSS is a single file that styles highlight.js output.
- highlight.js is loaded on all pages (including directory listings) for simplicity — no conditional loading.
- The `hljs.highlightAll()` call is a small inline script, not a separate asset.
- No server-side highlighting — all rendering happens client-side in the browser.
