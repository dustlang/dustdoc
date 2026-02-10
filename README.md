# Dustdoc – Literate documentation for DPL

`dustdoc` is the documentation generator for the **Dust Programming
Language (DPL)**.  It turns DPL source files into
human‑readable documentation by extracting *doc comments* and
interleaving them with code signatures.  The resulting output can be
rendered as Markdown or HTML, allowing the same source file to
simultaneously serve as a *paper* and as compilable code.  This
“papers‑as‑code” model embraces the literate programming
philosophy: a program is an explanation of how it works in natural
language interspersed with code【697973716384902†L144-L160】.  Tools like
`dustdoc` generate both a human‑readable “woven” document and the
compiler‑friendly “tangled” code from one file【697973716384902†L161-L164】.

## Motivation and design

Donald Knuth introduced *literate programming* in 1984 as a way to
write programs as essays for human readers, with code hidden behind
macros【697973716384902†L144-L160】.  Modern literate programming tools
produce two representations: compilable source code and formatted
documentation【697973716384902†L161-L164】.  The research community has
extended this idea into *reproducible manuscripts*, where a single file
contains text and code that generates figures and tables.  In this
workflow, the manuscript follows the flow of human logic, with code
executed as part of the narrative【697203471999866†L116-L123】.  `dustdoc` adopts this
approach for DPL: documentation lives alongside code using special
comments, and the generator creates a well‑structured document from
those comments.

Documentation comments in DPL follow the same conventions as Rust: a
line comment beginning with exactly three slashes (`///`) or a block
comment starting with `/**` creates a *doc comment*.  These comments
are treated as attributes on the following item【602051234871284†L254-L266】.
Doc comments may contain Markdown【312645311017844†L380-L384】 and are
attached to the item that follows them.  Inner doc comments beginning
with `//!` document the enclosing module【602051234871284†L271-L279】.  `dustdoc`
recognises these patterns and collects the doc text until it sees the
next code item.  When generating output it renders the comments as
Markdown and includes the signature of the item as a fenced code
block.

`dustdoc` is intentionally **non‑configurable**.  The Dust toolchain
embraces a structural and deterministic philosophy—every Dust program
has one canonical formatting and one canonical way to extract
documentation.  This makes it easy to read and maintain projects.

## Usage

```
Usage: dustdoc [OPTIONS] <source> [<output>]

Options:
    --html       Generate HTML instead of Markdown (default is Markdown).
    -h, --help   Print this help message.

Arguments:
    <source>     Path to a `.dust` or `.dpaper` source file.
    [output]     Optional output file.  If omitted, the result is printed to stdout.
```

Run `dustdoc` on a Dust source file that contains documentation
comments.  For example:

```rust
//! A tiny module illustrating doc comments.

/// Represents a user.
shape User {
    /// The user’s name.
    name: String,
    /// The user’s age.
    age: Int,
}

/// Creates a new user.
process new_user(name: String, age: Int) -> User {
    // …
}
```

Running `dustdoc module.dust module.md` will produce a Markdown file
with a top‑level description, then sections for `User` and
`new_user`, each containing the extracted doc comments and the
signature of the item as a code block.  Use `--html` to convert the
Markdown to a standalone HTML file via [`pulldown‑cmark`](https://crates.io/crates/pulldown-cmark).

## Papers‑as‑code

The Dust specification and technical papers can be written as Dust
files using doc comments to hold the prose.  When run through
`dustdoc`, they yield a document suitable for publication, while the
same source remains a valid Dust program that can be executed or
analysed.  This approach aligns with modern reproducible research
practices where a manuscript includes all code used to produce its
figures and tables【697203471999866†L43-L46】.  By keeping the code and
documentation in one place we ensure that the paper and its
implementation stay in sync.

## Tests and CI

This repository contains a small test suite under `tests/` that
exercises doc extraction and Markdown generation.  A GitHub
Actions workflow (`.github/workflows/ci.yml`) builds the crate and runs
the tests on each push, ensuring that the generator remains stable.

## License

`dustdoc` is distributed under the permissive **Dust Open Source
License (DOSL)**.  See `LICENSE` for the full text.