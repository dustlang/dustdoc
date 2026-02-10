//! Command‑line interface for the `dustdoc` documentation generator.
//!
//! This binary reads a Dust source file, extracts documentation
//! comments, and writes out a formatted document.  By default it
//! produces Markdown; pass `--html` to emit HTML instead.

use std::env;
use std::fs;
use std::path::Path;

use dustdoc::{generate_markdown, markdown_to_html, parse_file};

fn print_help() {
    eprintln!("Usage: dustdoc [OPTIONS] <source> [<output>]");
    eprintln!("\nOptions:");
    eprintln!("    --html       Generate HTML instead of Markdown (default is Markdown).");
    eprintln!("    -h, --help   Print this help message.");
    eprintln!("\nArguments:");
    eprintln!("    <source>     Path to a `.dust` or `.dpaper` source file.");
    eprintln!("    [output]     Optional output file.  If omitted, the result is printed to stdout.");
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        return;
    }
    // parse flags
    let mut html = false;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--html" {
            html = true;
            args.remove(i);
        } else {
            i += 1;
        }
    }
    if args.is_empty() {
        print_help();
        return;
    }
    let source = args[0].clone();
    let output = if args.len() > 1 { Some(args[1].clone()) } else { None };
    // parse file
    let module = match parse_file(&source) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: failed to read {}: {}", source, e);
            std::process::exit(1);
        }
    };
    let file_name = Path::new(&source)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&source);
    let md = generate_markdown(&module, file_name);
    let output_text = if html { markdown_to_html(&md) } else { md };
    match output {
        Some(ref out_path) => {
            if let Err(e) = fs::write(out_path, output_text) {
                eprintln!("error: failed to write {}: {}", out_path, e);
                std::process::exit(1);
            }
        }
        None => {
            println!("{}", output_text);
        }
    }
}