// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Markdown normalization tasks for Bake.
//!
//! [`normalize_document`] parses CommonMark, GFM, front matter, and math, then
//! serializes the result with soft source line breaks unwrapped. CommonMark
//! includes inline code. The `markdown:normalize` Bake task applies this to
//! one or more files.

use socketry_markdown::{
    LineWrapping, MarkdownOptions, ParseOptions, message::Message, to_markdown_with_options,
    to_mdast,
};

/// Normalize a Markdown document by joining soft source line breaks.
///
/// CommonMark, GFM, front matter, and math syntax are parsed so these features
/// survive the round trip. This includes inline code, fenced code, tables,
/// task lists, footnotes, and strikethrough. Explicit hard breaks, code
/// content, and block boundaries are preserved. The result ends with a newline
/// when it is non-empty.
///
/// # Errors
///
/// Returns an error if the Markdown syntax tree cannot be parsed or serialized.
pub fn normalize_document(source: &str) -> Result<String, Message> {
    let mut options = ParseOptions::gfm();
    options.constructs.frontmatter = true;
    options.constructs.math_flow = true;
    options.constructs.math_text = true;

    normalize_document_with_options(source, &options)
}

/// Normalize a Markdown document using explicit parser options.
///
/// Use this for syntax beyond the default GitHub Flavored Markdown, front
/// matter, and math support, such as MDX.
///
/// # Errors
///
/// Returns an error if the Markdown syntax tree cannot be parsed or serialized.
pub fn normalize_document_with_options(
    source: &str,
    parse_options: &ParseOptions,
) -> Result<String, Message> {
    let tree = to_mdast(source, parse_options)?;

    to_markdown_with_options(
        &tree,
        &MarkdownOptions {
            line_wrapping: LineWrapping::Unwrap,
            ..MarkdownOptions::default()
        },
    )
}

mod file_system;

use std::path::PathBuf;

/// Normalize one or more Markdown files beneath the Bake project root.
#[bake::task]
pub fn normalize(
    context: &mut bake::Context,
    #[bake(help = "Repeat for each Markdown file, relative to the project root.")] path: Vec<
        PathBuf,
    >,
) -> bake::Result<String> {
    let changed = file_system::normalize_files(context.root(), &path)?;
    Ok(format!(
        "Normalized {changed} of {} Markdown files",
        path.len()
    ))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
