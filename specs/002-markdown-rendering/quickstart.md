# Quickstart: Markdown-to-HTML Rendering

**Date**: 2026-02-20
**Feature**: 002-markdown-rendering

## Test Scenarios

### Scenario 1: Basic Heading Rendering
Create a `.md` file with `# Hello World`, request it, verify response contains `<h1>Hello World</h1>` wrapped in HTML5 structure with `<title>Hello World</title>`.

### Scenario 2: Paragraph and Inline Formatting
Create a `.md` file with paragraphs containing `**bold**`, `*italic*`, `***both***`, and `` `code` ``. Request it, verify `<strong>`, `<em>`, `<strong><em>`, and `<code>` elements appear correctly.

### Scenario 3: Links and Images
Create a `.md` file with `[text](url)` and `![alt](img.png)`. Verify `<a href="url">text</a>` and `<img src="img.png" alt="alt">` in output.

### Scenario 4: Code Blocks
Create a `.md` file with fenced code block (` ```rust ` ... ` ``` `). Verify output contains `<pre><code class="language-rust">` with HTML-escaped content inside.

### Scenario 5: Lists (Ordered, Unordered, Nested)
Create a `.md` file with:
- Unordered list items → verify `<ul><li>`
- Ordered list items → verify `<ol><li>`
- Nested list (2-3 levels) → verify nested `<ul>`/`<ol>` elements

### Scenario 6: Non-Markdown Files Unaffected
Request `.html`, `.txt`, `.css` files. Verify they are served as raw content with original Content-Type (no HTML wrapping).

### Scenario 7: Edge Cases
- Empty `.md` file → valid HTML page with empty `<body>`
- File with `<script>alert(1)</script>` → characters escaped to `&lt;script&gt;`
- File with CRLF line endings → renders identically to LF version
- File with no headings → `<title>` uses filename (e.g., `notes` from `notes.md`)

### Scenario 8: Blockquotes and Horizontal Rules
Create a `.md` file with `> quoted text` and `---`. Verify `<blockquote><p>quoted text</p></blockquote>` and `<hr>` in output.

### Scenario 9: HTML Escaping in All Contexts
Verify `<`, `>`, `&`, `"` are escaped in: paragraph text, headings, list items, blockquotes, link text, image alt text. Only Markdown-generated HTML tags should appear unescaped.
