# Feature Specification: HTML Styling

**Feature Branch**: `004-html-styling`  
**Created**: 2026-02-20  
**Status**: Draft  
**Input**: User description: "HTML styling with embedded CSS for rendered markdown pages and directory listings"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Readable Markdown Pages (Priority: P1)

A user opens a rendered markdown document in their browser and sees well-formatted content with readable typography, proper spacing, and visual hierarchy. Headings are clearly distinct from body text, code blocks are visually separated, and links are identifiable.

**Why this priority**: The primary purpose of docsvr is serving readable documentation. Without styling, raw HTML output is difficult to scan and read.

**Independent Test**: Can be fully tested by serving any `.md` file and verifying the browser renders it with styled typography, spacing, and visual hierarchy.

**Acceptance Scenarios**:

1. **Given** a markdown file with headings, paragraphs, and code blocks, **When** the user opens it in a browser, **Then** they see styled content with clear visual hierarchy.
2. **Given** a markdown file with a fenced code block, **When** rendered, **Then** the code block has a distinct background and monospace font.
3. **Given** a markdown file with inline code, **When** rendered, **Then** inline code is visually distinct from surrounding text.

---

### User Story 2 - Styled Directory Listings (Priority: P2)

A user navigates to a directory and sees a clean, scannable listing page. Files and directories are clearly presented with consistent spacing, and links are easy to click.

**Why this priority**: Directory listings are the navigation UI. Without styling, the raw `<ul>` output is difficult to browse.

**Independent Test**: Can be tested by navigating to a directory with multiple `.md` files and subdirectories and verifying the listing is styled and scannable.

**Acceptance Scenarios**:

1. **Given** a directory with `.md` files and subdirectories, **When** the user views the listing, **Then** entries are displayed in a clean list with adequate spacing.
2. **Given** a listing with a parent link (`..`), **When** rendered, **Then** the parent link is visually consistent with other entries.

---

### User Story 3 - Consistent Look Across Page Types (Priority: P3)

A user navigating between markdown pages and directory listings experiences a consistent visual style — same fonts, colors, and page layout — so the site feels cohesive.

**Why this priority**: Visual consistency reduces cognitive load and makes the tool feel polished.

**Independent Test**: Can be tested by opening a markdown page and a directory listing side by side and verifying consistent fonts, colors, and layout.

**Acceptance Scenarios**:

1. **Given** a rendered markdown page and a directory listing, **When** the user views both, **Then** the base typography, link colors, and page margins are identical.

---

### User Story 4 - Responsive Reading Width (Priority: P3)

A user on a wide monitor sees content constrained to a comfortable reading width rather than spanning the full viewport. On narrow screens, content fills the available width naturally.

**Why this priority**: Extremely long lines hurt readability. A max-width creates a better reading experience at no user cost.

**Acceptance Scenarios**:

1. **Given** a browser window wider than 900px, **When** viewing any page, **Then** the content area does not exceed a comfortable maximum width and is horizontally centered.
2. **Given** a narrow browser window (e.g., 400px), **When** viewing any page, **Then** content fills the available width with appropriate padding.

---

### Edge Cases

- What happens when a markdown file contains very wide content (e.g., a long unbroken string or wide code block)? Code blocks should scroll horizontally rather than breaking the page layout.
- What happens when a directory listing entry has a very long filename? The name should wrap or truncate gracefully.
- How does the page look with no content (empty markdown file)? The page should still display a properly styled shell with the title.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST embed a `<style>` block in every HTML page (both markdown-rendered and directory listing pages).
- **FR-002**: All styles MUST be embedded inline in the HTML — no external CSS files or network requests.
- **FR-003**: The system MUST style headings (h1–h6) with distinct sizes and appropriate vertical spacing.
- **FR-004**: The system MUST style paragraphs with readable line-height and vertical spacing.
- **FR-005**: The system MUST style fenced and indented code blocks with a distinct background, border, monospace font, and horizontal scrolling for overflow.
- **FR-006**: The system MUST style inline code with a distinct background and monospace font.
- **FR-007**: The system MUST style links with a visible color distinct from body text.
- **FR-008**: The system MUST style unordered and ordered lists with appropriate indentation and spacing.
- **FR-009**: The system MUST style blockquotes with a left border and indentation.
- **FR-010**: The system MUST style horizontal rules as a visible separator.
- **FR-011**: The system MUST constrain body content to a maximum width for comfortable reading and center it horizontally.
- **FR-012**: The system MUST apply consistent base typography (font family, size, color) to all page types.
- **FR-013**: The system MUST style directory listing entries with adequate spacing for easy scanning and clicking.
- **FR-014**: The system MUST include a viewport meta tag for proper rendering on mobile devices.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every HTML page served by docsvr contains an embedded `<style>` block with CSS rules.
- **SC-002**: All markdown element types (headings, paragraphs, code, links, lists, blockquotes, horizontal rules) have distinct, visually appropriate styling.
- **SC-003**: Directory listing pages have styled entries with readable spacing.
- **SC-004**: Content width is constrained on viewports wider than 900px.
- **SC-005**: Pages render with no external resource requests (verified by absence of `<link>` or `@import` in output).
- **SC-006**: Existing tests continue to pass (styling changes do not break current behavior).

## Clarifications

### Session 2026-02-20

- Q: Should CSS be embedded inline or served as a separate file? → A: Embedded inline as a `<style>` block. CSS is a compile-time constant in the binary, injected into every HTML page.

## Assumptions

- The CSS is stored as a compile-time string constant in the binary and injected into the `<head>` of every HTML page via a `<style>` block.
- The embedded CSS uses a system font stack (no web fonts) to maintain the zero-dependency constraint.
- Colors use a light-background / dark-text scheme (no dark mode).
- The CSS targets modern browsers — no IE compatibility needed.
- Images in markdown (`<img>`) are styled with a max-width of 100% to prevent overflow.
