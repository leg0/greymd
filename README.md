# greymd

A local HTTP server that serves Markdown files as HTML.

Point it at a directory of `.md` files and browse them in your web browser — Markdown is converted to HTML on the fly.

## Why greymd?

Most markdown previewers pull in hundreds of npm packages, require Node.js or Python runtimes, or phone home to cloud services. greymd is different:

- **Single binary, zero runtime dependencies.** Download one file, run it. No Node, no Python, no package manager. Built entirely on the Rust standard library — nothing to install, nothing to update, nothing to break.
- **Instant startup.** Launches in milliseconds. No bundling step, no dev server warmup, no "compiling dependencies" progress bar.
- **Tiny footprint.** The entire binary — including the HTTP server, markdown parser, syntax highlighter, and stylesheets — fits in a few hundred kilobytes. It uses almost no memory at runtime.
- **Works offline.** Everything is embedded in the binary. No CDN requests, no font downloads, no analytics scripts. Open your laptop on a plane and it just works.
- **Serves a directory, not a file.** Point it at a folder of `.md` files and browse them like a wiki — with directory listings, clickable navigation, and automatic index pages.

If you want a simple, fast, self-contained way to read markdown in a browser, greymd is all you need.

## Usage

```sh
greymd [directory]
```

Opens an HTTP server on `localhost` serving the contents of `directory` (defaults to the current directory). Navigate to any `.md` file path in your browser to see it rendered as HTML.

The server tries port 8080 first, then falls back to a random available port if 8080 is busy. The actual address is printed at startup.

## Features

### Markdown

- **Headings** (`#` through `######`) with auto-generated IDs
- **Paragraphs**, **bold**, *italic*, ***bold+italic***, ~~strikethrough~~
- `Inline code` and fenced code blocks (` ``` `) with language tags
- Indented code blocks (4+ spaces)
- Unordered lists (`-`, `*`, `+`), ordered lists (`1.`), and nested lists
- Task lists (`- [ ]`, `- [x]`)
- Blockquotes (`>`)
- [Links](url) and ![images](url), with auto-linking of bare URLs
- GFM-style tables with column alignment
- Horizontal rules (`---`, `***`, `___`)

### Browsing

- **Directory listings** — sorted directories-first, then files, with icons (📁 📄)
- **Auto-serve** — a directory with a single `.md` file serves it directly; `index.md` is preferred when multiple files exist
- **Table of contents** — auto-generated sidebar navigation when a page has two or more headings
- **Syntax highlighting** — built-in highlight.js for fenced code blocks
- **Copy button** — one-click clipboard copy on every code block

### Serving

- Serves non-markdown files raw with correct MIME types (HTML, CSS, JS, JSON, images, PDF, SVG, WASM, and more)
- Path traversal protection — requests that escape the served directory are blocked
- Percent-encoded URLs handled correctly
- Concurrent connections via thread-per-request

## Customization

Place files at these well-known paths to customize greymd's appearance:

- **`~/.config/greymd/css`** — Custom stylesheet appended after the built-in CSS. Use this to override colors, fonts, or layout.
- **`~/.config/greymd/js`** — Custom JavaScript that replaces the built-in highlight.js. Use this to bring your own syntax highlighter or add custom behavior.

File contents are re-read on every request, so you can add, edit, or remove custom files and refresh without restarting.

## Building

Requires Rust 1.85+ (edition 2024).

```sh
cargo build --release
```

## Design Goals

- **Reasonable markdown support** — headings, lists, tables, code blocks, inline formatting, and more
- **Zero runtime dependencies** — built entirely on the Rust standard library
- **Minimal resource usage** — low memory footprint, fast response times
- **Local only** — serves on `127.0.0.1` over plain HTTP, no TLS
