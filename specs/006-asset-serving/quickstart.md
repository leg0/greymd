# Quickstart: Asset Serving via Dedicated URI

## What Changes

Before: Every HTML page contains `<style>...</style>` with the full CSS (~1.5KB) inline.
After: Every HTML page contains `<link rel="stylesheet" href="/<guid>/style.css">` and the CSS is served once from a dedicated URI.

## Integration Scenarios

### Scenario 1: Normal Markdown Page

1. User requests `http://localhost:8080/readme.md`
2. Server resolves path, reads file, renders markdown to HTML
3. `wrap_html_page("readme", body, ASSET_PREFIX)` generates HTML with `<link>` tag
4. Browser fetches `/<ASSET_PREFIX>/style.css` — server intercepts, returns CSS
5. Subsequent page loads use cached CSS

### Scenario 2: Directory Listing

1. User requests `http://localhost:8080/docs/`
2. Server resolves to directory, calls `serve_directory`
3. `render_listing` calls `wrap_html_page` with asset prefix
4. HTML contains same `<link>` tag — CSS shared across all pages

### Scenario 3: Asset Request

1. Browser requests `http://localhost:8080/<ASSET_PREFIX>/style.css`
2. `handle_connection` detects GUID prefix before `resolve_path`
3. Matches asset name `style.css` → returns CSS content with `Content-Type: text/css`
4. Unknown asset names under prefix → 404

### Scenario 4: No Interference with User Content

1. User has files/directories in served root
2. None collide with GUID prefix (32 hex chars)
3. All normal file/directory requests routed through `resolve_path` as before

## Verification

```sh
# Start server
cargo run -- /path/to/docs

# Verify HTML contains <link> not <style>
curl -s http://localhost:8080/readme.md | grep '<link.*style.css'

# Verify asset is served
curl -s http://localhost:8080/<ASSET_PREFIX>/style.css | head -5

# Verify unknown asset returns 404
curl -s -o /dev/null -w '%{http_code}' http://localhost:8080/<ASSET_PREFIX>/nope.js
# Output: 404
```
