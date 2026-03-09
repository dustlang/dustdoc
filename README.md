# dustdoc

`dustdoc` is now expressed in the currently supported top-level Dust grammar profile:

- single executable unit: `src/main.ds`
- entrypoint form: `K main { ... }`

This profile is designed to build with the current `dust` compiler parser without forge/proc namespace syntax.

## Build

From `dust/` compiler workspace:

```bash
CARGO_HOME=/tmp/cargo-home cargo run -p dust -- build ../dustdoc/src --out ../dustdoc/target/dust/dustdoc
```

## Notes

- The previous Rust crate layout (`Cargo.toml`, `Cargo.lock`, and `crates/dustdoc`) was retired for Dust-native migration.
- Existing docs are retained for documentation-generator intent and roadmap continuity.
