# Research: Math Rendering

## Decision 1: LaTeX-to-MathML crate selection

**Decision**: Use `latex2mathml` 0.2.3 as an optional dependency.

**Rationale**: Zero transitive dependencies, pure Rust, simple API (`latex_to_mathml(latex, DisplayStyle)` → `Result<String>`). Aligns with constitution principle II (minimal dependencies). Covers common LaTeX math: fractions, subscripts, superscripts, Greek letters, integrals, sums, roots, matrices.

**Alternatives considered**:
- `math-core` (tmke8) — actively maintained, MathML Core compliant, broader LaTeX coverage. However, it pulls in transitive dependencies and has a more complex API. Better fit for projects where comprehensive LaTeX coverage is critical.
- Hand-written converter — would require ~3000+ lines for reasonable coverage. The long tail of ~300 LaTeX math commands makes this impractical for the value delivered.
- Client-side KaTeX — adds ~150KB of embedded JS. Violates the "no JavaScript for math" design goal.

## Decision 2: Delimiter parsing strategy

**Decision**: Parse `$...$` in `render_inline` (inline formatting pass). Parse `$$...$$` in `render_body` as a block-level construct (like fenced code blocks), requiring `$$` at start of line.

**Rationale**: Inline `$` must be handled alongside existing backtick/link/image parsing to respect precedence (code spans take priority over math). Block `$$` is a line-level construct that should be handled before inline processing, similar to fenced code blocks.

**Alternatives considered**:
- Pre-process all math before any markdown parsing — risks interfering with code blocks and link parsing.
- Post-process HTML output — fragile, would need to parse generated HTML.

## Decision 3: Feature flag design

**Decision**: Feature named `math`, NOT in default features. `Cargo.toml` uses `[features] math = ["dep:latex2mathml"]`.

**Rationale**: User requirement — math off by default. `cargo install greymd` gets the lean build. `cargo install greymd --features math` gets math. Release archives provide both variants pre-built.

**Alternatives considered**:
- Math in default features — rejected per user requirement.
- Runtime flag instead of compile-time — would always include the dependency, contradicting the goal of keeping the default build dependency-free.

## Decision 4: CSS font hint for MathML

**Decision**: When `math` feature is enabled, add a CSS `@font-face` / font-family rule targeting `math` elements to suggest STIX Two Math or similar math-optimized fonts. Include this conditionally in the built-in CSS via `#[cfg(feature = "math")]`.

**Rationale**: Chromium browsers need a math font hint for high-quality MathML rendering. Firefox and Safari work without it. The CSS addition is minimal (~3 lines) and only included when math is enabled.

**Alternatives considered**:
- Embed a math font — would add ~500KB+ to binary. Overkill for a hint.
- No font hint — Chromium renders math poorly without one. Unacceptable UX.

## Decision 5: Invalid LaTeX handling

**Decision**: When `latex2mathml::latex_to_mathml()` returns `Err`, render the raw LaTeX text wrapped in `<code>` tags. Do not crash or produce blank output.

**Rationale**: Graceful degradation. Users see what they typed and can fix it. Matches the spec requirement FR-006.

**Alternatives considered**:
- Show error message in rendered output — noisy and breaks page flow.
- Silently drop — user loses content with no feedback.

## Decision 6: Release workflow dual variants

**Decision**: Extend the GitHub Actions release workflow matrix to build each target twice: once default (no math) and once with `--features math`. Naming: `greymd-linux-x64.tar.gz` (standard) and `greymd-math-linux-x64.tar.gz` (math).

**Rationale**: Users choose their variant at download time. Both variants include themes in the same `bin/` + `share/` layout.

**Alternatives considered**:
- Single binary with runtime detection — would always ship the dependency.
- Separate repo for math variant — unnecessarily complex.
