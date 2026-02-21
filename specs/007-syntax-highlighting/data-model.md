# Data Model: Syntax Highlighting

## Entities

### Built-in Assets (additions)

| Name | Content Source | MIME Type | Served At |
|------|---------------|-----------|-----------|
| `highlight.min.js` | `pub const HLJS_JS` in `markdown.rs` (via `include_str!`) | `application/javascript` | `/<ASSET_PREFIX>/highlight.min.js` |
| `highlight-github.css` | `pub const HLJS_CSS` in `markdown.rs` (via `include_str!`) | `text/css` | `/<ASSET_PREFIX>/highlight-github.css` |

### Files Added

| File | Purpose |
|------|---------|
| `src/assets/highlight.min.js` | highlight.js common bundle, minified |
| `src/assets/highlight-github.css` | GitHub theme CSS for highlight.js |

## Relationships

- `HLJS_JS` and `HLJS_CSS` are defined in `markdown.rs` via `include_str!("assets/highlight.min.js")` etc.
- `server.rs` serves them from the GUID asset namespace (2 new match arms)
- `wrap_html_page` in `markdown.rs` emits `<link>` and `<script>` tags referencing these assets
