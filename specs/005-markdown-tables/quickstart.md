# Quickstart: Markdown Tables

## What Changes

All changes are in `src/markdown.rs`:
1. Add `Alignment` enum
2. Add helper functions: `is_table_separator`, `parse_alignment`, `split_table_cells`, `render_table`
3. Add table detection block in `render_body`'s main loop (before paragraph fallthrough)
4. Add table CSS rules to `const CSS`

## Verification

1. Create a test markdown file:
   ```markdown
   # Table Test

   | Name | Age | City |
   |------|----:|:----:|
   | Alice | 30 | NYC |
   | Bob | 25 | LA |
   | Charlie | 35 | Chicago |

   Some text after the table.
   ```

2. Build and serve:
   ```sh
   cargo build && cargo run -- /path/to/test/
   ```

3. Open browser and verify:
   - Table renders with 3 columns, 3 data rows
   - "Age" column is right-aligned
   - "City" column is center-aligned
   - Table has borders and padding
   - Header row is visually distinct

4. Run tests:
   ```sh
   cargo test
   ```

## Key Design Decisions

- Table detection uses one-line lookahead (peek at next line for separator)
- `split_table_cells` tracks backtick state to protect pipes in inline code
- Alignment stored as `Vec<Alignment>`, applied as `style` attribute
- Header-only tables (no data rows) are valid per GFM
- All changes in one file (`markdown.rs`)
