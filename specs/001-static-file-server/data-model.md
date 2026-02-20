# Data Model: Static File Server

This feature has no persistent data model. All data flows are stateless request-response cycles operating on the filesystem.

## Entities

### HttpRequest

Parsed from raw TCP stream. Read-only, lives for duration of request handling.

| Field | Type | Description |
|-------|------|-------------|
| method | &str | HTTP method (GET, POST, etc.) |
| path | &str | URL path (e.g., "/docs/readme.md") |
| version | &str | HTTP version (e.g., "HTTP/1.1") |

### HttpResponse

Constructed per-request and written to TCP stream.

| Field | Type | Description |
|-------|------|-------------|
| status_code | u16 | HTTP status (200, 404, 405) |
| status_text | &str | Status reason phrase |
| content_type | &str | MIME type for Content-Type header |
| body | Vec\<u8\> | Response body (file contents or error message) |

### ServedDirectory

Configured once at startup from CLI args.

| Field | Type | Description |
|-------|------|-------------|
| root_path | PathBuf | Canonical absolute path to served directory |
| port | u16 | Listening port (default 8080) |

## Relationships

- Each incoming TCP connection produces one **HttpRequest**
- Each **HttpRequest** produces one **HttpResponse**
- **ServedDirectory.root_path** + **HttpRequest.path** → filesystem lookup → file contents or 404

## State Transitions

None. The server is stateless — no sessions, no caching, no persistent connections.
