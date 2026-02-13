//! Core library for the `dustdoc` documentation generator.
//!
//! This crate defines simple data structures and functions for extracting
//! documentation comments from Dust source files and generating
//! human‑readable output.  See the crate‐level README for background and
//! motivation.

use std::fs::File;
use std::io::{self, Read};

/// Represents a documented module.
///
/// The `module_docs` field contains documentation associated with the
/// enclosing module (outer doc comments beginning with `//!` or block
/// comments `/*! ... */`).  Each entry in `items` represents a top‑level
/// declaration (forge, shape, process, etc.) along with its signature
/// and associated doc comments.
#[derive(Debug, Clone)]
pub struct DocModule {
    /// Module‑level documentation extracted from `//!` and `/*!` comments.
    pub module_docs: Vec<String>,
    /// Documented items within the module.
    pub items: Vec<DocItem>,
}

/// A single documented top‑level item.
#[derive(Debug, Clone)]
pub struct DocItem {
    /// Kind of the item (e.g., `forge`, `shape`, `process`, etc.).
    pub kind: String,
    /// Identifier of the item.
    pub name: String,
    /// Raw signature line for the item, trimmed of leading/trailing whitespace.
    pub signature: String,
    /// Documentation lines associated with the item.
    pub docs: Vec<String>,
    /// Whether this item is in an unsafe block (v0.2).
    pub is_unsafe: bool,
}

/// Parse a Dust source file and return its documentation module.
pub fn parse_file(path: &str) -> io::Result<DocModule> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(parse_str(&buf))
}

/// Parse the contents of a Dust source string into a `DocModule`.
///
/// This function scans the source line by line, collecting documentation
/// comments that precede top‑level items.  It understands line and
/// block comments of the form `///`, `/** */`, `//!` and `/*! */`
/// according to the conventions used by Rust【602051234871284†L254-L279】.  It does not
/// perform full syntactic analysis of Dust code; instead, it uses simple
/// heuristics to recognise top‑level declarations (`forge`, `shape`,
/// `process`, `bind`, `effect`) and treats the next non‑comment line
/// following a doc comment as that item’s signature.
pub fn parse_str(src: &str) -> DocModule {
    let mut module_docs: Vec<String> = Vec::new();
    let mut items: Vec<DocItem> = Vec::new();

    let mut current_docs: Vec<String> = Vec::new();
    let mut in_block_doc: Option<(bool, Vec<String>)> = None;

    // helper closure to flush pending docs into an item
    let mut flush_item = |signature: &str, is_unsafe: bool, docs: &mut Vec<String>| {
        if docs.is_empty() {
            return;
        }
        // Determine kind and name from signature
        let trimmed = signature.trim();
        let mut parts = trimmed.split_whitespace();
        let kind = parts.next().unwrap_or("").to_string();
        let name_part = parts.next().unwrap_or("");
        // Remove trailing punctuation from name (e.g. `{`, `)`, `(`) if present
        let name = name_part
            .trim_end_matches('{')
            .trim_end_matches(')')
            .trim_end_matches('(')
            .to_string();
        let item = DocItem {
            kind,
            name,
            signature: trimmed.to_string(),
            docs: docs.clone(),
            is_unsafe,
        };
        items.push(item);
        docs.clear();
    };

    // iterate over lines to collect docs and signatures
    let mut lines = src.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        // handle block doc comments
        if let Some((is_inner, ref mut accum)) = in_block_doc {
            // Continue until we find the terminating '*/'
            if let Some(end_idx) = trimmed.find("*/") {
                let before = &trimmed[..end_idx];
                // strip leading '*' characters commonly used in block comments
                let cleaned = before.trim_start_matches('*').trim();
                if !cleaned.is_empty() {
                    accum.push(cleaned.to_string());
                }
                // flush into module_docs or current_docs based on is_inner
                if is_inner {
                    module_docs.extend(accum.drain(..));
                } else {
                    current_docs.extend(accum.drain(..));
                }
                in_block_doc = None;
                // remainder after */ is ignored by comment parser
            } else {
                // line inside block doc comment
                let cleaned = trimmed.trim_start_matches('*').trim();
                accum.push(cleaned.to_string());
            }
            continue;
        }

        // detect start of block doc comment
        if trimmed.starts_with("/*!") {
            // inner doc for module
            let content = trimmed.trim_start_matches("/*!").trim();
            if let Some(end_idx) = content.find("*/") {
                // one-line block
                let before = &content[..end_idx];
                let cleaned = before.trim_start_matches('*').trim();
                if !cleaned.is_empty() {
                    module_docs.push(cleaned.to_string());
                }
            } else {
                in_block_doc = Some((true, Vec::new()));
            }
            continue;
        }
        if trimmed.starts_with("/**") {
            // outer block doc comment
            let content = trimmed.trim_start_matches("/**").trim();
            if let Some(end_idx) = content.find("*/") {
                // one-line block
                let before = &content[..end_idx];
                let cleaned = before.trim_start_matches('*').trim();
                if !cleaned.is_empty() {
                    current_docs.push(cleaned.to_string());
                }
            } else {
                in_block_doc = Some((false, Vec::new()));
            }
            continue;
        }

        // line doc comments
        if trimmed.starts_with("//!") {
            let text = trimmed.trim_start_matches("//!").trim().to_string();
            module_docs.push(text);
            continue;
        }
        if trimmed.starts_with("///") {
            let text = trimmed.trim_start_matches("///").trim().to_string();
            current_docs.push(text);
            continue;
        }
        // skip non-doc comments
        if trimmed.starts_with("//") {
            continue;
        }
        if trimmed.is_empty() {
            continue;
        }
        // at this point we have code; check if we have pending docs
        // v0.2 expanded keywords
        let keywords = [
            "forge",
            "shape",
            "process",
            "bind",
            "effect",
            "module",
            "type",
            "trait",
            "enum",
            "const",
            "K",
            "Q",
            "Φ",
            // v0.2 K-regime keywords
            "alloc",
            "free",
            "spawn",
            "join",
            "mutex_new",
            "mutex_lock",
            "mutex_unlock",
            "open",
            "read",
            "write",
            "close",
            "io_read",
            "io_write",
            "mmio_read",
            "mmio_write",
            "unsafe",
        ];
        let first_word = trimmed.split_whitespace().next().unwrap_or("");

        // Check for unsafe blocks (v0.2)
        let is_unsafe = trimmed.starts_with("unsafe");

        if keywords.contains(&first_word) || is_unsafe {
            flush_item(trimmed, is_unsafe, &mut current_docs);
            continue;
        }
        // Check for type declarations with generic parameters (v0.2)
        // Use word boundary check to avoid matching partial words like "Mem" in "Memory"
        let type_start = trimmed.split_whitespace().next().unwrap_or("");
        let v0_2_types = ["Thread", "Mem", "Mutex", "File", "Port", "Device", "Ptr"];
        if v0_2_types.contains(&type_start)
            || (type_start.starts_with("Thread") && type_start.contains('<'))
        {
            flush_item(trimmed, false, &mut current_docs);
            continue;
        }
        // no doc comment and not a recognised item; continue
    }
    // flush any trailing docs if file ends with docs but no item
    // (these docs are module-level by convention)
    if !current_docs.is_empty() {
        module_docs.extend(current_docs.clone());
        current_docs.clear();
    }
    DocModule { module_docs, items }
}

/// Generate a Markdown document from a parsed `DocModule`.
pub fn generate_markdown(module: &DocModule, file_name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Documentation for `{}`\n\n", file_name));
    // Module docs
    if !module.module_docs.is_empty() {
        for line in &module.module_docs {
            out.push_str(line);
            out.push('\n');
        }
        out.push('\n');
    }
    // Items
    for item in &module.items {
        let unsafe_badge = if item.is_unsafe { " **(unsafe)**" } else { "" };
        out.push_str(&format!(
            "## {} `{}`{}\n\n",
            item.kind, item.name, unsafe_badge
        ));
        for line in &item.docs {
            out.push_str(line);
            out.push('\n');
        }
        out.push('\n');
        out.push_str("```dpl\n");
        out.push_str(&item.signature.trim());
        out.push('\n');
        out.push_str("```\n\n");

        // Add type information for v0.2 resource types
        if item.name.starts_with("Thread<")
            || item.name.starts_with("Mem")
            || item.name == "Mutex"
            || item.name == "File"
            || item.name == "Port"
            || item.name == "Device"
            || item.name == "Ptr"
        {
            out.push_str("*Resource type (v0.2)*\n\n");
        }
    }
    out
}

/// Convert Markdown into HTML using `pulldown-cmark`.
pub fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{html::push_html, Options, Parser};
    let parser = Parser::new_ext(markdown, Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES);
    let mut html = String::new();
    push_html(&mut html, parser);
    html
}
