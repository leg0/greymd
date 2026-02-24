# Research: Custom Themes

## Decision 1: Theme directory resolution

**Decision**: Resolve `<prefix>` from the binary's own path — `std::env::current_exe()` → parent (bin/) → parent (prefix/).

**Rationale**: Standard Unix convention. Binary at `<prefix>/bin/greymd`, data at `<prefix>/share/greymd/themes/`. Works for `/usr/local/`, `~/.local/`, and tarball extractions.

**Alternatives considered**:
- Hardcoded path (`/usr/share/greymd/themes/`) — too inflexible, breaks for user installs.
- Environment variable (`GREYMD_THEMES_DIR`) — adds configuration complexity counter to project philosophy.
- Embed in binary — `cargo install` can't install data files, and this would bloat the binary.

## Decision 2: Warning vs error for missing theme

**Decision**: Print a warning to stderr and fall back to default (no theme). Do not exit.

**Rationale**: User clarification — graceful degradation is preferred. The server should still start even if the theme path is wrong (e.g., `cargo install` users).

**Alternatives considered**:
- Exit with error — rejected per user clarification.
- Silent fallback — rejected; user should know their `--theme` flag had no effect.

## Decision 3: Release archive layout

**Decision**: Create `.tar.gz` and `.zip` archives with standard Unix layout:
```
greymd-<version>-<target>/
├── bin/greymd
└── share/greymd/themes/
    ├── default/css
    ├── catppuccin-latte/css
    ├── catppuccin-frappe/css
    ├── catppuccin-macchiato/css
    ├── catppuccin-mocha/css
    └── tokyo-night/css
```

**Rationale**: Standard FHS layout. Users can extract to `/usr/local/` or any prefix and everything works.

**Alternatives considered**:
- Flat layout (binary + themes in same dir) — non-standard, confusing.
- Separate themes archive — extra download step, bad UX.

## Decision 4: Existing code reuse

**Decision**: The `--theme`, `--list-themes`, `resolve_theme_dir`, `pick_asset_path`, and `list_themes` functions already exist in `src/main.rs`. Modify the error-on-missing behavior to warning-and-fallback. Add tests.

**Rationale**: Minimize code changes per constitution principle III.

**Alternatives considered**:
- Rewrite from scratch — unnecessary, existing code is correct aside from error behavior.
