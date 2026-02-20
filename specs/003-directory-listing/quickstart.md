# Quickstart: Directory Listing

**Date**: 2026-02-20
**Feature**: 003-directory-listing

## Test Scenarios

### Scenario 1: Root URL Shows Listing
Start docsvr with a directory containing multiple `.md` files and subdirectories. Navigate to `/`. Verify an HTML page listing all `.md` files and subdirectories.

### Scenario 2: Subdirectory Navigation
Click a subdirectory link in the listing. Verify the subdirectory listing is displayed with its own entries and a `..` parent link.

### Scenario 3: Parent Directory Link
From a subdirectory listing, click `..`. Verify navigation returns to the parent directory listing. Verify root listing has no `..` link.

### Scenario 4: Auto-Serve Single .md File
Create a directory with exactly one `.md` file. Request that directory. Verify the `.md` file is auto-served as rendered HTML (not a listing).

### Scenario 5: Auto-Serve index.md
Create a directory with multiple `.md` files including `index.md`. Request that directory. Verify `index.md` is auto-served as rendered HTML.

### Scenario 6: Listing When No index.md
Create a directory with multiple `.md` files but no `index.md`. Request that directory. Verify a listing is shown with all `.md` files.

### Scenario 7: Only .md Files and Dirs Shown
Create a directory with `.md`, `.txt`, `.html`, and `.rs` files plus subdirectories. Verify only `.md` files and subdirectories appear in the listing.

### Scenario 8: Sorted and Grouped
Create a directory with files `z.md`, `a.md` and subdirectories `beta/`, `alpha/`. Verify listing shows `alpha/`, `beta/` first, then `a.md`, `z.md`.

### Scenario 9: Empty Directory
Request an empty directory. Verify a valid HTML page with no entries (just title and parent link if applicable).

### Scenario 10: Special Characters in Filenames
Create a file named `notes & thoughts.md`. Verify the listing escapes `&` correctly in both display text and href.

### Scenario 11: No Regression on Direct File Access
Request a `.txt` or `.html` file directly by path. Verify it is still served as raw content (not affected by directory listing feature).
