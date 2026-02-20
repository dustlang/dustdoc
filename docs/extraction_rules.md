# Extraction Rules

Source: `crates/dustdoc/src/lib.rs` (`parse_str`)

## Supported Doc Comment Forms

Module-level docs:

- `//! ...`
- `/*! ... */`

Item-level docs:

- `/// ...`
- `/** ... */`

Non-doc line comments (`// ...`) are skipped.

## Block Comment Handling

- multiline block docs are accumulated until `*/`.
- leading `*` is trimmed from each interior line.
- one-line block docs are handled inline.

## Item Detection

When pending item docs exist, the next non-empty non-comment code line may become a documented item if its first token matches heuristics.

Recognized first-token keywords include:

- `forge`, `shape`, `process`, `bind`, `effect`, `module`, `type`, `trait`, `enum`, `const`
- `K`, `Q`, `Phi`
- `alloc`, `free`, `spawn`, `join`, `mutex_new`, `mutex_lock`, `mutex_unlock`
- `open`, `read`, `write`, `close`, `io_read`, `io_write`, `mmio_read`, `mmio_write`
- `unsafe`

Additional type-style heuristic:

- first token in `{ Thread, Mem, Mutex, File, Port, Device, Ptr }`
- or token starts with `Thread` and contains `<`

## Signature, Kind, and Name Parsing

For matched item lines:

- `kind` = first whitespace-delimited token.
- `name` = second token after trimming trailing `{`, `)`, and `(`.
- `signature` = full trimmed line.

## Trailing Docs

If the file ends with pending item docs and no following item line, those docs are appended to `module_docs`.
