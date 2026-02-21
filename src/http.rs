use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
}

impl HttpRequest {
    pub fn parse(stream: &mut BufReader<&TcpStream>) -> Option<Self> {
        let mut request_line = String::new();
        stream.read_line(&mut request_line).ok()?;
        let mut parts = request_line.trim().splitn(3, ' ');
        let method = parts.next()?.to_string();
        let path = parts.next()?.to_string();
        // Consume remaining headers (read until empty line)
        loop {
            let mut line = String::new();
            match stream.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if line.trim().is_empty() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        Some(HttpRequest { method, path })
    }
}

pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: &'static str,
    pub content_type: &'static str,
    pub content_encoding: Option<&'static str>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn ok(content_type: &'static str, body: Vec<u8>) -> Self {
        Self {
            status_code: 200,
            status_text: "OK",
            content_type,
            content_encoding: None,
            body,
        }
    }

    pub fn ok_gzip(content_type: &'static str, body: Vec<u8>) -> Self {
        Self {
            status_code: 200,
            status_text: "OK",
            content_type,
            content_encoding: Some("gzip"),
            body,
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            status_text: "Not Found",
            content_type: "text/html",
            content_encoding: None,
            body: b"<h1>404 Not Found</h1>".to_vec(),
        }
    }

    pub fn method_not_allowed() -> Self {
        Self {
            status_code: 405,
            status_text: "Method Not Allowed",
            content_type: "text/html",
            content_encoding: None,
            body: b"<h1>405 Method Not Allowed</h1>".to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut header = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n",
            self.status_code,
            self.status_text,
            self.content_type,
            self.body.len()
        );
        if let Some(enc) = self.content_encoding {
            header.push_str(&format!("Content-Encoding: {}\r\n", enc));
        }
        header.push_str("\r\n");
        let mut bytes = header.into_bytes();
        bytes.extend_from_slice(&self.body);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // T006: Tests for HTTP request parsing
    #[test]
    fn parse_valid_get_request() {
        // We'll test the parsing logic by simulating input
        let raw = b"GET /index.html HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(&stream);
            HttpRequest::parse(&mut reader)
        });

        let mut client = std::net::TcpStream::connect(addr).unwrap();
        std::io::Write::write_all(&mut client, raw).unwrap();
        drop(client);

        let req = handle.join().unwrap().unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/index.html");
    }

    #[test]
    fn parse_post_request_extracts_method() {
        let raw = b"POST /upload HTTP/1.1\r\n\r\n";
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(&stream);
            HttpRequest::parse(&mut reader)
        });

        let mut client = std::net::TcpStream::connect(addr).unwrap();
        std::io::Write::write_all(&mut client, raw).unwrap();
        drop(client);

        let req = handle.join().unwrap().unwrap();
        assert_eq!(req.method, "POST");
        assert_eq!(req.path, "/upload");
    }

    #[test]
    fn parse_empty_request_returns_none() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(&stream);
            HttpRequest::parse(&mut reader)
        });

        let client = std::net::TcpStream::connect(addr).unwrap();
        drop(client);

        let req = handle.join().unwrap();
        assert!(req.is_none());
    }

    // T008: Tests for HTTP response formatting
    #[test]
    fn response_ok_formats_correctly() {
        let resp = HttpResponse::ok("text/html", b"<h1>Hi</h1>".to_vec());
        let bytes = resp.to_bytes();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(text.contains("Content-Type: text/html\r\n"));
        assert!(text.contains("Content-Length: 11\r\n"));
        assert!(text.ends_with("<h1>Hi</h1>"));
    }

    #[test]
    fn response_not_found_has_404_status() {
        let resp = HttpResponse::not_found();
        let bytes = resp.to_bytes();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.starts_with("HTTP/1.1 404 Not Found\r\n"));
    }

    #[test]
    fn response_method_not_allowed_has_405_status() {
        let resp = HttpResponse::method_not_allowed();
        let bytes = resp.to_bytes();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.starts_with("HTTP/1.1 405 Method Not Allowed\r\n"));
    }

    #[test]
    fn response_contains_correct_content_length() {
        let body = b"hello world".to_vec();
        let resp = HttpResponse::ok("text/plain", body);
        let bytes = resp.to_bytes();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains("Content-Length: 11\r\n"));
    }
}
