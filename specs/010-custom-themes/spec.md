# Feature Specification: Custom Themes

**Feature Branch**: `010-custom-themes`  
**Created**: 2026-02-24  
**Status**: Draft  
**Input**: User description: "Add support for custom themes"

## User Scenarios & Testing

### User Story 1 — Apply a Theme by Name (Priority: P1) 🎯 MVP

A user wants to change the look of greymd by selecting a bundled theme by name. They run `greymd --theme catppuccin-mocha` and the server uses that theme's CSS and/or JS files instead of (or in addition to) the ones in `~/.config/greymd/`.

**Why this priority**: This is the core feature — selecting a theme should be a single flag, not manual file copying.

**Independent Test**: Run `greymd --theme catppuccin-mocha /path/to/docs`, request a markdown page, verify the theme's CSS is served at `/?css2` and/or the theme's JS replaces `/?js`.

**Acceptance Scenarios**:

1. **Given** a theme named `catppuccin-mocha` is installed at `<prefix>/share/greymd/themes/catppuccin-mocha/`, **When** the user starts greymd with `--theme catppuccin-mocha`, **Then** the theme's `css` file is served at `/?css2` and the HTML includes a `<link>` to `/?css2`.
2. **Given** a theme directory containing a `js` file, **When** the user starts with `--theme <name>`, **Then** the `js` file replaces the built-in highlight.js at `/?js`.
3. **Given** a theme containing both `css` and `js` files, **When** the user starts with `--theme <name>`, **Then** both overrides apply.
4. **Given** a theme containing only `css` and the user also has `~/.config/greymd/js`, **When** the user starts with `--theme <name>`, **Then** the theme's CSS is used and `~/.config/greymd/js` is still used for JS.
5. **Given** a non-existent theme name, **When** the user starts greymd with `--theme <name>`, **Then** greymd prints a warning and starts normally without theme overrides.

---

### User Story 2 — List Available Themes (Priority: P2)

A user wants to see which themes are installed. They run `greymd --list-themes` and see the names of all themes available in the themes directory.

**Why this priority**: Discoverability — users need to know what themes exist before they can use one.

**Independent Test**: Run `greymd --list-themes`, verify it lists all installed themes from `<prefix>/share/greymd/themes/`.

**Acceptance Scenarios**:

1. **Given** themes are installed at `<prefix>/share/greymd/themes/`, **When** the user runs `--list-themes`, **Then** the theme names are printed.
2. **Given** no themes are installed, **When** the user runs `--list-themes`, **Then** a helpful message is shown.

---

### Edge Cases

- What happens when `--theme` directory exists but contains neither `css` nor `js`? The server starts normally with no custom overrides from the theme.
- What happens when `--theme` is passed without a value? Error message and exit.
- What happens when both `--theme` and `~/.config/greymd/` have a `css` file? The `--theme` file takes priority.
- What happens when the theme file is deleted while the server is running? The next request returns 404 for `/?css2` or falls back to built-in for `/?js`.

## Requirements

### Functional Requirements

- **FR-001**: The `--theme <name>` flag MUST accept a theme name and resolve it to `<prefix>/share/greymd/themes/<name>/` where `<prefix>` is determined from the binary's install location.
- **FR-002**: Theme files MUST override `~/.config/greymd/` files on a per-file basis — only files present in the theme directory override their config counterparts.
- **FR-003**: If the theme directory contains a `css` file, it MUST be served at `/?css2` and the HTML MUST include the `/?css2` link.
- **FR-004**: If the theme directory contains a `js` file, it MUST replace the built-in highlight.js at `/?js`.
- **FR-005**: If the named theme does not exist, greymd MUST print a warning and fall back to the default appearance (no theme applied). It MUST NOT exit.
- **FR-006**: The `--list-themes` flag MUST list all themes installed under `<prefix>/share/greymd/themes/`.
- **FR-007**: Theme files MUST be re-read on every request (same as `~/.config/greymd/` behavior).
- **FR-008**: The `--theme` flag MUST work alongside the positional `[directory]` argument.
- **FR-009**: The installation package MUST include the following bundled themes: `default`, `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`, `catppuccin-mocha`, `tokyo-night`.
- **FR-010**: Release archives (.tar.gz and .zip) MUST include bundled themes at `share/greymd/themes/` alongside the binary at `bin/`. Themes are NOT available via `cargo install`.

### Key Entities

- **Theme**: A directory containing a `css` file, a `js` file, or both. Named by its directory name.
- **Themes directory**: `<prefix>/share/greymd/themes/` — the install-time location for bundled themes.
- **Config directory**: `~/.config/greymd/` — the user's per-machine customization directory, overridden per-file by `--theme`.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Users can switch themes with a single `--theme <name>` flag without copying files.
- **SC-002**: Theme CSS/JS overrides are applied correctly and visible in the browser on first page load.
- **SC-003**: All existing tests continue to pass — no regressions.
- **SC-004**: `--list-themes` output includes all six bundled themes.
- **SC-005**: After installation, all bundled themes are present in `<prefix>/share/greymd/themes/`.

## Clarifications

### Session 2026-02-24

- Q: When themes directory doesn't exist (e.g. cargo install), what should --theme do? → A: Print a warning and fall back to default appearance (no theme applied). Do not exit.

## Assumptions

- Theme directories contain files named exactly `css` and/or `js` (no file extension) — matching the `~/.config/greymd/` convention.
- `<prefix>` is derived from the binary location: if the binary is at `<prefix>/bin/greymd`, themes are at `<prefix>/share/greymd/themes/`.
- Release archives (.tar.gz and .zip) follow standard Unix layout with `bin/` and `share/` directories. Themes are included in these packages.
- `cargo install` only installs the binary — themes are not available via `cargo install`. This is a known limitation.
