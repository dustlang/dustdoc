# CLI Reference

Source: `crates/dustdoc/src/main.rs`

## Usage

```text
dustdoc [OPTIONS] <source> [<output>]
```

## Options

- `--html`: generate HTML instead of Markdown.
- `-h`, `--help`: print help.

## Arguments

- `<source>`: input source file path.
- `[output]`: optional output file path. If omitted, output is written to stdout.

## Behavior

- no args, `-h`, or `--help`: print help and exit `0`.
- `--html` can appear among args and is removed before positional parsing.
- first positional arg is source.
- second positional arg is output (if present).
- additional positional args are ignored by current implementation.

## Error Paths

- read failure: `error: failed to read <source>: <io error>` then exit `1`.
- write failure: `error: failed to write <output>: <io error>` then exit `1`.

## Output Mode

- default: generate Markdown from parsed module.
- with `--html`: convert Markdown output to HTML before write/print.
