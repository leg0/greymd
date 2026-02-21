# Research: Syntax Highlighting for Code Blocks

## Decision: highlight.js Common Bundle

- **Decision**: Use the highlight.js common bundle (minified, ~40KB) with GitHub theme CSS
- **Rationale**: Common bundle covers ~40 popular languages (Rust, Python, JS, Go, C/C++, Java, SQL, Bash, TypeScript, etc.). Sufficient for a documentation server. GitHub theme matches existing page styling.
- **Alternatives considered**:
  - Full bundle (~300KB): Too large for binary embedding; most languages unused
  - Prism.js: Requires per-language module bundling; more setup complexity
  - Server-side highlighting: Would require a tokenizer per language; massive effort for zero-dep constraint

## Decision: Embedding via include_str!

- **Decision**: Save JS and CSS files under `src/assets/` and use `include_str!` to embed at compile time
- **Rationale**: Keeps source files clean (no 40KB string literals in .rs files). `include_str!` is a standard Rust macro that embeds file content at compile time — no runtime cost, no deps.
- **Alternatives considered**:
  - Raw string literal in source: Works but makes markdown.rs unreadable with 40KB of minified JS
  - Build script to download: Adds build complexity and network dependency

## Decision: Script Placement

- **Decision**: Place `<script>` tags at end of `<body>` with inline `hljs.highlightAll()`
- **Rationale**: Ensures DOM is fully loaded before highlighting runs. Standard web practice. Inline `highlightAll()` call avoids a separate asset for a one-liner.
- **Alternatives considered**:
  - `<script>` in `<head>` with `defer`: Works but less compatible with older browsers
  - DOMContentLoaded listener: Unnecessary if script is at end of body

## Decision: Load on All Pages

- **Decision**: Include highlight.js on all pages (including directory listings)
- **Rationale**: Simplicity. The assets are cached by the browser after first load. Conditional loading would complicate `wrap_html_page` for negligible benefit.
- **Alternatives considered**:
  - Conditional loading only on pages with code blocks: Would require analyzing rendered HTML or passing a flag through the call chain
