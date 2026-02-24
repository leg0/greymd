# Data Model: Math Rendering

## Entities

### Math Expression

A LaTeX string found in markdown source, delimited by `$` or `$$`.

| Attribute | Description |
|-----------|-------------|
| raw_latex | The LaTeX source text between delimiters (excluding the `$`/`$$` markers) |
| display_style | `Inline` (from `$...$`) or `Block` (from `$$...$$` at line start) |
| output | MathML markup (when math feature enabled) or raw text (when disabled or on conversion error) |

### Feature Flag

A compile-time toggle controlling math support.

| Attribute | Description |
|-----------|-------------|
| name | `math` |
| default | Off (not in default features) |
| effect | Includes `latex2mathml` crate; enables `$`/`$$` → MathML conversion in `render_inline` and `render_body` |

## Processing Flow

```
Markdown source
    │
    ├─ render_body() scans lines
    │   ├─ Line starts with "$$" → collect until closing "$$" line
    │   │   ├─ [math enabled] → latex2mathml(Block) → <math display="block">...</math>
    │   │   └─ [math disabled] → emit raw "$$...$$" as text
    │   │
    │   └─ Other lines → pass to render_inline()
    │
    └─ render_inline() scans characters
        ├─ Backtick span (existing) → takes priority, skip
        ├─ "$" (not preceded by "\", not inside code) → find closing "$"
        │   ├─ [math enabled] → latex2mathml(Inline) → <math>...</math>
        │   └─ [math disabled] → emit raw "$...$" as text
        └─ Other inline elements → existing behavior
```

## Precedence Rules

| Priority | Element | Wins Over |
|----------|---------|-----------|
| 1 (highest) | Fenced code block (```) | Everything (existing) |
| 2 | Indented code block | Everything (existing) |
| 3 | Inline code span (`) | Math delimiters |
| 4 | Display math ($$) | Inline math, paragraphs |
| 5 | Inline math ($) | Bold, italic, links |
| 6 (lowest) | Other inline formatting | — |

## Conditional Compilation

```
#[cfg(feature = "math")]     → call latex2mathml, emit MathML
#[cfg(not(feature = "math"))] → pass through raw text
```

Both code paths exist in the same functions, gated by `#[cfg]`. No separate modules.
