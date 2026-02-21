# Quickstart: Syntax Highlighting

## What Changes

Before: Fenced code blocks render as plain monospace text.
After: Fenced code blocks have syntax-colored keywords, strings, comments, etc.

## Integration Scenarios

### Scenario 1: Page with Highlighted Code

1. User requests `http://localhost:8080/readme.md` (contains ` ```rust ` block)
2. Server renders markdown to HTML
3. `wrap_html_page` adds highlight.js `<link>` and `<script>` tags
4. Browser loads page, fetches `/<guid>/highlight.min.js` and `/<guid>/highlight-github.css`
5. `hljs.highlightAll()` runs, finds `<code class="language-rust">`, applies highlighting

### Scenario 2: Asset Requests

1. Browser requests `/<guid>/highlight.min.js` → 200 with `application/javascript`
2. Browser requests `/<guid>/highlight-github.css` → 200 with `text/css`
3. Both cached by browser for subsequent pages

### Scenario 3: Directory Listing

1. User requests a directory listing
2. Page includes highlight.js tags (no-op since no code blocks in listings)
3. No errors, no visual impact

## Verification

```sh
cargo run -- /path/to/docs

# Check HTML includes highlight.js tags
curl -s http://localhost:8080/readme.md | grep 'highlight'

# Verify JS asset
curl -s -o /dev/null -w '%{http_code}' http://localhost:8080/<guid>/highlight.min.js
# 200

# Verify CSS asset
curl -s -o /dev/null -w '%{http_code}' http://localhost:8080/<guid>/highlight-github.css
# 200
```
