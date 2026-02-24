# Data Model: Custom Themes

## Entities

### Theme

A directory on disk containing customization files.

| Attribute | Description |
|-----------|-------------|
| name | Directory name (e.g., `catppuccin-mocha`) |
| css | Optional file at `<theme-dir>/css` — custom stylesheet |
| js | Optional file at `<theme-dir>/js` — custom JavaScript |

### Themes Directory

The install-time location for bundled themes.

| Attribute | Description |
|-----------|-------------|
| path | `<prefix>/share/greymd/themes/` |
| resolution | Derived from binary path: `current_exe() → parent → parent → share/greymd/themes/` |

## Runtime State

At startup, `main()` resolves:

1. `theme_dir: Option<PathBuf>` — from `--theme <name>` → `resolve_theme_dir(name)`
2. `config_dir: Option<PathBuf>` — from `config_dir()` → `~/.config/greymd/`
3. `css_path: PathBuf` — `pick_asset_path(theme_dir, config_dir, "css")`
4. `js_path: PathBuf` — `pick_asset_path(theme_dir, config_dir, "js")`

Per-request, `handle_connection` checks:
- `css_path.is_file()` → determines `has_custom_css` for HTML link injection
- `std::fs::read(css_path)` → serves `/?css2` content
- `std::fs::read(js_path)` → serves `/?js` content (or fallback to built-in)

## Precedence Rules

| Priority | Source | Condition |
|----------|--------|-----------|
| 1 (highest) | `--theme <name>` directory | File exists in theme dir |
| 2 | `~/.config/greymd/` | File exists in config dir |
| 3 (lowest) | Built-in assets | Default fallback |

Per-file override: each asset (css, js) is resolved independently.
