# Feature Specification: Markdown-to-HTML Rendering

**Feature Branch**: `002-markdown-rendering`
**Created**: 2026-02-20
**Status**: Draft
**Input**: User description: "Markdown-to-HTML rendering — When a .md file is requested, convert it to HTML on the fly and serve a complete HTML page"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Render Markdown as HTML (Priority: P1)

A developer browses documentation written in Markdown via the docsvr server. When they request a `.md` file, the browser displays rendered HTML instead of raw Markdown text.

**Why this priority**: This is the entire purpose of the feature. Without it, users see raw Markdown which browsers don't render.

**Independent Test**: Start docsvr with a directory containing a `.md` file, request it via browser, verify the response is an HTML page with the Markdown content converted to formatted HTML.

**Acceptance Scenarios**:

1. **Given** docsvr is serving a directory, **When** user requests a `.md` file, **Then** the response is a complete HTML page with the Markdown rendered as HTML elements
2. **Given** a Markdown file contains headings, **When** requested, **Then** the response contains corresponding `<h1>`–`<h6>` elements
3. **Given** a Markdown file contains paragraphs and line breaks, **When** requested, **Then** the response contains `<p>` elements with correct text
4. **Given** a Markdown file contains inline formatting (bold, italic, code), **When** requested, **Then** the response contains `<strong>`, `<em>`, `<code>` elements
5. **Given** a Markdown file contains links, **When** requested, **Then** the response contains `<a>` elements with correct href attributes
6. **Given** a Markdown file contains code blocks, **When** requested, **Then** the response contains `<pre><code>` elements

---

### User Story 2 - Serve Non-Markdown Files Unchanged (Priority: P1)

When a user requests a non-Markdown file (HTML, CSS, images, etc.), the server continues to serve it as raw content, exactly as before.

**Why this priority**: The Markdown feature must not break existing file serving behavior.

**Independent Test**: Request a `.html`, `.css`, `.txt`, or image file and verify the raw contents are served without modification.

**Acceptance Scenarios**:

1. **Given** docsvr is serving a directory, **When** user requests a `.html` file, **Then** the raw HTML file contents are served (no double-wrapping)
2. **Given** docsvr is serving a directory, **When** user requests a `.txt` file, **Then** the raw text contents are served

---

### User Story 3 - Render Lists and Block Elements (Priority: P2)

A developer's documentation includes lists, blockquotes, and horizontal rules. These render correctly in the HTML output.

**Why this priority**: Lists and blockquotes are extremely common in documentation but slightly more complex to parse than inline formatting.

**Independent Test**: Request a `.md` file containing ordered lists, unordered lists, and blockquotes; verify correct HTML elements.

**Acceptance Scenarios**:

1. **Given** a Markdown file contains unordered lists (`- item`), **When** requested, **Then** the response contains `<ul><li>` elements
2. **Given** a Markdown file contains ordered lists (`1. item`), **When** requested, **Then** the response contains `<ol><li>` elements
3. **Given** a Markdown file contains blockquotes (`> text`), **When** requested, **Then** the response contains `<blockquote>` elements
4. **Given** a Markdown file contains horizontal rules (`---`), **When** requested, **Then** the response contains `<hr>` elements

---

### User Story 4 - Render Images (Priority: P2)

A developer's documentation includes inline images. These render as `<img>` elements in the HTML output.

**Why this priority**: Images are common in documentation but are a distinct parsing concern from text formatting.

**Independent Test**: Request a `.md` file with `![alt](url)` syntax; verify the response contains `<img>` elements with correct src and alt attributes.

**Acceptance Scenarios**:

1. **Given** a Markdown file contains `![alt text](image.png)`, **When** requested, **Then** the response contains `<img src="image.png" alt="alt text">`

---

### Edge Cases

- What happens when a Markdown file is empty?
  - Server returns a valid HTML page with an empty body
- What happens when a Markdown file contains only raw HTML?
  - All HTML characters are escaped; raw HTML tags render as visible text (e.g., `<div>` becomes `&lt;div&gt;`)
- What happens when a Markdown file contains special characters (`<`, `>`, `&`)?
  - Characters outside of HTML/Markdown syntax are escaped to HTML entities
- What happens when a Markdown file uses Windows line endings (CRLF)?
  - Both LF and CRLF line endings are handled correctly

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: When a file with `.md` extension is requested, the system MUST convert its Markdown content to HTML
- **FR-002**: The converted HTML MUST be wrapped in a valid HTML5 document structure (`<!DOCTYPE html>`, `<html>`, `<head>`, `<body>`)
- **FR-003**: The HTML page MUST include a `<title>` element derived from the first heading in the Markdown file, or the filename if no heading exists
- **FR-004**: The system MUST support CommonMark-compatible rendering for: headings (h1-h6), paragraphs, bold, italic, combined bold+italic, inline code, code blocks, links, images, unordered lists, ordered lists, blockquotes, and horizontal rules
- **FR-005**: Non-Markdown files MUST continue to be served as raw content without modification
- **FR-006**: The response Content-Type for rendered Markdown MUST be `text/html` (not `text/markdown`)
- **FR-007**: The Markdown parser MUST be implemented with zero external dependencies
- **FR-008**: Special HTML characters in text content (`<`, `>`, `&`, `"`) MUST be escaped to prevent unintended HTML rendering
- **FR-010**: Fenced code blocks with a language identifier (e.g., ` ```rust `) MUST preserve the language as a class attribute on the `<code>` element (e.g., `<code class="language-rust">`)
- **FR-009**: Both LF and CRLF line endings MUST be handled correctly

### Key Entities

- **Markdown Source**: Raw `.md` file contents read from disk
- **HTML Output**: Rendered HTML document returned to the browser
- **Page Wrapper**: The HTML5 document shell (doctype, head, body) that wraps rendered content

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All Markdown elements listed in FR-004 render correctly in major browsers (Chrome, Firefox, Safari)
- **SC-002**: Requesting a `.md` file returns a valid HTML5 page that passes basic structure validation
- **SC-003**: Non-Markdown files continue to be served identically to spec 1 behavior (no regressions)
- **SC-004**: Rendering a typical documentation page (under 100KB Markdown) completes in under 100ms
- **SC-005**: The Markdown parser compiles with zero external dependencies

## Clarifications

### Session 2026-02-20

- Q: Should raw HTML in Markdown be passed through or escaped? → A: Escape all HTML characters everywhere (no raw HTML passthrough)
- Q: Should nested/combined inline formatting be supported? → A: Yes, support combined formatting (e.g., `***text***` → `<strong><em>text</em></strong>`)
- Q: Should fenced code block language info be preserved? → A: Yes, preserve as class attribute (`<code class="language-rust">`)

## Assumptions

- Only CommonMark-compatible subset is supported; extended syntax (tables, footnotes, task lists) is out of scope for this spec
- The HTML page wrapper uses a minimal structure with no styling (styling is spec 4)
- Nested lists are supported to a reasonable depth (at least 3 levels)
- The parser handles the most common Markdown patterns but is not required to pass the full CommonMark spec test suite
- Fenced code blocks (triple backtick) are supported; indented code blocks are supported
- Reference-style links (`[text][ref]`) are out of scope; only inline links supported
- All HTML characters in Markdown content are escaped; raw HTML is not passed through
