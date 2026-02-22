# Data Model: Asset Serving via Dedicated URI

## Entities

### Asset Namespace

- **ASSET_PREFIX**: A compile-time `const &str` — 32-character lowercase hex string (UUID v4 without dashes)
- **Location**: Defined in `server.rs`, passed to `wrap_html_page` and `render_listing` via function parameters
- **Purpose**: Reserved URL path segment that the server intercepts before filesystem resolution

### Built-in Asset

| Name | Content Source | MIME Type | Served At |
|------|---------------|-----------|-----------|
| `style.css` | `const CSS: &str` in `markdown.rs` | `text/css` | `/<ASSET_PREFIX>/style.css` |

Future assets (e.g., `highlight.js`) follow the same pattern: add a const and a match arm.

## Relationships

- `ASSET_PREFIX` is used by:
  - `handle_connection` in `server.rs` — intercepts matching requests
  - `serve_file` / `serve_directory` in `server.rs` — passes prefix to `wrap_html_page`
  - `wrap_html_page` in `markdown.rs` — generates `<link>` tag with prefix

- `CSS` constant remains in `markdown.rs` but is no longer injected inline; it's served by `server.rs` when the asset URI is requested.

## State Transitions

None — assets are static compile-time constants with no state changes.
