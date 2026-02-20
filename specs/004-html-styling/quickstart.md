# Quickstart: HTML Styling

## What Changes

The `wrap_html_page` function in `src/markdown.rs` is the single modification point. It gains:
1. A viewport meta tag in `<head>`
2. A `<style>` block containing the full CSS stylesheet

Both markdown pages and directory listings automatically inherit styling since they both call `wrap_html_page`.

## Verification

1. Build and run:
   ```sh
   cargo build && cargo run -- /path/to/markdown/files
   ```

2. Open browser to `http://127.0.0.1:8080/` and verify:
   - Directory listing has styled entries with spacing
   - Click a `.md` file — headings, code blocks, lists are styled
   - Content is centered with max-width on wide screens
   - View page source — `<style>` block present, no `<link>` tags

3. Run tests:
   ```sh
   cargo test
   ```
   All tests should pass. Tests verify `<style>` presence and viewport meta tag.

## Key Design Decisions

- **Single const**: CSS lives as `const CSS: &str` — no runtime allocation
- **System fonts**: No web font loading, no network requests
- **Light theme only**: No dark mode (explicit spec decision)
- **48em max-width**: ~768px optimal reading width
