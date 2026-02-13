use dustdoc::{generate_markdown, markdown_to_html, parse_str};

#[test]
fn parse_and_generate_basic() {
    let src = r#"//! Module description.

/// A simple forge.
forge MyForge {
    /// The answer.
    answer: Int,
}

/// Creates a value.
process make_value() -> MyForge {
    // body omitted
}
 "#;
    let module = parse_str(src);
    // one inner doc and two items
    assert_eq!(module.module_docs.len(), 1);
    assert_eq!(module.items.len(), 2);
    // verify names
    assert_eq!(module.items[0].name, "MyForge");
    assert_eq!(module.items[1].name, "make_value");
    // verify docs
    assert!(module.items[0].docs[0].contains("A simple forge"));
    let md = generate_markdown(&module, "test.dust");
    // markdown contains heading and code fences
    assert!(md.contains("# Documentation for `test.dust`"));
    assert!(md.contains("## forge `MyForge`"));
    assert!(md.contains("```dpl"));
    // convert to html just to ensure the function executes
    let html = markdown_to_html(&md);
    // Check for actual HTML output format
    assert!(html.contains("<h2>forge"));
}

#[test]
fn parse_block_docs() {
    let src = r#"/*!
This module exposes an API.
*/
/**
Multi‑line doc comment for a shape.

It has multiple paragraphs.
*/
shape Widget {}
"#;
    let module = parse_str(src);
    assert_eq!(module.module_docs.len(), 1);
    assert_eq!(module.items.len(), 1);
    assert_eq!(module.items[0].kind, "shape");
    assert!(module.items[0].docs[0].contains("Multi‑line doc comment"));
}
