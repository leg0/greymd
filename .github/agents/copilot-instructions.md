# docsvr Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-02-20

## Active Technologies
- Rust 2024 (edition 2024, rustc 1.91.1) + None (zero external crates) (002-markdown-rendering)
- Filesystem (read-only, `.md` files) (002-markdown-rendering)
- Filesystem (read-only directory listing via `std::fs::read_dir`) (003-directory-listing)
- Rust 2024 edition (rustc 1.91.1) + None (zero external crates) (004-html-styling)
- N/A (purely presentational) (004-html-styling)
- Rust 2024 edition (rustc 1.91.1) + Zero external crates (std only) (006-asset-serving)
- N/A (assets compiled into binary as `const &str`) (006-asset-serving)
- Rust 2024 edition (rustc 1.91.1) + Zero Rust crate deps; highlight.js embedded as static content (007-syntax-highlighting)
- Rust edition 2024, rustc 1.85+ + None (zero runtime crate dependencies per constitution) (009-config-file)
- Filesystem — reads `.greymd` config files and custom CSS/JS files (009-config-file)
- Filesystem — reads well-known custom CSS/JS files (009-config-file)
- Rust edition 2024 (rustc 1.85+) + None (std only); miniz_oxide for build-time gzip (010-custom-themes)
- Filesystem (`<prefix>/share/greymd/themes/`) (010-custom-themes)
- Rust edition 2024 (rustc 1.85+) + None (std only); `latex2mathml` optional behind `math` feature; `miniz_oxide` build-time only (011-math-rendering)
- N/A (server-side conversion, no storage) (011-math-rendering)
- Rust 1.85+ (edition 2024) + None (std only) (013-path-auto-linking)
- Rust, edition 2024 (Cargo.toml) + None (stdlib only; `latex2mathml` optional for math feature) (014-auto-open-browser)

- Rust 2024 edition (rustc 1.85+) + None (zero external crates per constitution) (001-static-file-server)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 2024 edition (rustc 1.85+): Follow standard conventions

## Recent Changes
- 014-auto-open-browser: Added Rust, edition 2024 (Cargo.toml) + None (stdlib only; `latex2mathml` optional for math feature)
- 013-path-auto-linking: Added Rust 1.85+ (edition 2024) + None (std only)
- 013-path-auto-linking: Added Rust 1.85+ (edition 2024) + None (std only)


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
