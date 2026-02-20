# Research: Markdown-to-HTML Rendering

**Date**: 2026-02-20
**Feature**: 002-markdown-rendering

## Decision 1: Parser Architecture

**Decision**: Single-pass line-oriented parser (no intermediate AST)

**Rationale**: A two-pass approach (parse → AST → render) is cleaner but allocates an entire tree structure. A single-pass parser reads lines sequentially, tracks minimal state (current block type, list nesting depth), and emits HTML directly into a `String`. This satisfies Constitution Principle III (Minimum Resource Usage) while keeping complexity manageable for the CommonMark subset we support.

**Alternatives considered**:
- Full AST (parse → tree → render): Clean separation but unnecessary allocation for our scope. Rejected per Principle III.
- Event-based/SAX-style: Similar to single-pass but with callback overhead. No benefit for our use case.

## Decision 2: Inline Formatting Strategy

**Decision**: Recursive descent inline parser with delimiter stack

**Rationale**: Inline formatting (bold, italic, combined, code, links, images) requires tracking delimiter pairs (`*`, `**`, `***`, `` ` ``, `[`, `!`). A delimiter stack approach processes text left-to-right, pushing openers and matching closers. This handles nested/combined formatting (`***bold italic***`) correctly without backtracking. Code spans (`` ` ``) take precedence over other formatting (no nesting inside code).

**Alternatives considered**:
- Regex-based: Fragile with nested/overlapping patterns, hard to get right for edge cases. Rejected.
- Multi-pass (bold pass, then italic pass): Can't handle combined `***` correctly. Rejected.

## Decision 3: Block Element Recognition

**Decision**: Line-by-line state machine with lookahead

**Rationale**: Each line is classified by its prefix: `#` (heading), `-`/`*`/`+` (unordered list), `1.` (ordered list), `>` (blockquote), `` ``` `` (fenced code), `---`/`***`/`___` (horizontal rule), 4-space indent (indented code), or paragraph text. A state variable tracks the current block context (paragraph, list, code block, blockquote). Blank lines trigger block transitions.

**Alternatives considered**:
- Token-based lexer: Overkill for line-oriented Markdown. Rejected.
- Character-by-character parser: Unnecessarily complex for block-level parsing. Rejected.

## Decision 4: HTML Escaping Strategy

**Decision**: Escape all `<`, `>`, `&`, `"` in text content unconditionally

**Rationale**: Per clarification, raw HTML is not passed through. This simplifies the parser significantly — every `<` in the source becomes `&lt;` in output, with no need to detect/parse HTML tags. Escaping is applied during inline text processing before any Markdown syntax interpretation of the surrounding context.

**Alternatives considered**:
- Selective escaping (detect HTML tags): Would require an HTML tag parser within the Markdown parser. Rejected per clarification decision.

## Decision 5: List Nesting

**Decision**: Indent-based nesting with 2-space minimum indent increment

**Rationale**: Nested lists are detected by indentation level relative to the parent list item. Each additional 2+ spaces of indentation creates a deeper nesting level. The parser maintains a stack of list types (ordered/unordered) and indentation levels, emitting `<ul>`/`<ol>` open/close tags as nesting changes. Maximum practical depth is unlimited but tested to 3 levels per spec assumption.

**Alternatives considered**:
- Fixed 4-space indent only: Too strict, doesn't match common Markdown usage. Rejected.
- Tab-only nesting: Non-standard. Rejected.

## Decision 6: HTML Page Wrapper

**Decision**: Minimal HTML5 wrapper with title extraction

**Rationale**: The wrapper is a static template: `<!DOCTYPE html><html><head><meta charset="utf-8"><title>{title}</title></head><body>{content}</body></html>`. Title is extracted from the first `#` heading found, falling back to the filename (without `.md` extension). No CSS or JS included (deferred to spec 4).

**Alternatives considered**:
- No wrapper (fragment only): Doesn't satisfy FR-002 (valid HTML5 document). Rejected.

## Decision 7: Integration Point

**Decision**: Modify `server.rs::handle_connection` to detect `.md` extension and render

**Rationale**: The existing handler reads file contents and serves with appropriate Content-Type. For `.md` files, instead of serving raw bytes, pass contents through `markdown::render()` and serve the result as `text/html`. This is a minimal change to the existing code path — a single branch in the match arm.

**Alternatives considered**:
- Middleware/filter pattern: Overengineered for a single transformation. Rejected.
- New handler function: Unnecessary; the existing handler already has the file contents and extension.
