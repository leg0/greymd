use std::path::{Path, PathBuf};

/// Result of resolving a URL path against the root directory.
pub enum ResolvedPath {
    File(PathBuf),
    Directory(PathBuf),
}

/// Resolves a URL path against the root directory, preventing directory traversal.
/// Returns None if the resolved path escapes the root or doesn't exist.
pub fn resolve_path(root: &Path, url_path: &str) -> Option<ResolvedPath> {
    let decoded = percent_decode(url_path);
    let relative = decoded.trim_start_matches('/');

    let root_canonical = root.canonicalize().ok()?;

    if relative.is_empty() {
        return Some(ResolvedPath::Directory(root_canonical));
    }

    let candidate = root.join(relative);
    let canonical = candidate.canonicalize().ok()?;

    // Security: ensure resolved path is within root
    if !canonical.starts_with(&root_canonical) {
        return None;
    }

    if canonical.is_file() {
        Some(ResolvedPath::File(canonical))
    } else if canonical.is_dir() {
        Some(ResolvedPath::Directory(canonical))
    } else {
        None
    }
}

fn percent_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().and_then(hex_val);
            let lo = chars.next().and_then(hex_val);
            if let (Some(h), Some(l)) = (hi, lo) {
                result.push((h << 4 | l) as char);
            }
        } else {
            result.push(b as char);
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_test_dir() -> tempdir::TempDir {
        let dir = tempdir::TempDir::new();
        fs::write(dir.path().join("hello.txt"), "hello").unwrap();
        fs::create_dir_all(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("sub/nested.txt"), "nested").unwrap();
        dir
    }

    // T010: Tests for path resolution and traversal prevention
    #[test]
    fn resolves_simple_file() {
        let dir = setup_test_dir();
        let result = resolve_path(dir.path(), "/hello.txt");
        assert!(matches!(result, Some(ResolvedPath::File(_))));
    }

    #[test]
    fn resolves_nested_file() {
        let dir = setup_test_dir();
        let result = resolve_path(dir.path(), "/sub/nested.txt");
        assert!(matches!(result, Some(ResolvedPath::File(_))));
    }

    #[test]
    fn rejects_directory_traversal() {
        let dir = setup_test_dir();
        let result = resolve_path(dir.path(), "/../../../etc/passwd");
        assert!(result.is_none());
    }

    #[test]
    fn rejects_encoded_traversal() {
        let dir = setup_test_dir();
        let result = resolve_path(dir.path(), "/%2e%2e/%2e%2e/etc/passwd");
        assert!(result.is_none());
    }

    #[test]
    fn rejects_nonexistent_file() {
        let dir = setup_test_dir();
        let result = resolve_path(dir.path(), "/nonexistent.txt");
        assert!(result.is_none());
    }

    #[test]
    fn resolves_root_as_directory() {
        let dir = setup_test_dir();
        let result = resolve_path(dir.path(), "/");
        assert!(matches!(result, Some(ResolvedPath::Directory(_))));
    }

    #[test]
    fn resolves_subdirectory() {
        let dir = setup_test_dir();
        let result = resolve_path(dir.path(), "/sub");
        assert!(matches!(result, Some(ResolvedPath::Directory(_))));
    }

    #[test]
    fn rejects_directory_traversal_for_dirs() {
        let dir = setup_test_dir();
        let result = resolve_path(dir.path(), "/../../../etc");
        assert!(result.is_none());
    }

    #[test]
    fn decodes_percent_encoded_path() {
        let dir = setup_test_dir();
        fs::write(dir.path().join("my file.txt"), "space").unwrap();
        let result = resolve_path(dir.path(), "/my%20file.txt");
        assert!(matches!(result, Some(ResolvedPath::File(_))));
    }
}

/// Minimal temp directory helper (no external deps)
#[cfg(test)]
pub(crate) mod tempdir {
    use std::path::{Path, PathBuf};

    pub struct TempDir(PathBuf);

    impl TempDir {
        pub fn new() -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!("docsvr-test-{}", std::process::id()));
            // Add a counter to avoid collisions between tests
            static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            path.push(format!("{}", id));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        pub fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
