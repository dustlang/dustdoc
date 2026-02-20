# Getting Started

## Prerequisites

- Rust toolchain with `cargo`.
- Dust source files to document.

## Build

From `dustdoc/`:

```bash
cargo build --release
```

Binary path (Windows):

```text
target\release\dustdoc.exe
```

## Basic Markdown Generation

```bash
cargo run -p dustdoc -- path\to\module.dust module.md
```

If output is omitted, result is printed to stdout:

```bash
cargo run -p dustdoc -- path\to\module.dust
```

## HTML Generation

```bash
cargo run -p dustdoc -- --html path\to\module.dust module.html
```

## Help

```bash
cargo run -p dustdoc -- --help
```

## Source File Notes

CLI help mentions `.dust` and `.dpaper`, but implementation does not enforce file extensions. Any readable text path is accepted.
