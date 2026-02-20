# Research: Directory Listing

**Date**: 2026-02-20
**Feature**: 003-directory-listing

## Decision 1: Directory Detection in Request Handler

**Decision**: Modify `path.rs` to return an enum distinguishing files from directories

**Rationale**: Currently `resolve_path` returns `Option<PathBuf>` for files only. We need to distinguish three cases: file found, directory found, or not found. Returning an enum `ResolvedPath { File(PathBuf), Directory(PathBuf) }` allows `server.rs` to branch cleanly without re-checking the filesystem.

**Alternatives considered**:
- Check `is_dir()` in server.rs after resolve: Duplicates filesystem access. Rejected per Principle III.
- Separate `resolve_directory` function: Would require two calls for every request. Rejected.

## Decision 2: Auto-Serve Logic

**Decision**: Three-step priority in `server.rs` when path is a directory

**Rationale**: Per clarification: (1) If directory contains exactly one `.md` file → auto-serve it. (2) If multiple `.md` files and `index.md` exists → auto-serve `index.md`. (3) Otherwise → show listing. This is evaluated once per directory request using `read_dir` results already needed for the listing.

**Alternatives considered**:
- Always show listing: Rejected by user — auto-serve is desired for documentation browsing.
- Check for index.md first: Would miss the single-file case. Rejected.

## Decision 3: Listing HTML Generation

**Decision**: New `listing.rs` module with a `render_listing` function

**Rationale**: Keeps directory listing logic separate from server request handling. The function takes the directory path (relative to root), a list of entries, and whether to show a parent link. Returns a complete HTML5 page string using the same `wrap_html_page` pattern from `markdown.rs`.

**Alternatives considered**:
- Inline in server.rs: Would bloat the handler. Rejected for readability.
- Reuse markdown.rs wrapper directly: The listing generates HTML directly (not from Markdown), but can reuse the `wrap_html_page` helper.

## Decision 4: Entry Sorting

**Decision**: Sort in-place after collecting entries — directories first, then files, alphabetically case-insensitive within each group

**Rationale**: Collect entries from `read_dir`, partition into dirs and files, sort each group by lowercased name, concatenate. Simple and efficient for expected directory sizes (< 1000 entries).

**Alternatives considered**:
- Sort during iteration: `read_dir` doesn't guarantee order, so we must collect first anyway. No benefit.
- Use BTreeSet: Over-engineered for simple alphabetical sort. Rejected.

## Decision 5: Path Resolution for Directories

**Decision**: Extend `resolve_path` to also accept directory paths (currently rejects them)

**Rationale**: The current `resolve_path` uses `canonicalize` which works on directories too. The function just needs to not reject paths that are directories. Return type changes to an enum to distinguish file vs directory.

**Alternatives considered**:
- New function `resolve_dir`: Duplicates canonicalize + prefix check logic. Rejected.
