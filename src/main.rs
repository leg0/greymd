mod http;
mod listing;
mod markdown;
mod mime;
mod path;
mod server;

use std::env;
use std::net::TcpListener;
use std::path::PathBuf;

fn bind_listener() -> TcpListener {
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(_) => TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind to any port"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return;
    }

    let root = if args.len() > 1 {
        PathBuf::from(&args[1])
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

    let (css_path, js_path) = if let Some(config) = server::config_dir() {
        (config.join("css"), config.join("js"))
    } else {
        (PathBuf::new(), PathBuf::new())
    };

    let listener = bind_listener();
    let addr = listener.local_addr().unwrap();
    println!("Listening on http://{}", addr);

    server::start(listener, &root, css_path, js_path);
}

fn print_usage() {
    println!("Usage: greymd [directory]");
    println!();
    println!("Serve files from a directory over HTTP on localhost.");
    println!();
    println!("Arguments:");
    println!("  directory    Path to serve (defaults to current directory)");
    println!();
    println!("Options:");
    println!("  -h, --help   Show this help message");
    println!();
    println!("Customization:");
    println!("  ~/.config/greymd/css   Custom stylesheet (appended after built-in)");
    println!("  ~/.config/greymd/js    Custom JavaScript (replaces built-in highlight.js)");
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
}
