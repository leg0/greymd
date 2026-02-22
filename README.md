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

## Building

```sh
cargo build --release
```

## Design Goals

- **Reasonable portion of MD support** — most useful bits of markdown supported
- **Zero dependencies** — built entirely on the Rust standard library
- **Minimal resource usage** — low memory footprint, fast response times
- **Local only** — serves on localhost over plain HTTP, no TLS
