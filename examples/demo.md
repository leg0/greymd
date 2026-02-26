# greymd Feature Demo

This page showcases every Markdown feature supported by **greymd**.

---

## Headings

### Third-level heading
#### Fourth-level heading
##### Fifth-level heading
###### Sixth-level heading

---

## Inline Formatting

This is **bold text**, this is *italic text*, and this is ***bold and italic***.

This is ~~strikethrough~~ text.

You can also use inline `code` within a sentence.

Double-backtick spans let you include literal backticks: `` `code` `` renders as `code` with backticks.

Special characters are escaped: <div>, &amp;, and "quotes" work safely.

---

## Links and Images

Visit [the Rust homepage](https://www.rust-lang.org) for more info.

Bare URLs are auto-linked: https://www.rust-lang.org

![Rust logo](https://www.rust-lang.org/logos/rust-logo-128x128.png)

---

## Blockquotes

> "Simplicity is the ultimate sophistication."
>
> — Leonardo da Vinci

> Blockquotes can contain **bold**, *italic*, and `code`.

---

## Lists

### Unordered

- Dash item
- Second dash item
  - Nested item A
  - Nested item B
    - Deeply nested

* Asterisk item
* Another asterisk item

+ Plus item
+ Another plus item

### Ordered

1. Clone the repository
2. Build the project
3. Run the server
   1. Pass a directory path
   2. Open the browser

### Task List

- [x] Implement markdown rendering
- [x] Add syntax highlighting
- [ ] Add dark mode
- [ ] ~~Add live reload~~

---

## Code Blocks

### Fenced code block (with language hint for syntax highlighting)

```rust
fn main() {
    let port = 8080;
    println!("Listening on http://localhost:{port}");
}
```

```sh
cargo build && cargo run -- ./docs
```

### Indented code block (4-space indent, no syntax highlighting)

    This is an indented code block.
    It preserves    spacing and <html> entities.

---

## Tables

### Basic Table

| Feature | Status |
|---------|--------|
| Static file serving | Done |
| Markdown rendering | Done |
| Directory listing | Done |
| HTML styling | Done |
| Table support | Done |

### Column Alignment

| Left-aligned | Centered | Right-aligned |
|:-------------|:--------:|--------------:|
| Apples       | 12       | $1.20         |
| Bananas      | 5        | $0.50         |
| Cherries     | 48       | $3.00         |
| **Total**    | **65**   | **$4.70**     |

### Table with Inline Formatting

| Syntax | Output | Notes |
|--------|--------|-------|
| `**bold**` | **bold** | Double asterisks |
| `*italic*` | *italic* | Single asterisks |
| `***both***` | ***both*** | Triple asterisks |
| `` `code` `` | `code` | Backticks |

---

## Horizontal Rules

All three styles render the same way:

---

***

___

## Putting It All Together

> **Tip:** Run greymd with `greymd /path/to/your/markdown/files`
> and then open `http://localhost:8080` in your browser.

1. All rendering happens **on the fly** — no build step needed
2. Navigate directories via the *auto-generated listing page*
3. Zero external dependencies: just `std`

| Component | Lines of Code | Module |
|:----------|:-------------:|-------:|
| HTTP parser | ~230 | `http.rs` |
| Markdown renderer | ~1440 | `markdown.rs` |
| Path resolver | ~170 | `path.rs` |
| Directory listing | ~190 | `listing.rs` |
| Server | ~620 | `server.rs` |

---

## Path Auto-Linking

Paths to `.md` files and directories are automatically converted to clickable links.

### These should be linked

- See math-demo.md for a math rendering demo.
- The project ../README.md is at the repo root.
- Check out the themes/ subdirectory for theme examples.
- This file is ./demo.md relative to itself.

### These should NOT be linked

- The file highlight.js provides syntax highlighting.
- Edit src/main.rs to change the entry point.
- Config is in Cargo.toml at the root.
- Fractions like 1/2 and phrases like and/or are left alone.
- Prices like $1.20 are not affected either.
- Tokens like hello/world without an extension stay plain.

### In code spans

- `math-demo.md` — path-only code span becomes clickable.
- `main.rs` — non-`.md` code span stays plain.
- `cargo run math-demo.md` — code span with extra text stays plain.
