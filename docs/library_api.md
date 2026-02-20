# Library API

Source: `crates/dustdoc/src/lib.rs`

## Public Types

## `DocModule`

```rust
pub struct DocModule {
    pub module_docs: Vec<String>,
    pub items: Vec<DocItem>,
}
```

## `DocItem`

```rust
pub struct DocItem {
    pub kind: String,
    pub name: String,
    pub signature: String,
    pub docs: Vec<String>,
    pub is_unsafe: bool,
}
```

## Public Functions

- `parse_file(path: &str) -> io::Result<DocModule>`
- `parse_str(src: &str) -> DocModule`
- `generate_markdown(module: &DocModule, file_name: &str) -> String`
- `markdown_to_html(markdown: &str) -> String`

## API Notes

- `parse_str` is heuristic and never returns parse errors.
- `parse_file` errors only on file I/O read failures.
- Markdown generation preserves doc text lines as collected.
