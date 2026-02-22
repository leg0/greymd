use std::io::Write;
use std::net::TcpListener;
use std::path::Path;

use crate::http::{HttpRequest, HttpResponse};
use crate::listing;
use crate::markdown;
use crate::mime::content_type_for;
use crate::path::{ResolvedPath, resolve_path};

pub fn start(root: &Path, port: u16) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: could not bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };
    println!("Listening on http://{}", addr);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };
        let root = root.to_path_buf();
        std::thread::spawn(move || {
            handle_connection(&stream, &root);
        });
    }
}

fn serve_file(file_path: &Path) -> HttpResponse {
    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext.eq_ignore_ascii_case("md") {
        match std::fs::read_to_string(file_path) {
            Ok(source) => {
                let filename = file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("untitled.md");
                let html = crate::markdown::render(&source, filename);
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

fn serve_directory(dir_path: &Path, root: &Path, url_path: &str) -> HttpResponse {
    let entries = listing::collect_entries(dir_path);

    // Auto-serve: single .md file
    let md_files: Vec<&listing::DirectoryEntry> = entries.iter().filter(|e| !e.is_dir).collect();
    if md_files.len() == 1 {
        let file_path = dir_path.join(&md_files[0].name);
        return serve_file(&file_path);
    }

    // Auto-serve: index.md when multiple .md files
    if md_files.len() > 1 && md_files.iter().any(|e| e.name == "index.md") {
        let file_path = dir_path.join("index.md");
        return serve_file(&file_path);
    }

    // Fall back to listing
    let root_canonical = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let show_parent = dir_path != root_canonical;
    let html = listing::render_listing(url_path, &entries, show_parent);
    HttpResponse::ok("text/html", html.into_bytes())
}

fn handle_connection(stream: &std::net::TcpStream, root: &Path) {
    let mut reader = std::io::BufReader::new(stream);
    let request = match HttpRequest::parse(&mut reader) {
        Some(r) => r,
        None => return,
    };

    let response = if request.method != "GET" {
        HttpResponse::method_not_allowed()
    } else if let Some(ref q) = request.query {
        match q.as_str() {
            "css" => HttpResponse::ok_gzip("text/css", markdown::CSS_GZ.to_vec()),
            "js" => HttpResponse::ok_gzip("application/javascript", markdown::HLJS_JS_GZ.to_vec()),
            _ => HttpResponse::not_found(),
        }
    } else {
        match resolve_path(root, &request.path) {
            Some(ResolvedPath::File(file_path)) => serve_file(&file_path),
            Some(ResolvedPath::Directory(dir_path)) => {
                serve_directory(&dir_path, root, &request.path)
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
                std::thread::spawn(move || {
                    handle_connection(&stream, &root);
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
        assert!(resp.contains("<h1 id=\"hello\">Hello</h1>"));
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
        // Only one .md file → auto-serve it
        let resp = get(port, "/");
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains("<h1 id=\"readme\">Readme</h1>"));
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
        assert!(resp.contains("HTTP/1.1 200 OK"));
        assert!(resp.contains("<h1 id=\"welcome\">Welcome</h1>"));
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
        assert!(resp.contains("<h1 id=\"only\">Only</h1>"));
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
}
