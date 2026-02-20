# Data Model: HTML Styling

This feature is purely presentational — no data entities, relationships, or state transitions are involved.

## Artifacts

### CSS Constant

- **Type**: Compile-time string constant (`const CSS: &str`)
- **Location**: `src/markdown.rs`
- **Content**: Complete CSS stylesheet covering all HTML elements produced by the markdown renderer and directory listing generator
- **Lifecycle**: Immutable at runtime; changes require recompilation

### HTML Page Structure (modified)

Current:
```
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>{title}</title>
</head>
<body>{body}</body>
</html>
```

After:
```
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>{CSS}</style>
</head>
<body>{body}</body>
</html>
```
