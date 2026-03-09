use std::process::Command;

fn open_with(command: &str, args: &[&str]) {
    let _ = Command::new(command).args(args).spawn();
}

pub fn open(url: &str) {
    if cfg!(target_os = "macos") {
        open_with("open", &[url]);
    } else if cfg!(target_os = "windows") {
        open_with("cmd", &["/c", "start", "", url]);
    } else {
        open_with("xdg-open", &[url]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // All tests use open_with() with nonexistent commands to avoid
    // actually launching a browser or file explorer during tests.

    #[test]
    fn open_with_nonexistent_command_does_not_panic() {
        open_with("nonexistent-command-xxxxx", &["http://example.com"]);
    }

    #[test]
    fn open_with_valid_url_does_not_panic() {
        open_with("nonexistent-command-xxxxx", &["http://127.0.0.1:9999"]);
    }

    #[test]
    fn open_with_empty_url_does_not_panic() {
        open_with("nonexistent-command-xxxxx", &[""]);
    }

    #[test]
    fn open_with_does_not_block_caller() {
        let start = std::time::Instant::now();
        open_with("nonexistent-command-xxxxx", &["http://127.0.0.1:9999"]);
        assert!(start.elapsed() < std::time::Duration::from_secs(1));
    }

    #[test]
    fn open_with_invalid_binary_is_silent() {
        // Validates FR-006: no panic, no visible error, returns normally
        open_with("__no_such_browser__", &["http://localhost"]);
    }
}