# Data Model: Configuration & Customization

## Entities

No complex data model needed. The "configuration" is implicit from the filesystem:

### Custom Asset Paths (computed at startup)

| Path | Purpose | Behavior |
|------|---------|----------|
| `~/.config/greymd/css` | Custom stylesheet | Served at `/?css2` if readable |
| `~/.config/greymd/js` | Custom JavaScript | Replaces built-in at `/?js` if readable |

### Runtime State

| Value | Type | Source | Description |
|-------|------|--------|-------------|
| `port` | `u16` | CLI `--port` or auto-selected | Actual bound port |
| `has_custom_css` | `bool` | `~/.config/greymd/css` exists at startup | Controls `/?css2` link in HTML |
| `css_path` | `PathBuf` | Computed from `HOME`/`USERPROFILE` | Full path to custom CSS file |
| `js_path` | `PathBuf` | Computed from `HOME`/`USERPROFILE` | Full path to custom JS file |

## Path Computation

```text
home = env::var("HOME") or env::var("USERPROFILE")
config_dir = home + "/.config/greymd/"
css_path = config_dir + "css"
js_path = config_dir + "js"
```

No merging, no precedence, no parsing needed.
