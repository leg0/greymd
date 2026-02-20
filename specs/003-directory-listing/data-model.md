# Data Model: Directory Listing

**Date**: 2026-02-20
**Feature**: 003-directory-listing

## Overview

This feature is stateless — no persistent data. All entities exist transiently during a single request-response cycle.

## Entities

### 1. ResolvedPath (modified from spec 1)

The result of resolving a URL path against the served root directory.

- Variant `File(PathBuf)` — path resolves to a regular file
- Variant `Directory(PathBuf)` — path resolves to a directory

Replaces the current `Option<PathBuf>` return type of `resolve_path`.

### 2. DirectoryEntry

A single entry in a directory listing.

- **name**: `String` — display name of the file or directory
- **is_dir**: `bool` — true for subdirectories, false for `.md` files
- **href**: `String` — relative URL for the link

### 3. DirectoryListing

The collected data needed to render a directory listing page.

- **path**: `String` — the directory path relative to root (used as page title)
- **entries**: `Vec<DirectoryEntry>` — sorted list of entries (dirs first, then files)
- **show_parent**: `bool` — whether to include `..` link (false for root)

## Relationships

```
URL path --[resolve_path()]--> ResolvedPath::Directory
    |
    v
read_dir() --> Vec<DirectoryEntry>
    |
    v
Auto-serve check:
  - 1 .md file → serve it via markdown::render()
  - multiple + index.md → serve index.md via markdown::render()
  - else → render_listing() → HTML page
```
