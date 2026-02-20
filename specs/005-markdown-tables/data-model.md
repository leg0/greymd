# Data Model: Markdown Tables

## Entities

### Alignment

Represents text alignment for a table column.

- **Variants**: Left, Center, Right
- **Derived from**: Separator row colon patterns (`:---` → Left, `:---:` → Center, `---:` → Right, `---` → Left)
- **Applied to**: `style` attribute on `<th>` and `<td>` elements

### Table Block

A table block recognized during parsing. Not stored as a persistent entity — consumed and rendered in-place.

- **Header cells**: Vec of strings (trimmed cell content from header row)
- **Alignments**: Vec of Alignment (one per column, from separator row)
- **Data rows**: Vec of Vec of strings (trimmed cell content per row)
- **Column count**: Determined by header row cell count

### Rendering Rules

- Header cells → `<th>` with optional `style="text-align: ..."` inside `<thead><tr>`
- Data cells → `<td>` with optional `style="text-align: ..."` inside `<tbody><tr>`
- Missing cells (row shorter than header) → empty `<td>` with alignment
- Excess cells (row longer than header) → truncated
- Cell content → processed through `render_inline` for bold/italic/code/links
- All text content → HTML-escaped via existing `escape_html`
