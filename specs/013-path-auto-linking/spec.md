# Feature Specification: Path Auto-Linking

**Feature Branch**: `013-path-auto-linking`  
**Created**: 2026-02-26  
**Status**: Draft  
**Input**: User description: "When a document mentions a relative path in prose, it should be automatically converted to a clickable link. File existence is irrelevant — purely pattern-based detection."

---

## Problem Statement

When markdown files reference other files by relative path (e.g., `examples/math-demo.md`, `docs/setup.md`), the rendered HTML shows them as plain text. Users must manually wrap paths in `[text](url)` link syntax to make them clickable. Since greymd is a file server, these `.md` paths are often navigable — auto-linking them improves browsing without requiring authors to change their markdown.

---

## User Scenarios & Testing

### User Story 1 — Basic Path Auto-Linking (Priority: P1) 🎯 MVP

A user writes a Markdown file mentioning a relative path like `examples/math-demo.md` in prose. When rendered in greymd, the path becomes a clickable link.

**Why this priority**: This is the entire feature — detect path-like tokens in prose and wrap them in `<a>` tags.

**Independent Test**: Create a `.md` file containing `See examples/math-demo.md for details.` and verify the HTML output contains `<a href="examples/math-demo.md">examples/math-demo.md</a>`.

**Acceptance Scenarios**:

1. **Given** a Markdown file containing `See examples/math-demo.md for details.`, **When** rendered, **Then** the output contains `<a href="examples/math-demo.md">examples/math-demo.md</a>`.
2. **Given** a Markdown file containing `Edit docs/setup.md for configuration.`, **When** rendered, **Then** `docs/setup.md` becomes a clickable link.
3. **Given** a Markdown file containing `Look at ./README.md`, **When** rendered, **Then** `./README.md` becomes a clickable link.
4. **Given** a Markdown file containing `The parent ../docs/guide.md has details.`, **When** rendered, **Then** `../docs/guide.md` becomes a clickable link.
5. **Given** a path-like token with no file extension such as `src/utils/`, **When** rendered, **Then** it becomes a clickable link (trailing slash indicates directory).
6. **Given** a single-segment filename like `README.md`, **When** rendered, **Then** it becomes a clickable link (has `.md` extension).
7. **Given** a non-`.md` extension like `highlight.js` or `src/main.rs`, **When** rendered, **Then** it is NOT auto-linked.

---

### User Story 2 — No Interference with Existing Syntax (Priority: P1)

Path auto-linking must not interfere with existing markdown syntax: explicit links `[text](url)`, inline code `` `path` ``, fenced code blocks, images, or bare URL auto-linking.

**Why this priority**: Breaking existing rendering would be a regression.

**Independent Test**: Render a line containing an explicit link ``[this file](src/main.rs)`` and a code span ``examples/demo.md`` and verify neither gets double-linked.

**Acceptance Scenarios**:

1. **Given** `[click here](src/main.rs)`, **When** rendered, **Then** the output is a normal link — the URL inside `()` is NOT auto-linked again.
2. **Given** `` `examples/math-demo.md` `` (code span containing only a path), **When** rendered, **Then** the output is `<code><a href="examples/math-demo.md">examples/math-demo.md</a></code>` — a clickable link inside code styling.
3. **Given** `` `run src/main.rs --verbose` `` (code span with extra text beyond the path), **When** rendered, **Then** the output is plain `<code>` text with no link.
4. **Given** a fenced code block containing `src/main.rs`, **When** rendered, **Then** the path is plain text inside the code block.
5. **Given** `Visit https://example.com/path/to/file.md`, **When** rendered, **Then** the full URL is auto-linked as before — the `/path/to/file.md` portion is NOT separately linked.
6. **Given** `![diagram](images/arch.png)`, **When** rendered, **Then** the image URL is NOT auto-linked — it remains an image tag.

---

### User Story 3 — Edge Cases and Boundaries (Priority: P2)

Path detection handles edge cases correctly: punctuation after paths, paths at start/end of line, multiple paths on one line, and tokens that look path-like but aren't.

**Why this priority**: Robustness — false positives are worse than false negatives.

**Independent Test**: Render `Check docs/a.md, docs/b.md, and docs/c.md.` and verify all three become separate links, with trailing punctuation excluded.

**Acceptance Scenarios**:

1. **Given** `Check docs/setup.md.`, **When** rendered, **Then** the trailing period is NOT part of the link — output is `<a href="docs/setup.md">docs/setup.md</a>.`
2. **Given** `(see docs/guide.md)`, **When** rendered, **Then** the parentheses are NOT part of the link.
3. **Given** `docs/a.md, docs/b.md, and docs/c.md`, **When** rendered, **Then** all three paths are separate links.
4. **Given** a token like `hello/world` with no file extension and no trailing slash, **When** rendered, **Then** it is NOT auto-linked (no extension, not a directory path).
5. **Given** a fraction like `1/2` or `3/4`, **When** rendered, **Then** it is NOT auto-linked (no extension, no trailing slash).
6. **Given** `and/or`, **When** rendered, **Then** it is NOT auto-linked (no extension, no trailing slash).
7. **Given** a path token immediately following `#` (heading), **When** rendered, **Then** it is auto-linked within the heading text.

---

## Clarifications

### Session 2026-02-26

- Q: How should extensionless single-slash tokens like `hello/world`, `and/or` be handled? → A: Only link tokens that end with `/` (directory) or whose last segment has `.md` extension. Single-segment files like `README.md` are also linked.
- Q: Should paths inside backtick code spans be linked? → A: Yes, if the code span contains ONLY a path (nothing else). Render as `<code><a href="path">path</a></code>`. If code span has any extra text, keep as plain `<code>`.

### Session 2026-02-28

- Q: Which file extensions should trigger auto-linking? → A: Only `.md` files and trailing-slash directories. Non-`.md` extensions (`.js`, `.rs`, `.toml`, etc.) are NOT auto-linked.

- **FR-001**: Tokens matching the relative path pattern in rendered prose are wrapped in `<a href="TOKEN">TOKEN</a>`.
- **FR-002**: Detection is purely syntactic — file existence on disk is not checked.
- **FR-003**: Path detection runs during inline rendering, after code spans, explicit links, images, and URL auto-linking have been processed.
- **FR-003a**: Code spans containing only a path token (matching the `.md`-only path pattern per FR-006) are rendered as `<code><a href="PATH">PATH</a></code>`. Code spans containing any additional text beyond the path, or a non-`.md` path, are rendered as plain `<code>` with no link.
- **FR-004**: Trailing punctuation (`.`, `,`, `;`, `:`, `!`, `?`, `)`) is stripped from the matched path.
- **FR-005**: Paths starting with `./` or `../` are matched if they end with `/` or their last segment has a `.md` extension.
- **FR-006**: A token is matched as a path if: (a) it ends with `/`, OR (b) its last path segment has a `.md` extension. The token must start with an alphabetic character or `.`. Single-segment tokens like `README.md` are also matched. Non-`.md` extensions (e.g., `.js`, `.rs`, `.toml`) are NOT matched.
- **FR-007**: Tokens without a `.md` extension in their last segment and not ending with `/` are NOT matched, regardless of `/` separators. This excludes `and/or`, `hello/world`, `1/2`, `true/false`, `highlight.js`, `src/main.rs`.

## Success Criteria

- **SC-001**: `examples/math-demo.md` in prose becomes a clickable link.
- **SC-002**: Paths inside code blocks, explicit links, and images are not affected. Code spans containing only a `.md` path become clickable per FR-003a.
- **SC-003**: All existing tests continue to pass.
- **SC-004**: No false positives on extensionless slash-separated tokens like `and/or`, `hello/world`, `1/2`, `true/false`, or non-`.md` extensions like `highlight.js`, `src/main.rs`.

## Constraints

- **C-001**: Changes are limited to the markdown rendering pipeline — no changes to HTTP routing or file serving.
- **C-002**: No filesystem access during rendering — pattern-based only.
- **C-003**: Must not regress rendering performance in any measurable way.

## Out of Scope

- Validating that linked paths exist on disk.
- Linking absolute paths (starting with `/`).
- Linking Windows-style paths (`C:\`, `foo\bar`).
- Adding new CLI flags or configuration for this feature.
