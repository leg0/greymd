// Directory listing HTML generation.
//
// Generates HTML pages listing .md files and subdirectories
// with clickable navigation links.

use std::path::Path;

use crate::markdown::{escape_html, wrap_html_page};

/// An entry in a directory listing.
pub struct DirectoryEntry {
    pub name: String,
    pub is_dir: bool,
}

/// Collect .md files and subdirectories from a directory, sorted:
/// directories first, then files, each group case-insensitive alphabetical.
pub fn collect_entries(dir: &Path) -> Vec<DirectoryEntry> {
    let read_dir = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return Vec::new(),
    };

    let mut entries: Vec<DirectoryEntry> = read_dir
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let ft = e.file_type().ok()?;
            if ft.is_dir() {
                Some(DirectoryEntry { name, is_dir: true })
            } else if ft.is_file() && name.ends_with(".md") {
                Some(DirectoryEntry {
                    name,
                    is_dir: false,
                })
            } else {
                None
            }
        })
        .collect();

    entries.sort_by(|a, b| {
        // Directories first, then files; alphabetical within each group
        b.is_dir.cmp(&a.is_dir).then_with(|| {
            a.name
                .to_ascii_lowercase()
                .cmp(&b.name.to_ascii_lowercase())
        })
    });

    entries
}

/// Render a directory listing as an HTML page.
pub fn render_listing(url_path: &str, entries: &[DirectoryEntry], show_parent: bool, has_custom_css: bool) -> String {
    let mut body = String::new();

    let title = if url_path == "/" {
        "Index of /".to_string()
    } else {
        format!("Index of {}", url_path)
    };

    body.push_str(&format!("<h1>{}</h1>\n", escape_html(&title)));

    if show_parent {
        let parent = parent_url(url_path);
        body.push_str(&format!(
            "<a href=\"{}\">📁 ..</a><br>\n",
            escape_html(&parent)
        ));
    }

    let base = url_path.trim_end_matches('/');
    for entry in entries {
        let icon = if entry.is_dir { "📁 " } else { "📄 " };
        let suffix = if entry.is_dir { "/" } else { "" };
        let href = format!("{}/{}{}", base, entry.name, suffix);
        let display = format!("{}{}", entry.name, suffix);
        body.push_str(&format!(
            "<a href=\"{}\">{}{}</a><br>\n",
            escape_html(&href),
            icon,
            escape_html(&display)
        ));
    }

    body.push('\n');
    wrap_html_page(&title, &body, has_custom_css)
}

fn parent_url(url_path: &str) -> String {
    let trimmed = url_path.trim_end_matches('/');
    match trimmed.rfind('/') {
        Some(0) | None => "/".to_string(),
        Some(pos) => trimmed[..pos].to_string() + "/",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::tempdir;
    use std::fs;

    #[test]
    fn collect_entries_only_md_and_dirs() {
        let dir = tempdir::TempDir::new();
        fs::write(dir.path().join("readme.md"), "# Hi").unwrap();
        fs::write(dir.path().join("notes.txt"), "skip").unwrap();
        fs::write(dir.path().join("data.json"), "{}").unwrap();
        fs::create_dir(dir.path().join("sub")).unwrap();

        let entries = collect_entries(dir.path());
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["sub", "readme.md"]);
    }

    #[test]
    fn collect_entries_sorted_dirs_first_alpha() {
        let dir = tempdir::TempDir::new();
        fs::create_dir(dir.path().join("zebra")).unwrap();
        fs::create_dir(dir.path().join("alpha")).unwrap();
        fs::write(dir.path().join("beta.md"), "b").unwrap();
        fs::write(dir.path().join("Alpha.md"), "a").unwrap();

        let entries = collect_entries(dir.path());
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["alpha", "zebra", "Alpha.md", "beta.md"]);
    }

    #[test]
    fn collect_entries_empty_dir() {
        let dir = tempdir::TempDir::new();
        let entries = collect_entries(dir.path());
        assert!(entries.is_empty());
    }

    #[test]
    fn render_listing_basic() {
        let entries = vec![
            DirectoryEntry {
                name: "docs".into(),
                is_dir: true,
            },
            DirectoryEntry {
                name: "readme.md".into(),
                is_dir: false,
            },
        ];
        let html = render_listing("/", &entries, false, false);
        assert!(html.contains("<h1>Index of /</h1>"));
        assert!(html.contains("<a href=\"/docs/\">📁 docs/</a>"));
        assert!(html.contains("<a href=\"/readme.md\">📄 readme.md</a>"));
        assert!(!html.contains(".."));
    }

    #[test]
    fn render_listing_with_parent() {
        let entries = vec![];
        let html = render_listing("/sub/dir/", &entries, true, false);
        assert!(html.contains("<a href=\"/sub/\">📁 ..</a>"));
    }

    #[test]
    fn render_listing_no_parent_at_root() {
        let entries = vec![];
        let html = render_listing("/", &entries, false, false);
        assert!(!html.contains(".."));
    }

    #[test]
    fn render_listing_escapes_names() {
        let entries = vec![DirectoryEntry {
            name: "a<b>.md".into(),
            is_dir: false,
        }];
        let html = render_listing("/", &entries, false, false);
        assert!(html.contains("a&lt;b&gt;.md"));
    }
    #[test]
    fn parent_url_from_subdir() {
        assert_eq!(parent_url("/sub/dir/"), "/sub/");
        assert_eq!(parent_url("/sub/dir"), "/sub/");
        assert_eq!(parent_url("/sub/"), "/");
        assert_eq!(parent_url("/sub"), "/");
    }

    #[test]
    fn parent_url_from_root() {
        assert_eq!(parent_url("/"), "/");
    }
}
