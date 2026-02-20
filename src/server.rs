use std::io::Write;
use std::net::TcpListener;
use std::path::Path;

use crate::http::{HttpRequest, HttpResponse};
use crate::mime::content_type_for;
use crate::path::resolve_path;

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

fn handle_connection(stream: &std::net::TcpStream, root: &Path) {
    let mut reader = std::io::BufReader::new(stream);
    let request = match HttpRequest::parse(&mut reader) {
        Some(r) => r,
        None => return,
    };

    let response = if request.method != "GET" {
        HttpResponse::method_not_allowed()
    } else {
        match resolve_path(root, &request.path) {
            Some(file_path) => match std::fs::read(&file_path) {
                Ok(contents) => {
                    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    HttpResponse::ok(content_type_for(ext), contents)
                }
                Err(_) => HttpResponse::not_found(),
            },
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
}
