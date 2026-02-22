# Research: Asset Serving via Dedicated URI

## Decision: GUID Format

- **Decision**: Use a hardcoded 32-character lowercase hex string (UUID v4 without dashes) as the asset prefix constant.
- **Rationale**: 32 hex chars are virtually collision-proof with real directory names. No dashes keeps it a single URL path segment. Compile-time constant means zero runtime overhead.
- **Alternatives considered**:
  - Runtime-generated UUID: Rejected — changes on every restart, breaks bookmarks/caching, adds complexity.
  - Short prefix (e.g., `_assets`): Rejected — too likely to collide with user directory names.
  - Hash of binary version: Rejected — unnecessary complexity for a local dev server.

## Decision: Interception Point

- **Decision**: Check for GUID prefix in `handle_connection` before calling `resolve_path`.
- **Rationale**: Keeps asset serving completely separate from filesystem resolution. No path traversal risk since asset names are matched against a hardcoded list, not resolved on disk.
- **Alternatives considered**:
  - New middleware layer: Rejected — over-engineering for a simple prefix check.
  - Separate listener/port: Rejected — unnecessary complexity, breaks single-origin caching.

## Decision: wrap_html_page Signature

- **Decision**: Add `asset_prefix: &str` parameter to `wrap_html_page` to generate `<link>` tags.
- **Rationale**: Keeps the function pure and testable. Callers pass the prefix; the function doesn't depend on global state.
- **Alternatives considered**:
  - Global/static access to ASSET_PREFIX from markdown.rs: Rejected — couples modules unnecessarily.
  - Keep inline CSS and also serve at URI: Rejected — defeats the purpose (no size reduction, confusing dual delivery).

## Decision: Asset Response Headers

- **Decision**: Return `Content-Type` based on file extension using existing `mime.rs` logic. No `Cache-Control` headers initially.
- **Rationale**: Browser default caching is sufficient for a local dev server. The existing MIME mapping already handles `.css` → `text/css`.
- **Alternatives considered**:
  - Aggressive cache headers: Deferred — not needed for local dev; can add later if asset versioning is implemented.
