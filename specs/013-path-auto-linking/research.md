# Research: Path Auto-Linking

## Decision 1: Path Token Character Set

**Decision**: Path tokens consist of characters matching `[a-zA-Z0-9._/-]`. The token must start with an alphabetic character or `.` (for `./`, `../`).

**Rationale**: This covers all common file path characters on Unix/macOS/Windows (except backslash, which is out of scope). Starting with alpha or `.` excludes numeric fractions (`1/2`) and punctuation-started tokens.

**Alternatives considered**:
- Allow `~` for home directory paths → rejected (out of scope, not relative)
- Allow `@` for scoped packages → rejected (not a file path pattern)
- Allow spaces (quoted paths) → rejected (too complex, rare in prose)

## Decision 2: Match Validation Rule

**Decision**: A token is a path if: (a) it ends with `/`, OR (b) its last segment (after the last `/`, or the whole token if no `/`) ends with `.md`. Non-`.md` extensions are NOT matched.

**Rationale**: The user clarified that only `.md` files should be auto-linked — non-`.md` extensions like `.js`, `.rs`, `.toml` appear frequently in prose as library names, tool names, or config references (e.g., `highlight.js`, `Cargo.toml`) and would produce unwanted links. A trailing `/` still indicates a directory.

**Alternatives considered**:
- Allow any file extension → rejected (too many false positives: `highlight.js`, `package.json`)
- Allow `.md` and `.html` → rejected (user explicitly chose `.md` only)
- Blocklist common non-path extensions → rejected (fragile, incomplete)

## Decision 3: Insertion Point in render_inline()

**Decision**: Path detection is placed after URL auto-linking (`try_parse_url`) and before the final character escape fallback, inside the existing `render_inline()` character loop.

**Rationale**: This ensures:
1. Code spans are already consumed (no paths inside backticks)
2. Explicit links `[text](url)` are already consumed
3. Images `![alt](url)` are already consumed
4. URLs starting with `http://`/`https://` are already consumed
5. Only remaining plain text reaches the path detector

**Alternatives considered**:
- Post-processing pass on rendered HTML → rejected (would need to parse HTML, complex, fragile)
- Pre-processing pass on source text → rejected (would need to skip code blocks, links, etc. — duplicating existing logic)

## Decision 4: Trailing Punctuation Stripping

**Decision**: After scanning a path token, strip trailing characters from the set `.,;:!?)` before validating the match rule. If after stripping the token no longer satisfies the match rule, don't link it.

**Rationale**: Prose often follows paths with punctuation: `see src/main.rs.` or `(docs/guide.md)`. The punctuation is not part of the path. The strip set matches what `try_parse_url` already uses for URL auto-linking.

**Alternatives considered**:
- Don't strip — require exact token match → rejected (too many false negatives in natural prose)
- Strip only `.` → rejected (commas and parentheses are equally common)

## Decision 5: Single-Segment Path Matching

**Decision**: A single token without `/` is matched only if it ends with `.md`. The token must start with an alphabetic character. Example: `README.md` matches, `highlight.js` and `3.14` do not.

**Rationale**: Single-segment filenames like `README.md` and `CHANGELOG.md` are commonly referenced in prose. The `.md`-only rule from Decision 2 applies uniformly — single-segment non-`.md` files are not linked.

**Alternatives considered**:
- Never match single-segment tokens → rejected (user explicitly wants `README.md` linked)
- Match any extension for single-segment → rejected (contradicts `.md`-only rule)

## Decision 6: Avoiding False Positives

**Decision**: The `.md`-only rule inherently eliminates most false positives (`e.g.`, `i.e.`, `$1.20`, `highlight.js`). Tokens starting with `.` require the next character to be `/` (for `./` and `../` relative prefixes), which prevents `.20` from being matched.

**Rationale**: The narrow `.md` extension + dot-prefix guard together eliminate virtually all false positive categories without needing blocklists or minimum length heuristics.

**Alternatives considered**:
- Blocklist `e.g.`, `i.e.` → not needed (`.md`-only rule handles it)
- Minimum extension length → not needed (`.md`-only rule handles it)
