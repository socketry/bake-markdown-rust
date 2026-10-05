// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake_markdown::normalize_document;

#[test]
fn serializes_a_normalized_markdown_document() {
    let normalized = normalize_document("# Heading\n\nA wrapped\nparagraph.\n").unwrap();

    assert_eq!(normalized, "# Heading\n\nA wrapped paragraph.\n");
}
