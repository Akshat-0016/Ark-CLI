use crate::storage::vault::Vault;

use crate::index::{builder::IndexBuilder, document::Document};

fn markdown_is_cleaned_before_indexing() {
    let mut vault = Vault::new();

    let markdown = r###"
# Rust

```rust
println!("hello");

Rust language.
"###;

    vault.add(Document::new(
        0,
        "Markdown".to_string(),
        "markdown.md".to_string(),
        markdown.to_string(),
    ));

    let artifacts = IndexBuilder::new().build(&vault);

    assert!(artifacts.index.contains("rust"));
    assert!(artifacts.index.contains("language"));
    assert!(!artifacts.index.contains("println"));
}
