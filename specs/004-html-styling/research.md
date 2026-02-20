# Research: HTML Styling

## R1: CSS Delivery Method

**Decision**: Compile-time string constant (`const CSS: &str`) embedded in the binary, injected as `<style>` block in `<head>`.

**Rationale**: Zero external dependencies, zero network requests, single function modification point (`wrap_html_page`). CSS is shared by both markdown and listing pages automatically.

**Alternatives considered**:
- External CSS file served at a special URL — adds routing complexity, extra HTTP request per page load. Rejected.
- Inline styles on individual elements — verbose, not DRY, hard to maintain. Rejected.

## R2: Font Stack

**Decision**: System font stack: `-apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji"`.

**Rationale**: Industry standard (used by GitHub, Bootstrap). Zero network requests, renders natively on all platforms, fast.

**Alternatives considered**:
- Web fonts (Google Fonts, Inter) — requires network requests, violates zero-dep constitution. Rejected.
- Single font like Arial — less aesthetic on macOS/iOS. Rejected.

## R3: Code Font Stack

**Decision**: `"SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace`.

**Rationale**: Standard monospace stack, renders well across platforms.

## R4: Color Scheme

**Decision**: Light theme only. Body text `#24292e`, background white, links `#0366d6`, code background `#f6f8fa`, blockquote border `#dfe2e5`.

**Rationale**: GitHub-like palette, proven readability. Dark mode explicitly out of scope per spec assumptions.

## R5: Max Width

**Decision**: `max-width: 48em` (~768px at 16px base) with `padding: 1em 2em`.

**Rationale**: Optimal reading width is 50-75 characters per line. 48em achieves this with the chosen font size. Padding provides breathing room on narrow screens.

## R6: Viewport Meta Tag

**Decision**: `<meta name="viewport" content="width=device-width, initial-scale=1">` in `<head>`.

**Rationale**: Required for proper mobile rendering. Standard practice for responsive pages.
