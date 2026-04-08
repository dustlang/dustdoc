# dustdoc Documentation

This directory contains Markdown documentation for `dustdoc`.

## Documentation Index

- `getting_started.md`: build, install, and first runs.
- `cli_reference.md`: command-line behavior and exit semantics.
- `architecture.md`: module layout and processing pipeline.
- `extraction_rules.md`: extraction and item-detection rules.
- `output_formats.md`: Markdown and HTML output generation notes.
- `library_api.md`: legacy API notes from pre-migration Rust implementation.
- `testing.md`: test strategy and migration caveats.
- `known_limitations.md`: implementation limits and heuristic behavior.
- `developer_guide.md`: local workflow for maintainers.

## Scope

`dustdoc` now ships as a Dust-native top-level grammar profile (`src/main.ds`).
Historical docs that reference the retired Rust crate layout are retained for roadmap context.
