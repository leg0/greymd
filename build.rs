use miniz_oxide::deflate::compress_to_vec;
use std::path::Path;
use std::{env, fs};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let asset_dir = Path::new("src/assets");

    // Combine CSS files
    let style = fs::read(asset_dir.join("style.css")).expect("style.css");
    let hlcss = fs::read(asset_dir.join("highlight-github.css")).expect("highlight-github.css");
    let mut combined_css = style;
    combined_css.extend_from_slice(&hlcss);

    let hljs = fs::read(asset_dir.join("highlight.min.js")).expect("highlight.min.js");

    fs::write(Path::new(&out_dir).join("combined.css.gz"), gzip(&combined_css)).expect("write combined.css.gz");
    fs::write(Path::new(&out_dir).join("highlight.min.js.gz"), gzip(&hljs)).expect("write highlight.min.js.gz");

    println!("cargo::rerun-if-changed=src/assets/style.css");
    println!("cargo::rerun-if-changed=src/assets/highlight-github.css");
    println!("cargo::rerun-if-changed=src/assets/highlight.min.js");
}

fn gzip(data: &[u8]) -> Vec<u8> {
    let compressed = compress_to_vec(data, 6);
    let crc = crc32(data);
    let len = data.len() as u32;

    let mut out = Vec::with_capacity(10 + compressed.len() + 8);
    // Gzip header (10 bytes)
    out.extend_from_slice(&[0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff]);
    out.extend_from_slice(&compressed);
    // Gzip trailer: CRC32 + original size (little-endian)
    out.extend_from_slice(&crc.to_le_bytes());
    out.extend_from_slice(&len.to_le_bytes());
    out
}

fn crc32(data: &[u8]) -> u32 {
    // Precompute CRC32 table (IEEE polynomial)
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut c = i;
        for _ in 0..8 {
            if c & 1 != 0 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        table[i as usize] = c;
    }

    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc = table[((crc ^ b as u32) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc ^ 0xFFFF_FFFF
}
