# Feature Specification: Math Rendering

**Feature Branch**: `011-math-rendering`  
**Created**: 2026-02-24  
**Status**: Draft  
**Input**: User description: "Add optional LaTeX math rendering via feature flag. Math off by default. Publish release variants with and without math. Server-side conversion to MathML — no JavaScript dependency."

## User Scenarios & Testing

### User Story 1 — Inline Math Rendering (Priority: P1) 🎯 MVP

A user writes a Markdown file containing inline LaTeX math expressions using `$...$` delimiters. When they browse the file in greymd (built with math support), the math is rendered as properly formatted equations in the browser.

**Why this priority**: This is the core value — users with technical or scientific documentation can see math rendered inline alongside normal text.

**Independent Test**: Create a `.md` file containing `The formula $E = mc^2$ is famous.` and verify the HTML output contains rendered math (MathML markup) rather than raw LaTeX.

**Acceptance Scenarios**:

1. **Given** a Markdown file containing `$E = mc^2$`, **When** a user views it in greymd built with math support, **Then** the equation renders as formatted math in the browser.
2. **Given** a Markdown file containing `The area is $A = \pi r^2$ square meters.`, **When** rendered, **Then** the inline math appears inline with surrounding text, not on its own line.
3. **Given** a Markdown file containing `Price is $5 and $10`, **When** rendered, **Then** the dollar amounts are NOT treated as math delimiters (non-math `$` followed by a digit and space is not a valid LaTeX expression — the converter should handle this gracefully or the parser should require no space after opening `$`).

---

### User Story 2 — Display Math Rendering (Priority: P1)

A user writes a Markdown file containing display (block) math using `$$...$$` delimiters. When browsed, the math renders as a centered block equation.

**Why this priority**: Display math is as fundamental as inline math — most math-heavy documents use both.

**Independent Test**: Create a `.md` file with `$$\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$$` on its own line and verify it renders as a centered block equation.

**Acceptance Scenarios**:

1. **Given** a Markdown file with `$$\int_0^\infty e^{-x} dx = 1$$` on its own line, **When** rendered, **Then** the equation displays as a centered block.
2. **Given** display math spanning multiple lines between `$$` delimiters, **When** rendered, **Then** the entire expression is treated as a single block equation.

---

### User Story 3 — Graceful Degradation Without Math Feature (Priority: P1)

A user builds greymd without math support (the default). Markdown files containing `$...$` or `$$...$$` are rendered with the LaTeX source visible as plain text, not garbled or hidden.

**Why this priority**: The default build must not break on math content — it should simply pass it through.

**Independent Test**: Build greymd without the math feature, serve a file with `$E = mc^2$`, and verify the raw LaTeX text appears in the output.

**Acceptance Scenarios**:

1. **Given** greymd built without math support, **When** a file containing `$x^2$` is rendered, **Then** the output shows `$x^2$` as plain text.
2. **Given** greymd built without math support, **When** a file containing `$$\sum x$$` is rendered, **Then** the output shows `$$\sum x$$` as plain text.

---

### User Story 4 — Math Support Visibility in Help (Priority: P2)

A user runs `greymd --help` and can see whether their build includes math support.

**Why this priority**: Users need to know if math rendering is available before diagnosing rendering issues.

**Independent Test**: Run `greymd --help` with and without math support and verify the output differs.

**Acceptance Scenarios**:

1. **Given** greymd built with math support, **When** the user runs `--help`, **Then** the output indicates math rendering is available.
2. **Given** greymd built without math support, **When** the user runs `--help`, **Then** there is no mention of math rendering.

---

### User Story 5 — Release Variants (Priority: P2)

A user downloading greymd from the release page can choose between a standard build and a build with math support.

**Why this priority**: Users who don't need math shouldn't pay the binary size cost, and those who do should have a ready-made download.

**Independent Test**: Check that a release produces two sets of archives per platform — one standard, one with math.

**Acceptance Scenarios**:

1. **Given** a release is created, **When** a user visits the release page, **Then** they see separate archives for the standard build and the math build (e.g., `greymd-linux-x64.tar.gz` and `greymd-math-linux-x64.tar.gz`).
2. **Given** a user downloads the math variant, **When** they run it, **Then** math rendering works without additional setup.

---

### Edge Cases

- What happens when `$` appears inside a fenced code block or inline code span? It MUST NOT be treated as a math delimiter.
- What happens when `$` appears inside a URL or HTML attribute? It MUST NOT be treated as a math delimiter.
- What happens with unmatched `$` — a single `$` with no closing? It MUST be passed through as literal text.
- What happens with empty math `$$`? It MUST be passed through as literal text.
- What happens with nested `$...$` inside `$$...$$`? The outer `$$` takes precedence.
- What happens with `\$` (escaped dollar sign)? It MUST render as a literal `$`.

## Requirements

### Functional Requirements

- **FR-001**: Math rendering MUST be available as an optional build feature, off by default.
- **FR-002**: When the math feature is enabled, inline math delimited by `$...$` MUST be converted to inline MathML in the HTML output.
- **FR-003**: When the math feature is enabled, display math delimited by `$$...$$` MUST be converted to block MathML in the HTML output. The opening `$$` MUST appear at the start of a line (block-level only).
- **FR-004**: When the math feature is disabled, `$...$` and `$$...$$` content MUST be passed through as plain text.
- **FR-005**: Math delimiters inside fenced code blocks, indented code blocks, and inline code spans MUST NOT be processed as math.
- **FR-006**: Invalid LaTeX that cannot be converted MUST be rendered as the raw LaTeX source text rather than producing an error or blank output.
- **FR-007**: The `--help` output MUST indicate whether math support is compiled in.
- **FR-008**: Release archives MUST be published in two variants per platform: standard (no math) and math-enabled.
- **FR-009**: The opening `$` for inline math MUST NOT be followed by a space, and the closing `$` MUST NOT be preceded by a space (to avoid false positives on currency like `$5`).
- **FR-010**: When math is enabled, the built-in CSS MUST include a font hint for math rendering quality in all browsers.

### Key Entities

- **Math Expression**: A LaTeX string delimited by `$...$` (inline) or `$$...$$` (display). Converted to MathML when the math feature is enabled.
- **Feature Flag**: A build-time toggle that includes or excludes math conversion capability.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Common LaTeX expressions (fractions, subscripts, superscripts, Greek letters, integrals, sums, square roots) render correctly in Chrome, Firefox, and any MathML-capable browser.
- **SC-002**: All existing tests continue to pass — no regressions from adding the feature flag.
- **SC-003**: The default build (without math) produces the same binary size as before this feature.
- **SC-004**: Dollar signs in normal prose, code blocks, and URLs are never incorrectly treated as math delimiters.
- **SC-005**: Each release includes both standard and math variants for every platform.

## Assumptions

- MathML Core is supported natively in Chrome 109+, Firefox 115+, and Safari 12+. No JavaScript polyfill is needed.
- The `latex2mathml` crate (pure Rust, zero transitive dependencies) is used for LaTeX-to-MathML conversion.
- `$...$` and `$$...$$` are the only supported math delimiters. `\(...\)` and `\[...\]` are not supported in this iteration.
- Math rendering adds no per-request overhead beyond the conversion itself — no additional assets to serve.
- The math feature flag name is `math`.

## Clarifications

### Session 2026-02-24

- Q: Should `$$...$$` display math be block-level only (opening `$$` at start of line) or work anywhere? → A: Block-level only — opening `$$` must be at the start of a line.
