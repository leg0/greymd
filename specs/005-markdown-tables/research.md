# Research: Markdown Tables

## R1: Table Detection Strategy

**Decision**: Lookahead-based detection in the main `render_body` loop. When a line contains `|`, peek at the next line to check for a valid separator row.

**Rationale**: Fits the existing single-pass, line-oriented architecture. No backtracking needed — just a one-line lookahead. This is how most single-pass markdown parsers handle tables.

**Alternatives considered**:
- Two-pass approach (scan for tables first, then render) — unnecessary complexity, violates minimum resource usage. Rejected.
- Regex-based detection — adds complexity, harder to handle edge cases like pipes in code. Rejected.

## R2: Cell Splitting with Code-Aware Parsing

**Decision**: Custom `split_table_cells` function that tracks backtick state while scanning for `|` delimiters.

**Rationale**: Pipe characters inside inline code (`` `a|b` ``) must not be treated as cell delimiters (FR-013). A simple `split('|')` would break this. Tracking backtick open/close state during the scan handles this correctly.

**Alternatives considered**:
- Pre-process code spans before splitting — fragile, hard to reconstruct positions. Rejected.
- Escape pipes in code spans first — adds unnecessary complexity. Rejected.

## R3: Alignment Representation

**Decision**: Use a simple `Alignment` enum (`Left`, `Center`, `Right`) stored in a `Vec<Alignment>` per table.

**Rationale**: Minimal memory, directly maps to the `style="text-align: ..."` attribute. No need for a more complex representation.

## R4: Table CSS

**Decision**: Add `table`, `th`, `td` rules to the existing `const CSS` in `markdown.rs`.

**Rationale**: Consistent with spec 4 approach. Tables need `border-collapse: collapse`, cell borders, padding, and distinct header background to be readable.

## R5: Header-Only Tables

**Decision**: Render header-only tables (header + separator, no data rows) as a valid table with `<thead>` and empty `<tbody>`.

**Rationale**: Matches GFM behavior per user clarification.
