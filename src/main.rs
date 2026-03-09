mod http;
mod listing;
mod markdown;
mod mime;
mod path;
mod server;

use std::env;
use std::net::TcpListener;
use std::path::{Path, PathBuf};

fn bind_listener() -> TcpListener {
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(_) => TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind to any port"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("greymd {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return;
    }

    if args.iter().any(|a| a == "--list-themes") {
        list_themes();
        return;
    }

    // Parse --theme <name>
    let theme_name = args.windows(2).find(|w| w[0] == "--theme").map(|w| w[1].clone());

    // Collect positional args (skip --theme and its value)
    let mut positional = Vec::new();
    let mut skip_next = false;
    for arg in &args[1..] {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--theme" {
            skip_next = true;
            continue;
        }
        if arg == "--list-themes" {
            continue;
        }
        positional.push(arg.clone());
    }

    let root = if !positional.is_empty() {
        PathBuf::from(&positional[0])
    } else {
        env::current_dir().unwrap_or_else(|e| {
            eprintln!("Error: cannot determine current directory: {}", e);
            std::process::exit(1);
        })
    };

    if !root.is_dir() {
        eprintln!("Error: '{}' is not a directory", root.display());
        std::process::exit(1);
    }

    let theme_dir = theme_name.as_ref().and_then(|name| resolve_theme_dir(name));
    if theme_name.is_some() && theme_dir.is_none() {
        eprintln!("Warning: theme '{}' not found, using default appearance", theme_name.as_ref().unwrap());
        eprintln!("Run 'greymd --list-themes' to see available themes.");
    }

    let config = server::config_dir();
    let css_path = pick_asset_path(&theme_dir, &config, "css");
    let js_path = pick_asset_path(&theme_dir, &config, "js");

    let listener = bind_listener();
    let addr = listener.local_addr().unwrap();
    println!("Listening on http://{}", addr);

    server::start(listener, &root, css_path, js_path);
}

/// Pick the asset path: theme dir overrides config dir if the file exists there.
fn pick_asset_path(theme_dir: &Option<PathBuf>, config_dir: &Option<PathBuf>, name: &str) -> PathBuf {
    if let Some(td) = theme_dir {
        let p = td.join(name);
        if p.is_file() {
            return p;
        }
    }
    if let Some(cd) = config_dir {
        return cd.join(name);
    }
    PathBuf::new()
}

/// Resolve theme directory relative to the binary's install prefix.
/// Looks for <prefix>/share/greymd/themes/<name> where <prefix> is
/// the parent of the binary's parent directory (bin/).
fn resolve_theme_dir(name: &str) -> Option<PathBuf> {
    let exe = env::current_exe().ok()?;
    let prefix = exe.parent()?.parent()?;
    resolve_theme_dir_in(prefix, name)
}

/// Resolve theme directory under a given prefix.
fn resolve_theme_dir_in(prefix: &Path, name: &str) -> Option<PathBuf> {
    let dir = prefix.join("share").join("greymd").join("themes").join(name);
    if dir.is_dir() {
        Some(dir)
    } else {
        None
    }
}

/// List available themes from the install prefix.
fn list_themes() {
    let exe = match env::current_exe() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("No themes found.");
            return;
        }
    };
    let prefix = match exe.parent().and_then(|p| p.parent()) {
        Some(p) => p.to_path_buf(),
        None => {
            eprintln!("No themes found.");
            return;
        }
    };
    let themes_dir = prefix.join("share").join("greymd").join("themes");
    let names = collect_themes(&themes_dir);
    print_themes(&names, &themes_dir);
}

/// Collect sorted theme names from a themes directory.
fn collect_themes(themes_dir: &Path) -> Vec<String> {
    let entries = match std::fs::read_dir(themes_dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let mut names: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    names.sort();
    names
}

/// Print theme names or a helpful message if none found.
fn print_themes(names: &[String], themes_dir: &Path) {
    if names.is_empty() {
        eprintln!("No themes found at {}", themes_dir.display());
        return;
    }
    println!("Available themes:");
    for name in names {
        println!("  {}", name);
    }
    println!();
    println!("Usage: greymd --theme <name> [directory]");
}

fn print_usage() {
    println!("greymd {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: greymd [options] [directory]");
    println!();
    println!("Serve files from a directory over HTTP on localhost.");
    println!();
    println!("Arguments:");
    println!("  directory              Path to serve (defaults to current directory)");
    println!();
    println!("Options:");
    println!("  --theme <name>         Use a bundled theme (warns and falls back to");
    println!("                         default if theme not found)");
    println!("  --list-themes          List available themes");
    println!("  -V, --version          Print version information");
    println!("  -h, --help             Show this help message");
    println!();
    println!("Customization:");
    println!("  ~/.config/greymd/css   Custom stylesheet (appended after built-in)");
    println!("  ~/.config/greymd/js    Custom JavaScript (replaces built-in highlight.js)");
    println!();
    println!("Theme files override ~/.config/greymd/ for files they contain.");
    println!("CSS and JS are re-read on every request.");
    #[cfg(feature = "math")]
    {
        println!();
        println!("Math rendering:          Enabled (LaTeX → MathML)");
        println!("  $...$                  Inline math");
        println!("  $$...$$                Display math (block-level)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_listener_returns_listener_on_8080_when_available() {
        // This test may get port 8080 or a random port depending on environment
        let listener = bind_listener();
        let addr = listener.local_addr().unwrap();
        assert_eq!(addr.ip(), std::net::Ipv4Addr::LOCALHOST);
        assert_ne!(addr.port(), 0);
    }

    #[test]
    fn bind_listener_falls_back_when_8080_busy() {
        // Grab port 8080 first
        let blocker = TcpListener::bind("127.0.0.1:8080");
        let listener = bind_listener();
        let addr = listener.local_addr().unwrap();
        assert_eq!(addr.ip(), std::net::Ipv4Addr::LOCALHOST);
        if blocker.is_ok() {
            // We successfully blocked 8080, so bind_listener must have picked another port
            assert_ne!(addr.port(), 8080);
        }
        assert_ne!(addr.port(), 0);
    }

    #[test]
    fn pick_asset_path_prefers_theme_dir() {
        let theme_dir = crate::path::tempdir::TempDir::new();
        let theme_css = theme_dir.path().join("css");
        std::fs::write(&theme_css, "theme css").unwrap();
        let config_dir = crate::path::tempdir::TempDir::new();
        std::fs::write(config_dir.path().join("css"), "config css").unwrap();

        let result = pick_asset_path(
            &Some(theme_dir.path().to_path_buf()),
            &Some(config_dir.path().to_path_buf()),
            "css",
        );
        assert_eq!(result, theme_css);
    }

    #[test]
    fn pick_asset_path_falls_back_to_config() {
        let theme_dir = crate::path::tempdir::TempDir::new();
        // No css in theme dir
        let config_dir = crate::path::tempdir::TempDir::new();
        let config_css = config_dir.path().join("css");
        std::fs::write(&config_css, "config css").unwrap();

        let result = pick_asset_path(
            &Some(theme_dir.path().to_path_buf()),
            &Some(config_dir.path().to_path_buf()),
            "css",
        );
        assert_eq!(result, config_css);
    }

    #[test]
    fn pick_asset_path_no_theme_uses_config() {
        let config_dir = crate::path::tempdir::TempDir::new();
        let config_js = config_dir.path().join("js");

        let result = pick_asset_path(
            &None,
            &Some(config_dir.path().to_path_buf()),
            "js",
        );
        assert_eq!(result, config_js);
    }

    // T001: resolve_theme_dir_in returns None for nonexistent theme → warning path
    #[test]
    fn resolve_theme_dir_in_missing_theme_returns_none() {
        let prefix = crate::path::tempdir::TempDir::new();
        // No themes directory at all
        let result = resolve_theme_dir_in(prefix.path(), "nonexistent");
        assert!(result.is_none());
    }

    // T002: resolve_theme_dir_in returns Some when theme dir exists
    #[test]
    fn resolve_theme_dir_in_found_returns_path() {
        let prefix = crate::path::tempdir::TempDir::new();
        let theme_path = prefix.path().join("share/greymd/themes/mocha");
        std::fs::create_dir_all(&theme_path).unwrap();
        std::fs::write(theme_path.join("css"), "body{}").unwrap();

        let result = resolve_theme_dir_in(prefix.path(), "mocha");
        assert_eq!(result, Some(theme_path));
    }

    // T003: resolve_theme_dir_in returns None for a name that doesn't match any dir
    #[test]
    fn resolve_theme_dir_in_wrong_name_returns_none() {
        let prefix = crate::path::tempdir::TempDir::new();
        let theme_path = prefix.path().join("share/greymd/themes/mocha");
        std::fs::create_dir_all(&theme_path).unwrap();

        let result = resolve_theme_dir_in(prefix.path(), "latte");
        assert!(result.is_none());
    }

    // T006: theme has css only, config has js — both used
    #[test]
    fn pick_asset_path_theme_css_config_js() {
        let theme_dir = crate::path::tempdir::TempDir::new();
        std::fs::write(theme_dir.path().join("css"), "theme").unwrap();
        let config_dir = crate::path::tempdir::TempDir::new();
        std::fs::write(config_dir.path().join("js"), "config js").unwrap();

        let td = Some(theme_dir.path().to_path_buf());
        let cd = Some(config_dir.path().to_path_buf());

        let css = pick_asset_path(&td, &cd, "css");
        let js = pick_asset_path(&td, &cd, "js");

        // Theme css wins
        assert_eq!(css, theme_dir.path().join("css"));
        // Config js used since theme has no js
        assert_eq!(js, config_dir.path().join("js"));
    }

    // T007: empty theme dir — no custom overrides
    #[test]
    fn pick_asset_path_empty_theme_dir() {
        let theme_dir = crate::path::tempdir::TempDir::new();
        // Empty — no css or js files

        let td = Some(theme_dir.path().to_path_buf());

        let css = pick_asset_path(&td, &None, "css");
        let js = pick_asset_path(&td, &None, "js");

        // Falls through to empty path (no config either)
        assert_eq!(css, PathBuf::new());
        assert_eq!(js, PathBuf::new());
    }

    // T008: collect_themes returns sorted theme names
    #[test]
    fn collect_themes_returns_sorted_names() {
        let dir = crate::path::tempdir::TempDir::new();
        let themes_dir = dir.path().join("themes");
        std::fs::create_dir_all(themes_dir.join("zulu")).unwrap();
        std::fs::create_dir_all(themes_dir.join("alpha")).unwrap();
        std::fs::create_dir_all(themes_dir.join("mocha")).unwrap();
        // A file (not a dir) should be excluded
        std::fs::write(themes_dir.join("not-a-theme"), "").unwrap();

        let names = collect_themes(&themes_dir);
        assert_eq!(names, vec!["alpha", "mocha", "zulu"]);
    }

    // T009: collect_themes with nonexistent dir returns empty
    #[test]
    fn collect_themes_missing_dir_returns_empty() {
        let names = collect_themes(Path::new("/nonexistent/themes"));
        assert!(names.is_empty());
    }
}
