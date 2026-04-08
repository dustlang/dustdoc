# Architecture

## Crate Layout

- `src/main.rs`: CLI and file I/O orchestration.
- `src/lib.rs`: parser/extractor and output generators.
- `tests/doc_gen.rs`: integration tests.

## Core Data Model

- `DocModule`
  - `module_docs: Vec<String>`
  - `items: Vec<DocItem>`
- `DocItem`
  - `kind`, `name`, `signature`, `docs`, `is_unsafe`

## Processing Pipeline

1. read file content (`parse_file`) or parse in-memory text (`parse_str`).
2. scan lines and extract doc comment blocks and target item signatures.
3. build `DocModule` structure.
4. generate Markdown (`generate_markdown`).
5. optionally convert Markdown to HTML (`markdown_to_html`).

## Design Characteristic

Extraction is heuristic and line-oriented. It is not a full Dust parser.
