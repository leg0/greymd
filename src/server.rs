use std::io::Write;
use std::net::TcpListener;
use std::path::{Path, PathBuf};

use crate::http::{HttpRequest, HttpResponse};
use crate::listing;
use crate::markdown;
use crate::mime::content_type_for;
use crate::path::{ResolvedPath, resolve_path};

pub fn config_dir() -> Option<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()?;
    Some(PathBuf::from(home).join(".config").join("greymd"))
}

pub fn start(listener: TcpListener, root: &Path, css_path: PathBuf, js_path: PathBuf) {
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };
        let root = root.to_path_buf();
        let css_path = css_path.clone();
        let js_path = js_path.clone();
        std::thread::spawn(move || {
            handle_connection(&stream, &root, &css_path, &js_path);
        });
    }
}

fn serve_file(file_path: &Path, has_custom_css: bool) -> HttpResponse {
    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext.eq_ignore_ascii_case("md") {
        match std::fs::read_to_string(file_path) {
            Ok(source) => {
                let filename = file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("untitled.md");
                let html = crate::markdown::render(&source, filename, has_custom_css);
                HttpResponse::ok("text/html", html.into_bytes())
            }
            Err(_) => HttpResponse::not_found(),
        }
    } else {
        match std::fs::read(file_path) {
            Ok(contents) => HttpResponse::ok(content_type_for(ext), contents),
            Err(_) => HttpResponse::not_found(),
        }
    }
}

fn serve_directory(dir_path: &Path, root: &Path, url_path: &str, has_custom_css: bool) -> HttpResponse {
    let entries = listing::collect_entries(dir_path);

    // Auto-serve: single .md file → redirect so browser URL matches the file
    let md_files: Vec<&listing::DirectoryEntry> = entries.iter().filter(|e| !e.is_dir).collect();
    if md_files.len() == 1 {
        let target = format!("{}{}", if url_path.ends_with('/') { url_path.to_string() } else { format!("{url_path}/") }, &md_files[0].name);
        return HttpResponse::redirect(&target);
    }

    // Auto-serve: index.md when multiple .md files → redirect
    if md_files.len() > 1 && md_files.iter().any(|e| e.name == "index.md") {
        let target = format!("{}index.md", if url_path.ends_with('/') { url_path.to_string() } else { format!("{url_path}/") });
        return HttpResponse::redirect(&target);
    }

    // Fall back to listing
    let root_canonical = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let show_parent = dir_path != root_canonical;
    let html = listing::render_listing(url_path, &entries, show_parent, has_custom_css);
    HttpResponse::ok("text/html", html.into_bytes())
}

fn handle_connection(stream: &std::net::TcpStream, root: &Path, css_path: &Path, js_path: &Path) {
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(1)));
    let mut reader = std::io::BufReader::new(stream);
    let request = match HttpRequest::parse(&mut reader) {
        Some(r) => r,
        None => return,
    };

    let has_custom_css = css_path.is_file();

    let response = if request.method != "GET" {
        HttpResponse::method_not_allowed()
    } else if let Some(ref q) = request.query {
        match q.as_str() {
            "css" => HttpResponse::ok_gzip("text/css", markdown::CSS_GZ.to_vec()),
            "css2" => {
                match std::fs::read(css_path) {
                    Ok(content) => HttpResponse::ok("text/css", content),
                    Err(_) => HttpResponse::not_found(),
                }
            }
            "js" => {
                match std::fs::read(js_path) {
                    Ok(content) => HttpResponse::ok("application/javascript", content),
                    Err(_) => HttpResponse::ok_gzip("application/javascript", markdown::HLJS_JS_GZ.to_vec()),
                }
            }
            _ => HttpResponse::not_found(),
        }
    } else {
        match resolve_path(root, &request.path) {
            Some(ResolvedPath::File(file_path)) => serve_file(&file_path, has_custom_css),
            Some(ResolvedPath::Directory(dir_path)) => {
                serve_directory(&dir_path, root, &request.path, has_custom_css)
            }
            None => HttpResponse::not_found(),
        }
    };

    let _ = reader.get_mut().write_all(&response.to_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn setup_test_dir() -> crate::path::tempdir::TempDir {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("hello.txt"), "hello world").unwrap();
        std::fs::create_dir_all(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("sub/nested.txt"), "nested").unwrap();
        std::fs::write(dir.path().join("page.html"), "<h1>Page</h1>").unwrap();
        dir
    }

    fn start_server(root: &Path) -> u16 {
        start_server_with_config(root, PathBuf::from("/nonexistent/.config/greymd/css"), PathBuf::from("/nonexistent/.config/greymd/js"))
    }

    fn start_server_with_config(root: &Path, css_path: PathBuf, js_path: PathBuf) -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let root = root.to_path_buf();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = match stream {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let root = root.clone();
                let css_path = css_path.clone();
                let js_path = js_path.clone();
                std::thread::spawn(move || {
                    handle_connection(&stream, &root, &css_path, &js_path);
                });
            }
        });
        port
    }

    fn get(port: u16, path: &str) -> String {
        let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        let request = format!("GET {} HTTP/1.1\r\nHost: localhost\r\n\r\n", path);
        stream.write_all(request.as_bytes()).unwrap();
        stream.shutdown(std::net::Shutdown::Write).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }

    fn get_bytes(port: u16, path: &str) -> Vec<u8> {
        let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        let request = format!("GET {} HTTP/1.1\r\nHost: localhost\r\n\r\n", path);
        stream.write_all(request.as_bytes()).unwrap();
        stream.shutdown(std::net::Shutdown::Write).unwrap();
        let mut response = Vec::new();
        stream.read_to_end(&mut response).unwrap();
        response
    }

    // T012+T013: Tests for TCP listener and file serving
    #[test]
    fn serves_existing_file() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/hello.txt");
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains("hello world"));
    }

    #[test]
    fn serves_nested_file() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/sub/nested.txt");
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains("nested"));
    }

    #[test]
    fn returns_404_for_missing_file() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/nonexistent.txt");
        assert!(resp.contains("HTTP/1.1 404 Not Found"));
    }

    #[test]
    fn blocks_directory_traversal() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/../../../etc/passwd");
        assert!(resp.contains("404"));
    }

    #[test]
    fn returns_correct_content_type() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/page.html");
        assert!(resp.contains("Content-Type: text/html"));
    }

    // T022: Tests for error responses
    fn send_method(port: u16, method: &str, path: &str) -> String {
        let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        let request = format!("{} {} HTTP/1.1\r\nHost: localhost\r\n\r\n", method, path);
        stream.write_all(request.as_bytes()).unwrap();
        stream.shutdown(std::net::Shutdown::Write).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }

    #[test]
    fn returns_405_for_post() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = send_method(port, "POST", "/hello.txt");
        assert!(resp.contains("HTTP/1.1 405 Method Not Allowed"));
    }

    #[test]
    fn returns_405_for_put() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = send_method(port, "PUT", "/hello.txt");
        assert!(resp.contains("HTTP/1.1 405 Method Not Allowed"));
    }

    #[test]
    fn returns_405_for_delete() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = send_method(port, "DELETE", "/hello.txt");
        assert!(resp.contains("HTTP/1.1 405 Method Not Allowed"));
    }

    // T025: Port-in-use is handled via TcpListener::bind error in start()
    // (tested via integration — binding to an occupied port prints error and exits)
    #[test]
    fn bind_to_occupied_port_fails() {
        // Bind a port first
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        // Try binding again — should fail
        let result = TcpListener::bind(format!("127.0.0.1:{}", port));
        assert!(result.is_err());
    }

    // US1+US2: Markdown rendering integration tests

    #[test]
    fn serves_markdown_as_html() {
        let dir = setup_test_dir();
        std::fs::write(dir.path().join("doc.md"), "# Hello\n\nWorld").unwrap();
        let port = start_server(dir.path());
        let resp = get(port, "/doc.md");
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains("Content-Type: text/html"));
        assert!(resp.contains("<!DOCTYPE html>"));
        assert!(resp.contains("id=\"hello\""));
        assert!(resp.contains("<p>World</p>"));
    }

    // US2: Non-markdown files unchanged
    #[test]
    fn html_files_served_raw_not_wrapped() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/page.html");
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains("<h1>Page</h1>"));
        // Must NOT be double-wrapped in <!DOCTYPE>
        assert!(!resp.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn txt_files_served_raw() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/hello.txt");
        assert!(resp.contains("hello world"));
        assert!(!resp.contains("<!DOCTYPE html>"));
    }

    // Spec 3: Directory listing integration tests

    #[test]
    fn root_listing_shows_md_files_and_dirs() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("readme.md"), "# Readme").unwrap();
        std::fs::write(dir.path().join("notes.txt"), "skip").unwrap();
        std::fs::create_dir(dir.path().join("docs")).unwrap();

        let port = start_server(dir.path());
        // Only one .md file → redirect to it
        let resp = get(port, "/");
        assert!(resp.contains("HTTP/1.1 302 Found"));
        assert!(resp.contains("Location: /readme.md"));
    }

    #[test]
    fn root_listing_falls_back_when_multiple_md_no_index() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("a.md"), "# A").unwrap();
        std::fs::write(dir.path().join("b.md"), "# B").unwrap();
        std::fs::create_dir(dir.path().join("sub")).unwrap();

        let port = start_server(dir.path());
        let resp = get(port, "/");
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains("Index of /"));
        assert!(resp.contains("a.md"));
        assert!(resp.contains("b.md"));
        assert!(resp.contains("sub/"));
    }

    #[test]
    fn root_auto_serves_index_md() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("index.md"), "# Welcome").unwrap();
        std::fs::write(dir.path().join("other.md"), "# Other").unwrap();

        let port = start_server(dir.path());
        let resp = get(port, "/");
        assert!(resp.contains("HTTP/1.1 302 Found"));
        assert!(resp.contains("Location: /index.md"));
    }

    #[test]
    fn subdir_listing_shows_parent_link() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::create_dir(dir.path().join("docs")).unwrap();
        std::fs::write(dir.path().join("docs/a.md"), "# A").unwrap();
        std::fs::write(dir.path().join("docs/b.md"), "# B").unwrap();

        let port = start_server(dir.path());
        let resp = get(port, "/docs/");
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains(".."));
    }

    #[test]
    fn subdir_auto_serves_single_md() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::create_dir(dir.path().join("docs")).unwrap();
        std::fs::write(dir.path().join("docs/only.md"), "# Only").unwrap();

        let port = start_server(dir.path());
        let resp = get(port, "/docs/");
        assert!(resp.contains("HTTP/1.1 302 Found"));
        assert!(resp.contains("Location: /docs/only.md"));
    }

    #[test]
    fn listing_excludes_non_md_files() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("a.md"), "# A").unwrap();
        std::fs::write(dir.path().join("b.md"), "# B").unwrap();
        std::fs::write(dir.path().join("data.json"), "{}").unwrap();
        std::fs::write(dir.path().join("script.js"), "").unwrap();

        let port = start_server(dir.path());
        let resp = get(port, "/");
        assert!(resp.contains("a.md"));
        assert!(resp.contains("b.md"));
        assert!(!resp.contains("data.json"));
        assert!(!resp.contains("script.js"));
    }

    #[test]
    fn does_not_auto_serve_index_html() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("index.html"), "<h1>No</h1>").unwrap();
        std::fs::write(dir.path().join("a.md"), "# A").unwrap();
        std::fs::write(dir.path().join("b.md"), "# B").unwrap();

        let port = start_server(dir.path());
        let resp = get(port, "/");
        // Should show listing since index.html is not auto-served
        assert!(resp.contains("Index of /"));
    }

    #[test]
    fn empty_dir_shows_listing() {
        let dir = crate::path::tempdir::TempDir::new();

        let port = start_server(dir.path());
        let resp = get(port, "/");
        assert!(resp.contains("Index of /"));
        assert!(!resp.contains("<ul>"));
    }

    // Spec 4: HTML Styling integration tests

    #[test]
    fn markdown_page_has_link_tag() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("doc.md"), "# Test").unwrap();
        let port = start_server(dir.path());
        let resp = get(port, "/doc.md");
        assert!(resp.contains("<link"));
        assert!(resp.contains("/?css"));
        assert!(resp.contains("name=\"viewport\""));
    }

    #[test]
    fn directory_listing_has_link_tag() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("a.md"), "# A").unwrap();
        std::fs::write(dir.path().join("b.md"), "# B").unwrap();
        let port = start_server(dir.path());
        let resp = get(port, "/");
        assert!(resp.contains("<link"));
        assert!(resp.contains("/?css"));
        assert!(resp.contains("name=\"viewport\""));
    }

    #[test]
    fn markdown_and_listing_share_same_css_link() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("a.md"), "# A").unwrap();
        std::fs::write(dir.path().join("b.md"), "# B").unwrap();
        let port = start_server(dir.path());

        let md_resp = get(port, "/a.md");
        let listing_resp = get(port, "/");

        // Both should reference query-string assets
        assert!(md_resp.contains("/?css"));
        assert!(listing_resp.contains("/?css"));
    }

    #[test]
    fn css_query_returns_gzipped_css() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get_bytes(port, "/?css");
        let header_end = resp.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
        let header = std::str::from_utf8(&resp[..header_end]).unwrap();
        assert!(header.contains("HTTP/1.1 200 OK"));
        assert!(header.contains("Content-Type: text/css"));
        assert!(header.contains("Content-Encoding: gzip"));
        // Body starts with gzip magic bytes
        assert_eq!(resp[header_end + 4], 0x1f);
        assert_eq!(resp[header_end + 5], 0x8b);
    }

    #[test]
    fn js_query_returns_gzipped_js() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get_bytes(port, "/?js");
        let header_end = resp.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
        let header = std::str::from_utf8(&resp[..header_end]).unwrap();
        assert!(header.contains("HTTP/1.1 200 OK"));
        assert!(header.contains("Content-Type: application/javascript"));
        assert!(header.contains("Content-Encoding: gzip"));
    }

    #[test]
    fn markdown_page_uses_link_tag_not_style() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("doc.md"), "# Test").unwrap();
        let port = start_server(dir.path());
        let resp = get(port, "/doc.md");
        assert!(resp.contains("/?css"));
        assert!(resp.contains("<link"));
        #[cfg(not(feature = "math"))]
        assert!(!resp.contains("<style>"));
    }

    #[test]
    fn directory_listing_uses_link_tag_not_style() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(dir.path().join("a.md"), "# A").unwrap();
        std::fs::write(dir.path().join("b.md"), "# B").unwrap();
        let port = start_server(dir.path());
        let resp = get(port, "/");
        assert!(resp.contains("/?css"));
        assert!(resp.contains("<link"));
        #[cfg(not(feature = "math"))]
        assert!(!resp.contains("<style>"));
    }

    #[test]
    fn unknown_query_returns_404() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/?unknown");
        assert!(resp.contains("HTTP/1.1 404 Not Found"));
    }

    #[test]
    fn normal_files_still_resolve() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/hello.txt");
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains("hello world"));
    }

    #[test]
    fn markdown_page_includes_highlight_script() {
        let dir = crate::path::tempdir::TempDir::new();
        std::fs::write(
            dir.path().join("doc.md"),
            "# Test\n\n```rust\nfn main() {}\n```",
        )
        .unwrap();
        let port = start_server(dir.path());
        let resp = get(port, "/doc.md");
        assert!(resp.contains("/?js"));
        assert!(resp.contains("hljs.highlightAll()"));
    }

    #[test]
    fn config_dir_returns_path_when_home_set() {
        let dir = config_dir();
        // HOME is set in our test environment
        if std::env::var("HOME").is_ok() || std::env::var("USERPROFILE").is_ok() {
            let path = dir.unwrap();
            assert!(path.ends_with(".config/greymd"));
        }
    }

    #[test]
    fn config_dir_path_structure() {
        // Verify the path is constructed as home + .config/greymd
        if let Some(home) = std::env::var("HOME").ok().or_else(|| std::env::var("USERPROFILE").ok()) {
            let expected = PathBuf::from(home).join(".config").join("greymd");
            assert_eq!(config_dir().unwrap(), expected);
        }
    }

    #[test]
    fn css2_serves_custom_css_file() {
        let dir = setup_test_dir();
        let css_dir = crate::path::tempdir::TempDir::new();
        let css_path = css_dir.path().join("css");
        std::fs::write(&css_path, "body { color: red; }").unwrap();
        let port = start_server_with_config(dir.path(), css_path, PathBuf::from("/nonexistent/js"));
        let resp = get(port, "/?css2");
        assert!(resp.contains("200 OK"));
        assert!(resp.contains("text/css"));
        assert!(resp.contains("body { color: red; }"));
        assert!(!resp.contains("Content-Encoding: gzip"));
    }

    #[test]
    fn css2_returns_404_when_no_custom_css() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get(port, "/?css2");
        assert!(resp.contains("404"));
    }

    #[test]
    fn js_serves_custom_js_when_file_exists() {
        let dir = setup_test_dir();
        let js_dir = crate::path::tempdir::TempDir::new();
        let js_path = js_dir.path().join("js");
        std::fs::write(&js_path, "console.log('custom');").unwrap();
        let port = start_server_with_config(dir.path(), PathBuf::from("/nonexistent/css"), js_path);
        let resp = get(port, "/?js");
        assert!(resp.contains("200 OK"));
        assert!(resp.contains("application/javascript"));
        assert!(resp.contains("console.log('custom');"));
        assert!(!resp.contains("Content-Encoding: gzip"));
    }

    #[test]
    fn js_serves_builtin_when_no_custom_js() {
        let dir = setup_test_dir();
        let port = start_server(dir.path());
        let resp = get_bytes(port, "/?js");
        let header = String::from_utf8_lossy(&resp[..resp.len().min(512)]);
        assert!(header.contains("200 OK"));
        assert!(header.contains("Content-Encoding: gzip"));
    }

    #[test]
    fn css2_link_appears_when_css_file_created_after_start() {
        let dir = setup_test_dir();
        std::fs::write(dir.path().join("test.md"), "# Hello").unwrap();
        let css_dir = crate::path::tempdir::TempDir::new();
        let css_path = css_dir.path().join("css");
        let port = start_server_with_config(dir.path(), css_path.clone(), PathBuf::from("/nonexistent/js"));

        // Before CSS file exists: no css2 link
        let resp = get(port, "/test.md");
        assert!(!resp.contains("/?css2"), "should not have css2 link before file exists");

        // Create CSS file
        std::fs::write(&css_path, "body { color: blue; }").unwrap();

        // After CSS file exists: css2 link appears
        let resp = get(port, "/test.md");
        assert!(resp.contains("/?css2"), "should have css2 link after file created");
    }

    // T004: theme dir with css → pick_asset_path → server serves at /?css2 and HTML includes link
    #[test]
    fn theme_css_served_via_pick_asset_path() {
        let dir = setup_test_dir();
        std::fs::write(dir.path().join("page.md"), "# Themed").unwrap();

        let theme_dir = crate::path::tempdir::TempDir::new();
        std::fs::write(theme_dir.path().join("css"), "body { background: #1e1e2e; }").unwrap();

        let td = Some(theme_dir.path().to_path_buf());
        let css_path = crate::pick_asset_path(&td, &None, "css");
        let js_path = crate::pick_asset_path(&td, &None, "js");

        let port = start_server_with_config(dir.path(), css_path, js_path);

        // HTML page should include /?css2 link
        let html = get(port, "/page.md");
        assert!(html.contains("/?css2"), "themed page should have css2 link");

        // /?css2 should serve the theme CSS
        let css = get(port, "/?css2");
        assert!(css.contains("200 OK"));
        assert!(css.contains("body { background: #1e1e2e; }"));
    }

    // T005: theme dir with js → pick_asset_path → server serves custom JS at /?js
    #[test]
    fn theme_js_served_via_pick_asset_path() {
        let dir = setup_test_dir();

        let theme_dir = crate::path::tempdir::TempDir::new();
        std::fs::write(theme_dir.path().join("js"), "alert('theme');").unwrap();

        let td = Some(theme_dir.path().to_path_buf());
        let css_path = crate::pick_asset_path(&td, &None, "css");
        let js_path = crate::pick_asset_path(&td, &None, "js");

        let port = start_server_with_config(dir.path(), css_path, js_path);

        let resp = get(port, "/?js");
        assert!(resp.contains("200 OK"));
        assert!(resp.contains("alert('theme');"));
        // Should NOT be gzipped (custom JS is served raw)
        assert!(!resp.contains("Content-Encoding: gzip"));
    }

    // T013: Math content passes through as plain text in default build
    #[cfg(not(feature = "math"))]
    #[test]
    fn math_content_passthrough_without_feature() {
        let dir = setup_test_dir();
        std::fs::write(dir.path().join("math.md"), "Inline $x^2$ and display:\n\n$$\\sum x$$\n").unwrap();
        let port = start_server(dir.path());
        let resp = get(port, "/math.md");
        assert!(resp.contains("200 OK"));
        assert!(resp.contains("$x^2$"), "inline math should pass through");
        assert!(resp.contains("$$"), "display math should pass through");
        assert!(!resp.contains("<math"), "should not contain MathML");
    }
}
