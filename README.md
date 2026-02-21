# docsvr

A local HTTP server that serves Markdown files as HTML.

Point it at a directory of `.md` files and browse them in your web browser — Markdown is converted to HTML on the fly.

## Usage

```sh
docsvr [directory]
```

Opens an HTTP server on `localhost` serving the contents of `directory` (defaults to the current directory). Navigate to any `.md` file path in your browser to see it rendered as HTML.

## Building

```sh
cargo build --release
```

## Roadmap

1. ~~**Static file server**~~ ✅ — Serve raw files from a directory over HTTP on localhost. CLI takes a directory path argument.
2. ~~**Markdown-to-HTML rendering**~~ ✅ — When a `.md` file is requested, convert it to HTML on the fly and serve a complete HTML page.
3. ~~**Directory listing**~~ ✅ — When a directory is requested, return an HTML page listing its contents with navigable links.
4. ~~**HTML styling**~~ ✅ — Apply a clean, readable stylesheet to rendered Markdown pages and directory listings. CSS served from a dedicated GUID-based URI for browser caching.
5. ~~**Markdown tables**~~ ✅ — GFM-style table support with column alignment.
6. **Nested block elements** — Support block-level constructs inside blockquotes (fenced code blocks, lists, nested blockquotes).
7. ~~**Syntax highlighting**~~ ✅ — Language-aware syntax highlighting for fenced code blocks using embedded highlight.js.

## Design Goals

- **Zero dependencies** — built entirely on the Rust standard library
- **Minimal resource usage** — low memory footprint, fast response times
- **Local only** — serves on localhost over plain HTTP, no TLS
