# Data Model: Markdown-to-HTML Rendering

**Date**: 2026-02-20
**Feature**: 002-markdown-rendering

## Overview

This feature is stateless — no persistent data. All entities exist transiently during a single request-response cycle.

## Entities

### 1. MarkdownInput

The raw Markdown source text read from a `.md` file.

- **source**: `&str` — borrowed reference to file contents (zero-copy from `fs::read_to_string`)
- **filename**: `&str` — the filename without path, used as fallback title

No owned data; the parser borrows from the file contents read by the server handler.

### 2. HtmlPage

The complete rendered HTML document returned to the browser.

- **title**: `String` — extracted from first `#` heading, or filename fallback
- **body**: `String` — rendered HTML content from Markdown conversion
- Combined into full HTML5 wrapper on output

### 3. ParserState (internal, not exposed)

Transient state during single-pass parsing:

- **current_block**: enum — Paragraph, FencedCode, IndentedCode, Blockquote, List
- **list_stack**: Vec of (list_type, indent_level) — tracks nested list context
- **output**: String — accumulated HTML output (pre-allocated capacity)

## Relationships

```
MarkdownInput --[render()]--> HtmlPage
                  |
                  v
            ParserState (transient, discarded after render)
```

## State Transitions

ParserState.current_block transitions:

```
None → Paragraph      (non-empty text line)
None → FencedCode     (``` line)
None → IndentedCode   (4-space indented line after blank)
None → Blockquote     (> line)
None → List           (- or 1. line)
Paragraph → None      (blank line)
FencedCode → None     (closing ``` line)
IndentedCode → None   (non-indented line or blank + non-indented)
Blockquote → None     (blank line without >)
List → None           (blank line + non-indented non-list line)
```
