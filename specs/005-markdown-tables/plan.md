# Implementation Plan: Markdown Tables

**Branch**: `005-markdown-tables` | **Date**: 2026-02-20 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-markdown-tables/spec.md`

## Summary

Add GFM-style pipe table parsing to the existing single-pass markdown renderer. Tables are detected as a new block type in `render_body`'s main loop: when a line containing pipes is followed by a valid separator row, we consume and render the table block. Column alignment is parsed from the separator row. Inline formatting is applied to cell content via the existing `render_inline`. CSS for tables is added to the existing `const CSS` string.

## Technical Context

**Language/Version**: Rust 2024 edition (rustc 1.91.1)
**Primary Dependencies**: None (zero external crates)
**Storage**: N/A
**Testing**: `cargo test` (103 tests currently passing)
**Target Platform**: Linux (local dev server)
**Project Type**: Single binary crate
**Performance Goals**: No measurable impact — tables are parsed in the existing single-pass loop
**Constraints**: Zero external dependencies; must integrate with existing line-oriented parser
**Scale/Scope**: Add table detection to `render_body`, helper functions for parsing, CSS rules

## Constitution Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | ✅ PASS | Tests for table parsing, alignment, edge cases written before implementation |
| II. Minimal Dependencies | ✅ PASS | Pure Rust, no crates. Uses existing `render_inline` and `escape_html` |
| III. Minimum Resource Usage | ✅ PASS | No intermediate AST. Tables parsed and rendered in-place during single pass |

## Project Structure

### Source Code

```text
src/
├── markdown.rs          # Table parsing + rendering added here (render_body, helpers, CSS, tests)
├── main.rs              # No changes
├── http.rs              # No changes
├── listing.rs           # No changes
├── mime.rs              # No changes
├── path.rs              # No changes
└── server.rs            # No changes
```

**Structure Decision**: All changes in `markdown.rs`. Table parsing is a new block type in the existing `render_body` function, with helper functions for separator validation, alignment parsing, and cell splitting.

## Design

### Parser Integration

The table block is detected in `render_body`'s main loop by lookahead:
1. When the current line contains `|`, peek at the next line
2. If the next line is a valid separator row (dashes, optional colons, pipes), begin table parsing
3. Consume the header row, separator row, and consecutive data rows
4. Emit `<table><thead>...<tbody>...</table>` HTML

### Key Functions (new)

- `is_table_separator(line: &str) -> bool` — validates separator row pattern
- `parse_alignment(separator: &str) -> Vec<Alignment>` — extracts column alignments from separator
- `split_table_cells(line: &str) -> Vec<&str>` — splits row into cells, respecting backtick-delimited code
- `render_table(header: &str, separator: &str, data_rows: &[&str]) -> String` — produces full `<table>` HTML

### Alignment Enum

```
enum Alignment { Left, Center, Right }
```

Applied as `style="text-align: ..."` on `<th>` and `<td>`.

### Cell Splitting (FR-013)

Pipe characters inside backtick code spans must not split cells. The `split_table_cells` function tracks backtick state while scanning for `|` delimiters.

### CSS Addition

Add to existing `const CSS`:
```css
table { border-collapse: collapse; width: 100%; margin: 1em 0; }
th, td { border: 1px solid #e1e4e8; padding: 0.5em 0.75em; }
th { background: #f6f8fa; font-weight: 600; }
```

### Edge Case Handling

| Edge Case | Behavior |
|-----------|----------|
| No separator row | Not a table; processed as paragraph text |
| Header-only (no data rows) | Valid table with `<thead>`, empty `<tbody>` |
| Fewer columns in data row | Pad with empty `<td>` |
| More columns in data row | Truncate to header count |
| Empty cell (`\|\|`) | Render empty `<td>` |
| Pipe in inline code | Not treated as delimiter |
| Leading/trailing pipes optional | Both `\| a \| b \|` and `a \| b` accepted |

## Complexity Tracking

No constitution violations. No complexity justifications needed.
