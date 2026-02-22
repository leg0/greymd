# Feature Specification: Asset Serving via Dedicated URI

**Feature Branch**: `006-asset-serving`  
**Created**: 2026-02-20  
**Status**: Draft  
**Input**: User description: "Serve built-in assets (CSS, JS) from a dedicated GUID-based URI path instead of embedding them inline in every page."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - CSS Served from Dedicated URI (Priority: P1)

A user opens any markdown page or directory listing. The HTML page contains a `<link>` tag referencing the stylesheet at a GUID-based path (e.g., `/<guid>/style.css`) instead of an inline `<style>` block. The browser fetches and caches the CSS, so subsequent page loads are faster.

**Why this priority**: This is the core value — moving CSS out of inline `<style>` into a cacheable asset reduces page size and enables browser caching across all pages.

**Independent Test**: Request any markdown page, verify the HTML contains a `<link rel="stylesheet">` pointing to the GUID path. Then request that GUID path directly and verify it returns valid CSS with `Content-Type: text/css`.

**Acceptance Scenarios**:

1. **Given** docsvr is running, **When** a user requests any `.md` page, **Then** the HTML contains `<link rel="stylesheet" href="/<guid>/style.css">` and no inline `<style>` block.
2. **Given** docsvr is running, **When** a user requests `/<guid>/style.css`, **Then** the server responds with `200 OK`, `Content-Type: text/css`, and the full stylesheet content.
3. **Given** docsvr is running, **When** a user requests a directory listing, **Then** the listing HTML also references the stylesheet via the same `<link>` tag.

---

### User Story 2 - GUID Path Isolation (Priority: P1)

The GUID used in the asset path is a compile-time constant that looks like a random string, ensuring it does not collide with any real file or directory names in the served content. Requests to the GUID path are handled by the server itself, not resolved against the filesystem.

**Why this priority**: Without proper isolation, the asset path could shadow real user content or be guessable/conflicting.

**Independent Test**: Verify that the GUID path is a hardcoded constant, that requests to `/<guid>/style.css` are intercepted before filesystem resolution, and that a real directory named like the GUID would not interfere.

**Acceptance Scenarios**:

1. **Given** docsvr is running, **When** a request arrives for `/<guid>/style.css`, **Then** the server serves the built-in CSS without touching the filesystem.
2. **Given** docsvr is running, **When** a request arrives for `/<guid>/unknown.file`, **Then** the server responds with `404 Not Found`.
3. **Given** docsvr is running with a content directory, **When** the content directory contains no folder matching the GUID, **Then** all normal file and directory requests work unchanged.

---

### User Story 3 - Future Asset Extensibility (Priority: P2)

The GUID-based asset path serves as a namespace for all built-in assets. Currently only `style.css` is served, but the mechanism supports adding more assets (e.g., `highlight.js`) in the future without any architectural changes.

**Why this priority**: This establishes the pattern for future assets like syntax highlighting JS, but only CSS is needed now.

**Independent Test**: Verify that the asset-serving mechanism is a simple lookup that can accommodate additional asset names.

**Acceptance Scenarios**:

1. **Given** docsvr is running, **When** a request arrives for `/<guid>/style.css`, **Then** the built-in CSS is served.
2. **Given** a new asset is added to the lookup in the future, **When** a request arrives for `/<guid>/new-asset.js`, **Then** the built-in JS would be served with the correct content type.

---

### Edge Cases

- What happens when a request targets `/<guid>/` with no filename? Server returns 404.
- What happens when a request targets `/<guid>/../../etc/passwd`? Path traversal is blocked; server returns 404.
- What happens when the GUID prefix matches a real directory? The GUID is intercepted before filesystem resolution, so it takes priority.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The server MUST define a compile-time constant GUID string (e.g., a UUID v4 without dashes) used as the asset namespace prefix.
- **FR-002**: The server MUST serve `/<guid>/style.css` with the full CSS content and `Content-Type: text/css`.
- **FR-003**: The server MUST return `404 Not Found` for any request under `/<guid>/` that does not match a known asset name.
- **FR-004**: Asset requests under `/<guid>/` MUST be intercepted before filesystem path resolution.
- **FR-005**: All generated HTML pages (markdown and directory listings) MUST reference the stylesheet via `<link rel="stylesheet" href="/<guid>/style.css">` instead of an inline `<style>` block.
- **FR-006**: The CSS content served from the asset URI MUST be identical to the CSS currently embedded inline.
- **FR-007**: The server MUST NOT allow path traversal through the asset namespace (e.g., `/<guid>/../file`).
- **FR-008**: Asset responses MUST include appropriate `Content-Type` headers based on the asset file extension.

### Key Entities

- **Asset Namespace**: A fixed GUID path prefix that the server reserves for built-in assets. Not a filesystem path.
- **Built-in Asset**: A content blob (CSS, JS, etc.) compiled into the binary as a `const &str`, served by name under the asset namespace.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every served HTML page references the stylesheet via a single `<link>` tag pointing to the GUID-based asset path.
- **SC-002**: Requesting the asset URI returns the correct content with the correct MIME type.
- **SC-003**: No inline `<style>` blocks appear in any served HTML page.
- **SC-004**: Page HTML size is reduced by the size of the CSS payload (currently ~1.5KB) compared to inline embedding.
- **SC-005**: All existing tests continue to pass (adapted to the new `<link>` approach).

## Assumptions

- The GUID is a hardcoded compile-time constant, not generated at runtime.
- Only `style.css` is served initially; the mechanism is designed to accommodate additional assets later.
- The GUID should be a valid URL path segment (alphanumeric, no special characters).
- Browser caching behavior is handled by the browser's default `<link>` caching; no explicit `Cache-Control` headers are required initially.
