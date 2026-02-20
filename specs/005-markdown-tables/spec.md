# Feature Specification: Markdown Tables

**Feature Branch**: `005-markdown-tables`  
**Created**: 2026-02-20  
**Status**: Draft  
**Input**: User description: "Extended Markdown support for table syntax"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Render Basic Tables (Priority: P1)

A user writes a standard pipe-delimited markdown table in their `.md` file and sees it rendered as a properly formatted HTML table with headers, rows, and cell content.

**Why this priority**: Tables are the most commonly requested extended markdown feature. Documentation frequently needs tabular data presentation.

**Independent Test**: Can be fully tested by creating a `.md` file with a pipe-delimited table and verifying the rendered HTML contains `<table>`, `<thead>`, `<tbody>`, `<th>`, and `<td>` elements with correct content.

**Acceptance Scenarios**:

1. **Given** a markdown file containing a table with a header row, separator row, and data rows, **When** the user views it in a browser, **Then** they see a formatted HTML table with distinct headers and data cells.
2. **Given** a table with 3 columns and 5 data rows, **When** rendered, **Then** all 15 data cells and 3 header cells are present with correct content.
3. **Given** a table cell containing inline formatting (bold, italic, code, links), **When** rendered, **Then** the inline formatting is preserved within the table cell.

---

### User Story 2 - Column Alignment (Priority: P2)

A user specifies column alignment in the separator row using colons (`:---`, `:---:`, `---:`) and sees the corresponding columns left-aligned, center-aligned, or right-aligned.

**Why this priority**: Alignment is a standard part of the table syntax and expected by users familiar with GitHub Flavored Markdown.

**Independent Test**: Can be tested by creating a table with different alignment markers and verifying the rendered HTML cells have the appropriate alignment attributes.

**Acceptance Scenarios**:

1. **Given** a separator with `:---` (left), **When** rendered, **Then** cells in that column are left-aligned.
2. **Given** a separator with `:---:` (center), **When** rendered, **Then** cells in that column are center-aligned.
3. **Given** a separator with `---:` (right), **When** rendered, **Then** cells in that column are right-aligned.
4. **Given** a separator with `---` (no colons), **When** rendered, **Then** cells in that column have default alignment (left).

---

### User Story 3 - Styled Tables (Priority: P3)

A user sees rendered tables with appropriate CSS styling — borders, padding, and alternating row colors or clear row separation — matching the overall page style.

**Why this priority**: Without table CSS, the HTML table elements render with no visible borders or padding, making data hard to read.

**Independent Test**: Can be tested by verifying the existing embedded CSS contains table-related rules (`table`, `th`, `td` styles).

**Acceptance Scenarios**:

1. **Given** a rendered table, **When** the user views it, **Then** the table has visible borders and cell padding.
2. **Given** a table with multiple rows, **When** rendered, **Then** header cells are visually distinct from data cells.

---

### Edge Cases

- What happens when a line looks like a table but has no separator row? It should be treated as regular text, not a table.
- What happens when rows have mismatched column counts? Cells should be added or truncated to match the header column count.
- What happens when a cell is empty (consecutive pipes `||`)? An empty cell should be rendered.
- What happens when pipe characters appear inside inline code within a table cell? They should not be treated as column delimiters.
- What happens when a table is preceded or followed by other block elements? The table should be a separate block, not merged with adjacent content.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST recognize a table as a block starting with a pipe-delimited header row, followed by a separator row (dashes and optional colons), followed by zero or more pipe-delimited data rows. Header-only tables (no data rows) are valid.
- **FR-002**: The system MUST render recognized tables as `<table>` with `<thead>` containing `<tr>` and `<th>` elements for the header, and `<tbody>` containing `<tr>` and `<td>` elements for data rows.
- **FR-003**: The system MUST support column alignment via the separator row: `:---` (left), `:---:` (center), `---:` (right), `---` (default/left).
- **FR-004**: The system MUST apply alignment as a `style` attribute on `<th>` and `<td>` elements (e.g., `style="text-align: center"`).
- **FR-005**: The system MUST process inline formatting (bold, italic, code, links, images) within table cells.
- **FR-006**: The system MUST handle rows with fewer columns than the header by adding empty cells to match.
- **FR-007**: The system MUST handle rows with more columns than the header by ignoring excess columns.
- **FR-008**: The system MUST handle empty cells (consecutive pipes) by rendering empty `<td>` or `<th>` elements.
- **FR-009**: The system MUST treat leading and trailing pipes on table rows as optional (both `| a | b |` and `a | b` are valid).
- **FR-010**: The system MUST NOT treat pipe-like lines as tables if no valid separator row follows the first row.
- **FR-011**: The system MUST HTML-escape cell content (consistent with existing escaping behavior).
- **FR-012**: The system MUST add CSS rules for `table`, `th`, and `td` to the embedded stylesheet for borders, padding, and header distinction.
- **FR-013**: The system MUST NOT split cells on pipe characters inside inline code spans.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A standard 3-column, 5-row markdown table renders as a valid HTML table with 3 `<th>` and 15 `<td>` elements.
- **SC-002**: All three alignment options (left, center, right) produce correct `style` attributes on rendered cells.
- **SC-003**: Inline formatting within cells (bold, italic, code, links) renders correctly.
- **SC-004**: Tables with mismatched column counts render without errors, with cells padded or truncated as specified.
- **SC-005**: Existing tests continue to pass (table parsing does not break other markdown features).
- **SC-006**: Tables have visible borders and padding in the rendered page.

## Assumptions

- Table syntax follows GitHub Flavored Markdown (GFM) conventions.
- The separator row must contain at least one dash per column (e.g., `|---|`).
- Whitespace around cell content is trimmed.
- A table must have a header row and a separator row; data rows are optional (header-only tables render with just `<thead>`).
- Pipe characters inside backtick-delimited inline code are not treated as cell delimiters.

## Clarifications

### Session 2026-02-20

- Q: Should header-only tables (header + separator, no data rows) render as a table? → A: Yes, render them as a valid table with `<thead>` and empty `<tbody>`, matching GFM behavior.
