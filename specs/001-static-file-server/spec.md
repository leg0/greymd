# Feature Specification: Static File Server

**Feature Branch**: `001-static-file-server`  
**Created**: 2025-01-20  
**Status**: Draft  
**Input**: User description: "Static file server — Serve raw files from a directory over HTTP on localhost. CLI takes a directory path argument. This is a Rust binary (edition 2024, zero dependencies per the constitution). The binary name is "docsvr". It listens on localhost only (no TLS needed). When a file is requested, serve its raw contents with the appropriate content type. This is spec 1 of 4 for the docsvr project."

## Clarifications

### Session 2026-02-20

- Q: SC-006 contradicts the Assumption about port behavior — should the server auto-discover a port or fail? → A: Fail to start with error message when port is busy.
- Q: What should happen with non-GET HTTP methods (POST, PUT, DELETE)? → A: Return 405 Method Not Allowed.
- Q: Should directory argument default to current directory when omitted? → A: Default to current directory when no argument given.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Serve Files from Directory (Priority: P1)

A developer wants to quickly share documentation or static files from their local machine. They run the docsvr command pointing to a directory, and can immediately access those files via a web browser on localhost.

**Why this priority**: This is the core functionality - the absolute minimum viable product. Without this, the tool has no purpose.

**Independent Test**: Can be fully tested by running `docsvr <directory>`, opening a browser to localhost, and requesting a file that exists in the directory. Delivers immediate value as a simple file server.

**Acceptance Scenarios**:

1. **Given** docsvr is started with a valid directory path, **When** user requests a file that exists in that directory via HTTP, **Then** the file contents are returned with HTTP 200 status
2. **Given** docsvr is serving files, **When** user requests a file using its relative path from the root directory, **Then** the correct file is served
3. **Given** docsvr is serving files from a directory with subdirectories, **When** user requests a file in a subdirectory, **Then** the file is served correctly

---

### User Story 2 - Correct Content Types (Priority: P2)

A developer serves various file types (HTML, CSS, JavaScript, images, JSON, markdown) and expects browsers to handle them correctly. The server automatically sets the appropriate Content-Type header based on file extension.

**Why this priority**: Without proper content types, browsers may not render files correctly (e.g., HTML shown as text, images not displaying). This is critical for usability but the server can technically function without it.

**Independent Test**: Can be tested by serving files with different extensions (.html, .css, .js, .json, .png, .jpg, .md, .txt) and verifying the Content-Type header in the HTTP response matches the expected MIME type.

**Acceptance Scenarios**:

1. **Given** docsvr is serving files, **When** user requests an HTML file, **Then** the response includes Content-Type: text/html
2. **Given** docsvr is serving files, **When** user requests a CSS file, **Then** the response includes Content-Type: text/css
3. **Given** docsvr is serving files, **When** user requests a JavaScript file, **Then** the response includes Content-Type: application/javascript
4. **Given** docsvr is serving files, **When** user requests a JSON file, **Then** the response includes Content-Type: application/json
5. **Given** docsvr is serving files, **When** user requests an image file (PNG, JPG, GIF, SVG), **Then** the response includes the appropriate image MIME type
6. **Given** docsvr is serving files, **When** user requests a file with unknown extension, **Then** the response includes Content-Type: application/octet-stream

---

### User Story 3 - Handle Missing Files (Priority: P3)

A user requests a file that doesn't exist in the served directory. The server returns a clear 404 Not Found error instead of crashing or hanging.

**Why this priority**: Error handling is important for reliability but not required for basic functionality. The tool can deliver value even with basic error handling.

**Independent Test**: Can be tested by requesting a non-existent file path and verifying a 404 response is returned with an appropriate error message.

**Acceptance Scenarios**:

1. **Given** docsvr is serving files, **When** user requests a file that doesn't exist, **Then** HTTP 404 status is returned
2. **Given** docsvr is serving files, **When** user requests a path that attempts directory traversal (e.g., ../../etc/passwd), **Then** the request is rejected with 404 or 403 status

---

### User Story 4 - Simple CLI Interface (Priority: P1)

A user runs docsvr from the command line. The interface is intuitive: `docsvr <directory>`. The server starts immediately and displays the listening address.

**Why this priority**: Usability is critical for adoption. If users can't figure out how to start the server, the tool is worthless.

**Independent Test**: Can be tested by running the command with various arguments (valid directory, invalid directory, no arguments, --help flag) and verifying expected behavior.

**Acceptance Scenarios**:

1. **Given** user has docsvr installed, **When** they run `docsvr <valid-directory>`, **Then** the server starts and displays "Listening on http://127.0.0.1:<port>"
2. **Given** user runs docsvr without arguments, **When** the command executes, **Then** the server starts serving files from the current working directory
3. **Given** user runs docsvr with a non-existent directory, **When** the command executes, **Then** an error message indicates the directory doesn't exist
4. **Given** user runs `docsvr --help`, **When** the command executes, **Then** usage information is displayed

---

### Edge Cases

- What happens when the directory path is valid but empty (no files)?
  - Server starts successfully but returns 404 for all file requests
- What happens when a file is being read while it's being modified?
  - Server reads whatever content is available at request time; concurrent modifications are outside scope
- What happens when the directory contains symbolic links?
  - Follow symbolic links to files within the served directory tree; reject links pointing outside the directory (security boundary)
- What happens when the port is already in use?
  - Server fails to start with clear error message indicating port conflict
- What happens when binary files are requested?
  - Binary files are served with raw byte content and appropriate Content-Type (e.g., application/octet-stream or specific type like image/png)
- What happens with very large files?
  - Files are served in their entirety; no streaming optimization required for MVP
- What happens when directory path contains spaces or special characters?
  - Standard filesystem path handling applies; paths are accepted as-is from command line
- What happens when user presses Ctrl+C?
  - Server shuts down gracefully

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept an optional directory path as a command-line argument, defaulting to the current working directory when omitted
- **FR-002**: System MUST validate that the provided path exists and is a directory
- **FR-003**: System MUST bind to localhost (127.0.0.1) only, not accepting external connections
- **FR-004**: System MUST listen for HTTP requests on a port (default 8080)
- **FR-005**: System MUST serve files from the specified directory when requested via HTTP GET
- **FR-006**: System MUST map URL paths to filesystem paths relative to the root directory
- **FR-007**: System MUST return HTTP 200 status for successful file requests
- **FR-008**: System MUST return HTTP 404 status for non-existent file requests
- **FR-009**: System MUST return file contents as raw bytes without modification
- **FR-010**: System MUST set Content-Type header based on file extension
- **FR-011**: System MUST prevent directory traversal attacks (e.g., requests for ../../etc/passwd)
- **FR-012**: System MUST display listening address and port when started
- **FR-013**: System MUST display usage information when run with --help flag
- **FR-014**: System MUST exit with error when directory argument is provided but invalid
- **FR-015**: System MUST use Rust edition 2024 with zero external dependencies
- **FR-016**: System MUST return HTTP 405 Method Not Allowed for non-GET requests

### Key Entities

- **File Request**: Represents an HTTP GET request for a file, containing the requested URL path
- **File Response**: Represents an HTTP response containing file contents, status code, and Content-Type header
- **Served Directory**: The root directory from which files are served, specified via command-line argument

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can start serving files with a single command in under 5 seconds
- **SC-002**: Common file types (HTML, CSS, JS, JSON, images, text) are served with correct Content-Type headers
- **SC-003**: Server successfully serves files of any size up to available system memory
- **SC-004**: Server responds to file requests in under 100ms for files under 1MB on typical hardware
- **SC-005**: All directory traversal attack attempts are blocked (0% success rate in security testing)
- **SC-006**: Server fails to start with a clear error message when the default port is occupied
- **SC-007**: Binary compiles with zero external dependencies (cargo tree shows only std library)

## Assumptions

- Default port is 8080; if occupied, server will fail to start (port auto-discovery is out of scope for MVP)
- Standard HTTP/1.1 protocol is sufficient; no HTTP/2 or HTTP/3 support needed
- No access logging required for MVP
- No index.html auto-serving for directory requests (requesting "/" returns 404 unless index.html is explicitly requested)
- No CORS headers needed since server is localhost-only
- File MIME type detection based on extension only (not content sniffing)
- UTF-8 encoding assumed for text files
- No caching headers or ETag support needed for MVP
- Server runs as a foreground process; daemonization is out of scope
- No authentication or authorization needed (localhost-only assumption)
- No TLS/HTTPS needed (localhost-only assumption)
