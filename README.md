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
greymd [options] [directory]
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

## Themes

Bundled themes are installed to `<prefix>/share/greymd/themes/`. Use `--theme` to activate one:

```sh
greymd --theme catppuccin-mocha
greymd --list-themes
```

Available themes: `default`, `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`, `catppuccin-mocha`, `tokyo-night`.

Theme files override `~/.config/greymd/` only for the assets they contain. For example, if a theme only has a `css` file, your `~/.config/greymd/js` still applies.

If a theme is not found (e.g. when installed via `cargo install`, which only installs the binary), greymd prints a warning and falls back to the default appearance.

### Release archives

Release `.tar.gz` and `.zip` archives include themes in the standard layout:

```
greymd-<version>-<target>/
├── bin/greymd
└── share/greymd/themes/
    ├── default/css
    ├── catppuccin-latte/css
    ├── catppuccin-frappe/css
    ├── catppuccin-macchiato/css
    ├── catppuccin-mocha/css
    └── tokyo-night/css
```

Extract to any prefix (e.g. `/usr/local/` or `~/.local/`) and `--theme` will find them automatically.

## Math Rendering

greymd supports optional LaTeX math rendering via a compile-time feature flag:

```sh
cargo build --release --features math
```

When enabled, LaTeX expressions are converted server-side to MathML:

- **Inline math**: `$x^2$` renders inline within text
- **Display math**: `$$\int_0^1 f(x)\,dx$$` renders as a centered block

Without the feature, `$...$` and `$$...$$` pass through unchanged.

Pre-built release archives are available in both standard and `-math` variants.

See `examples/math-demo.md` for a full demonstration.

## Building

Requires Rust 1.85+ (edition 2024).

```sh
cargo build --release                  # standard build
cargo build --release --features math  # with math rendering
```

## Design Goals

- **Reasonable markdown support** — headings, lists, tables, code blocks, inline formatting, and more
- **Zero runtime dependencies** — built entirely on the Rust standard library
- **Minimal resource usage** — low memory footprint, fast response times
- **Local only** — serves on `127.0.0.1` over plain HTTP, no TLS
