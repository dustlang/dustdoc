# Developer Guide

## Build

```bash
cargo build --workspace --verbose
```

## Test

```bash
cargo test --workspace --verbose
```

## Local Smoke Run

```bash
cargo run -p dustdoc -- sample.dust
cargo run -p dustdoc -- --html sample.dust sample.html
```

## Suggested Change Workflow

1. update extraction logic in `src/lib.rs`
2. update CLI behavior in `src/main.rs` if needed
3. add/adjust tests in `tests/doc_gen.rs`
4. update docs in `dustdoc/docs`

## High-Value Improvements

- replace heuristic parser with AST-backed extraction from compiler frontend
- tighten item signature parsing for `kind`/`name`
- validate argument count and reject unexpected extra positionals
- add richer HTML template support
