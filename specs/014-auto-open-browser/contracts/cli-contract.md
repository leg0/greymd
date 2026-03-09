# CLI Contract: Auto-Open Browser

**Feature**: 014-auto-open-browser  
**Date**: 2026-03-09

## New Flag

### `--no-browser`

**Type**: Boolean flag (no value)  
**Default**: Absent (browser opens by default)  
**Effect**: Suppresses automatic browser launch on startup  
**Interaction with other flags**: Independent — can be combined with all existing flags (`--theme`, positional directory argument, etc.)

## Updated Usage

```
greymd [OPTIONS] [DIRECTORY]

Options:
    --help, -h          Show help message
    --version, -V       Show version
    --list-themes       List available themes
    --theme <name>      Use specified theme
    --no-browser        Do not open the default browser on startup

Arguments:
    [DIRECTORY]         Directory to serve (defaults to current directory)
```

## Behavioral Contract

| Condition | Browser Opens? | URL Printed? | Server Starts? |
|-----------|---------------|-------------|----------------|
| No flags | Yes | Yes | Yes |
| `--no-browser` | No | Yes | Yes |
| Headless / no browser available | No (silent failure) | Yes | Yes |
| `--no-browser` + headless | No | Yes | Yes |

## Backward Compatibility

**Breaking change**: Existing users who run `greymd` will now see their browser open automatically. This is intentional per the clarified spec (open by default, opt-out via `--no-browser`). Users who want the old behavior must add `--no-browser`.
