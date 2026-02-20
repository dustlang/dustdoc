# Testing

Source: `crates/dustdoc/tests/doc_gen.rs`

## Current Tests

- `parse_and_generate_basic`
  - verifies module doc extraction
  - verifies item extraction for `forge` and `process`
  - verifies Markdown heading and code-fence output
  - verifies HTML conversion runs and contains expected heading markup

- `parse_block_docs`
  - verifies `/*! ... */` module docs
  - verifies `/** ... */` item docs

## Coverage Profile

Current tests focus on happy-path extraction and output generation. They do not exhaustively validate all keyword heuristics or edge cases.

## Run Tests

From `dustdoc/`:

```bash
cargo test --workspace --verbose
```
