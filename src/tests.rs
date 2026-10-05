// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use socketry_markdown::ParseOptions;

use super::{normalize_document, normalize_document_with_options};

#[test]
fn unwraps_soft_source_line_breaks() {
    let normalized = normalize_document("A paragraph\nwith a soft break.\n").unwrap();

    assert_eq!(normalized, "A paragraph with a soft break.\n");
}

#[test]
fn preserves_hard_breaks_code_content_and_block_boundaries() {
    let source = "First paragraph\ncontinues.\n\nHard break  \ncontinues.\n\n```text\ncode line one\ncode line two\n```\n\nLast paragraph.\n";

    let normalized = normalize_document(source).unwrap();

    assert_eq!(
        normalized,
        "First paragraph continues.\n\nHard break\\\ncontinues.\n\n```text\ncode line one\ncode line two\n```\n\nLast paragraph.\n"
    );
}

#[test]
fn normalization_is_idempotent() {
    let first = normalize_document("A paragraph\nwith a soft break.\n").unwrap();
    let second = normalize_document(&first).unwrap();

    assert_eq!(second, first);
}

#[test]
fn normalizes_unordered_lists_to_hyphen_markers() {
    let normalized = normalize_document("* first\n* second\n").unwrap();

    assert_eq!(normalized, "- first\n- second\n");
}

#[test]
fn preserves_front_matter_and_unwraps_nested_phrasing() {
    let source = "---\ntitle: Example\n---\n\nA *wrapped\nemphasis* paragraph.\n";

    let normalized = normalize_document(source).unwrap();

    assert_eq!(
        normalized,
        "---\ntitle: Example\n---\n\nA *wrapped emphasis* paragraph.\n"
    );
}

#[test]
fn preserves_toml_front_matter() {
    let source = "+++\ntitle = \"Example\"\n+++\n\nA paragraph.\n";

    let normalized = normalize_document(source).unwrap();

    assert_eq!(normalized, source);
}

#[test]
fn normalizes_crlf_soft_breaks_to_lf_output() {
    let normalized = normalize_document("A paragraph\r\nwith a soft break.\r\n").unwrap();

    assert_eq!(normalized, "A paragraph with a soft break.\n");
}

#[test]
fn preserves_inline_code_and_math() {
    let source = "Inline `code` and inline math $x + y$.\n";

    let normalized = normalize_document(source).unwrap();

    assert_eq!(normalized, source);
}

#[test]
fn preserves_gfm_tables_and_normalizes_task_list_markers() {
    let source = "| key | value |\n| --- | --- |\n| name | Ada |\n\n~~removed~~ and a task:\n\n- [x] done\n- [ ] pending\n\nA claim[^note].\n\n[^note]: A note.\n";

    let normalized = normalize_document(source).unwrap();

    assert_eq!(
        normalized,
        "| key | value |\n| --- | --- |\n| name | Ada |\n\n~~removed~~ and a task:\n\n- [x] done\n- [ ] pending\n\nA claim[^note].\n\n[^note]: A note.\n"
    );
}

#[test]
fn reports_errors_from_explicit_mdx_parsing() {
    let options = ParseOptions::mdx();

    let error = normalize_document_with_options("<Open></Different>", &options).unwrap_err();

    assert!(error.to_string().contains("Unexpected closing tag"));
}
