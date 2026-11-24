//! Standalone extraction of the Codex TUI Markdown Writer.

mod assistant_directives;
mod clipboard_copy;
mod line_truncation;
mod live_wrap;
mod markdown;
mod markdown_copy;
mod markdown_render;
mod markdown_text_merge;
mod render {
    pub(crate) mod highlight;
    pub(crate) mod line_utils;
}
mod string_utils;
mod style;
mod table_detect;
mod terminal_hyperlinks;
mod width;
mod wrapping;

use ratatui::text::Line;
use std::path::Path;

/// Render a complete Markdown message using the Codex Writer.
///
/// Width is in terminal columns. Code blocks and certain URL-like tokens can exceed it.
/// The working directory is used only to shorten displayed local links.
pub fn render(input: &str, width: usize, cwd: &Path) -> Vec<Line<'static>> {
    terminal_hyperlinks::visible_lines(markdown::render_markdown_agent_with_list_spacing(
        input,
        Some(width.max(1)),
        Some(cwd),
        markdown_render::ListSpacing::AfterMultiline,
    ))
}
