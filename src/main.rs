mod http;
mod mime;
mod path;
mod server;

use std::env;
use std::path::PathBuf;

const DEFAULT_PORT: u16 = 8080;

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

    server::start(&root, DEFAULT_PORT);
}

fn print_usage() {
    println!("Usage: docsvr [directory]");
    println!();
    println!("Serve files from a directory over HTTP on localhost.");
    println!();
    println!("Arguments:");
    println!("  directory    Path to serve (defaults to current directory)");
    println!();
    println!("Options:");
    println!("  -h, --help   Show this help message");
}
