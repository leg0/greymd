pub fn content_type_for(ext: &str) -> &'static str {
    match ext.to_ascii_lowercase().as_str() {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "txt" => "text/plain",
        "xml" => "application/xml",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_extension_defaults_to_octet_stream() {
        assert_eq!(content_type_for("xyz"), "application/octet-stream");
        assert_eq!(content_type_for(""), "application/octet-stream");
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(content_type_for("HTML"), "text/html");
        assert_eq!(content_type_for("Json"), "application/json");
        assert_eq!(content_type_for("PNG"), "image/png");
    }
}
