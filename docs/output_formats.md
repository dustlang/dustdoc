# Output Formats

## Markdown Generation

Source: `generate_markdown` in `crates/dustdoc/src/lib.rs`

Markdown structure:

1. top heading:

```text
# Documentation for `<file_name>`
```

2. module docs (raw lines)
3. one section per item:

```text
## <kind> `<name>`[ optional unsafe badge ]
```

4. item docs (raw lines)
5. fenced code block containing the signature:

```text
```dpl
<signature>
```
```

Unsafe badge:

- if `is_unsafe` is true, heading includes ` **(unsafe)**`.

Resource annotation:

- for item names matching specific resource patterns (`Thread<...>`, `Mem...`, `Mutex`, `File`, `Port`, `Device`, `Ptr`), appends:

```text
*Resource type (v0.2)*
```

## HTML Generation

Source: `markdown_to_html` in `crates/dustdoc/src/lib.rs`

- uses `pulldown-cmark`.
- enabled options:
  - `ENABLE_TABLES`
  - `ENABLE_FOOTNOTES`

HTML conversion is purely Markdown-to-HTML; no template/shell wrapping is added.
